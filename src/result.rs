use std::result::Result as StdResult;

use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
