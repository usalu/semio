//! 🏃️ Owner-private GIS Map proposal runtime: frozen binding, ledger, per-document gate, typed approval.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinHandle;

use directory::os_directory::{ArtifactFrontier, DocumentScope};

use super::catalog::VerifiedGisMapArtifactBindingV1;
use super::command::{encode_server_stamped_command_v1, CanonicalInferenceCommandPartsV1, CanonicalInferenceCommandV1};
use super::schema::{
    GisMapInferencePreviewV1, InferenceApprovalReceiptV1, InferenceEventPageV1, InferenceEventV1, InferenceIdentityV1, InferenceJobReceiptV1, InferenceJobReconcileResultV1, InferenceProgressV1, GIS_DOCUMENT_SCHEMA, PROPOSAL_MAX_BYTES,
    RESULT_MAX_BYTES,
};
use super::sqlite::{InferenceJobLedgerV1, InferenceReaderV1};
use super::wal::{CommittedInferenceWalWitnessV1, InferenceDocumentFenceV1, InferenceWalTargetV1, InferenceWalVerifierV1};
use super::{sha256, InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1};
use crate::directory::HubDirectory;

/// 🔢️ Fixed bounds every inference route enforces before it touches storage or the GIS executor.
pub const OPERATION_CAPACITY: usize = 32;
pub const DOCUMENT_GATE_CAPACITY: usize = 64;
pub const WORK_UNIT_LIMIT: u64 = 4096;
pub const ALLOCATION_BYTES: u64 = 1 << 20;
pub const RECURSION_DEPTH: u32 = 32;
pub const APPROVAL_MAX_RECORDS: u64 = 8;
pub const GIS_MAP_COMMITTER_CAPACITY: usize = 64;

#[path = "🧾️approval-contract/🦀️.rs"]
mod approval_contract;
pub use approval_contract::*;

#[path = "🌍️gis-map-binding/🦀️.rs"]
mod gis_map_binding;
use gis_map_binding::*;

#[path = "🔐️committer/🦀️.rs"]
mod committer;
pub use committer::*;

#[path = "🧱️document-preparation/🦀️.rs"]
mod document_preparation;

#[path = "🔁️commit-turns/🦀️.rs"]
mod commit_turns;

#[path = "🔎️checkpoint-verify/🦀️.rs"]
mod checkpoint_verify;

#[path = "📕️commit-and-close/🦀️.rs"]
mod commit_and_close;

#[path = "🧭️job-types-and-port/🦀️.rs"]
mod job_types_and_port;
pub use job_types_and_port::*;

#[cfg(feature = "integration-fixtures")]
#[path = "🚧️checkpoint-test-gate/🦀️.rs"]
mod checkpoint_test_gate;
#[cfg(feature = "integration-fixtures")]
pub use checkpoint_test_gate::*;

#[path = "🏭️inference-runtime/🦀️.rs"]
mod inference_runtime;
pub use inference_runtime::*;

#[path = "🛣️routes/🦀️.rs"]
mod routes;
pub use routes::*;

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
fn approval_stamp_v1(document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> protocol::HybridLogicalTimestamp {
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
fn fresh_now_ms() -> Result<u64, InferenceRouteErrorV1> {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| InferenceRouteErrorV1::Storage)?;
    u64::try_from(elapsed.as_millis()).ok().filter(|value| *value <= super::schema::SAFE_INTEGER_MAX).ok_or(InferenceRouteErrorV1::Bounds)
}

/// 🧊️ Decodes one canonical actor-frontier chain, including the genesis zero sentinel.
fn parse_frontier_chain_hash(value: &str) -> Option<directory::os_directory::ArtifactHash> {
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
