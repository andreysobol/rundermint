use sha2::{Sha256, Digest};
use crate::statemachine::state::State;

#[derive(Clone)]
pub struct StateTransition {
    pub data: Vec<u8>,
}

impl StateTransition {
    pub fn new(data: Vec<u8>) -> StateTransition {
        StateTransition {
            data,
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_data_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let result = hasher.finalize();
        result.to_vec()
    }

    pub fn apply_state_transition(&self, state: &State) -> State {

        let prev_state_hash = state.get_state_hash();
        let data_hash = self.get_data_hash();

        let mut result: Vec<u8> = prev_state_hash;
        result.extend(data_hash);

        // make our state machine slower
        for _ in 0..1000 {
            let mut hasher = Sha256::new();
            hasher.update(result);
            result = hasher.finalize().to_vec();
        }

        State {
            state_hash: result,
        }
    }
}