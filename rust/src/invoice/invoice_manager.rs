use std::{borrow::Borrow};
use std::collections::HashMap;

use anyhow::Result;
use super::{InvoiceMappingTable, INVOICE_MAPPING_JSON_PATH, Invoice};
use serde_json;
use super::super::qr_code::QRCode;
use super::invoice_qr::InvoiceQR;
use std::sync::mpsc;
use log::{error, debug};
use crate::invoice::subset_problem::SubsetSolver;
use crate::invoice::subset_problem::greedy_search::GreedySearchSolver;
use std::rc::Rc;

pub struct InvoiceManager {
    invoices: HashMap<String,Rc<dyn Invoice>>,
    name_mapping_table: InvoiceMappingTable,
    invoice_recv: mpsc::Receiver<Box<QRCode>>,

    subset_solver: GreedySearchSolver,
}


impl InvoiceManager  {
    
    pub fn new(invoice_recv : mpsc::Receiver<Box<QRCode>>) -> InvoiceManager{
        let name_mapping_table = InvoiceManager::load_name_mapping_from_file(INVOICE_MAPPING_JSON_PATH);
        return InvoiceManager{invoices: HashMap::new(), name_mapping_table: name_mapping_table, invoice_recv: invoice_recv,
            subset_solver: GreedySearchSolver{}};
    }

    pub fn check_qr_channel(&mut self) -> Result<Option<Rc<dyn Invoice>>> {
        for qr in self.invoice_recv.try_iter() {
            let invoice = Rc::new(InvoiceQR::new(qr)?);
            let curr_invoice_id = invoice.get_id().to_string();

            if self.invoices.contains_key(&curr_invoice_id) {
                debug!("Invoice already exists");
                return Ok(Some(self.get_invoice(&curr_invoice_id).unwrap()));
            }

            debug!("Found new invoice with id: {}, number of invoices saved: {}", curr_invoice_id, self.invoices.len());
            self.invoices.insert(curr_invoice_id.clone(),invoice);
            return Ok(Some(self.get_invoice(&curr_invoice_id).unwrap()));      
        }

        return Ok(None);
    }

    pub fn name_mapping_table(&self) -> &InvoiceMappingTable{
        return self.name_mapping_table.borrow();
    }

    pub fn get_invoices(&self) -> impl Iterator<Item= &Rc<dyn Invoice>> {

        //Get an iterator of the hashmap as a list of invoices
        let invoices = self.invoices.iter().map(|(_,v)| v);
        return invoices
    }

    pub fn get_invoice(&self, invoice_id: &str) -> Option<Rc<dyn Invoice>> {
        return self.invoices.get(invoice_id).map(|x| x.clone());
    }

    pub fn get_best_invoice_match(&self, sum: f64) -> Vec<Rc<dyn Invoice>> {
        let invoices : Vec<&Rc<dyn Invoice>> = self.invoices.iter().map(|(_,v)| v.borrow()).collect();
        let solved = self.subset_solver.solve_vector::<Rc<dyn Invoice>>(invoices.as_slice(), (sum*100.0) as i64, |x| (x.get_price() * 100.0) as i64);
        return solved.iter().map(|x| x.clone().to_owned()).collect();
    }

    fn load_name_mapping_from_file(path: &str) -> InvoiceMappingTable{
        // Load the first file into a string.
        let text = std::fs::read_to_string(path);

        match text {
            Ok(text) => {
                // Parse the string into a dynamically-typed JSON structure.
                let json = serde_json::from_str(&text).unwrap();
                return json;
            },
            Err(_) => {
                error!("Error loading the name mapping file: {}", path);
                return InvoiceMappingTable::new();
            }
        }

    }
}