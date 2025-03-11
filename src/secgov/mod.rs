// Archivo: src/secgov/mod.rs

mod parser;
pub mod models;

use self::models::FilingTransaction;
use self::parser::FilingDoc;
use chrono::{Datelike, NaiveDate};
use flate2::read::{DeflateDecoder, GzDecoder};
use parser::index::{extract_index_entries, get_quarter, IndexEntry};
use reqwest::header::{HeaderMap, ACCEPT_ENCODING, HOST, USER_AGENT};
use reqwest::{Client, RequestBuilder, Url};
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

const BASEURL: &str = "https://www.sec.gov/Archives/";
type Db = Arc<Mutex<Vec<FilingTransaction>>>;

pub async fn get_form(entry: &IndexEntry) -> Result<Vec<FilingTransaction>, Box<dyn Error>> {
    let url = format!("{BASEURL}{}", entry.filepath);
    println!("url: {url}");

    let client = Client::new();
    let res = client.get(
        Url::parse(&url).expect("Failed to parse valid URL")
    )
        .header("User-Agent", "Joordih Development jj@joordih.dev (Jordi Xavier)")
        .send()
        .await?;

    let body = res.text().await?;

    FilingDoc::new(&url, &body)
}

fn save_failed(index_url: &str) {
    let log_file_path = "filings/failed.txt";

    if let Err(err) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .and_then(|mut file| {
            use std::io::Write;
            writeln!(file, "Failed to proccess entry: {}", index_url)
        }) {
        eprintln!("Error occurred writing {} to failed.txt: {}", index_url, err);
    }
/*    let mut file = OpenOptions::new()
        .append(true)
        .open("filings/failed.txt")
        .unwrap();

    if let Err(_) = writeln!(file, "{}", index_url) {
        println!("Error occurred writing {} to failed.txt", index_url);
    }*/
}

pub async fn process_entries(entries: &[IndexEntry], db: Db, skip: usize, take: usize) -> Result<(), Box<dyn Error>> {
    for entry in entries.iter().cloned().skip(skip).take(take) {
        let db = db.clone();

        tokio::spawn(async move {
            let result = get_form(&entry).await;
            match result {
                Ok(mut filings) => {
                    db.lock()
                        .and_then(|mut v| { v.append(&mut filings); Ok(()) })
                        .expect("Could not push to mutex db");
                },
                Err(err) => {
                    println!("Error occurred for filing {}: {:?}", entry.filepath, err);
                    save_failed(&entry.filepath);
                }
            }
        });
    }

    Ok(())
}

pub async fn get_daily_entries(date: NaiveDate) -> Result<Vec<IndexEntry>, Box<dyn Error>> {
    let flat_date = date.format("%Y%m%d").to_string();
    let qtr = get_quarter(date);
    let index_url = format!(
        "https://www.sec.gov/Archives/edgar/daily-index/{}/{}/master.20250213.idx",
        date.year(), qtr
    );

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Joordih Development jj@joordih.dev (Jordi Xavier)".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip".parse().unwrap());
    headers.insert(HOST, "www.sec.gov".parse().unwrap());

    let request = client.get(
        Url::parse(&index_url).expect("Failed to parse valid URL")
    )
        .headers(headers.clone());

    println!("Send request to: {index_url}");
    let response = RequestBuilder::send(request).await.unwrap();
    let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
    let body = response.bytes().await.unwrap();

    let decompressed_data = body_decoder(body, content_encoding).await?;
    Ok(extract_index_entries(&String::from_utf8_lossy(&decompressed_data)))
}

async fn body_decoder(body: bytes::Bytes, content_encoding: Option<reqwest::header::HeaderValue>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decompressed_data = Vec::new();

    let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

    match encoding.as_deref() {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        Some("deflate") => {
            let mut decoder = DeflateDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        _ => {
            decompressed_data = body.to_vec();
        }
    }

    Ok(decompressed_data)
}