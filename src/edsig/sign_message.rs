use ed25519_dalek::{PublicKey, Signature, SecretKey, Signer, Keypair};

/*pub fn sign_message(message: &[u8], secret_key: &[u8]) -> Signature {
    let secret_key = ed25519_dalek::SecretKey::from_bytes(secret_key).unwrap();
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    let signature = secret_key.sign(message);
    signature
}*/

pub fn sign_message(message: &[u8], secret_key: SecretKey) -> Signature {
    let public_key = PublicKey::from(&secret_key);
    let Keypair = Keypair {
        public: public_key,
        secret: secret_key,
    };
    let signature = Keypair.sign(message);
    signature
}