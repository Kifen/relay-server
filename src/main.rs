use dotenv;
use ethers::prelude::*;
use lib::{error::RelayServerError, relayer::Relayer};
use std::error::Error;
use std::fs;

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
    let relayer = Relayer::new(config.private_key, provider)?;

    let signature = Relayer::<Provider<Http>>::new_signature(
        "0x57fffebf28f8ec6a40afa4a22ec4b681d3836ff240a6383e6d22a4c30ebb93ce",
        "0x337957898669ca77859d560caff24a5c8f84b54ed0a1fbb102dddb5f59d74097",
        28,
    )
    .unwrap();
    println!("{}", signature);
    Ok(())
}
