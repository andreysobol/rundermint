use crate::messages::proposal::Proposal;
use crate::messages::precommit::Precommit;
use ed25519_dalek::PublicKey;
use crate::edsig::verify_signature::verify_signature;

pub fn precommit_validty(
    prevote: Precommit,
    proposal: Option<Proposal>,
    validators: &[PublicKey],
    // already_voted: &[PublicKey],
    // Do we need already_voted?
) -> bool {

    if proposal.is_none() {
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