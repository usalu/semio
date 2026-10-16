//! 📜️ Inference runtime contract: the approval contract and committer port, the neutral composed-map binding, the job vocabulary and the bounded helpers every runtime crate shares.

#[path = "../🧾️approval-contract/🦀️.rs"]
mod approval_contract;
use approval_contract::*;

#[path = "../🔗️binding/🦀️.rs"]
mod binding;
use binding::*;

#[path = "../🧭️job-types-and-port/🦀️.rs"]
mod job_types_and_port;
use job_types_and_port::*;
use directory::os_directory::DocumentScope;
use semio_hub_inference::schema::InferenceIdentityV1;
use semio_hub_inference_ledger::InferenceReaderV1;
use std::time::{SystemTime, UNIX_EPOCH};
use semio_hub_inference::sha256;

/// 🔢️ Fixed bounds every inference route enforces before it touches storage or the GIS executor.
pub const OPERATION_CAPACITY: usize = 32;

pub const DOCUMENT_GATE_CAPACITY: usize = 64;

pub const WORK_UNIT_LIMIT: u64 = 4096;

pub const ALLOCATION_BYTES: u64 = 1 << 20;

pub const RECURSION_DEPTH: u32 = 32;

pub const APPROVAL_MAX_RECORDS: u64 = 8;

pub const GIS_MAP_COMMITTER_CAPACITY: usize = 64;

/// 🔑️ Renders the exact full document key every envelope and witness compares byte for byte.
/// ⏰️ Mints one approval stamp that is strictly after the document's own last edit.
///
/// The hub's `now_ms` alone is not enough: two approvals in the same millisecond, or an approval in
/// the same millisecond as the previous edit, both produce `logical: 0` and the authority mint then
/// refuses them because the stamp is not strictly after the Store's clock. Carrying the document's
/// clock into the mint makes the stamp dominate by construction, and it is the stamp the command
/// hash is computed over, so the WAL proof rebuilds the identical command from `meta.timestamp`.
/// `None` is the genesis case — no Store is mounted because the document has no history yet — and
/// then the hub's own tick is already after everything the document knows.
pub fn approval_stamp_v1(document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> protocol::HybridLogicalTimestamp {
    match document_clock {
        Some(clock) if now_ms > clock.physical_ms => protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
        Some(clock) => protocol::HybridLogicalTimestamp { actor: 1, physical_ms: clock.physical_ms, logical: clock.logical.saturating_add(1) },
        None => protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
    }
}

pub fn document_key(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

/// 🕰️ Reads the process clock for retained work after its request has already returned.
pub fn fresh_now_ms() -> Result<u64, InferenceRouteErrorV1> {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| InferenceRouteErrorV1::Storage)?;
    u64::try_from(elapsed.as_millis()).ok().filter(|value| *value <= semio_hub_inference::schema::SAFE_INTEGER_MAX).ok_or(InferenceRouteErrorV1::Bounds)
}

/// 🧊️ Decodes one canonical actor-frontier chain, including the genesis zero sentinel.
pub fn parse_frontier_chain_hash(value: &str) -> Option<directory::os_directory::ArtifactHash> {
    if value.len() != 64 || value.bytes().any(|byte| !byte.is_ascii_digit() && !matches!(byte, b'a'..=b'f')) {
        return None;
    }
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(directory::os_directory::ArtifactHash::new(bytes))
}

/// 🎭️ Renders the exact server-derived actor; a client never supplies or influences it.
pub fn approval_actor(identity: &InferenceIdentityV1) -> String {
    format!("user:{}#session:{}", identity.user_id, identity.session_id)
}

/// 🆔️ Derives the deterministic approval mutation identity from job and proposal alone.
pub fn approval_mutation_id(job_id: &str, proposal_hash: &str) -> String {
    sha256(format!("semio.hub.inference-approval-mutation/v1\0{job_id}\0{proposal_hash}").as_bytes())[..32].to_string()
}

/// 👤️ Renders the owner-private reader every read, cancel, and approval revalidates against.
pub fn reader<'a>(identity: &'a InferenceIdentityV1) -> InferenceReaderV1<'a> {
    InferenceReaderV1 { user_id: &identity.user_id, session_id: &identity.session_id, authorization_generation: identity.authorization_generation, space_id: &identity.space_id, document_id: &identity.document_id }
}

/// ⏱️ The fixed native inference budget every execution of the registered binding runs under.
pub fn inference_budget() -> semio_framework_plugin::WireArtifactInferenceBudget {
    semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH }
}

pub use binding::{GisMapAssemblyV1, GisMapClosingStoresV1, GisMapCommitHostV1, GisMapCommitIdentityV1, GisMapCommitOperationV1, GisMapDocumentStoresV1, GisMapDrawingStoreV1, GisMapJournalReceiptV1, GisMapParentSnapshotV1, GisMapParentStoreV1, GisMapRecoveryOwnerV1, GisMapValueStoreV1};
pub use job_types_and_port::{InferenceMapBaseV1, InferenceOperationAdmissionV1, InferenceProposalV1, InferenceRunKeyV1, RetainedInferenceRunSupervisorV1, RetainedInferenceRunV1};
pub use approval_contract::{GisMapApprovalCheckpointPublisherV1, GisMapApprovalCheckpointRequestV1, GisMapApprovalCommitErrorV1, GisMapApprovalCommitRequestV1, GisMapApprovalCommitterV1, GisMapApprovalIngressAuthorityV1, GisMapApprovalOutcomeV1, GisMapApprovalReceiptV1, GisMapApprovalRequestTokenV1, GisMapApprovalUndoCommitReceiptV1, GisMapApprovalUndoCommitRequestV1, GisMapCommitConflictArmV1, GisMapDocumentWriteAuthorityV1, GisMapDocumentWriteLeaseV1, GisMapPreparedUndoCommandV1, InferenceRouteErrorV1, UnavailableGisMapApprovalCommitterV1, gis_map_commit_conflict, gis_map_storage_refusal, record_abandoned_assembly_turn, record_close_store_step, record_identity_mismatch, record_prepare_conflict, recorded_abandoned_assembly_turn};
#[cfg(any(test, feature = "testkit"))]
pub use approval_contract::{last_abandoned_assembly_turn, last_close_store_step, last_gis_map_commit_conflict_arm, last_gis_map_identity_mismatch, last_gis_map_prepare_conflict, last_gis_map_storage_refusal};
