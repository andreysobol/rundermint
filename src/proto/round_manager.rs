use crate::types::Round;
use ed25519_dalek::PublicKey;

pub fn round_proposer(
    round: Round,
    validators: &[PublicKey],
) -> PublicKey {
    let proposer_index = round % (validators.len() as u64);
    validators[proposer_index as usize].clone()
}