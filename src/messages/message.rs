use crate::messages::precommit::Precommit;
use crate::messages::proposal::Proposal;
use crate::messages::prevote::Prevote;
use crate::messages::proof_of_lock::ProofOfLock;

pub enum Message {
    Proposal(Proposal),
    ProofOfLock(ProofOfLock),
    Precommit(Precommit),
    Prevote(Prevote),
}