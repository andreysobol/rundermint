use ed25519_dalek::{Signature, PublicKey};
use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct Precommit {
    pub proposal_hash: Vec<u8>,
    pub voter: PublicKey,
    pub signature: Signature,
}

impl Precommit {

    pub fn body_to_vec_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = "precommit".as_bytes().to_vec();
        result.extend(&self.proposal_hash);
        result
    }

    pub fn body_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.body_to_vec_bytes());
        
        hasher.finalize().to_vec()
    }

}