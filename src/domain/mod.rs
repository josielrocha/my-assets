use chrono::{NaiveDate};
use serde::Deserialize;
use std::error::{Error};
use std::vec::{Vec};

pub mod usecases;

#[derive(Debug, Deserialize)]
pub enum OperationType {
  Buy(String),
  Sell(String),
}

#[derive(Debug, Deserialize)]
pub struct Operation {
  pub negotiation_date: NaiveDate,
  pub asset_name: String,
  pub trading_value: f64,
  pub quantity: u64,
  #[serde(flatten)]
  pub operation_type: OperationType,
}

pub trait DirParser {
  fn new(root_dir: String, parsers: Vec<Parser>) -> Self;
  fn parse(&self) -> Result<Vec<Operation>, Box<dyn Error>> {
    let mut operations: Vec<Operation> = Vec::new();

    for parser of parsers {
      // For each parser traverse through eatch directory finding files
      let mut ops = parser::parse_file();
      operations.append()
    }

    operations;
  }
}

pub trait Parser {
  fn parse_file(path: String) -> Result<Vec<Operation>, Box<dyn Error>>;
}
