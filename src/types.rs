use std::{error::Error, fmt};

#[derive(Debug)]
pub struct MetaNotFoundError;

impl Error for MetaNotFoundError {}

impl fmt::Display for MetaNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ERROR] One or more files that you are trying to share was not found in metadata\nFirst, check if the file exist in the current folder\nThen, run 'drive update' and try againg")
    }
}
