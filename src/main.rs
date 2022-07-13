use std::env::{current_dir};
use std::path::{Path};
// use std::vec::{Vec};

mod domain;
mod data;
mod infra;

// use crate::domain::{Operation};
use crate::data::usecases::vendors::nuinvest::{NuInvestParser};

// data - concrete methods and entities
// domain - use_cases and entities
// infra - external services communications
// main - composition root
// presentation - user interface (UI/CLI)

fn main() -> std::io::Result<()> {
  let base_dir = current_dir()?.display().to_string();
  let data_directory = Path::new(&base_dir).join("data");
  let nuinvest_data = &data_directory.join("nuinvest");

  let parser: NuInvestParser = NuInvestParser::new(format!("{}", nuinvest_data.display()));
  let operations = parser.parse_files().unwrap();

  for operation in operations {
    println!("{:?}", operation);
  }

  Ok(())
}
