use crate::messages::precommit::Precommit;
use crate::messages::prevote::Prevote;
use crate::messages::proof_of_lock::ProofOfLock;
use crate::messages::proposal::Proposal;

pub enum Message {
    Proposal(Proposal),
    ProofOfLock(ProofOfLock),
    Precommit(Precommit),
    Prevote(Prevote),
}
