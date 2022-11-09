use crate::messages::prevote::Prevote;
use crate::messages::proposal::Proposal;

#[derive(Clone)]
pub struct ProofOfLock {
    pub proposal: Box<Proposal>,
    pub prevotes: Vec<Prevote>,
}