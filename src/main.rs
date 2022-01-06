use dotenv;
use lib::{error::RelayServerError, relayer::Relayer};
use std::error::Error;
use std::fs;
use ethers::prelude::*;

#[derive(Debug)]
struct Config {
    private_key: String,
    rpc_url: String,
}

impl Config {
    fn load_config() -> Result<Config, RelayServerError> {
        fs::metadata(".env").map_err(|e| RelayServerError::EnvFileNotFound)?;

        let private_key = dotenv::var("PK").map_err(|e| RelayServerError::InvalidPK)?;

        let rpc_url = dotenv::var("RPC_URL").map_err(|e| RelayServerError::InvalidRpcUrl)?;

        Ok(Config {
            private_key,
            rpc_url,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load_config()?;
    let provider = Relayer::<Provider<Http>>::provider(config.rpc_url)?;
    let relayer = Relayer::new(config.private_key, provider);
   
    println!("{:?}", relayer);

    Ok(())
}
