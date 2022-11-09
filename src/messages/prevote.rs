struct Prevote {
    // The round in which the prevote is being sent.
    pub round: Round,
    pub height: Height,
    pub proposal_hash: Vec<u8>,
}