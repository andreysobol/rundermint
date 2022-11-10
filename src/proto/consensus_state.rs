use crate::types::{Round, Height};
use crate::statemachine::state::State;
use ed25519_dalek::PublicKey;

pub struct ConsensusState {
    pub round: Round,
    pub height: Height,
    pub state: State,
    pub validators: PublicKey,
}