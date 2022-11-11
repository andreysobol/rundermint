use ed25519_dalek::{PublicKey, SecretKey};

use crate::edsig::sign_message::sign_message;
use crate::messages::message::Message;
use crate::messages::prevote::Prevote;
use crate::proto::consensus_state::ConsensusState;
use crate::messages::proposal::Proposal;
use crate::proto::predicates::proposal_validity::proposal_validty;

pub fn on_proposal(
    consensus_state: ConsensusState,
    proposal: Proposal,
    secret_key: SecretKey,
) -> (ConsensusState, Vec<Message>) {

    let mut new_consensus_state = consensus_state.clone();
    let mut messages = Vec::new();

    if consensus_state.proposal.is_none() && proposal_validty(
            proposal.clone(),
            consensus_state.round,
            consensus_state.height,
            &consensus_state.validators,
        ) {
        new_consensus_state.proposal = Some(proposal.clone());
        messages.push(Message::Proposal(proposal.clone()));

        let public_key = PublicKey::from(&secret_key);

        let proposal_hash = proposal.full_hash();

        let prevote = Prevote {
            voter: public_key,
            proposal_hash: proposal_hash.clone(),
            signature: sign_message(&proposal_hash, secret_key),
        };

        messages.push(Message::Prevote(prevote));
    }
    
    (new_consensus_state, messages)
}