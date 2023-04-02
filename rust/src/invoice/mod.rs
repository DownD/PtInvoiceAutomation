use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use chrono::{NaiveDate};

pub mod invoice_qr;
pub mod invoice_manager;
pub mod subset_problem;

pub type InvoiceMappingTable = HashMap<String, String>;

pub const INVOICE_MAPPING_JSON_PATH: &str = "name_mapping.json";

pub trait Invoice{
    fn get_id(&self) -> &str;
    fn get_price(&self) -> f64;
    fn get_emission_date(&self) -> NaiveDate;
    fn get_atcud(&self) -> &str;
}

impl PartialEq for dyn Invoice + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for dyn Invoice + '_ {}

impl Hash for dyn Invoice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}