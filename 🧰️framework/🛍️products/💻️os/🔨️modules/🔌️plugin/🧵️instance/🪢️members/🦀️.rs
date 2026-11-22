//! 🪢️ Instance use-cases of the plugin SDK: the transactions, document load, render surface, action dispatch, typed operation results and media transfer members of the mounted artifact over the members bundle.
//!
//! 🧭️ Root of `semio-plugin-instance-members`; the composer builds `MembersCtx` and delegates its `PluginApp` members here.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;

use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep, plugin_retirement_fault, plugin_turn_admitted};
use semio_plugin_decl_media::{ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, ArtifactDownloadOutput, ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_plugin_decl_media::original_media::{RawMediaPicker, issue::pending_picker_effect, runtime::RawLiveDocument};
use semio_framework::{ActionKind, AppRole, Fault, FaultCode, FaultFrom, FaultOrigin, Media, MediaClass, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, ViewModel};
use semio_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_plugin_document_media::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState, MediaArtifact, copy_media_text_parts, document_archive_history_decoder, whole_document_media};
use semio_framework::kernel::{AppEvent, Effect, FRAMEWORK_RESERVED_JOB_KIND, FixedCommandPage, HistoryPatch, InvocationResult, UiDirtyScope};
use semio_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_BUSY_CODE, DOCUMENT_LOAD_CHANGED_CODE, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_HISTORY_INVALID_CODE, DOCUMENT_LOAD_INCOMPLETE_CODE, DOCUMENT_LOAD_MEMBER_INVALID_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_TOO_LARGE_CODE, DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, EphemeralSnapshot, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption, NATURAL_FILE_PORT, TransactionPrepareOutcome, document_load_fault, media_projection_error, natural_file_input_media, whole_document_archive};
use store::os_store::store_schema::{ArtifactCommand, ArtifactEnvelope};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_plugin_decl_app::tool_contract::{ArtifactToolCompletionValue, ArtifactToolPublicContract, ToolOwnerWitness, TypedOperationResultLane};
use semio_plugin_decl_views::{ArtifactView, ConfigView, InteractionView};
use std::collections::{BTreeMap, HashMap};
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_ui_runtime::ComponentTree;
use semio_plugin_composition_publication::{ComposedGestureResult, MountedTypedCommandFullOperationStage};
use semio_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, FRAMEWORK_HISTORY_BODY_KEY, WindowEngagement, WindowMeasure};
use semio_framework_value::{DslValue, RetirementDemand, ValueError};
use semio_plugin_job::{EmitWire, PendingTransaction, TransactionProposalDraft, TypedOperationCompletion, TypedOperationResultPage, TypedOperationResultToken, TypedOperationUiProgress};
use semio_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use semio_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework::manifest::CommandAddress as ManifestCommandAddress;
use std::mem::ManuallyDrop;
use semio_plugin_instance_docs::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, ReferenceSpaceMember, SpaceMember};
use semio_plugin_mounted_owner::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use protocol::OpText;
use semio_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_plugin_peer::{PeerRosterPublication, PresenceRosterAdmission, PresenceRosterOutcome};
use semio_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_plugin_window_transient::window_transient::TransientView;
use semio_plugin_window_config::WindowConfigPack;
use semio_plugin_window_transient::built_to_component_tree;
use store::os_store::document_materialize::create_document_envelope;
use semio_plugin_instance_dispatch::dispatch_emit;
use semio_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job;
use semio_plugin_window_kits::{merge_ui_values, validate_ui_dispatch_classification};
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_instance_render as render;
use semio_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_ui_contract::*;
use semio_plugin_tool_run::TOOL_RUN_TRACE_DELTA_BYTES;
use semio_plugin_instance_commands::commands_instance_root::framework_reserved_action_kind;
use semio_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;

#[path = "📥️staging/🔀️transactions/🦀️.rs"]
mod members_transactions;
#[path = "📥️staging/🗃️document-load/🦀️.rs"]
mod members_document_load;
#[path = "📥️staging/🖼️render-surface/🦀️.rs"]
mod members_render_surface;
#[path = "📥️staging/📨️action-dispatch/🦀️.rs"]
mod members_action_dispatch;
#[path = "📥️staging/🧾️typed-operation-results/🦀️.rs"]
mod members_typed_operation_results;
#[path = "📥️staging/🎞️media-transfer/🦀️.rs"]
mod members_media_transfer;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod members_bundle;

pub use members_bundle::MembersCtx;
pub use members_document_load::document_archive;
