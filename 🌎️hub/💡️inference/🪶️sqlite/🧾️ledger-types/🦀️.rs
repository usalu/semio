//! 🧾️ Ledger rows and views: the ledger handle, job rows, run claims, progress and event rows, event pages and the reader.

use super::*;

pub struct InferenceJobLedgerV1 {
    pub(super) connection: Mutex<Connection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceLedgerJobRowV1 {
    pub job_id: String,
    pub identity_digest: String,
    pub expires_at_ms: u64,
}

pub struct InferenceJobViewV1 {
    pub receipt: InferenceLedgerJobRowV1,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub result: InferencePrivateBytesV1,
    pub proposal: InferencePrivateBytesV1,
}

pub struct InferenceApprovalOutboxV1 {
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub prepared_at_ms: u64,
    pub command: InferencePrivateBytesV1,
}

pub struct InferenceApprovalPageV1 {
    pub rows: Vec<InferenceApprovalOutboxV1>,
    pub next_cursor: Option<String>,
}

/// 🎟️ One exclusive execution turn: only this epoch may append progress or a terminal outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InferenceRunClaimV1 {
    pub run_epoch: u64,
    pub lease_expires_at_ms: u64,
}

/// 📈️ One appended owner-private progress row of the bounded monotonic cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceProgressRowV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓️ One appended lifecycle event of the ordered private job stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceEventRowV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📃️ Owner-private bounded page: ordered events, progress rows after a cursor, and current state.
pub struct InferenceLedgerEventPageV1 {
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    pub events: Vec<InferenceEventRowV1>,
    pub progress: Vec<InferenceProgressRowV1>,
    pub next_cursor: u64,
    pub proposal_hash: Option<String>,
}

pub struct InferenceReaderV1<'a> {
    pub user_id: &'a str,
    pub session_id: &'a str,
    pub authorization_generation: u64,
    pub space_id: &'a str,
    pub document_id: &'a str,
}

impl InferenceReaderV1<'_> {
    pub fn matches(&self, identity: &InferenceIdentityV1) -> bool {
        self.user_id == identity.user_id && self.session_id == identity.session_id && self.authorization_generation == identity.authorization_generation && self.space_id == identity.space_id && self.document_id == identity.document_id
    }
}
