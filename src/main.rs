extern crate walkdir;

use chrono::{NaiveDate};
use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::error::Error;
use std::vec::{Vec};
use walkdir::{WalkDir};

pub mod domain;
mod data;
mod infra;

use crate::infra::serializers::{brazilian_date, brazilian_float};
use crate::domain::{Operation, OperationType};

struct NuInvestParser {}

impl Parser for NuInvestParser {
  fn parse_files() -> Result<Vec<Operation>, Box<dyn Error>> {
    let mut operations: Vec<Operation> = Vec::new();

    for file in WalkDir::new("./data/nuinvest").into_iter().filter_map(|file| file.ok()) {
      if file.metadata().unwrap().is_file() {
        let mut reader = ReaderBuilder::new()
          .comment(Some(b'#'))
          .delimiter(b';')
          .flexible(false)
          .has_headers(true)
          .trim(Trim::All)
          .quote(b'"')
          .from_path(file.path())?;

        for result in reader.deserialize() {
          let record: NuInvestOperation = result?;
          operations.push(Operation {
            negotiation_date: record.negotiation_date,
            asset_name: record.asset_name,
            trading_value: record.trading_value,
            quantity: record.quantity,
            operation_type: if record.buy_amount > 0.0 {
              OperationType::Buy(String::from("C"))
            } else {
              OperationType::Sell(String::from("V"))
            },
          });
        }
      }
    }

    Ok(operations)
  }
}

// data - concrete methods and entities
// domain - use_cases and entities
// infra - external services communications
// main - composition root
// presentation - user interface (UI/CLI)

fn main() {
  println!("{:?}", NuInvestParser::parse_files());
}
