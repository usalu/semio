//! 🗄️ Backend-agnostic os-hub identity/tenancy directory. The capability traits of `🔌️port`
//! (`UserDirectory`, `SpaceDirectory`, `EventLog`, …) are the seam every persistence backend
//! (sqlite/postgres/neo4j — see the sibling `🪶️sqlite`/`🐘️postgres`/`🌐️neo4j`
//! component folders, each `#[cfg(feature = "…")]`-gated) implements; `bin.rs` never sees a driver
//! type (sqlx/neo4rs/rusqlite), only these traits and the DTOs in `model` — satisfies the "external
//! libraries stay behind an interface" rule for traits three backends must share. A caller bounds
//! itself on exactly the capabilities it uses, for example `D: SpaceDirectory + EventLog`.
//!
//! 🎯️ Design choice (split from the pre-CW6 `HubStorage`): document persistence (snapshots,
//! operations) and content-addressed blobs are no longer this crate's concern — `db::Database`
//! (server-side document authority) and `db`'s own `PayloadStorage` own that now (see `bin.rs`).
//! This module keeps exactly the identity/tenancy surface that has no `db` counterpart: users,
//! spaces, memberships, auth sessions, share tokens, and realtime sync sessions. The former VFS
//! tree (`NodeRecord`/`list_nodes`/`create_node`) was deleted in the space/collection/artifact
//! unification wave — the collection document now replaces the hub-side tree (see
//! `.claude/plans/the-final-goal-for-jolly-spindle.md`'s "Roles/kinds/visibility" design ruling).

use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, ArtifactChunkCasStorage};
use semio_hub_artifact_creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use directory::os_directory::{ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandResultV1, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, Hlc, PublishedArtifactBlob, PublishedArtifactCheckpoint};
use directory::os_directory::io::binary::descriptor_digest::{descriptor_digest_v1};
use directory::os_identity::time_ordered_id;
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_framework_hash::Sha256;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// 🗂️ One bounded page of private append-only reachability inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasSweepCandidatePage {
    pub observed_generation: u64,
    pub next_generation: u64,
    pub objects: Vec<ArtifactCasObjectKey>,
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

pub static UNCONTROLLED_PROJECTION_REBUILD: UncontrolledProjectionRebuild = UncontrolledProjectionRebuild;

pub fn checkpoint_projection_rebuild(control: &dyn ProjectionRebuildControl, completed_events: u64, total_events: u64) -> DirectoryResult<()> {
    if total_events > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
        return Err(DirectoryError::Conflict(format!("directory projection rebuild exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
    }
    control.report(ProjectionRebuildProgress { completed_events, total_events });
    if control.is_cancelled() {
        return Err(DirectoryError::Conflict("directory projection rebuild cancelled".into()));
    }
    Ok(())
}

pub fn bounded_event_read(since_seq: u64, limit: usize) -> DirectoryResult<(i64, i64)> {
    if since_seq > DIRECTORY_WIRE_INTEGER_MAX || limit == 0 || limit > DIRECTORY_EVENT_READ_MAX {
        return Err(DirectoryError::Conflict(format!("directory event read requires since <= {DIRECTORY_WIRE_INTEGER_MAX} and limit 1..={DIRECTORY_EVENT_READ_MAX}")));
    }
    Ok((i64::try_from(since_seq).map_err(|error| DirectoryError::Conflict(error.to_string()))?, i64::try_from(limit).map_err(|error| DirectoryError::Conflict(error.to_string()))?))
}

#[path = "🔤️wire/🦀️.rs"]
mod wire;
use wire::*;

#[path = "🔌️port/🦀️.rs"]
mod port;
use port::*;
use semio_hub_directory_contract::{DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DIRECTORY_WIRE_INTEGER_MAX};

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🔮️backend-corpus/🦀️.rs"]
pub mod backend_corpus;

#[cfg(test)]
#[path = "🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs"]
mod socket_grant_oracle;

pub use port::{AdminOperationDirectory, AgentDelegationDirectory, ArtifactCasDirectory, ArtifactCreationDirectory, AuthSessionDirectory, CredentialDirectory, DocumentDirectory, EventLog, InviteDirectory, PrincipalAuthority, ProjectionRebuild, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UserDirectory};
pub use wire::{DirectoryAppendOutcomeV1, DirectoryProjectionRejectionV1, MemoryArtifactProjection, capped_socket_session_binding, checkpoint_identity_input, directory_projection_rejection_v1, directory_projection_space_v1, document_genesis_completion_v1, document_index_projection_v1, frontier_strictly_advances, published_artifact_checkpoint, validate_checkpoint_index_v1, validate_checkpoint_shape, validate_document_descriptor, validate_document_genesis_append_v1, validate_published_checkpoint_lineage, validate_retention_shape, validate_verified_checkpoint_append};
