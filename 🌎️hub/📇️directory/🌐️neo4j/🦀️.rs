//! 🌐️ The directory capabilities over Neo4j (neo4rs). Users/Spaces/Memberships are real nodes+relationships —
//! where graph traversal earns its keep (role lookups, VFS tree walks). Document persistence and
//! blobs are no longer this crate's concern — `db::Database` and `db_storage_neo4j` own that now
//! (see `bin.rs`). `#[cfg(feature = "neo4j")]`-gated as a whole by the parent `directory` module
//! (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).

use semio_hub_artifact_creation::io::decide_artifact_creation_fact_append_v1;
use directory::os_directory::io::text::validate_directory_event_page_event;
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1};
use semio_hub_artifact_creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_contract::{actor_kind_to_str, admin_effect_preflight, backend, now_ms};
use semio_hub_directory_contract::{DIRECTORY_FORMAT_SCHEMA, DIRECTORY_FORMAT_VERSION, DirectoryFormatAdmission, admit_directory_format, ADMIN_PAGE_MAX, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, DIRECTORY_WIRE_INTEGER_MAX, HubClock, InviteCapability, InviteRedemptionPreflight, InviteRedemptionScopeHintV1, InviteRedemptionSpaceStateV1, NewDirectoryEvent, SessionCapability, ShareCapability, active_capability, admin_operation_effect_receipt_v1, auth_audit, decode_auth_digest_hex, directory_command_result_kind_str, encode_capability_bytes, invite_redemption_preflight, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire, role_to_wire, same_admin_operation_request, validate_admin_operation_audit, validate_admin_operation_effect_receipt, validate_bounded_auth_text, validate_checkpoint_publication_claim, validate_checkpoint_publication_completion, validate_directory_command_claim, verify_invite_redemption_event, verify_invite_redemption_scope_hint, visibility_to_str};
use semio_hub_directory::{AdminOperationDirectory, AgentDelegationDirectory, ArtifactCasDirectory, ArtifactCasSweepCandidatePage, ArtifactCreationDirectory, AuthSessionDirectory, CredentialDirectory, DirectoryAppendOutcomeV1, DirectoryProjectionRejectionV1, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ProjectionRebuildControl, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UNCONTROLLED_PROJECTION_REBUILD, UserDirectory, bounded_event_read, checkpoint_projection_rebuild, directory_projection_rejection_v1, directory_projection_space_v1, document_genesis_completion_v1, validate_document_genesis_append_v1, validate_verified_checkpoint_append};
use directory::os_directory::{ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, Hlc, PublishedArtifactCheckpoint};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use directory::os_identity::time_ordered_id;
use directory::{FromValue, ToValue};
use neo4rs::{Graph, Txn, query};
use semio_framework_hash::Sha256;
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AuthAuditRecord, AuthSessionIssue, AuthSessionRecord, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CheckpointPublicationDispositionV1, CheckpointPublicationReceiptRecordV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, InviteRecord, InviteRedemptionCommit, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession, ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use semio_hub_directory_neo4j_rows::{CONSTRAINTS, admin_effect_rollback, admin_operation_audit_from_node, artifact_creation_facts, artifact_creation_request_key, auth_audit_from_node, auth_session_from_node, cas_generation, cas_lock_space, cas_object_token, cas_project_publish, cas_project_release, cas_project_reserve, cas_reservation_barrier, checkpoint_key_v1, checkpoint_publication_receipt_from_node, checkpoint_publication_receipt_key, directory_command_receipt_from_node, directory_command_receipt_key, document_scope_key_v1, event_from_node, insert_admin_operation_effect_receipt, insert_artifact_creation_fact, insert_auth_audit, invite_from_node, lock_artifact_creation_request, neo4j_transient_retry_pause, share_from_node, space_from_node, sync_session_from_row, user_from_node, validate_artifact_creation_authority};
use directory::os_directory::DocumentScope;

macro_rules! admin_effect_try {
    ($txn:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return admin_effect_rollback($txn).await,
        }
    };
}

#[path = "🔌️connection/🦀️.rs"]
mod connection;
pub use connection::*;

#[path = "📽️projections/🦀️.rs"]
mod projections;

#[path = "🏭️artifact-creation/🦀️.rs"]
mod artifact_creation;

#[path = "🎟️capabilities/🦀️.rs"]
mod capabilities;

#[path = "👤️accounts/🦀️.rs"]
mod accounts;

#[path = "🏘️spaces/🦀️.rs"]
mod spaces;

#[path = "🛡️administration/🦀️.rs"]
mod administration;

#[path = "🔄️sync-sessions/🦀️.rs"]
mod sync_sessions;

#[path = "🧺️artifact-cas/🦀️.rs"]
mod artifact_cas;

#[path = "📜️event-log/🦀️.rs"]
mod event_log;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;
