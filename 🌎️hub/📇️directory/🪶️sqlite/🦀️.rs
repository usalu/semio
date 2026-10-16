//! 🪶️ The directory capabilities over SQLite (rusqlite) — the zero-touch default for local dev and single-user
//! self-hosting; no external database service required. `#[cfg(feature = "sqlite")]`-gated as a
//! whole by the parent `directory` module (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).
//!
//! Uses synchronous `rusqlite` behind `Arc<Mutex<Connection>>`, not an async SQLite driver: a
//! Cargo workspace may link only one native `sqlite3` (`links = "sqlite3"`), and `rusqlite` is
//! already the sqlite binding used elsewhere in this workspace (`vcs`, `db_storage_sqlite`,
//! semio_compose_rs's unrelated client lib) — adding `sqlx-sqlite`'s `libsqlite3-sys` alongside it
//! is a hard `cargo` resolution conflict, not a style choice. Trait methods stay `async fn`
//! (satisfying the shared capability traits of `semio_hub_directory`) but their bodies are synchronous rusqlite
//! calls: queries are short, the mutex guard is never held across an `.await`, so nothing here
//! blocks the executor for longer than a real query takes.

use semio_hub_artifact_creation::io::decide_artifact_creation_fact_append_v1;
use directory::os_directory::io::text::validate_directory_event_page_event;
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1};
use semio_hub_artifact_creation::{ArtifactCreationActorV1, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_contract::{AdminEffectPreflightFailure, actor_kind_from_str, actor_kind_to_str, admin_effect_preflight, backend, now_ms};
use semio_hub_directory_contract::{DIRECTORY_FORMAT_SCHEMA, DIRECTORY_FORMAT_VERSION, DirectoryFormatAdmission, admit_directory_format, ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_PAGE_MAX, AGENT_DELEGATED_EVENT, AGENT_DELEGATION_PAGE_MAX, AGENT_DELEGATION_REVOKED_EVENT, AGENT_IDENTITY_PROVIDER, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, HubClock, InviteCapability, InviteRedemptionPreflight, InviteRedemptionScopeHintV1, InviteRedemptionSpaceStateV1, NewDirectoryEvent, SessionCapability, ShareCapability, active_capability, admin_operation_effect_receipt_v1, auth_audit, directory_command_result_kind_from_str, directory_command_result_kind_str, invite_redemption_preflight, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire, role_to_wire, same_admin_operation_request, validate_admin_operation_audit, validate_admin_operation_effect_receipt, validate_bounded_auth_text, validate_checkpoint_publication_claim, validate_checkpoint_publication_completion, validate_directory_command_claim, verify_invite_redemption_event, verify_invite_redemption_scope_hint, visibility_to_str};
use semio_hub_directory::{AdminOperationDirectory, AgentDelegationDirectory, ArtifactCasDirectory, ArtifactCasSweepCandidatePage, ArtifactCreationDirectory, AuthSessionDirectory, CredentialDirectory, DirectoryAppendOutcomeV1, DirectoryProjectionRejectionV1, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ProjectionRebuildControl, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UNCONTROLLED_PROJECTION_REBUILD, UserDirectory, bounded_event_read, checkpoint_projection_rebuild, directory_projection_rejection_v1, directory_projection_space_v1, validate_verified_checkpoint_append};
use directory::os_directory::{
    ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentOwner,
    Hlc, PublishedArtifactCheckpoint, };
use directory::os_identity::time_ordered_id;
use directory::{DslValue, FromValue, ToValue};
use rusqlite::{Connection, OptionalExtension, Transaction};
use std::sync::{Arc, Mutex};
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AgentAudience, AgentDelegationRecord, AgentDelegationRow, AuthAuditRecord, AuthSessionIssue, AuthSessionKind, AuthSessionRecord, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CheckpointPublicationDispositionV1, CheckpointPublicationReceiptRecordV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, InviteRecord, InviteRedemptionCommit, IssuedAgentDelegation, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession, ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use directory::os_directory::DocumentScope;

macro_rules! admin_effect_try {
    ($tx:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return admin_effect_rollback($tx),
        }
    };
}

#[path = "🗃️table-schema/🦀️.rs"]
mod schema;
pub(crate) use schema::*;

#[path = "🔌️connection/🦀️.rs"]
mod connection;
pub use connection::*;

#[path = "🧱️rows/🦀️.rs"]
mod rows;
pub(crate) use rows::*;

#[path = "📽️projections/🦀️.rs"]
mod projections;

#[path = "🏭️artifact-creation/🦀️.rs"]
mod artifact_creation;

#[path = "🎟️capabilities/🦀️.rs"]
mod capabilities;

#[path = "👤️accounts/🦀️.rs"]
mod accounts;
pub(crate) use accounts::*;

#[path = "🏘️spaces/🦀️.rs"]
mod spaces;

#[path = "🛡️administration/🦀️.rs"]
mod administration;
pub(crate) use administration::*;

#[path = "🔄️sync-sessions/🦀️.rs"]
mod sync_sessions;

#[path = "🧺️artifact-cas/🦀️.rs"]
mod artifact_cas;

#[path = "📜️event-log/🦀️.rs"]
mod event_log;
