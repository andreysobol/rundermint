use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};

/*pub fn sign_message(message: &[u8], secret_key: &[u8]) -> Signature {
    let secret_key = ed25519_dalek::SecretKey::from_bytes(secret_key).unwrap();
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    let signature = secret_key.sign(message);
    signature
}*/

pub fn sign_message(message: &[u8], secret_key: SecretKey) -> Signature {
    let public_key = PublicKey::from(&secret_key);
    let keypair = Keypair {
        public: public_key,
        secret: secret_key,
    };

    keypair.sign(message)
}

#[cfg(test)]
mod tests {

    use super::*;
    use ed25519_dalek::Verifier;
    use rand::rngs::OsRng;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_sign_message() {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let secret_key = keypair.secret;
        let public_key = keypair.public;

        let pre_message = "hello world";
        let mut hasher = Sha256::new();
        hasher.update(pre_message.as_bytes());
        let message = hasher.finalize().to_vec();

        let signature = sign_message(&message[..], secret_key);

        let is_valid = public_key.verify(&message[..], &signature).is_ok();
        assert_eq!(is_valid, true);
    }
}
