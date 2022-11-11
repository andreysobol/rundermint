use crate::messages::proposal::Proposal;
use crate::types::{Height, Round};

use crate::edsig::verify_signature::verify_signature;
use crate::proto::round_manager::round_proposer;
use ed25519_dalek::PublicKey;

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
