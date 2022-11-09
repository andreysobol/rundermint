pub struct State {
    pub state_hash: Vec<u8>,
}

impl State {
    pub fn new() -> State {
        State {
            state_hash: vec![],
        }
    }

    pub fn get_state_hash(&self) -> Vec<u8> {
        self.state_hash.clone()
    }
}