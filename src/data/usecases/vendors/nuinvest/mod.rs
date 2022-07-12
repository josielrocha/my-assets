use chrono::{NaiveDate};
use csv::{ReaderBuilder, Trim};
use serde::{Deserialize};
use std::error::{Error};
use std::fs::{File};

use crate::infra::serializers::{brazilian_date, brazilian_float};
use crate::domain::{Operation, OperationType, Parser};

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

pub struct NuInvestParser;

impl Parser for NuInvestParser {
  fn parse_file(&self, path: String) -> Result<Vec<Operation>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut operations: Vec<Operation> = Vec::new();

    let mut reader = ReaderBuilder::new()
      .comment(Some(b'#'))
      .delimiter(b';')
      .flexible(false)
      .has_headers(true)
      .trim(Trim::All)
      .quote(b'"')
      .from_path(path)?;

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
