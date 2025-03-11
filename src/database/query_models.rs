use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use std::str::FromStr;
use tiberius::Row;

#[derive(Debug)]
pub struct Form {
    pub form_id: i64,
    pub issuer_id: i32,
    pub date_reported: NaiveDate,
    pub form_type: String,
    pub txt_url: String,
    pub access_no: String,
    pub web_url: String,
}

impl Form {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Form {
            form_id: row.get::<i64, _>("form_id").unwrap(),
            issuer_id: row.get::<i32, _>("issuer_id").unwrap(),
            date_reported: NaiveDate::from_str(row.get::<&str, _>("date_reported").unwrap()).unwrap(),
            form_type: row.get::<&str, _>("form_type").unwrap().to_string(),
            txt_url: row.get::<&str, _>("txt_url").unwrap().to_string(),
            access_no: row.get::<&str, _>("access_no").unwrap().to_string(),
            web_url: row.get::<&str, _>("web_url").unwrap().to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Individual {
    pub individual_id: i32,
    pub cik: String,
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Individual {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Individual {
            individual_id: row.get::<i32, _>("individual_id").unwrap(),
            cik: row.get::<&str, _>("cik").unwrap().to_string(),
            full_name: row.get::<&str, _>("full_name").unwrap().to_string(),
            first_name: Option::from(row.get::<&str, _>("first_name").unwrap().to_string()),
            last_name: Option::from(row.get::<&str, _>("last_name").unwrap().to_string()),
        })
    }
}

#[derive(Debug)]
pub struct Issuer {
    pub issuer_id: i32,
    pub name: String,
    pub symbol: String,
    pub cik: String,
}

impl Issuer {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Issuer {
            issuer_id: row.get::<i32, _>("issuer_id").unwrap(),
            name: row.get::<&str, _>("name").unwrap().to_string(),
            symbol: row.get::<&str, _>("symbol").unwrap().to_string(),
            cik: row.get::<&str, _>("cik").unwrap().to_string(),
        })
    }
}

#[derive(Debug)]
pub struct NonDerivTransaction {
    pub transaction_id: i64,
    pub date_reported: NaiveDate,
    pub form_id: i64,
    pub issuer_id: i32,
    pub individual_id: i32,
    pub action_code: Option<String>,
    pub ownership_code: Option<String>,
    pub transaction_code: Option<String>,
    pub shares_balance: BigDecimal,
    pub shares_traded: BigDecimal,
    pub avg_price: BigDecimal,
    pub amount: BigDecimal,
    pub relationships: Vec<i32>,
}

impl NonDerivTransaction {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>> {
        let shares_balance = BigDecimal::from_str(&row.get::<f64, _>("shares_balance").unwrap().to_string())?;
        let shares_traded = BigDecimal::from_str(&row.get::<f64, _>("shares_traded").unwrap().to_string())?;
        let avg_price = BigDecimal::from_str(&row.get::<f64, _>("avg_price").unwrap().to_string())?;
        let amount = BigDecimal::from_str(&row.get::<f64, _>("amount").unwrap().to_string())?;
        let relationships_str: String = row.get::<&str, _>("relationships").unwrap().to_string();

        let relationships: Vec<i32> = relationships_str
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        Ok(NonDerivTransaction {
            transaction_id: row.get::<i64, _>("transaction_id").unwrap(),
            date_reported: NaiveDate::from_str(row.get::<&str, _>("date_reported").unwrap()).unwrap(),
            form_id: row.get::<i64, _>("form_id").unwrap(),
            issuer_id: row.get::<i32, _>("issuer_id").unwrap(),
            individual_id: row.get::<i32, _>("individual_id").unwrap(),
            action_code: Option::from(row.get::<&str, _>("action_code").unwrap().to_string()),
            ownership_code: Option::from(row.get::<&str, _>("ownership_code").unwrap().to_string()),
            transaction_code: Option::from(row.get::<&str, _>("transaction_code").unwrap().to_string()),
            shares_balance,
            shares_traded,
            avg_price,
            amount,
            relationships,
        })
    }
}

