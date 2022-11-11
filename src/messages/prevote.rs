use ed25519_dalek::{PublicKey, Signature};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Prevote {
    pub proposal_hash: Vec<u8>,
    pub voter: PublicKey,
    pub signature: Signature,
}

impl Prevote {
    pub fn body_to_vec_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = "prevote".as_bytes().to_vec();
        result.extend(&self.proposal_hash);
        result
    }

    pub fn body_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.body_to_vec_bytes());

        hasher.finalize().to_vec()
    }
}
