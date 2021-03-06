// use walkdir::{WalkDir};
use chrono::{NaiveDate};
use serde::Deserialize;
use std::error::{Error};
use std::vec::{Vec};

pub mod usecases;

#[derive(Clone, Debug, Deserialize)]
pub enum OperationType {
  Buy(String),
  Sell(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Operation {
  pub negotiation_date: NaiveDate,
  pub asset_name: String,
  pub trading_value: f64,
  pub quantity: u64,
  #[serde(flatten)]
  pub operation_type: OperationType,
}

pub trait Parser {
  fn parse_file(&self, path: String) -> Result<Vec<Operation>, Box<dyn Error>>;
}

// pub struct DirParser {
  // fn parse(&self) -> Result<Vec<Operation>, Box<dyn Error>> {
    // let mut operations: Vec<Operation> = Vec::new();
    // for file in WalkDir::new(&self.root_dir).into_iter().filter_map(|file| file.ok()) {
//
      // for parser in &self.parsers {
        // For each parser traverse through eatch directory finding files
        // let mut ops = parser.parse_file();
        // operations.append()
      // }
    // }
//
    // Ok(operations)
  // }
// }
