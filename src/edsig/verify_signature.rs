use ed25519_dalek::{PublicKey, Signature, Verifier};

/*pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<(), ed25519_dalek::SignatureError> {
    let public_key = ed25519_dalek::PublicKey::from_bytes(public_key)?;
    let signature = ed25519_dalek::Signature::from_bytes(signature)?;
    public_key.verify(message, &signature)
}*/

pub fn verify_signature(message: Vec<u8>, signature: Signature, public_key: PublicKey) -> bool {
    let result = public_key.verify(&message[..], &signature);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    use ed25519_dalek::{Keypair, Signer};

    use rand::rngs::OsRng;

    use sha2::{Digest, Sha256};

    #[test]

    fn test_verify_signature() {
        let mut csprng = OsRng {};

        let keypair: Keypair = Keypair::generate(&mut csprng);

        let public_key = keypair.public;

        let pre_message = "hello world";

        let mut hasher = Sha256::new();

        hasher.update(pre_message.as_bytes());

        let message = hasher.finalize().to_vec();

        let signature = keypair.sign(&message[..]);

        let is_valid = verify_signature(message, signature, public_key);

        assert_eq!(is_valid, true);
    }
}
