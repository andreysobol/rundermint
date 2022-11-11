use crate::messages::message::Message;
use crate::messages::prevote::Prevote;
use crate::messages::proof_of_lock::ProofOfLock;
use crate::proto::consensus_state::ConsensusState;
use crate::proto::predicates::prevote_validity::prevote_validity;
use crate::proto::predicates::proof_of_lock_validity::proof_of_lock_validity;

pub fn on_prevote(
    consensus_state: ConsensusState,
    prevote: Prevote,
) -> (ConsensusState, Option<Message>) {
    let mut new_consensus_state = consensus_state.clone();
    let mut message = None;

    if consensus_state
        .prevotes
        .iter()
        .find(|el| el.voter == prevote.voter)
        .is_none()
        && prevote_validity(
            prevote.clone(),
            consensus_state.proposal.clone(),
            &consensus_state.validators,
        )
    {
        new_consensus_state.prevotes.push(prevote.clone());
        message = Some(Message::Prevote(prevote));

        match consensus_state.proposal {
            Some(p) => {
                let try_new_proof_of_lock = ProofOfLock {
                    proposal: Box::new(p),
                    prevotes: new_consensus_state.prevotes.clone(),
                };
                if proof_of_lock_validity(
                    &try_new_proof_of_lock,
                    consensus_state.round,
                    consensus_state.height,
                    &consensus_state.validators,
                    consensus_state.threshold,
                ) {
                    new_consensus_state.proof_of_lock = Some(try_new_proof_of_lock.clone());
                    message = Some(Message::ProofOfLock(try_new_proof_of_lock));
                }
            }
            None => {}
        }
    }

    (new_consensus_state, message)
}
