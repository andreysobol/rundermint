use crate::{messages::{precommit::Precommit, message::Message}, proto::consensus_state::ConsensusState};

pub fn on_precommit(
    consensus_state: ConsensusState,
    precommit: Precommit,
) -> (ConsensusState, Option<Message>) {

    let mut new_consensus_state = consensus_state.clone();
    let mut message = None;

    if consensus_state.precommits.iter().find(|el| {
        el.voter == precommit.voter
    }).is_none() {
        new_consensus_state.precommits.push(precommit.clone());
        message = Some(Message::Precommit(precommit));

        // ADD HERE COMMIT STATE
    }

    (new_consensus_state, message)

}