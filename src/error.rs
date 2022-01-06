use dotenv::*;
use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RelayServerError {
    #[error(".env file not found")]
    EnvFileNotFound,
    #[error("rpc url key not set")]
    InvalidRpcUrl,
    #[error("private key not set")]
    InvalidPK,
}
