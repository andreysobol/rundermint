use crate::statemachine::statetransition::StateTransition;
use crate::types::{Round, Height};
use ed25519_dalek::{Signature, PublicKey};
use ed25519_dalek::{SIGNATURE_LENGTH};
use sha2::{Sha256, Digest};
use crate::messages::proof_of_lock::ProofOfLock;

#[derive(Clone)]
pub struct Proposal {
    // mb id or pk or proposer
    pub proposer: PublicKey,
    pub round: Round,
    pub height: Height,
    pub state_transition: StateTransition,
    //pub proof_of_lock: ProofOfLock,
    //add here signature
    pub signature: Signature,
}

impl Proposal {
    pub fn body_to_vec_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = self.round.to_be_bytes().to_vec();
        result.extend(self.height.to_be_bytes().to_vec());
        result.extend(self.state_transition.get_data());
        result
    }

    pub fn body_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.body_to_vec_bytes());
        let result = hasher.finalize().to_vec();
        result
    }

}


/*impl Proposal {
    pub fn new(
        round: Round,
        height: Height,
        state_transition: StateTransition,
        proof_of_lock: ProofOfLock,
        signature: Signature,
    ) -> Proposal {
        Proposal {
            round: round,
            height: height,
            state_transition: state_transition,
            proof_of_lock: proof_of_lock,
            signature: signature,
        }
    }

    pub fn to_vec_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.round.to_be_bytes().to_vec());
        result.extend(self.height.to_be_bytes().to_vec());
        result.extend(self.state_transition.get_data());
        result.extend(self.proof_of_lock.clone());
        result.extend(self.signature.to_bytes());
        result
    }

    pub fn from_vec_bytes(data: Vec<u8>) -> Proposal {
        let round = u64::from_be_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
        let height = u64::from_be_bytes([data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]]);
        let state_transition = StateTransition::new(data[16..].to_vec());
        let signature:  Signature = Signature::try_from(&data[..]).unwrap();
        // fix last line
        let proof_of_lock = vec![];
        Proposal {
            round: round,
            height: height,
            state_transition: state_transition,
            proof_of_lock: proof_of_lock,
            signature: signature,
        }
    }
}*/