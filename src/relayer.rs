use ethers::{prelude::*, utils::Ganache};
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

    pub fn signer(&self) -> &impl Signer {
        self.client.signer()
    }

    pub fn verify_signature(
        sig: &Signature,
        address: H160,
        message: RecoveryMessage,
    ) -> Result<(), SignatureError> {
        sig.verify(message, address)
    }

    pub fn new_signature(r: &str, s: &str, v: u64) -> Result<Signature, Box<dyn Error>> {
        let r = U256::from_str_radix(r, 16)?;
        let s = U256::from_str_radix(s, 16)?;

        Ok(Signature { r, s, v })
    }

    pub fn new_recovery_message(hash_struct: &str) -> RecoveryMessage {
        let hash_struct = hash_struct.as_bytes();
        RecoveryMessage::Hash(H256::from_slice(hash_struct))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PK: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    const PK_ADDRESS: &str = "0x63f9725f107358c9115bc9d86c72dd5823e9b1e6";
    const BAD_PK: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72";

    const RPC_URL: &str = "https://ropsten.infura.io/v3/c3db5ca5240b4adfbc0bd227b9f3c77a";

    const VALID_SIG: &str = "57fffebf28f8ec6a40afa4a22ec4b681d3836ff240a6383e6d22a4c30ebb93ce337957898669ca77859d560caff24a5c8f84b54ed0a1fbb102dddb5f59d740971c";

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

    #[test]
    fn it_returns_signer() {
        let relayer = relayer(PK.to_string()).unwrap();
        let address = H160::from_slice(PK_ADDRESS.as_bytes());
        assert_eq!(relayer.signer().address(), address);
    }

    #[test]
    fn it_verifies_signature() {
        let signature = Relayer::<Provider<Http>>::new_signature(
            "0x57fffebf28f8ec6a40afa4a22ec4b681d3836ff240a6383e6d22a4c30ebb93ce",
            "0x337957898669ca77859d560caff24a5c8f84b54ed0a1fbb102dddb5f59d74097",
            28,
        )
        .unwrap();

        assert_eq!(VALID_SIG.to_string(), signature.to_string());
    }
}
