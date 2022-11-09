use crate::messages::proposal::Proposal;
use crate::types::{Round, Height, ProofOfLock};
use crate::statemachine::state::State;
use crate::proto::round_manager::round_proposer;
use ed25519_dalek::PublicKey;
use crate::edsig::verify_signature::verify_signature;

pub fn prevote_validty(
    prevote: Proposal,
    proposal: Option<Proposal>,
    validators: &[PublicKey],
    already_voted: &[PublicKey],
) -> bool {

    if proposal == None {
        return false;
    }

    let unwrap_proposal = proposal.unwrap();

    if unwrap_proposal.body_hash() != prevote.proposal_hash {
        return false;
    }

    if !validators.contains(&prevote.voter) {
        return false;
    }

    verify_signature(prevote.body_hash(), prevote.signature, prevote.voter)

}