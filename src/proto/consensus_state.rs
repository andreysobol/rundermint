use crate::types::{Round, Height};
use crate::statemachine::state::State;
use crate::messages::proof_of_lock::ProofOfLock;
use ed25519_dalek::PublicKey;

#[derive(Clone)]
pub struct ConsensusState {
    pub round: Round,
    pub height: Height,
    pub state: State,
    pub locked_state: Option<State>,
    pub validators: Vec<PublicKey>,
    pub threshold: usize,
    pub proof_of_lock: Option<ProofOfLock>,
}