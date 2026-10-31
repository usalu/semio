//! 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
//! STUDIOS, contract C1): `DirectoryEvent`/`DirectoryEventBody` (persisted, backend-assigned dense
//! `seq`), `DirectoryCommand` (client intent posted to `/directory/commands`), `DirectoryStreamMessage`
//! (the `/directory/socket/v1` wire envelope), and the read DTOs (`SpaceView`/`MemberView`/`UserView`/
//! `ConnectionView`/`DocumentView`/`InviteView`) the hub's REST surface returns. Pure data, no fold
//! logic — see the module root `../🦀️.rs`'s `DirectoryReadModel`/`fold`. `DirectorySpaceKind`/
//! `DirectorySpaceVisibility`/`DirectorySpaceRole` mirror `🪐️space`'s `SpaceKind`/`SpaceVisibility`/
//! `SpaceRole` vocabulary (atelier/studio/archive, private/public, author/spectator) string-identically;
//! this wasm-safe kernel crate does not mount that module (`🦀️.rs`'s header note: unwired pending
//! dep-DAG cleanup), so the enums are re-declared here, same convention `🌎️hub/📇️directory`'s
//! `SpaceRole` already uses for the same reason.
//!
//! 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `space_kind`
//! (`spaceKind` on the wire), not contract-freeze.md's bare `kind` — both bodies are internally
//! tagged (`#[value(tag = "kind")]`), so a same-named payload field would collide with the
//! discriminator. Flagged as a `sharedFileRequest` in lane 0-A's report.
//!
//! 🌉️ `ToValue`/`FromValue` (`#[derive(ToValue, FromValue)]`, not a `serde_json`-backed bridge):
//! unblocked by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
//! AND-ARTIFACTS/🔍️research/📓️dslvalue-integer-fidelity.md` — `DslValue::Number` now carries
//! `UInt`/`Int`/`Float` (not a lone `f64`), so `CreateInvite.ttl_secs: u64` etc. round-trip as bare
//! integers (`3600`, never `3600.0`) the way this contract's real external hub (`🌎️hub`'s sibling
//! Rust/serde types, strict — no `arbitrary_precision`) requires on the wire. An earlier pass
//! (`📓️directory-spr-serde-removal.md`) declined this conversion for exactly that reason, before the
//! fix landed. `#[value(...)]` mirrors every `#[serde(...)]` shape this file used: `tag` +
//! `rename_all_fields`, per-variant `rename`, and mixed `rename_all` casings across sibling enums —
//! all supported by `semio_framework_value_derive` today (see its own header docs).

use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🪪️session-authority-v1/🦀️.rs"]
pub mod session_authority;
pub use session_authority::DIRECTORY_SESSION_AUTHORITY_MAX_BYTES;
pub use session_authority::DirectorySessionAuthorityV1;
pub use session_authority::DirectorySessionKindV1;

#[path = "🌱️space-artifact-creation-v1/🦀️.rs"]
pub mod space_artifact_creation;

#[path = "📇️document-index-v1/🦀️.rs"]
pub mod document_index;
pub use document_index::DirectoryIndexedDocumentViewV1;
pub use document_index::DocumentIndexEntryV1;

pub use semio_framework_os_directory_browser_actor::DOCUMENT_BROWSER_ACTOR_INTERFACES;
pub use semio_framework_os_directory_browser_actor::DOCUMENT_BROWSER_ACTOR_MAX_BYTES;
pub use semio_framework_os_directory_browser_actor::DocumentBrowserActorByteLengthV1;
pub use semio_framework_os_directory_browser_actor::DocumentBrowserActorErrorV1;
pub use semio_framework_os_directory_browser_actor::DocumentBrowserActorSourceV1;
pub use semio_framework_os_directory_browser_actor::DocumentExecutionTargetBrowserActorV1;
pub use semio_framework_os_directory_browser_actor::DocumentOpenBrowserActorV1;

#[path = "📖️events-commands-and-admin/🦀️.rs"]
mod events_commands_and_admin;
pub use events_commands_and_admin::DirectorySpaceKind;
pub use events_commands_and_admin::DirectorySpaceVisibility;
pub use events_commands_and_admin::DirectorySpaceRole;
pub use events_commands_and_admin::DocumentScope;
pub use events_commands_and_admin::ArtifactHash;
pub use events_commands_and_admin::CheckpointId;
pub use events_commands_and_admin::DirectoryActorKind;
pub use events_commands_and_admin::DirectoryActor;
pub use events_commands_and_admin::Hlc;
pub use events_commands_and_admin::DirectoryEventBody;
pub use events_commands_and_admin::DirectoryEvent;
pub use events_commands_and_admin::directory_event_page_has_control;
pub use events_commands_and_admin::DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS;
pub use events_commands_and_admin::DIRECTORY_EVENT_PAGE_MAX_BYTES;
pub use events_commands_and_admin::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES;
pub use events_commands_and_admin::DIRECTORY_PREFERENCE_PAGE_PATH_V1;
pub use events_commands_and_admin::USER_PREFERENCE_SCHEMA_ID_MAX_BYTES;
pub use events_commands_and_admin::USER_PREFERENCE_MUTATION_MAX_BYTES;
pub use events_commands_and_admin::DirectoryEventPageV1;
pub use events_commands_and_admin::DirectoryEventPageErrorV1;
pub use events_commands_and_admin::DirectoryCommand;
pub use events_commands_and_admin::EditedArtifactFrontierV1;
pub use events_commands_and_admin::DIRECTORY_COMMAND_REQUEST_MAX_BYTES;
pub use events_commands_and_admin::DIRECTORY_COMMAND_RECEIPT_MAX_BYTES;
pub use events_commands_and_admin::DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS;
pub use events_commands_and_admin::DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES;
pub use events_commands_and_admin::DIRECTORY_COMMAND_REQUEST_ID_LEN;
pub use events_commands_and_admin::DirectoryCommandRequestV1;
pub use events_commands_and_admin::DirectoryCommandOutcomeV1;
pub use events_commands_and_admin::DirectoryCommandResultV1;
pub use events_commands_and_admin::DirectoryCommandReceiptV1;
pub use events_commands_and_admin::DirectoryCommandErrorCodeV1;
pub use events_commands_and_admin::mint_directory_command_request_id;
pub use events_commands_and_admin::AdminIntentV1;
pub use events_commands_and_admin::AdminIntentStateV1;
pub use events_commands_and_admin::AdminIntentOutcomeV1;
pub use events_commands_and_admin::AdminIntentResultV1;
pub use events_commands_and_admin::AdminIntentReceiptV1;
pub use events_commands_and_admin::AdminOperationProgressV1;
pub use events_commands_and_admin::AdminOperationStatusV1;
pub use events_commands_and_admin::AdminPageV1;
pub use events_commands_and_admin::AdminRecordedConnectionV1;
pub use events_commands_and_admin::AdminConnectionSnapshotV1;
pub use events_commands_and_admin::AdminOperationAuditPhaseV1;
pub use events_commands_and_admin::AdminOperationAuditV1;

#[path = "🪟️views-and-document-open/🦀️.rs"]
mod views_and_document_open;
pub use views_and_document_open::SpaceView;
pub use views_and_document_open::PublicSpaceViewV1;
pub use views_and_document_open::MemberSpaceViewV1;
pub use views_and_document_open::PublicDocumentCatalogEntryV1;
pub use views_and_document_open::DirectorySpaceListEntryV1;
pub use views_and_document_open::DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS;
pub use views_and_document_open::DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES;
pub use views_and_document_open::DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES;
pub use views_and_document_open::DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA;
pub use views_and_document_open::DirectorySpaceAdministrationSectionV1;
pub use views_and_document_open::DirectorySpaceAdministrationMemberRowV1;
pub use views_and_document_open::DirectorySpaceAdministrationInviteRowV1;
pub use views_and_document_open::DirectorySpaceAdministrationMemberWindowV1;
pub use views_and_document_open::DirectorySpaceAdministrationInviteWindowV1;
pub use views_and_document_open::DirectorySpaceAdministrationDocumentWindowV1;
pub use views_and_document_open::DirectorySpaceAdministrationPublicDocumentWindowV1;
pub use views_and_document_open::DirectorySpaceAdministrationCapabilitiesV1;
pub use views_and_document_open::DirectorySpaceAdministrationPageV1;
pub use views_and_document_open::DirectorySpaceAdministrationPageErrorV1;
pub use views_and_document_open::MemberView;
pub use views_and_document_open::UserView;
pub use views_and_document_open::ConnectionView;
pub use views_and_document_open::DocumentOwner;
pub use views_and_document_open::DocumentFrontier;
pub use views_and_document_open::DocumentDescriptor;
pub use views_and_document_open::DOCUMENT_OPEN_ID_MAX_BYTES;
pub use views_and_document_open::DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES;
pub use views_and_document_open::DOCUMENT_OPEN_PLAN_MAX_TTL_MS;
pub use views_and_document_open::DOCUMENT_OPEN_MAX_SAFE_INTEGER;
pub use views_and_document_open::DocumentOpenIntentV1;
pub use views_and_document_open::DocumentOpenRendererTargetV1;
pub use views_and_document_open::DocumentOpenSurfaceRoleV1;
pub use views_and_document_open::DocumentExecutionProtocolV1;
pub use views_and_document_open::DocumentOpenPackageV1;
pub use views_and_document_open::DocumentOpenCatalogV1;
pub use views_and_document_open::DocumentOpenArtifactV1;
pub use views_and_document_open::DocumentOpenParentDialectV1;
pub use views_and_document_open::DocumentOpenSurfaceV1;
pub use views_and_document_open::DocumentOpenGrantV1;
pub use views_and_document_open::DocumentOpenCheckpointV1;
pub use views_and_document_open::DocumentOpenRevalidationV1;
pub use views_and_document_open::DocumentOpenPlanV1;
pub use views_and_document_open::DocumentPlanSocketGrantIntentV1;
pub use views_and_document_open::DocumentOpenPlanErrorCodeV1;
pub use views_and_document_open::DocumentOpenPlanErrorV1;
pub use views_and_document_open::valid_document_open_hash;

#[path = "🛰️execution-target-artifacts-and-stream/🦀️.rs"]
mod execution_target_artifacts_and_stream;
pub use execution_target_artifacts_and_stream::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;
pub use execution_target_artifacts_and_stream::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES;
pub use execution_target_artifacts_and_stream::DocumentExecutionTargetComponentV1;
pub use execution_target_artifacts_and_stream::DocumentExecutionTargetDescriptorV1;
pub use execution_target_artifacts_and_stream::DocumentExecutionTargetLeaseFieldsV1;
pub use execution_target_artifacts_and_stream::lease_fields_from_plan_v1;
pub use execution_target_artifacts_and_stream::same_lease_fields_v1;
pub use execution_target_artifacts_and_stream::DocumentExecutionTargetStatusCodeV1;
pub use execution_target_artifacts_and_stream::DocumentExecutionTargetLocaleV1;
pub use execution_target_artifacts_and_stream::DescriptorDigestError;
pub use execution_target_artifacts_and_stream::validate_document_descriptor_v1;
pub use execution_target_artifacts_and_stream::ArtifactFrontier;
pub use execution_target_artifacts_and_stream::ArtifactBlobRef;
pub use execution_target_artifacts_and_stream::PublishedArtifactBlob;
pub use execution_target_artifacts_and_stream::PublishedArtifactCheckpoint;
pub use execution_target_artifacts_and_stream::ArtifactCheckpoint;
pub use execution_target_artifacts_and_stream::ArtifactRetention;
pub use execution_target_artifacts_and_stream::DocumentView;
pub use execution_target_artifacts_and_stream::InviteView;
pub use execution_target_artifacts_and_stream::DirectoryConnectionPhase;
pub use execution_target_artifacts_and_stream::DirectoryPresenceActor;
pub use execution_target_artifacts_and_stream::RebootstrapRequired;
pub use execution_target_artifacts_and_stream::DirectoryAccessChange;
pub use execution_target_artifacts_and_stream::DirectoryStreamMessage;

#[path = "../🚪️io/📝️text/🦀️.rs"]
pub mod text;
