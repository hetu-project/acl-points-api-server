use alloy_primitives::{aliases::B256, keccak256, Address, PrimitiveSignature};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;
use chrono::Utc;
use rand::Rng;

#[derive(Debug)]
pub struct SiweMessage {
    pub domain: String,
    pub address: String,
    pub statement: String,
    pub nonce: String,
    pub issued_at: String,
}

impl SiweMessage {
    fn new(domain: &str, address: &str, statement: &str) -> Self {
        let nonce = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        Self {
            domain: domain.to_string(),
            address: address.to_string(),
            statement: statement.to_string(),
            nonce,
            issued_at: Utc::now().to_rfc3339(),
        }
    }

    fn to_signable_message(&self) -> String {
        format!(
            "{} wants you to sign in with your Ethereum account:\n{}\n\n{}\n\nNonce: {}\nIssued At: {}",
            self.domain,
            self.address,
            self.statement,
            self.nonce,
            self.issued_at
        )
    }

    fn hash(&self) -> B256 {
        let message = self.to_signable_message();
        let prefixed_message =
            format!("\x19Ethereum Signed Message:\n{}", message.len()) + &message;
        keccak256(prefixed_message)
    }

    fn verify_signature_with_prehash(&self, signature: &PrimitiveSignature) -> bool {
        let hash = self.hash();
        match signature.recover_address_from_prehash(&hash) {
            Ok(recovered_address) => recovered_address.to_string() == self.address,
            Err(_) => false,
        }
    }

    fn verify_signature_with_msg(&self, signature: &PrimitiveSignature) -> bool {
        match signature.recover_address_from_msg(self.to_signable_message()) {
            Ok(recovered_address) => recovered_address.to_string() == self.address,
            Err(_) => false,
        }
    }
}

#[test]
fn test_erc4361() {
    let signer = PrivateKeySigner::random();
    let signer = signer.with_chain_id(Some(1));

    let message = SiweMessage::new(
        "example.com",
        &signer.address().to_string(),
        "Sign in to access the app",
    );

    let signable_message = message.to_signable_message();
    println!("Signable Message:\n{}", signable_message);

    let signature = signer
        .sign_message_sync(signable_message.as_bytes())
        .expect("Signing failed");

    let is_valid = message.verify_signature_with_prehash(&signature);
    println!("Signature is valid: {}", is_valid);

    let is_valid = message.verify_signature_with_msg(&signature);
    println!("Signature is valid: {}", is_valid);
}
