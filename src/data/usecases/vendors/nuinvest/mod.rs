use chrono::{NaiveDate};
use csv::{ReaderBuilder, Trim};
use iconv::{iconv, IconvReader};
use serde::{Deserialize};
use std::error::{Error};
use std::fs::{File};
use std::io::{Read, BufReader, BufRead};
use walkdir::{WalkDir};

use crate::infra::serializers::{brazilian_date, brazilian_float};
use crate::domain::{Operation, OperationType, Parser};

// 02/09/2021;266253;MXRF11;9,94;6;0;59,64;0,00
#[derive(Debug, Deserialize)]
struct NuInvestOperation {
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

pub struct NuInvestParser {
  root_dir: String,
}

impl NuInvestParser {
  pub fn new(root_dir: String) -> Self {
    Self {
      root_dir: root_dir
    }
  }

  pub fn parse_files(&self) -> Result<Vec<Operation>, Box<dyn Error>> {
    let mut operations: Vec<Operation> = Vec::new();
    for entry in WalkDir::new(&self.root_dir)
                         .into_iter()
                         .filter_map(|e| e.ok()) {
      if entry.metadata()?.is_file() {
        let ops = &self.parse_file(format!("{}", entry.path().display()))?;
        operations.append(&mut ops.to_vec());
      }
    }

    Ok(operations)
  }
}

impl Parser for NuInvestParser {
  fn parse_file(&self, path: String) -> Result<Vec<Operation>, Box<dyn Error>> {
    println!("\n{}", &path);

    let file = File::open(path).expect("Could not open file");
    let lines = BufReader::new(file)
      .lines()
      .skip(1)
      .map(|x| x.unwrap())
      .collect::<Vec<String>>()
      .join("\n");

    let mut reader = ReaderBuilder::new()
      .comment(Some(b'#'))
      .delimiter(b';')
      .flexible(false)
      .has_headers(true)
      .trim(Trim::All)
      .quote(b'"')
      .from_reader(lines.as_bytes());

    let mut operations: Vec<Operation> = Vec::new();
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

    Ok(operations)
  }
}
