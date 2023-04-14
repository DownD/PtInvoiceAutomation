use std::collections::HashMap;
use chrono::{NaiveDate};

use std::str::FromStr;
use super::super::qr_code::QRCode;
use super::Invoice;

#[derive(thiserror::Error, Debug)]
pub enum InvoiceParsingError {
    #[error("The QR code does not have the field {0}")]
    ErrorMissingKeyFromQRHeaderTable(String),
    #[error("{0}")]
    ErrorParsingQRCode(String),
}

fn get_field_error<T: FromStr>(data: &HashMap<String,String>, field: &str ) -> Result<T, InvoiceParsingError> {
    let str_value = data.get(field).ok_or(InvoiceParsingError::ErrorParsingQRCode(format!("Could not find {} in QR code", field)))?;
    
    str_value.parse::<T>().map_err(|_| InvoiceParsingError::ErrorParsingQRCode(format!("Error parsing '{}' for field {}", str_value, field)))
}

pub struct InvoiceQR {
    atcud: String,
    invoice_number: String,
    emission_date: NaiveDate,
    total_price: f64,
}

impl InvoiceQR{
    pub fn new(qr_code: Box<QRCode>) -> Result<InvoiceQR, InvoiceParsingError>{
        let data = qr_code.get_data();

        let tmp_map_result: Result<HashMap<String,String>, InvoiceParsingError> = data
            .split('*')
            .map(|s| {
                let idx_split = s.find(":").ok_or(InvoiceParsingError::ErrorMissingKeyFromQRHeaderTable(s.to_string()))?;
                let (key,val) = s.split_at(idx_split);

                return Ok((key.to_string(), val[1..].to_string()))
            })
            .collect();
        let tmp_map = tmp_map_result?;
        
        let emission_date_str = get_field_error::<String>(&tmp_map, "F")?; 
        let emission_date = NaiveDate::parse_from_str(emission_date_str.as_str(),"%Y%m%d").map_err(|_| InvoiceParsingError::ErrorParsingQRCode(format!("'{}' for field {}", emission_date_str, "F")))?;
        return Ok(InvoiceQR{            
            atcud: get_field_error::<String>(&tmp_map, "H")?,
            invoice_number: get_field_error::<String>(&tmp_map, "G")?,
            emission_date: emission_date,
            total_price: get_field_error::<f64>(&tmp_map, "O")?,
        });
    }

}

impl Invoice for InvoiceQR {
    fn get_id(&self) -> &str {
        return self.invoice_number.as_str();
    }

    fn get_price(&self) -> f64 {
        return self.total_price;
    }

    fn get_emission_date(&self) -> NaiveDate {
        return self.emission_date;
    }

    fn get_atcud(&self) -> &str {
        return self.atcud.as_str();
    }
}