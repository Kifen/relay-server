use ethers::{prelude::*, utils::Ganache};
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use url::ParseError;

#[derive(Debug)]
pub struct Relayer<P> {
    client: SignerMiddleware<P, LocalWallet>,
}

impl<P: Middleware> Relayer<P> {
    pub fn new(private_key: String, provider: P) -> Result<Relayer<P>, Box<dyn Error>> {
        let wallet = private_key.parse::<LocalWallet>()?;
        let client = SignerMiddleware::new(provider, wallet);

        Ok(Relayer { client })
    }

    pub fn provider(rpc_url: String) -> Result<Provider<Http>, ParseError> {
        Provider::<Http>::try_from(rpc_url)
    }

    pub fn generate_typed_data(&self) -> Result<(), Box<dyn Error>> {
        let typed_data: Value = serde_json::from_str("./typed_data.json")?;
        println!("{}", typed_data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PK: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    const BAD_PK: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72";

    const RPC_URL: &str = "https://ropsten.infura.io/v3/c3db5ca5240b4adfbc0bd227b9f3c77a";

    fn test_wallet() -> LocalWallet {
        let ganache = Ganache::new().spawn();
        ganache.keys()[0].clone().into()
    }

    fn provider(rpc_url: String) -> Result<Provider<Http>, ParseError> {
        Relayer::<Provider<Http>>::provider(rpc_url)
    }

    fn relayer(pk: String) -> Result<Relayer<Provider<Http>>, Box<dyn Error>> {
        Relayer::new(pk, provider(RPC_URL.to_string()).unwrap())
    }

    #[test]
    fn it_creates_new_relayer() {
        assert_eq!(relayer(PK.to_string()).is_err(), false);
    }

    #[test]
    fn it_errors_creating_relayer_with_invalid_pk() {
        assert_eq!(relayer(BAD_PK.to_string()).is_err(), true);
    }
}
