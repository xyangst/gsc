use std::io;

use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    IO(io::Error),
}

impl std::error::Error for Error {}
// impl std::error::Try for Error {}
