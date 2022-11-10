use crate::messages::proposal::Proposal;
use crate::types::{Round, Height};
use crate::statemachine::state::State;
use crate::proto::round_manager::round_proposer;
use ed25519_dalek::PublicKey;
use crate::edsig::verify_signature::verify_signature;
use crate::messages::proof_of_lock::ProofOfLock;

pub fn proposal_validty(
    proposal: Proposal,
    round: Round,
    height: Height,
    validators: &[PublicKey],
) -> bool {

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