use crate::messages::proposal::Proposal;
use crate::types::{Round, Height};
use crate::statemachine::state::State;
use crate::proto::round_manager::round_proposer;
use ed25519_dalek::PublicKey;
use crate::edsig::verify_signature::verify_signature;

pub fn commit_validty(
    proposal: Precommit,
    proof_of_lock: &ProofOfLock,
    precommits: Vec<Precommit>,
    round: Round,
    height: Height,
    last_state: State,
    validators: &[PublicKey],
    threshold: usize,
) -> bool {

    if !proof_of_lock_validity(proof_of_lock, round, height, last_state, validators, threshold) {
        return false;
    }

    let validity_map = validators.iter().map(|validator| {
        let possible_precommits = precommits.iter().filter(|precommit| &precommit.voter == validator);
        let possible_precommits_vec = possible_precommits.collect::<Vec<_>>(); 
        if possible_precommits_vec.len() == 1 {
            precommit_validty(possible_precommits_vec[0].clone(), Some(proposal.clone()), validators)
        } else {
            false
        }
    });
    
    let amount_of_true = validity_map.filter(|validity| *validity).count();

    amount_of_true >= threshold
}