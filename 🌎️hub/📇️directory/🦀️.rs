//! 🗄️ Backend-agnostic os-hub identity/tenancy directory. `HubDirectory` is the single seam every
//! persistence backend (sqlite/postgres/neo4j — see the sibling `🪶️sqlite`/`🐘️postgres`/`🌐️neo4j`
//! component folders, each `#[cfg(feature = "…")]`-gated) implements; `bin.rs` never sees a driver
//! type (sqlx/neo4rs/rusqlite), only this trait and the DTOs in `model` — satisfies the "external
//! libraries stay behind an interface" rule for a trait three backends must share.
//!
//! 🎯️ Design choice (split from the pre-CW6 `HubStorage`): document persistence (snapshots,
//! operations) and content-addressed blobs are no longer this crate's concern — `db::Database`
//! (server-side document authority) and `db`'s own `PayloadStorage` own that now (see `bin.rs`).
//! This module keeps exactly the identity/tenancy surface that has no `db` counterpart: users,
//! spaces, memberships, auth sessions, share tokens, and realtime sync sessions. The former VFS
//! tree (`NodeRecord`/`list_nodes`/`create_node`) was deleted in the space/collection/artifact
//! unification wave — the collection document now replaces the hub-side tree (see
//! `.claude/plans/the-final-goal-for-jolly-spindle.md`'s "Roles/kinds/visibility" design ruling).

//#region 🔖️Error
pub mod error {
    /// 🧯️ Opaque directory error — never wraps a backend driver's error type, so no `sqlx`/
    /// `neo4rs`/`rusqlite` type ever crosses this crate's public API.
    #[derive(Debug)]
    pub enum DirectoryError {
        NotFound(String),
        Conflict(String),
        Unauthorized,
        Backend(String),
    }

    impl std::fmt::Display for DirectoryError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotFound(detail) => write!(formatter, "not found: {detail}"),
                Self::Conflict(detail) => write!(formatter, "conflict: {detail}"),
                Self::Unauthorized => formatter.write_str("unauthorized"),
                Self::Backend(detail) => write!(formatter, "backend error: {detail}"),
            }
        }
    }

    impl std::error::Error for DirectoryError {}

    pub type DirectoryResult<T> = Result<T, DirectoryError>;
}
//#endregion 🔖️Error

#[path = "🧾️model/🦀️.rs"]
pub mod model;

use crate::artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasDeleteOutcome, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, ArtifactChunkCasStorage};
pub use crate::artifact_authority::creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use directory::os_directory::{ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandResultV1, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, Hlc, PublishedArtifactBlob, PublishedArtifactCheckpoint};
use directory::os_directory::io::binary::descriptor_digest::{descriptor_digest_v1};
use directory::os_identity::time_ordered_id;
use error::{DirectoryError, DirectoryResult};
use model::*;
use semio_framework_hash::Sha256;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// 🧯️ Immutable per-document checkpoint-lineage ceiling shared by every backend.
pub const ARTIFACT_CHECKPOINT_LINEAGE_MAX: u64 = 16_384;
/// 🧯️ Immutable full-directory replay ceiling; exceeding it requires an operator repair.
pub const DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS: u64 = 1_000_000;
/// 📖️ Immutable maximum number of public directory events returned by one read.
pub const DIRECTORY_EVENT_READ_MAX: usize = 10_000;
pub const ACTIVE_SYNC_SESSION_READ_MAX: usize = 4_096;
/// 🛡️ Exact public administrator request-body ceiling.
pub const ADMIN_INTENT_REQUEST_MAX_BYTES: usize = 8 * 1024;
/// 📄 Exact administrator query page ceiling.
pub const ADMIN_PAGE_MAX: usize = 100;
/// 📄️ One public page plus one private continuation probe.
pub const ADMIN_PAGE_FETCH_MAX: usize = ADMIN_PAGE_MAX + 1;
/// 🛡️ Exact serialized administrator response ceiling.
pub const ADMIN_RESPONSE_MAX_BYTES: usize = 64 * 1024;
/// 🏛️ Exact rows one space-administration page window returns.
pub const SPACE_ADMINISTRATION_PAGE_MAX: usize = 64;
/// 🏛️ One public window plus one private continuation probe.
pub const SPACE_ADMINISTRATION_PAGE_FETCH_MAX: usize = SPACE_ADMINISTRATION_PAGE_MAX + 1;
/// 🌐️ Largest exact integer shared by the Rust, JSON, and TypeScript contracts.
pub const DIRECTORY_WIRE_INTEGER_MAX: u64 = 9_007_199_254_740_991;
/// 🔑️ Fixed UTF-8 byte ceiling for one backend-private immutable blob locator.
pub const ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES: usize = 4_096;
/// 🧯️ Maximum private ownership journal rows returned by one sweep read.
pub const ARTIFACT_CAS_SWEEP_PAGE_MAX: usize = 16;
/// 🧯️ Maximum physical objects considered by one sweep request.
pub const ARTIFACT_CAS_SWEEP_OBJECT_MAX: usize = 4_096;
/// ⏳️ Maximum wall-clock lifetime of one private pre-write reservation.
pub const ARTIFACT_CAS_RESERVATION_MAX_TTL_MS: u64 = 300_000;
/// 🛡️ Crash-recoverable lifetime of one durable per-space physical-deletion lease.
pub const ARTIFACT_CAS_DELETE_LEASE_TTL_MS: u64 = 5_000;
const ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.sweep-continuation.v1\0";
const ARTIFACT_CAS_DELETE_LEASE_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.delete-lease.v1\0";
const ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES: usize = 21;
const ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES: usize = ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES + 32;

/// 🗂️ One bounded page of private append-only reachability inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepCandidatePage {
    pub observed_generation: u64,
    pub next_generation: u64,
    pub objects: Vec<ArtifactCasObjectKey>,
}

/// 🎫️ Server-instance-bound opaque position within one immutable ledger generation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ArtifactCasSweepContinuation([u8; ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES]);

impl fmt::Debug for ArtifactCasSweepContinuation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ArtifactCasSweepContinuation(<opaque>)")
    }
}

/// 🧹️ Host intent for one bounded sweep; dry-run is the only default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepRequest {
    pub execute: bool,
    pub max_objects: usize,
    pub continuation: Option<ArtifactCasSweepContinuation>,
}

impl Default for ArtifactCasSweepRequest {
    fn default() -> Self {
        Self { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }
    }
}

/// 📊️ Locator-free bounded sweep result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepResult {
    pub observed_generation: u64,
    pub final_generation: u64,
    pub examined_objects: u64,
    pub protected_objects: u64,
    pub eligible_objects: u64,
    pub deleted_objects: u64,
    pub missing_objects: u64,
    pub result_digest: ArtifactHash,
    pub continuation: Option<ArtifactCasSweepContinuation>,
}

/// 📈️ Bounded projection-rebuild progress emitted after every replayed event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildProgress {
    pub completed_events: u64,
    pub total_events: u64,
}

/// 🎛️ Host-owned cancellation/progress seam for a potentially expensive full-log replay.
pub trait ProjectionRebuildControl: Send + Sync {
    fn is_cancelled(&self) -> bool;
    fn report(&self, progress: ProjectionRebuildProgress);
}

pub(crate) struct UncontrolledProjectionRebuild;

impl ProjectionRebuildControl for UncontrolledProjectionRebuild {
    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: ProjectionRebuildProgress) {}
}

pub(crate) static UNCONTROLLED_PROJECTION_REBUILD: UncontrolledProjectionRebuild = UncontrolledProjectionRebuild;

pub(crate) fn checkpoint_projection_rebuild(control: &dyn ProjectionRebuildControl, completed_events: u64, total_events: u64) -> DirectoryResult<()> {
    if total_events > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
        return Err(DirectoryError::Conflict(format!("directory projection rebuild exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
    }
    control.report(ProjectionRebuildProgress { completed_events, total_events });
    if control.is_cancelled() {
        return Err(DirectoryError::Conflict("directory projection rebuild cancelled".into()));
    }
    Ok(())
}

pub(crate) fn bounded_event_read(since_seq: u64, limit: usize) -> DirectoryResult<(i64, i64)> {
    if since_seq > DIRECTORY_WIRE_INTEGER_MAX || limit == 0 || limit > DIRECTORY_EVENT_READ_MAX {
        return Err(DirectoryError::Conflict(format!("directory event read requires since <= {DIRECTORY_WIRE_INTEGER_MAX} and limit 1..={DIRECTORY_EVENT_READ_MAX}")));
    }
    Ok((i64::try_from(since_seq).map_err(|error| DirectoryError::Conflict(error.to_string()))?, i64::try_from(limit).map_err(|error| DirectoryError::Conflict(error.to_string()))?))
}

#[path = "🎟️capabilities/🦀️.rs"]
mod capabilities;
pub use capabilities::*;

#[path = "🔤️wire/🦀️.rs"]
mod wire;
pub use wire::*;

#[path = "🕰️events/🦀️.rs"]
pub(crate) mod events;
pub use events::*;

#[path = "🧭️decider/🦀️.rs"]
mod decider;
pub use decider::*;

#[path = "🛠️service/🦀️.rs"]
mod service;
pub use service::*;

#[path = "🔌️port/🦀️.rs"]
mod port;
pub use port::*;

//#region 🔖️Backends
// 🧭️ Top-level `mod` declarations in a real (file-backed, non-inline) module resolve `#[path]`
// relative to THIS file's own directory (🌎️hub/📇️directory/) — no cumulative/leaf-prefixed math
// needed here, that convention is for paths declared inside an entry file's inline nested `mod`
// blocks (see `rustEntryPathRules` in 🔣️taxonomy.json and `bin.rs`'s own `mod directory` line).
#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "🐘️postgres/🦀️.rs"]
pub mod postgres;

#[path = "🌐️neo4j/🦀️.rs"]
pub mod neo4j;
//#endregion 🔖️Backends

#[path = "🔀️dispatch/🦀️.rs"]
mod dispatch;
pub use dispatch::*;

//#region 🧪️Tests
// 🔬️ Exercises `decide`/`DirectoryService` against the sqlite backend (the only one the default
// feature set — Amendment 2 — actually compiles/runs here); postgres/neo4j get the same coverage
// via their own `#[cfg(test)]` modules once `🛢️db`'s optional-dependency gap is fixed (not this
// lane's to fix, see the lane report).
#[cfg(test)]
#[path = "🧪️tests/🔮️backend-corpus/🦀️.rs"]
pub(crate) mod backend_corpus;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
