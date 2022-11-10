use crate::messages::proposal::Proposal;
use crate::proto::consensus_state::ConsensusState;

pub fn on_proposal(
    consensus_state: ConsensusState,
    proposal: Proposal,
) -> ConsensusState {
    consensus_state
}