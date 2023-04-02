use chrono::NaiveDate;
use std::{collections::HashMap};
use thiserror;
use std::str::FromStr;
use std::any::type_name;

use super::qr_code::QRCode;



#[derive(thiserror::Error, Debug)]
pub enum InvoiceParsingError {
    #[error("The QR code does not produce a key value:pair for all the tokens")]
    ErrorProduceKeyValue,
    #[error("The QR code does not have the field {0}:")]
    ErrorMissingKeyInQR(String),
    #[error("Could not convert the value {0} to type {1}")]
    ErrorParsingType(String, String),
}

pub struct InvoiceQR {
    pub nif_emissor: u64,
    pub nif_comprador: u64,
    pub pais_comprador: String,
    pub documento_tipo: String,    //Usually FT
    pub documento_estado: String,    //N?
    pub documento_data: NaiveDate, //#YYYYMMDD
    pub documento_numero: String,
    pub atcud: String,
    pub espaco_fiscal: String,
    pub total_impostos: f64,
    pub total: f64,
    pub hash_suffix: String,
    pub cert_numero: u64,
    pub info_adicional: Option<String>,
}

impl InvoiceQR {
    pub fn parse_from_qr(qr_code: QRCode) -> Result<Self, InvoiceParsingError> {
        let data = qr_code.data;

        /*
        No way to convert Optional to Result and return immediately

        let tmp_map: HashMap<&str, &str> = data
            .split('*')
            .map(|s| s.split_at(s.find(":")))
            .map(|(key, val)| (key, &val[1..]))
            .collect();*/

        let mut tmp_map: HashMap<&str, &str> = HashMap::new();

        for pair in data.split('*') {
            let mut parts = pair.split(':');

            let key = parts.next().ok_or(InvoiceParsingError::ErrorProduceKeyValue)?;
            let value = parts.next().ok_or(InvoiceParsingError::ErrorProduceKeyValue)?;

            tmp_map.insert(key, value);
        }
        
        return Ok(Self {
            nif_emissor: InvoiceQR::get_hmap::<u64>(&tmp_map, "A")?,
            nif_comprador:InvoiceQR::get_hmap::<u64>(&tmp_map, "B")?,
            pais_comprador: InvoiceQR::get_hmap::<String>(&tmp_map, "C")?,
            documento_tipo: InvoiceQR::get_hmap::<String>(&tmp_map, "D")?,
            documento_estado: InvoiceQR::get_hmap::<String>(&tmp_map, "E")?,
            documento_data: InvoiceQR::get_date_hmap(&tmp_map, "F")?,
            documento_numero: InvoiceQR::get_hmap::<String>(&tmp_map, "G")?,
            atcud: InvoiceQR::get_hmap::<String>(&tmp_map, "H")?,
            espaco_fiscal: InvoiceQR::get_hmap::<String>(&tmp_map, "I1")?,
            total_impostos: InvoiceQR::get_hmap::<f64>(&tmp_map, "N")?,
            total: InvoiceQR::get_hmap::<f64>(&tmp_map, "O")?,
            hash_suffix: InvoiceQR::get_hmap::<String>(&tmp_map, "Q")?,
            cert_numero: InvoiceQR::get_hmap::<u64>(&tmp_map, "R")?,
            info_adicional: match tmp_map.get("S") {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        });
    }

    fn get_date_hmap(hmap: &HashMap<&str, &str>, key: &str) -> Result<NaiveDate, InvoiceParsingError> {
        let val = InvoiceQR::get_hmap::<String>(&hmap, key)?;
        match NaiveDate::parse_from_str(&val, "%Y%m%d"){
            Ok(u) => Ok(u),
            Err(e) => Err(InvoiceParsingError::ErrorParsingType(val.to_string(), type_name::<NaiveDate>().to_string()))
        }
    }

    fn get_hmap<T: FromStr>(hmap: &HashMap<&str, &str>, key: &str) -> Result<T, InvoiceParsingError> {
        let val = match hmap.get(key) {
            Some(s) => s,
            None => return Err(InvoiceParsingError::ErrorMissingKeyInQR(key.to_string())),
        };

        match val.parse::<T>(){
            Ok(u) => Ok(u),
            Err(e) => Err(InvoiceParsingError::ErrorParsingType(val.to_string(), type_name::<T>().to_string()))
        }
    }
}
