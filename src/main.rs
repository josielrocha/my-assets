extern crate walkdir;
use chrono::{NaiveDate};
use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::error::Error;
use std::vec::{Vec};
use walkdir::{WalkDir};

#[derive(Debug, Deserialize)]
enum OperationType {
  Buy(String),
  Sell(String),
}

#[derive(Debug, Deserialize)]
pub struct Operation {
  negotiation_date: NaiveDate,
  asset_name: String,
  trading_value: f64,
  quantity: u64,
  #[serde(flatten)]
  operation_type: OperationType,
}

// 02/09/2021;266253;MXRF11;9,94;6;0;59,64;0,00
#[derive(Debug, Deserialize)]
pub struct NuInvestOperation {
  #[serde(rename="Dt. Negociação", with="brazilian_date")]
  negotiation_date: NaiveDate,
  #[serde(rename="Conta")]
  account: u32,
  #[serde(rename="Ativo")]
  asset_name: String,
  #[serde(rename="Preço", with="brazilian_float")]
  trading_value: f64,
  #[serde(rename="Quantidade Compra")]
  quantity: u64,
  #[serde(rename="Financeiro Compra", with="brazilian_float")]
  buy_amount: f64,
  #[serde(rename="Financeiro Venda", with="brazilian_float")]
  sell_amount: f64
}

mod brazilian_float {
  use serde::{Deserialize, Serializer, Deserializer};

  pub fn serialize<S>(
    value: &f64,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = value.to_string().replace(".", ",");
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<f64, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let n: f64 = s.trim().replace(",", ".").parse().unwrap();
    Ok(n)
  }
}

mod brazilian_date {
  use chrono::{NaiveDate};
  use serde::{Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%d/%m/%Y";

  pub fn serialize<S>(
    date: &NaiveDate,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<NaiveDate, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
  }
}

pub trait Parser {
  fn parse_files() -> Result<Vec<Operation>, Box<dyn Error>>;
}

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

    println!("{:?}", operations);

    Ok(operations)
  }
}

// data - concrete methods and entities
// domain - use_cases and entities
// infra - external services communications
// main - composition root
// presentation - user interface (UI/CLI)

fn main() {
  // let operations: Vec<Operation> = NuInvestParser::parse_files();
  println!("{:?}", NuInvestParser::parse_files());
}
