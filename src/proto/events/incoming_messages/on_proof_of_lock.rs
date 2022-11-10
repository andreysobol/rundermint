use crate::messages::proof_of_lock::ProofOfLock;
use crate::proto::consensus_state::ConsensusState;
use crate::proto::predicates::proof_of_lock_validity::proof_of_lock_validity;

pub fn on_proof_of_lock(
    consensus_state: ConsensusState,
    proof_of_lock: ProofOfLock,
) -> ConsensusState {

    let mut new_consensus_state = consensus_state.clone();

    if consensus_state.locked_state.is_none() {
        let proof_of_lock_clone = proof_of_lock.clone();

        if proof_of_lock_validity(
            &proof_of_lock,
            consensus_state.round,
            consensus_state.height,
            &consensus_state.validators,
            consensus_state.threshold
        ) {
            let state_transition = proof_of_lock.proposal.state_transition;
            let new_locked_state = state_transition.apply_state_transition(&consensus_state.state);
            new_consensus_state.locked_state = Some(new_locked_state);
            new_consensus_state.proof_of_lock = Some(proof_of_lock_clone);
        }
    }

    new_consensus_state
}