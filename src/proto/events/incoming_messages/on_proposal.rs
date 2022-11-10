use crate::proto::consensus_state::ConsensusState;
use crate::messages::proposal::Proposal;

pub fn on_proposal(
    consensus_state: ConsensusState,
    proposal: Proposal,
) -> ConsensusState {
    consensus_state
}