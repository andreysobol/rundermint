use crate::messages::prevote::Prevote;
use crate::proto::consensus_state::ConsensusState;
use crate::messages::message::Message;

pub fn on_proposal(
    consensus_state: ConsensusState,
    prevote: Prevote,
) -> (ConsensusState, Option<Message>) {
    (consensus_state, None)
}