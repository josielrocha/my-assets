use std::vec::{Vec};
use std::error::{Error};
use super::{Operation};

pub trait ParseFilesFromDirectory {
    fn parse_files_from_dir(dirpath: String) -> Result<Vec<Operation>, Box<dyn Error>>;
}
