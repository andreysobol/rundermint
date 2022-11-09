use crate::messages::proposal::Proposal;
use crate::types::{Round, Height, ProofOfLock};
use crate::statemachine::state::State;
use crate::proto::round_manager::round_proposer;
use ed25519_dalek::PublicKey;
use crate::edsig::verify_signature::verify_signature;

pub fn proposal_validty(
    proposal: Proposal,
    round: Round,
    height: Height,
    last_state: State,
    validators: &[PublicKey],
) -> bool {

    // Check if the proposal is valid
    let proof_of_lock = &proposal.proof_of_lock;

    // check proof_of_lock
    // if proof_of_lock != last_state
    // last_state = proof_of_lock.state
    // i think we need to check that state is just one step ahead of last_state

    if proposal.round != round {
        return false;
    }

    if proposal.height != height {
        return false;
    }

    let round_proposer = round_proposer(round, validators);

    if proposal.proposer != round_proposer {
        return false;
    }

    verify_signature(proposal.body_hash(), proposal.signature, proposal.proposer)
   
}