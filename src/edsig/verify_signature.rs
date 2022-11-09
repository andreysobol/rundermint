use ed25519_dalek::{Verifier, PublicKey, Signature};

/*pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<(), ed25519_dalek::SignatureError> {
    let public_key = ed25519_dalek::PublicKey::from_bytes(public_key)?;
    let signature = ed25519_dalek::Signature::from_bytes(signature)?;
    public_key.verify(message, &signature)
}*/

pub fn verify_signature(
    message: Vec<u8>,
    signature: Signature,
    public_key: PublicKey,
) -> bool {
    let result = public_key.verify(&message[..], &signature);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
