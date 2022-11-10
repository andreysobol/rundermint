use crate::proto::consensus_state::ConsensusState;
use crate::messages::proposal::Proposal;

pub fn on_proposal(
    consensus_state: ConsensusState,
    proposal: Proposal,
) -> ConsensusState {
    if consensus_state.proposal.is_none() {
        let mut new_consensus_state = consensus_state.clone();
        new_consensus_state.proposal = Some(proposal);
        new_consensus_state
    } else {
        consensus_state
    }
}