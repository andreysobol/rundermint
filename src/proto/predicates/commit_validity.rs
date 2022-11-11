use crate::messages::proposal::Proposal;
use crate::types::{Round, Height};
use crate::statemachine::state::State;
use ed25519_dalek::PublicKey;
use crate::proto::predicates::proof_of_lock_validity::proof_of_lock_validity;
use crate::messages::precommit::Precommit;
use crate::proto::predicates::precommit_validity::precommit_validty;
use crate::messages::proof_of_lock::ProofOfLock;

pub fn commit_validty(
    proposal: Option<Proposal>,
    proof_of_lock: Option<ProofOfLock>,
    precommits: Vec<Precommit>,
    round: Round,
    height: Height,
    validators: &[PublicKey],
    threshold: usize,
) -> bool {

    fn check(
        validators: &[PublicKey],
        precommits: Vec<Precommit>,
        threshold: usize,
        proposal: Proposal,
    ) -> bool {
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

    match proposal {
        Some(proposal) => {
            match proof_of_lock {
                Some(proof_of_lock) => {
                    if !proof_of_lock_validity(&proof_of_lock, round, height, validators, threshold) {
                        false
                    } else {
                        check(validators, precommits, threshold, proposal)
                    }
                },
                None => {
                    false
                }                
            }
        },
        None => {
            false
        }
    }

}