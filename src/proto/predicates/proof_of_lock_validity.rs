use crate::messages::proof_of_lock::ProofOfLock;
use crate::messages::proposal::Proposal;
use crate::types::{Round, Height};
use crate::statemachine::state::State;
use ed25519_dalek::PublicKey;

use super::prevote_validity::prevote_validty;
use super::proposal_validity::proposal_validty;

pub fn proof_of_lock_validity(
    proof_of_lock: &ProofOfLock,
    round: Round,
    height: Height,
    validators: &[PublicKey],
    threshold: usize,
) -> bool {

    let proposal: &Proposal = &proof_of_lock.proposal.clone();

    proposal_validty(proposal.clone(), round, height, validators);

    let validity_map = validators.iter().map(|validator| {
        let possible_prevotes = proof_of_lock.prevotes.iter().filter(|prevote| &prevote.voter == validator);
        let possible_prevotes_vec = possible_prevotes.collect::<Vec<_>>(); 
        if possible_prevotes_vec.len() == 1 {
            prevote_validty(possible_prevotes_vec[0].clone(), Some(proposal.clone()), validators)
        } else {
            false
        }
    });

    let amount_of_true = validity_map.filter(|validity| *validity).count();

    amount_of_true >= threshold

}