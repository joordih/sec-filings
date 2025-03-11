use crate::database::{get_connection, SqlHelper};
use crate::secgov::models::FilingTransaction;
use crate::secgov::{get_daily_entries, process_entries};
use chrono::{Datelike, Days, NaiveDate};
use chrono_tz::America::New_York;
use futures::stream::{self, StreamExt};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Miner {
    pub mine_date: NaiveDate,
}

impl Miner {
    pub fn new(start: &NaiveDate) -> Miner {
        Miner { mine_date: *start }
    }

    fn yesterday() -> NaiveDate {
        chrono::Local::now()
            .with_timezone(&New_York)
            .date_naive()
            .checked_sub_days(Days::new(0))
            .unwrap()
    }

    fn increment_day(&mut self) {
        self.mine_date = self.mine_date.checked_add_days(Days::new(1)).unwrap()
    }

    fn save_dir(date: NaiveDate) -> String {
        let year = date.year();
        let month = date.format("%m");
        format!("filings/{year}/{month}")
    }

    fn file_path(&self) -> String {
        let date = self.mine_date.format("%Y%m%d");
        format!("{}/{date}-filing.json", Self::save_dir(self.mine_date))
    }

    fn save_filings_json(&self, filings: &[FilingTransaction]) {
        let filepath = self.file_path();

        fs::create_dir_all(Self::save_dir(self.mine_date))
            .expect("Failed to create dir path");

        let text = serde_json::to_string(&filings).expect("Failed to serialize struct");
        fs::write(filepath, text).expect("Unable to write file");
    }

    async fn save_filings_db(filings: &[FilingTransaction]) -> Result<(), Box<dyn Error>> {
        let client = Arc::new(Mutex::new(get_connection().await?));

        let helper = Arc::new(Mutex::new(SqlHelper::new()));

        let total = filings.len();
        let i = Arc::new(Mutex::new(0));

        let stream = stream::iter(filings);

        stream
            .for_each_concurrent(10, |trans| {
                let helper = Arc::clone(&helper);
                let i = Arc::clone(&i);
                let client = Arc::clone(&client);

                async move {
                    let mut helper = helper.lock().unwrap();
                    let mut client = client.lock().unwrap();

                    let issuer = helper.create_issuer(&mut client, trans).await.ok();
                    let ind = helper.create_individual(&mut client, trans).await.ok();

                    if ind.is_none() || issuer.is_none() {
                        let mut progress = i.lock().unwrap();
                        *progress += 1;
                        println!("failed insert {}/{}", *progress, total);
                        return;
                    }

                    let form_id = helper
                        .create_form(&mut client, trans, issuer.unwrap())
                        .await;
                    if let Ok(form_id) = form_id {
                        let result = helper
                            .insert_nonderiv(
                                &mut client,
                                trans,
                                form_id,
                                issuer.unwrap(),
                                ind.unwrap(),
                            )
                            .await;

                        if result.is_err() {
                            println!("Error occurred adding transaction for form ID: {}", form_id);
                        }
                    }

                    let mut progress = i.lock().unwrap();
                    *progress += 1;
                    println!("insert {}/{}", *progress, total);
                }
            })
            .await;

        Ok(())
    }

    pub async fn run(&mut self, batch: usize) {
        if batch > 10 {
            panic!("Due to SEC limits, batch per second must be <= 10");
        }

        if self.mine_date > Self::yesterday() {
            println!("{} is today... waiting for that to change", self.mine_date);

            let min_delay = Duration::from_secs(60);
            tokio::time::sleep(min_delay).await;

            return;
        }

        let db = Arc::new(Mutex::new(Vec::<FilingTransaction>::new()));

        // check for json file saved previously
        let path = self.file_path();
        let existing = Path::new(&path).exists();
        if existing {
            let file = File::open(&path).unwrap();
            let rdr = BufReader::new(file);

            let filings: Result<Vec<FilingTransaction>, serde_json::Error> = serde_json::from_reader(rdr);
            if let Ok(filings) = filings {
                println!("Inserting from previously saved file {path}");
                Self::save_filings_db(&filings)
                    .await
                    .expect("Error saving to db");

                self.increment_day();
                return;
            }
        }

        let body = get_daily_entries(self.mine_date).await.unwrap();

        if body.is_empty() {
            println!("Skip day {} index empty", self.mine_date);
            self.increment_day();
            return;
        }

        let second_delay = Duration::from_secs(1);

        let mut skip = 0;
        let total = body.len() / batch;
        for i in 0..total {
            println!("Get {i}/{total}");

            process_entries(&body, db.clone(), skip, batch).await.unwrap();

            skip += batch;

            tokio::time::sleep(second_delay).await;
        }

        let filings = db.lock().unwrap();

        self.save_filings_json(&filings);

        Self::save_filings_db(&filings).await.expect("Should have saved to local file and db");

        self.increment_day();
    }
}
