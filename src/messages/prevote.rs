use crate::types::{Round, Height};
use ed25519_dalek::{Signature, PublicKey};

struct Prevote {
    pub proposal_hash: Vec<u8>,
    pub voter: PublicKey,
    pub signature: Signature,
}

impl Prevote {

    pub fn body_to_vec_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = self.proposal_hash;
        result
    }

    pub fn body_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.body_to_vec_bytes());
        let result = hasher.finalize().to_vec();
        result
    }

}