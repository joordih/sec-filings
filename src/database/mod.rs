use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::database::insert_models::{NewForm, NewIndividual, NewIssuer, NewNonDerivTransaction};
use crate::database::query_models::NonDerivTransaction;
use crate::secgov::models::FilingTransaction;

pub mod insert_models;
pub mod query_models;

static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost\\SQLEXPRESS,1433;database=sec;user id=sa;password=tuputamadre02;".to_owned()
    })
});

pub async fn get_connection() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    let mut config = Config::from_ado_string(&CONN_STR_PORT)?;
    // let mut config = Config::new();
    // config.authentication(AuthMethod::sql_server("sa", "tuputamadre02"));
    // config.host("127.0.0.1");
    // // config.instance_name("SQLEXPRESS");
    // config.port(1433);
    // config.database("sec");

    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}

pub struct SqlHelper {
    issuers_cache: Arc<Mutex<HashMap<String, i32>>>,
    form_cache: Arc<Mutex<HashMap<String, i64>>>,
    ind_cache: Arc<Mutex<HashMap<String, i32>>>,
}

impl SqlHelper {
    pub fn new() -> SqlHelper {
        SqlHelper {
            issuers_cache: Arc::new(Mutex::new(HashMap::new())),
            form_cache: Arc::new(Mutex::new(HashMap::new())),
            ind_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_issuer(&mut self, client: &mut Client<Compat<TcpStream>>, filing: &FilingTransaction) -> Result<i32, Box<dyn std::error::Error>> {
        let new_issuer = NewIssuer::map(&filing);

        let mut cache = self.issuers_cache.lock().unwrap();
        if let Some(&issuer_id) = cache.get(&new_issuer.cik.to_string()) {
            return Ok(issuer_id);
        }

        let query = format!("SELECT issuer_id FROM issuer WHERE cik = '{}'", new_issuer.cik);
        let stream = client.query(query, &[]).await?;

        let insert_query = format!(
            "INSERT INTO issuer (Name, Symbol, cik) VALUES ('{}', '{}', '{}')",
            new_issuer.issuer_name, new_issuer.issuer_symbol, new_issuer.cik
        );

        if let Some(row) = stream.into_row().await? {
            let issuer_id: i32 = row.get("issuer_id").unwrap();
            cache.insert(new_issuer.cik.to_string(), issuer_id);
            return Ok(issuer_id);
        }

        client.execute(insert_query, &[]).await?;

        Err("Failed to create issuer".into())
    }

    pub async fn create_individual(&mut self, client: &mut Client<Compat<TcpStream>>, filing: &FilingTransaction) -> Result<i32, Box<dyn std::error::Error>> {
        let new_ind = NewIndividual::map(&filing);

        let mut cache = self.ind_cache.lock().unwrap();
        if let Some(&individual_id) = cache.get(&new_ind.cik.to_string()) {
            return Ok(individual_id);
        }

        let query = format!("SELECT individual_id FROM individual WHERE cik = '{}'", new_ind.cik);
        let stream = client.query(query, &[]).await?;

        if let Some(row) = stream.into_row().await? {
            let individual_id: i32 = row.get("individual_id").unwrap();
            cache.insert(new_ind.cik.to_string(), individual_id);
            return Ok(individual_id);
        }

        let insert_query = format!(
            "INSERT INTO individual (FullName, cik, FirstName, LastName) VALUES ('{}', '{}', '{}', '{}')",
            new_ind.full_name, new_ind.cik, new_ind.first_name.unwrap_or_default(), new_ind.last_name.unwrap_or_default()
        );

        client.execute(insert_query, &[]).await?;

        Err("Failed to create individual".into())
    }

    pub async fn create_form(&self, client: &mut Client<Compat<TcpStream>>, filing: &FilingTransaction, issuer_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let new_form = NewForm::map(&filing, issuer_id);

        let mut cache = self.form_cache.lock().unwrap();
        if let Some(&form_id) = cache.get(&new_form.access_no) {
            return Ok(form_id);
        }

        let query = format!("SELECT form_id FROM form WHERE AccessNo = '{}'", new_form.access_no);
        let stream = client.query(query, &[]).await?;

        if let Some(row) = stream.into_row().await? {
            let form_id: i64 = row.get("form_id").unwrap();
            cache.insert(new_form.access_no.clone(), form_id);
            return Ok(form_id);
        }

        let insert_query = format!(
            "INSERT INTO form (IssuerId, DateReported, FormType, TxtURL, WebURL, AccessNo) VALUES ({}, '{}', '{}', '{}', '{}', '{}')",
            new_form.issuer_id, new_form.date_reported, new_form.form_type, new_form.txt_url, new_form.web_url, new_form.access_no
        );

        client.execute(insert_query, &[]).await?;

        Err("Failed to create form".into())
    }

    pub async fn insert_nonderiv(&self, client: &mut Client<Compat<TcpStream>>, filing: &FilingTransaction, form_id: i64, issuer_id: i32, ind_id: i32) -> Result<NonDerivTransaction, Box<dyn std::error::Error>> {
        let new_trans = NewNonDerivTransaction::map(filing, form_id, issuer_id, ind_id);

        let query = format!(
            "SELECT * FROM non_deriv_transaction WHERE FormId = {} AND DateReported = '{}' AND SharesBalance = {}",
            form_id, filing.trans_date, new_trans.shares_balance
        );

        let stream = client.query(query, &[]).await?;

        if let Some(row) = stream.into_row().await? {
            return Ok(NonDerivTransaction::from_row(&row)?);
        }

        let insert_query = format!(
            "INSERT INTO non_deriv_transaction (DateReported, FormId, IssuerId, IndividualId, ActionCode, OwnershipCode, TransactionCode, SharesBalance, SharesTraded, AvgPrice, Amount, Relationships) VALUES ('{}', {}, {}, {}, '{}', '{}', '{}', {}, {}, {}, {}, ARRAY[{}])",
            new_trans.date_reported, new_trans.form_id, new_trans.issuer_id, new_trans.individual_id, new_trans.action_code.unwrap_or_default(), new_trans.ownership_code.unwrap_or_default(), new_trans.transaction_code.unwrap_or_default(), new_trans.shares_balance, new_trans.shares_traded, new_trans.avg_price, new_trans.amount, new_trans.relationships.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(",")
        );

        client.execute(insert_query, &[]).await?;

        let query = format!(
            "SELECT * FROM non_deriv_transaction WHERE FormId = {} AND DateReported = '{}' AND SharesBalance = {}",
            form_id, filing.trans_date, new_trans.shares_balance
        );

        let stream = client.query(query, &[]).await?;

        if let Some(row) = stream.into_row().await? {
            return Ok(NonDerivTransaction::from_row(&row)?);
        }

        Err("Failed to insert non-derivative transaction".into())
    }
}