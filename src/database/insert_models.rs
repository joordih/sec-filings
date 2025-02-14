use crate::secgov::models::FilingTransaction;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;

pub struct NewIssuer<'a> {
    pub issuer_name: &'a str,
    pub issuer_symbol: &'a str,
    pub cik: &'a str,
}

impl NewIssuer<'_> {
    pub fn map(filing: &FilingTransaction) -> NewIssuer {
        NewIssuer {
            issuer_name: &filing.company,
            issuer_symbol: &filing.symbol,
            cik: &filing.company_cik,
        }
    }
}

pub struct NewIndividual<'a> {
    pub full_name: String,
    pub cik: &'a str,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl NewIndividual<'_> {
    pub fn map(filing: &FilingTransaction) -> NewIndividual {
        let split: Vec<_> = filing.owner.split(" ")
            .map(|c| c.to_string())
            .collect();

        if split.len() >= 2 {
            let last_name = Some(split[0].clone());
            let first_name = Some(split[1..split.len()].join(" "));

            NewIndividual {
                full_name: filing.owner.to_string(),
                cik: &filing.owner_cik,
                first_name,
                last_name,
            }
        } else {
            NewIndividual {
                full_name: filing.owner.to_string(),
                cik: &filing.owner_cik,
                first_name: None,
                last_name: None,
            }
        }
    }
}

pub struct NewForm {
    pub issuer_id: i32,
    pub date_reported: NaiveDate,
    pub form_type: String,
    pub txt_url: String,
    pub web_url: String,
    pub access_no: String,
}

impl NewForm {
    pub fn map(filing: &FilingTransaction, issuer_id: i32) -> NewForm {
        NewForm {
            issuer_id,
            date_reported: filing.form_date,
            form_type: filing.form_type.to_string(),
            txt_url: filing.form_url.to_string(),
            web_url: filing.web_url.to_string(),
            access_no: filing.access_no.to_string(),
        }
    }
}

pub struct NewNonDerivTransaction {
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

impl NewNonDerivTransaction {
    pub fn map(filing: &FilingTransaction, form_id: i64, issuer_id: i32, individual_id: i32) -> NewNonDerivTransaction {
        let relationships = filing.relationship.iter()
            .map(|r| *r as i32)
            .collect();

        NewNonDerivTransaction {
            date_reported: filing.trans_date,
            form_id,
            issuer_id,
            individual_id,
            action_code: Some(filing.action_code.clone()),
            ownership_code: Some(filing.ownership_code.clone()),
            transaction_code: Some(filing.trans_code.clone()),
            shares_balance: BigDecimal::from_f32(filing.shares_owned).unwrap(),
            shares_traded: BigDecimal::from_f32(filing.shares_traded).unwrap(),
            avg_price: BigDecimal::from_f32(filing.avg_price).unwrap(),
            amount: BigDecimal::from_f32(filing.amount).unwrap(),
            relationships,
        }
    }
}