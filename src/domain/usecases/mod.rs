use std::vec::{Vec};
use std::error::{Error};
use super::{Operation, Parser};

pub trait ParseFilesFromDirectory {
    fn parse_files_from_dir(dirpath: String, parsers: Vec<Parser>) -> Result<Vec<Operation>, Box<dyn Error>>;
}


