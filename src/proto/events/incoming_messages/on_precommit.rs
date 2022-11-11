use crate::{messages::{precommit::Precommit, message::Message}, proto::{consensus_state::ConsensusState, predicates::{commit_validity::commit_validty, precommit_validity::{self, precommit_validty}}}};

pub fn on_precommit(
    consensus_state: ConsensusState,
    precommit: Precommit,
) -> (ConsensusState, Option<Message>) {

    let mut new_consensus_state = consensus_state.clone();
    let mut message = None;

    if consensus_state.precommits.iter().find(|el| {
        el.voter == precommit.voter
    }).is_none() {
        new_consensus_state.precommits.push(precommit.clone());

        if precommit_validty(
            precommit.clone(),
            consensus_state.proposal.clone(),
            &consensus_state.validators,
        ) {
            message = Some(Message::Precommit(precommit.clone()));

            if commit_validty(
                consensus_state.proposal,
                consensus_state.proof_of_lock,
                new_consensus_state.precommits.clone(),
                consensus_state.round,
                consensus_state.height,
                &consensus_state.validators,
                consensus_state.threshold,
            ) {
                if !consensus_state.commited {
                    new_consensus_state.commited = true;
                }
            }
        }
    }

    (new_consensus_state, message)

}