extern crate walkdir;

use std::env::{current_dir};
use std::path::{Path};

use crate::domain::{DirParser};
use crate::data::usecases::vendors::nuinvest::{NuInvestParser};

mod domain;
mod data;
mod infra;

// data - concrete methods and entities
// domain - use_cases and entities
// infra - external services communications
// main - composition root
// presentation - user interface (UI/CLI)

fn main() -> std::io::Result<()> {
  let base_dir = current_dir()?.display().to_string();
  let data_directory = Path::new(&base_dir).join("data");
  let data_dir_as_string = format!("{}", data_directory.display());

  let parsers = vec![NuInvestParser {}];
  let dir_parser = DirParser::new(data_dir_as_string, parsers);

  Ok(())
}
