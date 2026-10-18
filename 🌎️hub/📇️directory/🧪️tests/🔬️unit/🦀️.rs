use semio_hub_artifact_creation::io::artifact_creation_command_digest_v1;
use semio_hub_directory_sqlite::SqliteDirectory;

use semio_hub_artifact_authority::chunk_cas::{ArtifactCasObjectKind, ArtifactChunkBlobStore, FsArtifactChunkCasStorage, MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use semio_hub_artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, AuthorityProgressStage, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob};
use db::storage::{DB_IO_PAGE_BYTES, DbIoPageWriter, MemoryStorage as GenericMemoryStorage, PayloadStorage};
use directory::{DslValue, FromValue, ToValue};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use semio_hub_directory_contract::{ADMIN_PAGE_FETCH_MAX, ADMIN_PAGE_MAX, ARTIFACT_CAS_DELETE_LEASE_TTL_MS, ARTIFACT_CAS_SWEEP_OBJECT_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES, AUTH_ASSERTION_MAX_BYTES, CAPABILITY_MAX_TTL_SECS, DEVICE_INSTANCE_MAX_BYTES, DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DIRECTORY_WIRE_INTEGER_MAX, HubCapability, HubClock, IdentityAssertion, IdentityAssertionVerifier, IdentityAssurance, IdentityVerificationContext, IdentityVerificationControl, IdentityVerificationFuture, IdentityVerificationProgress, InviteCapability, NewDirectoryEvent, SessionCapability, ShareCapability, SocketGrantCapability, VerifiedIdentity, capability_window, constant_time_digest_eq, encode_capability_bytes, identity_subject_digest, now_ms, prepare_auth_session};
use semio_hub_artifact_creation::{ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationActorV1, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationIntentV1, ArtifactCreationPreparedV1};
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AuthSessionIssue, AuthSessionKind, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, InviteRedemptionCommit, NewAdminOperationAuditRecord, NewDirectoryCommandReceipt, SpaceRole};
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasDeleteOutcome, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, ArtifactChunkCasStorage};
use semio_hub_directory_service::{ArtifactCasSweepRequest, ArtifactDirectoryCommand, DirectoryCommandExecutionV1, DirectoryService, directory_command_result_kind, new_event};
use directory::os_directory::{ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentScope, Hlc, PublishedArtifactBlob, PublishedArtifactCheckpoint};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_directory::{AdminOperationDirectory, ArtifactCasDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ReceiptDirectory, SpaceDirectory, UserDirectory, MemoryArtifactProjection, ProjectionRebuildControl, ProjectionRebuildProgress, bounded_event_read, checkpoint_identity_input, checkpoint_projection_rebuild, published_artifact_checkpoint, validate_checkpoint_shape, validate_document_descriptor, validate_retention_shape, validate_verified_checkpoint_append};
use semio_framework_hash::Sha256;
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use directory::os_identity::time_ordered_id;

include!("🪪️identity-laws/🦀️.rs");

include!("🧰️artifact-fixtures/🦀️.rs");

include!("📒️admin-and-event-log-laws/🦀️.rs");

include!("🧺️artifact-cas-laws/🦀️.rs");

include!("🏘️space-and-membership-laws/🦀️.rs");

include!("🎟️invite-and-index-laws/🦀️.rs");
