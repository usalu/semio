//! 🐘️ The directory capabilities over PostgreSQL — direct `sqlx-postgres`/`sqlx-core` (not the `sqlx` facade),
//! avoiding the facade's optional SQLite resolver edge. The scale-out backend for multi-node
//! self-hosted deployments. `#[cfg(feature = "postgres")]`-gated as a whole by the parent
//! `directory` module (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).
//!
//! 🌳️ `SCHEMA` is inlined as a `const` string rather than `include_str!`-ed from a sibling
//! `.sql` file: Shape V2 tree purity allows only `component.<ext>` files, `📦️packages`, and plain
//! component folders below an owner root, so a standalone `🗄️.sql` asset has no home in this
//! tree (it is neither example/fixture/generated data for `rootDataDirNames` nor packaging code)
//! — folding it into a string literal is a zero-behavior-change mechanical transform (see
//! `📋️TEMPLATE-FAMILY.md`'s "non-source assets" section for the general rule this establishes).

use semio_hub_artifact_creation::io::decide_artifact_creation_fact_append_v1;
use directory::os_directory::io::text::validate_directory_event_page_event;
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1};
use semio_hub_artifact_creation::{ArtifactCreationActorV1, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_contract::{AdminEffectPreflightFailure, actor_kind_from_str, actor_kind_to_str, admin_effect_preflight, backend, now_ms};
use semio_hub_directory_contract::{DIRECTORY_FORMAT_SCHEMA, DIRECTORY_FORMAT_VERSION, DirectoryFormatAdmission, admit_directory_format, ADMIN_PAGE_MAX, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, HubClock, InviteCapability, InviteRedemptionPreflight, InviteRedemptionScopeHintV1, InviteRedemptionSpaceStateV1, NewDirectoryEvent, SessionCapability, ShareCapability, active_capability, admin_operation_effect_receipt_v1, auth_audit, directory_command_result_kind_from_str, directory_command_result_kind_str, invite_redemption_preflight, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire, role_to_wire, same_admin_operation_request, validate_admin_operation_audit, validate_admin_operation_effect_receipt, validate_bounded_auth_text, validate_checkpoint_publication_claim, validate_checkpoint_publication_completion, validate_directory_command_claim, verify_invite_redemption_event, verify_invite_redemption_scope_hint, visibility_to_str};
use semio_hub_directory::{AdminOperationDirectory, AgentDelegationDirectory, ArtifactCasDirectory, ArtifactCasSweepCandidatePage, ArtifactCreationDirectory, AuthSessionDirectory, CredentialDirectory, DirectoryAppendOutcomeV1, DirectoryProjectionRejectionV1, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ProjectionRebuildControl, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UNCONTROLLED_PROJECTION_REBUILD, UserDirectory, bounded_event_read, checkpoint_projection_rebuild, directory_projection_rejection_v1, directory_projection_space_v1, document_genesis_completion_v1, validate_document_genesis_append_v1, validate_verified_checkpoint_append};
use directory::os_directory::{
    ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, Hlc, PublishedArtifactCheckpoint,
    };
use directory::os_identity::time_ordered_id;
use directory::{DslValue, FromValue, ToValue};
use semio_framework_hash::Sha256;
use sqlx_postgres::{PgPool, PgPoolOptions};
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AuthAuditRecord, AuthSessionIssue, AuthSessionKind, AuthSessionRecord, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CheckpointPublicationDispositionV1, CheckpointPublicationReceiptRecordV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, InviteRecord, InviteRedemptionCommit, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession, ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use directory::os_directory::DocumentScope;

macro_rules! admin_effect_try {
    ($tx:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return admin_effect_rollback($tx).await,
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
pub(crate) use artifact_cas::*;

#[path = "📜️event-log/🦀️.rs"]
mod event_log;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;
