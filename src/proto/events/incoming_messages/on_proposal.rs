use crate::messages::message::Message;
use crate::proto::consensus_state::ConsensusState;
use crate::messages::proposal::Proposal;
use crate::proto::predicates::proposal_validity::proposal_validty;

pub fn on_proposal(
    consensus_state: ConsensusState,
    proposal: Proposal,
) -> (ConsensusState, Option<Message>) {

    let mut new_consensus_state = consensus_state.clone();
    let mut message = None;

    if consensus_state.proposal.is_none() {

        if proposal_validty(
            proposal.clone(),
            consensus_state.round,
            consensus_state.height,
            &consensus_state.validators,
        ) {
            new_consensus_state.proposal = Some(proposal.clone());
            message = Some(Message::Proposal(proposal));
        }
    }
    
    (new_consensus_state, message)
}