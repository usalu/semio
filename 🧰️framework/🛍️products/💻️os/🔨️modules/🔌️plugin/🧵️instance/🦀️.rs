//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use crate::child_content::mounted_child_group_receipt;
use crate::child_content::mounted_child_group_receipt_group::command::CommandAppendRefusal;
use crate::child_content::private_publication_group::owner::PrivateChildPublicationGroup;
use crate::child_content::private_publication_group::owner::PrivateChildGroupSource;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntrySource;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntry;
use crate::child_content::mounted_child_group_receipt_group::receipt_members::MountedGroupMemberReceipts;
use crate::gesture_contract::GestureCapture;
use crate::app_views::ArtifactOperationProgress;
use crate::gesture_contract::GestureSlot;
use crate::tool_run_contract::ToolRunJobRequest;
use crate::tool_run_contract::ToolRunJobPurpose;
use crate::tool_run_contract::ToolRunJobPort;
use crate::tool_run_contract::ToolRunTraceKeys;
use crate::plugin_runtime::LAST_MAINTENANCE_STAGE;
use crate::reserved_jobs::HISTORY_ACTION_IDS;
use crate::reserved_jobs::CLIPBOARD_ACTION_IDS;
use crate::reserved_jobs::is_framework_reserved_action_id;
#[cfg(any(test, feature = "artifact-app-testing"))] use std::sync::Arc;
#[cfg(test)] use crate::command_history::HISTORY_ROWS_BUILT;
#[cfg(test)] use crate::reserved_jobs::FrameworkCutJob;
use crate::interaction_topology::INTERACTION_ACTION_IDS;
use crate::document_ingress::ActiveDocumentArchiveLoadPhase;
use crate::document_ingress::CompositionPinsRetirement;
use crate::document_ingress::publish_boxed_document_store_candidate_if_authoritative;
use crate::document_ingress::ArtifactStoreReplacementPublicationGuard;
use crate::child_view::child_content_arc_bytes;
use crate::peer_roster::PeerRosterPublicationStage;
use crate::child_view::CHILD_CONTENT_SLOTS;
use crate::media_contract::ARTIFACT_OUTPUT_CHUNK_BYTES;
use crate::tool_cancellation::TOOL_CANCELLATION_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::plugin_turn_admitted;
use crate::command_history::HISTORY_STEP_BLOCKED_CODE;
use crate::tree_kit::TREE_WINDOW_DEFAULT_ROWS;
use crate::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use super::child_emit_preparation::ChildEmitPreparationStep;
use super::window_config::{WindowConfigMutation, WindowConfigOwnerRegistry, WindowConfigPack};
use super::window_transient::{WindowTransientOwnerRegistry, WindowTransientSnapshot};
use semio_framework_value::DslValue;
use protocol::OpText;
use semio_framework::kernel::{FixedCommandPage, UiDirtyScope};
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandInvocation as ManifestCommandInvocation, CommandAddress as ManifestCommandAddress, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework::{kernel::{ActorId, AppEvent, ArtifactDiff, ArtifactHandle, ArtifactVersion, ClipboardFragment, EditRef, Effect, HistoryEntry, HistoryPatch, HybridLogicalTimestamp, InverseMutation, InvocationId, InvocationResult, KernelMutation, MutationId, SchemaId, UndoGroup, UndoPolicy}, ActionArgDef, ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin, InteractionDefinition, InteractionVerb, MediaForm, ToolRunTraceCursor, ViewModel, NOTE_SHELL_COMMAND_ACTION_ID, RECORD_TUTORIAL_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::{ComponentTree, TreeNode};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use store::{build_history_columns, create_config_envelope, os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand, os_store::store_schema::ArtifactEnvelope, ArtifactStore, os_store::composition_coordinator::ChildDispatch, os_store::composition_coordinator::CompositionCoordinator, ConfigStore, os_store::composition_coordinator::GroupMeta, os_store::store_schema::HistoryLane, MemberFactory, Mutation, NoMembers, SpaceMember};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use ui_wgpu::wgpu::ContextMenuItemSpec;
use ui_wgpu::wgpu::ContextMenuRequest;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowMeasure;
use ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY;
/// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
/// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
/// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
/// The legacy format enum was retired in ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
/// Binary` now carries a plain format kind id string.
use semio_framework::MediaWireFormat;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
/// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
use semio_framework::{MediaClass, MediaType};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use crate::local_interaction::mutations::InteractionConfigMutation;
use super::completion_fault::ArtifactCompletionFault;
use crate::owned_context::ArtifactOwnedContextHandle;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework::AppRole;
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::app_emission::EphemeralEmit;
use crate::app_contract::copy_media_text_parts;
use crate::app_contract::media_projection_error;
use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use crate::typed_operation::PendingPublicationOutcome;
use crate::tool_jobs::tool_machine::ToolDispatch;
use crate::child_content::advance_original_child_root_retirement;
use crate::child_content::original_child_root_retirement_demands;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use crate::child_content::MountedPrivateChildGroup;
use crate::child_content::MountedCommandPrune;
use crate::child_content::PagedCommandLog;
use crate::document_ingress::OriginalCatalogConstructor;
use crate::child_content::private_publication_group::PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN;
use crate::window_kits::AppActionRegistry;
use crate::app_contract::ArtifactApp;
use crate::app_contract::DOCUMENT_LOAD_BUSY_CODE;
use crate::app_contract::DOCUMENT_LOAD_CHANGED_CODE;
use crate::app_contract::DOCUMENT_LOAD_FAILED_CODE;
use crate::app_contract::DOCUMENT_LOAD_HISTORY_INVALID_CODE;
use crate::app_contract::DOCUMENT_LOAD_INCOMPLETE_CODE;
use crate::app_contract::DOCUMENT_LOAD_MEMBERS_DIFFER_CODE;
use crate::app_contract::DOCUMENT_LOAD_MEMBER_INVALID_CODE;
use crate::app_contract::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE;
use crate::app_contract::DOCUMENT_LOAD_OTHER_DOCUMENT_CODE;
use crate::app_contract::DOCUMENT_LOAD_TOO_LARGE_CODE;
use crate::app_contract::DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE;
use crate::app_contract::EphemeralSnapshot;
use crate::app_contract::MediaArtifact;
use crate::app_contract::MediaArtifactDescriptor;
use crate::app_contract::MediaArtifactError;
use crate::app_contract::MediaConsumption;
use crate::app_contract::NATURAL_FILE_PORT;
use crate::plugin_app::PluginApp;
use crate::app_contract::TransactionPrepareOutcome;
use crate::app_contract::document_load_fault;
use crate::app_contract::natural_file_input_media;
use crate::window_kits::validate_ui_dispatch_classification;
use crate::app_contract::viewer_rejects_action;
use crate::app_contract::whole_document_archive;
use crate::app_contract::whole_document_media;
use crate::app_emission::ActionMeta;
use crate::app_emission::ChildEmit;
use crate::app_emission::Emit;
use crate::app_emission::HostEvent;
use crate::app_emission::InteractionWrite;
use crate::app_emission::TransactionPhase;
use crate::app_views::AppRenderOperationContext;
use crate::app_views::ArtifactView;
use crate::app_views::ConfigView;
use crate::app_views::InteractionHoverState;
use crate::app_views::InteractionView;
use crate::window_transient::TransientView;
use crate::artifact_registration::PluginRuntimeRegistry;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedQueue;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_ownership::bounded_ownership::DOCUMENT_ARCHIVE_LOAD_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::bounded_ownership::plugin_demand_grant;
use semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault;
#[cfg(test)]
use crate::child_content::ChildAdmissionTestState;
use crate::child_content::ChildContentRetirement;
use crate::child_view::ChildContentView;
use crate::child_content::ChildMemberEntry;
use crate::child_content::ChildMemberRegistrationError;
use crate::child_content::ChildMemberRegistry;
use crate::child_content::ChildMemberRetirement;
use crate::child_content::ChildStoreAdmission;
use crate::child_content::DocumentClosureSourceView;
use crate::child_view::MemberKey;
use crate::child_view::MemberKeyRef;
use crate::child_content::MemberPath;
use crate::child_content::OwnedDocumentMemberIngress;
use crate::child_content::SnapshotReadReturnPump;
use crate::command_admission::AdmittedToolCommand;
use crate::command_admission::PendingLatestWinsCommand;
use crate::command_admission::ToolLatestWinsRegistry;
use crate::command_admission::TypedCommandWireWriter;
use crate::command_history::AlternativeFact;
use crate::history_view::AlternativeView;
use crate::command_history::CATALOGUE_EXAMPLE_ACTION_ID;
use crate::command_history::CommandLogAppend;
use crate::command_history::CommandLogAppendKey;
use crate::history_view::CommandLogEntry;
use crate::history_view::CommandView;
use crate::command_history::DOCUMENT_LOADING_CODE;
use crate::command_history::HISTORY_ROW_OPERATION_PREVIEW;
use crate::command_history::HistoryBackfillMark;
use crate::history_view::HistoryCommandFilter;
use crate::command_history::HistoryRowReads;
use crate::history_view::HistoryView;
use crate::command_history::HistoryViewCache;
use crate::command_history::HistoryViewPatch;
use crate::command_history::HistoryViewStamp;
use crate::history_view::InverseAction;
use crate::history_view::MutationView;
use crate::command_history::PURE_HISTORY_UNAVAILABLE_CODE;
use crate::history_view::ShellHistoryReplay;
use crate::command_history::backfilled_edit_label;
use crate::command_history::history_alternative_views;
use crate::command_history::history_intent_label;
use crate::command_history::history_leaf_row_label;
use crate::command_history::history_row_applied;
use crate::command_history::history_row_is_recorded;
use crate::command_history::runtime_verb_label;
use crate::command_history::ui_history_panel;
use crate::document_ingress::ActiveArtifactEnvelopeDecode;
use crate::document_ingress::ActiveArtifactEnvelopeDecodeState;
use crate::document_ingress::ActiveArtifactEnvelopeIngress;
use crate::document_ingress::ActiveArtifactStoreReplacement;
use crate::document_ingress::ActiveArtifactStoreReplacementState;
use crate::document_ingress::ActiveDocumentArchiveLoad;
use crate::document_ingress::ActiveDocumentArchiveLoadState;
use crate::document_ingress::AppProjectionCache;
use crate::document_ingress::ArtifactEnvelopeDecodeOperationHandle;
use crate::document_ingress::ArtifactEnvelopeDecodeOperationPoll;
use crate::document_ingress::ArtifactStoreConstructorKind;
use crate::document_ingress::ArtifactStoreConstructorPhase;
use crate::document_ingress::ArtifactStoreReplacementAdmissionTarget;
use crate::document_ingress::ArtifactStoreReplacementRefusal;
use crate::document_ingress::PendingDocumentArchiveMember;
use crate::interaction_topology::AppliedInteractionVerb;
use crate::interaction_topology::INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE;
use crate::interaction_topology::InlineInteractionFold;
use crate::interaction_topology::InteractionRevalidateOrigin;
use crate::interaction_topology::derive_path_delimited_topology;
use crate::interaction_topology::empty_draft_references;
use crate::interaction_topology::interaction_after_document_change_v1;
use crate::interaction_topology::interaction_domain_id_arg;
use crate::interaction_topology::interaction_selection_loss_v1;
use crate::interaction_topology::is_inline_interaction_verb;
use crate::interaction_topology::leftover_after_app_selection_write_v1;
use crate::interaction_topology::leftover_gumball_active_v1;
use crate::interaction_topology::leftover_selected_ids_of;
use crate::interaction_topology::parse_interaction_targets;
use crate::interaction_topology::parse_merge_mode;
use crate::interaction_topology::parse_selection_mode;
use crate::interaction_topology::report_interaction_selection_loss;
use crate::interaction_topology::take_inline_interaction_verbs;
use crate::interaction_topology::ui_tree_domain_topology;
use crate::media_contract::ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES;
use crate::media_export::ActiveMediaExport;
use crate::media_export::ActiveMediaExportCloseStage;
use crate::media_contract::ArtifactDownloadOutput;
use crate::media_contract::ArtifactMediaExportCompletion;
use crate::media_contract::ArtifactMediaExportCredit;
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportJobRequest;
use crate::media_contract::ArtifactMediaExportPoll;
use crate::media_contract::ArtifactMediaExportResult;
use crate::media_contract::ArtifactOutputChunks;
use crate::media_contract::validate_media_export_structure;
use crate::app_views::PeerPresenceRoot;
use crate::peer_roster::PeerPresenceRootRetirement;
use crate::peer_roster::PeerRosterCandidate;
use crate::peer_roster::PeerRosterPublication;
use crate::peer_roster::PresenceRosterAdmission;
use crate::peer_roster::PresenceRosterOutcome;
use crate::peer_roster::ValidatedPeerRosterCommit;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::reserved_jobs::FRAMEWORK_RESERVED_ROUTE_CAPACITY;
use crate::reserved_jobs::FrameworkConfigurationBinaryJob;
use crate::reserved_jobs::FrameworkConfigurationBinaryStage;
use crate::reserved_jobs::FrameworkReservedCommit;
use crate::reserved_jobs::FrameworkReservedCommitPermit;
use crate::reserved_jobs::FrameworkReservedCommitProgress;
use crate::reserved_jobs::FrameworkReservedCommitStage;
use crate::reserved_jobs::PendingFrameworkReserved;
use crate::reserved_jobs::QualifiedBoundedFirstStepProof;
use crate::reserved_jobs::QualifiedToolProof;
use crate::reserved_jobs::RESERVED_OUTCOME_ACKNOWLEDGE_TURNS;
use crate::tool_contract::ToolOwnerWitness;
use crate::reactor::jobs::reserved::drive_framework_reserved_registry_job;
use crate::reserved_jobs::encode_framework_reserved_job_input;
use crate::reserved_jobs::plugin_job_yield_once;
use crate::reserved_jobs::register_framework_reserved_factories;
use crate::reserved_jobs::reserved_pump_fault_detail;
use crate::reserved_jobs::reserved_pump_fault_is_transient;
use crate::reserved_jobs::retained_payload_eq_slice;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDocumentStoreDisposer;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseLease;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseRetention;
use crate::store_publication::ChildPublicationResultV1;
use crate::store_publication::ComposedGestureResult;
use crate::store_publication::MountedTypedCommandFullOperationStage;
use crate::store_publication::PendingArtifactStorePublication;
use crate::store_publication::PendingChildGroupPublication;
use crate::store_publication::PendingChildGroupPublicationPhase;
use crate::tool_cancellation::ArtifactDocumentAuthority;
use crate::tool_cancellation::ToolCancellationHandle;
use crate::tool_cancellation::ToolCancellationLease;
use crate::tool_cancellation::ToolOperationKey;
use crate::tool_jobs::AgentLaneCarriage;
use crate::tool_jobs::AgentLanePublication;
use crate::tool_contract::ArtifactBoundedToolFault;
use crate::tool_contract::ArtifactInstanceOperationOwnerHandle;
use crate::tool_contract::ArtifactOwnedToolJobContext;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactOwnedToolJobSnapshots;
use crate::tool_contract::ArtifactReservedToolInput;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletion;
use crate::tool_contract::ArtifactToolCompletionValue;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::ArtifactToolPublicContract;
use crate::tool_contract::ArtifactToolPublicationLane;
use crate::tool_contract::ArtifactToolRawInput;
use crate::tool_contract::ArtifactToolRegistration;
use crate::app_views::CANCEL_TYPED_OPERATION_ACTION_ID;
use crate::tool_jobs::ChildEmissionPreviewJob;
use crate::tool_jobs::DownloadResultPayload;
use crate::tool_jobs::EmitWire;
use crate::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use crate::tool_jobs::INTERACTIVE_TURN_WORKER_PUMPS;
use crate::tool_jobs::INTERACTIVE_TURN_WORKER_WALL_US;
use crate::tool_jobs::PendingTransaction;
use crate::tool_jobs::SupersedeLedger;
use crate::tool_jobs::SupersedeRole;
use crate::tool_contract::TYPED_OPERATION_FAULT_PAGE_BYTES;
use crate::tool_jobs::TYPED_OPERATION_HOST_OUTBOX_SLOTS;
use crate::tool_jobs::TYPED_OPERATION_MAXIMUM_RETRIES;
use crate::tool_jobs::TYPED_OPERATION_PUBLICATION_PUMPS;
use crate::tool_jobs::TYPED_OPERATION_STALL_FAULT_CEILING;
use crate::tool_jobs::TYPED_OPERATION_STALL_PROGRESS_MIX;
use crate::tool_jobs::TYPED_OPERATION_STALL_PUBLISHING_STAGE;
use crate::tool_jobs::TYPED_OPERATION_STALL_WITNESS_FLOOR;
use crate::tool_jobs::TimeTravelLedger;
use crate::tool_jobs::ToolMachineRuntime;
use crate::tool_jobs::ToolRunLedger;
use crate::tool_jobs::ToolTag;
use crate::tool_jobs::TransactionProposalDraft;
use crate::tool_jobs::TypedCommandRoots;
use crate::tool_jobs::TypedOperationCompletion;
use crate::tool_jobs::TypedOperationCompletionWitness;
use crate::tool_jobs::TypedOperationLeftover;
use crate::tool_contract::TypedOperationResultLane;
use crate::tool_jobs::TypedOperationResultPage;
use crate::tool_jobs::TypedOperationResultToken;
use crate::tool_jobs::TypedOperationUiProgress;
use crate::tool_jobs::TypedOperationUnitKind;
use crate::tool_jobs::agent_lane_carriage;
use crate::tool_jobs::agent_lane_no_effect_fault;
use crate::tool_jobs::agent_lane_uncarried_fault;
use crate::tool_jobs::artifact_owned_tool_birth_grant;
use crate::tool_jobs::artifact_owned_tool_handoff_grant;
use crate::tool_jobs::drive_agent_lane_preview;
use crate::tool_jobs::history_mutation_entry;
use crate::app_contract::is_time_travel_action_id;
use crate::tool_jobs::is_tool_run_action_id;
use crate::tool_jobs::record_typed_operation_unit;
use crate::tool_jobs::time_travel;
use crate::tool_jobs::tool_run;
use crate::typed_operation::MountedTypedCommandFullOperation;
use crate::typed_operation::TypedCommandFullOperationJob;
use crate::typed_operation::TypedCommandFullOperationJobFactory;
use crate::typed_operation::TypedCommandFullOperationPhase;
use crate::typed_operation::bounded_json_items;
use crate::window_kits::merge_ui_values;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::built_to_component_tree;
use crate::window_surface::ui_assembly_error;
#[cfg(test)]
use crate::reactor::jobs::reserved::drive_framework_reserved_registry_job_counted;

fn original_render_posted_since(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
 let delta=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render recipient moved backwards"));
 Ok(RetainedCloneProgress{copied_items:delta(after.copied_items,before.copied_items)?,copied_bytes:delta(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:delta(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:delta(after.released_bytes,before.released_bytes)?})
}

struct OriginalRenderTreeIdentity {
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    parent: usize,
    recipient: usize,
    cancellation: usize,
    epoch: Option<u64>,
    clock: fn()->Option<u64>,
    deadline: u64,
}

impl OriginalRenderTreeIdentity {
    fn capture(cx:&semio_framework_job::StepContext<'_>)->Result<Self,ValueError> {
        Ok(Self { operation:cx.operation(), generation:cx.generation(), parent:cx.original_source_parent()?.source_identity()?, recipient:cx.original_recipient_identity(), cancellation:cx.original_cancel_token() as *const _ as usize, epoch:cx.original_source_epoch(), clock:cx.clock_source(), deadline:cx.deadline_us() })
    }
    fn validate(&mut self,cx:&semio_framework_job::StepContext<'_>)->Result<(),ValueError> {
        if self.operation!=cx.operation() || self.generation!=cx.generation() || self.parent!=cx.original_source_parent()?.source_identity()? || self.recipient!=cx.original_recipient_identity() || self.cancellation!=cx.original_cancel_token() as *const _ as usize || !std::ptr::fn_addr_eq(self.clock,cx.clock_source()) || self.deadline!=cx.deadline_us() || matches!((self.epoch,cx.original_source_epoch()),(Some(before),Some(after)) if after<before) || self.epoch.is_some()!=cx.original_source_epoch().is_some() {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"stationary render source requires its original parent recipient control and work lifetime"));
        }
        self.epoch=cx.original_source_epoch();
        Ok(())
    }
}

struct OriginalRenderInteractionStamp<'a> {
    registry: &'a AppActionRegistry,
    interaction_hover: &'a InteractionHoverState,
    peer_presence: &'a PeerPresenceRoot,
    own_color: Option<u8>,
    local_interaction_authority: &'a mut crate::local_interaction::authority::LocalInteractionTopologyAuthority,
    interaction_ui_topology: &'a mut HashMap<String, protocol::DomainTopology>,
    pending_presence: &'a mut Vec<PresenceUpdate>,
    presence_marked_keys: &'a mut BTreeMap<String, BTreeSet<String>>,
}

impl OriginalRenderInteractionStamp<'_> {
    pub(crate) async fn stamp_and_cache_interaction_ui(&mut self, tree: &ComponentTree, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        let hover = self.interaction_hover;
        let peers = self.peer_presence;
        let own_color = self.own_color;
        let interaction = InteractionView { state, hover, peers, draft_references: empty_draft_references() };
        struct Frame<'a> {
            node: &'a TreeNode,
            next: usize,
            entered: bool,
            domain: Option<UiText>,
        }
        let mut stack: UiFixedList<Frame<'_>, { semio_framework_ui_runtime::COMPONENT_TREE_PRODUCER_DEPTH }> = UiFixedList::default();
        let mut marked: BTreeSet<String> = BTreeSet::new();
        stack.try_push(Frame { node: &tree.root, next: 0, entered: false, domain: None }).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
        while let Some(mut frame) = stack.pop() {
            if !frame.entered {
                if let Component::Tree(props) = &frame.node.component {
                    if let Some(domain_id) = props.interaction_domain.clone() {
                        let granularity = self.registry.interaction(domain_id.as_str()).await.and_then(|def| def.granularities.first()).map(|granularity| granularity.id.clone()).unwrap_or_default();
                        let topology = ui_tree_domain_topology(&frame.node.children, &granularity).await?;
                        self.local_interaction_authority.before_cache_mutation().map_err(ui_assembly_error)?;
                        self.interaction_ui_topology.insert(domain_id.to_string(), topology);
                        frame.domain = Some(domain_id);
                    }
                }
                if let Some(domain_id) = frame.domain.as_ref() {
                    if self.derive_node_presence(frame.node, state, &interaction, domain_id.as_str(), body_key, own_color).await? {
                        marked.insert(frame.node.key.to_string());
                    }
                }
                frame.entered = true;
            }
            let Some(child) = frame.node.children.get(frame.next) else { continue };
            frame.next = frame.next.checked_add(1).ok_or_else(|| ui_assembly_error("interaction-walk.cursor"))?;
            let child_domain = frame.domain.clone();
            stack.try_push(frame).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
            stack.try_push(Frame { node: child, next: 0, entered: false, domain: child_domain }).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
        }
        self.retire_dropped_presence(body_key, &marked)?;
        Ok(())
    }

    /// 🧹️ Pushes one CLEARED `PresenceUpdate` per node key this body marked last render and did not
    /// mark this render, then records `marked` as the new set. `derive_node_presence` reports only
    /// nodes with something to say and `PresenceHub::record_own` holds own presence until it is
    /// overwritten, so this diff is the only write that ever says "this row is no longer selected" —
    /// without it a deselect is invisible and every renderer keeps the stale mark.
    fn retire_dropped_presence(&mut self, body_key: &str, marked: &BTreeSet<String>) -> UiAssemblyResult<()> {
        let previous = self.presence_marked_keys.get(body_key).cloned().unwrap_or_default();
        let surface = SurfaceId::try_from(body_key).map_err(|_| ui_assembly_error("presence.surface"))?;
        for key in previous.difference(marked) {
            self.pending_presence.push(PresenceUpdate { surface: surface.clone(), node_key: key.clone(), own: OwnPresence::default(), peers: Vec::new(), ttl_ms: PRESENCE_TTL_MS });
        }
        if marked.is_empty() {
            self.presence_marked_keys.remove(body_key);
        } else {
            self.presence_marked_keys.insert(body_key.to_string(), marked.clone());
        }
        Ok(())
    }

    /// 👥️ M2: derives `node`'s `PresenceUpdate` (if it has any own/peer selection or hover on
    /// `domain_id`) and pushes it onto `self.pending_presence`, answering whether it marked the node.
    /// A node with nothing to report is skipped entirely — an idle node costs no outbox entry,
    /// matching `PresenceHub::flush`'s own "only dirty keys" contract downstream; the `false` answer
    /// is what `retire_dropped_presence` diffs against to retire a mark this render dropped.
    async fn derive_node_presence(&mut self, node: &TreeNode, state: &protocol::InteractionState, interaction: &InteractionView<'_>, domain_id: &str, body_key: &str, own_color: Option<u8>) -> UiAssemblyResult<bool> {
        let key = node.key.as_str();
        let own_selected = state.selection.get(domain_id).is_some_and(|selection| selection.ids.iter().any(|id| id == key));
        let own_hovered = self.interaction_hover.get(domain_id).is_some_and(|hover| hover.ids.iter().any(|id| id == key));
        let selecting = interaction.peers_selecting(domain_id, key);
        let hovering = interaction.peers_hovering(domain_id, key);
        if !own_selected && !own_hovered && selecting.is_empty() && hovering.is_empty() {
            return Ok(false);
        }
        let mut marks: UiFixedList<PeerMark> = UiFixedList::default();
        for mark in &selecting {
            let existing = marks.iter().position(|entry| entry.actor == mark.actor);
            if let Some(index) = existing {
                if let Some(entry) = marks.get_mut(index) {
                    entry.selected = true;
                }
            } else {
                marks.try_push(PeerMark { actor: mark.actor.to_string(), color: mark.color, hovered: false, selected: true, label: mark.actor.to_string() }).map_err(|_| ui_assembly_error("presence.peer-marks"))?;
            }
        }
        for mark in &hovering {
            let existing = marks.iter().position(|entry| entry.actor == mark.actor);
            if let Some(index) = existing {
                if let Some(entry) = marks.get_mut(index) {
                    entry.hovered = true;
                }
            } else {
                marks.try_push(PeerMark { actor: mark.actor.to_string(), color: mark.color, hovered: true, selected: false, label: mark.actor.to_string() }).map_err(|_| ui_assembly_error("presence.peer-marks"))?;
            }
        }
        self.pending_presence.push(PresenceUpdate {
            surface: SurfaceId::try_from(body_key).map_err(|_| ui_assembly_error("presence.surface"))?,
            node_key: node.key.to_string(),
            own: OwnPresence { hovered: own_hovered, selected: own_selected, previewed: false, color: own_color },
            peers: marks.into_iter().collect(),
            ttl_ms: PRESENCE_TTL_MS,
        });
        Ok(true)
    }

}

struct MediaExportPreparation<A:ArtifactApp>{
 canonical_revision:[u8;32],metadata:ManuallyDrop<Option<[Vec<u8>;6]>>,metadata_stage:usize,metadata_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<[Vec<u8>;6]>>>,
 request_close:ManuallyDrop<Option<crate::media_contract::ArtifactMediaExportRequestRetirement<A>>>,parts:ManuallyDrop<Option<crate::media_export::MediaExportOwnedParts>>,parts_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<crate::media_export::MediaExportOwnedParts>>>,handle:ManuallyDrop<Option<ArtifactMediaExportHandle>>,handle_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportHandle>>>,operation:Option<semio_framework_job::Operation>,body_ready:bool,operation_bound:bool,assembled:bool,published:bool,
 snapshot_source:semio_framework_plugin_ownership::store_lifecycle::ArtifactMediaSnapshotSource<A::Snapshot>,
 snapshot:ManuallyDrop<Option<std::sync::Arc<A::Snapshot>>>,snapshot_lease:ManuallyDrop<Option<ArtifactSnapshotCloseLease<A::Snapshot>>>,snapshot_retention:ManuallyDrop<Option<ArtifactSnapshotCloseRetention>>,
 request:ManuallyDrop<Option<ArtifactMediaExportJobRequest<A>>>,body:ManuallyDrop<Option<Box<dyn crate::tool_contract::ArtifactReservedJob+Send>>>,producer:ManuallyDrop<Option<ArtifactReservedToolJob>>,
 source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),port_identity:(usize,usize),key:ToolOperationKey,
 lease:Option<ToolCancellationLease>,retirement:Option<Box<crate::tool_cancellation::ToolCancellationLeaseRetirement>>,
 completion:Option<ArtifactMediaExportCompletion>,completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportCompletion>>,
 credit:Option<ArtifactMediaExportCredit>,credit_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportCredit>>,
 chunks:Option<ArtifactOutputChunks>,chunks_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactOutputChunks>>,
}

impl<A:ArtifactApp> MediaExportPreparation<A>{
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  use semio_framework_job::InteractiveJob;
  let one=RetirementDemand{depth:1,..Default::default()};
  if let Some(job)=self.producer.as_ref(){return if job.terminal_is_empty(){Ok(one)}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.max(1)})}}
  if let Some(job)=self.body.as_ref(){return if job.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(&**job),..one})}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.max(1)})}}
  if let Some(owner)=self.request_close.as_ref(){return if owner.terminal_is_empty(){Ok(one)}else{owner.retirement_demands(body)}}
  if self.request.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<crate::media_contract::ArtifactMediaExportRequestRetirement<A>>(),..one})}
  if self.parts.is_some()||self.parts_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.parts,&self.parts_close,body)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.handle,&self.handle_close,body)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.metadata,&self.metadata_close,body)}
  if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..one})}
  if let Some(lease)=self.snapshot_lease.as_ref(){return Ok(if lease.terminal_is_empty(){one}else{lease.retirement_demands()})}
  if self.completion.is_some()||self.completion_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.completion,&self.completion_close,body)}
  if self.credit.is_some()||self.credit_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.credit,&self.credit_close,body)}
  if self.chunks.is_some()||self.chunks_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.chunks,&self.chunks_close,body)}
  if self.operation.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),..one})}
  Ok(one)
 }
 fn close_material_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_job::InteractiveJob;
  let item=RetainedCloneProgress{copied_items:1,..Default::default()};
  if let Some(job)=self.producer.as_mut(){if job.terminal_is_empty(){self.producer.take();return Ok(Some(item))}job.begin_close();return job.close_step(cx).map(|step|Some(step.progress()))}
  if let Some(job)=self.body.as_mut(){if job.terminal_is_empty(){let bytes=size_of_val(&**job);self.body.take();return Ok(Some(RetainedCloneProgress{released_bytes:bytes,..item}))}job.begin_close();return job.close_step(cx).map(|step|Some(step.progress()))}
  if let Some(owner)=self.request_close.as_mut(){if owner.terminal_is_empty(){self.request_close.take();return Ok(Some(item))}return owner.close_step(grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if let Some(request)=self.request.take(){*self.request_close=Some(crate::media_contract::ArtifactMediaExportRequestRetirement::new(request));return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<crate::media_contract::ArtifactMediaExportRequestRetirement<A>>(),..item}))}
  if self.parts.is_some()||self.parts_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.parts,&mut self.parts_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.handle,&mut self.handle_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.metadata,&mut self.metadata_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if let Some(snapshot)=self.snapshot.as_ref(){if !self.snapshot_lease.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending media snapshot lost its paired supplier").into())}self.snapshot.take();return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..item}))}
  if let Some(lease)=self.snapshot_lease.as_mut(){if lease.terminal_is_empty(){self.snapshot_lease.take();return Ok(Some(item))}return lease.close_step(grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.completion.is_some()||self.completion_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.completion,&mut self.completion_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.credit.is_some()||self.credit_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.credit,&mut self.credit_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.chunks.is_some()||self.chunks_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.chunks,&mut self.chunks_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.operation.is_some(){self.operation.take();return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<semio_framework_job::Operation>(),..item}))}
  Ok(None)
 }
}

pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
    pub(crate) app: A,
    pub(crate) mounted_policy: crate::MountedOwnerPolicyV1,
    constructor_receipts: std::mem::MaybeUninit<[Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>; 24]>,
    constructor_recipient_admitted: bool,
    constructor_recipient_retired: bool,
    constructor_terminal_receipt: Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)>,
    constructor_document: OriginalCatalogConstructor<A::Snapshot, A::Mutation>,
    constructor_config: OriginalCatalogConstructor<A::Config, A::ConfigMutation>,
    constructor_draft: OriginalCatalogConstructor<A::Draft, A::DraftMutation>,
    constructor_interaction: OriginalCatalogConstructor<protocol::InteractionState, InteractionConfigMutation>,
    constructor_lane: usize,
    pub(crate) store: ArtifactStore<A::Snapshot, A::Mutation>,
    pub(crate) config_store: ConfigStore<A::Config, A::ConfigMutation>,
    /// 📝️ Volatile draft lane — never checkpoints; prune via `ArtifactCommand::PruneDrafts`.
    /// Moves to host `ArtifactSession` when CHANNEL_VERSION 5 exchange lands.
    pub(crate) draft_store: store::DraftStore<A::Draft, A::DraftMutation>,
    /// 👥️ Ephemeral SHARED lane — a last-writer-wins peer roster, not an event log. Holds
    /// this actor's own presence plus whatever peers last broadcast; never persisted, never
    /// checkpointed, never undoable.
    pub(crate) presence_store: store::PresenceStore<A::Presence, A::PresenceMutation>,
    /// 🎨️ This actor's hub-assigned palette index, adopted from the last
    /// `AppCommand::Presence` (contract-freeze §C7.6) — `None` for a folder-only session with no
    /// hub, or before the first presence push.
    pub(crate) own_color: Option<u8>,
    /// 👥️ Every OTHER peer's last-adopted color/surface/interaction, keyed by actor —
    /// the ARTIFACT+APP-scope roster `InteractionView::peers_selecting`/`peers_hovering` and the
    /// UI-tree stamping pass read from; upserted/pruned atomically with `presence_store`'s own
    /// peer map by `adopt_presence`.
    pub(crate) peer_presence: ManuallyDrop<std::sync::Arc<PeerPresenceRoot>>,
    peer_presence_generation: u64,
    peer_roster_admission_generation: u64,
    pub(crate) peer_roster_processed_generation: u64,
    peer_roster_outcome_generation: u64,
    peer_roster_reservations: [Option<(u64, u64)>; ARTIFACT_LIVE_OUTPUT_SLOTS],
    peer_roster_scope: semio_framework_job::CancelToken,
    pub(crate) peer_roster_publications: ArtifactFixedRegistry<PeerRosterPublication<A>>,
    peer_roster_outcomes: ArtifactFixedRegistry<PresenceRosterOutcome>,
    pub(crate) peer_presence_retirements: ArtifactFixedRegistry<PeerPresenceRootRetirement>,
    pub(crate) presence_peer_retirements: ArtifactFixedRegistry<store::os_store::presence::PresencePeersRetirement<A::Presence>>,
    /// 🫧️ Ephemeral LOCAL-ONLY lane — typed UI state that never leaves this client and
    /// never becomes document content. The typed home for what used to live in plugin
    /// `thread_local!`s.
    pub(crate) transient_store: store::TransientStore<A::Transient, A::TransientMutation>,
    pub(crate) window_config_store: WindowConfigOwnerRegistry,
    pub(crate) window_transient_store: WindowTransientOwnerRegistry,
    retired_window_transient_stores: ArtifactFixedRegistry<WindowTransientOwnerRegistry>,
    maintenance_window_retirement_cursor: usize,
    close_window_retirement_cursor: usize,
    pub(crate) artifact_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>,
    pub(crate) config_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Config, A::ConfigMutation>>>,
    pub(crate) draft_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Draft, A::DraftMutation>>>,
    pub(crate) presence_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Presence, A::PresenceMutation>>>,
    pub(crate) transient_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Transient, A::TransientMutation>>>,
    pub(crate) presence_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,
    pub(crate) transient_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Transient>>>,
    unsupported_publication_contracts: BTreeMap<String, &'static str>,
    /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
    last_emit_wire: Option<EmitWire>,
    pub(crate) action_ingress_sources: ArtifactFixedRegistry<crate::action_ingress::ActionIngressRecord>,
    pending_tool_lease_admission:ManuallyDrop<Option<OriginalToolLeaseAdmission>>,
    pending_media_import_permit:ManuallyDrop<Option<(FrameworkReservedCommitPermit,(semio_framework_job::OperationId,semio_framework_job::Generation,usize))>>,
    pub(crate) pending_action_ingress: ManuallyDrop<Option<crate::action_ingress::ActionIngressSource>>,
    pub(crate) pending_action_ingress_caller: std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub(crate) pending_action_ingress_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub(crate) pending_action_ingress_retirement: ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Option<crate::action_ingress::ActionIngressSource>>>>,
    typed_effect_outbox: ArtifactFixedQueue<Effect>,
    typed_event_outbox: ArtifactFixedQueue<AppEvent>,
    typed_ui_outbox: ArtifactFixedQueue<UiDirtyScope>,
    operation_progress_retired: bool,
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — per mounted typed operation (keyed by
    /// its operation id), the guest-emitted interaction verbs (`Effect::ReplayShellCommand`/
    /// `DispatchAction` with an `INTERACTION_ACTION_IDS` action, see [`is_inline_interaction_verb`])
    /// peeled out of its emit the INSTANT the ladder takes the completion
    /// (`publish_mounted_typed_operation_unit`'s install step) — before any effect can be popped
    /// into `typed_effect_outbox` — and parked until `mounted_typed_inline_interaction_verbs_are_next`
    /// folds them in-reactor. The host never sees one: a verb that reached the effect outbox on
    /// the install pump was re-dispatched by the shell as a second `onAction` (18
    /// `replayShellCommand dispatch interactionSelect` lines per battery on the fem2d lane),
    /// behind any pointer input issued meanwhile. Bounded by the live operation slots; an entry
    /// is consumed by the fold or dropped with its operation's exact removal.
    typed_inline_interaction_verbs: Vec<(u64, Vec<(String, Option<DslValue>)>)>,
    /// 🔀️ The LAST folded verb's leftover `InteractionView` (`AppliedInteractionVerb::leftover`),
    /// one slot, latest wins (a later verb's snapshot already includes the earlier ones — the
    /// same rule `InlineInteractionFold::leftover` follows). It rides the next `Ui` progress
    /// frame's `output` (`take_typed_operation_ui_progress`) — the typed ladder's twin of the
    /// `Invocation.output.interactionView` lane a host-dispatched verb answers on.
    typed_inline_interaction_leftover: Option<TypedOperationLeftover>,
    typed_completion_outbox: ArtifactFixedQueue<TypedOperationCompletionWitness>,
    /// 🧩️ See [`ComposedGestureResult`] — one entry per composed gesture this actor published.
    pub(crate) typed_composed_outbox: ArtifactFixedQueue<ComposedGestureResult>,
    /// 📨️ The authoritative outcome of the current app command, reset before dispatch and then
    /// filled from the store receipt or policy rejection before the runtime frames it.
    dispatch_report: protocol::DispatchReport,
    /// 🗂️ Keyed on `(store.generation(), log_generation, history_filter)` — any of the three
    /// changing invalidates the cached snapshot/`HistoryView` pair.
    pub(crate) cache: Option<AppProjectionCache<A>>,
    pub(crate) registry: AppActionRegistry,
    tool_job_controller_id: String,
    tool_jobs: semio_framework::ActionBus,
    pending_render_tree: std::mem::ManuallyDrop<Option<ComponentTree>>,
    pending_render_tree_identity: Option<OriginalRenderTreeIdentity>,
    pending_render_tree_ready: bool,
    pending_render_tree_close:std::mem::ManuallyDrop<Option<semio_framework_ui_contract::OriginalBuiltTreeRetirement>>,
    pending_configuration_completion:Option<crate::reserved_jobs::ConfigurationDecodeRetirement<A::ConfigMutation>>,
    pending_configuration_command:std::mem::ManuallyDrop<Option<ArtifactCommand<A::ConfigMutation>>>,
    pending_configuration_command_close:Option<ControlledRetirement<ArtifactCommand<A::ConfigMutation>>>,
    pending_configuration_permit:Option<FrameworkReservedCommitPermit>,
    pending_configuration_failure:std::mem::ManuallyDrop<Option<crate::reserved_jobs::OriginalConfigurationFailure>>,
    pending_configuration_identity:Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pending_configuration_retirement:Option<ControlledRetirement<crate::reserved_jobs::OriginalConfigurationFailure>>,
    pending_render_failure: std::mem::ManuallyDrop<Option<crate::OriginalRenderFailure>>,
    pending_render_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pending_render_close_failure:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    pending_render_compound:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalClosureFailure>>,
    pending_render_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<crate::OriginalRenderFailure>>>,
    agent_lane_preview:Option<crate::tool_jobs::AgentLanePreview<A>>,
    bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>,
    bounded_tool_contracts: Vec<ArtifactToolPublicContract>,
    framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    pub(crate) instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub(crate) tool_cancellations: crate::tool_cancellation::ToolCancellationIssuer,
    live_runtime_instance_id: Option<u32>,
    pub(crate) tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>,
    #[cfg(test)]
    pub(crate) test_tool_clock: Option<fn() -> Option<u64>>,
    pub(crate) typed_operation_reservations: [Option<u64>; ARTIFACT_LIVE_OUTPUT_SLOTS],
    typed_publication_cursor: usize,
    typed_result_cursor: usize,
    pub(crate) latest_wins_commands: ArtifactFixedRegistry<PendingLatestWinsCommand<A>>,
    pub(crate) latest_wins_order: ArtifactFixedQueue<u64>,
    latest_wins_keys: ToolLatestWinsRegistry,
    latest_wins_turn: bool,
    media_exports: ArtifactFixedRegistry<ActiveMediaExport>,
    media_export_preparation:ManuallyDrop<Option<MediaExportPreparation<A>>>,
    media_download_preparation:ManuallyDrop<Option<crate::media_export::MediaDownloadPreparation>>,
    pending_reserved: ArtifactFixedRegistry<PendingFrameworkReserved>,
    reserved_commits: std::collections::VecDeque<FrameworkReservedCommit>,
    reserved_commit_outcome: Option<Result<InvocationResult, Fault>>,
    media_closures: ArtifactFixedRegistry<ActiveMediaExport>,
    snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>,
    current_media_export: Option<u64>,
    media_submission_refusal: std::mem::ManuallyDrop<Option<crate::media_operation_refusal::MediaOperationRefusal>>,
    media_submission_refusal_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    media_submission_refusal_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<crate::media_operation_refusal::MediaOperationRefusal>>>,
    pub(crate) segmented_downloads: ArtifactFixedRegistry<ArtifactDownloadOutput>,
    pub(crate) segmented_closures: ArtifactFixedRegistry<crate::media_export::MediaDownloadRetirement>,
    pub(crate) close_started: bool,
    pub(crate) close_cancellation_cursor: usize,
    pending_document_window_admission:std::mem::ManuallyDrop<Option<DocumentWindowResetAdmission>>,
    close_typed_operation_cursor: usize,
    pub(crate) close_media_cursor: usize,
    pub(crate) close_media_cleanup_cursor: usize,
    close_segment_cursor: usize,
    close_segment_cleanup_cursor: usize,
    close_snapshot_cursor: usize,
    close_instance_operation_owner_drained: bool,
    close_child_root_cursor: usize,
    close_child_root_detached: bool,
    close_child_member_cursor: usize,
    close_child_member: ManuallyDrop<Option<ChildMemberRetirement<M>>>,
    close_peer_presence_cursor: usize,
    close_peer_presence_detached: bool,
    close_presence_peer_cursor: usize,
    close_peer_roster_cursor: usize,
    pub(crate) maintenance_stage: u8,
    pub(crate) maintenance_cancellation_cursor: usize,
    maintenance_tool_cursor: usize,
    maintenance_media_cursor: usize,
    maintenance_segment_cursor: usize,
    maintenance_snapshot_cursor: usize,
    maintenance_child_root_cursor: usize,
    maintenance_peer_presence_cursor: usize,
    maintenance_presence_peer_cursor: usize,
    maintenance_peer_roster_cursor: usize,
    envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>,
    maintenance_envelope_ingress_cursor: usize,
    close_envelope_ingress_cursor: usize,
    close_envelope_ingress_drained: bool,
    envelope_decode_jobs: ArtifactFixedRegistry<ActiveArtifactEnvelopeDecode<A::Snapshot, A::Mutation>>,
    maintenance_envelope_decode_cursor: usize,
    close_envelope_decode_cursor: usize,
    close_envelope_decode_stage: u8,
    close_envelope_decode_jobs_drained: bool,
    pub(crate) store_replacement_jobs: ArtifactFixedRegistry<ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>>,
    maintenance_store_replacement_cursor: usize,
    close_store_replacement_cursor: usize,
    close_store_replacement_jobs_drained: bool,
    /// 📥️ This instance's open file-import runs: every chunked pick is reassembled here, so an app's import
    /// action only ever decodes a whole file (`semio_framework::kernel::ImportStaging`).
    import_staging: semio_framework::kernel::ImportStaging,
    pub(crate) document_archive_loads: ArtifactFixedRegistry<ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>>,
    maintenance_document_archive_cursor: usize,
    close_document_archive_cursor: usize,
    close_document_archive_loads_drained: bool,
    pub(crate) envelope_field_decoders: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot, A::Mutation>>,
    pub(crate) envelope_field_decoder_retirements: ArtifactFixedRegistry<store::os_store::owned_schema_decode::ArtifactEnvelopeReturnedFieldDecoder<A::Snapshot, A::Mutation>>,
    maintenance_envelope_field_decoder_cursor: usize,
    close_envelope_field_decoder_cursor: usize,
    close_envelope_field_decoders_drained: bool,
    pub(crate) envelope_completed_records: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot, A::Mutation>>,
    pub(crate) envelope_completed_record_retirements: ArtifactFixedRegistry<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<A::Snapshot, A::Mutation>>>,
    maintenance_envelope_completed_record_cursor: usize,
    close_envelope_completed_record_cursor: usize,
    close_envelope_completed_records_drained: bool,
    document_snapshot_read_returns: SnapshotReadReturnPump,
    close_snapshot_reads_drained: bool,
    pub(crate) close_owned_stage: u8,
    close_owned_advance_ready: bool,
    close_document_disposer: ManuallyDrop<ArtifactDisposal<ArtifactStore<A::Snapshot, A::Mutation>>>,
    close_config_disposer: ManuallyDrop<ArtifactDisposal<ConfigStore<A::Config, A::ConfigMutation>>>,
    close_draft_disposer: ManuallyDrop<ArtifactDisposal<store::DraftStore<A::Draft, A::DraftMutation>>>,
    close_presence_disposer: ManuallyDrop<ArtifactDisposal<store::PresenceStore<A::Presence, A::PresenceMutation>>>,
    close_transient_disposer: ManuallyDrop<ArtifactDisposal<store::TransientStore<A::Transient, A::TransientMutation>>>,
    close_interaction_disposer: ManuallyDrop<ArtifactDisposal<ConfigStore<protocol::InteractionState, InteractionConfigMutation>>>,
    invocation_kind: Option<ActionKind>,
    /// 🧰️ The utility each window last showed in a dispatch's view, so a switch becomes a [`HostEvent::UtilityChanged`].
    host_event_utilities: HashMap<String, Option<String>>,
    /// 🪟️ The window roster the last dispatch's view carried: the windows an instance-wide [`HostEvent`] reaches.
    host_event_roster: Vec<semio_framework::ViewWindowInstance>,
    /// 📨️ The typed operations a [`HostEvent`] admitted: never history rows of their own.
    host_event_operations: HashSet<u64>,
    /// 🔁️ Set while a host event dispatches, so its own dispatch delivers none.
    delivering_host_event: bool,
    /// 🧾️ Append-only session command log — see `🔖️CommandLog`. Never persisted, never
    /// truncated: undo/redo/revert push entries, they never remove any.
    pub(crate) command_log: PagedCommandLog,
    pub(crate) command_prune_generation: u64,
    command_prune_processed: u64,
    pub(crate) pending_command_prune: Option<MountedCommandPrune>,
    next_command_seq: u64,
    /// 🗂️ Bumped by `push_log_entry`/`record_command` on every log mutation (a push OR a fold)
    /// — part of the cache key so a folded ×count bump alone (no store-generation change) still
    /// invalidates a stale render.
    pub(crate) log_generation: u64,
    /// 🧾️ Sequence ids whose host-projected rows changed since the last invocation response.
    pub(crate) history_dirty_sequences: HashSet<u64>,
    /// 🗂️ The history view the next refresh patches (design §20.14); it outlives a dropped projection cache.
    pub(crate) history_view: Option<HistoryViewCache>,
    /// 🧾️ How far the command log is proven to reference every edit ([`HistoryBackfillMark`]).
    history_backfill: Option<HistoryBackfillMark>,
    /// 🐚️ Shell rows (`noteShellCommand`) already undone — they stay in the append-only log.
    shell_undone: HashSet<u64>,
    /// 🐚️ Redo stack for those shell rows (chrome order: last undone is first redone after VCS redo empties).
    shell_redo: Vec<ShellHistoryReplay>,
    history_filter: HistoryCommandFilter,
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every owned child's LIVE store, keyed by
    /// `(slot, child_id)` — mirrors how `store`/`config_store`/`draft_store` above each hold one
    /// lane's live `ArtifactStore`, generalized to N children instead of one fixed lane. Each
    /// entry also carries the dialect captured at open/genesis time (`SpaceMember` itself has no
    /// object-safe dialect getter — see `dispatch_emit_group`'s own doc comment), so a
    /// `store::os_store::composition_coordinator::ChildDispatch`/`store::ArtifactRef` can be rebuilt for it without re-deriving one.
    pub(crate) children: ChildMemberRegistry<M>,
    pub(crate) private_child_groups: ArtifactFixedRegistry<Box<MountedPrivateChildGroup<M>>>,
    pub(crate) child_content_root: ManuallyDrop<ChildContentView>,
    pub(crate) child_content_generation: u64,
    pub(crate) child_content_retirements: ArtifactFixedRegistry<ChildContentRetirement>,
    child_member_retirements: ArtifactFixedRegistry<ChildMemberRetirement<M>>,
    child_member_retirement_generation: u64,
    child_member_retirement_cursor: usize,
    followed_parent_generation: u64,
    /// ⏳️ The last follow pass waited on child retirements — its own publications were not yet admissible, or the child
    /// the parent names was still retiring — so the instance stays runnable until a later pass followed the parent.
    child_follow_awaits_retirement: bool,
    /// 📌️ Checkout pins for children that were NOT open when a checkpoint cascade ran. Draining
    /// this on `open_child` is what keeps a lazily-adopted child from silently sitting at head
    /// while the rest of the composition sits at a pinned checkpoint — the alternative (dropping
    /// the pin) would make a checked-out composition quietly inconsistent, which is exactly the
    /// class of bug the cascade exists to prevent.
    pub(crate) pending_child_pins: Vec<vcs::CompositionPin>,
    /// 🧩️ STATEFUL by design (owns the incrementally-maintained `store::os_store::composition_coordinator::CompositionGraph`) — see
    /// `store::os_store::composition_coordinator::CompositionCoordinator`'s own doc comment. One coordinator per `VcsArtifactApp`
    /// instance (i.e. per document), exactly like `store`/`config_store`/`draft_store` are
    /// per-instance rather than global.
    pub(crate) composition: CompositionCoordinator,
    /// 🕹️ The framework-owned, PERSISTED-LOCAL half of the interaction mechanism: one domain's
    /// current selection + active mode/granularity, written ONLY by the six intercepted interaction
    /// verbs (`dispatch_interaction_action`) through `ApplyInLane{lane: HistoryLane::Interaction}` —
    /// single writer, so it never drifts from what the presence broadcast reports. Persists/reloads
    /// like `config_store`; the general "undo"/"redo" actions above only ever dispatch against
    /// `self.store` (the DOCUMENT store), never this one, so a pick is never undoable by construction
    /// — the `HistoryLane` tag on its own entries additionally lets a lane-aware caller walk PAST them
    /// (`ArtifactCommand::UndoInLane`/`RedoInLane`) without needing a second store at all. A real
    /// `store::ConfigStore<InteractionState, _>` instance (not folded into `config_store: ConfigStore
    /// <A::Config, A::ConfigMutation>`, which is per-app-config-typed): keeps this zero-touch for
    /// every app's own `Config`/`ConfigMutation` types, since ~17 crates never need to know this
    /// exists — see `InteractionConfigMutation`'s own doc comment for the orphan-rule reason its
    /// `Mutation` impl couldn't target `A::Config` directly either way.
    pub(crate) interaction_store: ConfigStore<protocol::InteractionState, InteractionConfigMutation>,
    local_interaction_query: Option<crate::local_interaction::live::LocalInteractionLiveQuery<A::Snapshot, A::Config>>,
    local_interaction_query_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    local_interaction_authority: crate::local_interaction::authority::LocalInteractionTopologyAuthority,
    snapshot_read_return_stage: u8,
    local_interaction_query_turn: bool,
    local_interaction_emitted_bytes: u128,
    local_interaction_retired_bytes: u128,
    config_snapshot_read_returns: SnapshotReadReturnPump,
    interaction_snapshot_read_returns: SnapshotReadReturnPump,
    /// 🐁️ The EPHEMERAL-local half — see `InteractionHoverState`'s own doc comment. Never persisted,
    /// never undoable, mirrored into the ephemeral-shared `PresenceInteraction` broadcast
    /// (`interaction_state`) exactly like `presence_store`'s own local half.
    pub(crate) interaction_hover: InteractionHoverState,
    /// 🕹️ Wave B9: the just-computed selection `dispatch_interaction_action` published on leftover,
    /// retained for the guest's own render. `protocol::validate_state` prunes every id an empty
    /// topology does not contain, so the store snapshot a first pick reads can be empty while this
    /// overlay still names the picked id — Inspection reads this when the store does not.
    interaction_leftover_selection: Option<protocol::InteractionState>,
    interaction_leftover_ids: Vec<String>,
    /// 🌳️ One `DomainTopology` per `HierarchyProvider::UiTree` domain, derived from the LAST
    /// rendered `UiNode::Tree` carrying that `interaction_domain` (see `render`'s post-processing
    /// pass) — `resolve_domain_topology`'s cache for domains the app itself never supplies a
    /// topology for. Empty (safely degrading to Flat-like single-target behavior) until the first
    /// render of that domain's tree.
    pub(crate) interaction_ui_topology: HashMap<String, protocol::DomainTopology>,
    /// 🗺️ `A::interaction_topology`'s whole answer for one document generation.
    ///
    /// The app builds EVERY declared domain in one pass off a fresh decode of the document, and one
    /// pick asked for it at least twice — once in [`Self::dispatch_interaction_action`]'s own verb arm
    /// and once more inside [`Self::revalidate_and_persist_interaction_state`]'s
    /// `build_full_interaction_topology`, with nothing between them that can move the document. On the
    /// 180-object puzzle3d flagship that is two full fixture decodes plus ~900 `TopologyNode`
    /// allocations per pick and is the reason `interactionSelect` cost 20× there
    /// (26/09/02/PUZZLE-3D-END-TO-END wave B44 §2.2). Keyed on `(store.generation(),
    /// config_store.generation())` — the pair every input of `A::interaction_topology` is derived
    /// from, `live_render_operation` included — so a mutation invalidates it in the same turn it lands.
    interaction_topology_memo: Option<((u64, u64), protocol::InteractionTopology)>,
    /// 🔀️ The single in-flight transaction this instance has prepared into, if any (contract
    /// §5.9: "one pending transaction per instance" — a second `TransactionPrepare` while this
    /// is `Some` is rejected `transaction.instance-busy`, and `dispatch_emit` refuses any
    /// further artifact-mutating command per §5.10).
    pending_transaction: Option<PendingTransaction<A::Mutation>>,
    /// 🔀️ Set by `dispatch_emit` instead of applying, when a gesture's mutations carry foreign
    /// steps — drained once by `plugin_exchange` (`take_pending_transaction_proposal`) to frame
    /// `AppFrame::TransactionProposal` (contract §5.1).
    pending_transaction_proposal: Option<TransactionProposalDraft>,
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): the render-plane presence outbox —
    /// `stamp_and_cache_interaction_ui` pushes one `PresenceUpdate` per `(surface, node_key)` whose
    /// own/peer selection or hover it derived off `protocol::InteractionState`/`peer_presence` this
    /// render; `plugin_runtime::plugin_render_surface` drains it within the same surface render. NEVER a document
    /// revision — this lane has no op log, no undo group, no command-log row, mirroring
    /// `presence_store`/`transient_store`'s own ephemeral treatment above.
    pub(crate) pending_presence: Vec<PresenceUpdate>,
    /// 🧹️ Node keys this body's LAST render marked, per body key. `derive_node_presence` skips an
    /// idle node entirely, and `PresenceHub::record_own` keeps own presence until it is explicitly
    /// overwritten — so without this ledger a node that loses the selection is never written back to
    /// idle and every renderer keeps painting it selected for the life of the session. Each render
    /// diffs the keys it marked against this set and pushes one cleared `PresenceUpdate` per key it
    /// dropped, which is the ONLY thing that retires a selection on the render plane.
    presence_marked_keys: BTreeMap<String, BTreeSet<String>>,
    /// ⏯️ The ephemeral local-only tool run ledger (contract §3.3): provisional document ops folded
    /// into an overlay the renderer reads, never touching `store` until the one-edit finalize.
    pub(crate) tool_runs: ToolRunLedger<A>,
    /// ⏪️ The ephemeral local-only history-edit session (design §7 of ticket
    /// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): drafts previewed and replayed beside the store, never touching it
    /// until the one finalize commit.
    pub(crate) time_travel: TimeTravelLedger<A>,
    /// 🎚️ Every window's open continuous-control press (design §13.1): provisional leaves overlaid on every render,
    /// published as ONE transaction on the release, zero trace on any host abort.
    pub(crate) tool_machines: ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    /// ✏️ Every `Supersede` transition of the store's event log, classified per author (history edit, its undo, its
    /// redo) — derived from the transitions, so the history rows and undo of history edits survive reload.
    supersedes: SupersedeLedger,
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🔍️ Read-only inspection of this instance's tool run ledger for app-level tests.
    #[cfg(feature = "artifact-app-testing")]
    pub fn tool_run_ledger(&self) -> &ToolRunLedger<A> {
        &self.tool_runs
    }

    /// 🪪️ A command's whole-document replacement names this store's identity, not whatever the app
    /// minted: see [`store::os_store::text_format::stamp_document_spr_identity`].
    fn stamp_load_document_identity(&self, effect: Effect) -> Result<Effect, Fault> {
        let Effect::LoadDocument { pack, spr } = effect else { return Ok(effect) };
        let envelope = self.store.envelope();
        let identity = ArtifactRef { artifact_id: envelope.id.clone(), dialect: A::DIALECT.into() };
        let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::stamp_document_spr_identity(&spr, &identity.artifact_id, A::DOCUMENT_SCHEMA, &identity.dialect, envelope.owner.as_ref()))
            .map_err(|error| plugin_sdk_fault(format!("load-document identity stamp failed: {error}")))?;
        Ok(Effect::LoadDocument { pack, spr })
    }

    /// 🚪️ EVERY effect leaving this instance passes the stamp, not just the mounted typed-operation
    /// ladder's. An app mints its replacement log with `store::os_store::text_format::empty_document_spr`, which carries no
    /// composition section at all, and the archive loader's parent hydration refuses such a log
    /// outright (`MemberOpenDiagnostic::Identity`, `💧️hydration/🦀️.rs` `Phase::Begin`) before any of
    /// the three replacement legs even runs. The stamp is therefore not an optimisation on one lane,
    /// it is the whole contract between `Effect::LoadDocument` and `load_document_archive` — and the
    /// plain dispatch lane and the refresh poll both used to skip it, which is why every
    /// `setActiveExample` whose verb is not a mounted retained tool died in the browser.
    fn stamp_load_document_effects(&self, effects: Vec<Effect>) -> Result<Vec<Effect>, Fault> {
        if !effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return Ok(effects);
        }
        effects.into_iter().map(|effect| self.stamp_load_document_identity(effect)).collect()
    }

    /// 🪪️ The refresh poll's twin — that lane has no error channel, so an undecodable log rides on
    /// unchanged and the archive loader reports its own identity refusal, rather than the effect
    /// disappearing without a word.
    fn stamp_polled_load_document_effects(&self, effects: Vec<Effect>) -> Vec<Effect> {
        if !effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return effects;
        }
        let envelope = self.store.envelope();
        let dialect: ArtifactDialect = A::DIALECT.into();
        effects
            .into_iter()
            .map(|effect| match effect {
                Effect::LoadDocument { pack, spr } => {
                    let stamped = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::stamp_document_spr_identity(&spr, &envelope.id, A::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref())).ok();
                    Effect::LoadDocument { pack, spr: stamped.unwrap_or(spr) }
                }
                other => other,
            })
            .collect()
    }

    /// 🪆️ Every live owned child as its archive member (persisted envelope, identity, owner), sorted by owner and identity
    /// with dense ordinals — the member roster of the document archive, which is also the one `artifact:out` carrier.
    async fn archive_member_entries(&self) -> Result<Vec<protocol::OwnedDocumentMemberPackEntry>, Fault> {
        let mut members = Vec::new();
        members.try_reserve_exact(self.children.len()).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member roster could not reserve its exact live extent"))?;
        for entry in self.children.entries() {
            let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
            members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: protocol::DocumentArchiveArtifactRef {
                    artifact_id: entry.reference.artifact_id.clone(),
                    artifact_kind: entry.reference.dialect.artifact_kind.clone(),
                    standard: entry.reference.dialect.standard.clone(),
                    subset: entry.reference.dialect.subset.clone(),
                },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef {
                        artifact_id: entry.owner.parent.artifact_id.clone(),
                        artifact_kind: entry.owner.parent.dialect.artifact_kind.clone(),
                        standard: entry.owner.parent.dialect.standard.clone(),
                        subset: entry.owner.parent.dialect.subset.clone(),
                    },
                    slot: entry.owner.slot.clone(),
                    child_id: entry.owner.child_id.clone(),
                },
                envelope_pack,
            });
        }
        members.sort_by(|left, right| {
            (&left.owner.parent.artifact_id, &left.owner.slot, &left.owner.child_id, &left.reference.artifact_kind, &left.reference.standard, &left.reference.subset).cmp(&(
                &right.owner.parent.artifact_id,
                &right.owner.slot,
                &right.owner.child_id,
                &right.reference.artifact_kind,
                &right.reference.standard,
                &right.reference.subset,
            ))
        });
        for (ordinal, entry) in members.iter_mut().enumerate() {
            entry.ordinal = u32::try_from(ordinal).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member ordinal exceeds u32"))?;
        }
        Ok(members)
    }
}

/// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.
/// `result_from_last_edit`'s own `ArtifactHandle(meta.instance_id as u128)` only identifies the
/// ONE wasm-hosted document this plugin instance runs (the parent) — a child pulled in through
/// `dispatch_emit_group` is a different document with its own string `artifact_id` and no
/// instance id of its own, so `KernelMutation.document` needs a handle derived from THAT id
/// instead, so history correctly attributes a child's edit to the child, not the parent. FNV-1a
/// (128-bit, two interleaved 64-bit lanes) rather than `std::collections::hash_map::DefaultHasher`
/// — the latter's hash is only guaranteed stable for one process/compiler invocation, not the
/// byte-identical value the SAME child id must always map to.
pub(crate) async fn artifact_handle_of(artifact_id: &str) -> ArtifactHandle {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut lane0 = OFFSET;
    let mut lane1 = OFFSET ^ 0x9e37_79b9_7f4a_7c15;
    for byte in artifact_id.as_bytes() {
        lane0 = (lane0 ^ *byte as u64).wrapping_mul(PRIME);
        lane1 = (lane1 ^ (*byte as u64).rotate_left(7)).wrapping_mul(PRIME);
    }
    ArtifactHandle(((lane0 as u128) << 64) | lane1 as u128)
}

/// 🪪️ The persisted schema a derivable child of `dialect` is minted under — read from the member
/// roster's own open declarations, so an app deriving a child of a dialect its `M` never declares
/// is a fault rather than a member nothing could ever open.
pub(crate) fn genesis_member_schema<M: MemberFactory>(dialect: &ArtifactDialect) -> Result<&'static str, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    M::OPEN_DECLARATIONS
        .iter()
        .find(|declaration| declaration.kind == dialect.artifact_kind && declaration.standard == dialect.standard && declaration.subset == dialect.subset)
        .map(|declaration| declaration.schema)
        .ok_or_else(|| plugin_sdk_fault(format!("derived child dialect '{}' is not declared by this app's member roster", dialect.to_coordinate())))
}

/// 📚️ Turns a registered example body into the app snapshot. A `{` body is the JSON a deferred
/// producer emits; anything else is the subset's own DSL.
fn catalogue_snapshot_from_document<S>(body: &str) -> Result<S, Fault>
where
    S: pack::snapshot::ArtifactDsl + semio_framework_value::FromValue,
{
    if body.trim_start().starts_with('{') {
        semio_framework_pack_json::from_json_str(body, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.example.unreadable"), error.to_string()))
    } else {
        <S as pack::snapshot::ArtifactDsl>::parse_dsl(body).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.example.unreadable"), error.to_string()))
    }
}

fn framework_reserved_action_kind(action: &str) -> Option<ActionKind> {
    if !is_framework_reserved_action_id(action) {
        return None;
    }
    if CLIPBOARD_ACTION_IDS.contains(&action) {
        return Some(ActionKind::Clipboard);
    }
    if INTERACTION_ACTION_IDS.contains(&action) || action == semio_framework::HOST_EVENT_ACTION_ID {
        return Some(ActionKind::Interaction);
    }
    if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID || action == CANCEL_TYPED_OPERATION_ACTION_ID {
        return Some(ActionKind::View);
    }
    if action == NOTE_SHELL_COMMAND_ACTION_ID || matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
        return Some(ActionKind::Shell);
    }
    Some(ActionKind::History)
}

/// ⏱️ M2 (ticket 26/08/17 `design-unified.md`): default TTL for every `PresenceUpdate`
/// `stamp_and_cache_interaction_ui` derives — a peer mark not refreshed within this window ages
/// out on the reactor's `PresenceHub` with no goodbye message (matches the design's decided
/// default; own presence never expires by this or any other TTL — see `ui_runtime::PresenceHub`'s
/// own doc).
const PRESENCE_TTL_MS: u32 = 4_000;

/// 🧊️ The framework-reserved verbs a history-edit session freezes: the history-lane verbs (they move the applied
/// stack or the head and would invalidate the session base silently) and the clipboard verbs that edit the document.
fn time_travel_freezes_reserved_action(action: &str) -> bool {
    HISTORY_ACTION_IDS.contains(&action) || matches!(action, REVERT_TO_COMMAND_ACTION_ID | "cut" | "paste")
}

/// 🧊️ The `timeTravel.frozen` refusal of `verb` while a history edit is open on this instance.
fn time_travel_frozen_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE), format!("verb {verb:?} would change the document while a history edit is open on this instance"))
}

/// 🔒️ Frozen fault (contract §2.3): exact code `viewer.read-only`, `FaultOrigin::Framework` — a
/// mutating verb reached a `Viewer`-role `VcsArtifactApp` instance, either through
/// `dispatch_action`/`dispatch_import_media`'s string-verb guard or `dispatch_emit`'s non-empty
/// `artifact_mutations` backstop (the latter is unreachable through `ViewerApp`'s structurally
/// closed `ViewEmit`, contract §2.2 — this exists so a hand-written runtime `ArtifactApp` impl
/// that ignores `ViewEmit` is safe too, not "redundant").
// 🚫️async: E1 pure error constructor consumed pervasively by sync closures — see R9.
fn viewer_read_only_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new("viewer.read-only"), format!("'{verb}' is not permitted on a viewer instance (contract §2.3)"))
}

/// 🏷️ A noted shell command's history label in every locale: a full `LocalizedLabel`, an `{en, de}` pair, or plain
/// text (locale-invariant data, e.g. a file name); the optional `detail` is appended in every locale. Without a
/// label the command id stands, as data.
fn note_shell_command_label(label: Option<&DslValue>, command_id: &str, detail: Option<&DslValue>) -> LocalizedLabel {
    let base = match label {
        Some(DslValue::String(text)) => LocalizedLabel::data(text.clone()),
        Some(value @ DslValue::Object(_)) => match (value.get("en").and_then(DslValue::as_str), value.get("de").and_then(DslValue::as_str)) {
            (Some(en), Some(de)) => LocalizedLabel::native(en, de),
            _ => <LocalizedLabel as semio_framework_value::FromValue>::from_value(value.clone()).unwrap_or_else(|_| LocalizedLabel::data(command_id)),
        },
        _ => LocalizedLabel::data(command_id),
    };
    match detail.and_then(DslValue::as_str) {
        Some(detail) => LocalizedLabel::from_fn(|terminology, locale| format!("{} - {detail}", base.resolve(terminology, locale))),
        None => base,
    }
}

/// 🛠️ A committed tool transaction is one plain edit per touched member (the parent and every owned child it carries,
/// all stamped with the same ref — design §12); a streamed one is one open edit of the app's own document that its ticks
/// grow (design §15). Refused instead of silently losing a stamp or a trace: a stream or abort naming no transaction, a
/// stream or abort across owned children, a stream carrying operations with foreign steps, and an abort carrying
/// operations.
fn tool_transaction_shape_fault(verb: &str, transaction: Option<&protocol::TransactionRef>, phase: TransactionPhase, children: bool, carried: bool, foreign: bool) -> Option<Fault> {
    let refuse = |detail: String| Some(Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.shape"), detail));
    let streamed = matches!(phase, TransactionPhase::Stream | TransactionPhase::Abort);
    match transaction {
        None if streamed => refuse(format!("verb {verb:?} streamed or aborted no tool transaction")),
        Some(transaction) if streamed && children => refuse(format!("verb {verb:?} streamed or aborted tool transaction {:?} across owned children; a streamed transaction grows the app's own document", transaction.id)),
        Some(transaction) if phase == TransactionPhase::Stream && foreign => refuse(format!("verb {verb:?} streamed operations with foreign steps into tool transaction {:?}; a streamed transaction grows the app's own document", transaction.id)),
        Some(transaction) if phase == TransactionPhase::Abort && carried => refuse(format!("verb {verb:?} aborted tool transaction {:?} with mutations; an abort carries none", transaction.id)),
        _ => None,
    }
}

/// 🎞️ The document-store commands an emission's artifact lane dispatches (design §15): a streamed tick appends to its
/// transaction's open edit (opening it), a commit closes the open edit of its transaction (appending its last operations
/// first) or, with none open, records its operations as one edit carrying the ref, an abort reverts the open edit, and an
/// emission outside a transaction applies as one plain edit. An empty list publishes nothing.
fn artifact_lane_commands<M>(mutations: Vec<M>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::os_store::store_schema::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {
    let open = open.zip(transaction.as_ref()).is_some_and(|(open, transaction)| open.transaction.id == transaction.id);
    let append = |mutations: Vec<M>, transaction: protocol::TransactionRef| (!mutations.is_empty()).then(|| ArtifactCommand::AppendTransaction { mutations, transaction });
    match (transaction, phase) {
        (Some(transaction), TransactionPhase::Stream) => append(mutations, transaction).into_iter().collect(),
        (Some(transaction), TransactionPhase::Commit) if open => {
            let transaction_id = transaction.id.clone();
            append(mutations, transaction).into_iter().chain([ArtifactCommand::CommitTransaction { transaction_id }]).collect()
        }
        (Some(transaction), TransactionPhase::Abort) => open.then(|| ArtifactCommand::AbortTransaction { transaction_id: transaction.id }).into_iter().collect(),
        (_, _) if mutations.is_empty() => Vec::new(),
        (transaction, _) => vec![ArtifactCommand::Apply { mutations, transaction }],
    }
}

/// 🧯️ A batched store publication's refusal: the history, tool-transaction and size refusals keep their own fault codes
/// (`history.full`, `toolTransaction.open`, `toolTransaction.unknown`, `mutation.too-large` — the shells show them as notices);
/// every other one is the SDK's publication fault.
fn store_publication_fault(error: vcs::VcsError) -> Fault {
    match error {
        vcs::VcsError::TransactionOpen { .. } | vcs::VcsError::UnknownTransaction(_) | vcs::VcsError::HistoryFull { .. } | vcs::VcsError::TooLarge { .. } => error.into_fault(),
        error => plugin_sdk_fault(error.to_string()),
    }
}

pub(crate) fn typed_operation_document_is_fresh(operation: &semio_framework_job::Operation, canonical_revision: [u8; 32], live_revision: [u8; 32], live_generation: u64) -> bool {
    let live_short = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision[..8].try_into().expect("revision lane width")));
    canonical_revision == live_revision && matches!(semio_framework_job::validate_commit(operation, live_short, semio_framework_job::Generation(live_generation)), semio_framework_job::CommitValidation::Accepted)
}

include!("../🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs");
include!("../🎮️command/🎟️lease/🦀️.rs");

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    #[cfg(test)]
    pub(crate) fn test_registered_tool_keys(&self) -> BTreeSet<(String, String)> {
        self.tool_jobs.keys().into_iter().map(|key| (key.controller_id, key.tool_id)).collect()
    }

    #[cfg(test)]
    pub(crate) fn test_framework_tool_registrations(&self) -> BTreeMap<String, (semio_framework::ToolFactoryKey, String, std::any::TypeId, &'static str, semio_framework::ToolExecutionContract)> {
        self.framework_tool_registrations
            .iter()
            .map(|(tool_id, registration)| (tool_id.clone(), (registration.key.clone(), registration.schema_id.clone(), registration.factory_type_id, registration.factory_type_name, registration.contract)))
            .collect()
    }

    pub fn tool_cancellation_handle(&self) -> &ToolCancellationHandle {
        &self.tool_cancellations
    }

    /// 🎟️ Seals one genuine publication slot and keyed lease before a reserved producer receives source owners.
    fn admit_reserved_emit_producer(&mut self,verb:&str,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FrameworkReservedCommitPermit>,semio_framework_job::OriginalTurnFailure>{
        let proof=self.qualified_tool_proof(verb)?;
        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("reserved producer lacks its exact registered publication proof").into())}
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,true,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🧷️ Clears only the original reserved publication identity without changing a foreign slot.
    fn release_reserved_emit_slot(&mut self, operation: u64) {
        let reservation = &mut self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if *reservation == Some(operation) { *reservation = None; }
    }

    /// 🏁️ Returns a reserved producer's original slot and keyed cancellation authority after ordinary completion.
    fn finish_reserved_emit_producer(&mut self, permit: FrameworkReservedCommitPermit) {
        self.release_reserved_emit_slot(permit.operation.operation.0);
        permit.finish();
    }

    /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
    #[cfg(test)]
    pub(crate) fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta) -> bool {
        let before = self.typed_operation_reservations;
        for slot in 0..occupied { self.typed_operation_reservations[slot] = Some((slot + 128) as u64); }
        let original = self.typed_operation_reservations;
        let admitted = match self.admit_reserved_emit_producer("cut", meta) {
            Ok(permit) => {
                let id = permit.operation.operation.0;
                let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
                assert_eq!(slot, occupied);
                assert_eq!(self.typed_operation_reservations[slot], Some(id));
                assert!(permit.lease.keyed);
                assert_eq!(permit.lease.key.operation_id, permit.operation.operation);
                assert_eq!(&self.typed_operation_reservations[..occupied], &original[..occupied]);
                self.release_reserved_emit_slot(id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64);
                assert_eq!(self.typed_operation_reservations[slot], Some(id));
                self.finish_reserved_emit_producer(permit);
                assert_eq!(self.typed_operation_reservations, original);
                true
            }
            Err(_) => { assert_eq!(self.typed_operation_reservations, original); false }
        };
        self.typed_operation_reservations = before;
        admitted
    }

    /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_cancelled_worker_handoff(&mut self, meta: &ActionMeta) -> bool {
        let permit = self.admit_reserved_emit_producer("cut", meta).expect("original cut producer admission");
        let id = permit.operation.operation;
        permit.lease.cancel();
        let job = ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(), 0));
        match self.run_framework_reserved_job("cut", &[], 0, job, meta, None, Some(permit)).await {
            Ok(permit) => { assert_eq!(permit.operation.operation, id); assert_eq!(self.typed_operation_reservations[id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(id.0)); self.finish_reserved_emit_producer(permit); true }
            Err(_) => false,
        }
    }

    /// 🧫️ Observes busy original completion retention without exposing producer authority.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_busy_completion_owner(&mut self, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
        let permit=self.admit_reserved_emit_producer("cut",meta).expect("fixture cut admission");
        let id=permit.operation.operation.0;
        let completion=ArtifactToolCompletion::<A>::new();
        let mut emit=Emit::default();emit.artifact_mutations=Vec::with_capacity(3);
        completion.complete(Ok(emit),EphemeralEmit::default()).expect("fixture original completion");
        let original=completion.inner.clone();
        let guard=original.lock().expect("fixture original lock");
        let result=self.commit_framework_clipboard_completion("cut",Some(completion),meta,permit, cx).await;
        let retained=result.is_ok()&&self.tool_operations.get(id).is_some_and(|owner|owner.completion.as_ref().is_some_and(|cell|cell.inner.same_cell(&original))&&owner.terminal_fault.is_some()&&owner.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()));
        drop(guard);
        let mut original=semio_framework_value::retirement::controlled::ControlledRetirement::new(ArtifactToolCompletion::<A>{inner:original}).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original completion observation alias retirement was refused");});
        for _ in 0..100000 {
            if original.terminal_is_empty(){break;}
            let copy=original.next_copy_byte_demand().expect("original completion alias copy quote").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:original.next_capacity_byte_demand(copy).expect("original completion alias capacity quote"),maximum_release_bytes:original.next_release_byte_demand().expect("original completion alias release quote"),maximum_depth:original.next_depth_demand().expect("original completion alias depth quote")};
            assert!(original.step(grant).expect("funded original completion alias closure").progress().fits(grant));
        }
        assert!(original.terminal_is_empty(),"original completion observation alias remains in custody");
        if !retained {self.release_reserved_emit_slot(id);}
        retained
    }

    /// 🛡️ Keeps producer admission failures tied to the original reserved identity.
    async fn run_framework_reserved_job<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self, verb: &str, raw: &[u8], decoded_items: usize, job: J, meta: &ActionMeta,
        supplied_admission: Option<semio_framework::ToolWireAdmission>,
        supplied_authority: FrameworkReservedCommitPermit,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<FrameworkReservedCommitPermit, (semio_framework_job::OriginalTurnFailure,FrameworkReservedCommitPermit)> {
        let mut permit=supplied_authority;
        match self.run_framework_reserved_job_inner(verb, raw, decoded_items, job, meta, supplied_admission, &permit,cx).await {
            Ok(()) => Ok(permit),
            Err(fault) => { permit.lease.cancel(); Err((fault,permit)) },
        }
    }

    async fn run_framework_reserved_job_inner<J: semio_framework_job::InteractiveJob + Send + 'static>(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:J,meta:&ActionMeta,supplied_admission:Option<semio_framework::ToolWireAdmission>,supplied_authority:&FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if supplied_authority.lease.keyed && (self.typed_operation_reservations[supplied_authority.operation.operation.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS]!=Some(supplied_authority.operation.operation.0)||supplied_authority.lease.key.operation_id!=supplied_authority.operation.operation){return Err(plugin_sdk_fault("reserved producer lost its original keyed slot admission").into());}
        self.drive_framework_reserved_worker(verb,raw,decoded_items,job,meta,supplied_admission,supplied_authority.operation,supplied_authority.lease.cancel_token(),false,cx).await
    }

    async fn drive_framework_reserved_worker<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self,
        verb: &str,
        raw: &[u8],
        decoded_items: usize,
        job: J,
        _meta: &ActionMeta,
        supplied_admission: Option<semio_framework::ToolWireAdmission>,
        operation: semio_framework_job::Operation,
        cancel: &semio_framework_job::CancelToken,
        mounted_owner: bool,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let proof = self.qualified_tool_proof(verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory"))).into());
        }
        let key = proof.key();
        let admission = match supplied_admission {
            Some(admission) => admission,
            None => self.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?,
        };
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission"))).into());
        }
        let generation = operation.generation;
        if mounted_owner && !self.tool_operations.get(operation.operation.0).is_some_and(|owner| owner.operation.operation == operation.operation && owner.operation.base_revision == operation.base_revision && owner.operation.generation == operation.generation) { return Err(plugin_sdk_fault("reserved producer lost its original mounted operation authority").into()); }
        let operation_id = operation.operation;
        let dispatch = self
            .tool_jobs
            .dispatch(semio_framework::ToolOperationSpec::new(key.controller_id, key.tool_id, proof.schema_id(), job, operation))
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;

        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores));
        let step_grant = self.mounted_policy.preparation;
        let close_grant = self.mounted_policy.close;
        let source=(cx.operation(),cx.generation());
        let mut pending=semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,source);
        let config=semio_framework_job::BatchDriveConfig{retained:step_grant,site:"plugin_framework_reserved_route",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:proof.contract().max_work_units_per_step,step_budget_us:semio_framework_job::INTERACTIVE_LANE_WALL_US};
        let mut session=loop{plugin_job_yield_once().await;if let Some(session)=pending.admit_pending(config,cx)?{break session}};
        drop(pending);
        let mut checkpoint_progress = 0u64;
        let mut step_overruns = semio_framework_trace::StepOverrunLedger::new();
        loop {
            plugin_job_yield_once().await;
            let poll = match session.pump_one(&pool, semio_framework_async::Lane::Interactive, step_grant) {
                Ok(poll) => poll,
                Err(fault) if reserved_pump_fault_is_transient(&fault) => {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(now_ms) = semio_framework_job::default_now_ms() {
                        pool.pump(now_ms);
                    }
                    continue;
                }
                Err(fault) => {
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.worker-pump"), format!("framework route '{verb}' lost its mounted worker transition: {}", reserved_pump_fault_detail(&fault)))).into());
                }
            };
            if !matches!(poll, semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) {
                #[cfg(target_arch = "wasm32")]
                if let Some(now_ms) = semio_framework_job::default_now_ms() {
                    pool.pump(now_ms);
                }
                continue;
            }
            let Some((issued, receipt)) = session.take_checked_out_retained_step_receipt() else { return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.outcome-missing"), "framework reserved mounted worker checkout lost its exact outcome")).into()) };
            if !receipt.fits(issued) {
                cancel.cancel_now();
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.step-receipt"), format!("framework route '{verb}' reported a step receipt beyond its issued grant"))).into());
            }
            let mut protocol_fault = None;
            let (terminal, terminal_kind) = {
                let Some(view) = session.checked_out_outcome().map_err(|error| plugin_sdk_fault(error.into_message()))? else {
                    session.resume().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.resume"), "framework reserved paid step lost its exact resume authority"))?;
                    continue;
                };
                let kind = match &view {
                    semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::PreviewReady { .. } => 0,
                    semio_framework_job::JobOutcomeView::CheckpointReady { state, applied_progress, .. } => {
                        if *applied_progress < checkpoint_progress || state.len() > proof.contract().max_output_bytes {
                            protocol_fault = Some("interactive-job.checkpoint");
                        }
                        checkpoint_progress = *applied_progress;
                        0
                    }
                    semio_framework_job::JobOutcomeView::Complete { output, .. } => {
                        if output.map_or(!raw.is_empty(), |output| output.len() > proof.contract().max_output_bytes || !retained_payload_eq_slice(output, raw)) {
                            protocol_fault = Some("interactive-job.output-envelope");
                        }
                        1
                    }
                    semio_framework_job::JobOutcomeView::Cancelled { .. } => 2,
                    semio_framework_job::JobOutcomeView::Fault { .. } => 3,
                };
                (view.is_terminal(), kind)
            };
            let overrun = session.callback_verdict().copied().is_some_and(|verdict| step_overruns.admit(&verdict).is_terminal());
            if !terminal {
                let mut acknowledged = false;
                for _ in 0..RESERVED_OUTCOME_ACKNOWLEDGE_TURNS {
                    plugin_job_yield_once().await;
                    if session.checked_out_outcome_descriptor().is_none() {
                        acknowledged = true;
                        break;
                    }
                    let step = session.acknowledge_checked_out_outcome(close_grant);
                    if step.progress() == RetainedCloneProgress::default() {
                        break;
                    }
                }
                if !acknowledged {
                    cancel.cancel_now();
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.outcome-acknowledge"), format!("framework route '{verb}' could not acknowledge its retained outcome"))).into());
                }
                if let Some(code) = protocol_fault {
                    cancel.cancel_now();
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' returned an invalid retained checkpoint"))).into());
                }
                if overrun {
                    cancel.cancel_now();
                }
                session.resume().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.resume"), "framework reserved outcome lost its exact resume authority"))?;
                continue;
            }
            session.begin_close();
            loop {
                plugin_job_yield_once().await;
                let grant=close_grant;
                if matches!(session.has_original_cancel_alias_witness(&cancel),Ok(true)){
                    match session.return_original_cancel_alias_step(&cancel,grant){
                        Ok(_)|Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>continue,
                        Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(error.into()),
                    }
                }
                let demand=session.retirement_demands(grant.maximum_copy_bytes).map_err(|error|match error{semio_framework_job::WorkerJobDemandError::Refused(error)=>semio_framework_job::OriginalTurnFailure::from(error),semio_framework_job::WorkerJobDemandError::Contention(_)=>semio_framework_job::OriginalTurnFailure::from(plugin_sdk_fault("reserved original session close remains contended"))})?;
                if !plugin_grant_funds(demand,grant){continue}
                match cx.with_retained_loan(grant,|same|session.close_step(same))?{
                    semio_framework_job::WorkerJobCloseStep::Pending{..}|semio_framework_job::WorkerJobCloseStep::Blocked=>{},
                    semio_framework_job::WorkerJobCloseStep::Complete{..}if session.terminal_is_empty()=>break,
                    semio_framework_job::WorkerJobCloseStep::Complete{..}=>return Err(plugin_sdk_fault("reserved original session closed without its terminal witness").into()),
                }
            }
            if overrun {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.step-overrun"),
                    format!("framework route '{verb}' overran the 8 ms step ceiling for {} consecutive steps, worst {}us", step_overruns.consecutive_overruns(), step_overruns.worst_elapsed_us()),
                )).into());
            }
            if let Some(code) = protocol_fault {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' did not preserve its admitted envelope"))).into());
            }
            match terminal_kind {
                1 => break,
                2 => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{verb}' was cancelled"))).into()),
                3 => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.fault"), format!("framework route '{verb}' returned a retained fault"))).into()),
                _ => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.terminal-kind"), "framework reserved session closed without a terminal outcome")).into()),
            }
        }
        if cancel.is_cancelled().await {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled-or-output"), format!("framework route '{verb}' was cancelled before commit"))).into());
        }
        let live_revision_bytes = self.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(self.store.generation());
        if !matches!(semio_framework_job::validate_commit(&operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' completed against a stale revision or generation"))).into());
        }
        Ok(())
    }

    async fn validate_framework_reserved_commit(&self, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {
        if self.time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(verb) {
            return Err(time_travel_frozen_fault(verb));
        }
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{verb}' was cancelled before commit")));
        }
        let live_revision_bytes = self.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(self.store.generation());
        if !matches!(semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' reached commit with a stale revision or generation")));
        }
        Ok(())
    }

    fn qualified_tool_proof(&self, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = self.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("typed command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if self.bounded_tool_proofs.contains_key(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{verb}' has only a generic proof and no exact app-owned retained decoder/reducer factory")));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("typed command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    fn qualified_host_configuration_tool_proof(&self, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = self.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("host configuration command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if let Some(proof) = self.bounded_tool_proofs.get(verb) {
            return Ok(QualifiedToolProof::Bounded(proof.clone()));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("host configuration command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    /// 🚦️ The UI-safety backstop, read from the app's own manifest declaration: a verb that is not
    /// `Migrated` is refused by NAME here, before any factory, proof or wire admission is resolved.
    /// Resolving the proof first answered `interactive-job.missing-factory` for a verb whose real
    /// refusal is `interactive-job.not-ui-safe` — a non-migrated verb has no factory precisely
    /// BECAUSE it is not migrated, so the proof lookup reported the consequence, not the cause.
    fn require_ui_safe_declaration(&self, verb: &str) -> Result<(), Fault> {
        let declaration = match self.registry.get(verb) {
            Some(definition) => definition.semantics.execution.interactive_job,
            None => self
                .registry
                .get_command(verb)
                .map(|definition| definition.semantics.execution.interactive_job)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("typed command '{verb}' has no exact manifest declaration")))?,
        };
        validate_ui_dispatch_classification("owner", verb, declaration)
    }

    async fn admit_command_wire_with_proof(&self, proof: QualifiedToolProof, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) =
            self.tool_jobs.begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), payload.len()).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        std::io::Write::write_all(&mut writer, payload).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let raw_wire = writer.finish()?;
        if decoded_items > proof.contract().max_decoded_items {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), format!("decoded command contains {decoded_items} items; maximum is {}", proof.contract().max_decoded_items)));
        }
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items, raw_wire })
    }

    pub(crate) async fn admit_command_wire(&self, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_tool_proof(verb)?;
        self.admit_command_wire_with_proof(proof, verb, payload, decoded_items).await
    }

    async fn admit_command_json_with_proof(&self, proof: QualifiedToolProof, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) = self
            .tool_jobs
            .begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), proof.contract().max_raw_wire_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        let wire_args = args.cloned();
        let encoded = semio_framework_pack_json::to_json_string(&(verb, wire_args));
        std::io::Write::write_all(&mut writer, encoded.as_bytes()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-wire-limit"), "typed command raw JSON exceeds its registered retained-page admission"))?;
        let raw_wire = writer.finish_admitted_prefix()?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items: 0, raw_wire })
    }

    async fn admit_host_configuration_json(&self, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_host_configuration_tool_proof(verb)?;
        self.admit_command_json_with_proof(proof, verb, args).await
    }

    /// 🧬️ Constructs a store-only wrapper with an empty registry. Every action and typed
    /// command fails closed before decoding until a registry-backed wrapper is constructed.
    pub async fn new(app: A, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry(app, AppActionRegistry::default(), actor, mounted_policy, identity).await
    }

    /// 🧬️ Constructs a wrapper carrying the app's {@link AppActionRegistry} so `handle_action`
    /// enforces default materialization, required-arg validation, and kind discipline.

    /// 🧱 Runs `initial_snapshot` on its own frame so fixture sync cannot nest inside envelope construction.
    #[inline(never)]
    async fn isolate_artifact_app_initial_snapshot() -> A::Snapshot {
        A::initial_snapshot().await
    }

    pub async fn with_registry(app: A, registry: AppActionRegistry, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry_on_bus(app, registry, semio_framework::ActionBus::new(), actor, mounted_policy, identity).await
    }

    /// 🫂️ Activates an app controller against Platform's injected shared production registry.
    pub async fn with_registry_on_bus(app: A, registry: AppActionRegistry, tool_jobs: semio_framework::ActionBus, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        let mounted_policy = mounted_policy.validate().expect("explicit app embedding policy");
        let app_id = app.instance_id().await.to_string();
        PluginRuntimeRegistry::publish_framework_schema_documents().expect("the OS's own schema documents never conflict");
        for documents in <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let snapshot = Self::isolate_artifact_app_initial_snapshot().await;
        let mut envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &app_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.dialect = Some(A::DIALECT.into());
        let config_id = format!("{}-config", app_id);
        let config_envelope = create_config_envelope::<A::Config, A::ConfigMutation>(A::config_schema().await, &config_id, A::initial_config().await, None);
        let draft_id = format!("{}-draft", app_id);
        let draft_envelope = create_document_envelope::<A::Draft, A::DraftMutation>("draft.empty", &draft_id, A::initial_draft().await, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let interaction_id = format!("{}-interaction", app_id);
        let interaction_envelope = create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", &interaction_id, protocol::InteractionState::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor.clone()).await.expect("failed to create document store");
        store.install_operation_wire(store::ArtifactOperationWire::semantic::<A::Snapshot>().with_batch_digest());
        store.enable_convergence_early_exit();
        store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));
        store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));
        let config_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(config_envelope.await, actor.clone()).await.expect("failed to create config store");
        let draft_store = store::DraftStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(draft_envelope, actor.clone()).await.expect("failed to create draft store");
        let interaction_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(interaction_envelope, actor.clone()).await.expect("failed to create interaction store");
        let genesis_mutations = A::genesis().await;
        if !genesis_mutations.is_empty() {
            store.dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, transaction: None }, identity).await.expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
        }
        let dispatch_report = protocol::DispatchReport { policy: store.merge_policy(), worst: None, messages: Vec::new() };
        let mut framework_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        register_framework_reserved_factories(&mut framework_tool_registry).expect("framework-owned reserved factories must preserve exact owner/controller/schema/tool authority");
        let framework_tool_registrations = framework_tool_registry.finish();
        let mut app_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        A::register_tool_job_factories(&mut app_tool_registry).expect("app-owned tool factories must preserve exact owner/controller/schema/tool authority");
        let app_tool_registrations = app_tool_registry.finish();
        let (tool_job_controller_id, tool_job_registrations) = crate::reserved_jobs::tool_job_registration::<A>(&registry, &app_id, A::DOCUMENT_SCHEMA, <A::Command as ::protocol::OpBinary>::TOOL_JOB_IDS, &tool_jobs, &app_tool_registrations)
            .expect("tool proof catalog must exactly join migrated generated declarations to live concrete factories");
        let artifact_one_item_factory = A::build_artifact_store_one_item_preparation_factory();
        let config_one_item_factory = A::build_config_store_one_item_preparation_factory();
        let draft_one_item_factory = A::build_draft_store_one_item_preparation_factory();
        let presence_one_item_factory = A::build_presence_store_one_item_preparation_factory();
        let transient_one_item_factory = A::build_transient_store_one_item_preparation_factory();
        let presence_local_root_retirement_factory = A::build_presence_local_root_retirement_factory();
        let transient_local_root_retirement_factory = A::build_transient_local_root_retirement_factory();
        let mut window_config_store = WindowConfigOwnerRegistry::new(actor);
        A::register_window_config_owners(&mut window_config_store).expect("window config owners must be uniquely registered by exact window kind");
        let mut window_transient_store = WindowTransientOwnerRegistry::default();
        A::register_window_transient_owners(&mut window_transient_store).expect("window transient owners must be uniquely registered by exact window kind");
        let mut unsupported_publication_contracts = BTreeMap::new();
        for registration in framework_tool_registrations.values().chain(app_tool_registrations.values()) {
            let unsupported = registration.publication_lanes.iter().copied().find_map(|lane| match lane {
                ArtifactToolPublicationLane::Artifact if artifact_one_item_factory.is_none() => Some("artifact"),
                ArtifactToolPublicationLane::Config if config_one_item_factory.is_none() => Some("config"),
                ArtifactToolPublicationLane::Draft if draft_one_item_factory.is_none() => Some("draft"),
                ArtifactToolPublicationLane::Presence if presence_one_item_factory.is_none() || presence_local_root_retirement_factory.is_none() => Some("presence"),
                ArtifactToolPublicationLane::Transient if transient_one_item_factory.is_none() || transient_local_root_retirement_factory.is_none() => Some("transient"),
                ArtifactToolPublicationLane::WindowConfig if window_config_store.is_empty() => Some("window-config"),
                ArtifactToolPublicationLane::WindowTransient if window_transient_store.is_empty() => Some("window-transient"),
                ArtifactToolPublicationLane::Child => None,
                ArtifactToolPublicationLane::Interaction => None,
                ArtifactToolPublicationLane::HostOnly
                | ArtifactToolPublicationLane::Artifact
                | ArtifactToolPublicationLane::Config
                | ArtifactToolPublicationLane::Draft
                | ArtifactToolPublicationLane::Presence
                | ArtifactToolPublicationLane::Transient => None,
                ArtifactToolPublicationLane::WindowConfig => None,
                ArtifactToolPublicationLane::WindowTransient => None,
            });
            if let Some(lane) = unsupported {
                unsupported_publication_contracts.insert(registration.key.tool_id.clone(), lane);
            }
        }
        let instance_operation_owner = ArtifactInstanceOperationOwnerHandle::new(A::build_instance_operation_owner());
        let mut bounded_tool_proofs = BTreeMap::new();
        let mut bounded_tool_contracts = Vec::with_capacity(tool_job_registrations.len());
        for proof in tool_job_registrations {
            let tool_id = proof.row.tool_id.to_string();
            if app_tool_registrations.contains_key(&tool_id) || framework_tool_registrations.contains_key(&tool_id) {
                continue;
            }
            bounded_tool_contracts.push(ArtifactToolPublicContract {
                owner: proof.owner,
                controller_id: proof.row.controller_id.to_string(),
                tool_id: tool_id.clone(),
                schema_id: proof.schema_id(),
                max_raw_wire_bytes: proof.contract().max_raw_wire_bytes,
                publication_lanes: &[ArtifactToolPublicationLane::HostOnly],
            });
            let factory = TypedCommandFullOperationJobFactory::<A>::from_proof(proof.clone()).expect("validated bounded reducer proof must retain its concrete owner authority");
            tool_jobs.register_once(factory).expect("validated bounded reducer factory must register exactly once");
            assert!(bounded_tool_proofs.insert(tool_id, proof).is_none(), "validated bounded reducer proofs must remain unique by tool id");
        }
        let mut presence_store = store::PresenceStore::new(A::Presence::default());
        if let Some(factory) = presence_local_root_retirement_factory.as_ref() {
            presence_store.install_local_retirement_factory(factory.clone()).expect("presence local retirement factory installs exactly once during app construction");
        }
        if let Some(factory) = A::build_presence_peer_retirement_factory() {
            presence_store.install_peer_retirement_factory(factory).expect("presence peer retirement factory installs exactly once during app construction");
        }
        let mut this = Self {
            app,
            mounted_policy,
            constructor_receipts: std::mem::MaybeUninit::uninit(),
            constructor_recipient_admitted: false,
            constructor_recipient_retired: false,
            constructor_terminal_receipt: None,
            constructor_document: OriginalCatalogConstructor::new(),
            constructor_config: OriginalCatalogConstructor::new(),
            constructor_draft: OriginalCatalogConstructor::new(),
            constructor_interaction: OriginalCatalogConstructor::new(),
            constructor_lane: 0,
            store,
            config_store,
            draft_store,
            presence_store,
            own_color: None,
            peer_presence: ManuallyDrop::new(PeerPresenceRoot::empty_shared()),
            peer_presence_generation: 0,
            peer_roster_admission_generation: 0,
            peer_roster_processed_generation: 0,
            peer_roster_outcome_generation: 0,
            peer_roster_reservations: std::array::from_fn(|_| None),
            peer_roster_scope: semio_framework_job::CancelToken::root_now(),
            peer_roster_publications: ArtifactFixedRegistry::new(),
            peer_roster_outcomes: ArtifactFixedRegistry::new(),
            peer_presence_retirements: ArtifactFixedRegistry::new(),
            presence_peer_retirements: ArtifactFixedRegistry::new(),
            transient_store: store::TransientStore::new(A::Transient::default()),
            window_config_store,
            window_transient_store,
            retired_window_transient_stores: ArtifactFixedRegistry::new(),
            maintenance_window_retirement_cursor: 0,
            close_window_retirement_cursor: 0,
            artifact_one_item_factory,
            config_one_item_factory,
            draft_one_item_factory,
            presence_one_item_factory,
            transient_one_item_factory,
            presence_local_root_retirement_factory,
            transient_local_root_retirement_factory,
            unsupported_publication_contracts,
            last_emit_wire: None,
            action_ingress_sources: ArtifactFixedRegistry::unallocated(16),
            pending_tool_lease_admission:ManuallyDrop::new(None),
            pending_media_import_permit:ManuallyDrop::new(None),
            pending_action_ingress:ManuallyDrop::new(None),
            pending_action_ingress_caller: std::mem::ManuallyDrop::new(None),
            pending_action_ingress_identity:None,
            pending_action_ingress_retirement:ManuallyDrop::new(None),
            typed_effect_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_event_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_ui_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            operation_progress_retired: false,
            typed_inline_interaction_verbs: Vec::new(),
            typed_inline_interaction_leftover: None,
            typed_completion_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_composed_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            dispatch_report,
            cache: None,
            registry,
            tool_job_controller_id,
            tool_jobs,
            pending_render_tree:std::mem::ManuallyDrop::new(None),
            pending_render_tree_identity:None,
            pending_render_tree_ready:false,
            pending_render_tree_close:std::mem::ManuallyDrop::new(None),
            pending_configuration_completion:None,
            pending_configuration_command:std::mem::ManuallyDrop::new(None),
            pending_configuration_command_close:None,
            pending_configuration_permit:None,
            pending_configuration_failure:std::mem::ManuallyDrop::new(None),
            pending_configuration_identity:None,
            pending_configuration_retirement:None,
            pending_render_failure:std::mem::ManuallyDrop::new(None),
            pending_render_identity:None,
            pending_render_close_failure:std::mem::ManuallyDrop::new(None),
            pending_render_compound:std::mem::ManuallyDrop::new(None),
            pending_render_retirement:std::mem::ManuallyDrop::new(None),
            agent_lane_preview:None,
            bounded_tool_proofs,
            bounded_tool_contracts,
            framework_tool_registrations,
            app_tool_registrations,
            instance_operation_owner,
            tool_cancellations: crate::tool_cancellation::ToolCancellationIssuer::empty(),
            live_runtime_instance_id: None,
            tool_operations: ArtifactFixedRegistry::new(),
            #[cfg(test)]
            test_tool_clock: None,
            typed_operation_reservations: [None; ARTIFACT_LIVE_OUTPUT_SLOTS],
            typed_publication_cursor: 0,
            typed_result_cursor: 0,
            latest_wins_commands: ArtifactFixedRegistry::new(),
            latest_wins_order: ArtifactFixedQueue::new(ARTIFACT_LIVE_OUTPUT_SLOTS),
            latest_wins_keys: ToolLatestWinsRegistry::new(),
            latest_wins_turn: true,
            media_exports: ArtifactFixedRegistry::new(),
            media_export_preparation:ManuallyDrop::new(None),
            media_download_preparation:ManuallyDrop::new(None),
            pending_reserved: ArtifactFixedRegistry::new(),
            reserved_commits: std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS),
            reserved_commit_outcome: None,
            media_closures: ArtifactFixedRegistry::new(),
            snapshot_retirements: ArtifactFixedRegistry::new(),
            current_media_export: None,
            media_submission_refusal: std::mem::ManuallyDrop::new(None),
            media_submission_refusal_identity: None,
            media_submission_refusal_retirement: std::mem::ManuallyDrop::new(None),
            segmented_downloads: ArtifactFixedRegistry::new(),
            segmented_closures: ArtifactFixedRegistry::new(),
            close_started: false,
            close_cancellation_cursor: 0,
            pending_document_window_admission:std::mem::ManuallyDrop::new(None),
            close_typed_operation_cursor: 0,
            close_media_cursor: 0,
            close_media_cleanup_cursor: 0,
            close_segment_cursor: 0,
            close_segment_cleanup_cursor: 0,
            close_snapshot_cursor: 0,
            close_instance_operation_owner_drained: false,
            close_child_root_cursor: 0,
            close_child_root_detached: false,
            close_child_member_cursor: 0,
            close_child_member: ManuallyDrop::new(None),
            close_peer_presence_cursor: 0,
            close_peer_presence_detached: false,
            close_presence_peer_cursor: 0,
            close_peer_roster_cursor: 0,
            maintenance_stage: 0,
            maintenance_cancellation_cursor: 0,
            maintenance_tool_cursor: 0,
            maintenance_media_cursor: 0,
            maintenance_segment_cursor: 0,
            maintenance_snapshot_cursor: 0,
            maintenance_child_root_cursor: 0,
            maintenance_peer_presence_cursor: 0,
            maintenance_presence_peer_cursor: 0,
            maintenance_peer_roster_cursor: 0,
            envelope_ingress: ArtifactFixedRegistry::new(),
            maintenance_envelope_ingress_cursor: 0,
            close_envelope_ingress_cursor: 0,
            close_envelope_ingress_drained: false,
            envelope_decode_jobs: ArtifactFixedRegistry::new(),
            maintenance_envelope_decode_cursor: 0,
            close_envelope_decode_cursor: 0,
            close_envelope_decode_stage: 0,
            close_envelope_decode_jobs_drained: false,
            store_replacement_jobs: ArtifactFixedRegistry::new(),
            maintenance_store_replacement_cursor: 0,
            close_store_replacement_cursor: 0,
            close_store_replacement_jobs_drained: false,
            import_staging: semio_framework::kernel::ImportStaging::default(),
            document_archive_loads: ArtifactFixedRegistry::with_slots(DOCUMENT_ARCHIVE_LOAD_SLOTS),
            maintenance_document_archive_cursor: 0,
            close_document_archive_cursor: 0,
            close_document_archive_loads_drained: false,
            envelope_field_decoders: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry::new(),
            envelope_field_decoder_retirements: ArtifactFixedRegistry::new(),
            maintenance_envelope_field_decoder_cursor: 0,
            close_envelope_field_decoder_cursor: 0,
            close_envelope_field_decoders_drained: false,
            envelope_completed_records: store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry::new(),
            envelope_completed_record_retirements: ArtifactFixedRegistry::new(),
            maintenance_envelope_completed_record_cursor: 0,
            close_envelope_completed_record_cursor: 0,
            close_envelope_completed_records_drained: false,
            document_snapshot_read_returns: SnapshotReadReturnPump::new(),
            close_snapshot_reads_drained: false,
            close_owned_stage: 0,
            close_owned_advance_ready: false,
            close_document_disposer: ManuallyDrop::new(A::build_document_store_disposer()),
            close_config_disposer: ManuallyDrop::new(A::build_config_store_disposer()),
            close_draft_disposer: ManuallyDrop::new(A::build_draft_store_disposer()),
            close_presence_disposer: ManuallyDrop::new(A::build_presence_store_disposer()),
            close_transient_disposer: ManuallyDrop::new(A::build_transient_store_disposer()),
            close_interaction_disposer: ManuallyDrop::new(Some(Box::new(ArtifactDocumentStoreDisposer::<protocol::InteractionState, InteractionConfigMutation>::new()))),
            invocation_kind: None,
            host_event_utilities: HashMap::new(),
            host_event_roster: Vec::new(),
            host_event_operations: HashSet::new(),
            delivering_host_event: false,
            command_log: PagedCommandLog::new(),
            command_prune_generation: 0,
            command_prune_processed: 0,
            pending_command_prune: None,
            next_command_seq: 0,
            log_generation: 0,
            history_dirty_sequences: HashSet::new(),
            history_view: None,
            history_backfill: None,
            shell_undone: HashSet::new(),
            shell_redo: Vec::new(),
            history_filter: HistoryCommandFilter::default(),
            children: ChildMemberRegistry::new(),
            private_child_groups: ArtifactFixedRegistry::new(),
            child_content_root: ManuallyDrop::new(ChildContentView::EMPTY),
            child_content_generation: 0,
            child_content_retirements: ArtifactFixedRegistry::new(),
            child_member_retirements: ArtifactFixedRegistry::new(),
            child_member_retirement_generation: 0,
            child_member_retirement_cursor: 0,
            followed_parent_generation: 0,
            child_follow_awaits_retirement: false,
            pending_child_pins: Vec::new(),
            composition: CompositionCoordinator::new().await,
            interaction_store,
            local_interaction_query: None,
            local_interaction_query_receipt: None,
            local_interaction_authority: Default::default(),
            snapshot_read_return_stage: 0,
            local_interaction_query_turn: false,
            local_interaction_emitted_bytes: 0,
            local_interaction_retired_bytes: 0,
            config_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_hover: InteractionHoverState::new(),
            interaction_leftover_selection: None,
            interaction_leftover_ids: Vec::new(),
            interaction_ui_topology: HashMap::new(),
            interaction_topology_memo: None,
            pending_transaction: None,
            pending_transaction_proposal: None,
            pending_presence: Vec::new(),
            presence_marked_keys: BTreeMap::new(),
            tool_runs: ToolRunLedger::default(),
            time_travel: TimeTravelLedger::default(),
            tool_machines: ToolMachineRuntime::default(),
            supersedes: SupersedeLedger::default(),
        };
        this.seed_genesis_children(identity).await.expect("ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store");
        this.followed_parent_generation = this.store.generation_now();
        this
    }

    fn has_constructor_receipt(&self)->bool { self.constructor_terminal_receipt.is_some() || (self.constructor_recipient_admitted && unsafe { self.constructor_receipts.assume_init_ref() }.iter().any(Option::is_some)) }
    fn constructor_receipt(&self)->Option<&(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if let Some(receipt)=self.constructor_terminal_receipt.as_ref(){return Some(receipt)} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_ref()}.iter().find_map(Option::as_ref) }
    fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if self.constructor_terminal_receipt.is_some(){return self.constructor_terminal_receipt.take()} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_mut()}.iter_mut().find_map(Option::take) }
    fn store_constructor_is_pending(&self)->bool { (!self.constructor_recipient_admitted&&!self.constructor_recipient_retired) || self.has_constructor_receipt() || self.constructor_lane<4 }
    pub(crate) fn drive_store_constructor_unit(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError> {self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Preparation)}
    fn drive_store_constructor_unit_in_phase(&mut self,grant:RetainedCloneGrant,phase:ArtifactStoreConstructorPhase)->Result<bool,ValueError> {
        if self.has_constructor_receipt(){return Ok(true)}
        if !self.constructor_recipient_admitted {
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            let recipient=self.constructor_receipts.write([None;24]);self.constructor_recipient_admitted=true;
            recipient[0]=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        if self.constructor_lane>=4{return Ok(false)}
        let copied_bytes=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copied_bytes,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match self.constructor_lane {
            0=>{let receipt=self.constructor_document.advance(&mut self.store,child,A::build_document_store_owners)?;(ArtifactStoreConstructorKind::Document,receipt,self.constructor_document.terminal_is_empty())},
            1=>{let receipt=self.constructor_config.advance(&mut self.config_store,child,A::build_config_store_owners)?;(ArtifactStoreConstructorKind::Config,receipt,self.constructor_config.terminal_is_empty())},
            2=>{let receipt=self.constructor_draft.advance(&mut self.draft_store,child,A::build_draft_store_owners)?;(ArtifactStoreConstructorKind::Draft,receipt,self.constructor_draft.terminal_is_empty())},
            _=>{let receipt=self.constructor_interaction.advance(&mut self.interaction_store,child,|grant|Some(crate::local_interaction::retirement::interaction_store_owners(grant)))?;(ArtifactStoreConstructorKind::Interaction,receipt,self.constructor_interaction.terminal_is_empty())},
        };
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog parent exceeded its full grant"))}
        if done { self.constructor_lane+=1; }
        receipt.copied_bytes=receipt.copied_bytes.checked_add(copied_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original catalog outer header extent overflow"))?;
        unsafe{self.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    pub(crate) fn constructor_terminal_is_empty(&self)->bool{self.constructor_recipient_retired&&!self.has_constructor_receipt()&&self.constructor_document.terminal_is_empty()&&self.constructor_config.terminal_is_empty()&&self.constructor_draft.terminal_is_empty()&&self.constructor_interaction.terminal_is_empty()}
    pub(crate) fn constructor_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.has_constructor_receipt(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original constructor receipt awaits its recipient"))}
        if self.constructor_recipient_retired{return Ok(Default::default())}
        if !self.constructor_recipient_admitted{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        if self.constructor_lane>=4{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        let parent=0usize;
        let mut demand=match self.constructor_lane{0=>self.constructor_document.demands(body.saturating_sub(parent),self.store.owned_disposer_installed(),A::document_store_owners_source_demands)?,1=>self.constructor_config.demands(body.saturating_sub(parent),self.config_store.owned_disposer_installed(),A::config_store_owners_source_demands)?,2=>self.constructor_draft.demands(body.saturating_sub(parent),self.draft_store.owned_disposer_installed(),A::draft_store_owners_source_demands)?,_=>self.constructor_interaction.demands(body.saturating_sub(parent),self.interaction_store.owned_disposer_installed(),crate::local_interaction::retirement::interaction_store_owners_source_demands)?};
        demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"constructor close parent quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"constructor close parent depth overflow"))?;Ok(demand)
    }
    pub(crate) fn close_store_constructor_unit(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError>{
        let phase=ArtifactStoreConstructorPhase::Close;
        if self.has_constructor_receipt(){return Ok(true)}
        if self.constructor_recipient_retired{return Ok(false)}
        let installed=match self.constructor_lane{0=>self.store.owned_disposer_installed(),1=>self.config_store.owned_disposer_installed(),2=>self.draft_store.owned_disposer_installed(),_=>self.interaction_store.owned_disposer_installed()};
        if self.constructor_lane<4&&!installed{return self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Close)}
        if !self.constructor_recipient_admitted{return self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Close)}
        if self.constructor_lane>=4{
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            unsafe{self.constructor_receipts.assume_init_drop()}self.constructor_recipient_admitted=false;self.constructor_recipient_retired=true;self.constructor_terminal_receipt=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        let parent=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match self.constructor_lane{0=>{let receipt=self.constructor_document.close(child)?;(ArtifactStoreConstructorKind::Document,receipt,self.constructor_document.terminal_is_empty())},1=>{let receipt=self.constructor_config.close(child)?;(ArtifactStoreConstructorKind::Config,receipt,self.constructor_config.terminal_is_empty())},2=>{let receipt=self.constructor_draft.close(child)?;(ArtifactStoreConstructorKind::Draft,receipt,self.constructor_draft.terminal_is_empty())},_=>{let receipt=self.constructor_interaction.close(child)?;(ArtifactStoreConstructorKind::Interaction,receipt,self.constructor_interaction.terminal_is_empty())}};
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog close exceeded its full grant"))}
        if done{self.constructor_lane+=1}receipt.copied_bytes+=parent;unsafe{self.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    /// 🌱️ Opens every composed child the initial snapshot declares and the app derives
    /// (`ArtifactApp::genesis_child_pack`) through the ordinary restore path, so `children`, the
    /// composition graph and the child content root are complete from the first frame — a saved
    /// archive of a never-edited document then carries its derivable members instead of relying on
    /// the loading side to re-derive them.
    async fn seed_genesis_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut genesis = Vec::new();
        {
            let snapshot = self.store.snapshot_ref();
            let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("initial snapshot child projection failed: {error}")))?;
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
                let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                genesis.push((slot.to_string(), fields.child_id.to_string(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
            }
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        Ok(())
    }

    /// 🪆️ Derivable composed children follow their coordinate. A content-addressed child is minted from the
    /// parent's own content, so a parent-lane change can re-point a declared coordinate at a child id no store
    /// holds (P8 `composedChildOrphaned`). This opens every declared child no store holds that
    /// `ArtifactApp::genesis_child_pack` derives — the restore path `seed_genesis_children` uses — and retires
    /// every held child whose slot now names another derived child, so exporters, archives, hub members,
    /// agents and the next verb read the child the parent names. Gated by the parent store's generation and
    /// deferred while a child admission is in flight or while the named child is still retiring (an undo back
    /// to it) — the pass then repeats until that retirement drained. It also waits, before it changes anything,
    /// while the retirements its own publications need are not yet admissible ([`Self::reclaim_child_retirements`]):
    /// the parent already holds every edit in order, the child it names follows once maintenance returned them, and
    /// the instance stays runnable meanwhile (`child_follow_awaits_retirement`).
    pub(crate) async fn follow_derivable_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let generation = self.store.generation_now();
        if generation == self.followed_parent_generation || self.children.has_admission_in_flight() {
            return Ok(());
        }
        let mut genesis = Vec::new();
        let mut retire = Vec::new();
        let mut deferred = false;
        {
            let snapshot = self.store.snapshot_ref();
            let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
            let mut declared = Vec::with_capacity(projection.len());
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let key = (slot.to_string(), fields.child_id.to_string());
                if self.children.get(&key).is_none() {
                    if crate::child_content::member_is_retiring(&self.child_member_retirements, fields.artifact_id) {
                        deferred = true;
                    } else if let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? {
                        let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                        genesis.push((key.0.clone(), key.1.clone(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
                    }
                }
                declared.push(key);
            }
            for (member, _) in self.children.keyed_entries().filter(|(member, _)| member.owner.is_empty()) {
                let key = (member.slot.to_string(), member.child_id.to_string());
                if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
                    retire.push(key);
                }
            }
        }
        let publications = retire.len() + genesis.len();
        if !self.reclaim_child_retirements(publications, publications)? {
            self.child_follow_awaits_retirement = true;
            return Ok(());
        }
        for (slot, child_id) in retire {
            self.retire_followed_child(&slot, &child_id).await?;
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("followed child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        if !deferred {
            self.followed_parent_generation = generation;
        }
        self.child_follow_awaits_retirement = deferred;
        Ok(())
    }

    /// 🍂️ Moves one held child its coordinate left out of every live authority — the member map, the ownership
    /// graph and the published child-content root (the previous root goes to bounded content retirement) —
    /// into bounded child-member retirement, where it stays the disposer of the snapshot leases that previous
    /// root still holds (`ChildContentRetirement::close_step` finds it there) and closes only after them.
    async fn retire_followed_child(&mut self, slot: &str, child_id: &str) -> Result<(), Fault> {
        let retirement_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child member retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("child member retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let publication_generation = self.admit_child_content_publication()?;
        let next = self.child_content_root.without_member(MemberKeyRef::root(slot, child_id))?;
        let entry = self.children.remove(MemberKeyRef::root(slot, child_id))?.ok_or_else(|| plugin_sdk_fault("followed child left the member map before its exact retirement"))?;
        self.composition.graph_mut().await.remove_owns(&entry.reference.artifact_id).await;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = publication_generation;
        self.child_member_retirements.insert_admitted(retirement_generation, ChildMemberRetirement::new(entry));
        self.child_member_retirement_generation = retirement_generation;
        Ok(())
    }

    pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
        semio_framework_job::Generation(self.store.generation_now())
    }

    /// 🫧️ Reads the immutable transient projection of one exact live window.
    pub fn window_transient_snapshot(&mut self, view_state: &ViewModel) -> Result<Option<WindowTransientSnapshot>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.snapshot))
    }

    pub fn window_transient_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.generation))
    }

    pub async fn window_config_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_config_store.capture(Some(view_state)).await?.map(|authority| authority.generation))
    }

    /// 📃️ One bounded unit of the live local-interaction query. Its own captured roots are the
    /// only ownership it advances: the Store-side reclamation of the leases it hands back is
    /// [`Self::advance_snapshot_read_returns_one`]'s cooperative-maintenance business, because a
    /// registry's one-slot-per-step cleanup cursor is bounded by the registry's fixed capacity
    /// and would otherwise put a whole capacity's worth of probes inside a host continuation
    /// budget before a read could publish its terminal reply.
    fn advance_local_interaction_query_one(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, output_width: usize) -> Result<(), Fault> {
        if self.local_interaction_query_receipt.is_some(){return Ok(())}
        let Some(query)=self.local_interaction_query.as_mut()else{return Ok(())};
        let result=if query.is_closing(){
            let before=query.retired_bytes();let result=query.close_step(grant.retained_grant());self.local_interaction_retired_bytes+=(query.retired_bytes()-before)as u128;result.map(|step|step.progress())
        }else{
            query.advance(grant,output_width).map(|step|{if let crate::local_interaction::live::LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,..}=step{self.local_interaction_emitted_bytes+=emitted_bytes as u128;self.local_interaction_retired_bytes+=retired_bytes as u128;}step.progress()})
        };
        let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
        if progress!=Default::default(){self.local_interaction_query_receipt=Some((grant.retained_grant(),progress));}
        result.map(|_|()).map_err(|error|plugin_sdk_fault(error.to_string()))
    }

    /// ♻️ One bounded unit of the fixed document/config/interaction/transient returned-read rotation: each
    /// pump either drains the disposer it owns or probes its Store for the next returned lease.
    fn snapshot_read_returns_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let stage=self.next_snapshot_read_return_stage().unwrap_or(self.snapshot_read_return_stage);
        let parent_copy=size_of::<usize>();let child=match stage{0=>self.document_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.store.returned_snapshot_read_retirement_demand())?,1=>self.config_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.config_store.returned_snapshot_read_retirement_demand())?,2=>self.interaction_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.interaction_store.returned_snapshot_read_retirement_demand())?,_=>match self.transient_local_root_retirement_factory.as_deref(){Some(factory)=>self.transient_store.maintenance_returned_reads_demands_root(factory,body.saturating_sub(parent_copy))?,None=>Default::default()}};ChildContentRetirement::parent_snapshot_demand(child,parent_copy)
    }

    fn advance_snapshot_read_returns_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let demand=self.snapshot_read_returns_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let stage = self.next_snapshot_read_return_stage().unwrap_or(self.snapshot_read_return_stage);
        let next_demand=match stage{0=>self.store.returned_snapshot_read_retirement_demand(),1=>self.config_store.returned_snapshot_read_retirement_demand(),2=>self.interaction_store.returned_snapshot_read_retirement_demand(),_=>Ok(Default::default())}.map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=match stage{0=>self.document_snapshot_read_returns.drive(next_demand,|original|self.store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),1=>self.config_snapshot_read_returns.drive(next_demand,|original|self.config_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),2=>self.interaction_snapshot_read_returns.drive(next_demand,|original|self.interaction_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),_=>match self.transient_local_root_retirement_factory.clone(){Some(factory)=>self.transient_store.maintenance_returned_reads_step_root(factory.as_ref(),child).map_err(|error|plugin_sdk_fault(error.into_message())),None=>Ok(RetainedCloneStep::Progress(empty))}}?;let mut receipt=step.progress();self.snapshot_read_return_stage=(stage+1)%4;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original snapshot rotation preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original snapshot return rotation").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    /// 👁️ Borrows whether the original returned-read frontier allows another publication item.
    /// Paid retirement remains on the mounted maintenance turn before another semantic callback.
    ///
    /// 🐛️ Every batch item is prepared against a registry lease on the previous item's post root, and the registry
    /// keeps that root alive after the lease returns until a pump takes it. The maintenance rotation reclaims one
    /// returned read per rotation at one item per step, while a publication folds an item every turn — so a
    /// 408-placement fill finalize (816 ops over a 409-object puzzle 3d document) retained one ~5 MB root per
    /// folded op and trapped the guest at its 512 MiB ceiling (`memory allocation of 1232 bytes failed`, ticket
    /// 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS). Draining here bounds a publication to the staged root plus the
    /// base its current item reads, whatever the batch size.
    ///
    /// A disposer that waits on outside ownership cannot be hurried here; the publication must not stall on it.
    fn reclaim_document_snapshot_read_returns(&self)->Result<bool,Fault>{
        Ok(self.document_snapshot_read_returns.terminal_is_empty() && self.store.returned_snapshot_read_count() == 0)
    }

    /// 🪹️ No capture-store disposer or queued document read still awaits acknowledgement.
    fn snapshot_read_returns_terminal_is_empty(&self) -> bool {
        self.next_snapshot_read_return_stage().is_none()
    }

    /// 🧭️ The first rotation stage at or after the cursor whose pump or store still holds a returned read.
    fn next_snapshot_read_return_stage(&self) -> Option<u8> {
        (0..4u8).map(|offset| (self.snapshot_read_return_stage + offset) % 4).find(|stage| match stage {
            0 => !self.document_snapshot_read_returns.terminal_is_empty() || self.store.returned_snapshot_read_count() != 0,
            1 => !self.config_snapshot_read_returns.terminal_is_empty() || self.config_store.returned_snapshot_read_count() != 0,
            2 => !self.interaction_snapshot_read_returns.terminal_is_empty() || self.interaction_store.returned_snapshot_read_count() != 0,
            _ => self.transient_local_root_retirement_factory.is_some() && self.transient_store.returned_read_pending(),
        })
    }

    /// 🕰️ The fixed maintenance stage the NEXT [`PluginApp::maintenance_step`] call will run.
    /// A step-budget law reads this before each unit so an over-budget unit names its own stage
    /// instead of the stale round-robin cursor an idle early return leaves behind.
    pub fn next_maintenance_stage(&self) -> u8 {
        self.maintenance_stage
    }

    /// 🎟️ Pre-admits the exact page and byte extent before a producer constructs any
    /// envelope input owner. Occupied fixed slots and allocation failure leave no input owner.
    pub fn begin_artifact_envelope_ingress(&mut self, maximum_pages: usize, maximum_bytes: usize) -> Result<ArtifactEnvelopeDecodeOperationHandle, Fault> {
        let generation = self.artifact_generation_now();
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-saturated"), "every fixed artifact envelope ingress slot already owns a live operation"));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let pages = store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages, maximum_bytes })
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-credits"), format!("artifact envelope ingress credits were rejected: {fault:?}")))?;
        self.envelope_ingress.insert_admitted(operation.0, ActiveArtifactEnvelopeIngress::new(pages));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 📦️ Admits one already-decoded authoritative `.pack` + `.spr` envelope directly
    /// into the app-owned retained initializer. This path preserves the binary document format;
    /// the fresh JSON envelope decoder remains a separate ingress contract.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact decoded envelope so its domain retirement catalog can close every nested owner.")]
    fn begin_persisted_document_store_replacement(&mut self, envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationHandle, (Fault, ArtifactEnvelope<A::Snapshot, A::Mutation>)> {
        let generation = self.artifact_generation_now();
        let expected_dialect: ArtifactDialect = A::DIALECT.into();
        if envelope.schema != A::DOCUMENT_SCHEMA || envelope.dialect.as_ref() != Some(&expected_dialect) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-envelope-identity"), "persisted document envelope does not match the app's exact schema and dialect"), envelope));
        }
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-ingress-saturated"), "every fixed persisted document replacement slot already owns a live operation"), envelope));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let job = match A::build_document_store_initialization_job(envelope, operation, generation, self.store.local_actor_id().clone()) {
            Ok(job) => job,
            Err(envelope) => {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-initializer-refused"), "app refused the persisted document's retained initialization authority"), envelope));
            }
        };
        self.store_replacement_jobs.insert_admitted(operation.0, ActiveArtifactStoreReplacement::new(operation,generation,self.child_content_generation,job,(cx.operation(),cx.generation())));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 🔎️ Rejects stale, closed, full, or over-byte page ingress before the producer copies its raw owner.
    pub fn preflight_artifact_envelope_ingress_page(&self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize) -> Result<(), Fault> {
        if self.artifact_generation_now() != handle.generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page preflight"));
        }
        let ingress = self.envelope_ingress.get(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
        if ingress.closing {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
        }
        ingress
            .pages
            .as_ref()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?
            .preflight_page_bytes(page_bytes)
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page preflight was rejected before owner construction: {fault:?}")))
    }

    /// 📨️ Runs the bounded page producer only after the same mutable ingress authority accepts its extent.
    pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize, build: Build) -> Result<(), Fault>
    where
        Build: FnOnce() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage,
    {
        self.preflight_artifact_envelope_ingress_page(handle, page_bytes)?;
        let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle after preflight"))?;
        let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority after preflight"))?;
        build().admit_preflighted_into(pages);
        Ok(())
    }

    /// 📥️ Moves one exact fixed page into a pre-admitted operation or returns that page
    /// untouched with the fault. No page clone or aggregate command buffer is constructed.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn admit_artifact_envelope_ingress_page(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage) -> Result<(), (Fault, store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage)> {
        if self.artifact_generation_now() != handle.generation {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page admission"), page));
        }
        let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"), page));
        };
        if ingress.closing {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"), page));
        }
        let Some(pages) = ingress.pages.as_mut() else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"), page));
        };
        page.admit_into(pages).map_err(|(fault, page)| (Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page was rejected untouched: {fault:?}")), page))
    }

    /// 🔒️ Seals and transfers the exact page authority into the retained decoder. A
    /// saturated decoder keeps the sealed ingress owner in its original fixed slot for retry.
    pub fn seal_artifact_envelope_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        if self.artifact_generation_now() != handle.generation {
            self.cancel_artifact_envelope_load(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before sealing"));
        }
        if !self.envelope_decode_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        {
            let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
            if ingress.closing {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
            }
            let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?;
            if !pages.is_sealed() {
                pages.seal().map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-seal"), format!("artifact envelope ingress could not seal: {fault:?}")))?;
            }
        }
        let mut ingress = self.envelope_ingress.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress changed before exact transfer"))?;
        let pages = ingress.take_pages().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact transfer owner"))?;
        match self.submit_artifact_envelope_decode(handle.operation, handle.generation, pages,cx) {
            Ok(submitted) if submitted == handle => {
                drop(ingress);
                Ok(true)
            }
            Ok(_) => unreachable!("exact envelope transfer preserves its operation and generation"),
            Err(pages) => {
                *ingress.pages = Some(pages);
                self.envelope_ingress.insert_admitted(handle.operation.0, ingress);
                Ok(false)
            }
        }
    }

    /// 🔄️ Advances decode-to-initializer handoff without running either job inline. A decode that
    /// has not reached [`ArtifactEnvelopeDecodeOperationPoll::Ready`] reports its OWN poll — the
    /// store replacement it will hand to does not exist yet, and reading that absent job here
    /// turned every in-progress decode into a terminal `Fault` for all eight plugin loops
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn advance_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationPoll, crate::PluginLifecycleFailure> {
        if let Some(active) = self.store_replacement_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "store replacement handle is stale").into());
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if let Some(active) = self.envelope_decode_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "envelope decode handle is stale").into());
            }
            let decode = active.poll();
            if decode != ArtifactEnvelopeDecodeOperationPoll::Ready {
                return Ok(decode);
            }
            let _=self.advance_ready_envelope_decode_worker_owners(handle,cx.retained_grant(),cx)?;
            if !self.artifact_envelope_decode_worker_owners_are_terminal(handle) {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            if !self.try_begin_artifact_store_replacement(handle,cx)? {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if self.envelope_ingress.get(handle.operation.0).is_some() {
            return Ok(ArtifactEnvelopeDecodeOperationPoll::Pending);
        }
        Ok(ArtifactEnvelopeDecodeOperationPoll::Fault)
    }

    pub fn cancel_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        if let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) {
            ingress.closing = true;
            return Ok(());
        }
        if self.envelope_decode_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_envelope_decode(handle);
        }
        if self.store_replacement_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_store_replacement(handle);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.load-stale-handle"), "unknown artifact envelope load handle"))
    }

    /// 📥️ Admits one exact fixed-page envelope owner into app maintenance. A missing
    /// domain catalog or occupied operation slot returns every page untouched.
    pub fn submit_artifact_envelope_decode(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let Some(bundle) = A::build_envelope_decode_owner_bundle() else { return Err(pages) };
        let completion = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion::new();
        let fields = bundle.begin_fresh_decoder(operation, generation, std::sync::Arc::clone(&self.envelope_completed_records), std::sync::Arc::clone(&completion));
        self.admit_artifact_envelope_decode_owner(operation, generation, pages, fields, completion,cx)
    }

    /// 📥️ Admits one exact envelope decode against a caller-supplied field owner. The domain
    /// route builds that owner from its declared catalog; the framework's own decode-ladder laws
    /// supply a bare one so the ladder is provable without a domain field catalog.
    pub(crate) fn admit_artifact_envelope_decode_owner(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        fields: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<A::Snapshot, A::Mutation>>,
        completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let record = store::os_store::owned_schema_decode::artifact_envelope_decode_record(operation, generation, pages)?;
        let active = match store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority::try_new(record, &self.envelope_field_decoders, fields) {
            Ok(decode) => ActiveArtifactEnvelopeDecode::admitted(operation,generation,decode,completion,(cx.operation(),cx.generation())),
            Err((record, _fault, fields)) => ActiveArtifactEnvelopeDecode::rejected(operation, generation, Box::new(store::os_store::owned_schema_decode::ArtifactEnvelopeUnadmittedDecodeRejected::new(record, fields)), completion),
        };
        self.envelope_decode_jobs.insert_admitted(operation.0, active);
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    pub fn poll_artifact_envelope_decode(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, ActiveArtifactEnvelopeDecode::poll)
    }

    /// 🧹️ Whether the decode worker session/outcome ladder is fully retired for one live handle.
    pub fn artifact_envelope_decode_worker_owners_are_terminal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> bool {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).is_none_or(ActiveArtifactEnvelopeDecode::decode_worker_owners_are_terminal)
    }

    /// 🎟️ Advances the original ready worker ladder once under the caller's unchanged grant.
    pub(crate) fn advance_ready_envelope_decode_worker_owners(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, grant: RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        let Some(active)=self.envelope_decode_jobs.get_mut(handle.operation.0)else{return Ok(PluginLifecycleStep::Complete(Default::default()));};
        if active.operation!=handle.operation||active.generation!=handle.generation{return Err(plugin_sdk_fault("ready envelope worker advancement lost its original handle").into());}
        if active.decode_worker_owners_are_terminal(){return Ok(PluginLifecycleStep::Complete(Default::default()));}
        active.drive(cx,active.generation,&self.envelope_completed_records,grant)
    }

    /// 📤️ Applies the sole non-suspending completed-record publication after exact live
    /// generation validation. Backpressure leaves both the record and active operation retained.
    pub fn try_publish_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation>) -> Result<bool, Fault> {
        if semio_framework_job::Generation(self.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-publication"), "decoded envelope generation changed before exact publication"));
        }
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        if active.state != ActiveArtifactEnvelopeDecodeState::Ready || active.cancel.is_cancelled_now() {
            return Ok(false);
        }
        let ticket = active.completion.ticket().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.output-missing"), "ready envelope decode has no exact completed-record ticket"))?;
        match self.envelope_completed_records.try_publish_to(ticket, target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;
                if !active.terminal_is_empty(&self.envelope_completed_records) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-terminal"), "published envelope operation retained a nonterminal owner"));
                }
                let active =
                    self.envelope_decode_jobs.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-owner"), "published envelope operation changed before exact removal"))?;
                drop(active);
                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-ticket"), "completed envelope ticket was stale or invalid")),
        }
    }

    pub fn cancel_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        active.cancel.cancel_now();
        Ok(())
    }

    /// 🏗️ Atomically transfers one decoded envelope into the app-owned retained store
    /// initializer. Capacity or factory rejection leaves the completed record in its exact slot.
    pub fn try_begin_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        if semio_framework_job::Generation(self.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-admission"), "store replacement generation changed before initializer admission").into());
        }
        if !self.store_replacement_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        let ticket = self
            .envelope_decode_jobs
            .get(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation && active.state == ActiveArtifactEnvelopeDecodeState::Ready && !active.cancel.is_cancelled_now())
            .and_then(|active| active.completion.ticket())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-output-not-ready"), "decoded envelope is not ready for retained store initialization"))?;
        if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){let _=self.advance_ready_envelope_decode_worker_owners(handle,cx.retained_grant(),cx)?;return Ok(false);}
        let child_content_generation = self.child_content_generation;
        let mut target = ArtifactStoreReplacementAdmissionTarget::<A, M> { source_identity:(cx.operation(),cx.generation()),jobs: &mut self.store_replacement_jobs, operation: handle.operation, generation: handle.generation, child_content_generation, actor: self.store.local_actor_id().clone(), marker: std::marker::PhantomData };
        match self.envelope_completed_records.try_publish_to(ticket, &mut target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){return Err(plugin_sdk_fault("initializer transfer changed its already admitted original decode worker witness").into());}
                let active = self
                    .envelope_decode_jobs
                    .get_mut(handle.operation.0)
                    .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-owner"), "decoded envelope owner changed during exact initializer admission"))?;
                if !active.decode_worker_owners_are_terminal() {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-terminal"), "decoded envelope retained an owner after initializer transfer").into());
                }
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;

                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-completed-ticket"), "decoded envelope completed-record ticket was stale or invalid").into()),
        }
    }

    /// 🌳️ Pre-admits the exact recursive member count after the retained parent initializer
    /// is terminal and before any member input may enter the candidate transaction.
    pub fn try_begin_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member ingress has no matching replacement authority"))?;
        active.begin_members(expected, expires_at_us)
    }

    /// 📥️ Transfers one admitted member-open request into its exact retained ordinal.
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_owned_document_member(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        let Some(active) = self.store_replacement_jobs.get_mut(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation) else {
            return Err((plugin_sdk_fault("owned document member ingress has no matching replacement authority"), ingress));
        };
        active.admit_member(ingress)
    }

    /// 🔒️ Freezes the complete ordinal set so bounded typed opens can begin.
    pub fn seal_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member seal has no matching replacement authority"))?;
        active.seal_members()
    }

    pub fn poll_artifact_store_replacement(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, |active| match active.state {
            ActiveArtifactStoreReplacementState::Initializing
            | ActiveArtifactStoreReplacementState::AwaitingMembers
            | ActiveArtifactStoreReplacementState::OpeningMembers
            | ActiveArtifactStoreReplacementState::ValidatingClosure
            | ActiveArtifactStoreReplacementState::PreparingCandidateViews
            | ActiveArtifactStoreReplacementState::CandidateReady => ArtifactEnvelopeDecodeOperationPoll::Pending,
            ActiveArtifactStoreReplacementState::ClosingRejectedMember
            | ActiveArtifactStoreReplacementState::RetiringRejectedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedStore
            | ActiveArtifactStoreReplacementState::RetiringRejectedCandidate => ArtifactEnvelopeDecodeOperationPoll::Progress,
            ActiveArtifactStoreReplacementState::Complete if active.committed && active.refusal.is_none() => ArtifactEnvelopeDecodeOperationPoll::Ready,
            ActiveArtifactStoreReplacementState::Complete if matches!(active.refusal, Some(ArtifactStoreReplacementRefusal::Cancelled)) => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactStoreReplacementState::Complete => ArtifactEnvelopeDecodeOperationPoll::Fault,
        })
    }

    /// 🧭️ The exact leg that refused this replacement, minted as a typed fault. Read BEFORE
    /// {@link acknowledge_artifact_store_replacement} retires the operation — after that the
    /// evidence is gone and only the generic collapse remains.
    pub fn artifact_store_replacement_refusal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Option<Fault> {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).and_then(|active| active.refusal).map(ArtifactStoreReplacementRefusal::into_fault)
    }

    pub fn acknowledge_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<bool, Fault> {
        let Some(active) = self.store_replacement_jobs.get(handle.operation.0) else { return Ok(false) };
        if active.operation != handle.operation || active.generation != handle.generation || !active.terminal_is_empty() {
            return Ok(false);
        }
        let active = self
            .store_replacement_jobs
            .remove(handle.operation.0)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-terminal-owner"), "terminal store replacement changed before exact acknowledgement"))?;
        drop(active);
        Ok(true)
    }

    pub fn cancel_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "unknown or stale store replacement handle"))?;
        if !active.committed {
            active.request_cancel();
        }
        Ok(())
    }

    /// 🌱️ Completes an archive's member roster with every child the hydrated candidate parent declares,
    /// the archive omits, and the app derives (`ArtifactApp::genesis_child_pack`) — the content-addressed
    /// members a whole-document load (`Effect::LoadDocument`, which carries pack+spr only) can never
    /// ship. Each minted entry is owner-stamped to the candidate root and takes the next ordinal after the
    /// archived ones; the fixed member and byte authorities are re-checked over the completed roster. A
    /// slot the app cannot derive is left alone, so closure validation still rejects it as `Incomplete`.
    ///
    /// 🔢️ Archived entries are already reversed for ordinal-ordered popping; ordinals stay
    /// `0..n` for them and continue at `n` for every minted member, whatever the pop order.
    fn complete_document_archive_genesis(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive member roster owner is absent"))?;
        let candidate = self
            .store_replacement_jobs
            .get(handle.operation.0)
            .filter(|replacement| replacement.operation == handle.operation && replacement.generation == handle.generation)
            .and_then(ActiveArtifactStoreReplacement::candidate_awaiting_members)
            .ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis has no hydrated candidate parent awaiting members"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive genesis child projection failed: {error}")))?;
        let mut ordinal = archive.members.len();
        let mut payload_bytes = archive.members.iter().fold(archive.parent_spr.len(), |total, entry| total + entry.envelope_pack.len());
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            if archive.members.iter().any(|entry| entry.owner.slot == slot && entry.owner.child_id == fields.child_id) {
                continue;
            }
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive genesis exceeds its fixed 1024-member authority").with_param("count", (ordinal + 1).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
            }
            payload_bytes = payload_bytes.checked_add(envelope_pack.len()).filter(|bytes| *bytes <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive genesis exceeds its fixed byte authority").with_param("bytes", payload_bytes.saturating_add(envelope_pack.len()).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()))?;
            archive.members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
            active.total = active.total.saturating_add(1);
        }
        Ok(())
    }

    /// 🌱️ Mints every derivable composed child onto a live envelope replacement's member roster —
    /// the JSON ingress path ships only the parent envelope, so `ArtifactApp::genesis_child_pack`
    /// must supply the same members `seed_genesis_children` opened at boot.
    fn complete_store_replacement_genesis(&mut self, operation_id: u64) -> Result<(), Fault> {
        let active = self.store_replacement_jobs.get(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis lost its exact replacement authority"))?;
        let handle = ArtifactEnvelopeDecodeOperationHandle { operation: active.operation, generation: active.generation };
        let candidate = active.retained_store.as_ref().ok_or_else(|| plugin_sdk_fault("live store replacement genesis has no hydrated candidate parent"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| plugin_sdk_fault("live store replacement genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("live store replacement genesis child projection failed: {error}")))?;
        let mut entries = Vec::new();
        let mut ordinal = 0usize;
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(plugin_sdk_fault("live store replacement genesis exceeds its fixed 1024-member authority"));
            }
            entries.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("live store replacement genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
        }
        let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis changed before member admission"))?;
        active.begin_members(entries.len(), u64::MAX)?;
        for entry in entries {
            let mut pending = PendingDocumentArchiveMember::new(entry).map_err(|(fault, _)| fault)?;
            while !pending.fill_one_page(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES)? {}
            while !matches!(pending.retire_source_step(RetainedCloneGrant { maximum_copy_bytes: store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES, ..plugin_demand_grant(pending.source_retirement_demands()) }), RetainedCloneStep::Complete(_)) {}
            let ingress = pending.take_ingress(handle, self.store.local_actor_id().clone()).map_err(|(fault, _)| fault)?;
            active.admit_member(ingress).map_err(|(fault, _)| fault)?;
        }
        active.seal_members()?;
        Ok(())
    }

    /// 🔀️ Merges a decoded read-back into the live document (design §22.22) in the host's own poll turn and requests the
    /// operation's terminal: `Ready` with what it took (`completed == total`) and is ahead by, or the fault that
    /// refused it. Nothing is replaced.
    async fn merge_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let merged = self.merge_decoded_document_archive(operation, identity, cx).await;
        let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its merge"))?;
        match merged {
            Ok((taken, ahead)) => {
                active.completed = taken;
                active.total = taken;
                active.ahead = ahead;
                active.request_terminal(ActiveDocumentArchiveLoadState::Ready);
            }
            Err(fault) => active.request_fault(&fault),
        }
        Ok(())
    }

    /// 🧬️ The merge itself, answering the events taken and the events this program holds that the archive lacks: the
    /// archive's members must be the live ones (a member merge is the member's own, not yet available), its decoded
    /// log joins the store's in one store call — an archive of another document is refused — and an open history edit
    /// hears the base move like after any remote edit.
    async fn merge_decoded_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64), Fault> {
        let live = self.archive_member_entries().await?;
        let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its merge"))?;
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge input owner is absent"))?;
        if !live.iter().eq(archive.members.iter().rev()) {
            return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, "the archive's members differ from this document's live members; a member merge is not available"));
        }
        let pack = std::mem::take(&mut archive.parent_pack);
        let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive decoded history owner is absent"))?;
        let generation = self.store.generation();
        let merge = match self.store.merge_persisted_history(&pack, history).await {
            Ok(merge) => merge,
            Err(store::VcsError::ValidationFailed(detail)) => return Err(document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail)),
            Err(error) => return Err(error.into_fault()),
        };
        self.cache = None;
        if self.store.generation() != generation {
            self.deliver_base_moved(identity, cx).await?;
        }
        self.follow_derivable_children(identity).await?;
        Ok((u64::try_from(merge.merged).unwrap_or(u64::MAX), u64::try_from(merge.ahead).unwrap_or(u64::MAX)))
    }

    /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
    /// work until maintenance returned them (see `child_follow_awaits_retirement`).
    fn derivable_follow_awaits_retirement(&self) -> bool {
        self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
    }

    /// ♻️ Whether the next `roots` child-content and `members` child-member retirement generations each find a vacant
    /// slot of their fixed registry.
    fn child_retirements_admit(&self, roots: usize, members: usize) -> bool {
        (1..=roots as u64).all(|offset| self.child_content_generation.checked_add(offset).is_some_and(|generation| self.child_content_retirements.can_insert(generation)))
            && (1..=members as u64).all(|offset| self.child_member_retirement_generation.checked_add(offset).is_some_and(|generation| self.child_member_retirements.can_insert(generation)))
    }

    /// ⏳️ Queries the original fixed retirement slots; funded maintenance makes room.
    fn reclaim_child_retirements(&self, roots: usize, members: usize) -> Result<bool, Fault> {
        if roots > ARTIFACT_LIVE_OUTPUT_SLOTS || members > ARTIFACT_LIVE_OUTPUT_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-span"), "one child publication span exceeds the fixed retirement authority"));
        }
        Ok(self.child_retirements_admit(roots, members))
    }

    /// 🎟️ Admits the next immutable child-content publication once its retirement slot is vacant — reclaimed first when
    /// an older retirement still holds it ([`Self::reclaim_child_retirements`]).
    pub(crate) fn admit_child_content_publication(&mut self) -> Result<u64, Fault> {
        self.admit_child_content_publication_span(1)?;
        Ok(self.child_content_generation + 1)
    }

    /// 🎟️ Admits `count` consecutive child-content publications (see [`Self::admit_child_content_publication`]). Only a
    /// ring of retirements that are every one still borrowed answers the retryable
    /// `interactive-job.child-root-retirement-saturated`.
    fn admit_child_content_publication_span(&mut self, count: usize) -> Result<(), Fault> {
        if self.child_content_generation.checked_add(count as u64).is_none() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"));
        }
        if !self.reclaim_child_retirements(count, 0)? {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication waits on retirements that are all still borrowed").with_retryable(true));
        }
        Ok(())
    }

    pub(crate) async fn publish_child_content_member(&mut self, generation: u64, slot: &str, child_id: &str) -> Result<(), Fault> {
        self.publish_member_content(generation, MemberKeyRef::root(slot, child_id)).await
    }

    /// 📰️ Republishes the immutable child-content root with the member at one exact owner edge captured anew.
    async fn publish_member_content(&mut self, generation: u64, key: MemberKeyRef<'_>) -> Result<(), Fault> {
        if generation != self.child_content_generation.saturating_add(1) || !self.child_content_retirements.can_insert(generation) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-publication-authority"), "immutable child-content publication lost its exact admitted generation"));
        }
        let entry = self.children.member(key).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-member"), "immutable child-content publication lost its exact live member"))?;
        let next = self.child_content_root.with_member(key, &entry.reference, &entry.member).await?;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if previous.root.as_ref().is_some_and(|root| root.len != 0) {
            self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = generation;
        Ok(())
    }

    /// 📮️ Sends one member's events on the document's backbone under that member's lane: the owner-path text of the
    /// member that owns it (empty for the document itself) and its step there (design §21.9). A member whose owner is
    /// not live has no lane: its events are refused here, never sent under another lane and never dropped.
    async fn send_member_lane(&mut self, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
        if payload.is_empty() || self.store.backbone_ref().is_none() {
            return Ok(());
        }
        let Some(owner) = self.children.path_of(lane.borrowed()).map(|path| path.owner_text()) else {
            return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
        };
        self.store.send_member_mutations(&owner, &lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
    }

    fn peer_roster_slot(generation: u64) -> usize {
        generation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS
    }

    /// 🧭️ The lowest mounted peer roster publication the ordered cursor has already passed. A
    /// publication whose generation is not ahead of `peer_roster_processed_generation` can never be the
    /// rotation's next exact generation again, so without this the maintenance stage answers
    /// `Pending { 0, 0 }` for it forever: it never reaches its outcome slot, `take_presence_outcome`
    /// never answers, and the app's close conjunction — which demands an empty publication registry —
    /// is unreachable. Walking it through the very same validate/outcome ladder lets
    /// `validate_peer_roster_publication` refuse its generation and hand the refusal to its caller.
    fn orphaned_peer_roster_generation(&self) -> Option<u64> {
        let processed = self.peer_roster_processed_generation;
        let mut orphaned: Option<u64> = None;
        self.peer_roster_publications.each_id(|generation| {
            if generation <= processed && orphaned.is_none_or(|lowest| generation < lowest) {
                orphaned = Some(generation);
            }
        });
        orphaned
    }

    fn validate_peer_roster_publication(&self, seq: u64, generation: u64, cancel: &semio_framework_job::CancelToken) -> Result<ValidatedPeerRosterCommit, Fault> {
        let slot = Self::peer_roster_slot(generation);
        if self.peer_roster_reservations[slot] != Some((generation, seq))
            || generation != self.peer_roster_processed_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster publication generation exhausted"))?
            || !self.peer_presence_retirements.can_insert(generation)
            || !self.presence_peer_retirements.can_insert(generation)
            || !self.peer_roster_outcomes.can_insert(generation)
            || !self.presence_store.peer_retirement_available()
            || self.presence_store.retirement_started()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-publication-authority"), "peer roster lost its exact generation, outcome, or retirement admission before publication"));
        }
        if ::semio_framework_async::poll::resolve_ready(cancel.is_cancelled()) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-cancelled"), "peer roster publication was cancelled before atomic commit"));
        }
        Ok(ValidatedPeerRosterCommit { generation })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
    fn publish_peer_roster_candidate_admitted(&mut self, admission: ValidatedPeerRosterCommit, candidate: PeerRosterCandidate<A::Presence>) -> Result<(), PeerRosterCandidate<A::Presence>> {
        let PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed } = candidate;
        let ValidatedPeerRosterCommit { generation } = admission;
        let _ = now_ms;
        let typed_retirement = match self.presence_store.publish_peer_commit(typed) {
            Ok(retirement) => retirement,
            Err(typed) => return Err(PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed }),
        };
        let previous = std::mem::replace(&mut *self.peer_presence, root);
        if previous.is_empty() {
            drop(previous);
        } else {
            self.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
        }
        self.own_color = own_color;
        self.peer_presence_generation = generation;
        if let Some(retirement) = typed_retirement {
            self.presence_peer_retirements.insert_admitted(generation, retirement);
        }
        Ok(())
    }

    fn declared_child_reference(&self, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {
        let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
        let fields = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.child_id == child_id).map(|(_, fields)| fields).ok_or_else(|| plugin_sdk_fault("child member identity is not declared by the current parent"))?;
        if fields.artifact_kind != dialect.artifact_kind || fields.standard != dialect.standard || fields.subset != dialect.subset {
            return Err(plugin_sdk_fault("child member dialect differs from the declared target"));
        }
        Ok(ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() })
    }

    async fn admit_child_member(&mut self, slot: String, child_id: String, dialect: ArtifactDialect) -> Result<ChildStoreAdmission, Fault> {
        let expected = self.declared_child_reference(&slot, &child_id, &dialect)?;
        let root_index = self.child_content_root.admit_member(MemberKeyRef::root(&slot, &child_id))?;
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("owned member admission requires the parent's exact declared dialect"));
        }
        let generation = self.admit_child_content_publication()?;
        let parent_id = self.store.envelope().id.clone();
        let graph = self.composition.graph_mut().await.admit_owns(&parent_id, &slot, &expected.artifact_id).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: parent_id, dialect: parent_dialect }, slot: slot.clone(), child_id: child_id.clone() };
        let key = MemberKey::root(slot, child_id);
        let member = self.children.admit_identity(key.borrowed())?;
        Ok(ChildStoreAdmission { key, expected, owner, member, graph, root_index, generation })
    }

    async fn prepare_child_member(&self, admission: &ChildStoreAdmission, member: &mut M, restored: bool, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(ChildContentView, Option<usize>), Fault> {
        let owner = member.owner_ref();
        if member.artifact_ref().as_ref() != Some(&admission.expected) || (owner.as_ref() != Some(&admission.owner) && (restored || owner.is_some())) {
            return Err(plugin_sdk_fault("child member does not match its exact admitted identity and ownership"));
        }
        let pin_index = self.pending_child_pins.iter().position(|pin| pin.child_ref.artifact_id == admission.expected.artifact_id);
        if let Some(index) = pin_index {
            if !restored {
                return Err(plugin_sdk_fault("direct child registration cannot consume a queued restore checkpoint"));
            }
            let pin = &self.pending_child_pins[index];
            if pin.child_ref != admission.expected {
                return Err(plugin_sdk_fault("pending child checkpoint belongs to a different exact dialect"));
            }
            let alternative = member.current_alternative_id().await.ok_or_else(|| plugin_sdk_fault("pending child checkpoint has no live alternative authority"))?;
            member.checkout(&pin.checkpoint_id, &alternative, identity).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        }
        let root = self.child_content_root.capture_member_admitted(admission.root_index, admission.key.borrowed(), &admission.expected, member).await?;
        Ok((root, pin_index))
    }

    async fn commit_child_member(&mut self, admission: ChildStoreAdmission, mut member: M, root: ChildContentView, pin_index: Option<usize>) {
        member.set_owner(Some(admission.owner.clone())).await;
        self.composition.graph_mut().await.commit_owns_admitted(admission.graph);
        self.children.insert_admitted(admission.member, admission.expected, admission.owner, member);
        let previous = std::mem::replace(&mut *self.child_content_root, root);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(admission.generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = admission.generation;
        if let Some(index) = pin_index {
            self.pending_child_pins.remove(index);
        }
    }

    #[cfg(test)]
    pub(super) fn test_child_admission_state(&self, abort_generation: u64) -> ChildAdmissionTestState {
        ChildAdmissionTestState {
            children_empty: self.children.is_empty(),
            generation: self.child_content_generation,
            abort_generation: self.child_member_retirement_generation,
            abort_empty: self.child_member_retirements.is_empty(),
            requested_abort_retained: self.child_member_retirements.get(abort_generation).is_some(),
            content_empty: self.child_content_root.is_empty(),
            roots_retiring_empty: self.child_content_retirements.is_empty(),
        }
    }

    /// ♻️ The ONE release site of an admitted typed-operation slot. Every outcome an operation can reach
    /// — settled, faulted, refused, replaced, cancelled — funnels its mounted owner into
    /// [`MountedTypedCommandFullOperationStage::Retiring`], and this is the only code that may remove it
    /// from `tool_operations` and so free its residue class for [`Self::admit_typed_operation_slot`]: one
    /// bounded retirement unit, the terminal-empty witness, then the exact removal. The turn driver
    /// ([`Self::advance_typed_operation_publication_one`]), the cooperative maintenance rotation and the
    /// app close ladder all walk THIS function, so no authority can admit a slot and leave its release to
    /// another (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8).
    /// 🧾️ The migrated twin of [`Self::dispatch_emit`]'s own command-log append. A typed command's
    /// emit reaches its stores through the mounted publication ladder, which never walks
    /// `dispatch_emit`, so before this every migrated dispatch reached the history panel only as
    /// [`Self::backfill_command_log`]'s anonymous `apply` row — losing the verb, its
    /// declared [`ActionKind`], its own label and its revert target — and a dispatch that published
    /// no durable lane at all (a `Mutation` verb whose reducer emitted nothing, every `View` verb
    /// such as `select`) reached it not at all.
    ///
    /// It runs on the durable RECEIPT, not at retirement, because the operation mints its `Ui` page
    /// between the two and rendering the history body backfills: an edit this row owns would already
    /// be filed under `apply` by the time the slot retired. Only the document lane opens or amends a row: a config-lane
    /// edit is never a history row (design §20.13, L4). A coalesced gesture amends the edit its first dispatch already
    /// logged, so later dispatches append no second row — and they mark that row dirty, so the admitting completion
    /// re-upserts it.
    fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) {
        if mounted.terminal_fault.is_some() {
            return;
        }
        let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) else { return };
        let logged_seq = self.command_log.iter().find(|entry| entry.edit_id.as_deref() == Some(edit_id.as_str())).map(|entry| entry.seq);
        if let Some(seq) = logged_seq {
            mounted.command_logged = true;
            self.history_dirty_sequences.insert(seq);
            return;
        }
        if mounted.command_logged {
            if let Some(entry) = self.command_log.last_mut().filter(|entry| entry.action_id == mounted.verb) {
                entry.edit_id = Some(edit_id);
                self.history_dirty_sequences.insert(entry.seq);
                self.log_generation += 1;
            }
            return;
        }
        mounted.command_logged = true;
        let kind = self.typed_operation_command_kind(&mounted.verb, true);
        let verb = mounted.verb.clone();
        let label = self.verb_label(&verb);
        self.record_command(&verb, kind, label, Some(edit_id), Vec::new(), None);
    }

    /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then
    /// delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —
    /// addressed through `meta`'s view when it holds that window, else through the last roster — as an ordinary typed
    /// operation that is never a history row of its own. An event the app ignores, or for a window no view holds,
    /// dispatches nothing.
    async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_window_gesture(&event);
        let Some(command) = A::host_event(&event) else { return Ok(()) };
        let window_id = event.window_id();
        let view = meta.view_state.as_ref().and_then(|view| view.for_window_instance(window_id)).or_else(|| ViewModel { window_instances: self.host_event_roster.clone(), ..meta.view_state.as_ref()?.clone() }.for_window_instance(window_id));
        let Some(view) = view else { return Ok(()) };
        let window_meta = ActionMeta { view_state: Some(view), ..meta.clone() };
        let verb = A::command_id(&command).await;
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(verb, &payload, 1).await?;
        self.delivering_host_event = true;
        let result = Box::pin(self.dispatch_typed_command_inner(Box::new(command), admission, &window_meta, identity, cx)).await;
        self.delivering_host_event = false;
        result.map(|_| ())
    }

    /// 🧰️ Delivers [`HostEvent::UtilityChanged`] for every window whose utility `meta`'s view moved since a dispatch
    /// last showed it — the targeted window reads the per-call overlay, every other one the view's map, and a window
    /// the view names no utility for is unknown, never a switch — and remembers the view's window roster.
    async fn deliver_utility_host_events(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|_| !self.delivering_host_event) else { return Ok(()) };
        if !view.window_instances.is_empty() && self.host_event_roster != view.window_instances {
            self.host_event_roster = view.window_instances.clone();
            self.host_event_utilities.retain(|window_id, _| view.window_instances.iter().any(|window| window.id == *window_id));
        }
        let mut switched = Vec::new();
        for window in &view.window_instances {
            let utility = if view.window_id.as_deref() == Some(window.id.as_str()) { Some(view.active_utility_id.as_deref()) } else { view.active_utility_by_window_id.get(&window.id).map(|utility| Some(utility.as_str())) };
            let Some(utility) = utility else { continue };
            match self.host_event_utilities.get_mut(&window.id) {
                Some(previous) if previous.as_deref() == utility => {}
                Some(previous) => {
                    let from = std::mem::replace(previous, utility.map(str::to_string));
                    switched.push(HostEvent::UtilityChanged { window_id: window.id.clone(), from, to: utility.map(str::to_string) });
                }
                None => {
                    self.host_event_utilities.insert(window.id.clone(), utility.map(str::to_string));
                }
            }
        }
        for event in switched {
            self.deliver_host_event(event, meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers one instance-wide event, built per window, to every window of `meta`'s view, else of the last roster.
    async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_every_gesture(&event(String::new()));
        let roster = meta.view_state.as_ref().map(|view| view.window_instances.as_slice()).filter(|roster| !roster.is_empty()).unwrap_or(&self.host_event_roster);
        let windows: Vec<String> = roster.iter().map(|window| window.id.clone()).collect();
        for window_id in windows {
            self.deliver_host_event(event(window_id), meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers [`HostEvent::BaseMoved`] to every window once a remote edit moved the base, as the local actor.
    async fn deliver_base_moved(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(instance_id) = self.live_runtime_instance_id else { return Ok(()) };
        let meta = ActionMeta { actor: self.store.local_actor_id().0.clone(), instance_id, view_state: None };
        self.deliver_host_event_to_every_window(|window_id| HostEvent::BaseMoved { window_id }, &meta, identity, cx).await
    }

    /// 📨️ `hostEvent{windowId, kind}`: the host-forwarded window fact, delivered at once; an unknown kind or a
    /// missing window is refused.
    async fn dispatch_host_event_action(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str);
        let event = text(semio_framework::HOST_EVENT_ARG_WINDOW_ID).zip(text(semio_framework::HOST_EVENT_ARG_KIND)).and_then(|(window_id, kind)| HostEvent::forwarded(window_id, kind));
        let Some(event) = event else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("hostEvent.invalid"), "hostEvent requires a windowId and a kind of blur, captureLost or retiring"));
        };
        self.deliver_host_event(event, meta, identity, cx).await?;
        let ui_scope = if self.take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };
        Ok(Self::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await)
    }

    /// 🧾️ A settled typed operation that published no document lane and no shared-config lane still
    /// owes the history panel one row, on the admitting completion: `dispatch_emit`'s empty
    /// `artifact_mutations` branch files the verb under its declared kind (`View` for `select`,
    /// `Mutation` for a reducer that emitted nothing, `edit_id: None`). Recording that row only from
    /// retirement lands it after [`Self::take_typed_operation_completion`] has built `history_patch`,
    /// so the row surfaced on the next command. Both the completion take and
    /// [`Self::retire_typed_operation_unit`] call this; `command_logged` makes the second a no-op.
    /// Silent for a faulted operation, exactly as `dispatch_emit`'s `Err` path is, and for a `View`
    /// whose only durable emission is the per-window config lane — an orbit tick is not a command.
    fn record_settled_typed_operation_command(&mut self, operation_id: u64) {
        if self.host_event_operations.remove(&operation_id) {
            if let Some(operation) = self.tool_operations.get_mut(operation_id) {
                operation.command_logged = true;
            }
            return;
        }
        let logged = {
            let Some(operation) = self.tool_operations.get(operation_id) else { return };
            if operation.command_logged || operation.terminal_fault.is_some() || operation.published_artifact || operation.published_config {
                return;
            }
            let kind = self.typed_operation_command_kind(&operation.verb, false);
            if operation.published_window_config && matches!(kind, ActionKind::View) {
                return;
            }
            (operation.verb.clone(), kind)
        };
        let (verb, kind) = logged;
        if let Some(operation) = self.tool_operations.get_mut(operation_id) {
            operation.command_logged = true;
        }
        self.record_command(&verb, kind, None, None, Vec::new(), None);
    }

    /// 🧾️ The declared [`ActionKind`] one migrated verb logs under — the same three-step resolution
    /// [`Self::dispatch_emit`] runs, with the same last resort: an unresolved verb that reached a
    /// document store is a `Mutation`, one that reached nothing is a `View`.
    fn typed_operation_command_kind(&self, verb: &str, artifact_lane: bool) -> ActionKind {
        match self.invocation_kind {
            Some(kind) => kind,
            None => match self.registry.get(verb).map(|definition| definition.kind) {
                Some(kind) => kind,
                None => match self.registry.get_command(verb) {
                    Some(definition) => definition.kind,
                    None if artifact_lane => ActionKind::Mutation,
                    None => ActionKind::View,
                },
            },
        }
    }

    /// 📏️ Quotes the next retiring typed-operation turn: its private child group's close, one owner hand-off, or the operation's own retirement.
    pub(crate) fn typed_operation_retirement_demands(&self, operation_id: u64, body: usize) -> Result<RetirementDemand, Fault> {
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed before its retirement quote"))?;
        if operation.close_fault_pending(){return operation.close_fault_demands(body).map_err(plugin_retirement_fault);}
        if let Some(group) = operation.owned_child_group {
            if self.private_child_groups.get(group).is_some() { return self.private_child_group_operation_close_demands(group, body).map_err(plugin_retirement_fault); }
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if operation.lease_retirement_ready(){return operation.lease_retirement_demands(&self.tool_cancellations,body).map_err(plugin_retirement_fault);}
        operation.retirement_demands(body).map_err(plugin_retirement_fault)
    }

    #[inline(never)]
    pub(crate) fn retire_typed_operation_unit(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=self.typed_operation_retirement_demands(operation_id,grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|self.retire_typed_operation_granted(operation_id,same,actual).map(|step|(step,false)))
    }

    fn retire_typed_operation_granted(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        self.request_gesture_cancellation(operation_id);
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::close_fault_pending){return self.tool_operations.get_mut(operation_id).unwrap().close_fault_step(grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(group) = self.tool_operations.get(operation_id).and_then(|operation| operation.owned_child_group) {
            if self.private_child_groups.get(group).is_some() {
                return Ok(match self.close_private_child_group_operation_step(group,cx,grant)? {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::InteractiveJobCloseStep::Blocked => PluginLifecycleStep::Blocked { reason: "private child group close awaits its owner" },
                });
            }
            if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())); }
            self.tool_operations.get_mut(operation_id).unwrap().owned_child_group = None;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::lease_retirement_ready){let step=self.tool_operations.get_mut(operation_id).unwrap().lease_retirement_step(&self.tool_cancellations,grant).map_err(plugin_retirement_fault)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        self.record_settled_typed_operation_command(operation_id);
        let operation = self
            .tool_operations
            .get_mut(operation_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed during one bounded retirement unit"))?;
        let step = operation.retirement_step(cx)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::terminal_is_empty) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-terminal"), "typed operation reported Complete without exact terminal emptiness").into());
        }
        let operation =
            self.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "terminal typed operation changed before exact removal"))?;
        self.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    /// ♻️ Drives ONE retiring typed operation for a bounded slice of this turn, the exact twin of the
    /// slice [`Self::drive_typed_operation_worker`] gives a Worker-stage operation — same ladder, same
    /// owner, same `INTERACTIVE_TURN_WORKER_PUMPS`/`INTERACTIVE_TURN_WORKER_WALL_US` bound. Without it a
    /// `Retiring` operation answered `has_runnable_typed_operations` forever while the turn driver advanced nothing
    /// for it: the slot was released only by `maintenance_step` stage 0 of [`MAINTENANCE_STAGES`], one
    /// unit per rotation on the shared live-cleanup job, so a sustained action rate filled all
    /// [`ARTIFACT_LIVE_OUTPUT_SLOTS`] classes and every later action was refused
    /// `interactive-job.typed-operation-capacity`.
    fn retire_typed_operation_run(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let started_us=cx.now_us();
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            if !self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
                return Ok(());
            }
            if self.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
                let grant=crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                let demand=self.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
                let progress=step.progress();if progress==RetainedCloneProgress::default(){return Ok(());}
                let owner=self.tool_operations.get_mut(operation_id).unwrap();assert!(owner.original_retirement_receipt.is_none(),"original mounted Close receipt cannot be overwritten before acknowledgement");owner.original_retirement_receipt=Some((grant,progress));return Ok(());

            }
            let grant = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
            let demand = self.typed_operation_retirement_demands(operation_id, grant.maximum_copy_bytes)?;
            if !plugin_grant_funds(demand, grant) {
                return Ok(());
            }
            match cx.with_retained_loan(grant,|same|self.retire_typed_operation_unit(operation_id,same).map_err(Into::into))? {
                PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. } => return Ok(()),
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => {
                    if !progress.fits(grant) {
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"typed operation retirement exceeded its issuer grant").with_retained_progress(progress).into());
                    }
                    if progress == RetainedCloneProgress::default() {
                        return Ok(());
                    }
                }
            }
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 📐️ Quotes the original registry and its real child snapshot owners; an empty registry owes only its backing, and only once the close has detached the child root that still inserts its retirement.
    fn child_root_retirement_quote(&self, body: usize, closing: bool) -> Result<Option<RetirementDemand>, ValueError> {
        original_child_root_retirement_demands(&self.child_content_retirements, &self.children, Some(&self.child_member_retirements), self.maintenance_child_root_cursor, body, closing)
    }

    /// 📏️ Quotes the next child-root retirement turn; nothing is owed once the registry and its backing are gone.
    fn child_root_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_root_retirement_quote(body, self.close_child_root_detached)?.unwrap_or_default())
    }

    /// 🎟️ Advances one child-root retirement under the caller's unchanged grant.
    fn child_root_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_root_retirement_unit(grant, self.close_child_root_detached).map(|step| step.progress())
    }

    /// 🌳️ Supplies the exact live and saved previews to the common original child disposer.
    fn child_root_retirement_unit(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneStep, Fault> {
        let member = self.time_travel.member.as_ref().and_then(|member| member.children.as_ref());
        let review = self.time_travel.member.as_ref().and_then(|member| member.review_children.as_ref().map(|review|&review.view));
        advance_original_child_root_retirement(&mut self.child_content_retirements, &mut self.children, Some(&mut self.child_member_retirements), &self.child_content_root, member, review, &mut self.maintenance_child_root_cursor, grant, closing)
    }

    /// 📏️ Quotes the next child-member retirement turn; nothing is owed once the registry and its backing are gone.
    fn child_member_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_member_retirement_quote(body, self.close_child_root_detached)?.unwrap_or_default())
    }

    /// 🎟️ Advances one child-member retirement under the caller's unchanged grant.
    fn child_member_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_member_retirement_unit(grant, self.close_child_root_detached).map(|step| step.progress())
    }

    fn child_member_retirement_quote(&self,body:usize,closing:bool)->Result<Option<RetirementDemand>,ValueError>{
        let Some((_,generation))=self.child_member_retirements.next_id_from(self.child_member_retirement_cursor)else{let release_bytes=self.child_member_retirements.empty_backing_byte_demand().unwrap_or(0);return Ok((closing&&release_bytes!=0).then_some(RetirementDemand{copy_bytes:size_of_val(&self.child_member_retirements),release_bytes,depth:1,..Default::default()}))};let retirement=self.child_member_retirements.get(generation).unwrap();let parent_copy=size_of::<usize>();
        if retirement.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:parent_copy+size_of::<Option<(u64,ChildMemberRetirement<M>)>>(),depth:1,..Default::default()}))}
        ChildContentRetirement::parent_snapshot_demand(retirement.retirement_demands(body.saturating_sub(parent_copy))?,parent_copy).map(Some)
    }

    fn child_member_retirement_unit(&mut self,grant:RetainedCloneGrant,closing:bool)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let Some(demand)=self.child_member_retirement_quote(grant.maximum_copy_bytes,closing).map_err(|error|plugin_sdk_fault(error.into_message()))?else{return Ok(RetainedCloneStep::Progress(empty))};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let Some((index, generation)) = self.child_member_retirements.next_id_from(self.child_member_retirement_cursor) else {
            let backing=std::mem::take(&mut self.child_member_retirements.slots);self.child_member_retirements.allocation_admitted=false;drop(backing);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));
        };
        let retirement = self.child_member_retirements.get_mut(generation).expect("exact child-member retirement remains admitted");
        if retirement.entry.as_ref().is_some_and(|entry| !entry.member.snapshot_read_leases_terminal_is_empty()) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if retirement.terminal_is_empty(){drop(self.child_member_retirements.remove(generation));self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=retirement.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,retirement.terminal_is_empty(),"original child member registry").map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let mut receipt=step.progress();if receipt!=empty{self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original child registry cursor preflight")}
        let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original child member registry parent").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    fn validate_parent_child_restore(&self, slot: &str, child_id: &str, expected: &ArtifactRef) -> Result<(), Fault> {
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("child restore requires the parent's exact declared dialect"));
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: parent_dialect };
        let source = DocumentClosureSourceView::new(self.store.generation_now(), &parent, self.store.snapshot_ref(), &self.children);
        let projection = store::OwnedDocumentClosureSource::child_projection(&source, None).map_err(|error| plugin_sdk_fault(format!("child restore projection failed: {error:?}")))?;
        if !(0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot && fields.child_id == child_id && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.dialect.artifact_kind && fields.standard == expected.dialect.standard && fields.subset == expected.dialect.subset) {
            return Err(plugin_sdk_fault("child restore is not declared by the loaded parent snapshot"));
        }
        Ok(())
    }

    /// 🌱️ Restores one exact owned member and publishes its graph, map and snapshot together.
    pub async fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let slot = slot.into();
        let child_id = child_id.into();
        let expected = self.declared_child_reference(&slot, &child_id, &dialect)?;
        self.validate_parent_child_restore(&slot, &child_id, &expected)?;
        let parent_generation = self.store.generation_now();
        let abort_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("failed child retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("failed child retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let admission = self.admit_child_member(slot, child_id, dialect).await?;
        let mut member = match M::open(&admission.expected, Some(&admission.owner), envelope_pack, self.store.local_actor_id().clone()).await {
            Ok(member) => member,
            Err(error) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child open retains its exact admission until cancellation");
                return Err(plugin_sdk_fault(error.to_string()));
            }
        };
        let prepared = if self.store.generation_now() != parent_generation {
            Err(plugin_sdk_fault("loaded parent changed during child restore"))
        } else {
            match self.validate_parent_child_restore(&admission.key.slot, &admission.key.child_id, &admission.expected) {
                Ok(()) => self.prepare_child_member(&admission, &mut member, true, identity).await,
                Err(fault) => Err(fault),
            }
        };
        let (root, pin_index) = match prepared {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                self.child_member_retirements.insert_admitted(abort_generation, ChildMemberRetirement::new(ChildMemberEntry { reference: admission.expected, owner: admission.owner, member }));
                self.child_member_retirement_generation = abort_generation;
                return Err(fault);
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of
    /// `open_child`/`register_child`.
    pub async fn child_store(&self, slot: &str, child_id: &str) -> Option<&M> {
        self.children.get(&(slot.to_string(), child_id.to_string())).map(|entry| &entry.member)
    }

    /// 📣️ Announces every live composed member's whole event log on the PARENT's backbone
    /// under that member's own `(slot, child_id)` lane. A composed document owns exactly one
    /// replica endpoint — `ArtifactStore::announce_history` covers the parent lane alone, so
    /// without this a joining replica never learns a single child-lane edit and the two replicas
    /// diverge on the very content a composing plugin keeps in its children (measured on
    /// `🌊️flow`'s `content` child, ticket 26/09/19 `📓️flow.md` §5.3).
    async fn announce_member_event_logs(&mut self) -> Result<(), Fault> {
        if self.children.is_empty() {
            return Ok(());
        }
        let lanes: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
        for lane in lanes {
            let Some(entry) = self.children.member_mut(lane.borrowed()) else { continue };
            let payload = SpaceMember::event_log_payload(&entry.member).await.map_err(|error| error.into_fault())?;
            self.send_member_lane(&lane, payload).await?;
        }
        Ok(())
    }

    /// 📤️ Announces the tail edit of every member a composite gesture just touched, under
    /// that member's own lane. Called from the ONE choke point child edits land at
    /// (`dispatch_emit_group`), so a child-lane edit travels exactly like a parent-lane one.
    async fn announce_member_tail_edits(&mut self, lanes: &[(String, String)]) -> Result<(), Fault> {
        if self.store.backbone_ref().is_none() {
            return Ok(());
        }
        for (slot, child_id) in lanes {
            let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else { continue };
            let payload = SpaceMember::announce_tail_edit_payload(&mut entry.member).await.map_err(|error| error.into_fault())?;
            self.store.send_member_mutations("", slot, child_id, payload).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// 📥️ Routes every member-addressed backbone message the parent store pumped into the
    /// live member that owns that exact lane, and answers how many lanes folded something. Runs after the
    /// derivable children followed the freshly folded parent, so a lane for the child a parent op just named
    /// finds its member. A lane naming a member the CURRENT parent declares but this replica does not hold is
    /// a fault, never a silent drop: the two replicas would otherwise disagree about the document's own
    /// composition. A lane naming a child the parent no longer declares is a superseded content address (a
    /// peer's lane for the child its own later edit — or ours — replaced), and a lane naming a child the
    /// follow pass has not opened yet because it waits on retirements is the child that pass derives: both
    /// carry nothing the composition does not already hold and are skipped.
    async fn fold_member_inbound(&mut self) -> Result<usize, Fault> {
        let inbound = self.store.take_member_inbound();
        if inbound.is_empty() {
            return Ok(0);
        }
        let mut folded = 0usize;
        for (owner, slot, child_id, envelopes) in inbound {
            let lane = MemberPath::under(&owner, slot.as_str(), child_id.as_str()).and_then(|path| self.children.resolve(&path).map(|(key, _)| key));
            let entry = match lane.as_ref() {
                Some(key) => self.children.member_mut(key.borrowed()),
                None => None,
            };
            let Some(entry) = entry else {
                let declared = if owner.is_empty() {
                    let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                    (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                } else {
                    match MemberPath::parse(&owner).and_then(|path| self.children.resolve(&path)) {
                        Some((_, owning)) => {
                            let projection = owning.member.child_restore_projection().map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                            (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                        }
                        None => false,
                    }
                };
                if declared && !self.derivable_follow_awaits_retirement() {
                    let lane = if owner.is_empty() { format!("{slot}/{child_id}") } else { format!("{owner}/{slot}/{child_id}") };
                    return Err(plugin_sdk_fault(format!("backbone member lane {lane} names no live composed member of this replica")));
                }
                continue;
            };
            SpaceMember::ingest_remote_payload(&mut entry.member, &envelopes).await.map_err(|error| error.into_fault())?;
            folded += 1;
        }
        self.cache = None;
        Ok(folded)
    }

    /// 🌱️ Adopts an already-live `M` member into the child-store map directly — the
    /// counterpart `dispatch_emit_group` uses to absorb a `store::os_store::composition_coordinator::GroupReceipt::created_children`
    /// entry (a `ChildGenesis`-minted member has no prior caller-held reference an `open_child`
    /// call could have produced it from) and the general escape hatch for a caller that already
    /// holds a constructed member from elsewhere (e.g. a host-level `SpaceHost`).
    /// 🌱️ Adopts an already-built child store under `(slot, child_id)`. Like [`Self::open_child`]
    /// it MUST seed the ownership edge into `self.composition`'s graph: `dispatch_group`'s phase 1
    /// ownership check (`CompositionGraph::owner_of`) is fail-closed, so a child present in
    /// `self.children` but absent from the graph is rejected as an `OwnershipViolation` the first
    /// time anything dispatches against it. Registering in one place and validating from the other
    /// is precisely the inconsistency this seeds away.
    pub async fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), ChildMemberRegistrationError<M>> {
        let admission = match self.admit_child_member(slot.into(), child_id.into(), dialect).await {
            Ok(admission) => admission,
            Err(fault) => return Err(ChildMemberRegistrationError { member: Some(member), fault }),
        };
        let (root, pin_index) = match self.prepare_child_member(&admission, &mut member, false, identity).await {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                return Err(ChildMemberRegistrationError { member: Some(member), fault });
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 📌️ Leaves-first half of a composing document's checkpoint: commits every DIRTY
    /// child (a clean child is already pinned at its current checkpoint, and committing it again
    /// would mint an empty checkpoint per parent commit) and returns the `CompositionPin`s the
    /// parent's own about-to-be-created checkpoint must carry.
    ///
    /// A child with no checkpoint at all after committing contributes no pin rather than
    /// aborting the parent's checkpoint: a pin that named nothing would be worse than an absent
    /// one, and the parent's history is still perfectly valid without it.
    /// 📌️ One checkpoint-cascade unit: commits the child at `slot`/`child_id` when it is dirty,
    /// republishes its content root, and answers the pin its current checkpoint contributes.
    async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], key: MemberKeyRef<'_>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<vcs::CompositionPin>, Fault> {
        let publication_generation = self.admit_child_content_publication()?;
        let (reference, checkpoint_id) = {
            let entry = self.children.member_mut(key).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
            if entry.member.is_dirty().await {
                entry.member.commit_checkpoint(message.to_string(), authors.to_vec(), identity).await.map_err(|error| error.into_fault())?;
            }
            (entry.reference.clone(), entry.member.current_checkpoint_id().await)
        };
        self.publish_member_content(publication_generation, key).await?;
        Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: reference, checkpoint_id }))
    }

    /// 📌️ Records `pins` on the checkpoint the parent's dispatch just created. Runs AFTER
    /// that dispatch because the checkpoint does not exist until then, and `composition_pins`
    /// participates in `content_addressed_checkpoint_id`, so the pins must be present on the
    /// envelope that gets persisted.
    async fn stamp_checkpoint_composition_pins(&mut self, pins: Vec<vcs::CompositionPin>) -> Result<(), Fault> {
        if pins.is_empty() {
            return Ok(());
        }
        if let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) {
            self.store.set_checkpoint_composition_pins(&checkpoint_id, pins).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// ⏮️ Checkout half of the cascade: the pins the parent's now-current checkpoint recorded,
    /// each restored by one [`Self::checkout_child_unit`]. A pin naming a child that is not currently
    /// open is QUEUED (`pending_child_pins`) rather than dropped, so a child adopted later still lands
    /// on its pinned state instead of silently staying at head — see `open_child`.
    fn checkout_cascade_pins(&mut self) -> Result<Vec<vcs::CompositionPin>, Fault> {
        let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) else { return Ok(Vec::new()) };
        let Some(pins) = self.store.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| checkpoint.composition_pins.clone()) else { return Ok(Vec::new()) };
        self.pending_child_pins.clear();
        self.admit_child_content_publication_span(pins.len())?;
        Ok(pins)
    }

    /// ⏮️ One checkout-cascade unit: restores one live child to `pin`, or queues the pin.
    async fn checkout_child_unit(&mut self, pin: &vcs::CompositionPin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let child_key = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == pin.child_ref.artifact_id).map(|(key, _)| key.to_key());
        let Some(key) = child_key else {
            self.pending_child_pins.push(pin.clone());
            return Ok(());
        };
        let publication_generation = self.admit_child_content_publication()?;
        let entry = self.children.member_mut(key.borrowed()).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
        let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
        let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id, identity).await;
        self.publish_member_content(publication_generation, key.borrowed()).await
    }

    #[cfg(test)]
    pub(crate) async fn test_snapshot(&self) -> A::Snapshot {
        self.store.snapshot().expect("materialize snapshot")
    }

    /// 🧪️ The document store itself — needed to assert on checkpoint metadata
    /// (`composition_pins`), which no snapshot-level accessor exposes.
    #[cfg(test)]
    pub(crate) async fn test_store(&self) -> &ArtifactStore<A::Snapshot, A::Mutation> {
        &self.store
    }

    /// ⚔️ Test-only mutable store access for conflict lifecycle fixtures.
    #[cfg(test)]
    pub(crate) async fn test_store_mut(&mut self) -> &mut ArtifactStore<A::Snapshot, A::Mutation> {
        &mut self.store
    }

    /// 🧹️ Test-only probe: whether a cancelled segmented output still waits in bounded cleanup.
    #[cfg(test)]
    pub(crate) fn test_segmented_closure_contains(&self, operation_id: u64) -> bool {
        self.segmented_closures.contains(operation_id)
    }

    /// 🧪️ The config-store twin of `test_snapshot`.
    #[cfg(test)]
    pub(crate) async fn test_config(&self) -> A::Config {
        self.config_store.snapshot().expect("materialize config snapshot")
    }

    /// 🧪️ Direct access to the wrapped app — used to assert on app-private test fixtures (e.g.
    /// `TestApp::received_actions`) that a framework-owned interception must never populate.
    #[cfg(test)]
    pub(crate) async fn test_app(&self) -> &A {
        &self.app
    }

    /// 🧪️ Refreshes (backfilling the command log) and returns the current `HistoryView` —
    /// the merged command+operation timeline test harness accessor.
    #[cfg(test)]
    pub(crate) async fn test_history(&mut self) -> HistoryView {
        self.refresh_cache().await.expect("refresh cache");
        self.build_history_view(None).await
    }

    /// 🧪️ Lands one app `Emit` through the production `dispatch_emit` seam under a named
    /// verb — the harness for laws about what the reactor does with an emit's effect list before
    /// the host sees it (ticket 26/09/16/INPUT-CAUSALITY-LEDGER: the inline interaction fold),
    /// which no registry-less typed command can reach and no reserved route carries.
    #[cfg(test)]
    pub(crate) async fn test_dispatch_emit(&mut self, verb: &str, emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        self.dispatch_emit(verb, emit, meta, identity, cx).await
    }

    /// 🧪️ The `KernelMutation`/`UndoGroup` half of an `InvocationResult` as it stands NOW,
    /// read from the store's own last edit. A migrated dispatch returns an admission receipt whose
    /// `mutations` are empty by construction — the work is handed to a worker and the document only
    /// advances at `settle_registered_typed_operation` — so a settling fixture rebuilds this half
    /// afterwards through the very same `result_from_last_edit` the unmigrated route uses, instead of
    /// every law reaching into the store by hand.
    #[cfg(test)]
    pub(crate) async fn test_result_from_last_edit(&self, verb: &str, meta: &ActionMeta, tail_offset: (usize, usize)) -> InvocationResult {
        self.result_from_last_edit(verb, meta, Vec::new(), Vec::new(), UiDirtyScope::default(), tail_offset).await
    }

    /// 🧪️ `(forwards, inverse)` lengths of the store's last edit — the `before` half of
    /// `dispatch_emit`'s own `tail_offset`, so a settling fixture can report only the operations ONE
    /// dispatch added to a coalesced edit.
    #[cfg(test)]
    pub(crate) fn test_edit_tail_lengths(&self) -> (usize, usize) {
        self.store.edit_mutations().map_or((0, 0), |(forwards, inverse, _)| (forwards.len(), inverse.len()))
    }

    /// 🧪️ The store's last edit id, or `None` on a store that has never committed one —
    /// `dispatch_emit`'s `amended_same_edit` identity test, for a settling fixture.
    #[cfg(test)]
    pub(crate) fn test_last_edit_id(&self) -> Option<String> {
        self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone())
    }

    /// 🧪️ The persisted-plus-leftover selection half the panels and every app read —
    /// what a folded interaction verb must have moved.
    #[cfg(test)]
    pub(crate) fn test_interaction_selection_snapshot(&self) -> protocol::InteractionState {
        self.interaction_selection_snapshot()
    }

    /// 📸️ Materializes and returns the current snapshot — the typed counterpart to
    /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
    /// need direct structural access to document state instead of a rendered node.
    pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
        self.store.snapshot().map_err(|error| error.into_fault())
    }

    /// 🧪️ Returns the fixed store revision that a host render or admitted command observes.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_document_revision(&self) -> [u8; 32] {
        self.store.content_revision_now()
    }

    /// 🔗️ The store's current backbone descriptor, `None` when unattached (the default).
    pub fn backbone_ref(&self) -> Option<&store::os_store::store_schema::ArtifactBackboneRef> {
        self.store.backbone_ref()
    }

    /// 🧾️ Appends one entry to the session command log. `timestamp: None` stamps "now"
    /// (live dispatch); `Some(..)` preserves an edit's original `started_at` (backfill). Always a
    /// fresh row (`count: 1`) — folding consecutive `View`/`🐚️Shell` dispatches is `record_command`'s job.
    pub(crate) fn push_log_entry(&mut self, append: CommandLogAppend<'_>) {
        let CommandLogAppend { action_id, label, kind, edit_id, transition_id, timestamp, inverse } = append;
        self.next_command_seq += 1;
        let mut record = Some(CommandLogEntry {
            seq: self.next_command_seq,
            action_id: action_id.to_string(),
            label,
            kind,
            timestamp: timestamp.unwrap_or_else(store::os_store::document_materialize::now_iso),
            edit_id,
            child_edit_ids: Vec::new(),
            transition_id,
            count: 1,
            inverse,
        });
        let mut page = self.command_log.prepare_append().expect("live command history admits its exact next record");
        page.admit_step(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: page.next_capacity_byte_demand(), maximum_copy_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 });
        self.command_log.commit_append(&mut page, &mut record).expect("preborn command page retains its original live prefix");
        self.history_dirty_sequences.insert(self.next_command_seq);
        self.log_generation += 1;
    }

    /// 🧾️ The single entry point every live dispatch logs through (`push_log_entry` remains for
    /// backfill, which never folds). Consecutive `View`/`🐚️Shell` dispatches of the SAME `(action_id,
    /// kind)` with no `edit_id` fold into one row — its `count` increments and its `label`/`timestamp`
    /// refresh, but its ORIGINAL `seq` is kept so the panel's tree-item id stays stable across
    /// re-renders. `Mutation`/`History`/`Clipboard` entries and anything with an `edit_id` never fold.
    /// `inverse` (computed from state BEFORE this dispatch) is only stored on a FRESH row — a folded
    /// row keeps its original inverse, since inverse on a folded "×N" row must undo the whole run,
    /// not just the last dispatch that folded into it.
    ///
    /// 🙈️ Interaction verbs (hover, pick, clear selection) and pure view verbs (a `View` that edited no store and
    /// carries no inverse) are never rows: they change no history and undo cannot reach them
    /// ([`history_row_is_recorded`]).
    ///
    /// 🌐️ The row keeps the action's FULL locale matrix — no resolve happens here, so the
    /// History panel renders every row in whatever locale the shell is showing right now.
    /// A row with no declaring definition falls back to its `action_id`, which is
    /// locale-invariant data, not untranslated English.
    pub(crate) fn record_command(&mut self, action_id: &str, kind: ActionKind, label: Option<LocalizedLabel>, edit_id: Option<String>, child_edit_ids: Vec<String>, inverse: Option<InverseAction>) {
        if !history_row_is_recorded(kind, edit_id.is_some() || !child_edit_ids.is_empty(), inverse.is_some()) {
            return;
        }
        let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
        self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, transition_id: None, timestamp: None, inverse });
        if let Some(last) = self.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }
        if !matches!(kind, ActionKind::History) {
            self.shell_redo.clear();
        }
    }

    /// 🏷️ The registry label (every locale) of the action or command `verb` names — how a persisted
    /// [`protocol::Edit::verb`] becomes a history label at projection time; `None` for a verb this app does not declare.
    pub(crate) fn verb_label(&self, verb: &str) -> Option<LocalizedLabel> {
        self.registry.get(verb).map(|definition| definition.label.clone()).or_else(|| self.registry.get_command(verb).map(|definition| definition.label.clone())).or_else(|| runtime_verb_label(verb))
    }

    /// 🕰️ Appends a command-log entry for every VCS edit not yet referenced by the log —
    /// covers seeded (`app.seed`), ingested (`ingest_operations*`), and loaded
    /// (document archive load) edits that never passed through `dispatch_emit`.
    /// Invariant: after this runs, every `envelope.vcs.edits` entry is referenced by exactly one
    /// `CommandLogEntry`. Idempotent — re-running finds nothing missing. Always `push_log_entry`
    /// (never `record_command`) — a backfilled edit is always its own distinct row, never folded.
    ///
    /// 🌉️ Hoisted out of a `.map()` closure into an explicit loop (sync — `Iterator::map`
    /// cannot take an async closure, and `OpText::print_op` is genuinely async) — see
    /// `build_history_view`'s sibling comment for the same pattern.
    ///
    /// 🧮️ Same backfill, for the CONFIG store's own edits — a config edit reached via `seed`/
    /// `load_config_pack`/ingest never passes through `dispatch_emit` either.
    ///
    /// 🧾️ O(change) (design §20.14): past a [`HistoryBackfillMark`] whose ledger prefixes still stand, only the edits, member
    /// edits and supersede records that came after it are read, against the rows logged since (the then-newest row
    /// included — a typed operation attaches its edit to it); anything else backfills over the whole history.
    async fn backfill_command_log(&mut self) {
        self.refresh_supersede_ledger();
        let (ledgers, members_held) = self.member_ledger_tails(self.history_backfill.as_ref().map(|mark| mark.members.as_slice()));
        let edits = &self.store.envelope().vcs.edits;
        let scope = self.history_backfill.as_ref().filter(|mark| {
            let prefix = mark.edits <= edits.len() && (mark.edits == 0 || edits.iter().rev().nth(edits.len() - mark.edits).map(|edit| &edit.id) == mark.last_edit.as_ref());
            prefix && mark.supersede_records == self.supersedes.records.len() && members_held && ledgers.iter().all(|(_, tail)| tail.fresh.is_some())
        });
        let (log_from, edits_from, records_from) = scope.map_or((0, 0, 0), |mark| (self.command_log.len() - self.command_log.iter().rev().take_while(|entry| entry.seq >= mark.log_seq).count(), mark.edits, mark.supersede_records));
        let wanted_members: Option<HashSet<&str>> = scope.map(|_| ledgers.iter().flat_map(|(_, tail)| tail.fresh.iter().flatten().map(String::as_str)).collect());
        let rows = || self.command_log.iter().rev().take(self.command_log.len() - log_from).rev();
        let logged: HashSet<&str> = rows().filter_map(|entry| entry.edit_id.as_deref()).collect();
        let logged_transitions: HashSet<&str> = rows().filter_map(|entry| entry.transition_id.as_deref()).collect();
        let mut transitions = self.supersedes.records.iter().enumerate().skip(records_from).filter(|(_, record)| !logged_transitions.contains(record.transition_id.as_str())).map(|(index, record)| (index, record.timestamp)).peekable();
        let logged_children: HashSet<&str> = rows().flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
        let fresh_edits: Vec<&protocol::Edit<A::Mutation>> = {
            let mut fresh: Vec<&protocol::Edit<A::Mutation>> = edits.iter().rev().take(edits.len() - edits_from).filter(|edit| !logged.contains(edit.id.as_str())).collect();
            fresh.reverse();
            fresh
        };
        let parent_transactions: HashMap<String, String> = fresh_edits.iter().filter_map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.as_ref()).map(|transaction| (transaction.id.clone(), edit.id.clone()))).collect();
        let histories = self.member_edit_histories(&HashMap::new(), wanted_members.as_ref());
        let time_travel::MemberBackfill { mut attached, groups } = time_travel::member_backfill(histories.into_values(), &logged_children, &parent_transactions);
        let mut members = groups.into_iter().peekable();
        let mut missing: Vec<(CommandLogAppendKey, LocalizedLabel, String)> = Vec::new();
        let mark = HistoryBackfillMark {
            edits: edits.len(),
            last_edit: edits.last().map(|edit| edit.id.clone()),
            members: ledgers.iter().map(|(member, tail)| (member.clone(), tail.len, tail.last.clone())).collect(),
            supersede_records: self.supersedes.records.len(),
            log_seq: 0,
        };
        for edit in fresh_edits {
            let at = edit.mutation_meta.first().map(|meta| meta.timestamp);
            loop {
                let transition_at = transitions.peek().map(|(_, timestamp)| *timestamp).filter(|timestamp| at.is_some_and(|at| *timestamp < at));
                let member_at = members.peek().filter(|group| at.is_some_and(|at| group.at.is_some_and(|member| member < at))).and_then(|group| group.at);
                match (transition_at, member_at) {
                    (Some(transition), Some(member)) if member < transition => {
                        let group = members.next().expect("peeked member group");
                        missing.push((CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at));
                    }
                    (Some(_), _) => {
                        let (index, _) = transitions.next().expect("peeked transition");
                        let record = &self.supersedes.records[index];
                        missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
                    }
                    (None, Some(_)) => {
                        let group = members.next().expect("peeked member group");
                        missing.push((CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at));
                    }
                    (None, None) => break,
                }
            }
            let label = backfilled_edit_label(edit, edit.verb.as_deref().and_then(|verb| self.verb_label(verb)), |op| protocol::SemanticMutation::<A::Snapshot>::label(op));
            missing.push((CommandLogAppendKey::Edit { edit_id: edit.id.clone(), verb: edit.verb.clone(), child_edit_ids: attached.remove(&edit.id).unwrap_or_default() }, label, edit.started_at.clone()));
        }
        for (index, _) in transitions {
            let record = &self.supersedes.records[index];
            missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
        }
        missing.extend(members.map(|group| (CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at)));
        for (key, label, timestamp) in missing {
            match key {
                CommandLogAppendKey::Edit { edit_id, verb, child_edit_ids } => {
                    self.push_log_entry(CommandLogAppend { action_id: verb.as_deref().unwrap_or("apply"), label, kind: ActionKind::Mutation, edit_id: Some(edit_id), transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.command_log.last_mut() {
                        last.child_edit_ids = child_edit_ids;
                    }
                }
                CommandLogAppendKey::Members { edit_ids } => {
                    self.push_log_entry(CommandLogAppend { action_id: "apply", label, kind: ActionKind::Mutation, edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.command_log.last_mut() {
                        last.child_edit_ids = edit_ids;
                    }
                }
                CommandLogAppendKey::Transition(transition_id) => self.push_log_entry(CommandLogAppend {
                    action_id: semio_framework::HISTORY_EDIT_COMMIT_ACTION_ID,
                    label,
                    kind: ActionKind::History,
                    edit_id: None,
                    transition_id: Some(transition_id),
                    timestamp: Some(timestamp),
                    inverse: None,
                }),
            }
        }
        self.history_backfill = Some(HistoryBackfillMark { log_seq: self.command_log.iter().next_back().map_or(0, |entry| entry.seq), ..mark });
    }

    /// 🧩️ Every live child's TAIL applied edit id, plus whether any child has an edit on
    /// its redo stack — the composed-artifact halves of `can_undo`/`can_redo`/`revertible`. A
    /// `Child`-lane document verb (see `ArtifactToolPublicationLane::Child`) never touches the
    /// parent store at all, so without this the shell disables its own undo control over a
    /// perfectly undoable ledger row. Empty and free for every leaf app: `entries()` yields
    /// nothing when the registry holds no child.
    async fn child_history_tails(&self) -> (HashSet<String>, bool) {
        let mut applied_tails = HashSet::new();
        let mut has_redo_tail = false;
        for entry in self.children.entries() {
            if let Some(edit_id) = entry.member.tail_edit_id().await {
                let _ = applied_tails.insert(edit_id);
            }
            if !has_redo_tail {
                has_redo_tail = entry.member.redo_tail().await.is_some();
            }
        }
        (applied_tails, has_redo_tail)
    }

    /// 🧩️↩️ The composite group id an `undo`/`redo` must target when the PARENT store never
    /// moved. `commit_framework_history_route` reads `self.store.tail_group_id()` first, which is
    /// `None` for a gesture that published only on the `Child` lane; every such gesture still
    /// stamped its `invocation_id` onto each touched child (`CompositionCoordinator::dispatch_group`
    /// phase 2), so the group is recoverable from the children's own tails. Ordered by the
    /// append-only command log rather than by any member's clock: for `undo` the LAST row naming a
    /// live child tail is the most recent still-applied group, and for `redo` the FIRST row naming
    /// a child redo-tail is the one a single `redo` reapplies. A child whose tail carries no group
    /// id is not a composite gesture and is never a target.
    async fn child_group_history_target(&self, action: &str) -> Option<String> {
        if self.children.is_empty() {
            return None;
        }
        let mut candidates: Vec<(String, String)> = Vec::new();
        for entry in self.children.entries() {
            let candidate = if action == "undo" {
                match (entry.member.tail_edit_id().await, entry.member.tail_group_id().await) {
                    (Some(edit_id), Some(group_id)) => Some((edit_id, group_id)),
                    _ => None,
                }
            } else {
                match entry.member.redo_tail().await {
                    Some((edit_id, Some(group_id))) => Some((edit_id, group_id)),
                    _ => None,
                }
            };
            if let Some(candidate) = candidate {
                candidates.push(candidate);
            }
        }
        if candidates.is_empty() {
            return None;
        }
        let matched = |entry: &CommandLogEntry| candidates.iter().find(|(edit_id, _)| entry.child_edit_ids.iter().any(|logged| logged == edit_id)).map(|(_, group_id)| group_id.clone());
        let ordered = if action == "undo" { self.command_log.iter().rev().find_map(matched) } else { self.command_log.iter().find_map(matched) };
        ordered.or_else(|| candidates.first().map(|(_, group_id)| group_id.clone()))
    }

    /// 🧾️ Projects the whole command log into its host view, reusing the op text a PREVIOUS view
    /// already printed for every edit that did not grow.
    ///
    /// ⏱️ Both reuses are what make this O(changed) instead of O(n²) (ticket
    /// 26/09/02/PUZZLE-3D-END-TO-END wave B54). A mutation bumps `log_generation`, so
    /// [`Self::refresh_cache`]'s incremental arm misses and this runs on every single completion —
    /// and it used to resolve each row's edit by a LINEAR scan of the whole edit list (161 rows ×
    /// 161 edits on the battery's brush-painted document) and re-`print_op` every operation of every
    /// edit in the log, both for rows whose text had not changed since the last projection.
    ///
    /// 🧩️ Child lane: a composed app whose document verbs declare
    /// `ArtifactToolPublicationLane::Child` writes every user edit into a CHILD store, so the
    /// parent's own applied stack stays empty for exactly the gesture the user wants back.
    /// `SpaceMember` exposes only each child's TAIL applied/redo edit object-safely — which is
    /// all `undo`/`redo` ever reach anyway, since both walk one member's stack one step.
    ///
    /// 🌉️ Hoisted out of the loop below — both envelopes are the SAME value for every row, so
    /// awaiting once here is strictly cheaper than re-awaiting per row.
    ///
    /// 🌉️ Rewritten from a `.map(|entry| {...}).collect()` into an explicit loop (sync —
    /// `Iterator::map` cannot take an async closure, and `OpText::print_op` is genuinely async).
    ///
    /// 🪞️ Mirrors `store::ArtifactStore::edit_is_local` (private to that crate): an edit
    /// with no recorded actor is treated as local, same as a real undo would.
    ///
    /// ⏪️ Three disjoint ways a row earns "inverse": document edit-linked (applied +
    /// locally authored), config edit-linked (same, on the config store — B1's replacement
    /// for the old memory-only "View"-kind inverse), or memory-only (a stored
    /// `InverseAction`, the remaining `🐚️Shell`-kind `noteShellCommand` path).
    ///
    /// ✅️ `applied` is "this row's edit is live somewhere", and a row publishes into
    /// exactly one of THREE lanes — the parent document store, the config store, or a
    /// composed child's store. Asking only the first made every `Child`- and `Config`-lane
    /// row report `applied: false`, which the host reads as UNDONE: the History panel
    /// dimmed the row and `uncommittedEditCount` (`#s-checkin`) never counted it. That is
    /// one root under three kinds — 🌊️flow's `addWidget` and 🎬️sequence's `addStep` are
    /// `Child`, 🌀️procedural's `generate` is `Config` — each of which moved its document
    /// and still read `edits 0` (ticket 26/09/18 S10; PB3 §3.6 named the child half).
    /// `revertible` below already consulted all three; this is the same law for `applied`.
    ///
    /// ⏪️ …but ONLY for a row that has no parent-document edit of its own. `undo`/`redo`
    /// dispatch against `self.store` alone — `interaction_store`'s own doc states it in as
    /// many words — so a MULTI-lane row (`Artifact` + `Config`, which is what
    /// 💠️lowpoly's `addPrimitive` and 🎥️shooting's `addShot` declare) keeps a live config
    /// edit after its document edit has been undone. Reading those two with `||` made such
    /// a row report `applied: true` for ever: the History panel never dimmed it and
    /// `#s-checkin`'s uncommitted count never came back down. Measured inside `s` on
    /// 2026-09-22 at three different machine loads as `edits [0,1,1,1]` with the ledger
    /// still growing on the undo AND the redo (`applied [0,1,3,6]` / `[1,2,4,7]`) — a
    /// regression of the fix above, in the two kinds that publish on more than one lane
    /// (ticket 26/09/18 S11). The parent document lane is authoritative whenever the row
    /// published into it; the other two answer only for a row that has no parent edit at
    /// all, which is exactly the Child/Config-only case the fix above exists for.
    pub(crate) async fn build_history_view(&self, previous: Option<&HistoryView>) -> HistoryView {
        let envelope = self.store.envelope();
        let labelled: HashMap<&str, &MutationView> =
            previous.map_or_else(HashMap::new, |previous| previous.commands.iter().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect());
        let members = if self.command_log.iter().any(|entry| !entry.child_edit_ids.is_empty()) { self.member_edit_histories(&labelled, None) } else { HashMap::new() };
        let (child_applied_tails, child_has_redo_tail) = self.child_history_tails().await;
        let reads = HistoryRowReads {
            full: true,
            positions: self.store.applied_edit_ids().iter().enumerate().map(|(position, edit_id)| (edit_id.as_str(), Some(position))).collect(),
            edits: envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect(),
            previous: previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect()),
            labelled,
            members,
            child_applied_tails,
            applied_entries: self.supersedes.applied_entries(self.store.supersessions().iter()),
            app_id: self.app.instance_id().await.to_string(),
        };
        let commands: Vec<CommandView> = self.command_log.iter().rev().map(|entry| self.history_row(entry, &reads)).collect();
        let (can_undo, can_redo) = self.history_view_stacks(&reads.child_applied_tails, child_has_redo_tail, self.command_log.iter().filter(|entry| entry.inverse.is_some()).count());
        HistoryView {
            columns: build_history_columns(envelope).await,
            can_undo,
            can_redo,
            active_alternative_id: envelope.active_alternative_id.clone(),
            alternatives: self.history_alternatives(),
            current_checkpoint_id: self.store.current_checkpoint_id().map(str::to_string),
            commands,
            command_filter: self.history_filter,
        }
    }

    /// 🩹️ Rebuilds the rows `patch` names in the cached `view` and puts the new log rows in front (design §20.14:
    /// O(change)): each rebuilt row reads its own edit's slice and its composed members' edits, reusing its previous
    /// projection's printed lines and labels; the undo and redo state follows. Answers the rows whose projection changed.
    async fn patch_history_view(&self, view: &mut HistoryView, patch: HistoryViewPatch, inverse_rows: usize) -> Vec<u64> {
        let (child_applied_tails, child_has_redo_tail) = self.child_history_tails().await;
        let app_id = self.app.instance_id().await.to_string();
        let (rows, fresh, stacks) = {
            let held = |seq: &u64| view.commands.binary_search_by(|row| seq.cmp(&row.seq)).ok().map(|index| &view.commands[index]);
            let logged = |seq: &u64| self.command_log.iter().rev().find(|entry| entry.seq == *seq);
            let previous: HashMap<u64, &CommandView> = patch.rebuild.iter().filter_map(|seq| held(seq).map(|row| (*seq, row))).collect();
            let labelled: HashMap<&str, &MutationView> = previous.values().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect();
            let rebuilt: Vec<&CommandLogEntry> = patch.rebuild.iter().filter_map(logged).collect();
            let fresh = || self.command_log.iter().rev().take(self.command_log.len() - patch.fresh_from).rev();
            let wanted: HashSet<&str> = rebuilt.iter().copied().chain(fresh()).flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
            let members = self.member_edit_histories(&labelled, Some(&wanted));
            let reads = HistoryRowReads {
                full: false,
                positions: patch.positions.iter().map(|(edit_id, position)| (edit_id.as_str(), *position)).collect(),
                edits: HashMap::new(),
                previous,
                labelled,
                members,
                child_applied_tails,
                applied_entries: self.supersedes.applied_entries(self.store.supersessions().iter()),
                app_id,
            };
            let rows: Vec<(u64, CommandView)> = rebuilt.iter().map(|entry| (entry.seq, self.history_row(entry, &reads))).collect();
            let fresh: Vec<CommandView> = fresh().rev().map(|entry| self.history_row(entry, &reads)).collect();
            (rows, fresh, self.history_view_stacks(&reads.child_applied_tails, child_has_redo_tail, inverse_rows))
        };
        let mut changed: Vec<u64> = fresh.iter().map(|row| row.seq).collect();
        for (seq, row) in rows {
            if let Ok(index) = view.commands.binary_search_by(|held| seq.cmp(&held.seq)) {
                if view.commands[index] != row {
                    changed.push(seq);
                    view.commands[index] = row;
                }
            }
        }
        view.commands.splice(0..0, fresh);
        (view.can_undo, view.can_redo) = stacks;
        view.command_filter = self.history_filter;
        changed
    }

    /// 🩹️ Which rows of the cached view a refresh rebuilds after the stores and the command log moved from `before` to
    /// `after` (design §20.14): every new log row, the previous newest row (the log amends only its newest row), the held
    /// rows of every edit and composed-member edit an appended, amended, undone or redone tail touched
    /// ([`time_travel::history_store_change`]) — found from the log's tail — and the shell rows an undo or redo of theirs
    /// flipped. `None` when the change is not such a patch (rows retired, a member came or went, a history edit, a replay,
    /// a checkout …): the view then rebuilds.
    fn history_view_patch(&self, before: &HistoryViewStamp, after: &HistoryViewStamp, members: &[(String, time_travel::HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)]) -> Option<HistoryViewPatch> {
        if before.supersede_records != after.supersede_records || before.members.len() != members.len() || before.members.iter().zip(members).any(|((held, _), (member, _, _))| held != member) {
            return None;
        }
        let fresh_count = self.command_log.iter().rev().take_while(|entry| entry.seq > before.newest_seq).count();
        let fresh_from = self.command_log.len() - fresh_count;
        if fresh_from != before.rows || (fresh_from > 0 && self.command_log.iter().rev().nth(fresh_count).map(|entry| entry.seq) != Some(before.newest_seq)) {
            return None;
        }
        let positions = time_travel::history_store_change(&before.document, &after.document, self.store.applied_edit_ids())?;
        let fresh = || self.command_log.iter().rev().take(fresh_count);
        let mut children: HashSet<&str> = HashSet::new();
        for (_, _, change) in members {
            children.extend(change.as_ref()?.iter().map(|(edit_id, _)| edit_id.as_str()));
        }
        children.retain(|edit_id| !fresh().any(|entry| entry.child_edit_ids.iter().any(|child| child == edit_id)));
        let mut edits: HashSet<&str> = positions.iter().map(|(edit_id, _)| edit_id.as_str()).filter(|edit_id| !fresh().any(|entry| entry.edit_id.as_deref() == Some(*edit_id))).collect();
        let mut rebuild: BTreeSet<u64> = BTreeSet::new();
        if fresh_from > 0 {
            rebuild.insert(before.newest_seq);
        }
        for entry in self.command_log.iter().rev().skip(fresh_count) {
            if edits.is_empty() && children.is_empty() {
                break;
            }
            let edited = entry.edit_id.as_deref().is_some_and(|edit_id| edits.remove(edit_id));
            let membered = entry.child_edit_ids.iter().fold(false, |found, child| children.remove(child.as_str()) || found);
            if edited || membered {
                rebuild.insert(entry.seq);
            }
        }
        if before.shell_undone != after.shell_undone {
            rebuild.extend(self.history_dirty_sequences.iter().copied().filter(|seq| self.command_log.iter().rev().find(|entry| entry.seq == *seq).is_some_and(|entry| entry.inverse.is_some())));
        }
        Some(HistoryViewPatch { fresh_from, rebuild, positions })
    }

    /// 🧱️ One history row of `entry` over `reads` — what a full build and an incremental patch share (design §20.14): a
    /// supersede record's row, or a command row with its edit's printed lines, its applied and revertible state, its mutation
    /// rows (its edit's applied operations read as one slice, then its composed members'), its transaction and its label.
    fn history_row(&self, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>) -> CommandView {
        #[cfg(test)]
        HISTORY_ROWS_BUILT.with(|built| built.set(built.get() + 1));
        let previous = reads.previous.get(&entry.seq).copied();
        if let Some((index, record)) = entry.transition_id.as_deref().and_then(|transition_id| self.supersedes.record(transition_id)) {
            let supersede_author = self.supersede_author();
            return CommandView {
                seq: entry.seq,
                action_id: entry.action_id.clone(),
                label: self.supersede_row_label(index),
                kind: entry.kind,
                timestamp: entry.timestamp.clone(),
                edit_id: None,
                child_edit_ids: Vec::new(),
                transition_id: Some(record.transition_id.clone()),
                author: Some(record.actor.as_str().to_owned()),
                op_lines: Vec::new(),
                op_count: 0,
                applied: self.supersedes.row_applied(index, &reads.applied_entries),
                revertible: record.role != SupersedeRole::Undo && record.actor.as_str() == supersede_author && self.supersedes.undoable(supersede_author, record.entry, &reads.applied_entries),
                count: entry.count,
                inverse: None,
                transaction: None,
                mutations: Vec::new(),
            };
        }
        let edit = entry.edit_id.as_deref().and_then(|edit_id| match reads.full {
            true => reads.edits.get(edit_id).copied(),
            false => self.store.envelope().vcs.edits.iter().rev().find(|edit| edit.id == edit_id),
        });
        let position = self.history_row_position(entry, reads, previous);
        let applied_ops = position.and_then(|position| self.store.applied_edit_mutations(position).ok()).unwrap_or_default();
        let durable = position.and_then(|position| self.store.applied_edit_outcomes(position).ok()).unwrap_or_default();
        let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
        let ops: Vec<&store::AppliedMutation<'_, A::Mutation>> = applied_ops.iter().collect();
        let mut op_lines = Vec::new();
        let mut op_count = 0;
        if let Some(edit) = edit {
            op_count = edit.forwards.len();
            match previous.filter(|row| row.edit_id == entry.edit_id && row.op_count == op_count) {
                Some(row) => op_lines.extend_from_slice(&row.op_lines),
                None => op_lines.extend(edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op())),
            }
        }
        let document_applied = position.is_some();
        let child_applied = entry.child_edit_ids.iter().any(|edit_id| reads.child_applied_tails.contains(edit_id));
        let local_actor = self.store.local_actor_id();
        let revertible = (document_applied && edit.is_some_and(|edit| edit.actor.as_deref() == Some(local_actor.0.as_str()))) || child_applied || (entry.inverse.is_some() && !self.shell_undone.contains(&entry.seq));
        let child_edits: Vec<&time_travel::MemberEditHistory> = entry.child_edit_ids.iter().filter_map(|edit_id| reads.members.get(edit_id.as_str())).collect();
        if edit.is_none() {
            op_count = child_edits.iter().map(|child| child.op_count).sum();
            op_lines.extend(child_edits.iter().flat_map(|child| child.op_lines.iter().cloned()));
            op_lines.drain(..op_lines.len().saturating_sub(HISTORY_ROW_OPERATION_PREVIEW));
        }
        let mut mutations = time_travel::history_mutation_views::<A>(&ops, &outcomes, &reads.labelled);
        mutations.extend(child_edits.iter().flat_map(|child| child.mutations.iter().cloned()));
        let transaction = edit.and_then(|edit| edit.mutation_meta.first()).and_then(|meta| meta.transaction.clone()).or_else(|| child_edits.iter().find_map(|child| child.transaction.clone()));
        let tool_run_label = transaction.as_ref().and_then(|transaction| self.tool_run_transaction_label(&reads.app_id, transaction));
        let leaf_label = ops.first().map(|op| op.operation).or_else(|| edit.and_then(|edit| edit.forwards.first())).map(|op| protocol::SemanticMutation::<A::Snapshot>::label(op));
        let verb_label = edit.and_then(|edit| edit.verb.as_deref()).and_then(|verb| self.verb_label(verb));
        let intent = transaction.as_ref().and_then(|transaction| history_intent_label::<A>(&transaction.tool, &ops, &mutations)).or_else(|| child_edits.iter().find_map(|child| child.intent_label.clone()));
        let label = match (edit, mutations.first()) {
            _ if tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
            (Some(_), Some(first)) if transaction.is_some() => {
                history_leaf_row_label(intent.as_ref().unwrap_or(&first.label), op_count)
            }
            (Some(_), _) if op_count == 1 && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
            (Some(_), _) if verb_label.is_some() => verb_label.expect("verb label checked above"),
            (Some(_), _) if leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
            (None, Some(first)) if !child_edits.is_empty() => history_leaf_row_label(intent.as_ref().unwrap_or(&first.label), op_count),
            _ => entry.label.clone(),
        };
        CommandView {
            seq: entry.seq,
            action_id: entry.action_id.clone(),
            label,
            kind: entry.kind,
            timestamp: entry.timestamp.clone(),
            edit_id: entry.edit_id.clone(),
            child_edit_ids: entry.child_edit_ids.clone(),
            transition_id: None,
            author: edit.and_then(|edit| edit.actor.as_ref().map(|actor| actor.as_str().to_owned())),
            op_lines,
            op_count,
            applied: history_row_applied(entry.edit_id.is_some(), document_applied, child_applied),
            revertible,
            count: entry.count,
            inverse: entry.inverse.clone(),
            transaction,
            mutations,
        }
    }

    /// 📍️ The applied position of `entry`'s document edit as `reads` know it: from their positions (a full build knows
    /// every applied edit), else — a patch, for a row whose edit no change touched — the position its previous projection
    /// names (its own mutation rows, checked against the applied stack; an edit it held unapplied stays unapplied, no change
    /// named it), else a search from the stack's tail (a row new to the view whose edit the change did not name, e.g. a
    /// backfilled one). An edit neither knows is not applied.
    fn history_row_position(&self, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>, previous: Option<&CommandView>) -> Option<usize> {
        let edit_id = entry.edit_id.as_deref()?;
        if let Some(position) = reads.positions.get(edit_id) {
            return *position;
        }
        if reads.full {
            return None;
        }
        let applied = self.store.applied_edit_ids();
        let held = previous.filter(|row| row.edit_id.as_deref() == Some(edit_id));
        if held.is_some_and(|row| !row.applied) {
            return None;
        }
        held.and_then(|row| row.mutations.iter().find(|mutation| mutation.store.is_none()))
            .map(|mutation| mutation.position as usize)
            .filter(|position| applied.get(*position).is_some_and(|held| held == edit_id))
            .or_else(|| self.store.applied_edit_position(edit_id))
    }

    /// ↩️ The history body's undo and redo availability: a document edit, a composed member's applied tail, a live shell
    /// row (`inverse_rows` of them, the undone ones aside) or a history edit to undo; a redo of any of them.
    fn history_view_stacks(&self, child_applied_tails: &HashSet<String>, child_has_redo_tail: bool, inverse_rows: usize) -> (bool, bool) {
        let can_undo = !self.store.applied_edit_ids().is_empty() || !child_applied_tails.is_empty() || inverse_rows > self.shell_undone.len() || self.supersede_can_undo();
        let can_redo = !self.store.redo_edit_ids().is_empty() || child_has_redo_tail || !self.shell_redo.is_empty() || self.supersede_redo_target().is_some();
        (can_undo, can_redo)
    }

    /// 🌿️ The alternatives the history body lists ([`history_alternative_views`]) over the log's branch and supersede
    /// transitions.
    fn history_alternatives(&self) -> Vec<AlternativeView> {
        let envelope = self.store.envelope();
        let transitions: Vec<(&store::os_spr::MutationEnvelope, store::os_spr::HistoryTransition)> =
            envelope.transitions.iter().filter_map(|transition| store::os_spr::history_transition_from_envelope(transition).ok().flatten().map(|decoded| (transition, decoded))).collect();
        let facts: Vec<AlternativeFact<'_>> = transitions
            .iter()
            .filter_map(|(transition, decoded)| match decoded {
                store::os_spr::HistoryTransition::Branch { alternative_id, .. } => Some(AlternativeFact::Branch { alternative_id, actor: &transition.actor.0, physical_ms: transition.timestamp.physical_ms }),
                store::os_spr::HistoryTransition::Supersede(supersede) => Some(AlternativeFact::Supersede { scope: supersede.scope.as_deref() }),
                _ => None,
            })
            .collect();
        let line = self.store.active_line_id();
        history_alternative_views(envelope.vcs.alternatives.iter().map(|alternative| (alternative.id.as_str(), alternative.name.as_str())), Some(line.as_str()), &facts)
    }

    /// 🗂️ The history view of the current command log and stores (design §20.14): the cached one patched in place where
    /// the change only appended, amended, undid or redid edits and rows ([`Self::history_view_patch`]) — O(change), and in
    /// the same allocation unless a command job still shares it — else rebuilt over the whole history, reusing the previous
    /// projection's printed lines and labels. Every row whose projection changed is marked for the next history patch, so
    /// the wire carries exactly the changed rows (an undo no longer re-sends the whole history).
    async fn refresh_history_view(&mut self) -> std::sync::Arc<HistoryView> {
        let retained = self.history_view.take();
        let members = self.member_history_stamps(retained.as_ref().map_or(&[][..], |cache| cache.stamp.members.as_slice()));
        let mut stamp = HistoryViewStamp {
            document: time_travel::history_store_stamp(&self.store),
            members: members.iter().map(|(member, stamp, _)| (member.clone(), stamp.clone())).collect(),
            newest_seq: self.command_log.iter().next_back().map_or(0, |entry| entry.seq),
            rows: self.command_log.len(),
            inverse_rows: 0,
            shell_undone: self.shell_undone.len(),
            supersede_records: self.supersedes.records.len(),
        };
        let patch = retained.as_ref().and_then(|cache| self.history_view_patch(&cache.stamp, &stamp, &members).map(|patch| (patch, cache.stamp.inverse_rows)));
        let (view, changed) = match (retained, patch) {
            (Some(HistoryViewCache { mut view, .. }), Some((patch, inverse_rows))) => {
                stamp.inverse_rows = inverse_rows + self.command_log.iter().rev().take(self.command_log.len() - patch.fresh_from).filter(|entry| entry.inverse.is_some()).count();
                let changed = self.patch_history_view(std::sync::Arc::make_mut(&mut view), patch, stamp.inverse_rows).await;
                (view, changed)
            }
            (retained, _) => {
                stamp.inverse_rows = self.command_log.iter().filter(|entry| entry.inverse.is_some()).count();
                let previous = retained.as_ref().map(|cache| cache.view.as_ref());
                let view = self.build_history_view(previous).await;
                let held: HashMap<u64, &CommandView> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect());
                let changed = view.commands.iter().filter(|row| held.get(&row.seq).is_none_or(|before| *before != *row)).map(|row| row.seq).collect();
                (std::sync::Arc::new(view), changed)
            }
        };
        self.history_dirty_sequences.extend(changed);
        self.history_view = Some(HistoryViewCache { view: std::sync::Arc::clone(&view), stamp });
        view
    }

    /// 🧾️ Converts the authoritative command log into a typed host projection patch. Rows are
    /// never folded or capped here; the host controls its own visible window. A delta patch reads only its dirty rows
    /// (each found by its `seq` in the newest-first view, design §20.14) and shares the cached view, never copies it.
    pub(crate) async fn history_patch(&mut self, snapshot: bool) -> Result<HistoryPatch, Fault> {
        self.refresh_cache().await?;
        let history = std::sync::Arc::clone(&self.cache.as_ref().expect("history cache refreshed").3);
        let dirty = std::mem::take(&mut self.history_dirty_sequences);
        let time_travel = self.time_travel.panel();
        let rows: Vec<&CommandView> = match snapshot {
            true => history.commands.iter().collect(),
            false => {
                let mut seqs: Vec<u64> = dirty.into_iter().collect();
                seqs.sort_unstable_by(|a, b| b.cmp(a));
                seqs.iter().filter_map(|seq| history.commands.binary_search_by(|row| seq.cmp(&row.seq)).ok().map(|index| &history.commands[index])).collect()
            }
        };
        let upserts = rows
            .into_iter()
            .map(|entry| (entry, entry.mutations.iter().map(|mutation| history_mutation_entry(mutation, time_travel.as_ref())).collect::<Vec<_>>()))
            .map(|(entry, mutations)| HistoryEntry {
                seq: entry.seq,
                action_id: entry.action_id.clone(),
                label: entry.label.clone(),
                kind: match entry.kind {
                    ActionKind::Mutation => "mutation",
                    ActionKind::History => "history",
                    ActionKind::Clipboard => "clipboard",
                    ActionKind::View => "view",
                    ActionKind::Shell => "shell",
                    ActionKind::Interaction => "interaction",
                }
                .into(),
                timestamp: entry.timestamp.clone(),
                op_lines: entry.op_lines.clone(),
                op_count: entry.op_count as u64,
                applied: entry.applied,
                revertible: entry.revertible,
                count: entry.count,
                edit_id: entry.edit_id.clone(),
                transaction: entry.transaction.as_ref().map(|transaction| semio_framework::kernel::HistoryTransaction { id: transaction.id.clone(), tool: transaction.tool.clone() }),
                transition_id: entry.transition_id.clone(),
                author: entry.author.clone(),
                worst: mutations.iter().filter_map(|mutation| mutation.worst).max(),
                mutations,
            })
            .collect();
        Ok(HistoryPatch {
            cursor: self.log_generation,
            upserts,
            can_undo: history.can_undo,
            can_redo: history.can_redo,
            active_alternative_id: history.active_alternative_id.clone(),
            current_checkpoint_id: history.current_checkpoint_id.clone(),
            command_filter: history.command_filter.value().into(),
            time_travel: time_travel.map(|panel| panel.status),
            reprojection: self.reprojection_status(),
            edit_count: u32::try_from(self.store.envelope().vcs.edits.len()).unwrap_or(u32::MAX),
        })
    }

    /// 🗂️ Refreshes the snapshot cache if the store advanced, the command log grew/folded, or
    /// the history filter changed since the last materialization. The key is recomputed a SECOND time
    /// after `backfill_command_log` — backfill itself may `push_log_entry` (bumping `log_generation`),
    /// so keying only on the pre-backfill snapshot would store a stale key and thrash on every call.
    ///
    /// 🧾️ The history view is patched, not rebuilt ([`Self::refresh_history_view`], design §20.14): the cache's own
    /// share of it is dropped first, so the retained view stays the only owner and is patched in its own allocation.
    pub(crate) async fn refresh_cache(&mut self) -> Result<(), Fault> {
        while self.command_prune_requested() || self.pending_command_prune.is_some() {
            self.advance_command_prune_step(self.mounted_policy.maintenance, false)?;
            plugin_job_yield_once().await;
        }
        let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
        if self.cache.as_ref().map(|(cached_key, _, _, _)| *cached_key) == Some(key) {
            return Ok(());
        }
        self.backfill_command_log().await;
        let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
        let (snapshot, config) = match self.cache.take() {
            Some((cached_key, snapshot, config, history)) => {
                drop(history);
                (if cached_key.0 == key.0 { snapshot } else { self.store.snapshot_owner() }, if cached_key.1 == key.1 { config } else { self.config_store.snapshot_owner() })
            }
            None => (self.store.snapshot_owner(), self.config_store.snapshot_owner()),
        };
        let history = self.refresh_history_view().await;
        self.cache = Some((key, snapshot, config, history));
        self.follow_tool_machines(false);
        Ok(())
    }

    /// 🎯️ Shares immutable cache inputs with a command job without cloning the document or history.
    pub(crate) fn command_cache_inputs(&self) -> (std::sync::Arc<A::Snapshot>, std::sync::Arc<A::Config>, std::sync::Arc<HistoryView>) {
        let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed before command scheduling");
        (std::sync::Arc::clone(snapshot), std::sync::Arc::clone(config), std::sync::Arc::clone(history))
    }

    /// 🖋️ Decodes `args.authors` (`[{id, name, avatar?}]`, both shells' wire shape for a
    /// checkpoint dispatch) into `vcs::Author`s — empty when absent or malformed, never a hard
    /// error: an authorless checkpoint stays valid, it just can't say who checked in.
    async fn history_command_authors(args: Option<&DslValue>) -> Vec<vcs::Author> {
        args.and_then(|value| value.get("authors")).and_then(|value| <Vec<vcs::Author> as semio_framework_value::FromValue>::from_value(value.clone()).ok()).unwrap_or_default()
    }

    /// 🕰️ Maps one of the six injected history action ids to its `ArtifactCommand`.
    async fn history_command(action: &str, args: Option<&DslValue>) -> Option<ArtifactCommand<A::Mutation>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        match action {
            "undo" => Some(ArtifactCommand::Undo),
            "redo" => Some(ArtifactCommand::Redo),
            "commitCheckpoint" => Some(ArtifactCommand::CommitCheckpoint { message: arg_str("message"), authors: Self::history_command_authors(args).await }),
            "createAlternative" => Some(ArtifactCommand::CreateAlternative { name: arg_str("name").unwrap_or_else(|| "Alternative".into()) }),
            "switchAlternative" => arg_str("alternativeId").map(|alternative_id| ArtifactCommand::SwitchAlternative { alternative_id }),
            "checkoutCheckpoint" => arg_str("checkpointId").map(|checkpoint_id| ArtifactCommand::CheckoutCheckpoint { checkpoint_id }),
            _ => None,
        }
    }

    async fn reset_dispatch_report(&mut self) {
        self.dispatch_report = protocol::DispatchReport { policy: self.store.merge_policy(), worst: None, messages: Vec::new() };
    }

    async fn record_dispatch_receipt(&mut self, receipt: store::CommandReceipt) {
        self.dispatch_report = protocol::DispatchReport { policy: self.store.merge_policy(), worst: receipt.worst, messages: receipt.messages.into_iter().flat_map(|entry| entry.messages).collect() };
    }

    async fn record_rejected_dispatch(&mut self, policy: protocol::MergePolicy, messages: Vec<protocol::MutationMessage>) -> Fault {
        self.dispatch_report = protocol::DispatchReport { policy, worst: protocol::worst_level(&messages), messages };
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.rejected"), "mutation rejected by the active merge policy")
    }

    async fn apply_merge_policy(&mut self, policy: protocol::MergePolicy) {
        self.store.set_merge_policy(policy);
        self.reset_dispatch_report().await;
    }

    async fn resolve_store_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        let report = self.store.resolve_conflict(conflict_id, resolution).await.map_err(|error| error.into_fault())?;
        self.dispatch_report = protocol::DispatchReport { policy: report.policy, worst: report.worst, messages: report.replayed.iter().flat_map(|entry| entry.messages.iter().cloned()).collect() };
        self.cache = None;
        Ok(report)
    }

    async fn current_open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.store.open_conflicts().cloned().collect()
    }

    /// 🔍️ Pure dry run (§C9): folds `ops` over the CURRENT persisted document snapshot via
    /// `SpaceMember::preview_wire` — never applies anything, never touches `self.dispatch_report` —
    /// and packages the resulting messages under the store's active `MergePolicy` as a
    /// `DispatchReport`. This is the single predicate a host/UI consults to grey out an action or
    /// context-menu item BEFORE a real dispatch is even attempted, so the "alone ⇒ illegal actions
    /// are prevented up front" half of the ticket holds without a round trip through `dispatch_emit`.
    pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport {
        let mut op_bytes: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
        for op in ops.iter() {
            op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let messages = SpaceMember::preview_wire(&self.store, &op_bytes).await;
        protocol::DispatchReport { policy: self.store.merge_policy(), worst: protocol::worst_level(&messages), messages }
    }

    /// 📇️ An empty `InvocationResult` carrying only host effects/events (view/shell actions,
    /// no-operation commands, and history notifications produce no `KernelMutation`s).
    async fn empty_result(verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope) -> InvocationResult {
        let invocation_id = InvocationId(format!("{verb}:{}", meta.instance_id));
        InvocationResult {
            output: DslValue::Null,
            mutations: Vec::new(),
            inverse_group: UndoGroup { invocation_id, mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        }
    }

    /// 🧱️ Builds the `InvocationResult` for a just-dispatched edit: one `KernelMutation` per
    /// forward operation NEW in this dispatch (`tail_offset`), each carrying just this dispatch's
    /// `inverse` as its inverse diff. For a streamed transaction's open edit, `edit_mutations()` returns
    /// the WHOLE accumulated edit — without slicing to `tail_offset`, every dispatch would rebuild and
    /// serialize every `KernelMutation` since the gesture started (O(edit-size) per dispatch, O(edit-
    /// size²) over the whole gesture) purely to report operations the caller already knows about.
    ///
    /// 🎯️ B5: real binary — each backward op's own `OpBinary::encode_op()`, framed as a
    /// binary ops-vec (`protocol::encode_ops_vec`, replacing the old `json!({"inverse":
    /// [...]})` convention). Every op in `inverse` shares the same encoding; a decode
    /// failure here would mean a real corrupt/foreign operation, not a schema choice, so
    /// `unwrap_or_default` on an individual encode failure degrades to an empty inverse
    /// (same fallback behavior `payload: Vec::new()` already has elsewhere) rather than
    /// panicking mid-invocation.
    ///
    /// 🎯️ W6 kernel unification: `mutation_meta` entries are `protocol::MutationMeta`,
    /// and this kernel's own `ArtifactDiff`/`UndoPolicy`/`HybridLogicalTimestamp` are now
    /// `pub use` re-exports of the SAME `protocol`/`protocol_core` types (see
    /// `framework/core`'s kernel cut-over note) — no bridging left to do, just direct
    /// field moves. `ArtifactDiff.schema`/`.payload` are `SchemaId`/`Vec<u8>` now (was
    /// `schema_id`/`Value`), so the payload is JSON-encoded to bytes at construction.
    async fn result_from_last_edit(&self, verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope, tail_offset: (usize, usize)) -> InvocationResult {
        let schema = A::DOCUMENT_SCHEMA.to_string();
        let invocation_id = InvocationId(format!("{verb}:{}:{}", meta.instance_id, self.store.generation()));
        let document = ArtifactHandle(meta.instance_id as u128);
        let mut mutations: Vec<KernelMutation> = Vec::new();
        if let Some((forwards, inverse, mutation_meta)) = self.store.edit_mutations() {
            let (forwards_offset, backwards_offset) = tail_offset;
            let forwards = &forwards[forwards_offset.min(forwards.len())..];
            let mutation_meta = &mutation_meta[forwards_offset.min(mutation_meta.len())..];
            let mut inverse_op_bytes = Vec::with_capacity(inverse.len().saturating_sub(backwards_offset));
            for op in inverse.iter().skip(backwards_offset) {
                inverse_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
            }
            let inverse_payload = protocol::encode_ops_vec(&inverse_op_bytes);
            for (index, forward) in forwards.iter().enumerate() {
                let entry = mutation_meta.get(index);
                let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                mutations.push(KernelMutation {
                    id: mutation_id.clone(),
                    document,
                    base_version,
                    invocation_id: invocation_id.clone(),
                    diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: ::protocol::OpBinary::encode_op(forward).unwrap_or_default() },
                    inverse: InverseMutation {
                        target_mutation: mutation_id,
                        inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: inverse_payload.clone() },
                        base_version,
                        dependencies: Vec::new(),
                        undo_policy,
                    },
                    dependencies: Vec::new(),
                    author,
                    timestamp,
                });
            }
        }
        let mutation_ids: Vec<MutationId> = mutations.iter().map(|operation| operation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|operation| operation.inverse.clone()).collect();
        InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits: Vec::new() },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        }
    }

    /// 🧬️ Shared dispatch tail for `handle_action`/`handle_command`/`import_media`: given the
    /// app's `ActionEmit`, either records the op-less dispatch and returns an empty result, or commits
    /// `Apply` or the transaction commands, records the resulting edit, and builds the `InvocationResult` from it.
    /// `verb` is the action/command id, used to resolve the registry kind/label and to synthesize the
    /// `InvocationId`.
    /// 🧬️ B1: the single dispatch tail for BOTH `dispatch_typed_command` (the app's own
    /// `Emit`) and the framework-reserved clipboard actions below — commits `artifact_mutations` to
    /// the document store and `config_mutations` to the config store (independently, each its own
    /// undo stack), records exactly one command-log row carrying whichever edit id(s) were produced,
    /// and builds the resulting `InvocationResult` from the document side (config-only dispatches
    /// never touch `KernelMutation`/space-sync — see `🔖️CommandLog`'s doc region for why).
    /// 🔀️ Builds a typed rejection `Fault` using one of the frozen transaction rejection codes
    /// (contract-freeze.md §5's "Rejection taxonomy") — shared by `dispatch_emit`'s freeze check
    /// and every `transaction_prepare`/`transaction_commit`/`transaction_rollback`/
    /// `transaction_undo`/`transaction_redo` rejection path below.
    /// 🧮️ B1: `materialize_args` (JSON-args default-fill + required-arg enforcement for app-declared
    /// actions/commands, plus its now-unused `effective_action_args`/`missing_required_args` imports)
    /// was deleted — dead code once `dispatch_action`'s generic app-action fallback and
    /// `dispatch_command`'s registry-backed dispatch were removed (an app's own behavior now dispatches
    /// exclusively through the typed `Self::Command` channel, where a Rust caller supplies a complete
    /// value — there is no "missing arg" to materialize).
    ///
    /// 🚫️async: E1 pure error constructor consumed pervasively by sync closures — see R9.
    fn transaction_fault(origin: FaultOrigin, code: &'static str, message: impl Into<String>) -> Fault {
        Fault::new(origin, FaultCode::new(code), message)
    }

    /// 🔁️ Drains `emit.extension_invocations` through the guest request registry: each entry
    /// allocates a `RequestId`, records its `response_action` continuation against that id and
    /// queues the resulting `Effect::InvokeExtension` on the registry's own outbound queue (the
    /// one `reactor::poll` drains into `turn-result.effects` later in the SAME turn). Also the
    /// fail-closed gate on the defect this lane replaces: an app that smuggles a hand-built
    /// `Effect::InvokeExtension` into `emit.effects` is rejected outright rather than having its
    /// result silently dropped at `Event::Completed` for an id that owns no registry slot.
    fn mint_extension_invocations(instance: u32, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        if emit.effects.iter().any(|effect| matches!(effect, Effect::InvokeExtension { .. })) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unminted-extension-invocation"), "Effect::InvokeExtension carries a registry-minted request id — emit it through Emit::extension_invocations instead"));
        }
        for invocation in std::mem::take(&mut emit.extension_invocations) {
            crate::reactor::queue_extension_invocation(instance, &invocation)?;
        }
        Ok(())
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the reactor-side fold of a guest emit's
    /// interaction verbs: every `Effect::ReplayShellCommand { action_id ∈ INTERACTION_ACTION_IDS }`
    /// and every `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }` (the same intent in
    /// `📏️layout`/`💡️reasoning`'s emission shape, any `delay_ms`) is peeled BEFORE the effects
    /// are handed out ([`take_inline_interaction_verbs`]), the emit
    /// lands exactly as before ([`Self::dispatch_emit_inner`]), and then — after this command's
    /// own store lanes and `interaction_writes` landed, so a just-created id is already in
    /// topology — each peeled verb is applied inline ([`Self::fold_inline_interaction_verbs`]).
    /// The result leaves with the verbs' scopes merged into its `ui_scope`, the last verb's
    /// leftover `InteractionView` as its `output` when the carrier produced none (the SAME
    /// `Invocation.output.interactionView` lane a host-dispatched verb publishes on, so the
    /// world overlay still lands), and one diagnostic per verb that could not be applied.
    /// A failed carrier applies no verb: its pick did not happen. The typed-operation ladder,
    /// which never enters here, has its own twin ([`Self::publish_mounted_typed_inline_interaction_unit`]).
    pub(crate) async fn dispatch_emit(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if !emit.owned_child_emits.is_empty()||emit.owned_child_emits.capacity()!=0||!emit.child_preparations.is_empty()||emit.child_preparations.capacity()!=0{
            return self.mount_original_emit_publication(verb, emit, meta, None, cx).await;
        }
        let folded = take_inline_interaction_verbs(&mut emit.effects);
        let mut result = self.dispatch_emit_inner(verb, emit, meta, identity).await?;
        if folded.is_empty() {
            return Ok(result);
        }
        let scope = std::mem::replace(&mut result.ui_scope, UiDirtyScope::None);
        let fold = self.fold_inline_interaction_verbs(verb, folded, meta, scope, identity).await;
        result.ui_scope = fold.scope;
        if let (DslValue::Null, Some(leftover)) = (&result.output, fold.leftover) {
            result.output = leftover;
        }
        result.diagnostics.extend(fold.diagnostics);
        Ok(result)
    }

    /// 🧵️ The task lane belongs to the MIGRATED publication ladder
    /// (`publish_mounted_typed_operation_unit`), which owns the per-operation cancellation
    /// lease a spawned task must be cancellable under and spends exactly one task per
    /// bounded publication unit. This unmigrated route has neither, so a task emitted here
    /// is refused by name rather than spawned unowned.
    ///
    /// 🔒️ Contract §2.3 clause 2 — hard backstop, unreachable through `ViewerApp` (its `handle`
    /// builds `Emit` solely from `ViewEmit`'s three fields, contract §2.2), but this is what
    /// makes a hand-written runtime `ArtifactApp` impl that sets `ROLE = Viewer` safe too.
    ///
    /// 🔀️ Transaction freeze (contract §5.10): while a transaction is pending on this
    /// instance, reject anything that would emit artifact mutations rather than applying or
    /// queuing it — config/draft-only and op-less dispatches are unaffected. Read-only
    /// commands (RefreshUi/ReadDocument/ContextMenu/ephemeral lanes) never reach
    /// `dispatch_emit` at all, so they are unaffected by construction.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): when any artifact mutation carries
    /// foreign steps (folded forward exactly like `ArtifactStore::replay_mutations` folds its
    /// own snapshot, via the same public `Mutation::diff`/`protocol::apply_diff` pair — the
    /// private `apply_mutation` helper lives across the crate boundary in `store`), apply
    /// NOTHING — stash a `TransactionProposalDraft` instead, drained once by `plugin_exchange`
    /// (`take_pending_transaction_proposal`) and framed as `AppFrame::TransactionProposal`
    /// rather than the ordinary `Invocation`/`Done` frame.
    ///
    /// 📝️ Draft lane — ephemeral; applied without command-log rows (never checkpoints).
    ///
    /// 🚦️ Kind discipline, enforced where the operations actually arrive: a migrated dispatch hands
    /// the reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own
    /// doc promises can only be made against the published emit, never against the admission receipt.
    ///
    /// 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
    /// — captures the resulting (possibly amended) config edit id for the command-log row below.
    ///
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): a non-empty `child_emits` means this
    /// gesture must land as ONE atomic multi-document group — routed entirely through
    /// `dispatch_emit_group` (parent ops included), never through the solitary single-store
    /// path below. See that method's own doc comment for why the two paths stay genuinely
    /// separate rather than being unified into one (the group protocol has no streamed open edit).
    ///
    /// 🧾️ An app-declared action logs under its declared kind (so an `Mutation`-kind action
    /// that happened to produce zero document operations — e.g. paste with no clipboard
    /// fragment, or a pure config-op dispatch — still gets a real row, `edit_id: None`,
    /// correctly filed under "Without Operations"); a declared command logs under its own
    /// independently declared kind; anything unresolved (registry-less test construction, an ad-hoc
    /// verb like an import-media port id) logs as `View`.
    ///
    /// 🪟️ A `View`-kind dispatch whose ONLY emission is the per-window config lane is session
    /// view state of one window (its camera pose, grid/LOD/selectable-kind options), not a
    /// command: the window-config store keeps its own per-window lane and nothing here is
    /// revertible, so a row would only pad the artifact command history with one entry per
    /// orbit tick. A `View` action that reaches the document or the shared config store still
    /// logs — `select`, whose emission is a config op, is unaffected.
    ///
    /// 🪢️ Captured before dispatch so `result_from_last_edit` can report only the operations THIS dispatch
    /// added — if a streamed tick grows the same open edit (`before_edit_id` unchanged after dispatch), these
    /// are the tail offsets into that edit's now-longer forwards/inverse; if a new edit was created
    /// instead, the offsets are moot (checked via edit identity, not reused blindly).
    ///
    /// 🕹️ Task 4: after EVERY artifact (document) dispatch, re-derive fresh topology and prune
    /// any selection/hover id no longer present — a document edit that deleted a node must not
    /// leave it lingering in another window's selection.
    ///
    /// 🧾️ One command-log entry per VCS edit — a coalesced gesture (`amended_same_edit`) grows the
    /// existing entry's `op_lines` live (see `build_history_view`), it never appends a new entry. The
    /// entry is logged BEFORE the interaction state is revalidated: a `Topology` domain refreshes the
    /// history cache, whose backfill would otherwise file the new edit under `apply` first and this
    /// dispatch would log it a second time (a committed typing run showed as two rows of one edit).
    ///
    /// ⏪️ No memory `inverse` for an edit-linked row — the VCS edit's own `Mutation::inverse`
    /// is already the real inverse, and `revertToCommand`'s edit_id branch replays that.
    async fn dispatch_emit_inner(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        Self::mint_extension_invocations(meta.instance_id, &mut emit)?;
        let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, owned_child_emits, child_preparations, interaction_writes, tasks } = emit;
        assert!(owned_child_emits.is_empty()&&owned_child_emits.capacity()==0,"applying retained children require their staged publication owner");
        assert!(child_preparations.is_empty()&&child_preparations.capacity()==0,"dispatch owns only completely prepared child operation groups");
        if let Some(fault) = tool_transaction_shape_fault(verb, transaction.as_ref(), transaction_phase, !child_emits.is_empty(), !artifact_mutations.is_empty(), artifact_mutations.iter().any(Mutation::may_emit_foreign_steps)) {
            return Err(fault);
        }
        let effects = self.stamp_load_document_effects(effects)?;
        debug_assert!(extension_invocations.is_empty(), "mint_extension_invocations drains the lane before destructuring");

        if !tasks.is_empty() {
            return Err(Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.task-lane-unmigrated-route"),
                format!(
                    "verb {verb:?} emitted {} async task(s) on the unmigrated dispatch route; the task lane is published by the migrated typed-operation ladder, which owns the cancellation lease every spawned task is retired under",
                    tasks.len()
                ),
            ));
        }

        if A::ROLE == AppRole::Viewer && !artifact_mutations.is_empty() {
            return Err(viewer_read_only_fault(verb));
        }

        if !artifact_mutations.is_empty() && self.tool_runs.freezes_local_emits() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.busy"), format!("verb {verb:?} would emit artifact mutations while a freezing tool run is non-terminal on this instance")));
        }

        if (!artifact_mutations.is_empty() || !child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
            return Err(time_travel_frozen_fault(verb));
        }

        if !artifact_mutations.is_empty() && self.pending_transaction.is_some() {
            let pending_txn_id = self.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()).unwrap_or_default();
            return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {verb:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance")));
        }

        if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
            let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
            let foreign = Vec::new();
            for op in &artifact_mutations {
                if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                let outcome = op.diff(&running);
                if !outcome.is_applicable(protocol::MergePolicy::default()) {
                    return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                }
                running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
            }
            if !foreign.is_empty() {
                let mut local_ops = Vec::with_capacity(artifact_mutations.len());
                for op in artifact_mutations.iter() {
                    local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                }
                self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
            }
        }

        let mut artifact_op_bytes = Vec::with_capacity(artifact_mutations.len());
        for op in artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut config_op_bytes = Vec::with_capacity(config_mutations.len());
        for op in config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut draft_op_bytes = Vec::with_capacity(draft_mutations.len());
        for op in draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: Vec::new() });

        if !draft_mutations.is_empty() {
            self.draft_store.dispatch(ArtifactCommand::Apply { mutations: draft_mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }

        let declared_kind = self.declared_dispatch_kind(verb);
        if !artifact_mutations.is_empty() {
            self.require_operation_emitting_kind(verb, declared_kind)?;
        }

        let config_edited = !config_mutations.is_empty();
        if config_edited {
            let config_command = ArtifactCommand::Apply { mutations: config_mutations, transaction: None };
            self.config_store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.config_store.dispatch(config_command, identity).await;
            self.config_store.set_authoring_verb(None);
            dispatched.map_err(|error| error.into_fault())?;
        }

        if !window_config_mutations.is_empty() {
            let authority = self
                .window_config_store
                .capture(meta.view_state.as_ref())
                .await?
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.window-context"), "window config emission requires one exact registered ViewModel window authority"))?;
            for mutation in window_config_mutations {
                self.window_config_store.dispatch(&authority, &meta.actor, mutation, identity).await?;
            }
            self.tool_runs.note_window_config_published();
        }
        #[cfg(test)]
        debug_assert!(tasks.is_empty());

        if !child_emits.is_empty() {
            let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, effects, events, ui_scope, meta, None, transaction, identity).await?;
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(result);
        }

        let in_transaction = transaction.is_some();
        let log_label = self.verb_label(verb);
        let vcs_commands = artifact_lane_commands(artifact_mutations, transaction, transaction_phase, self.store.open_transaction());
        if vcs_commands.is_empty() {
            let kind = match self.invocation_kind {
                Some(kind) => kind,
                None => match self.registry.get(verb).map(|definition| definition.kind) {
                    Some(kind) => kind,
                    None => match self.registry.get_command(verb) {
                        Some(command) => command.kind,
                        None => ActionKind::View,
                    },
                },
            };
            if !in_transaction && !config_edited {
                self.record_command(verb, kind, log_label, None, Vec::new(), None);
            }
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let aborting = matches!(vcs_commands.as_slice(), [ArtifactCommand::AbortTransaction { .. }]);
        let before_edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
        let (before_forwards_len, before_backwards_len) = self.store.edit_mutations().map_or((0, 0), |(f, b, _)| (f.len(), b.len()));
        for vcs_command in vcs_commands {
            self.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.store.dispatch(vcs_command, identity).await;
            self.store.set_authoring_verb(None);
            match dispatched {
                Ok(receipt) => self.record_dispatch_receipt(receipt),
                Err(vcs::VcsError::Rejected { policy, messages }) => return Err(self.record_rejected_dispatch(policy, messages).await),
                Err(error) => return Err(error.into_fault()),
            }
            .await;
        }
        if aborting {
            self.revalidate_interaction_on_document_change(meta, identity).await?;
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            self.retire_displaced_document_rows();
            return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let amended_same_edit = before_edit_id.is_some() && self.store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_edit_id.as_ref();
        if !amended_same_edit {
            if let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) {
                let kind = match self.invocation_kind {
                    Some(kind) => kind,
                    None => match self.registry.get(verb) {
                        Some(def) => def.kind,
                        None => match self.registry.get_command(verb) {
                            Some(def) => def.kind,
                            None => ActionKind::Mutation,
                        },
                    },
                };
                self.record_command(verb, kind, log_label, Some(edit_id), Vec::new(), None);
            }
        }
        self.revalidate_interaction_on_document_change(meta, identity).await?;
        self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
        let tail_offset = if amended_same_edit { (before_forwards_len, before_backwards_len) } else { (0, 0) };
        Ok(self.result_from_last_edit(verb, meta, effects, events, ui_scope, tail_offset).await)
    }

    /// 🎗️ The store half of a migrated emission that closes a streamed tool transaction without new operations (design
    /// §15): an abort reverts the transaction's open edit and retires its history row, a commit with no last operations
    /// closes the edit (its announcement leaves with the run's next flush). Either is consumed without suspending, so the
    /// one-page publication continues with the emission's other lanes; a closing commit with operations and every stream
    /// are left to the batched route. A commit carrying composed-child emissions keeps its ref: the child group stamps it
    /// on every member op (design §12), so a child-only tool transaction is still one transaction row.
    fn close_streamed_transaction_unit(&mut self, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        let Some(transaction) = emit.transaction.as_ref() else { return Ok(()) };
        let abort = emit.transaction_phase == TransactionPhase::Abort;
        if !abort && (emit.transaction_phase == TransactionPhase::Stream || !emit.artifact_mutations.is_empty()) {
            return Ok(());
        }
        let open = self.store.open_transaction().is_some_and(|open| open.transaction.id == transaction.id);
        let transaction_id = transaction.id.clone();
        if abort || emit.child_emits.is_empty() {
            emit.transaction = None;
            emit.transaction_phase = TransactionPhase::Commit;
        }
        if !open {
            return Ok(());
        }
        if abort {
            self.store.abort_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
            self.retire_displaced_document_rows();
        } else {
            self.store.commit_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// 🚧️ The typed refusal of a batched artifact publication while a tool transaction it does not carry is open on the
    /// document store (the store refuses its admission too, untyped).
    fn open_transaction_admission_fault(&self, transaction: Option<&protocol::TransactionRef>) -> Option<Fault> {
        let open = self.store.open_transaction()?;
        transaction.is_none_or(|transaction| transaction.id != open.transaction.id).then(|| vcs::VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() }.into_fault())
    }

    /// 🌱️ Absorbs every `store::os_store::composition_coordinator::GroupReceipt::created_children` entry into the live
    /// `self.children` map — B2 flagged that a `ChildGenesis`-created member has no pre-existing
    /// caller-held reference the way an already-registered child does, so without this it would
    /// be silently dropped the moment `dispatch_group` returns, making `ChildGenesis` pointless
    /// (see `GroupReceipt::created_children`'s own doc comment). The slot name is recovered from
    /// `self.composition`'s own graph (`dispatch_group`'s phase 2 already called `insert_owns`
    /// for every genesis child before returning the receipt, so `slot_of` is always populated
    /// here). A free-standing method (not inlined into `dispatch_emit_group`) so it is directly
    /// unit-testable against a synthetic `created_children` list, independent of the `ChildEmit`
    /// wire path (which never emits a `ChildGenesis` itself today).
    pub(crate) async fn absorb_created_children(&mut self, created_children: Vec<(ArtifactRef, M)>) -> Result<(), Fault> {
        for (target, mut member) in created_children {
            let publication_generation = self.admit_child_content_publication()?;
            let slot = self.composition.graph().await.slot_of(&target.artifact_id).await.ok_or_else(|| plugin_sdk_fault("created child has no exact target ownership slot"))?.to_owned();
            let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("created child projection failed: {error}")))?;
            let child_id = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.artifact_id == target.artifact_id && fields.artifact_kind == target.dialect.artifact_kind && fields.standard == target.dialect.standard && fields.subset == target.dialect.subset).map(|(_, fields)| fields.child_id.to_owned()).ok_or_else(|| plugin_sdk_fault("created child is not declared by the published parent"))?;
            ChildContentView::hash(MemberKeyRef::root(&slot, &child_id))?;
            let admission = self.children.admit(&(slot.clone(), child_id.clone()))?;
            let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
            let owner = store::OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() };
            member.set_owner(Some(owner.clone())).await;
            self.children.insert_admitted(admission, target, owner, member);
            self.publish_child_content_member(publication_generation, &slot, &child_id).await?;
        }
        Ok(())
    }

    /// 🧩️ `dispatch_emit`'s composite-gesture branch, taken whenever `emit.child_emits` is
    /// non-empty. Routes `artifact_mutations` (the parent's own ops) plus every `ChildEmit`
    /// through `store::os_store::composition_coordinator::CompositionCoordinator::dispatch_group` as ONE atomic multi-document
    /// gesture, so a single user action spanning a parent and N owned children still produces ONE
    /// `UndoGroup`: `inverse_group.member_edits` names every touched document (parent included),
    /// and `dispatch_action`'s history-action arm routes an undo/redo of a group-tailed edit
    /// through `CompositionCoordinator::undo_group`/`redo_group` — see that arm's own doc comment.
    /// Deliberately narrower than the solitary (zero-children) path above: `dispatch_group`'s
    /// phase 2 always issues a plain `Apply` on every touched member (there is no group-aware
    /// streamed open edit), so a composite gesture with children is
    /// always ONE described edit per member, never folded across ticks — only the solitary path
    /// streams a transaction's ticks into one open edit. `inverse_group.mutations`/`.inverse_mutations`
    /// are best-effort for CHILD members specifically: real per-op inverse bytes are retrievable
    /// only through `ArtifactStore<P, Mutation>`'s own inherent `edit_mutations()`, and each
    /// child's concrete `P`/`Mutation` is erased behind the generic `M: SpaceMember` — the REAL
    /// reversal mechanism for a child member is `CompositionCoordinator::undo_group` calling
    /// `undo()` directly on the live child store (driven by `inverse_group.member_edits`, not by
    /// replaying these bytes).
    /// 🚦️ Kind discipline where the operations actually arrive. A migrated dispatch hands its
    /// reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own doc
    /// promises can only be made against the published emit, never against the admission receipt the
    /// caller gets back. An unresolved verb (registry-less construction, an ad-hoc port id) is not a
    /// declared `View`, so it is not refused here.
    /// 🚫️async: E1 pure classification over an already-resolved kind
    fn require_operation_emitting_kind(&self, verb: &str, declared_kind: Option<ActionKind>) -> Result<(), Fault> {
        match declared_kind {
            Some(kind @ (ActionKind::View | ActionKind::Shell)) => Err(Fault::from(format!("{kind:?}-kind command '{verb}' must not emit operations"))),
            _ => Ok(()),
        }
    }

    /// 🏷️ The kind a verb dispatches under: the in-flight invocation's own kind when one is
    /// mounted, otherwise the action declaration, otherwise the command declaration.
    /// 🚫️async: E1 pure registry lookup
    fn declared_dispatch_kind(&self, verb: &str) -> Option<ActionKind> {
        match self.invocation_kind {
            Some(kind) => Some(kind),
            None => self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind)),
        }
    }

    /// 🧩️ Commits an agent transaction that carries owned-child op groups as ONE composite gesture
    /// (`dispatch_emit_group`, the same path the shell lane's child-group verbs take) whose group identity is
    /// `txn_id` on every member it touches, so the gateway's `TransactionUndo{group_id: txn_id}` moves them all.
    /// The parent's tail edit (when the group touched the parent) carries the prepared origin like a solitary
    /// commit does. Returns the parent's edit id, else the first touched child's.
    async fn commit_transaction_group(&mut self, txn_id: &str, ops: Vec<A::Mutation>, children: Vec<ChildEmit>, origin: protocol::MutationOrigin, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let parent_edits_before = self.store.envelope().vcs.edits.len();
        let result = self
            .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, Vec::new(), Vec::new(), UiDirtyScope::Full, meta, Some(txn_id.to_string()), None, identity)
            .await
            .map_err(|fault| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", fault.message))?;
        if self.store.envelope().vcs.edits.len() > parent_edits_before {
            self.store.stamp_tail_origin(origin).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
            return Ok(self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default());
        }
        Ok(result.inverse_group.member_edits.first().map(|edit| edit.edit_id.clone()).unwrap_or_default())
    }

    /// ↩️ `transaction_undo`/`transaction_redo` for a composing instance: moves every member (parent and
    /// owned children) whose tail carries `group_id` through `CompositionCoordinator::undo_group`/`redo_group`,
    /// republishing each moved child's content like the shell lane's group history does. A group no member
    /// carries is refused by name, exactly like the solitary path.
    async fn transaction_group_history(&mut self, action: &str, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.admit_child_content_publication_span(self.children.len())?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.children.entries_physical().map(|entry| entry.reference.clone()).collect();
        let mut members: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.children.entries_mut_physical()) {
            members.push((reference, &mut entry.member));
        }
        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut members, group_id, identity).await };
        drop(members);
        self.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.admit_child_content_publication()?;
            self.publish_member_content(publication_generation, key.borrowed()).await?;
        }
        if report.undone.is_empty() {
            let skipped = report.skipped.iter().map(|(reference, error)| format!("{} ({error})", reference.artifact_id)).collect::<Vec<_>>().join(", ");
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-tail"), format!(
                "transaction_{action}: no member of this instance carries group {group_id:?} at its {} tail{}",
                if action == "undo" { "applied" } else { "redo" },
                if skipped.is_empty() { String::new() } else { format!(" (skipped: {skipped})") }
            )));
        }
        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        Ok(())
    }

    /// 🚧️ `ArtifactEnvelope.dialect` stays `Option<ArtifactDialect>` per B2's own DEFERRED
    /// scope decision (`📓️wave1-reports/b2-store-composition-report.md`) — this fallback (a
    /// synthetic "native" dialect from the app's own `DOCUMENT_SCHEMA`) is only ever consulted
    /// when no real dialect was ever threaded through `create_document_envelope`, and only
    /// matters for the OWNERSHIP-GRAPH bookkeeping `dispatch_group` needs a real `ArtifactRef`
    /// for, not for any wire/codec decision.
    ///
    /// 🛡️ Keys are unique and `self.children` is not resized until after `dispatch_group`, so
    /// these entry pointers stay valid and non-aliasing for the simultaneous child borrows
    /// `dispatch_group` requires.
    ///
    /// 🎛️ `meta.actor` is honored; a group is always one plain edit per member.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md` §dispatch_group,
    /// and `📓️terra-dispatch-group-split-report.md` for the fix): `store::os_store::composition_coordinator::CompositionCoordinator::
    /// dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>` now takes SEPARATE type
    /// parameters for `parent` and `children` — `self.store` (`ArtifactStore<A::Snapshot,
    /// A::Mutation>`, this app's own fixed type, only ever mutated) unifies as `Mp`; `dispatches`
    /// (`Vec<(&mut M, ChildDispatch<'_>)>`, this composition's CHILD-kind enum, the only side ever
    /// genesis-constructed) unifies as `Mc`. Dyn-free (O1-compliant) and now type-checks.
    ///
    /// 🌱️ Absorb any freshly-created children into the live map — extracted into its own
    /// method (below) both so it is directly unit-testable and so it needs no revisiting once
    /// a genesis-emitting `Emit` constructor lands (`genesis` is always `Vec::new()` on THIS
    /// call today — a `ChildEmit` only ever targets an ALREADY-live child).
    ///
    /// 🪆️ Child-lane replication: a composed member's edit is a real event of a real document,
    /// and the parent's backbone is the only endpoint it can cross. Announced here, at the one
    /// choke point every child edit lands at, so a replica folds it into ITS member of the
    /// same lane instead of diverging (ticket 26/09/19 `📓️flow.md` §5.3).
    ///
    /// 🪪️ The PARENT's handle must be the same value its own `KernelMutation.document` carries
    /// (`ArtifactHandle(meta.instance_id)`), not `artifact_handle_of(parent_id)` — otherwise one
    /// `InvocationResult` identifies the same document two different ways and any consumer
    /// correlating `mutations` with `member_edits` by handle silently fails to match the parent.
    /// Children keep the content-addressed `artifact_handle_of`, which is what their own
    /// `KernelMutation.document` uses.
    ///
    /// 🧱️ Parent side: `dispatch_wire` landed a REAL edit on `self.store` above (whenever
    /// `parent_ops` was non-empty), so its tail edit's real forward/inverse mutations are
    /// available exactly like the solitary path's `result_from_last_edit`.
    ///
    /// 🧩️ Child side: real per-op inverse bytes are not retrievable through the object-safe
    /// `SpaceMember` seam (see this fn's own doc comment) — each touched child's edit surfaces
    /// as ONE best-effort `KernelMutation` bundling all of that child's ops as its diff
    /// payload, with an empty (but validly-encoded, decodes to zero ops) inverse payload.
    /// `dispatch_group`'s phase 2 walks `children` (built here in the SAME order as
    /// `child_emits`) by index, skipping empty-ops entries — so the non-empty-ops subsequence
    /// of `child_emits`, in order, lines up 1:1 with the non-parent tail of `member_edits`.
    ///
    /// 🧾️ One command-log row for the whole group, carrying every touched child's edit in `child_edit_ids` (see
    /// `CommandLogEntry::child_edit_ids`'s own doc comment); a config-lane edit never joins it (design §20.13, L4).
    #[allow(clippy::too_many_arguments)]
    async fn dispatch_emit_group(
        &mut self,
        verb: &str,
        artifact_mutations: &[A::Mutation],
        child_emits: &[ChildEmit],
        effects: Vec<Effect>,
        events: Vec<AppEvent>,
        ui_scope: UiDirtyScope,
        meta: &ActionMeta,
        group_id: Option<String>,
        transaction: Option<protocol::TransactionRef>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        if !artifact_mutations.is_empty() {
            self.require_operation_emitting_kind(verb, self.declared_dispatch_kind(verb))?;
        }
        let touched_child_count = child_emits.iter().filter(|child| !child.ops.is_empty()).count();
        let child_publication_generation = (touched_child_count != 0).then(|| self.admit_child_content_publication()).transpose()?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("composition requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let parent_ops=artifact_mutations.iter().map(::protocol::OpBinary::encode_op).collect::<Result<Vec<_>,_>>().map_err(|error|plugin_sdk_fault(error.to_string()))?;

        let mut seen_keys = ::std::collections::HashSet::with_capacity(child_emits.len());
        let mut child_ptrs: Vec<(*mut M, ChildDispatch<'_>)> = Vec::with_capacity(child_emits.len());
        for child_emit in child_emits {
            let key = (child_emit.slot.clone(), child_emit.child_id.clone());
            if !seen_keys.insert(key.clone()) {
                return Err(plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted duplicate ChildEmit for slot {:?} child {:?}", child_emit.slot, child_emit.child_id)));
            }
            let entry = self.children.get_mut(&key).ok_or_else(|| {
                plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted a ChildEmit for slot {:?} child {:?} with no live child store — call VcsArtifactApp::open_child/register_child first", child_emit.slot, child_emit.child_id))
            })?;
            let target = entry.reference.clone();
            let member_ptr: *mut M = &mut entry.member;
            child_ptrs.push((member_ptr, ChildDispatch::borrowed(target, &child_emit.ops, &child_emit.op_schema, &child_emit.labels)));
        }
        let child_refs: Vec<ArtifactRef> = child_ptrs.iter().map(|(_, dispatch)| dispatch.child.clone()).collect();
        let child_member_ptrs: Vec<*mut M> = child_ptrs.iter().map(|(member, _)| *member).collect();
        let mut dispatches: Vec<(&mut M, ChildDispatch<'_>)> = child_ptrs.into_iter().map(|(ptr, dispatch)| (unsafe { &mut *ptr }, dispatch)).collect();

        let group_meta = GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction };
        self.store.set_authoring_verb(Some(verb.to_string()));
        let receipt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_meta, identity).await;
        self.store.set_authoring_verb(None);
        let receipt = receipt.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(dispatches);
        self.cache = None;
        if let Some(publication_generation) = child_publication_generation {
            let mut next = ChildContentView::clone(&self.child_content_root);
            for child_emit in child_emits.iter().filter(|child| !child.ops.is_empty()) {
                let captured = match self.children.get(&(child_emit.slot.clone(), child_emit.child_id.clone())) {
                    Some(entry) => next.with_member(MemberKeyRef::root(&child_emit.slot, &child_emit.child_id), &entry.reference, &entry.member).await,
                    None => Err(plugin_sdk_fault("committed child group lost its exact live member before immutable-root capture")),
                };
                match captured {
                    Ok(captured) => next = captured,
                    Err(fault) => {
                        if !next.is_empty() {
                            self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(next, false));
                        }
                        let mut members: Vec<(&ArtifactRef, &mut M)> = child_refs.iter().zip(child_member_ptrs.iter().copied()).map(|(reference, member)| (reference, unsafe { &mut *member })).collect();
                        let compensation = CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, &receipt.invocation_id, identity).await;
                        drop(members);
                        if compensation.undone.len() != receipt.member_edits.len() {
                            return Err(plugin_sdk_fault(format!("child root capture failed ({}) and group compensation moved {}/{} members", fault.message, compensation.undone.len(), receipt.member_edits.len())));
                        }
                        return Err(fault);
                    }
                }
            }
            let previous = std::mem::replace(&mut *self.child_content_root, next);
            if !previous.is_empty() {
                self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
            }
            self.child_content_generation = publication_generation;
        }

        self.absorb_created_children(receipt.created_children).await?;

        let announced_lanes: Vec<(String, String)> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).map(|child_emit| (child_emit.slot.clone(), child_emit.child_id.clone())).collect();
        self.announce_member_tail_edits(&announced_lanes).await?;

        let invocation_id = InvocationId(receipt.invocation_id.clone());
        let mut member_edits: Vec<EditRef> = Vec::with_capacity(receipt.member_edits.len());
        for (reference, edit_id) in receipt.member_edits.iter() {
            let document = if reference.artifact_id == parent_id { ArtifactHandle(meta.instance_id as u128) } else { artifact_handle_of(&reference.artifact_id).await };
            member_edits.push(EditRef { document, edit_id: edit_id.clone() });
        }

        let mut mutations: Vec<KernelMutation> = Vec::new();

        let parent_touched = receipt.member_edits.iter().any(|(reference, _)| reference.artifact_id == parent_id);
        if parent_touched {
            let operation_wires = self.store.authored_edit_operation_bytes(identity).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if let (Some((forward_wires, inverse_wires)), Some((forwards, _, mutation_meta))) = (operation_wires, self.store.edit_mutations()) {
                let inverse_payload = identity.encode(|control| ::protocol::os_spr::io::binary::operation_sequence::encode(&inverse_wires, control)).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                let document = ArtifactHandle(meta.instance_id as u128);
                let schema = A::DOCUMENT_SCHEMA.to_string();
                for ((index, _forward), forward_wire) in forwards.iter().enumerate().zip(forward_wires) {
                    let entry = mutation_meta.get(index);
                    let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                    let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                    let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                    let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                    let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                    mutations.push(KernelMutation {
                        id: mutation_id.clone(),
                        document,
                        base_version,
                        invocation_id: invocation_id.clone(),
                        diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: forward_wire },
                        inverse: InverseMutation {
                            target_mutation: mutation_id,
                            inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: identity.encode(|control| control.copy_bytes(&inverse_payload)).map_err(|error| plugin_sdk_fault(error.to_string()))? },
                            base_version,
                            dependencies: Vec::new(),
                            undo_policy,
                        },
                        dependencies: Vec::new(),
                        author,
                        timestamp,
                    });
                }
            }
        }

        let child_member_edits: Vec<&(ArtifactRef, String)> = receipt.member_edits.iter().filter(|(reference, _)| reference.artifact_id != parent_id).collect();
        let touched_child_emits: Vec<&ChildEmit> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).collect();
        debug_assert_eq!(touched_child_emits.len(), child_member_edits.len(), "dispatch_group's per-child edit order must match the non-empty-ops subsequence of child_emits");
        let mut child_edit_ids: Vec<String> = Vec::with_capacity(child_member_edits.len());
        for (child_emit, (reference, edit_id)) in touched_child_emits.into_iter().zip(child_member_edits) {
            child_edit_ids.push(edit_id.clone());
            let document = artifact_handle_of(&reference.artifact_id).await;
            let mutation_id = MutationId(edit_id.clone());
            mutations.push(KernelMutation {
                id: mutation_id.clone(),
                document,
                base_version: ArtifactVersion(0),
                invocation_id: invocation_id.clone(),
                diff: ArtifactDiff { schema: child_emit.op_schema.clone(), payload: protocol::encode_ops_vec(&child_emit.ops) },
                inverse: InverseMutation {
                    target_mutation: mutation_id,
                    inverse_diff: ArtifactDiff { schema: SchemaId(format!("{}.inverse", child_emit.op_schema.0)), payload: protocol::encode_ops_vec(&[]) },
                    base_version: ArtifactVersion(0),
                    dependencies: Vec::new(),
                    undo_policy: UndoPolicy::ExactBaseOnly,
                },
                dependencies: Vec::new(),
                author: ActorId(meta.actor.clone()),
                timestamp: HybridLogicalTimestamp::new(0, 0),
            });
        }

        let mutation_ids: Vec<MutationId> = mutations.iter().map(|mutation| mutation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|mutation| mutation.inverse.clone()).collect();

        let parent_edit_id = receipt.member_edits.iter().find(|(reference, _)| reference.artifact_id == parent_id).map(|(_, edit_id)| edit_id.clone());
        let kind = self.registry.get(verb).map_or(ActionKind::Mutation, |def| def.kind);
        let label = self.verb_label(verb);
        self.record_command(verb, kind, label, parent_edit_id, child_edit_ids, None);

        Ok(InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        })
    }

    /// 🕸️ Re-syncs `self.composition`'s ownership/link graph from the parent's own live
    /// `ArtifactRefs` projection (`child_refs()`/`links()`) — the mechanism
    /// `store::os_store::composition_coordinator::CompositionGraph::sync_member`'s own doc comment names as the required follow-up
    /// "after any dispatch that might have changed an artifact's own `ArtifactRefs`". Deliberately
    /// NOT called automatically by `dispatch_emit`/`dispatch_emit_group`: doing so would force
    /// `A::Snapshot: store::ArtifactRefs` onto EVERY `ArtifactApp` impl in the workspace (today
    /// zero of them implement it — `ArtifactRefs`-bearing snapshots are new, later-wave-scoped
    /// schema work), breaking every existing plugin's compile for a graph no leaf artifact needs
    /// synced. A composed artifact's own app calls this explicitly wherever its `handle` might
    /// have changed a `#[child(...)]`/`#[link_slot(...)]` field — see B2's own
    /// `📓️wave1-reports/b2-store-composition-report.md` sharedFileRequest #6.
    pub async fn sync_composition_graph(&mut self) -> Result<(), Fault>
    where
        A::Snapshot: store::ArtifactRefs,
    {
        let snapshot = self.store.snapshot().map_err(|error| error.into_fault())?;
        let parent_id = self.store.envelope().id.clone();
        self.composition.graph_mut().await.sync_member(&parent_id, &snapshot).await.map_err(|error| plugin_sdk_fault(error.to_string()))
    }

    /// ↩️ Task 4: group-aware undo/redo. Routes through
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group` across `self.store` (the parent)
    /// plus every LIVE child in `self.children` — the coordinator itself filters to members whose
    /// tail actually carries `group_id` (a child never touched by this particular gesture, or one
    /// whose own `undo()`/`redo()` call fails, is SKIPPED, not aborted — see `GroupUndoReport`'s
    /// own doc comment for why abort-all would be actively harmful: one collaborator's foreign or
    /// failed child edit must never permanently freeze this parent's undo stack). This generalizes
    /// the plain single-store path's existing benign `NothingToUndo`/`NothingToRedo`/`ForeignEdit`
    /// collapse to the multi-member case. `self.store` (the parent) and `self.children` (this
    /// composition's CHILD-kind type) are passed as SEPARATE arguments now — the same `Mp`/`Mc`
    /// split `dispatch_group` uses (see its own doc comment) — rather than one homogeneous list,
    /// so no more caller-side ordering dance either: `undo_group`/`redo_group` order parent vs.
    /// children internally to match `dispatch_group`'s apply order, and (per their own doc
    /// comments) treat every member independently regardless of order anyway.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md`
    /// §undo_group/redo_group, and `📓️terra-dispatch-group-split-report.md` for the fix):
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group<Mp: SpaceMember, Mc:
    /// SpaceMember>` now take `parent`/`children` as separate arguments, so `self.store`
    /// (`ArtifactStore<A::Snapshot, A::Mutation>`) unifies as `Mp` and `self.children`'s
    /// member type (`M`, this composition's CHILD-kind type) unifies as `Mc` independently.
    ///
    /// 🧾️ Nothing actually moved (every member was foreign/failed) — benign collapse,
    /// mirroring `NothingToUndo`/`NothingToRedo`/`ForeignEdit` just above: NOT logged
    /// (never touched any store), but the skip diagnostics still ride along so the caller
    /// can see WHY nothing happened instead of silently no-op'ing.
    ///
    /// 🧾️ Append-only, same as the plain path: undo/redo are pure cursor motion, never
    /// logged with an `edit_id` — `child_edit_ids` records every CHILD member this group call
    /// actually touched.
    async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.admit_child_content_publication_span(self.children.len())?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.children.entries_physical().map(|entry| entry.reference.clone()).collect();

        let mut children: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.children.entries_mut_physical()) {
            children.push((reference, &mut entry.member));
        }

        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut children, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut children, group_id, identity).await };
        drop(children);
        self.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.admit_child_content_publication()?;
            self.publish_member_content(publication_generation, key.borrowed()).await?;
        }

        let diagnostics: Vec<semio_framework_diagnostic::Diagnostic> = report
            .skipped
            .iter()
            .map(|(reference, error)| {
                semio_framework_diagnostic::Diagnostic::error("composition.group-history.skipped-member", semio_framework_diagnostic::TextSpan::default(), format!("{action} skipped member {} ({error})", reference.artifact_id))
            })
            .collect();

        if report.undone.is_empty() {
            let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
            result.diagnostics = diagnostics;
            return Ok(result);
        }

        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        let child_edit_ids: Vec<String> = report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id).map(|(_, edit_id)| edit_id.clone()).collect();
        if let Some(last) = self.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }

        let mut result = Self::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await;
        result.diagnostics = diagnostics;
        Ok(result)
    }

    /// 🧾️ `domain=granularity:id,id` for one `InteractionState`'s selection half — the compact witness
    /// the persist round-trip check below compares, so a lost selection names its own domain instead of
    /// reaching the user as a silently empty panel.
    fn interaction_selection_witness(state: &protocol::InteractionState) -> String {
        state.selection.iter().map(|(domain, selection)| format!("{domain}={}:{}", selection.granularity, selection.ids.join(","))).collect::<Vec<_>>().join(";")
    }

    /// 🕹️ Combines the persisted-local `interaction_store` snapshot with the ephemeral
    /// `interaction_hover` map into one `InteractionState` — the "app-side source" a host's presence
    /// heartbeat reads for THIS app instance before calling `assemble_presence_interaction`
    /// (`🏪️store/🔄️sync/🦀️.rs`, wave 2a) with it plus this app's declared hover/selection
    /// specs (from `AppDefinition.interactions`, already available wherever a heartbeat is built).
    pub async fn interaction_state(&self) -> protocol::InteractionState {
        let mut state = self.interaction_selection_snapshot();
        state.hover = self.interaction_hover.clone();
        state
    }

    /// 🕹️ The persisted-local selection half, with a store read failure NAMED instead of silently
    /// collapsing into an empty `InteractionState`. Every reader treats "no selection" and "the
    /// selection could not be read" identically, so a swallowed read reaches the user as a panel that
    /// renders the empty-document summary on a live pick — measured in the browser on
    /// `framework.panel.inspection` (ticket 26/09/02/PUZZLE-3D-END-TO-END, wave B6: 25 full refreshes
    /// over 73 s kept the pre-pick body hash while the same turn's leftover `InteractionView` carried
    /// the picked id).
    fn interaction_selection_snapshot(&self) -> protocol::InteractionState {
        let mut state = match self.interaction_store.snapshot() {
            Ok(state) => state,
            Err(error) => {
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] interaction-store snapshot unavailable, selection read as empty: {error}"));
                protocol::InteractionState::default()
            }
        };
        if let Some(overlay) = &self.interaction_leftover_selection {
            for (domain, selection) in &overlay.selection {
                let empty = state.selection.get(domain).map(|current| current.ids.is_empty()).unwrap_or(true);
                if empty && !selection.ids.is_empty() {
                    state.selection.insert(domain.clone(), selection.clone());
                    if let Some(mode) = overlay.active_mode.get(domain) {
                        state.active_mode.insert(domain.clone(), *mode);
                    }
                    if let Some(granularity) = overlay.active_granularity.get(domain) {
                        state.active_granularity.insert(domain.clone(), granularity.clone());
                    }
                }
            }
            let leftover_ids: Vec<String> = overlay.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
            let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
            if vortex_empty && !leftover_ids.is_empty() {
                let granularity = overlay
                    .active_granularity
                    .get("vortex")
                    .cloned()
                    .or_else(|| overlay.selection.get("vortex").map(|selection| selection.granularity.clone()))
                    .filter(|granularity| !granularity.is_empty())
                    .unwrap_or_else(|| "object".to_string());
                state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: leftover_ids, anchor_id: None });
                if let Some(mode) = overlay.active_mode.get("vortex") {
                    state.active_mode.insert("vortex".to_string(), *mode);
                }
                state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
            }
        }
        let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
        if vortex_empty && !self.interaction_leftover_ids.is_empty() {
            let granularity = state.active_granularity.get("vortex").cloned().filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        state
    }

    /// 🌳️ Resolves `def`'s `DomainTopology` for the current document — the CLOSURE/RANGE-ARITHMETIC
    /// use (always a concrete, possibly-empty topology; safe for `next_selection`/`next_hover`, which
    /// degrade gracefully to single-target over an empty one). `HierarchyProvider::Topology` calls the
    /// app's own `interaction_topology`; `HierarchyProvider::UiTree` reads the cache `render`'s
    /// post-processing pass populates from the last rendered tree; `HierarchyProvider::Flat` is
    /// trivially empty (no structure by declaration); `HierarchyProvider::PathDelimited` self-derives
    /// a topology from `known_ids` (the ids already involved in this dispatch — the current batch's
    /// targets plus whatever was already selected/hovered) by splitting each on `delimiter` — an
    /// INHERENT approximation (only ids the interaction machine has already seen are represented,
    /// since the wrapper has no document access of its own for an arbitrary id space); an app needing
    /// a real full-universe topology for a delimited id scheme declares `HierarchyProvider::Topology`
    /// and implements `interaction_topology` instead.
    async fn resolve_domain_topology(&mut self, def: &InteractionDefinition, known_ids: impl Iterator<Item = String>) -> Result<protocol::DomainTopology, Fault> {
        match &def.hierarchy {
            protocol::HierarchyProvider::Flat => Ok(protocol::DomainTopology::default()),
            protocol::HierarchyProvider::Topology => Ok(self.app_interaction_topology().await?.domains.get(&def.id).cloned().unwrap_or_default()),
            protocol::HierarchyProvider::UiTree => Ok(self.interaction_ui_topology.get(&def.id).cloned().unwrap_or_default()),
            protocol::HierarchyProvider::PathDelimited { delimiter } => Ok(derive_path_delimited_topology(def, delimiter, known_ids).await),
        }
    }

    /// 🗺️ `A::interaction_topology` for the LIVE document generation, built at most once per generation
    /// — see [`Self::interaction_topology_memo`] for why one pick used to build it twice.
    async fn app_interaction_topology(&mut self) -> Result<&protocol::InteractionTopology, Fault> {
        self.refresh_cache().await?;
        let key = (self.store.generation(), self.config_store.generation());
        if self.interaction_topology_memo.as_ref().map(|(cached, _)| *cached) != Some(key) {
            let render_operation = self.live_render_operation();
            let parent_document_id = self.store.envelope().id.clone();
            let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.instance_operation_owner.clone());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            let topology = A::interaction_topology(&doc, &cfg).await.map_err(|error| plugin_sdk_fault(error.into_message()))?;
            self.interaction_topology_memo = Some((key, topology));
        }
        Ok(&self.interaction_topology_memo.as_ref().expect("memo filled above").1)
    }

    /// 🧹 Builds the `InteractionTopology` `validate_state`'s PRUNING (existence-check) use needs —
    /// deliberately DIFFERENT from `resolve_domain_topology`'s closure/range use: a `Flat` domain is
    /// OMITTED from the map entirely (not inserted as an empty topology), because `validate_state`
    /// treats a missing domain entry as "no membership information, keep every id" while an EMPTY
    /// `DomainTopology` would mean "no id is a member" and prune every `Flat` selection on every
    /// dispatch — `Flat` genuinely has no structure to check staleness against, by declaration; an app
    /// wanting deleted-node pruning for a nominally-flat domain declares `HierarchyProvider::Topology`
    /// with one root `TopologyNode` per valid id instead.
    ///
    /// 🧹️ On the DOCUMENT-CHANGE pass, an EMPTY topology for a domain that still carries a
    /// selection is not "no information": it is the app answering that none of those ids
    /// exist any more — the exact case this prune exists for (every id deleted). Skipping it
    /// left `validate_state` with no existence set to check against, so the stale ids
    /// survived. A `Pick` pass keeps the old rule: an app that publishes no topology at all
    /// has not made a statement about existence, and its hover must not be pruned.
    async fn build_full_interaction_topology(&mut self, state: &protocol::InteractionState, origin: InteractionRevalidateOrigin) -> Result<protocol::InteractionTopology, Fault> {
        let defs: Vec<InteractionDefinition> = self.registry.interactions().await.cloned().collect();
        let mut domains = BTreeMap::new();
        for def in &defs {
            if matches!(def.hierarchy, protocol::HierarchyProvider::Flat) {
                continue;
            }
            let selected_ids = state.selection.get(&def.id).map(|selection| selection.ids.clone()).unwrap_or_default();
            let hovered_ids = state.hover.get(&def.id).map(|hover| hover.ids.clone()).unwrap_or_default();
            let checkable = origin == InteractionRevalidateOrigin::DocumentChange && !selected_ids.is_empty();
            let topology = self.resolve_domain_topology(def, selected_ids.into_iter().chain(hovered_ids)).await?;
            if topology.ordered.is_empty() && !checkable {
                continue;
            }
            domains.insert(def.id.clone(), topology);
        }
        Ok(protocol::InteractionTopology { domains })
    }

    /// 🧹 `validate_state` against fresh topology, then persists the result: hover into the ephemeral
    /// `interaction_hover` map, selection/mode/granularity into `interaction_store` via
    /// `ApplyInLane{lane: HistoryLane::Interaction}` — but ONLY when the persisted half actually
    /// changed, so a no-op revalidation (the common case for `revalidate_interaction_on_document_change`,
    /// called after every artifact dispatch even when nothing was selected) never mints an empty edit.
    ///
    /// 🕳️ `origin` decides whether a lost id is a DEFECT or the mechanism working: a
    /// [`InteractionRevalidateOrigin::DocumentChange`] pass exists precisely to prune ids the document
    /// no longer has, while a [`InteractionRevalidateOrigin::Pick`] pass carries ids a user or an app
    /// just named and may never drop them silently — see [`interaction_selection_loss_v1`].
    async fn revalidate_and_persist_interaction_state(&mut self, combined: protocol::InteractionState, _meta: &ActionMeta, origin: InteractionRevalidateOrigin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<protocol::InteractionState, Fault> {
        let mut outlines: Vec<protocol::InteractionOutline> = Vec::new();
        for def in self.registry.interactions().await {
            outlines.push(def.outline().await);
        }
        let topology = self.build_full_interaction_topology(&combined, origin).await?;
        let validated = protocol::validate_state(&outlines, &topology, &combined).await;
        let validated = if origin == InteractionRevalidateOrigin::DocumentChange {
            let declared: Vec<String> = self.registry.interactions().await.map(|def| def.id.clone()).collect();
            let (pruned, overlay, leftover_ids) = interaction_after_document_change_v1(&validated, &declared, self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids);
            self.interaction_leftover_selection = overlay;
            self.interaction_leftover_ids = leftover_ids;
            pruned
        } else {
            validated
        };
        self.interaction_hover = validated.hover.clone();
        let persisted_before = self.interaction_store.snapshot().unwrap_or_default();
        let persisted = protocol::InteractionState { selection: validated.selection, hover: BTreeMap::new(), active_mode: validated.active_mode, active_granularity: validated.active_granularity };
        let dispatched = Self::interaction_selection_witness(&combined);
        let requested = Self::interaction_selection_witness(&persisted);
        let minted = persisted != persisted_before;
        if minted {
            self.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(persisted.clone())], lane: HistoryLane::Interaction, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }
        if origin == InteractionRevalidateOrigin::Pick {
            let readback = Self::interaction_selection_witness(&self.interaction_selection_snapshot());
            if let Some(reason) = interaction_selection_loss_v1(&dispatched, &requested, &readback, minted) {
                report_interaction_selection_loss(reason, &dispatched, &requested, &readback, &topology);
            }
        }
        Ok(persisted)
    }

    /// 🕹️ Task 4: called after every artifact (document) dispatch so an id deleted elsewhere in the
    /// document drops out of selection/hover automatically. Skips the topology/validate work
    /// entirely when the app declares no interaction domains at all (the overwhelming majority of
    /// dispatches, for an app mid-migration or with no interactions yet).
    /// 🧹️ The prune covers the leftover cache the guest lays over an empty store selection too
    /// ([`interaction_after_document_change_v1`]), so a whole-document replacement cannot leave the
    /// replaced document's pick standing in `interaction.selection(..)` — which is what every app
    /// command that reads a selection (`deleteSelection`, the transform verbs) would then act on.
    async fn revalidate_interaction_on_document_change(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if self.registry.interactions().await.next().is_none() {
            return Ok(());
        }
        self.revalidate_and_persist_interaction_state(self.interaction_state().await, meta, InteractionRevalidateOrigin::DocumentChange, identity).await?;
        Ok(())
    }

    /// 🕹️ Applies an emit's [`InteractionWrite`]s through the SAME single-writer machine
    /// `dispatch_interaction_action` uses (`protocol::next_selection` over freshly resolved
    /// topology, then `revalidate_and_persist_interaction_state`), so an app-initiated re-select
    /// obeys every declared `SelectionSpec` rule — mode clamping, transitive closure, pruning —
    /// instead of poking `interaction_store` behind the state machine's back. Called only AFTER
    /// the triggering action's own mutations landed, so a just-created id is already in topology.
    /// An undeclared domain is a plugin bug, not a client input, so it faults rather than
    /// silently dropping the write.
    async fn apply_interaction_writes(&mut self, writes: &[InteractionWrite], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if writes.is_empty() {
            return Ok(());
        }
        let mut state = self.interaction_selection_snapshot();
        for write in writes {
            let def = self.registry.interaction(&write.domain).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("interaction write names undeclared domain {}", write.domain)))?;
            let mode = state.active_mode.get(&write.domain).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Multiple));
            let current = state.selection.get(&write.domain).cloned().unwrap_or_default();
            let known_ids = current.ids.iter().cloned().chain(write.targets.iter().map(|target| target.id.clone()));
            let topology = self.resolve_domain_topology(&def, known_ids).await?;
            let selection_input = protocol::SelectionInput { targets: write.targets.clone(), merge: write.merge, mode };
            let next = if write.targets.is_empty() && write.merge == protocol::MergeMode::Replace {
                protocol::DomainSelection { ids: Vec::new(), anchor_id: None, ..current }
            } else {
                protocol::next_selection(&def.selection, &current, &topology, &selection_input).await
            };
            state.selection.insert(write.domain.clone(), next);
            state.active_mode.insert(write.domain.clone(), mode);
        }
        state.hover = self.interaction_hover.clone();
        let domains: Vec<String> = writes.iter().map(|write| write.domain.clone()).collect();
        let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids, &state, &domains);
        self.interaction_leftover_selection = overlay;
        self.interaction_leftover_ids = leftover_ids;
        self.revalidate_and_persist_interaction_state(state, meta, InteractionRevalidateOrigin::Pick, identity).await?;
        Ok(())
    }

    /// 🕹️ The actual body of `dispatch_action`'s interception for the six framework interaction verbs
    /// (`INTERACTION_ACTION_IDS`) — the ONE place any app's hover/selection ever mutates. Runs the
    /// pure os-kernel machine (`next_selection`/`next_hover`), re-validates+persists via
    /// `revalidate_and_persist_interaction_state`, records the command-log row under `ActionKind::Interaction`
    /// (kept out of the history panel — `finish_recorded`'s `skip_history_panel` check), and closes
    /// on the APP-DECLARED refresh scope for the dispatched verb and the domains it actually
    /// touched (`A::interaction_scope`, one entry per `InteractionVerb`), falling back to the scope
    /// DERIVED from the app's own surface declarations
    /// (`semio_framework::interaction_declared_refresh_scope` over `WindowKindDefinition.interactions`)
    /// and only then to `UiDirtyScope::Full`, for an app that declares neither. `touched` is the single domain the
    /// verb names, or every declared domain for the two whole-app verbs
    /// (`clearSelection`/`selectAll`) — the exact set the arms above wrote.
    ///
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the state transition itself lives in
    /// [`Self::apply_interaction_verb`], shared with the in-reactor fold of a GUEST-emitted verb
    /// ([`Self::fold_inline_interaction_verbs`]); this wrapper is the HOST-dispatched half, which
    /// alone owns a reserved-commit permit and shapes the result as its own `InvocationResult`.
    async fn dispatch_interaction_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.validate_framework_reserved_commit(action, permit).await?;
        let applied = self.apply_interaction_verb(action, args, meta, Some(permit), identity).await?;
        let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), applied.scope).await;
        result.output = applied.leftover;
        Ok(result)
    }

    /// 🕹️ ONE interaction verb applied to this instance's session interaction state — the body
    /// `dispatch_interaction_action` (host-dispatched, `permit: Some`) and the guest-emitted fold
    /// (`fold_inline_interaction_verbs`, `permit: None`) share, so the six match arms exist once.
    ///
    /// 🔏️ The reserved-commit permit is a HOST-dispatch concern: it is the cancellation lease and
    /// revision witness of the reserved `FrameworkInteraction*Job` that admitted the host's
    /// verb, and `validate_framework_reserved_commit` refuses a commit against a document
    /// revision that moved while that job was queued. An inline verb has no such job: it runs
    /// inside an ALREADY-ADMITTED app command on the same instance, in the same turn, after that
    /// command's own store lanes landed, and it touches only the session interaction snapshot
    /// (`interaction_store` / `interaction_hover` / the leftover overlay — never the document, so
    /// there is no revision to go stale against). Skipping the permit there is therefore sound
    /// and the two `if let Some(permit)` checks below are exactly the host path's own two.
    async fn apply_interaction_verb(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: Option<&FrameworkReservedCommitPermit>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<AppliedInteractionVerb, Fault> {
        self.refresh_cache().await?;
        let mut state = self.interaction_selection_snapshot();
        let verb = InteractionVerb::of_action(action).ok_or_else(|| plugin_sdk_fault(format!("{action} is not one of the six framework interaction verbs")))?;
        let mut retire_leftover_domains: Vec<String> = Vec::new();
        match verb {
            InteractionVerb::Select => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let targets = parse_interaction_targets(args, action).await?;
                let merge = parse_merge_mode(args).await?;
                let mode = state.active_mode.get(&domain_id).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Single));
                let current = state.selection.get(&domain_id).cloned().unwrap_or_default();
                let known_ids = current.ids.iter().cloned().chain(targets.iter().map(|target| target.id.clone()));
                let topology = self.resolve_domain_topology(&def, known_ids).await?;
                if targets.is_empty() && merge == protocol::MergeMode::Replace {
                    let cleared_ids: HashSet<String> = state.selection.get(&domain_id).map(|selection| selection.ids.iter().cloned().collect()).unwrap_or_default();
                    retire_leftover_domains.push(domain_id.clone());
                    state.selection.remove(&domain_id);
                    if domain_id != "vortex" {
                        if let Some(vortex) = state.selection.get("vortex") {
                            if !vortex.ids.is_empty() && vortex.ids.iter().all(|id| cleared_ids.contains(id)) {
                                retire_leftover_domains.push("vortex".to_string());
                                state.selection.remove("vortex");
                            }
                        }
                    }
                    self.interaction_leftover_ids.retain(|id| !cleared_ids.contains(id));
                    state.active_mode.insert(domain_id, mode);
                } else {
                    let selection_input = protocol::SelectionInput { targets: targets.clone(), merge, mode };
                    let mut next = protocol::next_selection(&def.selection, &current, &topology, &selection_input).await;
                    if next.ids.is_empty() && !targets.is_empty() {
                        next.granularity = targets.last().map(|target| target.granularity.clone()).unwrap_or_else(|| if current.granularity.is_empty() { "object".to_string() } else { current.granularity.clone() });
                        next.ids = targets.iter().map(|target| target.id.clone()).collect();
                        next.anchor_id = targets.last().map(|target| target.id.clone());
                    }
                    state.selection.insert(domain_id.clone(), next);
                    state.active_mode.insert(domain_id, mode);
                }
            }
            InteractionVerb::Hover => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let channel = args.and_then(|value| value.get("channel")).and_then(DslValue::as_str).unwrap_or("pointer").to_string();
                let targets = parse_interaction_targets(args, action).await?;
                let current = self.interaction_hover.get(&domain_id).cloned().unwrap_or_default();
                let known_ids = current.ids.iter().cloned().chain(targets.iter().map(|target| target.id.clone()));
                let topology = self.resolve_domain_topology(&def, known_ids).await?;
                let hover_input = protocol::HoverInput { channel, targets };
                let next = protocol::next_hover(&def.hover, &topology, &hover_input).await;
                if next.ids.is_empty() {
                    self.interaction_hover.remove(&domain_id);
                } else {
                    self.interaction_hover.insert(domain_id, next);
                }
            }
            InteractionVerb::ClearSelection => {
                retire_leftover_domains = self.registry.interactions().await.map(|def| def.id.clone()).collect();
                self.interaction_leftover_ids.clear();
                for domain_id in &retire_leftover_domains {
                    state.selection.remove(domain_id);
                }
            }
            InteractionVerb::SelectAll => {
                for def in self.registry.interactions().await.cloned().collect::<Vec<_>>() {
                    let granularity = state.active_granularity.get(&def.id).cloned().unwrap_or_else(|| def.granularities.first().map(|granularity| granularity.id.clone()).unwrap_or_default());
                    let mode = state.active_mode.get(&def.id).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Multiple));
                    let current = state.selection.get(&def.id).cloned().unwrap_or_default();
                    let hovered_ids = self.interaction_hover.get(&def.id).map(|hover| hover.ids.clone()).unwrap_or_default();
                    let known_ids = current.ids.iter().cloned().chain(hovered_ids);
                    let topology = self.resolve_domain_topology(&def, known_ids).await?;
                    let ids: Vec<String> = topology.ordered.iter().filter(|node| node.granularity == granularity).map(|node| node.id.clone()).collect();
                    if !ids.is_empty() {
                        state.selection.insert(def.id.clone(), protocol::DomainSelection { granularity, ids, anchor_id: None });
                    }
                    state.active_mode.insert(def.id.clone(), mode);
                }
            }
            InteractionVerb::SetSelectionMode => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let raw = args.and_then(|value| value.get("mode")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing mode")))?;
                let mode = parse_selection_mode(raw).await?;
                if !def.selection.modes.contains(&mode) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare mode {raw}")));
                }
                state.active_mode.insert(domain_id, mode);
            }
            InteractionVerb::SetGranularity => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let granularity_id = args.and_then(|value| value.get("granularityId")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing granularityId")))?.to_string();
                if !def.granularities.iter().any(|granularity| granularity.id == granularity_id) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare granularity {granularity_id}")));
                }
                state.active_granularity.insert(domain_id, granularity_id);
            }
        }
        let touched = match verb {
            InteractionVerb::ClearSelection | InteractionVerb::SelectAll => self.registry.interactions().await.map(|def| def.id.clone()).collect::<Vec<String>>(),
            _ => vec![interaction_domain_id_arg(args, action).await?],
        };
        state.hover = self.interaction_hover.clone();
        if !retire_leftover_domains.is_empty() {
            let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids, &state, &retire_leftover_domains);
            self.interaction_leftover_selection = overlay;
            self.interaction_leftover_ids = leftover_ids;
        }
        if verb == InteractionVerb::Hover {
            let current_empty = state.selection.values().all(|selection| selection.ids.is_empty());
            if current_empty {
                if let Some(prior) = &self.interaction_leftover_selection {
                    if prior.selection.values().any(|selection| !selection.ids.is_empty()) {
                        state.selection = prior.selection.clone();
                        for (domain, mode) in &prior.active_mode {
                            state.active_mode.entry(domain.clone()).or_insert(*mode);
                        }
                        for (domain, granularity) in &prior.active_granularity {
                            state.active_granularity.entry(domain.clone()).or_insert_with(|| granularity.clone());
                        }
                    }
                }
            }
        }
        Self::overlay_leftover_ids_into_vortex(&mut state);
        let current_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        if !current_ids.is_empty() {
            self.interaction_leftover_ids = current_ids;
        } else if !self.interaction_leftover_ids.is_empty() && retire_leftover_domains.is_empty() {
            let granularity =
                state.active_granularity.get("vortex").cloned().or_else(|| state.selection.get("vortex").map(|selection| selection.granularity.clone())).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        let selection_cleared = !retire_leftover_domains.is_empty();
        let leftover = Self::leftover_interaction_view_from(&state, &self.interaction_hover, meta.view_state.as_ref().and_then(|view| view.window_id.as_deref()), selection_cleared);
        self.interaction_leftover_selection = Some(state.clone());
        if let Some(permit) = permit {
            self.validate_framework_reserved_commit(action, permit).await?;
        }
        self.revalidate_and_persist_interaction_state(state, meta, InteractionRevalidateOrigin::Pick, identity).await?;
        if let Some(permit) = permit {
            if permit.lease.is_cancelled().await {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled before interaction publication")));
            }
        }
        self.record_command(action, ActionKind::Interaction, None, None, Vec::new(), None);
        let declared: Vec<&str> = touched.iter().map(String::as_str).collect();
        let scope = A::interaction_scope(verb, &declared).or_else(|| self.registry.interaction_declared_refresh_scope(verb, &declared)).unwrap_or(UiDirtyScope::Full);
        Ok(AppliedInteractionVerb { scope, leftover })
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — applies every interaction verb a guest
    /// `Emit` asked the host to replay (`Effect::ReplayShellCommand { action_id ∈
    /// INTERACTION_ACTION_IDS }` or `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }`,
    /// peeled by [`take_inline_interaction_verbs`]) INLINE, in this
    /// same turn, on this same instance, in emission order. Until now the host received that
    /// effect, re-entered `onAction` with it, and the verb landed one guest round trip later —
    /// queued BEHIND any pointer input the user issued meanwhile (design §0 "ordering hazard").
    ///
    /// 🧮️ The scope merge is `UiDirtyScope::merged_with` (widest wins: `Full` absorbs, `None`
    /// yields, two `Partial`s union) over the carrying emit's own scope and every applied verb's
    /// app-declared interaction refresh scope, so the one refresh the host runs for the carrying
    /// command already covers the surfaces the pick dirtied.
    ///
    /// 🩹 A verb that faults (undeclared domain, malformed targets, unknown merge …) must NOT fail
    /// the app command that carried it — its mutations already landed and the pick already
    /// happened — so the fault becomes a `dsl::Diagnostic` the caller attaches to its result and
    /// the remaining verbs still apply. Infallible by construction for that reason.
    async fn fold_inline_interaction_verbs(&mut self, carrier: &str, folded: Vec<(String, Option<DslValue>)>, meta: &ActionMeta, scope: UiDirtyScope, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> InlineInteractionFold {
        let mut fold = InlineInteractionFold { scope, leftover: None, diagnostics: Vec::new() };
        for (action, args) in folded {
            match self.apply_interaction_verb(&action, args.as_ref(), meta, None, identity).await {
                Ok(applied) => {
                    fold.scope = fold.scope.merged_with(applied.scope);
                    fold.leftover = Some(applied.leftover);
                }
                Err(fault) => {
                    fold.diagnostics.push(semio_framework_diagnostic::Diagnostic::error(
                        INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE,
                        semio_framework_diagnostic::TextSpan::default(),
                        format!("{carrier}: inline {action} was not applied ({}: {})", fault.code.0, fault.message),
                    ));
                }
            }
        }
        fold
    }

    /// 🕹️ Reserved leftover publication the host already peels (`leftoverShellInvocationFrames` →
    /// leftover `Invocation.output`). Built from the just-computed InteractionView, not the
    /// post-revalidate store snapshot, so a pick still lands on leftover when topology later prunes.
    /// 🕹️ leftoverInteractionStateV1 twin: leftover.ids become the vortex snapshot Inspection reads
    /// when persist `selection(vortex)` is empty after topology prune.
    fn overlay_leftover_ids_into_vortex(state: &mut protocol::InteractionState) {
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
        if vortex_empty && !leftover_ids.is_empty() {
            let granularity =
                state.active_granularity.get("vortex").cloned().or_else(|| state.selection.get("vortex").map(|selection| selection.granularity.clone())).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: leftover_ids, anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
    }

    /// 🪟️ `window_id` is the window INSTANCE the action that produced this leftover addressed, and
    /// it rides the encode so the host publishes the overlay under that pane rather than inferring one
    /// (`publishLeftoverWorldSelectionV1`, `🏛️ShellHost/🟦️.tsx`). `None` is the document scope of a
    /// windowless action — never a synthetic window (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
    fn leftover_interaction_view_from(state: &protocol::InteractionState, hover: &InteractionHoverState, window_id: Option<&str>, selection_cleared: bool) -> DslValue {
        let selected_ids = leftover_selected_ids_of(state);
        let locked: Vec<(String, DslValue)> = selected_ids.iter().map(|id| (id.clone(), DslValue::Bool(false))).collect();
        let hover_target = hover.iter().find_map(|(domain, hover)| {
            hover.ids.first().map(|id| DslValue::object([("domain".to_string(), DslValue::String(domain.clone())), ("channel".to_string(), DslValue::String(hover.channel.clone())), ("id".to_string(), DslValue::String(id.clone()))]))
        });
        let gumball = DslValue::object([("active".to_string(), DslValue::Bool(leftover_gumball_active_v1())), ("anchorId".to_string(), selected_ids.first().cloned().map(DslValue::String).unwrap_or(DslValue::Null))]);
        DslValue::object([(
            "interactionView".to_string(),
            DslValue::object([
                ("selection".to_string(), semio_framework_value::ToValue::to_value(&state.selection)),
                ("hover".to_string(), semio_framework_value::ToValue::to_value(hover)),
                ("activeMode".to_string(), semio_framework_value::ToValue::to_value(&state.active_mode)),
                ("activeGranularity".to_string(), semio_framework_value::ToValue::to_value(&state.active_granularity)),
                ("selectedIds".to_string(), semio_framework_value::ToValue::to_value(&selected_ids)),
                ("locked".to_string(), DslValue::Object(locked)),
                ("gumball".to_string(), gumball),
                ("hoverTarget".to_string(), hover_target.unwrap_or(DslValue::Null)),
                ("windowId".to_string(), window_id.map(|id| DslValue::String(id.to_string())).unwrap_or(DslValue::Null)),
                ("selectionCleared".to_string(), DslValue::Bool(selection_cleared)),
            ]),
        )])
    }
    /// 🕹️ Task 5: recursively walks a just-presented `ComponentTree`, and for every `Component::Tree`
    /// carrying an `interaction_domain` caches a `DomainTopology` derived from its children (the
    /// `HierarchyProvider::UiTree` half of `resolve_domain_topology`) into `interaction_ui_topology`.
    /// Recursion is uniform over `TreeNode::children` now — every component nests the same way (no
    /// more per-variant `Stack`/`Section`/`Group`/`Field` match arms; a leaf is just a node with no
    /// children).
    ///
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): ALSO derives one `PresenceUpdate` per node
    /// whose id is selected/hovered — own state off `state`/`self.interaction_hover`, peers off
    /// `InteractionView::peers_selecting`/`peers_hovering` — pushed onto `self.pending_presence`
    /// for `plugin_runtime::plugin_render_surface` to drain. This closes the gap the prior packet's
    /// own report named ("nothing in this function currently reads `state`" — see git history):
    /// selection/hover is `ui_contract::PresenceUpdate` now, never written back onto the tree
    /// itself (`TreeNode`/`Component::TreeItem` carry no presence field — the render-plane
    /// derivation lives entirely in this outbox, never in a document mutation).
    ///
    /// 🪟️ A windowed tree hands `ui_tree_domain_topology` only the rows this render
    /// materialised, never the container's full `TreeWindow::total` — see that function's own note.
    ///
    /// 🕹️ Materialized owned BEFORE the mutable walk — same "clone owned before the field-wise
    /// destructure" reasoning `dispatch_typed_command_inner` already uses: `pending_presence` is
    /// WRITTEN inside the walk while `interaction_hover`/`peer_presence` are only ever READ by
    /// it, so a snapshot avoids an aliasing conflict against `&mut self`.
    #[cfg(test)]
    pub(crate) async fn stamp_and_cache_interaction_ui(&mut self, tree: &ComponentTree, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        OriginalRenderInteractionStamp { registry: &self.registry, interaction_hover: &self.interaction_hover, peer_presence: &self.peer_presence, own_color: self.own_color, local_interaction_authority: &mut self.local_interaction_authority, interaction_ui_topology: &mut self.interaction_ui_topology, pending_presence: &mut self.pending_presence, presence_marked_keys: &mut self.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }

    async fn stamp_pending_render_interaction_ui(&mut self, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        let tree = self.pending_render_tree.as_ref().ok_or_else(||ui_assembly_error("render.source-absent"))?;
        OriginalRenderInteractionStamp { registry: &self.registry, interaction_hover: &self.interaction_hover, peer_presence: &self.peer_presence, own_color: self.own_color, local_interaction_authority: &mut self.local_interaction_authority, interaction_ui_topology: &mut self.interaction_ui_topology, pending_presence: &mut self.pending_presence, presence_marked_keys: &mut self.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }

    async fn build_artifact_reserved_action_job(&mut self, action: &str, args: Option<&DslValue>, raw_wire: Vec<u8>, meta: &ActionMeta) -> Result<Option<(ArtifactReservedToolJob, ArtifactToolCompletion<A>)>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let parent_document_id = self.store.envelope().id.clone();
        let canonical_base_revision = self.store.content_revision();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.qualified_tool_proof(action)?.schema_id(),
            contract: self.qualified_tool_proof(action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id,
            canonical_base_revision,
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_content_root),
            raw_wire,
            input: ArtifactReservedToolInput::Action { args: args.cloned(), interaction: self.interaction_selection_snapshot(), hover: self.interaction_hover.clone() },
            completion: completion.clone(),
        };
        Ok(A::build_reserved_tool_job(request)?.map(|job| (job, completion)))
    }

    async fn build_artifact_reserved_media_job(&mut self, port: &str, media: Media, meta: &ActionMeta) -> Result<(ArtifactReservedToolJob, ArtifactToolCompletion<A>), Fault> {
        let action = "import-media";
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.qualified_tool_proof(action)?.schema_id(),
            contract: self.qualified_tool_proof(action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id: self.store.envelope().id.clone(),
            canonical_base_revision: self.store.content_revision(),
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_content_root),
            raw_wire: Vec::new(),
            input: ArtifactReservedToolInput::Media { port: port.to_string(), media },
            completion: completion.clone(),
        };
        let job = A::build_reserved_tool_job(request)?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-reserved-builder"), format!("media port '{port}' is registered but has no concrete resumable importer")))?;
        Ok((job, completion))
    }

    async fn receive_owned_media_poll(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<ArtifactMediaExportPoll>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        match self.drive_owned_media_export(operation_id,cx).await{
            Ok(poll)=>Ok(Some(poll)),
            Err(crate::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure),
            Err(crate::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal(source,cx)
        }
    }

    pub(crate) fn render_failure_terminal_is_empty(&self)->bool{self.pending_render_tree.is_none()&&self.pending_render_tree_close.is_none()&&self.pending_render_tree_identity.is_none()&&self.pending_render_failure.is_none()&&self.pending_render_close_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_retirement.is_none()&&self.pending_render_identity.is_none()}
    pub(crate) fn render_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.pending_render_tree.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(tree)=self.pending_render_tree_close.as_ref(){return tree.next_demand(body)}
        if self.pending_render_close_failure.is_some()||self.pending_render_compound.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.pending_render_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.pending_render_failure.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }
    pub(crate) fn close_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let before=cx.retained_progress();
        if !self.prepare_render_closure_failure(cx)?{return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?)),Err(failure)=>{self.retain_render_closure_failure(failure,cx);return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}}
        let demand=self.render_failure_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(owner)=self.pending_render_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.pending_render_retirement.take());self.pending_render_identity=None;self.pending_render_tree_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.pending_render_failure.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.pending_render_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))},Err((failure,source))=>{*self.pending_render_failure=Some(source);Err(failure.into())}}
            }else{self.pending_render_identity=None;self.pending_render_tree_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    fn close_pending_render_tree_on_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if let Some(identity)=self.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}
        if let Some(tree)=self.pending_render_tree.as_mut(){
            if let Some(resources)=tree.native_resources.as_mut(){
                if resources.close_step_on_original(cx)?!=Some(true){return Ok(None)}
                tree.native_resources=None;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            let tree=self.pending_render_tree.take().expect("original tree source remains stationary until funded close donation");
            *self.pending_render_tree_close=Some(semio_framework_ui_contract::OriginalBuiltTreeRetirement::new(tree.root));
            self.pending_render_tree_ready=false;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        if let Some(body)=self.pending_render_tree_close.as_mut(){
            if !body.terminal_is_empty(){
                let demand=body.next_demand(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx,demand,|grant|body.step(grant))?;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            drop(self.pending_render_tree_close.take());
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        self.pending_render_tree_ready=false;
        Ok(Some(true))
    }

    fn retain_pending_render_tree(&mut self,tree:ComponentTree){
        assert!(self.pending_render_tree.is_none()&&self.pending_render_tree_identity.is_some(),"original render tree destination must be reserved before production");
        self.pending_render_tree=std::mem::ManuallyDrop::new(Some(tree));
        self.pending_render_tree_ready=false;
    }

    fn retain_render_failure<T>(&mut self,source:crate::OriginalRenderFailure,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending render source cannot be overwritten").into())}
        *self.pending_render_failure=Some(source);self.pending_render_identity=Some((cx.operation(),cx.generation()));let _=self.receive_render_failure(cx)?;Ok(None)
    }

    fn retain_render_closure_failure(&mut self,failure:semio_framework_job::OriginalTurnFailure,cx:&semio_framework_job::StepContext<'_>){
        assert!(self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(),"original render closure failure cannot overwrite a retained cause");
        if self.pending_render_failure.is_some(){*self.pending_render_close_failure=Some(failure);}else{*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(failure));self.pending_render_identity=Some((cx.operation(),cx.generation()));}
    }

    fn prepare_render_closure_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_compound.is_some(){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_compound,cx)?{assert!(self.pending_render_failure.is_none());*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(original));}
            return Ok(self.pending_render_compound.is_none());
        }
        if self.pending_render_close_failure.is_none(){return Ok(true)}
        if !matches!(self.pending_render_failure.as_ref(),Some(crate::OriginalRenderFailure::Original(_))){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(original));}
            return Ok(false);
        }
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let Some(crate::OriginalRenderFailure::Original(initial))=self.pending_render_failure.take()else{unreachable!()};
        let closure=self.pending_render_close_failure.take().expect("original render closure cause remains stationary");
        *self.pending_render_compound=Some(semio_framework_job::OriginalClosureFailure{initial,closure});
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    fn receive_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(){return Ok(false)}
        if !self.prepare_render_closure_failure(cx)?{return Ok(true)}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(true),Err(failure)=>{self.retain_render_closure_failure(failure,cx);return Ok(true)}}
        if matches!(self.pending_render_failure.as_ref(),Some(crate::OriginalRenderFailure::Original(_))){
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(true)}
            let Some(crate::OriginalRenderFailure::Original(failure))=self.pending_render_failure.take() else{unreachable!()};
            self.pending_render_identity=None;self.pending_render_tree_identity=None;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Err(failure);
        }
        if self.pending_render_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending render domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{self.pending_render_identity=None;self.pending_render_tree_identity=None;return Err(failure)}
        Ok(true)
    }

    pub(crate) fn media_submission_refusal_terminal_is_empty(&self)->bool{self.media_download_preparation.is_none()&&self.media_export_preparation.is_none()&&self.media_submission_refusal.is_none()&&self.media_submission_refusal_retirement.is_none()&&self.media_submission_refusal_identity.is_none()}
    pub(crate) fn media_submission_refusal_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        use semio_framework_value::retirement::RetireOwned;
        if let Some(original)=self.media_download_preparation.as_ref(){return original.retirement_demands(body)}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if original.producer.is_some()||original.body.is_some()||original.request.is_some()||original.request_close.is_some()||original.parts.is_some()||original.parts_close.is_some()||original.handle.is_some()||original.handle_close.is_some()||original.metadata.is_some()||original.metadata_close.is_some()||original.snapshot.is_some()||original.snapshot_lease.is_some()||original.completion.is_some()||original.completion_close.is_some()||original.credit.is_some()||original.credit_close.is_some()||original.chunks.is_some()||original.chunks_close.is_some()||original.operation.is_some(){return original.retirement_demands(body)}
            if let Some(owner)=original.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<crate::tool_cancellation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(&self.tool_cancellations,body)}}
            if original.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<crate::tool_cancellation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
            if let Some(owner)=original.snapshot_retention.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.retirement_demands(body)}}
            if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.retirement_demands(body)}
            return Ok(RetirementDemand{copy_bytes:size_of::<MediaExportPreparation<A>>(),depth:1,..Default::default()})
        }
        if let Some(owner)=self.media_submission_refusal_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.media_submission_refusal_retirement),depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.media_submission_refusal.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }
    pub(crate) fn close_media_submission_refusal(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        if let Some(original)=self.media_download_preparation.as_ref(){self.snapshot_retirements.get(original.operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending download lost its original snapshot source"))?.validate_retirement(cx)?}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if let Some(retention)=original.snapshot_retention.as_ref(){if !retention.terminal_is_empty(){retention.validate_retirement(cx)?}}
            else if let Some(retention)=self.snapshot_retirements.get(original.key.operation_id.0){retention.validate_retirement(cx)?}
            else if !original.snapshot_source.terminal_is_empty(){original.snapshot_source.validate_retirement(cx)?}
            else if original.source_identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"empty media preparation footer requires its unchanged original receiving source").into())}
        }
        let demand=self.media_submission_refusal_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(original)=self.media_download_preparation.as_mut(){if let Some(step)=original.close_step(grant)?{return Ok((PluginLifecycleStep::Progress(step.progress()),false))}self.media_download_preparation.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))}
            if let Some(original)=self.media_export_preparation.as_mut(){
                if let Some(progress)=original.close_material_step(cx,grant)?{return Ok((PluginLifecycleStep::Progress(progress),false))}
                let step=if let Some(owner)=original.retirement.as_mut(){if owner.terminal_is_empty(){RetainedCloneStep::Progress(crate::tool_cancellation::original_lease_cursor_release(&mut original.retirement,grant)?)}else{owner.close_step(&self.tool_cancellations,grant)?}}else if original.lease.is_some(){let(owner,progress)=crate::tool_cancellation::original_lease_cursor_admit(&mut original.lease,grant)?;original.retirement=owner;RetainedCloneStep::Progress(progress)}
                else if let Some(owner)=original.snapshot_retention.as_mut(){if owner.terminal_is_empty(){original.snapshot_retention.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{return owner.close_step(cx,grant).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}}
                else if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.close_step(cx).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}
                else{self.media_export_preparation.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})};
                return Ok((PluginLifecycleStep::Progress(step.progress()),false))
            }
            if let Some(owner)=self.media_submission_refusal_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.media_submission_refusal_retirement.take());self.media_submission_refusal_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.media_submission_refusal.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.media_submission_refusal_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))},Err((failure,source))=>{*self.media_submission_refusal=Some(source);Err(failure.into())}}
            }else{self.media_submission_refusal_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    fn retain_media_submission_refusal<T>(&mut self,source:impl Into<crate::media_operation_refusal::MediaOperationSource>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending media source cannot be overwritten").into())}
        *self.media_submission_refusal=Some(crate::media_operation_refusal::MediaOperationRefusal{source:source.into()});self.media_submission_refusal_identity=Some((cx.operation(),cx.generation()));let _=self.receive_media_submission_refusal(cx)?;Ok(None)
    }

    fn receive_media_submission_refusal(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_none(){return Ok(false)}
        if self.media_submission_refusal_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending media domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.media_submission_refusal,cx)?{self.media_submission_refusal_identity=None;return Err(failure)}
        Ok(true)
    }

    async fn receive_owned_media_export(&mut self,port:&str,app_instance_id:u32,receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        match self.submit_owned_media_export(port,app_instance_id,receiver,cx).await{
            Ok(handle)=>Ok(handle),
            Err(crate::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal::<()>(source,cx).map(|_|false),
            Err(crate::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure)
        }
    }

    async fn submit_owned_media_export(&mut self,port:&str,app_instance_id:u32,mut receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,crate::media_operation_refusal::MediaOperationFailure>{
        if self.media_export_preparation.is_none(){if let Some(handle)=receiver.as_deref().and_then(|slot|slot.as_ref()){
            let active=self.media_exports.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its live operation"))?;
            if active.handle.original_loan()!=handle.original_loan()||active.port!=port{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle belongs to another operation").into())}
            self.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its original source"))?.validate(cx)?;return Ok(true)
        }}
        let source_identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);let port_identity=(port.as_ptr()as usize,port.len());let grant=cx.retained_grant();
        if let Some(original)=self.media_export_preparation.as_ref(){if original.source_identity!=source_identity||original.port_identity!=port_identity||original.key.app_instance_id!=app_instance_id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media submission requires its unchanged original source and borrowed port").into())}}
        if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media_export_preparation.is_none(){
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<MediaExportPreparation<A>>(){return Ok(false)}
            let Some(slot)=(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot|self.media_exports.slot_is_vacant(*slot)&&self.media_closures.slot_is_vacant(*slot)&&self.snapshot_retirements.slot_is_vacant(*slot))else{return Ok(false)};
            let revision=self.store.content_revision_now();let operation_id=semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64,slot as u64);let key=ToolOperationKey{app_instance_id,document:ArtifactDocumentAuthority(app_instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),generation:semio_framework_job::Generation(self.store.generation_now())};
            *self.media_export_preparation=Some(MediaExportPreparation{canonical_revision:revision,metadata:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),metadata_stage:0,metadata_close:ManuallyDrop::new(None),request_close:ManuallyDrop::new(None),parts:ManuallyDrop::new(None),parts_close:ManuallyDrop::new(None),handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),operation:None,body_ready:false,operation_bound:false,assembled:false,published:false,snapshot_source:semio_framework_plugin_ownership::store_lifecycle::ArtifactMediaSnapshotSource::empty(),snapshot:ManuallyDrop::new(None),snapshot_lease:ManuallyDrop::new(None),snapshot_retention:ManuallyDrop::new(None),request:ManuallyDrop::new(None),body:ManuallyDrop::new(None),producer:ManuallyDrop::new(None),source_identity,port_identity,key,lease:None,retirement:None,completion:None,completion_close:None,credit:None,credit_close:None,chunks:None,chunks_close:None});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media_export_preparation.as_mut().unwrap();if original.retirement.is_some(){return Ok(false)}
        if original.published{
            self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its original supplier"))?.validate(cx)?;
            let Some(receiver)=receiver.as_deref_mut()else{return Ok(true)};
            if original.handle.is_some(){let _=ArtifactMediaExportHandle::receive_original(&mut original.handle,receiver,cx)?;return Ok(false)}
            if receiver.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its retained receiving handle").into())}
            let bytes=size_of::<MediaExportPreparation<A>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            self.media_export_preparation.take();
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if self.store.content_revision_now()!=original.canonical_revision||self.store.generation_now()!=original.key.generation.0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media preparation requires its exact captured canonical revision and generation").into())}
        let(id,registration)=self.app_tool_registrations.iter().find(|(id,_)|id.strip_prefix("export-media:")==Some(port)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"media output lacks its published application factory"))?;
        if registration.owner!=crate::tool_contract::ToolOwnerWitness::of::<A>()||registration.key.tool_id!=*id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media output requires its exact registered concrete application owner").into())}let contract=registration.contract;
        if original.snapshot.is_none()&&!original.assembled{
            if original.snapshot_source.ready(){let _=original.snapshot_source.handoff(&mut original.snapshot,&mut original.snapshot_lease,&mut original.snapshot_retention,cx)?;return Ok(false)}
            let factory=self.store.owned_snapshot_retirement_factory()?;let store=&self.store;let _=original.snapshot_source.admit_step(factory,||store.snapshot_owner(),cx)?;return Ok(false)
        }
        if let Some(retention)=original.snapshot_retention.as_ref(){retention.validate(cx)?}else{self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media producer lost its retained original source and installed supplier"))?.validate(cx)?}
        if original.lease.is_none(){let Some(lease)=self.tool_cancellations.begin(&original.key,cx)?else{return Ok(false)};original.lease=Some(lease);return Ok(false)}
        if !original.assembled{
            if original.completion.is_none(){let _=ArtifactMediaExportCompletion::original_birth_admit(&mut original.completion,cx)?;return Ok(false)}
            if original.credit.is_none(){let _=ArtifactMediaExportCredit::original_birth_admit(contract.max_output_bytes,&mut original.credit,cx)?;return Ok(false)}
            if original.chunks.is_none(){let _=ArtifactOutputChunks::original_birth_admit(contract.max_output_bytes,&mut original.chunks,cx)?;return Ok(false)}
            if original.metadata_stage<6{
                let index=original.metadata_stage;let input=match index{0=>id.as_bytes(),1|3|5=>self.store.envelope().id.as_bytes(),2|4=>port.as_bytes(),_=>unreachable!()};let output=&mut original.metadata.as_mut().unwrap()[index];
                if output.capacity()==0&&!input.is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"media original metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.metadata_stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
            }
            let copy=size_of::<ArtifactMediaExportJobRequest<A>>()+size_of::<crate::media_export::MediaExportOwnedParts>()+size_of::<ArtifactMediaExportHandle>()+size_of::<ArtifactMediaExportCredit>()+size_of::<ArtifactOutputChunks>()+size_of::<ArtifactMediaExportCompletion>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
            let text=|bytes:Vec<u8>|String::from_utf8(bytes).expect("original source metadata retains its complete UTF-8 bytes");let fields=original.metadata.as_mut().unwrap();let tool_id=text(std::mem::take(&mut fields[0]));let parent_document_id=text(std::mem::take(&mut fields[1]));let request_port=text(std::mem::take(&mut fields[2]));let parts_document=text(std::mem::take(&mut fields[3]));let parts_port=text(std::mem::take(&mut fields[4]));let published_document=text(std::mem::take(&mut fields[5]));
            let operation=semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0);
            let completion=original.completion.take().unwrap();let output_credit=original.credit.take().unwrap();let output_chunks=original.chunks.take().unwrap();
            *original.parts=Some(crate::media_export::MediaExportOwnedParts{handle:ArtifactMediaExportHandle{app_instance_id,parent_document_id:parts_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation},port:parts_port,retained_result:None,output_credit:output_credit.clone(),output_chunks:output_chunks.clone(),completion:completion.clone()});
            *original.handle=Some(ArtifactMediaExportHandle{app_instance_id,parent_document_id:published_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation});
            *original.request=Some(ArtifactMediaExportJobRequest{source_identity:(cx.operation(),cx.generation()),retained:cx.retained_grant(),operation,tool_id,parent_document_id,canonical_base_revision:original.canonical_revision,snapshot:original.snapshot.take().unwrap(),snapshot_close:original.snapshot_lease.take().unwrap(),port:request_port,output_credit,output_chunks,completion});original.operation=Some(operation);original.assembled=true;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if !original.body_ready{original.body_ready=A::build_media_export_job(&mut original.request,&mut original.body,cx)?;if original.body_ready&&(original.request.is_some()||original.body.is_none()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media factory readiness must retain one actual body and consume its exact request").into())}return Ok(false)}
        if original.producer.is_none(){let _=ArtifactReservedToolJob::admit_boxed_original(&mut original.body,&mut original.producer,cx)?;return Ok(false)}
        if !original.operation_bound{original.operation_bound=original.producer.as_mut().unwrap().bind_original_operation(&mut original.operation,cx)?;return Ok(false)}
        let bytes=size_of::<crate::media_export::ActiveMediaExport>()+size_of::<ArtifactSnapshotCloseRetention>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let operation=original.request.as_ref().map_or_else(||semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0),|request|request.operation);let operation_id=operation.operation;
        let retention=original.snapshot_retention.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media publication lost its actual snapshot supplier"))?;
        self.snapshot_retirements.insert_admitted(operation_id.0,retention);
        let active=ActiveMediaExport{parts:ManuallyDrop::new(original.parts.take()),parts_retirement:ManuallyDrop::new(None),cancellation_retirement:None,operation,contract,session:None,session_rejected:Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(original.producer.take().unwrap(),(cx.operation(),cx.generation()))),outcome_pending:false,resume_pending:false,lease:ManuallyDrop::new(original.lease.take()),checkpoint_available:false,applied_progress:0,download_pending:false,close_stage:ActiveMediaExportCloseStage::Live};
        self.media_exports.insert_admitted(operation_id.0,active);self.current_media_export=Some(operation_id.0);original.published=true;
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
        Ok(false)
    }

    fn quarantine_media_snapshot(&self, operation_id: u64, active: &ActiveMediaExport) -> Result<(), MediaError> {
        self.snapshot_retirements.get(operation_id).map(|_| ()).ok_or_else(|| MediaError::Payload(active.port.clone(), "media export lost its pre-admitted app-owned snapshot retirement authority".into()))
    }

    fn finish_media_poll(&mut self, operation_id: u64, active: ActiveMediaExport, poll: ArtifactMediaExportPoll) -> Result<ArtifactMediaExportPoll,crate::media_operation_refusal::MediaOperationFailure> {
        if let Err(error) = self.quarantine_media_snapshot(operation_id, &active) {
            debug_assert!(self.media_exports.can_insert(operation_id));
            self.media_exports.insert_admitted(operation_id, active);
            return Err(error.into());
        }
        debug_assert!(self.media_closures.can_insert(operation_id));
        self.media_closures.insert_admitted(operation_id, active);
        Ok(poll)
    }

    async fn drive_owned_media_export(&mut self, operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactMediaExportPoll,crate::media_operation_refusal::MediaOperationFailure> {
        self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media step lost its original retained source and snapshot supplier"))?.validate(cx)?;
        if !self.media_closures.can_insert(operation_id) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media close authority is saturated or already owns this operation").into());
        }
        if self.media_exports.get(operation_id).is_none() {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media registry lost its admitted operation").into());
        }
        let mut active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        let grant=cx.retained_grant();
        if active.resume_pending || active.outcome_pending {
            if let Err(failure)=active.retire_nonterminal_outcome(cx){self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);
        }
        if let Some(rejected) = active.session_rejected.as_mut() {
            if rejected.terminal_is_empty() {
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of_val(rejected){let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
                let cancelled=cx.is_cancelled()||active.lease.as_ref().expect("live media lease").token.is_cancelled_now();drop(active.session_rejected.take());
                return self.finish_media_poll(operation_id,active,if cancelled{ArtifactMediaExportPoll::Cancelled}else{ArtifactMediaExportPoll::Failed("media export mounted session capacity is exhausted".into())});
            }
            if rejected.admission_pending()&&!cx.is_cancelled()&&!active.lease.as_ref().expect("live media lease").token.is_cancelled_now(){
                let config=semio_framework_job::BatchDriveConfig{retained:grant,site:"plugin_media_export_submission",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let result=rejected.admit_pending(config,cx);
                match result{Ok(Some(session))=>{active.session=Some(session);drop(active.session_rejected.take());},Ok(None)=>{},Err(failure)=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}}
            }else{
                let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
                let result=crate::document_ingress::original_rejection_lifecycle(cx,demand,grant,"media rejection awaits its original worker",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
                if let Err(failure)=result{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            }
            let poll = active.running_poll();
            self.media_exports.insert_admitted(operation_id, active);
            return Ok(poll);
        }
        if grant.maximum_items==0||grant.maximum_depth==0{let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
        let pump = active.session.as_mut().map(|session| session.pump_one_for_interactive_turn(cx));
        match pump {
            Some(Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) => {}
            Some(Err(failure))=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());},
            Some(Ok(_)) => {
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                return Ok(poll);
            }
            None => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export lost its mounted session".into())),
        }
        let receipt = active.session.as_mut().and_then(semio_framework_job::MountedWorkerJobSession::take_checked_out_retained_step_receipt);
        if !receipt.is_some_and(|(issued, progress)| progress.fits(issued) && progress.fits(grant)) {
            return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export mounted worker receipt is absent or exceeded its issued grant".into()));
        }
        enum SeenMediaOutcome { Progress, Checkpoint { applied_progress: u64, state_len: usize }, Complete { output_len: usize }, Cancelled, Fault(String), Lost }
        let seen = match active.session.as_ref().map(semio_framework_job::MountedWorkerJobSession::checked_out_outcome) {
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::PreviewReady { .. }))) => SeenMediaOutcome::Progress,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::CheckpointReady { state, applied_progress, .. }))) => SeenMediaOutcome::Checkpoint { applied_progress, state_len: state.len() },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Complete { output, .. }))) => SeenMediaOutcome::Complete { output_len: output.map_or(0, semio_framework_job::RetainedJobPayload::len) },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Cancelled { .. }))) => SeenMediaOutcome::Cancelled,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Fault { detail, .. }))) => SeenMediaOutcome::Fault(detail.single_page().and_then(|bytes| std::str::from_utf8(bytes).ok()).filter(|detail| !detail.is_empty()).unwrap_or("media export job faulted").to_string()),
            _ => SeenMediaOutcome::Lost,
        };
        match seen {
            SeenMediaOutcome::Progress => {
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Checkpoint { applied_progress, state_len } => {
                if applied_progress < active.applied_progress || state_len > active.contract.max_output_bytes {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export checkpoint is non-monotone or oversized".into()));
                }
                active.applied_progress = applied_progress;
                active.checkpoint_available = true;
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Complete { output_len } => {
                if output_len > active.contract.max_output_bytes || active.lease.as_ref().expect("live media lease").is_cancelled().await {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export was cancelled or its commit candidate exceeded the output cap".into()));
                }
                let live_revision_bytes = self.store.content_revision();
                let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
                let live_generation = semio_framework_job::Generation(self.store.generation());
                if !matches!(semio_framework_job::validate_commit(&active.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export completed against a stale revision or generation".into()));
                }
                if active.retained_result.is_none(){match active.completion.take(){Ok(Some(result))=>active.retained_result=Some(result),Ok(None)=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export completed without producer output".into())),Err(error)=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed(error.message))};}
                let result=match active.retained_result.as_ref(){Some(Ok(result))=>result,Some(Err(error))=>{let Some(Err(source))=active.retained_result.take()else{unreachable!("observed exact original media domain result")};self.media_closures.insert_admitted(operation_id,active);return Err(source.into());},None=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export lost its original completion result".into()))};
                let output_bytes = match active.output_credit.validate_terminal() {
                    Ok(output_bytes) => output_bytes,
                    Err(error) => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message)),
                };
                if let Err(error) = validate_media_export_structure(result, &active.output_chunks, output_bytes, active.contract.max_output_bytes) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message));
                }
                active.lease.as_mut().expect("live media lease").finish_in_place();
                let Some(Ok(result))=active.retained_result.take()else{return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export original completion changed during publication".into()));};
                if self.current_media_export == Some(operation_id) {
                    self.current_media_export = None;
                }
                self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Complete(result))
            }
            SeenMediaOutcome::Cancelled => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Cancelled),
            SeenMediaOutcome::Fault(detail) => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(detail)),
            SeenMediaOutcome::Lost => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export original outcome changed during publication".into())),
        }
    }

    async fn cancel_owned_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        let operation_id = handle.operation_id.0;
        if let Some(output) = self.segmented_downloads.get(operation_id) {
            if !output.owns_media_export(handle) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download handle authority mismatch".into()));
            }
            if !self.segmented_closures.can_insert(operation_id) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download cleanup authority is saturated".into()));
            }
            let output = self.segmented_downloads.remove(operation_id).ok_or_else(|| MediaError::Payload(handle.parent_document_id.clone(), "media export download changed before cancellation".into()))?;
            self.segmented_closures.insert_admitted(operation_id,crate::media_export::MediaDownloadRetirement::pending(output));
            return Ok(());
        }
        if !self.media_closures.can_insert(operation_id) {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media close authority is saturated or already owns this operation".into()));
        }
        let Some(active) = self.media_exports.get(operation_id) else {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "unknown or completed media export handle".into()));
        };
        if active.handle != *handle {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export handle authority mismatch".into()));
        }
        self.quarantine_media_snapshot(operation_id, active)?;
        let active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        self.media_closures.insert_admitted(operation_id, active);
        if self.current_media_export == Some(operation_id) {
            self.current_media_export = None;
        }
        Ok(())
    }

    /// 🧮️ One reserved gesture's work, in the STAGED EDIT ROWS every other capacity in the
    /// runtime is counted in — never the bare item count the route used to hand its job, which
    /// no longer matched the two-row footprint its own config preflight declares.
    async fn framework_reserved_work_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        let items = self.framework_reserved_route_items(action, args).await?;
        FRAMEWORK_RESERVED_ROUTE_CAPACITY.rows_for_items(items).ok_or_else(|| plugin_sdk_fault(format!("framework reserved route '{action}' declares {items} items beyond its exact work capacity")))
    }

    async fn framework_reserved_route_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        match action {
            "commitCheckpoint" | "checkoutCheckpoint" => Ok(self.children.len().saturating_add(1)),
            REVERT_TO_COMMAND_ACTION_ID => {
                self.refresh_cache().await?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
                let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                match target {
                    Some(edit_id) => {
                        let applied = self.store.applied_edit_ids();
                        Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                    }
                    None => Ok(1),
                }
            }
            "copy" | "cut" => {
                let interaction = self.interaction_selection_snapshot();
                Ok(interaction.selection.values().map(|selection| selection.ids.len()).sum::<usize>().max(1))
            }
            "paste" => {
                let fragment_bytes = args
                    .and_then(|value| value.get("fragment"))
                    .and_then(|value| <ClipboardFragment as semio_framework_value::FromValue>::from_value(value.clone()).ok())
                    .map(|fragment| fragment.pack_bytes.map_or(fragment.dsl_text.len(), |bytes| bytes.len()))
                    .unwrap_or_default();
                Ok(fragment_bytes.div_ceil(4_096).max(1))
            }
            _ => Ok(1),
        }
    }

    /// 🖱️ Vortex pointermove is a storm of `interactionHover` (then a click `interactionSelect`)
    /// before the host finishes any Isolated reserved job. Exclusive `job % 64` slots made that
    /// storm `interactive-job.reserved-spawn-capacity`. Latest-wins: retire the same verb, and
    /// any other interaction occupying this hash slot, so hover preview and place can commit.
    fn retire_pending_reserved_latest_wins(&mut self, action: &str, job: u64) {
        if !INTERACTION_ACTION_IDS.contains(&action) {
            return;
        }
        let slot = job as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let mut ids = Vec::new();
        self.pending_reserved.each_id(|id| ids.push(id));
        let retire: Vec<u64> =
            ids.into_iter().filter(|&id| self.pending_reserved.get(id).is_some_and(|pending| pending.action == action || (id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS == slot && INTERACTION_ACTION_IDS.contains(&pending.action.as_str())))).collect();
        for id in retire {
            if let Some(pending) = self.pending_reserved.remove(id) {
                pending.permit.finish();
            }
        }
    }

    /// 🎰️ `pending_reserved` is DIRECT-MAPPED on `job % ARTIFACT_LIVE_OUTPUT_SLOTS`, and this caller
    /// MINTS the id — so a residue class already held by an unrelated reserved verb (a
    /// `noteShellCommand` the shell records for every user command, an in-flight `undo`) must be
    /// answered by minting into a vacant class, never by faulting the whole route. It used to fault:
    /// a canvas pick landed as `interactive-job.reserved-spawn-capacity` with 63 slots standing empty
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END battery #44-pre: `framework route 'interactionSelect'
    /// has no exact pending spawn slot`). `retire_pending_reserved_latest_wins` still runs first, so
    /// an interaction storm keeps its latest-wins retirement rather than filling the table.
    async fn dispatch_framework_reserved_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if self.time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(action) {
            return Err(time_travel_frozen_fault(action).into());
        }
        let contract = self.qualified_tool_proof(action)?.contract();
        let decoded_items = bounded_json_items(args, contract.max_decoded_items)?;
        let wire_args = args.cloned();
        let raw = semio_framework_pack_json::to_json_string(&(action, wire_args)).into_bytes();
        let work_items = self.framework_reserved_work_items(action, args).await?;
        if CLIPBOARD_ACTION_IDS.contains(&action) {
            let Some(authority)=self.admit_reserved_emit_producer(action,meta,cx)?else{return Ok(None)};
            let producer = match self.build_artifact_reserved_action_job(action, args, raw.clone(), meta).await { Ok(producer) => producer, Err(fault) => { self.finish_reserved_emit_producer(authority); return Err(fault.into()); } };
            let mut app_completion = None;
            let job = match producer {
                Some((job, completion)) => {
                    app_completion = Some(completion);
                    job
                }
                None => { self.finish_reserved_emit_producer(authority); return Err(plugin_sdk_fault("clipboard producer has no exact installed reserved builder").into()); },
            };
            return self.dispatch_reserved_original_producer(action,&raw,decoded_items,job,app_completion.expect("installed clipboard producer owns its original completion"),meta,None,authority,cx).await.map(Some).map_err(Into::into);
        }
        let Some(permit)=self.admit_framework_reserved_spawn(action,&raw,decoded_items,meta,cx).await?else{return Ok(None)};
        let mut job = permit.operation.operation.0;
        self.retire_pending_reserved_latest_wins(action, job);
        assert!(self.pending_reserved.can_insert(job),"original reserved admission seals its exact vacant spawn slot");
        self.pending_reserved.insert_admitted(job, PendingFrameworkReserved { action: action.to_string(), args: args.cloned(), meta: meta.clone(), permit });
        self.release_reserved_emit_slot(job);
        let mut admitted = Self::empty_result(
            action,
            meta,
            vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: encode_framework_reserved_job_input(work_items as u64, &raw), placement: semio_framework::kernel::JobPlacement::Isolated }],
            Vec::new(),
            UiDirtyScope::None,
        )
        .await;
        admitted.output = DslValue::Object(vec![("operationId".into(), DslValue::String(job.to_string())), ("generation".into(), DslValue::String(self.store.generation().to_string()))]);
        Ok(Some(admitted))
    }

    async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.qualified_tool_proof(verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🕰️ Hands one finished framework-reserved spawn-job to the app's commit queue. Nothing here
    /// suspends: the commit itself runs unit by unit in [`Self::step_framework_reserved_commit`].
    /// Answers `false` for a job this app never admitted.
    fn admit_framework_reserved_commit(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        let Some(pending) = self.pending_reserved.remove(job) else {
            return Ok(false);
        };
        if let Err(fault) = output {
            pending.permit.finish();
            return Err(fault);
        }
        if self.reserved_commits.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            pending.permit.finish();
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-capacity"), format!("framework route '{}' found the reserved commit queue full", pending.action)));
        }
        let PendingFrameworkReserved { action, args, meta, permit } = pending;
        self.reserved_commits.push_back(FrameworkReservedCommit { action, args, meta, permit, log_generation_before: self.log_generation, stage: FrameworkReservedCommitStage::Validate, applied: 0, total: 1 });
        Ok(true)
    }

    /// 📊️ Progress of the commit at the head of the reserved-commit queue.
    pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.reserved_commits.front().map(|commit| FrameworkReservedCommitProgress { operation: commit.permit.operation.operation.0, applied: commit.applied, total: commit.total })
    }

    /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
    /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
    /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
    async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
        if self.reserved_commit_outcome.is_some() {
            return Ok(());
        }
        let Some(mut commit) = self.reserved_commits.pop_front() else {
            return Ok(());
        };
        match self.run_framework_reserved_commit_unit(&mut commit).await {
            Ok(Some(result)) => {
                let result = self.finish_recorded(commit.log_generation_before, &commit.action, result).await;
                let _ = self.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                commit.permit.finish();
                self.reserved_commit_outcome = Some(Ok(result));
            }
            Ok(None) => {
                commit.applied = commit.applied.saturating_add(1);
                self.reserved_commits.push_front(commit);
            }
            Err(fault) => {
                commit.permit.finish();
                self.reserved_commit_outcome = Some(Err(fault));
            }
        }
        Ok(())
    }

    /// 🧾️ Takes the result of the last finished reserved commit.
    fn take_framework_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.reserved_commit_outcome.take()
    }

    async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
        let receipt = commit.permit.identity.take().ok_or_else(|| plugin_sdk_fault("reserved commit lost its authoring receipt"))?;
        let token = commit.permit.lease.token.clone();
        let publication = commit.permit.lease.publication_claim.clone();
        let mut progress = commit.permit.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
            Ok(output) => self.follow_derivable_children(&mut identity).await.map(|_| output),
            Err(fault) => Err(fault),
        };
        commit.permit.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        commit.permit.identity_progress = progress;
        result
    }

    async fn run_framework_reserved_commit_unit_admitted(&mut self, commit: &mut FrameworkReservedCommit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        if commit.permit.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{}' was cancelled between commit units", commit.action)));
        }
        if matches!(commit.stage, FrameworkReservedCommitStage::Validate) {
            self.validate_framework_reserved_commit(&commit.action, &commit.permit).await?;
            commit.stage = match Self::history_command(&commit.action, commit.args.as_ref()).await {
                Some(ArtifactCommand::CommitCheckpoint { message, authors }) if !self.children.is_empty() => {
                    let keys: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
                    self.admit_child_content_publication_span(keys.len())?;
                    commit.total = keys.len() as u64 + 1;
                    FrameworkReservedCommitStage::CheckpointChildren { message: message.unwrap_or_else(|| "checkpoint".to_string()), authors, keys, next: 0, pins: Vec::new() }
                }
                _ => {
                    commit.total = 1;
                    FrameworkReservedCommitStage::Route { pins: Vec::new() }
                }
            };
        }
        match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
            FrameworkReservedCommitStage::Validate => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stage"), format!("framework route '{}' re-entered its validation unit", commit.action))),
            FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next, mut pins } => {
                if let Some(key) = keys.get(next) {
                    if let Some(pin) = self.checkpoint_child_unit(&message, &authors, key.borrowed(), identity).await? {
                        pins.push(pin);
                    }
                }
                commit.stage = if next + 1 < keys.len() {
                    FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next: next + 1, pins }
                } else {
                    pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
                    FrameworkReservedCommitStage::Route { pins }
                };
                Ok(None)
            }
            FrameworkReservedCommitStage::Route { pins } => {
                let action = commit.action.clone();
                if HISTORY_ACTION_IDS.contains(&action.as_str()) {
                    let result = self.commit_framework_history_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, pins, identity).await?;
                    if action == "checkoutCheckpoint" && result.ui_scope != UiDirtyScope::None {
                        let pins = self.checkout_cascade_pins()?;
                        if !pins.is_empty() {
                            commit.total = commit.total.saturating_add(pins.len() as u64);
                            commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: 0, result };
                            return Ok(None);
                        }
                    }
                    Ok(Some(result))
                } else if action == REVERT_TO_COMMAND_ACTION_ID {
                    match self.begin_framework_revert_route(commit.args.as_ref(), &commit.meta, identity).await? {
                        Ok(result) => Ok(Some(result)),
                        Err(edit_id) => {
                            commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                            Ok(None)
                        }
                    }
                } else {
                    self.commit_framework_shared_host_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, identity).await.map(Some)
                }
            }
            FrameworkReservedCommitStage::CheckoutChildren { pins, next, result } => {
                let Some(pin) = pins.get(next) else { return Ok(Some(result)) };
                self.checkout_child_unit(pin, identity).await?;
                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                Ok(None)
            }
            FrameworkReservedCommitStage::Revert { edit_id } => {
                if self.framework_revert_unit(&edit_id, identity).await? {
                    commit.total = commit.total.saturating_add(1);
                    commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                    return Ok(None);
                }
                self.cache = None;
                self.record_command(REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);
                Ok(Some(Self::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
            }
        }
    }

    /// 🧩️ A pending CHILD redo outranks a chrome replay for the same reason a parent one
    /// does: the document stack is reapplied before the shell rows stacked above it.
    ///
    /// 🧩️ A `Child`-lane row is a real undo target too, so chrome undo never jumps OVER
    /// the user's own composed-document edit to replay a panel toggle behind it.
    async fn dispatch_chrome_history_action(&mut self, action: &str, meta: &ActionMeta) -> Result<Option<InvocationResult>, Fault> {
        if action == "redo" {
            if !self.store.redo_edit_ids().is_empty() || self.child_history_tails().await.1 {
                return Ok(None);
            }
            let Some(replay) = self.shell_redo.pop() else {
                return Ok(None);
            };
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=redo seq={}", replay.seq));
            self.shell_undone.remove(&replay.seq);
            self.history_dirty_sequences.insert(replay.seq);
            self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
            return Ok(Some(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: replay.redo.action_id, args: replay.redo.args }], vec![history_changed_event().await], UiDirtyScope::Full).await));
        }
        if action != "undo" {
            return Ok(None);
        }
        let (child_applied_tails, _) = self.child_history_tails().await;
        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
        let target = self.command_log.iter().rev().find(|entry| {
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            shell || document || child
        });
        let Some(entry) = target else {
            return Ok(None);
        };
        if entry.edit_id.is_some() || entry.inverse.is_none() {
            return Ok(None);
        }
        let seq = entry.seq;
        let inverse = entry.inverse.clone().expect("chrome undo target has inverse");
        let redo = InverseAction { action_id: entry.action_id.clone(), args: None };
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=undo seq={seq} inverse={}", inverse.action_id));
        self.shell_undone.insert(seq);
        self.shell_redo.push(ShellHistoryReplay { seq, redo });
        self.history_dirty_sequences.insert(seq);
        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        Ok(Some(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], vec![history_changed_event().await], UiDirtyScope::Full).await))
    }

    async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] history route action={action}"));
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled at commit")));
        }
        if action == "undo" || action == "redo" {
            if let Some(result) = self.dispatch_supersede_history_action(action, meta, identity).await? {
                return Ok(result);
            }
            if let Some(result) = self.dispatch_chrome_history_action(action, meta).await? {
                return Ok(result);
            }
            let parent_group_id = if action == "undo" { self.store.tail_group_id().await } else { self.store.redo_tail().await.and_then(|(_, group_id)| group_id) };
            let group_id = match parent_group_id {
                Some(group_id) => Some(group_id),
                None => self.child_group_history_target(action).await,
            };
            if let Some(group_id) = group_id {
                let grouped = self.dispatch_group_history_action(action, &group_id, meta, identity).await?;
                if grouped.ui_scope != UiDirtyScope::None {
                    return Ok(grouped);
                }
            }
        }
        let command = Self::history_command(action, args).await.ok_or_else(|| format!("history action {action} missing required argument"))?;
        match self.store.dispatch(command, identity).await {
            Ok(_) => {
                self.stamp_checkpoint_composition_pins(pins).await?;
                self.cache = None;
                if self.store.local_step_pending() {
                    self.defer_history_step_row(action);
                } else {
                    self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
                }
                Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await)
            }
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() == vcs::os_store::document_materialize::EMPTY_CHECKPOINT_MESSAGE => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
    /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
    /// answers its complete result.
    async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Result<InvocationResult, String>, Fault> {
        let action = REVERT_TO_COMMAND_ACTION_ID;
        self.refresh_cache().await?;
        let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
        let transition_id = entry_seq.and_then(|seq| self.command_log.iter().find(|entry| entry.seq == seq)).and_then(|entry| entry.transition_id.clone());
        if let Some(transition_id) = transition_id {
            return Ok(Ok(self.revert_history_edit_row(&transition_id, meta, identity).await?));
        }
        let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
        let edit_id = match target {
            Some((Some(edit_id), _, _)) => edit_id,
            Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
            _ => return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
        };
        if !self.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
            return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
        }
        Ok(Err(edit_id))
    }

    /// ⏪️ One revert unit: undoes the newest document edit while `edit_id` is not yet the document store's tail.
    /// Answers whether an undo was applied (`false` once the target is the tail or the store refuses).
    async fn framework_revert_unit(&mut self, edit_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<bool, Fault> {
        let applied = self.store.applied_edit_ids();
        if !(applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)) {
            return Ok(false);
        }
        let undone = self.store.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ());
        match undone {
            Ok(()) => Ok(true),
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(false),
            Err(error) => Err(error.into_fault()),
        }
    }

    async fn commit_framework_clipboard_completion(&mut self,action:&str,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        if let Some(fault)=permit.producer_fault{let mounted=self.tool_operations.get_mut(id).expect("original completion owns its admitted slot");mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original completion lease").cancel();}
        self.finish_mounted_reserved_completion(id);
        Ok(started)
    }

    /// 📥️ Keeps the original producer and completion in their fixed slot through every fallible worker boundary.
    async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        let proof=self.qualified_tool_proof(verb)?;
        let key=proof.key();
        let admitted=match admission{Some(admitted)=>admitted,None=>self.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
        if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
        let mounted=self.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
        let job=mounted.reserved_producer.as_ref().expect("original reserved producer remains retained").clone();
        let dispatched=self.tool_jobs.dispatch(semio_framework::ToolOperationSpec::new(key.controller_id,key.tool_id,proof.schema_id(),job,mounted.operation)).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        mounted.session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatched.job,mounted.source_identity));
        mounted.terminal_seen=false;
        mounted.stage=MountedTypedCommandFullOperationStage::Worker;
        Ok(started)
    }

    /// 📨️ Moves the original completion enum before validating its route-specific output kind.
    fn finish_mounted_reserved_completion(&mut self,id:u64){
        let mounted=self.tool_operations.get_mut(id).expect("original reserved completion retains its exact operation");
        let taken=mounted.completion.as_ref().map_or(Ok(None),ArtifactToolCompletion::take);
        let fault=match taken{
            Ok(Some(publication))=>{mounted.publication=Some(publication);match mounted.publication.as_ref().expect("original completion enum is retained"){
                ArtifactToolCompletionValue::Emit(Ok(_),_)=>None,
                ArtifactToolCompletionValue::Emit(Err(fault),_)=>Some(fault.bounded),
                ArtifactToolCompletionValue::Download(_,_)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer completed with original download output"))),
            }},
            Ok(None)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer has no completed original output"))),
            Err(fault)=>Some(ArtifactBoundedToolFault::from_fault(&fault)),
        };
        if let Some(fault)=fault{mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original reserved completion lease").cancel();}
    }

    async fn commit_framework_shared_host_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.validate_framework_reserved_commit(action, permit).await?;
        if INTERACTION_ACTION_IDS.contains(&action) {
            return self.dispatch_interaction_action(action, args, meta, permit, identity).await;
        }
        if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
            let value = args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or(semio_framework::HISTORY_COMMAND_FILTER_ALL);
            self.history_filter = HistoryCommandFilter::from_value(value).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("history-filter.unknown"), format!("'{value}' is no declared history filter option")))?;
            self.log_generation += 1;
            self.history_dirty_sequences.extend(self.command_log.iter().map(|entry| entry.seq));
            return Ok(Self::empty_result(
                action,
                meta,
                Vec::new(),
                Vec::new(),
                UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false },
            )
            .await);
        }
        if action == NOTE_SHELL_COMMAND_ACTION_ID {
            let command_id = args.and_then(|value| value.get("commandId")).and_then(DslValue::as_str).ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;
            let detail = args.and_then(|value| value.get("detail")).cloned();
            let label = note_shell_command_label(args.and_then(|value| value.get("label")), command_id, detail.as_ref());
            let inverse = args
                .and_then(|value| value.get("inverseCommandId"))
                .and_then(DslValue::as_str)
                .map(|inverse_command_id| InverseAction { action_id: inverse_command_id.to_string(), args: args.and_then(|value| value.get("inverseArgs")).cloned().or(detail) });
            self.validate_framework_reserved_commit(action, permit).await?;
            self.record_command(command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);
            return Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        if action == RECORD_TUTORIAL_ACTION_ID {
            return Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-shared-host-route"), format!("unknown shared host route '{action}'")))
    }

    /// 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
    /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
    /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard/interaction) —
    /// an app's own behavior is dispatched exclusively through `dispatch_typed_command` now.
    async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;
        let pack = snapshot.encode_pack();
        let spr = store::os_store::text_format::empty_document_spr("", A::DOCUMENT_SCHEMA).await;
        self.dispatch_emit(action, Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }, meta, identity, cx).await
    }

    /// ⏳️ The `document.loading` refusal of `verb` while a whole-document load still runs (`📓️api-stepped-document-load.md`
    /// §3, gap N17): every verb but a view or interaction verb waits — nothing reads or edits the document being replaced;
    /// hover and pick read only the selection (audit W2A-13) — except `historyEditCancelReplay`, which cancels the load;
    /// `None` while no load runs.
    pub(crate) fn document_loading_refusal(&self, verb: &str) -> Option<Fault> {
        let mut loading = false;
        self.document_archive_loads.each_id(|operation| loading |= self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading));
        let exempt = matches!(self.declared_action_kind(verb), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&verb) || verb == semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID;
        (loading && !exempt).then(|| Fault::new(FaultOrigin::Framework, FaultCode::new(DOCUMENT_LOADING_CODE), format!("'{verb}' waits: the document is still loading")))
    }

    /// 🏷️ The kind an action or command `verb` is declared with; `None` for a verb this app does not declare.
    fn declared_action_kind(&self, verb: &str) -> Option<ActionKind> {
        self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind))
    }

    pub(crate) async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let Some(slot)=self.stage_action_source(action,source_args,original_fault,cx)?else{return Ok(None)};
        let Some(mut source)=self.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
        let terminal=!matches!(&result,Ok(None));
        self.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.pending_action_ingress_identity=None;}
        result
    }

    async fn dispatch_action_received(&mut self,action:&str,source:&mut crate::action_ingress::ActionIngressSource,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if A::ROLE == AppRole::Viewer && viewer_rejects_action(action) {
            return Err((viewer_read_only_fault(action)).into());
        }
        if action == CANCEL_TYPED_OPERATION_ACTION_ID {
            return self.dispatch_operation_cancellation(args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.dismiss_refused_reprojection(action);
        if let Some(fault) = self.document_loading_refusal(action) {
            return Err((fault).into());
        }
        if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || is_time_travel_action_id(action)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance"))).into());
        }
        Box::pin(self.deliver_utility_host_events(meta, identity, cx)).await?;
        if action == semio_framework::HOST_EVENT_ACTION_ID {
            return Box::pin(self.dispatch_host_event_action(args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Box::pin(self.commit_typing_before(action, args, meta, identity, cx)).await?;
        if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("framework.document-transfer.shell-owned"), format!("{action} is performed by the shell, never by the program"))).into());
        }
        if is_tool_run_action_id(action) {
            return self.dispatch_tool_run_action(action, args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_time_travel_action_id(action) {
            self.freeze_tool_machines();
            return Box::pin(self.dispatch_time_travel_action(action, args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_framework_reserved_action_id(action) {
            return self.dispatch_framework_reserved_action(action,args,meta,cx).await;
        }
        if action == CATALOGUE_EXAMPLE_ACTION_ID && self.registry.get(action).is_none() {
            let example_id = args.and_then(|value| value.get("exampleId")).and_then(DslValue::as_str).unwrap_or("");
            if let Some(document) = A::catalogue_example_document(example_id) {
                let body = document?;
                return self.dispatch_catalogue_example(action, &body, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        let definition = self.registry.get(action).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{action}' before command construction")))?;
        validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;
        let definition_kind = definition.kind;
        let tool = match Box::pin(self.admit_tool_dispatch(action, args, meta, identity, cx)).await {
            ToolDispatch::Plain => None,
            ToolDispatch::Tagged(tag) => Some(tag),
            ToolDispatch::Settled(result) => return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from),
        };
        if source.assembled.is_none(){match self.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        }}
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {
            let admission = self.admit_host_configuration_json(action, args).await?;
            self.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration action '{action}' exceeds its exact output cap"))).into());
            }
            let mut emit = Emit::config(vec![config_mutation]);
            if let Some(ToolTag::Scrub(tag)) = &tool {
                let base = self.store.content_revision_now();
                let published = self.settle_press(tag, &base, &meta.actor, &mut emit)?;
                self.follow_tool_machines(true);
                if !published {
                    return Ok(Some(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await));
                }
            }
            let previous_kind = self.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, emit, meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {
            let previous_kind = self.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, Emit::default(), meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else {
            let command=A::command_from_action(action,if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fault).await?;
            let command=Box::new(command);
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
            let admission = self.admit_command_wire(action, &wire, 1).await?;
            self.require_complete_tool_operation_pipeline(&admission)?;
            self.tool_machines.ingress = tool;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await;
            self.tool_machines.ingress = None;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }
    }

    /// 🎯️ Validates manifest structural app/mode ownership, converts the addressed
    /// invocation arguments once into the app's typed command, and enters the typed dispatch channel.
    async fn dispatch_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let command_id = address.command_id.as_str();
        let owner_app_id = self.app.instance_id().await.to_string();
        let (owner, definition) = match &address.owner {
            ManifestCommandOwnerAddress::App { app_id, .. } => {
                if app_id != &owner_app_id {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}"))).into());
                }
                (format!("app {owner_app_id}"), self.registry.app_command(command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}")))?)
            }
            ManifestCommandOwnerAddress::Mode { app_id, mode_id, .. } => {
                if app_id != &owner_app_id || active_mode_id != Some(mode_id.as_str()) {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}"))).into());
                }
                (format!("mode {mode_id}"), self.registry.mode_command(mode_id, command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}")))?)
            }
            _ => return Err((Fault::from(format!("command '{command_id}' is not app- or mode-owned"))).into()),
        };
        validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(slot)=self.stage_action_source(command_id,source_args,original_fault,cx)?else{return Ok(None)};
        let Some(mut source)=self.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=async {
        let args = source.args.as_ref();
        match self.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(Self::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        };
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(command_id, args)? {
            let admission = self.admit_host_configuration_json(command_id, args).await?;
            self.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration command '{command_id}' exceeds its exact output cap"))).into());
            }
            let previous_kind = self.invocation_kind.replace(kind);
            let result = self.dispatch_emit(command_id, Emit::config(vec![config_mutation]), meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let crate::action_ingress::ActionIngressSource{assembled,prepared,args,fault,..}=&mut *source;
        let command=A::command_from_action(command_id,if assembled.is_some(){assembled}else if prepared.is_some(){prepared}else{args},fault).await?;
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(command_id, &wire, 1).await?;
        self.require_complete_tool_operation_pipeline(&admission)?;
        let previous_kind = self.invocation_kind.replace(kind);
        let result = self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.invocation_kind = previous_kind;
        result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }.await;
        let terminal=!matches!(&result,Ok(None));
        self.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.pending_action_ingress_identity=None;}
        result
    }

    /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
    /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
    /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
    /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
    /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
    /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
    /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
    async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut BTreeMap<String, DslValue>) -> Result<(), Fault> {
        let declared = match address.window_kind_id.as_str() {
            "*" => self.registry.get(&address.action_id).map(|action| action.args.clone()),
            kind => self.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
        }
        .unwrap_or_default();
        let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
        for arg in omitted.iter() {
            let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
            let token = match format {
                semio_framework::manifest::ArgFormat::DocumentRevision => self.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                semio_framework::manifest::ArgFormat::TargetRevision => {
                    self.refresh_cache().await?;
                    let render_operation = self.live_render_operation();
                    let parent_document_id = self.store.envelope().id.clone();
                    let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                    let (_, snapshot, _, history) = self.cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.instance_operation_owner.clone());
                    A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                        Fault::new(
                            FaultOrigin::Framework,
                            FaultCode::new("agent.target-revision-unresolved"),
                            format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                        )
                    })?
                }
                _ => continue,
            };
            arguments.insert(arg.id.clone(), DslValue::String(token));
        }
        Ok(())
    }

    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
    /// **without applying any of them**.
    ///
    /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
    /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
    /// operation whose publication ladder commits into `self.store` and mints an undo group. An
    /// agent's `action_prepare` must not do that — its own protocol commits separately
    /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
    /// guest instance per plugin, so a prepare that applied would leave the commit applying the
    /// same edit a second time. `history_undo` (which walks back exactly one application) would
    /// then leave half the edit standing — a GREEN gate that lies, which is worse than a red one.
    ///
    /// ✅️ What it shares with the shell lane, deliberately and by call rather than by copy:
    /// the owner-qualified `ManifestActionInvocation` address, `admit_addressed_action_view`'s
    /// window projection, the mode/window-kind ownership checks `handle_action_invocation` runs,
    /// the `windowId` argument it injects, `validate_ui_dispatch_classification`'s interactive-job
    /// discipline, `A::command_from_action`, the exact retained-wire admission `dispatch_action`
    /// begins (`admit_command_wire` + `require_complete_tool_operation_pipeline`) and the app's own
    /// job, built and dispatched through the same factory entry ([`Self::preview_typed_command_job`]).
    /// The one thing it does NOT share is the applying tail: the job runs to the completion it would
    /// publish, and nothing is published. It fails closed ([`agent_lane_carriage`]): a completion that publishes
    /// anything an agent transaction cannot carry is refused by name, never settled without it, and one that carries
    /// nothing but a notice or a selection is refused `app.command.no-effect`, its notices as structured causes.
    ///
    /// 🎟️ The admission it begins is consumed by the previewed job within this same call and retired
    /// through that job's own close protocol, so a prepared handle the caller never invokes leaves
    /// nothing behind.
    ///
    /// 🪟️ `"*"` is the catalog's own marker for an APP-scope verb — one that belongs to no
    /// particular window kind (`🗒️note` declares all 48 of its verbs this way). A concrete
    /// kind must own the verb; `"*"` addresses the app registry directly, exactly as
    /// `dispatch_action` does for every verb the shell sends.
    pub(crate) async fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let address=&invocation.address;
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {owner_app_id}", address.app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() && !self.registry.has_mode(&address.mode_id).await {
            return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
        }
        if address.window_kind_id != "*" && reserved_kind.is_none() && self.registry.window_action(&address.window_kind_id, &address.action_id).await.is_none() {
            return Err((plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id))).into());
        }
        let context = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("addressed action requires the caller's explicit language and terminology context"))?;
        let view = crate::plugin_runtime::admit_addressed_action_view(context, &invocation.address)?;
        let definition = self
            .registry
            .get(&address.action_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{}' before command construction", address.action_id)))?;
        let kind = definition.kind;
        validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
        if self.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
            if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
            if !self.action_source_capture_ready(&address.action_id,cx){return Ok(None)}
            let mut original_args=if self.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
            Some(DslValue::Object(arguments.into_iter().collect()))};
            let Some(command)=self.receive_action_command(&address.action_id,&mut original_args,&mut None,cx).await?else{return Ok(None)};
            let wire=<A::Command as protocol::OpBinary>::encode_op(&command).map_err(|source|source.into_fault())?;
            let admission=self.admit_command_wire(&address.action_id,&wire,1).await?;self.require_complete_tool_operation_pipeline(&admission)?;
            self.begin_agent_lane_preview(Box::new(command),admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
            return Ok(None)
        }
        let maximum_bytes=self.agent_lane_preview.as_ref().unwrap().maximum_bytes;
        let Some((emit,(presence,transient,window_transient),job_steps))=self.advance_agent_lane_preview(&address.action_id,cx)?else{return Ok(None)};
        let carriage = agent_lane_carriage(&AgentLanePublication {
            carried_ops: emit.artifact_mutations.len() + emit.config_mutations.len() + emit.draft_mutations.len() + emit.child_emits.len(),
            effects: &emit.effects,
            window_config_ops: emit.window_config_mutations.len(),
            extension_calls: emit.extension_invocations.len(),
            events: emit.events.len(),
            selection_writes: emit.interaction_writes.len(),
            tasks: emit.tasks.len(),
            presence,
            transient,
            window_transient,
        });
        match carriage {
            AgentLaneCarriage::Carried => {}
            AgentLaneCarriage::NoEffect => return Err((agent_lane_no_effect_fault(&address.action_id, &emit.effects)).into()),
            AgentLaneCarriage::Uncarried(lanes) => return Err((agent_lane_uncarried_fault(&address.action_id, &lanes.join(", "))).into()),
        }
        if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
            return Err((viewer_read_only_fault(&address.action_id)).into());
        }
        let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
        for op in emit.artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        for child in emit.child_emits.iter() {
            if !child.owner.is_empty() || self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-child-missing"),
                    format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                )).into());
            }
        }
        let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);
        let mut config_op_bytes = Vec::with_capacity(emit.config_mutations.len());
        for op in emit.config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let mut draft_op_bytes = Vec::with_capacity(emit.draft_mutations.len());
        for op in emit.draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
        if priced > maximum_bytes {
            return Err((Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.preview-output"),
                format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, maximum_bytes),
            )).into());
        }
        self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });
        let mut result = Self::empty_result(&address.action_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output = DslValue::Object(vec![
            ("previewedAction".into(), DslValue::String(address.action_id.clone())),
            ("actionKind".into(), DslValue::String(format!("{kind:?}"))),
            ("documentOps".into(), DslValue::String(artifact_op_bytes.len().to_string())),
            ("configOps".into(), DslValue::String(config_op_bytes.len().to_string())),
            ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
            ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
            ("opBytes".into(), DslValue::String(priced.to_string())),
            ("jobSteps".into(), DslValue::String(job_steps.to_string())),
        ]);
        Ok(Some(result))
    }

    /// 🪪️ The identity every id one admitted command mints is scoped by ([`AppOperationContext::authoring_seed`]):
    /// content-addressed with [`store::content_id`] over the admitting actor and this document store's hybrid
    /// logical clock ticked for the admission — per author and moment, never a per-instance counter a fresh
    /// session restarts.
    fn authoring_seed(&self, actor: &str) -> String {
        let mut clock = self.store.clock_now();
        clock.tick(semio_framework_job::default_now_ms().unwrap_or(clock.physical_ms));
        store::content_id("authoring-seed", format!("{actor}\u{1f}{}:{}:{}", clock.actor, clock.physical_ms, clock.logical).as_bytes())
    }

    /// 👁️ Builds the SAME app-owned job the shell lane builds for this admitted command (`A::build_tool_job` over
    /// [`Self::capture_typed_command_roots`]), dispatches it through the SAME retained-wire factory entry
    /// `start_typed_command_operation` uses, and drives it through its own steps on this turn
    /// ([`drive_agent_lane_preview`]) to the completion it would publish — publishing nothing. Every job shape an
    /// app owns previews this way: the SDK's retained command, a plugin's own tool-command job (puzzle, writer, …)
    /// and a download job, whose file an agent transaction cannot carry and is refused by name. The run holds a
    /// keyed cancellation lease, so cancelling the document cancels the preview. Answers the emit, how many presence,
    /// transient and window-transient mutations the job's completion would publish (lanes an agent
    /// transaction cannot carry, see [`agent_lane_uncarried_lanes`]) and how many of the job's own steps ran.
    fn admit_agent_lane_preview_key(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.agent_lane_preview.is_none(){
            let grant=cx.retained_grant();let bytes=size_of::<crate::tool_jobs::AgentLanePreview<A>>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let Some(operation_id)=self.admit_typed_operation_slot()else{return Ok(false)};
            let revision=self.store.content_revision_now();let key=ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),generation:semio_framework_job::Generation(self.store.generation_now())};
            let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            let config=semio_framework_job::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
            self.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
        }
        let state=self.agent_lane_preview.as_mut().unwrap();
        if state.closing{return Ok(false)}
        state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if key.app_instance_id!=meta.instance_id{return Err(plugin_sdk_fault("original preview operation belongs to another app instance").into())}
        if state.lease.is_none(){state.lease=self.tool_cancellations.begin_keyed(key,cx)?;}
        Ok(state.lease.is_some())
    }

    async fn begin_agent_lane_preview(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let verb = admission.verb.clone();
        let roots = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let state=self.agent_lane_preview.as_ref().ok_or_else(||plugin_sdk_fault("original preview admission source is absent"))?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if !state.starting||state.lease.is_none()||key.base_revision!=roots.base_revision||key.generation!=roots.generation{return Err(plugin_sdk_fault("original preview roots no longer match their admitted operation key").into())}
        let operation_id=key.operation_id;
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tool_job_controller_id, roots.base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        let completion = ArtifactToolCompletion::<A>::new();
        let presence_local = self.presence_store.local_read().map_err(Fault::from)?;
        let (context, _context_birth) = ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                roots.canonical_base_revision,
                roots.draft_generation,
                roots.transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&roots.children),
                    draft: std::sync::Arc::clone(&roots.draft_snapshot),
                    transient: std::sync::Arc::clone(&roots.transient),
                    window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.draft_store, &self.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&roots.presence_peers), self.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),
            artifact_owned_tool_birth_grant(),
        );
        let key = admission.proof.key();
        let contract = admission.proof.contract();
        let spec = A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            command,
            raw_wire: ArtifactToolRawInput::transferred_to_factory(),
            operation,
            controller_id: key.controller_id,
            tool_id: key.tool_id,
            payload_schema_id: admission.proof.schema_id(),
            contract,
            decoded_items: admission.decoded_items,
            app_instance_id: meta.instance_id,
            parent_document_id: self.store.envelope().id.clone(),
            canonical_base_revision: roots.canonical_base_revision,
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot: roots.snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config: roots.config,
            window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
            history: roots.history,
            interaction_state: roots.interaction_state,
            interaction_hover: std::sync::Arc::new(roots.interaction_hover),
            context,
            instance_operation_owner: self.instance_operation_owner.clone(),
            output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
            completion: completion.clone(),
        })
        .await?
        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?;
        let wire_admission = admission.wire_admission.clone();
        let dispatch =
            self.tool_jobs.dispatch_wire_retained_with_spec(&wire_admission, admission.raw_wire, None, spec).map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?;
        let config=semio_framework_job::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
        let state=self.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer=Some(crate::tool_jobs::OriginalPreviewDrive::new(dispatch.job,config,cx));state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
        Ok(())
    }

    fn advance_agent_lane_preview(&mut self,verb:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,(usize,usize,usize),u64)>,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        let state=self.agent_lane_preview.as_mut().ok_or_else(||plugin_sdk_fault("original preview source is absent"))?;
        if state.starting||state.closing{return Ok(None)}
        if !state.complete{state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;}
        if let Some(producer)=state.producer.as_mut(){let Some(steps)=(match drive_agent_lane_preview(producer,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=steps;state.producer.take();return Ok(None)}
        if let Some(children)=state.children.as_mut(){let Some(steps)=(match drive_agent_lane_preview(children,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=state.steps.saturating_add(steps);state.children.take();state.complete=true;return Ok(None)}
        if !state.complete{
            let completion=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?;
            match completion.take()?{
                Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>{let emit=emit.map_err(ArtifactCompletionFault::into_fault)?;let job=ChildEmissionPreviewJob::<A>{emit:Some(emit),ephemeral:Some(ephemeral),completion:Some(completion.clone()),retirement:None,retirement_payload:None,retirement_fault:None,rejection:None,completion_retirement:None,maximum_bytes:state.maximum_bytes,wire_mode:true,fault_publication:semio_framework_job::RetainedFaultPublication::new(),closing:false};state.children=Some(crate::tool_jobs::OriginalPreviewDrive::new(job,state.config,cx));return Ok(None)},
                Some(value@ArtifactToolCompletionValue::Download(..))=>{state.publication=Some(value);state.complete=true;},
                None=>return Err(plugin_sdk_fault("preview producer completed without its original output").into()),
            }
        }
        if state.publication.is_none(){state.publication=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?.take()?;return Ok(None)}
        if state.completion.is_some()||state.completion_close.is_some(){let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut state.completion,&mut state.completion_close,cx.retained_grant())?;cx.consume_retained(step.progress())?;return Ok(None)}
        if let Some(owner)=state.lease_close.as_mut(){let step=if owner.terminal_is_empty(){crate::tool_cancellation::original_lease_cursor_release(&mut state.lease_close,cx.retained_grant()).map(RetainedCloneStep::Progress)?}else{owner.close_step(&self.tool_cancellations,cx.retained_grant())?};cx.consume_retained(step.progress())?;return Ok(None)}
        if state.lease.is_some(){let(owner,progress)=crate::tool_cancellation::original_lease_cursor_admit(&mut state.lease,cx.retained_grant())?;state.lease_close=owner;cx.consume_retained(progress)?;return Ok(None)}
        if let Some(caller)=state.caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){state.caller.take();}return Ok(None)}
        if cx.retained_grant().maximum_items<2||cx.retained_grant().maximum_depth==0{return Ok(None)}
        cx.consume_retained(RetainedCloneProgress{copied_items:2,..Default::default()})?;
        state.key.take();
        let state=self.agent_lane_preview.take().expect("original preview publication owner remains exact");
        match state.publication{
            Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>emit.map(|emit|Some((emit,(ephemeral.presence.len(),ephemeral.transient.len(),ephemeral.window_transient.len()),state.steps))).map_err(|source|ArtifactCompletionFault::into_fault(source).into()),
            Some(ArtifactToolCompletionValue::Download(..))=>Err(agent_lane_uncarried_fault(verb,"a file download").into()),
            None=>Err(plugin_sdk_fault("preview closed without its original publication").into()),
        }
    }

    pub(crate) fn close_agent_lane_preview(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let Some(state)=self.agent_lane_preview.as_mut()else{return Ok(PluginLifecycleStep::Complete(Default::default()))};
        if state.terminal_is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(PluginLifecycleStep::Progress(Default::default()))}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.agent_lane_preview.take();return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        state.close_step(cx,self.tool_cancellations.handle())
    }

    /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`
    /// carries. Decoding it here is what gives the agent lane the same dispatch the shell's UI
    /// lane has; running it through [`Self::preview_addressed_action`] rather than
    /// `handle_action_invocation` is what makes it a PREPARE (see that method's own doc).
    async fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let invocation: ManifestActionInvocation = crate::plugin_runtime::decode_wire_serialized(command_bytes).await.map_err(|_| {
            Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.missing-exact-key"),
                "a typed command frame carries one owner-qualified ManifestActionInvocation — the same wire AppCommand::Command carries; this frame decoded as neither, so no command-specific pre-serde envelope can be selected",
            )
        })?;
        self.preview_addressed_action(&invocation, meta, cx).await
    }

    /// 📬️ Advances exactly one retained publication unit. Store lanes return one-item
    /// receipts; host lanes remain in the operation until their fixed result page is ACKed.
    /// 🏃️ Drives one Worker-stage typed operation for a bounded slice of this turn. The mounted
    /// worker session only submits a job step to the process pool; on wasm that pool has no threads
    /// and runs a step solely when it is pumped, and until now the only pump was the
    /// cooperative-maintenance cadence (`maintenance_step` stage 0 of 21, one call per ~8 reactor
    /// turns) — measured 2026-09-09: 3 interactive job steps in 4 096 reactor turns, every action
    /// exhausting the host's continuation budget with `more-work`. Bounded by
    /// [`INTERACTIVE_TURN_WORKER_PUMPS`] and [`INTERACTIVE_TURN_WORKER_WALL_US`].
    ///
    /// 🛑️ ONE operation's structural fault is that operation's fault, never the turn's:
    /// see [`Self::fault_typed_operation_worker`].
    /// 🏃️ Spends ONE host turn's grant on consecutive worker steps of one mounted operation: every step receives what the turn still owns, the receipts are summed into the one original receipt the host takes, and the drive hands the turn back when the grant is exhausted, a step made no progress, the operation left the worker stage or its close mode changed.
    fn drive_typed_operation_worker(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();let started_us=cx.now_us();
        let mut turn: Option<(bool, RetainedCloneGrant)> = None;
        let mut spent = RetainedCloneProgress::default();
        let mut failure = None;
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            let Some(operation)=self.tool_operations.get_mut(operation_id)else{break};
            if operation.stage!=MountedTypedCommandFullOperationStage::Worker{break;}
            let closing=operation.terminal_seen||operation.session_rejected.is_some()||operation.worker_outcome_pending;
            if turn.is_some_and(|(kind,_)|kind!=closing){break;}
            let policy=if closing{self.mounted_policy.close}else{self.mounted_policy.preparation};
            let (_,turn_grant)=*turn.get_or_insert((closing,crate::plugin_runtime::original_plugin_turn_grant(policy,retained)));
            let grant=crate::plugin_runtime::original_plugin_turn_grant(policy,crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent));
            let stepped=cx.with_retained_loan(grant,|same|operation.drive_worker_step(same));
            let progress=match &stepped{Ok(step)=>step.progress().unwrap_or_default(),Err(failure)=>failure.primary.retained_progress()};
            spent=spent.checked_add(progress)?;
            if let Err(fault)=stepped{failure=Some(fault);break;}
            if progress.copied_items==0||crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent).maximum_items==0{break;}
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)){break;}
        }
        if spent!=RetainedCloneProgress::default(){
            let (closing,grant)=turn.expect("a worker turn that spent anything was granted");
            let operation=self.tool_operations.get_mut(operation_id).ok_or_else(||plugin_sdk_fault("worker receipt lost its mounted operation"))?;
            if closing{assert!(operation.original_retirement_receipt.is_none(),"worker Close receipt awaits its original recipient");operation.original_retirement_receipt=Some((grant,spent));}
            else{assert!(operation.publication_ownership_progress.is_none(),"worker preparation receipt awaits its original recipient");operation.publication_ownership_progress=Some((grant,spent));}
        }
        #[cfg(target_arch="wasm32")]
        if let Some(now_ms)=semio_framework_job::default_now_ms(){semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get))).pump(now_ms);}
        match failure{Some(failure)=>Err(failure.into()),None=>Ok(())}
    }

    /// 🔎️ Whether any live envelope decode still has worker close or step work this turn can run.
    fn has_runnable_artifact_envelope_decode(&self) -> bool {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| self.envelope_decode_jobs.entry(index).is_some_and(|(_, active)| active.has_runnable_work()))
    }

    fn has_runnable_artifact_envelope_decode_worker_step(&self) -> bool {
        self.has_runnable_artifact_envelope_decode()
    }

    /// 🎯️ The next mounted operation this turn's one publication unit can actually advance, taken from
    /// the rotating publication cursor. An `AwaitingAck` operation whose page is already out keeps the
    /// turn in `MoreWork` — the host owes it an acknowledgement event — but owns no step of its own
    /// until that event lands, so spending the turn's only unit on it starves every sibling that does
    /// have one (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16).
    /// 🚫️async: E1 pure census over an already-owned fixed table — see R9.
    fn next_advanceable_typed_operation(&self) -> Option<(usize, u64)> {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (self.typed_publication_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.tool_operations.entry(index).filter(|(_, operation)| operation.stage != MountedTypedCommandFullOperationStage::AwaitingAck).map(|(id, _)| (index, *id))
        })
    }

    /// 🩺️ What the publication cursor is about to advance, as the four facts that decide whether the
    /// ladder can move at all AND whether it did: the operation it selected, that operation's stage,
    /// the ownership flags the arms branch on, and the publication's own monotone progress.
    /// Compared before and after one unit by [`Self::typed_operation_stall_streak`].
    ///
    /// 🐛️ The progress term is the fourth fact because the first three are constant for a HEALTHY
    /// publication: an edit of 5 745 mutations (puzzle5d's `capsule-dream` example switch) folds one
    /// bounded mutation per unit for ~19 000 units with the same operation id, the same `Publishing`
    /// stage and the same ownership flags, so the streak counter could not tell it from an owner that
    /// can never advance and terminated it at [`TYPED_OPERATION_STALL_FAULT_CEILING`] — about a fifth
    /// of the way through — with `interactive-job.publication-stalled`. Measured by the
    /// "Semio-tech play" session on 2026-09-21. Raising the ceiling would only move the cliff to the
    /// next document size and re-open the silent spins the guard exists to catch.
    /// 🚫️async: E1 pure census over an already-owned fixed table — see R9.
    fn typed_operation_stall_witness(&self) -> Option<(u64, u8, u8, u64)> {
        let (_, operation_id) = self.next_advanceable_typed_operation()?;
        let operation = self.tool_operations.get(operation_id)?;
        let stage = match operation.stage {
            MountedTypedCommandFullOperationStage::Worker => 0,
            MountedTypedCommandFullOperationStage::Publishing => 1,
            MountedTypedCommandFullOperationStage::AwaitingAck => 2,
            MountedTypedCommandFullOperationStage::Retiring => 3,
        };
        let flags = u8::from(operation.session.is_some())
            | u8::from(operation.session_rejected.is_some()) << 1
            | u8::from(operation.completion.is_some()) << 2
            | u8::from(operation.publication.is_some()) << 3
            | u8::from(operation.pending_artifact_publication.is_some()) << 4
            | u8::from(operation.worker_outcome_pending) << 5
            | u8::from(operation.terminal_seen) << 6
            | u8::from(operation.result_page.is_some()) << 7;
        Some((operation_id, stage, flags, operation.publication_progress))
    }

    /// 🩺️ How many consecutive publication units advanced the SAME operation without changing any of
    /// [`Self::typed_operation_stall_witness`]'s three facts — `Some(streak)` only on a power of two at
    /// or above [`TYPED_OPERATION_STALL_WITNESS_FLOOR`], so one stalled operation costs at most a
    /// handful of lines however long it spins.
    ///
    /// 🐛️ A mounted operation that never publishes its terminal keeps the actor in `MoreWork` with no
    /// effect, no patch and no fault, which every other instrument reads as silence: the host's own
    /// drain polls forever, the reactor's more-work streak names only `typed_operation` as its source,
    /// and nothing says WHICH arm of the publication ladder is not moving. Measured 2026-09-20 on the
    /// signed-in `s` Home, whose `applyDirectoryEventPage` operation spun for the whole session
    /// (ticket 26/09/18 S8).
    fn typed_operation_stall_streak(before: (u64, u8, u8, u64), after: (u64, u8, u8, u64)) -> Option<u64> {
        static WITNESS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        static STREAK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = (after.0.wrapping_shl(16) | u64::from(after.1).wrapping_shl(8) | u64::from(after.2)) ^ after.3.wrapping_mul(TYPED_OPERATION_STALL_PROGRESS_MIX);
        if before != after || WITNESS.swap(key, std::sync::atomic::Ordering::Relaxed) != key {
            STREAK.store(0, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        let streak = STREAK.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
        (streak >= TYPED_OPERATION_STALL_WITNESS_FLOOR && streak.is_power_of_two()).then_some(streak)
    }

    pub(crate) async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let before = self.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.typed_operation_stall_witness()) {
            if let Some(streak) = Self::typed_operation_stall_streak(before, after) {
                let verb = self.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
                if after.1 == TYPED_OPERATION_STALL_PUBLISHING_STAGE && streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    self.fault_stalled_typed_operation_publication(after.0, streak)?;
                }
            }
        }
        advanced
    }

    /// 🛑️ Terminates the ONE `Worker`-stage operation whose own step answered a structural
    /// `Err` — an operation that lost its persistent worker session, or whose mounted session
    /// reported a state its own ladder cannot continue from — with that operation's terminal
    /// fault page, and lets the turn carry on.
    ///
    /// 🎯️ Why it is not a `?`: `drive_typed_operation_worker` is called from
    /// [`Self::advance_typed_operation_publication_unit`], the actor's ONE publication unit per
    /// turn. Propagating the `Err` aborted that unit for the WHOLE actor, so a single
    /// structurally faulted operation stopped every other mounted operation from ever advancing
    /// again — with no fault page, no effect and no patch to say why. The `Publishing` lane has
    /// answered this correctly since wave B16 (`advance_typed_operation_publication_unit`'s
    /// retry ladder, and [`Self::fault_stalled_typed_operation_publication`]'s own doc: a
    /// per-operation failure becomes that operation's fault, never the turn's); the `Worker`
    /// lane was the one lane that did not honour it.
    ///
    /// 🔒️ The operation is terminal the moment its worker ladder refuses: the lease is
    /// cancelled so no publication claim can be taken again, the fault page is minted against
    /// the operation's own token, and the stage moves to `Publishing` so the ordinary
    /// cancelled-publication close (`reject_cancelled_publication`) retires everything it still
    /// owns. Nothing is dropped here — the retained owners leave through the close ladder they
    /// always did.
    fn fault_typed_operation_worker(&mut self, operation_id: u64, fault: &Fault) -> Result<(), Fault> {
        let Some(mounted) = self.tool_operations.get_mut(operation_id) else { return Ok(()) };
        if mounted.stage != MountedTypedCommandFullOperationStage::Worker || mounted.result_page.is_some() {
            return Ok(());
        }
        let bounded = ArtifactBoundedToolFault::from_fault(fault);
        if let Some(lease) = mounted.cancellation_lease.as_ref() {
            lease.cancel();
        }
        let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
        let framed_len = bounded.framed_page_bytes(&mut framed);
        mounted.stage = MountedTypedCommandFullOperationStage::Publishing;
        let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
        mounted.terminal_fault = Some(bounded);
        mounted.queue_page(page)
    }

    /// 🛑️ Terminates a `Publishing`-stage operation whose ladder has not moved for
    /// [`TYPED_OPERATION_STALL_FAULT_CEILING`] units with a real fault page instead of leaving it to
    /// spin. The publication ladder's non-advancing answers are all `Ok(())` by design — a completion
    /// that is not ready yet, and `ArtifactStoreOneItemAdvance::Blocked` from a store owner that
    /// refuses the granted page — so an owner that can NEVER advance pins its actor in `MoreWork`
    /// with no effect, no patch and no fault, which no instrument reads as a failure.
    ///
    /// Only the `Publishing` stage is bounded this way: a `Worker`-stage operation's progress is its
    /// own job's business and a long tool run legitimately leaves this witness unchanged for as long
    /// as it runs. Measured first on the signed-in `s` Home, whose `applyDirectoryEventPage` config
    /// publication answered `Blocked` on every unit (ticket 26/09/18 S8).
    fn fault_stalled_typed_operation_publication(&mut self, operation_id: u64, streak: u64) -> Result<(), Fault> {
        let Some(mounted) = self.tool_operations.get_mut(operation_id) else { return Ok(()) };
        if mounted.stage != MountedTypedCommandFullOperationStage::Publishing || mounted.result_page.is_some() {
            return Ok(());
        }
        let fault = ArtifactBoundedToolFault::from_fault(&Fault::new(
            FaultOrigin::Framework,
            FaultCode::new("interactive-job.publication-stalled"),
            format!("typed operation '{}' advanced {streak} publication units without moving its ladder", mounted.verb),
        ));
        if let Some(lease) = mounted.cancellation_lease.as_ref() {
            lease.cancel();
        }
        let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
        let framed_len = fault.framed_page_bytes(&mut framed);
        let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
        mounted.terminal_fault = Some(fault);
        mounted.queue_page(page)
    }

    /// ♻️ A retiring operation answers `has_runnable_typed_operations`, so the turn driver that reports it
    /// runnable must be the one that releases its slot — see [`Self::retire_typed_operation_run`].
    ///
    /// 🕹️ "After EVERY artifact (document) dispatch, re-derive fresh topology and prune any
    /// selection/hover id no longer present" — the clause `dispatch_emit_inner` carries, which
    /// the migrated publication ladder never reaches (see the emit backstops above it). A
    /// migrated dispatch's document change lands on the Artifact lane, so this is where the
    /// revalidation belongs; `mounted` is out of the registry here, so the async pass is free
    /// to borrow the app, and the flag makes it exactly one pass per operation.
    async fn advance_typed_operation_publication_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if !self.latest_wins_commands.is_empty() && (self.latest_wins_turn || self.tool_operations.is_empty()) {
            self.latest_wins_turn = false;
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.latest_wins_turn = true;
        let Some((index, operation_id)) = self.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        if self.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
            let mounted=self.tool_operations.get_mut(operation_id).unwrap();
            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
            let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
            mounted.cancellation_lease=self.tool_cancellations.begin_keyed(&key,cx)?;
            if let Some(lease)=mounted.cancellation_lease.as_ref(){
                if mounted.terminal_fault.is_some()||mounted.user_cancel_requested{lease.cancel();}
                let maximum=self.qualified_tool_proof(&self.tool_operations.get(operation_id).unwrap().verb)?.contract().max_output_bytes;
                let mounted=self.tool_operations.get_mut(operation_id).unwrap();let lease=mounted.cancellation_lease.as_ref().unwrap();
                let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
                mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);
            }
            return Ok(());
        }
        if self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker) {
            record_typed_operation_unit(TypedOperationUnitKind::Worker);
            return self.drive_typed_operation_worker(operation_id,cx);
        }
        if self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
            record_typed_operation_unit(TypedOperationUnitKind::Retirement);
            return self.retire_typed_operation_run(operation_id,cx);
        }
        if !self.tool_operations.get_mut(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Publishing) {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        }
        let mut mounted =
            self.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority"), "typed-operation publication owner changed before one-unit transfer"))?;
        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;
        let mut driver_result = Ok(());
        if let Err(failure)=outcome{self.tool_operations.insert_admitted(operation_id,mounted);return Err(failure);}
        mounted.publication_attempt=0;
        if driver_result.is_ok() && mounted.published_artifact && !mounted.interaction_revalidated {
            mounted.interaction_revalidated = true;
            let meta = mounted.meta.clone();
            let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
            let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
            let token = lease.token.clone();
            let publication = lease.publication_claim.clone();
            let mut progress = mounted.identity_progress;
            let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
            driver_result = self.revalidate_interaction_on_document_change(&meta, &mut identity).await;
            mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
            drop(observer);
            mounted.identity_progress = progress;
        }
        self.tool_operations.insert_admitted(operation_id, mounted);
        driver_result.map_err(Into::into)
    }

    /// 🏃️ Drives ONE mounted operation's publication ladder for a bounded slice of this continuation
    /// unit — the exact twin of the slice [`Self::drive_typed_operation_worker`] gives a Worker-stage
    /// operation and [`Self::retire_typed_operation_run`] gives a retiring one, for the publication
    /// state machine that sits between them.
    ///
    /// 🧾️ Every ladder step — install the taken completion, stage the batch, prepare one item, fold
    /// it, retire its preparation owner, validate the cursor, preflight the commit, commit, mint the
    /// receipt — used to be its own host round trip, because the caller is driven once per reactor
    /// turn and the store's grant pages one item per call. One `deleteSelection` on a ONE-object
    /// document therefore cost 104–235 round trips (wave B44 §2.3): a document-INDEPENDENT constant
    /// that alone burns seconds of a 30-second interaction budget, and the reason a `setCamera` on a
    /// document it never reads cost 2.3 s.
    ///
    /// 🛑️ The run stops on exactly the protocol's own bounds and on nothing else: a queued result page
    /// the host must acknowledge before the operation may continue (`stage` leaves `Publishing`), a
    /// completion that is not ready yet so nothing is staged to advance, [`TYPED_OPERATION_PUBLICATION_PUMPS`],
    /// and [`INTERACTIVE_TURN_WORKER_WALL_US`]. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
    pub(crate) async fn publish_mounted_typed_operation_run(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.identity.is_none() {
            let completion = mounted.completion.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original effect completion"))?;
            let (receipt, progress) = completion.take_authoring_identity()?.ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original app-owned effect receipt"))?;
            mounted.identity = Some(receipt);
            mounted.identity_progress = progress;
        }
        let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its admitted authoring receipt"))?;
        let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original cancellation lease"))?;
        let token = lease.token.clone();
        let publication = lease.publication_claim.clone();
        let mut progress = mounted.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.publish_mounted_typed_operation_run_admitted(mounted, &mut identity,cx,retained).await {
            Ok(()) => self.follow_derivable_children(&mut identity).await.map_err(Into::into),
            Err(fault) => Err(fault),
        };
        mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        mounted.identity_progress = progress;
        result
    }

    async fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.publication_ownership_progress.is_some(){return Ok(());}
        let mut spent = RetainedCloneProgress::default();
        let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
        if spent != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
        }
        outcome
    }

    /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
    async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
        let started_us=cx.now_us();
        let turn = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            if cx.should_yield(){return Ok(())}
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();
            if mounted.owned_child_group.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_owned_child_operation_unit(mounted,cx).await?;
            } else if mounted.pending_child_publication.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_child_operation_unit(mounted, identity).await?;
            } else if Self::mounted_typed_interaction_writes_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity).await?;
            } else if self.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await?;
            } else {
                let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                let receipt = mounted.publication_ownership_progress.take();
                if let Some((_, unit)) = receipt {
                    cx.consume_retained(unit)?;
                    *spent = spent.checked_add(unit).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    if unit.copied_items == 0 || crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent).maximum_items == 0 { return Ok(()); }
                }
                if self.store.backbone_ref().is_some() {
                    self.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    }
                }
                record_typed_operation_unit(if mounted.result_page.is_some() { TypedOperationUnitKind::Page } else { TypedOperationUnitKind::Store });
                if receipt.is_none() && before == (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage) {
                    return Ok(());
                }
            }
            if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
            if mounted.owned_child_group.is_some(){let delta=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;*spent=spent.checked_add(delta)?;}
            if mounted.stage != MountedTypedCommandFullOperationStage::Publishing {
                return Ok(());
            }
            if mounted.publication.is_none() && mounted.pending_artifact_publication.is_none() && mounted.pending_child_publication.is_none() && mounted.owned_child_group.is_none() {
                return Ok(());
            }
            let Some(started_us) = started_us else { return Ok(()) };
            if cx.now_us().is_some_and(|now_us| now_us.saturating_sub(started_us) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 🕹️ Whether this operation's next publication unit is its [`InteractionWrite`] lane: every
    /// durable store lane of its emit is already drained (a selection may only name ids the
    /// document already holds), and there is at least one write left to apply. Sync and
    /// allocation-free so the async caller can branch on it before mounting anything.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🧺️ A batched durable lane empties its emit vector the moment it is admitted, so the
    /// in-flight publication itself — not the drained vector — is what still owes a turn.
    fn mounted_typed_interaction_writes_are_next(mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        !emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
    }

    /// 🕹️ Publishes this operation's whole [`InteractionWrite`] lane as ONE unit through the same
    /// `apply_interaction_writes` single-writer path `dispatch_emit` uses — the retained-tool twin
    /// of that call. It is one unit rather than one-write-per-turn because a selection is a single
    /// state-machine transition per domain: draining it write-by-write would publish intermediate
    /// selections no user ever asked for. `MountedTypedCommandFullOperation` needs no extra field:
    /// the writes live in the retained emit until taken here, and the lane is only entered once
    /// every durable lane is drained (`mounted_typed_interaction_writes_are_next`).
    async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation interaction publication lost its emit owner"));
        };
        let writes = std::mem::take(&mut emit.interaction_writes);
        let meta = mounted.meta.clone();
        if let Err(fault) = self.apply_interaction_writes(&writes, &meta, identity).await {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();
        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
        mounted.queue_page(page)
    }

    /// 🔀️ Whether this operation's next publication unit is the inline fold of its emit's
    /// guest-emitted interaction verbs (`Effect::ReplayShellCommand { action_id ∈
    /// INTERACTION_ACTION_IDS }` or `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }`,
    /// see [`is_inline_interaction_verb`]): every durable store lane is drained (a pick may only name ids
    /// the document already holds), the `InteractionWrite` lane went first (it precedes the verb
    /// in the emit's own order, exactly as it did when the verb still took a host round trip),
    /// and at least one such verb is still in the effect list. Sync and allocation-free like its
    /// `mounted_typed_interaction_writes_are_next` twin.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🅿️ The verbs were parked at install (`typed_inline_interaction_verbs`, keyed by this
    /// operation); the effect-list scan stays as the belt to that brace, for an emit whose
    /// effects were (re)filled after install.
    fn mounted_typed_inline_interaction_verbs_are_next(&self, mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        let parked = self.typed_inline_interaction_verbs.iter().any(|(operation, verbs)| *operation == mounted.operation.operation.0 && !verbs.is_empty());
        (parked || emit.effects.iter().any(is_inline_interaction_verb))
            && emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.window_config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
            && ephemeral.window_transient.is_empty()
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the typed-operation twin of
    /// `dispatch_emit`'s inline interaction fold, for the ladder every app command
    /// (`dispatch_typed_command_inner`) and retained tool publishes through and that never enters
    /// `dispatch_emit`. Peels every interaction verb out of the retained emit's effect list as ONE
    /// unit (the same one-transition-per-domain reasoning as the `InteractionWrite` unit above),
    /// applies each through `apply_interaction_verb`, and widens the emit's `ui_scope` with the
    /// verbs' app-declared refresh scopes so the `Ui` page and the completion witness — published
    /// later in this same ladder — already carry them. No result page is queued: the host has
    /// nothing to acknowledge, because the verb it used to be asked to replay never leaves the
    /// reactor, and the ladder simply continues with the remaining effects next pump.
    ///
    /// 🩹 A verb that could not be applied is reported on the runtime's `[TRACE]` diagnostics line
    /// and dropped — the typed ladder has no `InvocationResult.diagnostics` lane (its only fault
    /// lane is terminal), and failing the carrying command for a pick that already happened is
    /// exactly what `fold_inline_interaction_verbs` exists to prevent.
    async fn publish_mounted_typed_inline_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner"));
        };
        let mut folded: Vec<(String, Option<DslValue>)> = Vec::new();
        if let Some(index) = self.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub(crate) async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tool_cancellations,cx);
            self.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.advance_latest_wins_admission_unit(&mut pending,retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tool_operations.can_insert(operation_id) || self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
            let started = self.start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            }
            started?;
            return Ok(());
        }
        if pending.closing && pending.command_owners_are_empty() {
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "latest-wins-admission".into(),
                meta: pending.meta.clone(),
                operation: pending.operation,
                source_identity:pending.source_identity,
                canonical_revision: pending.revision,
                artifact_generation: pending.operation.generation.0,
                config_generation: 0,
                draft_generation: 0,
                presence_generation: 0,
                transient_generation: 0,
                window_config_authority: None,
                window_transient_authority: None,
                publication_lanes: &[],
                session: None,
                session_rejected: None,
                reserved_producer: None,
                completion: None,
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: None,
                cancellation_lease: pending.lease.take(),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: true,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;
            self.tool_operations.insert_admitted(operation_id, mounted);
            self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
    }

    /// 🎟️ Advances the latest-wins key registry by exactly its own quoted turn.
    fn advance_latest_wins_keys(&mut self) -> Result<(), Fault> {
        let demand = self.latest_wins_keys.advance_demands().map_err(plugin_retirement_fault)?;
        self.latest_wins_keys.advance(plugin_demand_grant(demand))?;
        Ok(())
    }

    /// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: a rebase re-bases the LEASE, it does not
    /// change the key. The key is `(instance, envelope id, controller id, tool id, target)` — not one
    /// of which depends on the document revision or generation — so retiring it here only made the
    /// restart throw away an in-flight `ToolLatestWinsKeyCopy` one character per host continuation and
    /// rebuild it from scratch, several hundred round trips per rebase. And the registry's in-flight
    /// unit is driven here for ANY owner, not only for this one: `active_operation` is cleared solely
    /// by `ToolLatestWinsRegistry::advance`, and a sibling that was accepted and started leaves its
    /// closing update behind, so gating the drive on `active_operation == Some(operation)` let a
    /// restarting front-of-FIFO command wait forever on a registry no one would advance.
    fn advance_latest_wins_admission_unit(&mut self,pending:&mut PendingLatestWinsCommand<A>,retained:RetainedCloneGrant)->Result<bool,crate::PluginLifecycleFailure>{
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(false);}
        let operation=pending.operation.operation.0;
        if pending.lease.as_ref().is_none_or(|lease|lease.token.is_cancelled_now()){pending.closing=true;}
        if pending.closing{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);let step=if self.latest_wins_keys.active_operation==Some(operation){PluginLifecycleStep::Progress(self.latest_wins_keys.advance(retained)?.progress())}else{pending.close_step(retained)?};pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(false);}
        let live_revision=self.store.content_revision_now();if live_revision!=pending.revision||self.store.generation_now()!=pending.operation.generation.0{pending.restarting=true;}
        if pending.restarting{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);if !self.latest_wins_keys.can_begin(){let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;return Ok(false);}let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(live_revision[..8].try_into().expect("original revision width")));let generation=semio_framework_job::Generation(self.store.generation_now());if !pending.lease.as_mut().expect("original keyed rebase lease").rebind_keyed(base_revision,generation)?{return Ok(false);}pending.lookup_started=false;pending.accepted=false;pending.restarting=false;pending.revision=live_revision;pending.operation.base_revision=base_revision;pending.operation.generation=generation;pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false);}
        if pending.accepted{let step=pending.close_key_step(retained)?;pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(matches!(step,PluginLifecycleStep::Complete(_)));}
        if pending.lookup_started{if let Some(accepted)=self.latest_wins_keys.take_outcome(operation){pending.accepted=accepted;pending.closing=!accepted;if !accepted{pending.lease.as_ref().expect("original pending lease").cancel();}pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        if let Some(key)=pending.key.as_ref(){if let Some(progress)=self.latest_wins_keys.begin(operation,key,pending.lease.as_ref().expect("original pending lease"),retained)?{pending.lookup_started=self.latest_wins_keys.active_operation==Some(operation);pending.record_original_receipt(retained,progress)?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        let admission=pending.admission.as_ref().expect("original pending admission");let QualifiedToolProof::AppOwned(registration)=&admission.proof else{return Err(plugin_sdk_fault("latest-wins admission lost its concrete factory").into());};let command=pending.command.as_ref().expect("original pending command");let target=(registration.latest_wins_target)(command.as_ref()).ok_or_else(||plugin_sdk_fault("latest-wins original command lost its borrowed target"))?;let parts=[&*self.store.envelope().id,&*registration.key.controller_id,&*registration.key.tool_id,target];
        if pending.key_copy.is_none(){if let Some((copy,progress))=ToolLatestWinsKeyCopy::admit_original(pending.meta.instance_id,parts,retained).map_err(|error|super::child_emit_preparation::retirement_refusal_fault(&error))?{pending.key_copy=Some(copy);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let copy=pending.key_copy.as_mut().expect("original key byte cursor");
        if copy.part>4{if let Some((key,progress))=copy.take_key(retained).map_err(|error|super::child_emit_preparation::retirement_refusal_fault(&error))?{pending.key=Some(key);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let step=copy.advance(parts,retained).map_err(|error|super::child_emit_preparation::retirement_refusal_fault(&error))?;pending.record_original_receipt(retained,step.progress())?;Ok(false)
    }

    /// 🧩️ The composed lane is PUBLISHED, not discarded: `dispatch_emit_group` has just
    /// built the parent's mutations, one per touched child under the child's own
    /// content-addressed handle, and the single `UndoGroup` that names them all. The
    /// three scalars of `receipt` are what the host ACKs; this is what the host READS.
    async fn publish_mounted_typed_child_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if mounted.pending_child_publication.as_ref().is_some_and(|owner|matches!(owner.phase,PendingChildGroupPublicationPhase::Acknowledged|PendingChildGroupPublicationPhase::Closing)){return Ok(());}
        let Some(mut pending) = mounted.pending_child_publication.take() else {
            return Err(plugin_sdk_fault("typed-operation child publication lost its retained owner"));
        };
        match pending.phase {
            PendingChildGroupPublicationPhase::Acknowledged | PendingChildGroupPublicationPhase::Closing => {
                mounted.pending_child_publication=Some(pending);
                return Err(plugin_sdk_fault("original mounted child publication escaped its full-grant close frontier"));
            }
            PendingChildGroupPublicationPhase::Committed => {
                let receipt = pending.receipt.as_ref().cloned().ok_or_else(|| plugin_sdk_fault("committed child publication lost its exact result receipt"))?;
                mounted.published_artifact = true;
                self.record_typed_operation_lane(mounted);
                mounted.pending_child_publication = Some(pending);
                return mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
            }
            PendingChildGroupPublicationPhase::Faulted => {
                let fault = pending.fault().unwrap_or_else(|| plugin_sdk_fault("typed-operation child publication fault lost its bounded detail"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            PendingChildGroupPublicationPhase::Ready => {}
            PendingChildGroupPublicationPhase::Dispatching => {
                mounted.pending_child_publication = Some(pending);
                return Err(plugin_sdk_fault("typed-operation child publication re-entered a non-dispatchable phase"));
            }
        }
        match self.reclaim_child_retirements(1, 0) {
            Ok(true) => {}
            Ok(false) => {
                mounted.pending_child_publication = Some(pending);
                return Ok(());
            }
            Err(error) => {
                let fault = pending.reject_and_fault(&error);
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let live_revision = self.store.content_revision_now();
        let live_generation = self.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_content_generation && captured.identity_digest() == self.child_content_root.identity_digest();
        if !parent_fresh || !child_root_fresh {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale parent or child immutable roots"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let mut seen = HashSet::with_capacity(pending.child_emits.len());
        for child in pending.child_emits.iter() {
            if !seen.insert((child.slot.clone(), child.child_id.clone())) {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication contains a duplicate exact target"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            let expected_revision = match captured.revision(&child.slot, &child.child_id) {
                Ok(revision) => revision,
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            };
            let Some(expected_dialect) = captured.dialect(&child.slot, &child.child_id) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from its captured root"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            let Some(entry) = self.children.get(&(child.slot.clone(), child.child_id.clone())) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from the live member registry"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            if entry.reference.dialect != expected_dialect || entry.member.content_revision().await != expected_revision {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale member identity, dialect, or revision"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        if mounted.cancellation_lease.as_ref().expect("claimed child publication retains its cancellation lease").is_cancelled().await {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication was cancelled before group linearization"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        if let Err(error) = pending.begin_dispatch() {
            let fault = pending.reject_and_fault(&error);
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.store.generation_now();
                mounted.canonical_revision = self.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_content_generation };
                if self.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
                    let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation composed gesture receiver is saturated"));
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                if let Err(error) = pending.commit(receipt.clone()) {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                mounted.pending_child_publication = Some(pending);
                mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
            }
            Err(fault) => {
                let fault = pending.reject_and_fault(&fault);
                mounted.pending_child_publication = Some(pending);
                Err(fault)
            }
        }
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — THE point the reactor first owns a
    /// typed command's emit. The interaction verbs are peeled here, before this same call
    /// can reach the effect lane below: an emit with no durable lane (fem2d's
    /// `canvasPointerUp` is `effects: [interactionSelect], ui_scope: partial` and nothing
    /// else) used to fall straight through to `emit.effects.pop()` → `typed_effect_outbox`
    /// on the install pump, while `mounted_typed_inline_interaction_verbs_are_next` had
    /// never seen the emit (it gates on `mounted.publication`, which was still `None`
    /// when this pump's ladder branch was chosen). Per-turn law: no host-visible turn of a
    /// typed operation ever carries one of the six interaction verbs as an effect.
    ///
    /// 🪜️ Hand the pump back to the ladder so its own lane order picks the first unit: the
    /// `InteractionWrite` lane and the inline fold both precede the effect/`Ui`/terminal
    /// pages of this same emit, and the fold must land BEFORE the `Ui` page is minted so
    /// that page already carries the verbs' widened scope and their leftover view.
    ///
    /// 🔒️ The three emit-time backstops of contract §2.3 clause 2, §5.10 and the tool-run
    /// freeze live in `dispatch_emit_inner`, which the migrated publication ladder never
    /// calls for the single-store artifact lane (FP5 §3.4 measured the same thing for the
    /// kind guard above). A migrated dispatch answers with an admission receipt and its
    /// mutations only exist HERE, so this is the only point at which they can be refused.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): apply NOTHING and stash a
    /// `TransactionProposalDraft` instead, drained once by `plugin_exchange`.
    ///
    /// 🧺️ ONE gesture is ONE batched publication: the whole artifact lane is drained
    /// into a single staged `Edit` (one history ledger slot, one undo step), instead
    /// of one edit per mutation per turn — see `store::begin_apply_batch`.
    ///
    /// 🧵️ A config lane is one plain edit per emission (design §20.1): a continuous view/config gesture rides its press
    /// in the window transient and commits its ONE config edit at gesture end (`settle_press`), so nothing here folds
    /// ticks into an earlier edit.
    ///
    /// 🎚️ The authority was captured when this command was DISPATCHED, and a window's
    /// config partition is its OWN exact-base document: any earlier command that
    /// settled against the same window has already moved that partition's generation
    /// and revision on, so a second window-config command dispatched in the same turn
    /// carries a stale base by the time its batch begins. `begin_apply_batch` then
    /// refuses it — and because the mutation was already POPPED out of `emit`, the
    /// retry `advance_typed_operation_publication_unit` grants on a publication fault
    /// found nothing left to publish and the operation completed clean: no page, no
    /// fault, the amend lost (measured on flow as one window-config page for two
    /// commands, ticket 26/09/19 `📓️flow.md` §5.2). Two repairs, both of which every
    /// sibling lane of this ladder already had: re-read the authority before beginning
    /// (the window-TRANSIENT arm below always did), and hand the mutation BACK to
    /// `emit` when admission refuses it, so a retry is a real retry and an exhausted
    /// one refuses by name on the Fault lane instead of vanishing.
    ///
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4) — the
    /// task lane, ONE task per publication unit. The unit spends itself here
    /// and hands the pump straight back to the ladder without a host page of
    /// its own: a spawn is actor-local bookkeeping, and the task's eventual
    /// `TaskResolution` re-enters as its own follow-up dispatch
    /// (`drain_task_resumes`), never as this operation's result. `remove(0)`
    /// because spawn order is observable — a same-key respawn cancels the
    /// live task under that key, so the LAST declared task under a key must
    /// be the one that survives. A refused spawn (quota, key supersession,
    /// executor capacity) is this operation's own `Fault`: the ladder above
    /// gives the slot a chance to free and then mints its terminal fault page.
    fn handoff_mounted_durable_publication(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<bool, Fault> {
        let Some(pending) = mounted.pending_artifact_publication.take() else {
            return Ok(false);
        };
        let (result, restore) = match pending {
            PendingArtifactStorePublication::Artifact(publication) => {
                let mut owner = Some(publication);
                let result = self.store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Artifact))
            }
            PendingArtifactStorePublication::Config(publication) => {
                let mut owner = Some(publication);
                let result = self.config_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Config))
            }
            PendingArtifactStorePublication::Draft(publication) => {
                let mut owner = Some(publication);
                let result = self.draft_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Draft))
            }
            pending => {
                mounted.pending_artifact_publication = Some(pending);
                return Ok(false);
            }
        };
        if let Some(owner) = restore {
            mounted.pending_artifact_publication = Some(owner);
        }
        result
    }

    /// 🪟️ Continues the window-config authority refresh a published receipt retains until the displaced snapshot has admitted custody, then queues the retained receipt's result page; every unit reports its actual progress to the host turn.
    fn advance_mounted_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant) -> Result<(), Fault> {
        let receipt = mounted.pending_window_config_receipt.ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;
        let authority = mounted.window_config_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window config receipt lost its captured window authority"))?;
        self.window_config_store.refresh_demands(authority, retained.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        let step = self.window_config_store.refresh(authority, retained)?;
        let (super::window_mutation::WindowAuthorityRefreshStep::Pending(progress) | super::window_mutation::WindowAuthorityRefreshStep::Ready(progress)) = step;
        if progress != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "window config refresh cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((retained, progress));
            mounted.publication_progress = mounted.publication_progress.saturating_add(1);
        }
        if matches!(step, super::window_mutation::WindowAuthorityRefreshStep::Pending(_)) { return Ok(()); }
        mounted.pending_window_config_receipt = None;
        self.tool_runs.note_window_config_published();
        let token = mounted.next_token();
        mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)
    }

    pub(crate) fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if Self::mounted_typed_interaction_writes_are_next(mounted) || self.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.store.content_revision_now();
        let live_generation = self.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.advance_mounted_window_config_refresh(mounted, retained); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.reclaim_document_snapshot_read_returns()?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.config_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.draft_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.presence_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.transient_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt) => {
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.store.content_revision_now();
                            mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                            mounted.operation.generation = semio_framework_job::Generation(receipt.generation_after);
                        }
                        TypedOperationResultLane::Config => {
                            mounted.published_config = true;
                            mounted.config_generation = receipt.generation_after;
                        }
                        TypedOperationResultLane::Draft => mounted.draft_generation = receipt.generation_after,
                        TypedOperationResultLane::Presence => mounted.presence_generation = receipt.generation_after,
                        TypedOperationResultLane::Transient => mounted.transient_generation = receipt.generation_after,
                        TypedOperationResultLane::WindowConfig => {
                            mounted.published_window_config = true;
                            mounted.pending_window_config_receipt = Some(receipt);
                            return self.advance_mounted_window_config_refresh(mounted, retained);
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    let token = mounted.next_token();
                    mounted.queue_page(TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)?)
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant"));}
                    assert!(mounted.publication_ownership_progress.is_none(),"typed preparation cannot overwrite an unreturned original receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused => {
                    let reason=pending.preparation_refusal().ok_or_else(||plugin_sdk_fault("typed preparation refused without its original retained failure"))?;
                    let ownership=reason.retained_progress();
                    assert!(mounted.publication_ownership_progress.is_none(),"typed refusal cannot overwrite its unreturned actual receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Err(plugin_sdk_fault("artifact-store publication awaits an ACK without its retained host page")),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK")),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let page = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, &framed[..len])?
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane"));
                }
                if let Some(mutation) = ephemeral.presence.pop() {
                    match self.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.presence_one_item_factory.as_deref(), self.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.transient_one_item_factory.as_deref(), self.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else {
                    TypedOperationResultPage::try_serialize(
                        token,
                        TypedOperationResultLane::Download,
                        &DownloadResultPayload { filename: download.filename.as_str(), mime_type: download.mime_type.as_str(), encoding: download.encoding.as_deref(), bytes: download.chunks.bytes() },
                    )?
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let preparation=match emit.prepare_child_one(retained){Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy"));}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault);
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy"));}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault)}
                if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
                    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
                    || (!emit.window_config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowConfig))
                    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
                    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                    || ((!emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
                    || (!emit.interaction_writes.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Interaction))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.require_operation_emitting_kind(&mounted.verb, self.declared_dispatch_kind(&mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if self.tool_runs.freezes_local_emits() {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.busy"), format!("verb {:?} would emit artifact mutations while a freezing tool run is non-terminal on this instance", mounted.verb)));
                    }
                    if let Some(pending_txn_id) = self.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                            emit.artifact_mutations.clear();
                            return Ok(());
                        }
                    }
                }
                if let Some(fault) = tool_transaction_shape_fault(
                    &mounted.verb,
                    emit.transaction.as_ref(),
                    emit.transaction_phase,
                    !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty(),
                    !emit.artifact_mutations.is_empty(),
                    emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps),
                ) {
                    return Err(fault);
                }
                if (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
                    return Err(time_travel_frozen_fault(&mounted.verb));
                }
                self.close_streamed_transaction_unit(emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.admit_private_child_group_frame(operation,captured_millis,u64::MAX,grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.presence_one_item_factory.as_deref(), self.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.transient_one_item_factory.as_deref(), self.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner"));
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    Self::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
                                    return Err(plugin_sdk_fault("typed-operation completion receiver is saturated"));
                                }
                                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, b"typed-operation-complete")?
                            }
                        }
                    }
                }
            }
        };
        mounted.queue_page(page)
    }

    fn require_tool_operation_authority(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        let contract = admission.proof.contract();
        let exact = admission.decoded_items <= contract.max_decoded_items
            && contract.max_work_units_per_step != 0
            && contract.max_output_bytes != 0
            && contract.max_step_micros != 0
            && contract.max_step_micros < semio_framework::ToolExecutionContract::INTERACTIVE_MAX_STEP_MICROS
            && matches!(contract.cancellation, semio_framework::ToolCancellationPolicy::PerOperation)
            && matches!(contract.freshness, semio_framework::ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure)
            && matches!(contract.shape, semio_framework::ToolExecutionShape::Resumable | semio_framework::ToolExecutionShape::BoundedFirstStep);
        if !exact {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.incomplete-operation-authority"), format!("typed command '{}' has no complete bounded prepare/reducer/commit authority", admission.verb)));
        }
        Ok(())
    }

    fn require_complete_tool_operation_pipeline(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        self.require_tool_operation_authority(admission)?;
        match admission.proof {
            QualifiedToolProof::AppOwned(_) => Ok(()),
            QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {
                Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{}' has no exact app-owned retained decoder/reducer factory", admission.verb)))
            }
        }
    }

    pub(crate) fn can_admit_typed_operation(&self, operation: u64) -> bool {
        self.tool_operations.can_insert(operation) && self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_none()
    }

    /// 🎰️ Whether every fixed typed-operation and segmented-output authority owns nothing in this one
    /// residue class of `id % ARTIFACT_LIVE_OUTPUT_SLOTS`.
    pub(crate) fn typed_operation_slot_is_vacant(&self, slot: usize) -> bool {
        self.tool_operations.slot_is_vacant(slot)
            && self.typed_operation_reservations[slot].is_none()
            && self.latest_wins_commands.slot_is_vacant(slot)
            && self.pending_reserved.slot_is_vacant(slot)
            && self.segmented_downloads.slot_is_vacant(slot)
            && self.segmented_closures.slot_is_vacant(slot)
    }

    /// 🎰️ How many of the fixed residue classes currently own a live typed operation or segmented
    /// output — the exact complement of what [`Self::admit_typed_operation_slot`] can still hand out,
    /// so `0` is the only value that proves every admitted slot reached its release path.
    fn live_typed_operation_slots(&self) -> usize {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter(|slot| !self.typed_operation_slot_is_vacant(*slot)).count()
    }

    /// 🎫️ Pre-admits the exact operation slot BEFORE an operation id exists. Every typed authority is a
    /// direct-mapped table addressed by `id % ARTIFACT_LIVE_OUTPUT_SLOTS`, so minting a shared-counter id
    /// first and then testing its one residue class refuses an action whenever an unrelated live
    /// operation happens to share that class — a hover/tick/camera storm then fails while 63 slots sit
    /// empty. Ingress therefore chooses the vacant slot and mints the id for it, and refusal now means
    /// actual saturation of all fixed slots.
    pub(crate) fn admit_typed_operation_slot(&self) -> Option<semio_framework_job::OperationId> {
        if !self.latest_wins_order.allocation_admitted || self.latest_wins_order.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            return None;
        }
        let slot = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.typed_operation_slot_is_vacant(*slot))?;
        Some(semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64))
    }

    pub(crate) async fn dispatch_typed_command_inner(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if self.live_runtime_instance_id != Some(meta.instance_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.live-instance"), format!("typed command '{}' does not belong to the mounted live app instance", admission.verb)));
        }
        self.require_complete_tool_operation_pipeline(&admission)?;
        self.deliver_utility_host_events(meta, identity, cx).await?;
        let verb = A::command_id(&command).await.to_string();
        if admission.verb != verb {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.command-identity"), format!("exact admitted command '{}' decoded as '{verb}'", admission.verb)));
        }
        if let Some(fault) = self.document_loading_refusal(&verb) {
            return Err(fault);
        }
        let Some(operation_id) = self.admit_typed_operation_slot() else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"));
        };
        if self.delivering_host_event {
            self.host_event_operations.insert(operation_id.0);
        }
        if let Some(tag) = self.tool_machines.ingress.take() {
            self.tool_machines.bind(operation_id.0, tag);
        }
        if let QualifiedToolProof::AppOwned(registration) = &admission.proof {
            {
                let ordinary = (registration.latest_wins_target)(command.as_ref()).is_none();
                let disposer = (registration.latest_wins_command_disposer)()
                    .and_then(|owner| owner.downcast::<Box<dyn ArtifactOwnedDisposer<A::Command>>>().ok())
                    .map(|owner| *owner)
                    .ok_or_else(|| plugin_sdk_fault("latest-wins target requires its exact compiler-owned command retirement factory"))?;
                let revision = self.store.content_revision_now();
                let generation = semio_framework_job::Generation(self.store.generation_now());
                let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")));
                let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, operation_id.0);
                self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0);
                self.latest_wins_commands.insert_admitted(
                    operation_id.0,
                    PendingLatestWinsCommand {
                        command: ManuallyDrop::new(Some(command)),
                        disposer,
                        admission: Some(admission),
                        meta: meta.clone(),
                        operation,
                        source_identity:(cx.operation(),cx.generation()),
                        revision,
                        lease: None,
                        key_copy: None,
                        key: None,
                        retired_key: None,
                        lookup_started: false,
                        accepted: ordinary,
                        restarting: false,
                        closing: false,
                        receipt:None,
                        fault:ManuallyDrop::new(None),fault_retirement:None,
                    },
                );
                self.latest_wins_order.push(operation_id.0).expect("exact FIFO command slot was pre-admitted");
                let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
                started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
                return Ok(started);
            }
        }
        Err(plugin_sdk_fault("typed admission requires its exact retained command owner"))
    }

    /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
    /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
    /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
    /// the same verb against different state.
    async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let canonical_base_revision = self.store.content_revision();
        let window_config_authority = self.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_content_root)),
            draft_snapshot: self.draft_store.snapshot_root(),
            interaction_state: self.interaction_store.snapshot_root(),
            interaction_hover: self.interaction_hover.clone(),
            presence_peers: self.presence_store.peers_root(),
            transient: self.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.store.generation()),
            config_generation: self.config_store.generation(),
            draft_generation: self.draft_store.generation(),
            presence_generation: self.presence_store.generation().await,
            transient_generation: self.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    }

    /// 🧪️ Observes actual capture authority without exposing production dispatch preparation.
    #[cfg(test)]
    pub(crate) async fn test_capture_presence_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<(std::sync::Arc<PeerPresenceRoot>, std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>), Fault> {
        let TypedCommandRoots { peer_presence, presence_peers, .. } = self.capture_typed_command_roots(command, meta).await?;
        Ok((peer_presence, presence_peers))
    }

    /// 🧪️ Mounts an exact typed admission so laws can observe the boundary before producer completion.
    #[cfg(test)]
    pub(crate) async fn test_mount_typed_command(&mut self, command: A::Command, verb: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(verb, &payload, 1).await?;
        self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await
    }

    /// 🧵️ Captures immutable roots and mounts one admitted worker turn. Ephemeral producers
    /// execute inside their app-owned job and publish through its declared completion lanes.
    async fn start_typed_command_operation(
        &mut self,
        command: Box<A::Command>,
        admission: AdmittedToolCommand,
        meta: &ActionMeta,
        operation_id: semio_framework_job::OperationId,
        cancellation_lease: ToolCancellationLease,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        assert!(self.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
        assert_eq!(self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
        let verb = admission.verb.clone();
        let TypedCommandRoots {
            snapshot,
            config,
            history,
            children,
            draft_snapshot,
            interaction_state,
            interaction_hover,
            presence_peers,
            transient,
            peer_presence,
            canonical_base_revision,
            base_revision,
            generation,
            config_generation,
            draft_generation,
            presence_generation,
            transient_generation,
            window_config_authority,
            window_transient_authority,
        } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let parent_document_id = self.store.envelope().id.clone();
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tool_job_controller_id, base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        assert_eq!(cancellation_lease.key.base_revision, base_revision, "fresh worker receives a revision-rebound cancellation lease");
        assert_eq!(cancellation_lease.key.generation, generation, "fresh worker receives a generation-rebound cancellation lease");
        let payload_schema_id = admission.proof.schema_id();
        let qualified_key = admission.proof.key();
        let publication_lanes = admission.proof.publication_lanes();
        let completion = ArtifactToolCompletion::<A>::new();
        let mut identity_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !cancellation_lease.token.is_cancelled_now() && !cancellation_lease.publication_claim.is_cancelled();
        let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(identity_observer);
        completion.install_authoring_identity(identity, &cancellation_lease)?;
        let retained_wire_admission = admission.wire_admission.clone();
        let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
        let mut retained_wire = Some(admission.raw_wire);
        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
        let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
        let mut request_snapshot_read = if app_owned_retained_route { Some(self.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
        let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
        let owned_context = if app_owned_retained_route {
            let presence_local = self.presence_store.local_read().map_err(Fault::from)?;
            let gesture = self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta,artifact_owned_tool_birth_grant(),artifact_owned_tool_handoff_grant())?;
            Some(ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                canonical_base_revision,
                draft_generation,
                transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&children),
                    draft: std::sync::Arc::clone(&draft_snapshot),
                    transient: std::sync::Arc::clone(&transient),
                    window_config: window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.draft_store, &self.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&presence_peers), self.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())
            .with_gesture(gesture),
            artifact_owned_tool_birth_grant(),
        ).0) } else { None };
        let operation_spec = match admission.proof.clone() {
            QualifiedToolProof::Bounded(_) => {
                let job = TypedCommandFullOperationJob::<A> {
                    operation: None,
                    parent_document_id: Some(parent_document_id.clone()),
                    command: Some(std::sync::Arc::from(command)),
                    snapshot: Some(snapshot),
                    config: Some(config),
                    history: Some(history),
                    children: Some(std::sync::Arc::clone(&children)),
                    draft: Some(draft_snapshot),
                    interaction_state: Some(interaction_state),
                    interaction_hover: Some(std::sync::Arc::new(interaction_hover)),
                    peer_presence: Some(peer_presence),
                    presence_local: Some(self.presence_store.local_read().map_err(Fault::from)?),
                    presence_peers: Some(presence_peers),
                    transient: Some(transient),
                    contract: admission.proof.contract(),
                    decoded_items: admission.decoded_items,
                    phase: TypedCommandFullOperationPhase::Prepare,
                    prepare_cursor: 0,
                    reducer_cursor: 0,
                    output: Some(completion.clone()),
                    state: [0; 3],
                    fault_message: b"",
                    pending: None,
                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
                    closing: false,
                };
                semio_framework::ToolOperationSpec::new(self.tool_job_controller_id.clone(), verb.clone(), payload_schema_id, job, operation)
            }
            QualifiedToolProof::AppOwned(_) => A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
                command,
                raw_wire: raw_input.clone(),
                operation,
                controller_id: qualified_key.controller_id,
                tool_id: qualified_key.tool_id,
                payload_schema_id,
                contract: admission.proof.contract(),
                decoded_items: admission.decoded_items,
                app_instance_id: meta.instance_id,
                parent_document_id,
                canonical_base_revision,
                authoring_seed: self.authoring_seed(&meta.actor),
                snapshot,
                snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                config,
                window_config: request_window_config.take(),
                history,
                interaction_state,
                interaction_hover: std::sync::Arc::new(interaction_hover),
                context: owned_context.expect("app-owned route retains its original admitted context"),
                instance_operation_owner: self.instance_operation_owner.clone(),
                output_chunks: output_chunks.clone(),
                completion: completion.clone(),
            })
            .await?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?,
            QualifiedToolProof::FrameworkOwned(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.framework-route"), format!("framework-owned tool '{verb}' cannot enter the application typed-command reducer")));
            }
        };
        let dispatch = if app_owned_retained_route {
            self.tool_jobs
                .dispatch_wire_retained_with_spec(&retained_wire_admission, retained_wire.take().expect("app-owned retained wire owner transfers once"), None, operation_spec)
                .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?
        } else {
            self.tool_jobs.dispatch(operation_spec).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?
        };
        let session=None;let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,(cx.operation(),cx.generation())));
        self.tool_operations.insert_admitted(
            operation_id.0,
            MountedTypedCommandFullOperation {
                verb: verb.clone(),
                meta: meta.clone(),
                operation,
                source_identity:(cx.operation(),cx.generation()),
                canonical_revision: canonical_base_revision,
                artifact_generation: generation.0,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
                publication_lanes,
                session,
                session_rejected,
                reserved_producer: None,
                completion: Some(completion),
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: (!app_owned_retained_route).then_some(raw_input),
                output_chunks: Some(output_chunks),
                cancellation_lease: Some(cancellation_lease),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: false,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::clone(&children)),
                captured_child_content_generation: self.child_content_generation,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Worker,
            },
        );
        self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
        Ok(started)
    }

    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> {
        let command = Box::new(command);
        async move {
            let log_generation_before = self.log_generation;
            let verb = A::command_id(command.as_ref()).await;
            let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
            let admission = self.admit_command_wire(verb, &payload, 1).await?;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await?;
            Ok(self.finish_recorded(log_generation_before, verb, result).await)
        }
    }

    /// 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
    ///
    /// 🔒️ Contract §2.3 clause 1 — `import` is not a string action (see `VIEWER_REJECTED_ACTION_IDS`'s
    /// own doc), so it is guarded here instead of `dispatch_action`.
    async fn dispatch_import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if A::ROLE == AppRole::Viewer {
            return Err(viewer_read_only_fault("import"));
        }
        let proof = self.qualified_tool_proof("import-media")?;
        let key = proof.key();
        let admission = self.tool_jobs.admit_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), &[]).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), "media import resolved outside its exact controller/owner/factory/tool/schema proof"));
        }
        let (authority,source)=self.pending_media_import_permit.take().ok_or_else(||plugin_sdk_fault("media source requires its original pre-admitted import permit"))?;
        if (source.0,source.1)!=(cx.operation(),cx.generation()){*self.pending_media_import_permit=Some((authority,source));return Err(plugin_sdk_fault("media import requires its unchanged original caller"))}
        let (job, completion) = match self.build_artifact_reserved_media_job(port, media, meta).await { Ok(producer) => producer, Err(fault) => { self.finish_reserved_emit_producer(authority); return Err(fault); } };
        let raw = Vec::new();
        self.dispatch_reserved_original_producer("import-media",&raw,1,job,completion,meta,Some(admission),authority, cx).await
    }

    /// 🧮️ B1: dispatches a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` against
    /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_configuration_failure(cx)?{return Ok(None)}
        if self.pending_configuration_permit.is_some()&&self.pending_configuration_completion.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
        let proof=self.qualified_tool_proof("configuration-binary")?;let key=proof.key();
        let admission=match self.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.receive_configuration_failure(cx)?;return Ok(None)}};
        if self.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending_configuration_permit=Some(authority);self.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
        if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
        type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
        let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
        let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
        self.pending_configuration_identity=Some((cx.operation(),cx.generation()));
        self.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
        let authority=self.pending_configuration_permit.take().unwrap();
        let permit=match self.run_framework_reserved_job("configuration-binary",&[],1,job,meta,Some(admission),authority,cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending_configuration_permit=Some(permit);*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}};
        self.pending_configuration_permit=Some(permit);
        let command=self.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
        match command{Ok(command)=>*self.pending_configuration_command=Some(command),Err(source)=>{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}}
        self.finish_configuration_command(meta,identity,cx).await
    }

    async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration commit requires its unchanged original caller").into())}
        if let Some(owner)=self.pending_configuration_completion.as_mut(){
            let demand=if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.demand(cx.retained_grant().maximum_copy_bytes)?};
            let terminal=owner.terminal_is_empty();let mut removed=false;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{if terminal{removed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{owner.step(grant)}})?;
            if removed{self.pending_configuration_completion.take();}return Ok(None)
        }
        let permit=self.pending_configuration_permit.as_ref().expect("configuration commit retains its admitted permit");self.validate_framework_reserved_commit("configuration-binary",permit).await?;
        if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let command=self.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let dispatched=self.config_store.dispatch(command,identity).await;let permit=self.pending_configuration_permit.take().unwrap();self.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}self.pending_configuration_identity=None;self.cache=None;Ok(Some(Self::empty_result("configCommand",meta,Vec::new(),Vec::new(),UiDirtyScope::Full).await))
    }

    fn receive_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.pending_configuration_failure.is_none(){return Ok(false)}if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration failure requires its unchanged original caller").into())}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_configuration_failure,cx)?{self.pending_configuration_identity=None;return Err(failure)}Ok(true)}
    pub(crate) fn configuration_failure_terminal_is_empty(&self)->bool{self.pending_configuration_completion.is_none()&&self.pending_configuration_command.is_none()&&self.pending_configuration_command_close.is_none()&&self.pending_configuration_permit.is_none()&&self.pending_configuration_failure.is_none()&&self.pending_configuration_retirement.is_none()&&self.pending_configuration_identity.is_none()}
    pub(crate) fn configuration_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.pending_configuration_completion.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.demand(body)}}if let Some(owner)=self.pending_configuration_command_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}self.pending_configuration_retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(!self.configuration_failure_terminal_is_empty()),..Default::default()}),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)})}
    pub(crate) fn close_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{let demand=self.configuration_failure_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{if let Some(owner)=self.pending_configuration_completion.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_completion.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(owner)=self.pending_configuration_command_close.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_command_close.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(command)=self.pending_configuration_command.take(){match ControlledRetirement::new(command){Ok(owner)=>self.pending_configuration_command_close=Some(owner),Err((error,command))=>{*self.pending_configuration_command=Some(command);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(permit)=self.pending_configuration_permit.take(){self.finish_reserved_emit_producer(permit);return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(owner)=self.pending_configuration_retirement.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_retirement.take();return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(source)=self.pending_configuration_failure.take(){match ControlledRetirement::new(source){Ok(owner)=>self.pending_configuration_retirement=Some(owner),Err((error,source))=>{*self.pending_configuration_failure=Some(source);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}self.pending_configuration_identity=None;Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))})}

    /// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a
    /// command-log row for EVERY dispatch, view actions included (`ActionKind::View` is also the
    /// fallback for an unresolved verb), so the log generation advancing is not by itself evidence that
    /// anything a user would call history happened. A `View` verb paints, it does not command; an
    /// `Interaction` verb moves selection/hover, which `dispatch_interaction_action` has always
    /// declared as kept out of the panel. Handing either one a history patch made every background
    /// tick a history update — measured 2026-09-09 20:55 as 90 host `readHistory` round trips beside
    /// 252 `fillBuildTick`s in 35 s.
    /// 🚫️async: E1 pure classification consumed by sync-only `matches!` call sites — see R9
    fn verb_dirties_history_panel(kind: Option<ActionKind>) -> bool {
        !matches!(kind, Some(ActionKind::View | ActionKind::Interaction))
    }

    /// 🧾️ Attaches the command-log delta directly to the accepted invocation response. History is
    /// no longer a plugin-rendered panel body, so this deliberately leaves `ui_scope` untouched.
    /// `verb` is the DECLARED action/command id — every caller must pass the real one, never a generic
    /// placeholder, or the kind lookup below silently resolves to nothing and the skip is unreachable.
    async fn finish_recorded(&mut self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
        if self.log_generation == log_generation_before {
            return result;
        }
        let kind = self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind));
        if !Self::verb_dirties_history_panel(kind) {
            return result;
        }
        result.history_patch = self.history_patch(false).await.ok();
        result
    }

    fn retained_fields_terminal_is_empty(&self) -> bool {
        self.retained_fields_own_terminal_is_empty() && self.children.is_empty() && self.local_interaction_query.is_none() && self.snapshot_read_returns_terminal_is_empty()
    }

    /// 🪪️ The immutable identity this instance's live document revision is observed under — the ONE
    /// derivation every entry point that hands a plugin an [`ArtifactView`] stamps into it
    /// (`render`, `pending_effects`, `window_engagements`, `window_measures`, `tool_measures`,
    /// `context_menu`). Chrome that reaches for its own per-instance session (puzzle 3d's
    /// `puzzle3d_view_session_key`) read `None` on the four view-only lanes and ran cold against a
    /// session-less app; deriving the identity here — not per call site — makes that key total.
    /// 🚫️async: E1 pure identity projection over already-owned store fields — see R9.
    fn live_render_operation(&self) -> AppRenderOperationContext {
        let canonical_base_revision = self.store.content_revision();
        let [lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7, ..] = canonical_base_revision;
        AppRenderOperationContext {
            app_instance_id: self.live_runtime_instance_id.unwrap_or_default(),
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes([lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7])),
            generation: semio_framework_job::Generation(self.store.generation()),
            canonical_base_revision,
            mounted_policy: self.mounted_policy,
        }
    }

    #[cfg(test)]
    pub(crate) fn typed_operation_result_state_for_test(&self, operation_id: u64) -> Option<(TypedOperationResultToken, TypedOperationResultLane, bool, bool)> {
        let operation = self.tool_operations.get(operation_id)?;
        let page = operation.result_page.as_ref()?;
        Some((page.token, page.lane, operation.result_page_presented, operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck))
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory> Drop for VcsArtifactApp<A, M> {
    fn drop(&mut self) {
        self.tool_cancellations.cancel_scope_generation();
        while let Some((_, id)) = self.pending_reserved.next_id_from(0) {
            if let Some(pending) = self.pending_reserved.remove(id) {
                pending.permit.finish();
            } else {
                break;
            }
        }
        while let Some(commit) = self.reserved_commits.pop_front() {
            commit.permit.finish();
        }
    }
}

/// 📣️ Signals the shell that the document's checkpoint/alternative history changed (after an
/// undo/redo/checkpoint/alternative command) so it can re-render history-dependent surfaces.
async fn history_changed_event() -> AppEvent {
    AppEvent { kind: "history-changed".into(), payload: DslValue::Null }
}

/// 🧪 Guest laws in dependent crates (puzzle 3d) settle Isolated reserved admits the host would.
pub async fn settle_framework_reserved_admission<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<InvocationResult,semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok(admitted);
    };
    let output=drive_framework_reserved_registry_job(job,input,app.mounted_policy.preparation)?;
    settle_framework_reserved_commit(app,job,Ok(output)).await.map_err(Into::into)
}

/// 🪜️ Most units one reserved commit may take in a settle law — one per composed child or undone
/// edit, well above anything a fixture composes.
pub const FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS: usize = 4_096;

/// 🪜️ Hands a finished reserved spawn-job to `app`'s commit queue and drives the commit one unit
/// per call — the same units the reactor's typed-operation continuation runs across turns — to its
/// outcome.
pub async fn settle_framework_reserved_commit<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, job: u64, output: Result<Vec<u8>, Fault>) -> Result<InvocationResult, Fault> {
    if !app.admit_framework_reserved_commit(job, output)? {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"));
    }
    for _ in 0..FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS {
        app.step_framework_reserved_commit().await?;
        if let Some(outcome) = app.take_framework_reserved_commit_outcome() {
            return outcome;
        }
    }
    Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stall"), format!("framework reserved commit did not finish within {FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS} units")))
}

/// 🧪 Browser `driveSpawnedJob` contract: the isolated reserved dummy job is admitted from its original request and driven
/// by funded `step_job` turns to its terminal output, which is then settled through the reserved commit; the turn count is returned.
#[cfg(test)]
pub async fn drive_framework_reserved_spawn_like_host<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<(InvocationResult,u32),semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok((admitted, 0));
    };
    let (output, steps) = drive_framework_reserved_registry_job_counted(job, input, app.mounted_policy.preparation)?;
    let settled = settle_framework_reserved_commit(app, job, Ok(output)).await?;
    Ok((settled, steps))
}

/// 🕰️ Length of [`PluginApp::maintenance_step`]'s fixed cooperative round robin. Every stage
/// runs at most one bounded unit per call, so a step-budget law needs exactly this many calls
/// to observe each stage once.
pub const MAINTENANCE_STAGES: u8 = 27;

/// 🧹️ The rotation stage that drains the document store's displaced-owner queue.
pub const MAINTENANCE_DOCUMENT_DISPLACED_STAGE: u8 = 9;

/// 🧹️ The rotation stage that drains the config-lane stores' displaced-owner queues.
pub const MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE: u8 = 25;

/// 🍂️ The rotation stage that disposes retired child-content roots, also run out of turn under child pressure.
const MAINTENANCE_CHILD_ROOT_STAGE: u8 = 4;

/// 🥀️ The rotation stage that closes retired child members, also run out of turn under child pressure.
const MAINTENANCE_CHILD_MEMBER_STAGE: u8 = 20;

/// 🌡️ Occupancy of the child-root or child-member retirement registry at which maintenance spends out-of-turn steps on
/// it — a quarter of the fixed [`ARTIFACT_LIVE_OUTPUT_SLOTS`], as the stores' displaced-owner pressure is of theirs.
pub(crate) const MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY: usize = ARTIFACT_LIVE_OUTPUT_SLOTS / 4;

/// ⏳️ Rounds one child publication spends reclaiming the retirements ahead of it before it waits instead.

/// ♻️ Returned document roots a publication leaves to the maintenance rotation: only beyond this many does a publication
/// unit drain them on its critical path. A one-item publication returns exactly one pre-edit root, so it never walks the
/// document; a multi-item batch drains before its second item reads a base, which keeps it bounded to the staged root,
/// the base it reads and this many returned roots (the 816-op fill OOM), never to one root per folded item.
const PUBLICATION_RETURNED_ROOT_ALLOWANCE: usize = 1;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🌡️ True while the child-root or child-member retirement registry holds at least
    /// [`MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`] retirements. An app that re-points a slot at a new
    /// content-addressed child per edit (writer mints one per keystroke) admits one of each per commit, which the single
    /// fair step per turn drains once per rotation: a typing run saturated the fixed slots with
    /// `interactive-job.child-root-retirement-saturated`.
    fn child_retirements_under_pressure(&self) -> bool {
        self.child_content_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY || self.child_member_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY
    }

}

/// 🪪️ A document loaded from its text or pack belongs to the app that loads it: a pack that names no dialect is stamped
/// with the app's own, exactly as every admission path (`try_adopt_completed`, the constructor) stamps it — without it the
/// first child restore faulted "child restore requires the parent's exact declared dialect".
fn loaded_with_app_dialect<A: ArtifactApp>(mut envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>) -> ArtifactEnvelope<A::Snapshot, A::Mutation> {
    if envelope.dialect.is_none() {
        envelope.dialect = Some(A::DIALECT.into());
    }
    envelope
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {
    fn admit_cancellation_issuer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let parent=cx.original_source_parent()?;self.tool_cancellations.admit_step(parent,cx)}
    fn mounted_owner_maintenance_phase(&self) -> Result<MountedOwnerPhaseV1, Fault> {
        let instance_id = self.live_runtime_instance_id.ok_or_else(|| plugin_sdk_fault("mounted phase requires the live instance"))?;
        if A::mounted_jobs_terminal_is_empty(instance_id) { return Ok(MountedOwnerPhaseV1::Maintenance); }
        A::mounted_job_maintenance_phase(instance_id).ok_or_else(|| plugin_sdk_fault("retained mounted owner did not declare its preparation or maintenance phase"))
    }
    fn mounted_owner_maintenance_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let identity=turn.identity();if self.live_runtime_instance_id!=Some(identity.instance_id)||!matches!(turn.phase(),MountedOwnerPhaseV1::Preparation|MountedOwnerPhaseV1::Maintenance){return Err(plugin_sdk_fault("mounted maintenance does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_maintenance_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_maintenance_step(identity.instance_id,same,actual)?,true)))}else{self.maintenance_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.maintenance_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn mounted_owner_close_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let identity=turn.identity();if self.live_runtime_instance_id!=Some(identity.instance_id)||turn.phase()!=MountedOwnerPhaseV1::Close{return Err(plugin_sdk_fault("mounted close does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_close_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_close_step(identity.instance_id,same,actual)?,A::mounted_jobs_terminal_is_empty(identity.instance_id))))}else{self.close_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.close_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.has_constructor_receipt() { return Ok(Default::default()); }
        if !self.constructor_terminal_is_empty() { return self.constructor_close_demands(body); }
        self.close_ladder_demand(body)
    }

    fn maintenance_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.maintenance_ladder_demands(body)
    }

    async fn bind_instance_id(&mut self, instance_id: u32) {
        self.live_runtime_instance_id = Some(instance_id);
    }

    async fn bind_actor(&mut self, actor: &str) {
        assert_eq!(self.store.local_actor_id().0.as_str(), actor, "opening actor must own the document store from construction");
        assert_eq!(self.config_store.local_actor_id().0.as_str(), actor, "opening actor must own the config store from construction");
        assert_eq!(self.draft_store.local_actor_id().0.as_str(), actor, "opening actor must own the draft store from construction");
        assert_eq!(self.interaction_store.local_actor_id().0.as_str(), actor, "opening actor must own the interaction store from construction");
        assert_eq!(self.window_config_store.local_actor_id().0.as_str(), actor, "opening actor must own the window config registry from construction");
    }

    /// 🎟️ Advances one genuine original owner under the complete caller grant.
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        if self.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"original constructor receipt awaits its phase recipient"})}
        let demand=self.close_retirement_demands(loan.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{
            if !self.constructor_terminal_is_empty(){self.close_store_constructor_unit(grant)?;let progress=self.constructor_receipt().map_or(Default::default(),|(_,_,_,progress)|*progress);self.take_constructor_receipt();return Ok((PluginLifecycleStep::Progress(progress),false))}
            let step=self.close_ladder_step(same,grant)?;let terminal=self.close_terminal_is_empty();Ok((PluginLifecycleStep::retained(step,terminal),terminal))
        })
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.constructor_terminal_is_empty()
            && self.close_started
            && self.pending_command_prune.is_none()
            && !self.command_prune_requested()
            && self.command_log.pruned_terminal_is_empty()
            && self.tool_runs.terminal_is_empty()
            && self.time_travel.terminal_is_empty()
            && self.store.close_owned_store_terminal_is_empty()
            && self.config_store.close_owned_store_terminal_is_empty()
            && self.draft_store.close_owned_store_terminal_is_empty()
            && self.interaction_store.close_owned_store_terminal_is_empty()
            && self.close_presence_disposer.is_none()
            && self.close_transient_disposer.is_none()
            && self.window_config_store.terminal_is_empty()
            && self.window_transient_store.terminal_is_empty()
            && self.retired_window_transient_stores.is_empty()
            && self.retired_window_transient_stores.empty_backing_byte_demand()==Some(0)
            && self.close_interaction_disposer.is_none()
            && self.tool_cancellations.terminal_is_empty()
            && self.pending_document_window_admission.is_none()
            && self.pending_tool_lease_admission.is_none()
            && self.pending_media_import_permit.is_none()
            && self.tool_operations.is_empty()
            && self.private_child_groups_terminal_is_empty()
            && self.latest_wins_commands.is_empty()
            && self.typed_operation_reservations.iter().all(Option::is_none)
            && self.latest_wins_order.len() == 0
            && self.latest_wins_keys.terminal_is_empty()
            && self.close_instance_operation_owner_drained
            && self.instance_operation_owner.close_terminal_is_empty().unwrap_or(false)
            && self.render_failure_terminal_is_empty()
            && self.agent_lane_preview.is_none()
            && self.media_submission_refusal_terminal_is_empty()
            && self.action_ingress_sources_closed()
            && self.media_exports.is_empty()
            && self.pending_reserved.is_empty()
            && self.reserved_commits.is_empty()
            && self.reserved_commit_outcome.is_none()
            && self.media_closures.is_empty()
            && self.snapshot_retirements.is_empty()
            && self.segmented_downloads.is_empty()
            && self.segmented_closures.is_empty()
            && self.child_content_retirements.is_empty()
            && self.child_content_retirements.empty_backing_byte_demand()==Some(0)
            && self.close_child_member.is_none()
            && self.child_member_retirements.is_empty()
            && self.child_member_retirements.empty_backing_byte_demand()==Some(0)
            && self.children.is_empty()
            && self.peer_roster_publications.is_empty()
            && self.peer_roster_outcomes.is_empty()
            && self.peer_presence_retirements.is_empty()
            && self.presence_peer_retirements.is_empty()
            && self.envelope_ingress.is_empty()
            && self.envelope_decode_jobs.is_empty()
            && self.store_replacement_jobs.is_empty()
            && self.document_archive_loads.is_empty()
            && self.document_archive_loads.empty_backing_byte_demand()==Some(0)
            && self.envelope_field_decoder_retirements.is_empty()
            && self.envelope_completed_record_retirements.is_empty()
            && self.retained_fields_terminal_is_empty()
    }

    /// 🎡️ Visits the live-maintenance units in fair order and forwards the unchanged grant to the first that releases something.
    fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::PluginLifecycleFailure> {
        self.maintenance_ladder_step(cx,loan)
    }

    fn maintenance_terminal_is_empty(&self) -> bool {
        self.maintenance_ladder_terminal_is_empty()
    }

    fn maintenance_under_pressure(&self) -> bool {
        self.maintenance_ladder_under_pressure()
    }

    async fn app_id(&self) -> &str {
        self.app.instance_id().await
    }

    async fn artifact_schema(&self) -> &str {
        A::DOCUMENT_SCHEMA
    }

    async fn tool_owner_witness(&self) -> ToolOwnerWitness {
        ToolOwnerWitness::of::<A>()
    }

    async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> {
        self.bounded_tool_contracts
            .iter()
            .cloned()
            .chain(self.framework_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .chain(self.app_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .collect()
    }

    async fn advance_typed_operation_publication(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        let retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if self.local_interaction_query_receipt.is_some(){return Ok(())}
        if self.has_constructor_receipt() { return Ok(()); }
        if self.store_constructor_is_pending() { self.drive_store_constructor_unit(crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,retained))?; return Ok(()); }
        if self.time_travel.has_command_receipt() || self.tool_runs.has_original_receipt() || (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index|self.tool_operations.entry(index).is_some_and(|(_,owner)|owner.publication_ownership_progress.is_some()||owner.original_retirement_receipt.is_some())){return Ok(());}
        if !self.reserved_commits.is_empty() && self.reserved_commit_outcome.is_none() {
            self.step_framework_reserved_commit().await?;
            return Ok(());
        }
        if self.tool_run_has_pending_work() {
            let tool_run_before=cx.retained_progress();
            self.drive_tool_run_turn(cx, identity).await?;
            if cx.retained_progress()!=tool_run_before{
                if !self.tool_runs.has_original_receipt(){
                    let progress=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(tool_run_before,cx.retained_progress())?;
                    self.tool_runs.retain_original_receipt(retained,progress)?;
                }
                return Ok(())
            }
            if self.tool_runs.has_original_receipt(){return Ok(());}
        }
        let retained=cx.retained_grant();
        if self.time_travel_has_pending_work() {
            self.drive_time_travel_turn(crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.maintenance,retained), identity, cx).await?;
            if self.time_travel.has_command_receipt(){return Ok(());}
        }
        if self.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work()) {
            self.local_interaction_query_turn = !self.local_interaction_query_turn;
            if self.local_interaction_query_turn {
                self.advance_local_interaction_query_one(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained),crate::local_interaction::query::LOCAL_INTERACTION_QUERY_PAGE_BYTES)?;
                return Ok(());
            }
        }
        self.advance_typed_operation_publication_one(cx).await.map_err(Into::into)
    }

    fn take_typed_operation_preparation_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.latest_wins_commands.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.receipt.take(){return Some((owner.operation.operation,grant,progress));}
        }

        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.tool_operations.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.publication_ownership_progress.take(){return Some((owner.operation.operation,grant,progress));}
        }
        None
    }

    fn take_typed_operation_retirement_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{let Some((_,owner))=self.tool_operations.entry_mut(index)else{continue;};if let Some((grant,progress))=owner.original_retirement_receipt.take(){return Some((owner.operation.operation,grant,progress));}}None
    }

    fn take_history_command_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.time_travel.take_command_receipt()
    }

    fn take_tool_run_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.tool_runs.take_original_receipt()
    }
    fn take_local_interaction_query_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.local_interaction_query_receipt.take()}

    fn store_constructor_is_pending(&self) -> bool { VcsArtifactApp::store_constructor_is_pending(self) }

    fn take_constructor_receipt(&mut self) -> Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)> { VcsArtifactApp::take_constructor_receipt(self) }

    fn tool_run_presence(&self) -> Option<protocol::PresenceToolRun> {
        self.tool_runs.presence()
    }

    fn tool_run_trace_delta(&self, cursor: Option<ToolRunTraceCursor>) -> Option<String> {
        self.tool_runs.trace_delta(cursor, tool_run::TOOL_RUN_TRACE_DELTA_BYTES)
    }

    fn typed_operation_refusal(&self) -> Option<String> {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| {
            let (_, operation) = self.tool_operations.entry(index)?;
            operation.retained_structural_refusal().map(|reason| format!("typed operation '{}' {reason}", operation.verb))
        })
    }

    fn close_diagnostic(&self) -> String {
        self.close_rung_name()
    }

    fn typed_operation_diagnostic(&self) -> String {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter_map(|index| {
            let (id, operation) = self.tool_operations.entry(index)?;
            Some(format!("operation {id} '{}' stage={:?} session={} terminal={} refusal={}", operation.verb, operation.stage, operation.session.is_some(), operation.terminal_seen, operation.retained_structural_refusal().unwrap_or_else(|| "none".into())))
        }).collect::<Vec<_>>().join("; ")
    }

    fn has_pending_typed_operations(&self) -> bool {
        self.store_constructor_is_pending() || self.operation_progress_retired
            || self.derivable_follow_awaits_retirement()
            || self.tool_run_has_pending_work()
            || self.time_travel_has_pending_work()
            || !self.reserved_commits.is_empty()
            || self.reserved_commit_outcome.is_some()
            || !self.tool_operations.is_empty()
            || !self.latest_wins_commands.is_empty()
            || self.typed_effect_outbox.len() != 0
            || self.typed_event_outbox.len() != 0
            || self.typed_ui_outbox.len() != 0
            || self.typed_completion_outbox.len() != 0
            || self.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
    }

    fn live_typed_operation_slots(&self) -> usize {
        VcsArtifactApp::<A, M>::live_typed_operation_slots(self)
    }

    fn has_runnable_typed_operations(&self) -> bool {
        self.store_constructor_is_pending() || self.operation_progress_retired
            || self.derivable_follow_awaits_retirement()
            || self.tool_run_has_pending_work()
            || self.time_travel_has_pending_work()
            || !self.reserved_commits.is_empty()
            || self.reserved_commit_outcome.is_some()
            || !self.tool_operations.is_empty()
            || self.has_runnable_artifact_envelope_decode_worker_step()
            || !self.latest_wins_commands.is_empty()
            || self.typed_effect_outbox.len() != 0
            || self.typed_event_outbox.len() != 0
            || self.typed_ui_outbox.len() != 0
            || self.typed_completion_outbox.len() != 0
            || self.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
    }

    fn begin_local_interaction_query(&mut self, request_id: u64, query_generation: u64) -> Option<protocol::LocalInteractionQueryReply> {
        if self.close_started || self.live_runtime_instance_id.is_none() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed });
        }
        if self.local_interaction_query.is_some() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Busy });
        }
        let document_revision = self.store.content_revision_now();
        let config_revision = self.config_store.content_revision_now();
        let topology_revision = match self.local_interaction_authority.revision(document_revision, config_revision) {
            Ok(revision) => revision,
            Err(_) => return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed }),
        };
        let identity = protocol::LocalInteractionIdentity {
            app_instance_id: self.live_runtime_instance_id.expect("live instance checked"),
            generation: self.interaction_store.generation_now(),
            revision: self.interaction_store.content_revision_now(),
            document_revision,
            topology_revision,
        };
        self.local_interaction_query =
            Some(crate::local_interaction::live::LocalInteractionLiveQuery::new(request_id, query_generation, identity, self.store.snapshot_read().ok(), self.config_store.snapshot_read().ok(), self.interaction_store.snapshot_read().ok()));
        self.local_interaction_emitted_bytes = 0;
        self.local_interaction_retired_bytes = 0;
        None
    }

    fn acknowledge_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.local_interaction_query.as_mut().is_some_and(|query| query.acknowledge(token))
    }

    fn cancel_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.local_interaction_query.as_mut().is_some_and(|query| query.cancel_authorized(token))
    }

    fn take_local_interaction_query_reply(&mut self) -> Option<protocol::LocalInteractionQueryReply> {
        let query = self.local_interaction_query.as_mut()?;
        let reply = query.take_reply();
        if query.terminal_is_empty() {
            self.local_interaction_query = None;
        }
        reply
    }

    fn publish_local_interaction_query_reply(&mut self, frames: &mut Vec<Vec<u8>>, maximum_frames: usize) -> bool {
        let Some(query) = self.local_interaction_query.as_mut() else {
            return false;
        };
        if !query.reply_ready() || frames.len() >= maximum_frames {
            return false;
        }
        let mut encoded = Vec::new();
        if frames.try_reserve_exact(1).is_err() || encoded.try_reserve_exact(512).is_err() {
            return false;
        }
        let mut encoded = Some(encoded);
        let delivered = query
            .take_reply_admitted(|reply| {
                if protocol::encode_local_interaction_query_frame_into(reply, encoded.as_mut().expect("one admitted reply buffer")).is_err() {
                    return false;
                }
                frames.push(encoded.take().expect("one admitted reply buffer"));
                true
            })
            .is_some();
        if query.terminal_is_empty() {
            self.local_interaction_query = None;
        }
        delivered
    }

    fn take_typed_operation_result_page(&mut self, receiver: u32) -> Option<TypedOperationResultPage> {
        let (index, operation_id) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (self.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.tool_operations.entry(index).filter(|(_, operation)| operation.meta.instance_id == receiver && !operation.result_page_presented && operation.result_page.is_some()).map(|(id, _)| (index, *id))
        })?;
        self.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        self.tool_operations.get_mut(operation_id)?.take_result_page()
    }

    fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> {
        if token.receiver != self.live_runtime_instance_id.unwrap_or(u32::MAX) {
            return Ok(false);
        }
        let download_ack = self.tool_operations.get(token.operation).and_then(|operation| operation.result_page.as_ref().map(|page| (operation, page))).is_some_and(|(operation, page)| {
            page.token == token
                && page.lane == TypedOperationResultLane::Download
                && operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck
                && matches!(operation.publication.as_ref(), Some(ArtifactToolCompletionValue::Download(Ok(_), _)))
        });
        if download_ack && (!self.segmented_downloads.can_insert(token.operation) || !self.segmented_closures.can_insert(token.operation)) {
            return Ok(false);
        }
        let Some(operation) = self.tool_operations.get_mut(token.operation) else { return Ok(false) };
        let accepted = operation.acknowledge_result_page(token)?;
        if !accepted || !download_ack {
            return Ok(accepted);
        }
        let publication = operation.publication.take().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK lost its exact retained publication owner"))?;
        let ArtifactToolCompletionValue::Download(Ok(download), ephemeral) = publication else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK resolved a non-download retained publication owner"));
        };
        if !ephemeral.presence.is_empty() || !ephemeral.transient.is_empty() {
            operation.publication = Some(ArtifactToolCompletionValue::Download(Ok(download), ephemeral));
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-ephemeral"), "download ACK preceded retained ephemeral publication"));
        }
        self.segmented_downloads.insert_admitted(token.operation, download);
        Ok(true)
    }

    fn take_typed_operation_effect(&mut self) -> Option<Effect> {
        self.typed_effect_outbox.pop()
    }

    fn take_typed_operation_event(&mut self) -> Option<AppEvent> {
        self.typed_event_outbox.pop()
    }

    fn take_typed_operation_ui_scope(&mut self) -> Option<UiDirtyScope> {
        self.typed_ui_outbox.pop().or_else(|| self.take_operation_progress_scope())
    }
    fn take_typed_operation_ui_progress(&mut self) -> Option<TypedOperationUiProgress> {
        let ui_scope = match self.typed_ui_outbox.pop().or_else(|| self.take_operation_progress_scope()) {
            Some(scope) => scope,
            None if self.time_travel.has_patch() => UiDirtyScope::None,
            None => return None,
        };
        Some(TypedOperationUiProgress { ui_scope, leftover: self.typed_inline_interaction_leftover.take(), history_patch: self.time_travel.take_patch() })
    }

    /// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: the dirty set is only authoritative AFTER the
    /// backfill. A typed operation publishes its edit through
    /// [`Self::publish_mounted_typed_operation_unit`], which stages a batched `Edit` and never calls
    /// `record_command` — that edit reaches the command log exclusively through
    /// [`Self::backfill_command_log`], which runs inside [`Self::refresh_cache`], which
    /// [`Self::history_patch`] runs FIRST. Reading `history_dirty_sequences` before that backfill
    /// therefore answered "nothing dirty" on the very call whose publication had just landed, so the
    /// terminal completion carried `history_patch: None` and the row surfaced on whatever later call
    /// happened to refresh the cache — the one-command lag measured in the browser on wasm #47
    /// (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
    ///
    /// 🧾️ Lane-less rows (a View, a Mutation that emitted nothing) join this patch. Retirement
    /// still records them, but only after this function has already read `history_dirty_sequences`.
    async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> {
        let Some(witness) = self.typed_completion_outbox.pop() else { return Ok(None) };
        self.record_settled_typed_operation_command(witness.operation);
        self.refresh_cache().await?;
        let history_patch = if self.history_dirty_sequences.is_empty() { None } else { Some(self.history_patch(false).await?) };
        let revision = self.store.content_revision_now();
        Ok(Some(TypedOperationCompletion { operation: witness.operation, revision: u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")), ui_scope: witness.ui_scope, history_patch }))
    }

    fn take_typed_operation_composed_result(&mut self) -> Option<ComposedGestureResult> {
        self.typed_composed_outbox.pop()
    }

    async fn begin_dispatch_report(&mut self) {
        self.reset_dispatch_report().await;
    }

    async fn dispatch_report(&self) -> &protocol::DispatchReport {
        &self.dispatch_report
    }

    async fn set_merge_policy(&mut self, policy: protocol::MergePolicy) {
        self.apply_merge_policy(policy).await;
    }

    async fn resolve_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        self.resolve_store_conflict(conflict_id, resolution).await
    }

    async fn open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.current_open_conflicts().await
    }

    async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.log_generation;
        let result = self.dispatch_action(action, args, &mut None, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        self.follow_derivable_children(identity).await?;
        Ok(Some(self.finish_recorded(log_generation_before, action, result).await))
    }

    fn admit_reserved_spawned_job(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        self.admit_framework_reserved_commit(job, output)
    }

    fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.take_framework_reserved_commit_outcome()
    }

    fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.framework_reserved_commit_progress()
    }

    async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {}", address.app_id, owner_app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() {
            if !self.registry.has_mode(&address.mode_id).await {
                return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
            }
            if active_mode_id != Some(address.mode_id.as_str()) {
                return Err((plugin_sdk_fault(format!("action {} belongs to inactive mode {}", address.action_id, address.mode_id))).into());
            }
        }
        let kind = match self.registry.window_action(&address.window_kind_id, &address.action_id).await {
            Some(definition) => definition.kind,
            None => reserved_kind.ok_or_else(|| plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id)))?,
        };
        if !self.action_source_capture_ready(&address.action_id,cx){return Ok(None)}
        let log_generation_before = self.log_generation;
        let previous_kind = self.invocation_kind.replace(kind);
        let result = self.dispatch_action(&address.action_id, source_args, original_fault, meta, identity, cx).await;
        self.invocation_kind = previous_kind;
        let Some(result) = result? else{return Ok(None)};
        Ok(Some(self.finish_recorded(log_generation_before, &address.action_id, result).await))
    }

    async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if !self.action_source_capture_ready(action,cx){return Ok(None)}
        let definition = self.registry.app_command(action).await.ok_or_else(|| plugin_sdk_fault(format!("continuation command '{action}' is not owned by the receiving app")))?;
        validate_ui_dispatch_classification("receiving app", action, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(command) = self.receive_action_command(action, args, original_fault, cx).await? else { return Ok(None) };
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(action, &wire, 1).await?;
        self.require_complete_tool_operation_pipeline(&admission)?;
        let log_generation_before = self.log_generation;
        let previous_kind = self.invocation_kind.replace(kind);
        let result = self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.invocation_kind = previous_kind;
        let result = result?;
        Ok(Some(self.finish_recorded(log_generation_before, action, result).await))
    }

    async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.log_generation;
        let result = self.dispatch_command(address, source_args, original_fault, active_mode_id, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        Ok(Some(self.finish_recorded(log_generation_before, &address.command_id, result).await))
    }

    fn original_command_frame_pending(&self)->bool{self.agent_lane_preview.is_some()}
    async fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.log_generation;
        let result = self.dispatch_command_frame(command_bytes, meta, identity, cx).await?;
        match result{Some(result)=>Ok(Some(self.finish_recorded(log_generation_before,"typed-command",result).await)),None=>Ok(None)}
    }

    /// 🎯️ M1: resolves `A::command_from_intent(intent)` then dispatches through the SAME
    /// `dispatch_typed_command_inner` `dispatch_typed_command`/`dispatch_typed` use — kind
    /// discipline, the command log, undo grouping and `AsyncTask` spawning apply unchanged.
    async fn handle_intent_frame(&mut self, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let verb = intent.action.name.as_str();
        if intent.action.version != 1 {
            return Err((Fault::new(FaultOrigin::App, FaultCode::new("app.intent.version-mismatch"), format!("intent action '{verb}' targets version {} but the intent bridge only resolves version-1 actions", intent.action.version))).into());
        }
        let _ = self.qualified_tool_proof(verb)?;
        if !self.action_source_capture_ready(verb,cx){return Ok(None)}
        let mut merged = if self.action_source_pending(){None}else{merge_ui_values(intent.args.as_ref(), intent.input.as_ref()).await?};
        let Some(command)=self.receive_action_command(verb,&mut merged,&mut None,cx).await?else{return Ok(None)};
        self.dispatch_typed(command, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes the task resolution's own
    /// `OpBinary`-encoded `A::Command` and re-enters `dispatch_typed` with it — see the trait
    /// method's own doc for why this is the applying lane and not `handle_command_frame`.
    async fn resume_task_command(&mut self, command_bytes: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let command = <A::Command as ::protocol::OpBinary>::decode_op(&command_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-command"), format!("an AsyncTask resolved TaskResolution::Command with bytes that are not this app's typed Command: {error}")))?;
        self.dispatch_typed(command, meta, identity, cx).await
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes each lane's wire ops back into this
    /// app's concrete `A::Mutation`/`A::ConfigMutation`/`A::DraftMutation` via `OpBinary::
    /// decode_op` (the SAME decode idiom `dispatch_typed_command` uses for `A::Command`) and
    /// dispatches them directly — see the trait method's own doc for why this skips `handle`.
    async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        async fn decode_lane<T: ::protocol::OpBinary>(bytes: &[u8]) -> Result<Vec<T>, Fault> {
            let op_bytes_list = protocol::decode_ops_vec(bytes).map_err(|error| error.into_fault())?;
            let mut decoded = Vec::with_capacity(op_bytes_list.len());
            for op_bytes in op_bytes_list.iter() {
                decoded.push(<T as ::protocol::OpBinary>::decode_op(op_bytes).map_err(|error| error.into_fault())?);
            }
            Ok(decoded)
        }
        let log_generation_before = self.log_generation;
        let artifact_mutations = decode_lane::<A::Mutation>(&artifact_ops).await?;
        let config_mutations = decode_lane::<A::ConfigMutation>(&config_ops).await?;
        let draft_mutations = decode_lane::<A::DraftMutation>(&draft_ops).await?;
        let result = self.dispatch_emit("task-resume-emit", Emit { artifact_mutations, config_mutations, draft_mutations, ..Default::default() }, meta, identity, cx).await?;
        Ok(self.finish_recorded(log_generation_before, "task-resume-emit", result).await)
    }

    async fn take_last_emit_wire(&mut self) -> Option<EmitWire> {
        self.last_emit_wire.take()
    }

    async fn take_pending_presence(&mut self) -> Vec<PresenceUpdate> {
        std::mem::take(&mut self.pending_presence)
    }

    async fn take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft> {
        self.pending_transaction_proposal.take()
    }

    /// 🔀️ Decodes EITHER wire form (contract §2/§5.3): pre-planned (`prepared_ops`
    /// non-empty) carries its own `origin`; owner-mutation (`prepared_ops` empty)
    /// decodes the single `payload` op and has no origin on the wire — see the 🚧️ note below.
    ///
    /// 🚧️ The owner-mutation form carries no `origin` on the wire (contract §2/§5.3)
    /// — this member is by construction a foreign target of someone else's
    /// transaction, so the best available provenance is `Transaction` with an
    /// initiator identity the wire does not convey. Flagged in `📓️w1-b-report.md`
    /// as a contract gap for the coordinator/host lanes to close (e.g. by always
    /// preferring the pre-planned form, which does carry `origin`).
    async fn transaction_prepare(&mut self, txn_id: &str, _mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome {
        if self.time_travel.freezes_local_emits() {
            return TransactionPrepareOutcome {
                foreign: Vec::new(),
                rejection: Some(Self::transaction_fault(FaultOrigin::Framework, semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE, format!("transaction {txn_id:?} would change the document while a history edit is open on this instance"))),
            };
        }
        if self.pending_transaction.is_some() {
            return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", "a transaction is already pending on this instance")) };
        }
        let children = match ChildEmit::decode_groups(prepared_child_ops) {
            Ok(children) => children,
            Err(fault) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(fault) },
        };
        if let Some(missing) = children.iter().find(|child| !child.owner.is_empty() || self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {
            return TransactionPrepareOutcome {
                foreign: Vec::new(),
                rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("owned child {:?} in slot {:?} is not held by this instance", missing.child_id, missing.slot))),
            };
        }
        let (ops, resolved_origin): (Vec<A::Mutation>, protocol::MutationOrigin) = if !prepared_ops.is_empty() || !children.is_empty() {
            let mut decoded = Vec::with_capacity(prepared_ops.len());
            for op_bytes in prepared_ops {
                match <A::Mutation as ::protocol::OpBinary>::decode_op(op_bytes) {
                    Ok(op) => decoded.push(op),
                    Err(error) => {
                        return TransactionPrepareOutcome {
                            foreign: Vec::new(),
                            rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("prepared op did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                        };
                    }
                }
            }
            (decoded, origin.unwrap_or_default())
        } else {
            match <A::Mutation as ::protocol::OpBinary>::decode_op(payload) {
                Ok(op) => (vec![op], protocol::MutationOrigin::Transaction { initiator: protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None } }),
                Err(error) => {
                    return TransactionPrepareOutcome {
                        foreign: Vec::new(),
                        rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("owner-mutation payload did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                    };
                }
            }
        };
        let mut running = match self.store.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("{error:?}"))) },
        };
        let foreign = Vec::new();
        for op in &ops {
            match op.foreign_step_source(&running, 0) { Ok(None) => {}, Ok(Some(_)) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")) }, Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(error.into_fault()) } }
            let outcome = op.diff(&running);
            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages()))) };
            }
            running = match protocol::apply_diff(outcome.diff(), &running) {
                Ok(next) => next,
                Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string())) },
            };
        }
        let base_generation = self.store.generation();
        self.pending_transaction = Some(PendingTransaction { txn_id: txn_id.to_string(), ops, children, origin: resolved_origin, base_generation });
        TransactionPrepareOutcome { foreign, rejection: None }
    }

    /// 🔀️ Contract §5.8: reject commit when the base generation observed at prepare time no
    /// longer matches the current one — the pending state is restored (not discarded) so an
    /// explicit `TransactionRollback`/retry can still act on it.
    ///
    /// 🔀️ Contract §5.6: this member's prepared ops land as exactly ONE `Edit` — the
    /// one-edit-per-member invariant `CompositionCoordinator::undo_group`'s tail-based group
    /// undo depends on — stamped with `group_id = txn_id` and every `MutationMeta.origin`.
    async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let pending = match self.pending_transaction.take() {
            Some(pending) if pending.txn_id == txn_id => pending,
            Some(other) => {
                let message = format!("transaction_commit: txn_id {txn_id:?} does not match the pending transaction {:?}", other.txn_id);
                self.pending_transaction = Some(other);
                return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", message));
            }
            None => return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("transaction_commit: no transaction is pending (requested txn_id {txn_id:?})"))),
        };
        if pending.base_generation != self.store.generation() {
            let message = format!("transaction {txn_id:?}'s base generation {} no longer matches the current generation {}", pending.base_generation, self.store.generation());
            self.pending_transaction = Some(pending);
            return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.generation-mismatch", message));
        }
        let PendingTransaction { txn_id, ops, children, origin, .. } = pending;
        if !children.is_empty() {
            return self.commit_transaction_group(&txn_id, ops, children, origin, meta, identity).await;
        }
        let label = ops.first().map(|op| protocol::SemanticMutation::<A::Snapshot>::label(op));
        self.store.dispatch(ArtifactCommand::Apply { mutations: ops, transaction: None }, identity).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.cache = None;
        self.store.stamp_tail_group_id(&txn_id).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.store.stamp_tail_origin(origin).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        let edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default();
        self.record_command(&format!("transaction:{txn_id}"), ActionKind::Mutation, label, Some(edit_id.clone()), Vec::new(), None);
        Ok(edit_id)
    }

    async fn transaction_rollback(&mut self, txn_id: &str) -> Result<(), Fault> {
        match &self.pending_transaction {
            Some(pending) if pending.txn_id == txn_id => {
                self.pending_transaction = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.rollback-unknown"), format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}"))),
        }
    }

    async fn transaction_undo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.children.is_empty() {
            return self.transaction_group_history("undo", group_id, identity).await;
        }
        if self.store.tail_group_id().await.as_deref() != Some(group_id) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-foreign-tail"), format!("transaction_undo: this instance's tail edit does not belong to group {group_id:?}")));
        }
        self.store.undo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-failed"), format!("{error:?}")))?;
        self.cache = None;
        Ok(())
    }

    async fn transaction_redo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.children.is_empty() {
            return self.transaction_group_history("redo", group_id, identity).await;
        }
        match self.store.redo_tail().await {
            Some((_, Some(tail_group))) if tail_group == group_id => {
                self.store.redo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-failed"), format!("{error:?}")))?;
                self.cache = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-foreign-tail"), format!("transaction_redo: this instance's redo-tail edit does not belong to group {group_id:?}"))),
        }
    }

    async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
        if self.time_travel.history_unavailable() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), "this head-only pure evaluation holds no history; use a live instance"));
        }
        self.history_patch(true).await
    }

    /// 🫥️ A pure command's head is the HEAD snapshot without history (design §20.8, `📓️api-stepped-document-load.md` §4):
    /// it hydrates in O(snapshot) — no history fold — and marks the document head-only, so history verbs and queries answer
    /// `pure.history-unavailable`. An empty head keeps the live document.
    async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if head.is_empty() {
            return Ok(true);
        }
        if self.pending_document_window_admission.as_ref().is_some_and(|admission|admission.committed){return self.close_document_window_admission(cx,false).map(|step|matches!(step,PluginLifecycleStep::Complete(_))).map_err(Into::into)}
        if !self.admit_document_window_reset(cx)?{return Ok(false)}
        let head = <A::Snapshot as ArtifactPack>::decode_pack(head).map_err(|error| plugin_sdk_fault(format!("the pure head snapshot does not decode: {error:?}")))?;
        let id = self.store.envelope().id.clone();
        let envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &id, head, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut window_reset=Some(self.prepare_document_window_reset()?);
        let actor = self.store.local_actor_id().clone();
        self.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
        self.commit_document_window_reset(&mut window_reset,cx)?;
        assert_eq!(self.store.local_actor_id(), &actor, "pure head retains its opened actor");
        self.cache = None;
        self.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(true);
        Ok(false)
    }

    /// 🎯️ Empty domains ⇒ empty bytes (contract-freeze §C7.6) — `encode_presence_interaction`
    /// itself always writes at least `app_id` + a domain count, so an explicit short-circuit
    /// is required rather than trusting the codec's own "nothing to say" shape.
    ///
    /// 👤️ Generation 0 ⇒ empty bytes, the presence twin of the interaction short-circuit above: a
    /// local presence root that was never published has nothing to say, and `encode_pack` always
    /// writes a record header, so an untouched lane would otherwise ship a pack on every frame.
    async fn ephemeral_snapshot(&self) -> EphemeralSnapshot {
        let hover_specs: BTreeMap<String, protocol::HoverSpec> = self.registry.interactions().await.map(|def| (def.id.clone(), def.hover.clone())).collect();
        let selection_specs: BTreeMap<String, protocol::SelectionSpec> = self.registry.interactions().await.map(|def| (def.id.clone(), def.selection.clone())).collect();
        let interaction_state = self.interaction_state().await;
        let interaction = protocol::assemble_presence_interaction(self.app_id().await, &interaction_state, &hover_specs, &selection_specs).await;
        let interaction_bytes = if interaction.domains.is_empty() {
            Vec::new()
        } else {
            let mut bytes = Vec::new();
            protocol::encode_presence_interaction(&interaction, &mut bytes).await;
            bytes
        };
        let presence_generation = self.presence_store.generation().await;
        EphemeralSnapshot {
            presence: if presence_generation == 0 { Vec::new() } else { self.presence_store.local().encode_pack() },
            presence_generation,
            transient_generation: self.transient_store.generation().await,
            interaction: interaction_bytes,
            tool_run: self.tool_runs.presence(),
            history_edit: self.time_travel.presence(),
        }
    }

    fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
        let generation = self.peer_roster_admission_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster admission generation exhausted"))?;
        let slot = Self::peer_roster_slot(generation);
        if self.peer_roster_reservations[slot].is_some()
            || !self.peer_roster_publications.can_insert(generation)
            || !self.peer_roster_outcomes.can_insert(generation)
            || !self.peer_presence_retirements.can_insert(generation)
            || !self.presence_peer_retirements.can_insert(generation)
            || !self.presence_store.peer_retirement_available()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-saturated"), "peer roster ingress, outcome, or retirement authority is saturated before payload decode"));
        }
        self.peer_roster_reservations[slot] = Some((generation, seq));
        self.peer_roster_admission_generation = generation;
        Ok(PresenceRosterAdmission { seq, generation, cancel: self.peer_roster_scope.child_now() })
    }

    fn admit_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64) {
        let slot = Self::peer_roster_slot(admission.generation);
        assert_eq!(command.seq(), admission.seq, "Presence cursor sequence is bound by its unforgeable reservation");
        assert_eq!(self.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "Presence reservation remains live under the app's exclusive owner");
        assert!(self.peer_roster_publications.can_insert(admission.generation), "Presence publication slot was reserved before cursor construction");
        let generation = admission.generation;
        self.peer_roster_publications.insert_admitted(generation, PeerRosterPublication::<A>::new(admission, now_ms, command));
    }

    fn push_presence_ingress(&mut self, generation: u64, page_index: u32, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)> {
        let Some(publication) = self.peer_roster_publications.get_mut(generation) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-owner"), "Presence page targets no retained publication owner"), page));
        };
        publication.push_page(page_index, page)
    }

    fn reject_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: FixedCommandPage, fault: Fault) {
        let slot = Self::peer_roster_slot(admission.generation);
        assert_eq!(self.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "malformed Presence owner retains its exact reservation");
        assert!(self.peer_roster_publications.can_insert(admission.generation), "malformed Presence cleanup uses its pre-reserved publication slot");
        let generation = admission.generation;
        self.peer_roster_publications.insert_admitted(generation, PeerRosterPublication::<A>::rejected(admission, command, fault));
    }

    fn take_presence_outcome(&mut self) -> Option<PresenceRosterOutcome> {
        let generation = self.peer_roster_outcome_generation.checked_add(1)?;
        let outcome = self.peer_roster_outcomes.remove(generation)?;
        self.peer_roster_outcome_generation = generation;
        Some(outcome)
    }

    async fn config_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.config_store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn window_config_packs(&self) -> Result<Vec<WindowConfigPack>, Fault> {
        self.window_config_store.packs().await
    }

    async fn load_window_config_pack(&mut self, pack: WindowConfigPack) -> Result<(), Fault> {
        self.window_config_store.load(pack).await
    }

    async fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.open_child(slot, child_id, dialect, envelope_pack, identity).await?;
        self.cache = None;
        Ok(())
    }

    /// 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
    /// persisted child list that reshuffles between reads would make every save look like a
    /// change to anything diffing it.
    async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut entries: Vec<protocol::ChildPackEntry> = Vec::with_capacity(self.children.len());
        for (key, entry) in self.children.keyed_entries() {
            let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), envelope_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }

    async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut entries: Vec<protocol::ChildHeadPackEntry> = Vec::with_capacity(self.children.len());
        for (key, entry) in self.children.keyed_entries() {
            let head_pack = entry.member.document_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildHeadPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), head_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }

    fn begin_document_archive_load(&mut self, operation: u64, mut archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        if archive.members.len() > protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive exceeds its fixed 1024-member authority").with_param("count", archive.members.len().to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
        }
        let archive_payload_bytes = archive.members.iter().fold(archive.parent_pack.len().checked_add(archive.parent_spr.len()), |total, entry| total.and_then(|total| total.checked_add(entry.envelope_pack.len())));
        if archive_payload_bytes.is_none_or(|bytes| bytes > protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES) {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive typed payload exceeds its fixed byte authority").with_param("bytes", archive_payload_bytes.unwrap_or(usize::MAX).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()));
        }
        if !self.document_archive_loads.can_insert(operation) {
            return Err(document_load_fault(DOCUMENT_LOAD_BUSY_CODE, "document archive operation identity is already live or its fixed slot is occupied"));
        }
        if archive.parent_pack.is_empty() || archive.parent_spr.is_empty() {
            return Err(document_load_fault(DOCUMENT_LOAD_INCOMPLETE_CODE, "document archive parent pack and SPR must both be present"));
        }
        archive.members.sort_by_key(|entry| entry.ordinal);
        for (ordinal, entry) in archive.members.iter().enumerate() {
            let fields = [
                entry.reference.artifact_id.as_str(),
                entry.reference.artifact_kind.as_str(),
                entry.reference.standard.as_str(),
                entry.reference.subset.as_str(),
                entry.owner.parent.artifact_id.as_str(),
                entry.owner.parent.artifact_kind.as_str(),
                entry.owner.parent.standard.as_str(),
                entry.owner.parent.subset.as_str(),
                entry.owner.slot.as_str(),
                entry.owner.child_id.as_str(),
            ];
            if usize::try_from(entry.ordinal).ok() != Some(ordinal)
                || entry.envelope_pack.is_empty()
                || fields.iter().any(|field| field.is_empty() || field.len() > store::MEMBER_OPEN_IDENTITY_BYTES || field.chars().any(char::is_control))
            {
                return Err(document_load_fault(DOCUMENT_LOAD_MEMBER_INVALID_CODE, "document archive member ordinal, identity, owner, or envelope is invalid").with_param("ordinal", (ordinal + 1).to_string()));
            }
        }
        archive.members.reverse();
        let limits =
            protocol::RetainedSprLimits { file_bytes: protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64, frame_body_bytes: store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES as u64 * 256, records: protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 * 8 };
        let history = protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(), limits).map_err(|error| document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, error.to_string()))?;
        self.document_archive_loads.insert_admitted(operation, ActiveDocumentArchiveLoad::new(operation, archive, history));
        Ok(())
    }

    fn begin_document_archive_merge(&mut self, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        PluginApp::begin_document_archive_load(self, operation, archive)?;
        self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge admission changed before it was flagged"))?.merge = true;
        Ok(())
    }

    /// 📬️ Observes the original mounted load; its registered maintenance job drives retained work.
    async fn poll_document_archive_load(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        if self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::awaits_merge) {
            self.merge_document_archive(operation, identity, cx).await?;
        }
        self.document_archive_loads.get(operation).map(ActiveDocumentArchiveLoad::status).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive authority changed before status publication"))
    }

    fn cancel_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> {
        let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?;
        match active.state {
            ActiveDocumentArchiveLoadState::Pending | ActiveDocumentArchiveLoadState::Running => {
                active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
                Ok(())
            }
            ActiveDocumentArchiveLoadState::Cancelled => Ok(()),
            ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Fault => Err(document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive operation cannot be cancelled")),
        }
    }

    fn acknowledge_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> {
        let active = self.document_archive_loads.get(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?;
        if !active.terminal() {
            return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive operation cannot be acknowledged before terminal status"));
        }
        if !active.inputs_are_empty() {
            return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "terminal recursive document archive operation still owns retained input"));
        }
        let mut active = self.document_archive_loads.remove(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive authority changed before acknowledgement"))?;
        drop(std::mem::take(&mut active.fault));
        drop(active);
        Ok(())
    }

    fn document_load_archive(&self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
        let Effect::LoadDocument { pack, spr } = self.stamp_load_document_identity(Effect::LoadDocument { pack: files.pack.clone(), spr: files.spr.clone() })? else {
            return Err(plugin_sdk_fault("the identity stamp answered something other than a document load"));
        };
        Ok(protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() })
    }

    fn document_identity(&self) -> Option<String> {
        let id = &self.store.envelope().id;
        (1..=512).contains(&id.chars().count()).then(|| id.clone())
    }

    async fn document_archive(&self) -> Result<protocol::DocumentArchivePack, Fault> {
        let parent_generation = self.store.generation_now();
        let child_generation = self.child_content_generation;
        let parent = store::os_store::text_format::print_document_pack(self.store.envelope()).await.map_err(|error| error.into_fault())?;
        let members = self.archive_member_entries().await?;
        if self.store.generation_now() != parent_generation || self.child_content_generation != child_generation {
            return Err(document_load_fault(DOCUMENT_LOAD_CHANGED_CODE, "document archive authority changed during generation-fenced export"));
        }
        Ok(protocol::DocumentArchivePack { parent_pack: parent.pack, parent_spr: parent.spr, members })
    }

    async fn load_config_pack(&mut self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<(), Fault> {
        let parsed: store::os_store::text_format::ParsedDocumentText<A::Config, A::ConfigMutation> = store::os_store::text_format::parse_document_pack(&files.pack, &files.spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.into_fault())?;
        self.config_store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
        self.cache = None;
        Ok(())
    }

    async fn dispatch_config_command(&mut self, command_bytes: &mut Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.log_generation;
        let Some(result)=self.dispatch_config_command_inner(command_bytes,meta,identity,cx).await? else{return Ok(None)};
        Ok(Some(self.finish_recorded(log_generation_before,"configCommand",result).await))
    }

    async fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let envelopes = protocol::decode_envelopes(mutations).map_err(|error| error.into_fault())?;
        let mut reports = Vec::with_capacity(envelopes.len());
        let generation = self.store.generation();
        for envelope in envelopes {
            reports.push(self.store.ingest_remote(envelope).await.map_err(|error| error.into_fault())?);
        }
        self.cache = None;
        if self.store.generation() != generation {
            self.deliver_base_moved(identity, cx).await?;
        }
        Ok(reports)
    }

    async fn ingest_operations_text(&mut self, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut mutations: Vec<A::Mutation> = Vec::new();
        for line in operations_text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            mutations.push(A::Mutation::parse_op(line).map_err(|error| error.into_fault())?);
        }
        if mutations.is_empty() {
            return Ok(());
        }
        self.store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        self.cache = None;
        Ok(())
    }

    async fn document_text(&self) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
        store::os_store::text_format::print_document_text(self.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn document_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.store.attach_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.announce_member_event_logs().await?;
        self.cache = None;
        Ok(())
    }

    async fn attach_hot_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.store.attach_hot_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.cache = None;
        Ok(())
    }

    async fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let generation = self.store.generation();
        let reports = self.store.tick_backbone_reports().await.map_err(|error| error.into_fault())?;
        if self.store.generation() != generation {
            self.deliver_base_moved(identity, cx).await?;
        }
        self.follow_derivable_children(identity).await?;
        let folded_member_lanes = self.fold_member_inbound().await?;
        if !reports.is_empty() || folded_member_lanes != 0 {
            self.cache = None;
        }
        Ok(reports)
    }

    async fn detach_backbone(&mut self) -> Result<(), Fault> {
        self.store.detach_backbone().map_err(|error| error.into_fault())?;
        self.cache = None;
        Ok(())
    }

    /// 🕰️ Framework-owned, snapshot-independent — served before any app body-key match.
    ///
    /// 🕹️ Task 5: materialized once, before either branch, then used to stamp EVERY
    /// `interaction_domain`-bound `UiTree` this render produces — see `stamp_and_cache_interaction_ui`.
    ///
    /// 🕹️ Materialized owned BEFORE the field-wise destructure below (same reasoning as
    /// `stamp_and_cache_interaction_ui`'s own comment): `interaction_hover`/`peer_presence`
    /// are only ever READ here, so a snapshot avoids an aliasing conflict against `&mut self`.
    async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<()>,semio_framework_job::OriginalTurnFailure> {
        if self.receive_render_failure(cx)?||cx.should_yield(){return Ok(None)}
        if self.pending_render_tree.is_none()&&self.pending_render_tree_close.is_none()&&self.pending_render_tree_ready{self.pending_render_tree_identity=None;self.pending_render_tree_ready=false;}
        if let Some(identity)=self.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}else{self.pending_render_tree_identity=Some(OriginalRenderTreeIdentity::capture(cx)?);}
        if self.pending_render_tree.is_some(){return Ok(self.pending_render_tree_ready.then_some(()));}
        let original_render_result:crate::OriginalRenderResult<()>=async {
        if let Some(section)=semio_framework::UiRefreshSection::from_body_key(body_key){
            let tree=crate::plugin_runtime::original_render_section(self,section,view_state)?;
            self.retain_pending_render_tree(tree);
            self.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        self.refresh_cache().await?;
        if body_key == FRAMEWORK_HISTORY_BODY_KEY {
            let Some((_, _, _, history)) = self.cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let mut time_travel = self.time_travel.panel();
            if let Some(panel) = time_travel.as_mut() {
                self.resolve_time_travel_snaps(panel, view_state);
                self.resolve_time_travel_options(panel);
                self.resolve_time_travel_reference_labels(panel);
            }
            let reprojection = self.reprojection_status();
            let pages = self.history_mutation_pages(history, time_travel.as_ref(), view_state);
            let root =
                ui_history_panel(history, time_travel.as_ref(), reprojection.as_ref(), &pages, &self.registry.controller_id, view_state.locale, A::ROLE == AppRole::Viewer, self.time_travel_busy(), view_state).await?;
            self.retain_pending_render_tree(built_to_component_tree(root));
            self.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        if body_key == FRAMEWORK_TOOL_RUN_BODY_KEY {
            let (registry, locale) = (&self.registry, view_state.locale);
            let ready_tool = [view_state.active_utility_id.as_deref(), view_state.active_tool_id.as_deref()].into_iter().flatten().find(|tool_id| registry.tool_run(tool_id).is_some());
            let root = self
                .tool_runs
                .panel(&registry.controller_id, locale, ready_tool, |tool_id| registry.tool_run(tool_id).map(|(label, _)| label.resolve(Terminology::Native, locale).to_string()))
                ?;
            self.retain_pending_render_tree(built_to_component_tree(root));
            self.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let interaction_state = self.interaction_state().await;
        let interaction_hover = self.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = self.window_config_store.capture(Some(view_state)).await?;
        let window_transient = self.window_transient_store.capture(Some(view_state))?;
        if let Some(json) = snapshot_override_json {
            let snapshot: A::Snapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|plugin_sdk_fault(error.to_string()))?;
            let history = std::sync::Arc::clone(&self.cache.as_ref().ok_or_else(|| plugin_sdk_fault("render cache unavailable after refresh"))?.3);
            let doc = ArtifactView::new(&snapshot, history.as_ref()).with_instance(self.instance_operation_owner.clone());
            let config = self.config_store.snapshot().unwrap_or_else(|_| A::Config::default());
            let cfg = ConfigView { snapshot: &config, window: window_config.as_ref().map(|authority| &authority.snapshot) };
            let transient = self.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let Some(node) = A::render_with_request_context(&self.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await? else{return Ok(None)};
            self.retain_pending_render_tree(node);
            self.stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
            self.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let operation_progress = self.operation_progress_snapshot();
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let Some(node) = ({
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
            let Some((_, snapshot, config, history)) = cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let doc = ArtifactView::with_render_context(
                time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
                history.as_ref(),
                ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
                render_operation,
                parent_document_id,
                None,
            )
            .await
            .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
            .with_instance(self.instance_operation_owner.clone())
            .with_operations(&operation_progress);
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient = self.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            A::render_with_request_context(&self.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await?
        }) else{return Ok(None)};
        self.retain_pending_render_tree(node);
        self.inject_tool_run_trace_lane(body_key, view_state)?;
        self.stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
        self.pending_render_tree_ready=true;
        Ok(Some(()))
        }.await;
        match original_render_result{Ok(tree)=>Ok(tree),Err(crate::OriginalRenderFailure::Original(failure))=>self.retain_render_failure(crate::OriginalRenderFailure::Original(failure),cx),Err(source)=>self.retain_render_failure(source,cx)}
    }

    fn original_render_tree_source<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<&'a mut Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>{
        self.pending_render_tree_identity.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source has no initiating identity"))?.validate(cx)?;
        if !self.pending_render_tree_ready || self.pending_render_failure.is_some() || self.pending_render_compound.is_some() || self.pending_render_close_failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source is not ready for donation").into());}
        Ok(&mut self.pending_render_tree)
    }

    /// 🕹️ Same materialize-before-destructure shape the `render`/`window_measures`/`context_menu`
    /// paths use: `interaction_hover`/`peer_presence` are read-only here, so an owned snapshot
    /// avoids aliasing the `&mut self` field-wise destructure below.
    async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction_state().await;
        let interaction_hover = self.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, window_transient_store, cache, child_content_root, transient_store, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.instance_operation_owner.clone());
        let mut engagements = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let window_transient = match window_transient_store.capture(Some(&window_view_state)) {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient_root = transient_store.current_root();
            let transient = TransientView { snapshot: transient_root.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let mut projected = A::window_engagements_with_request_context(&doc, &cfg, &window_view_state, &transient, &interaction).await;
            if let Some(engagement) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                engagements.insert(window.id.clone(), engagement);
            }
        }
        engagements
    }

    async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction_state().await;
        let interaction_hover = self.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.instance_operation_owner.clone());
        let mut measures = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let mut projected = A::window_measures_with_request_context(&doc, &cfg, &window_view_state, &interaction).await;
            if let Some(window_measures) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                measures.insert(window.id.clone(), window_measures);
            }
        }
        measures
    }

    async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let window_config = match self.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return HashMap::new(),
        };
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
        A::tool_measures(&doc, &cfg, view_state).await
    }

    async fn app_catalogue_json(&mut self) -> String {
        A::app_catalogue_json().await
    }

    /// 🔒️ The app instance's RETAINED operation owner, handed to the poll for the same
    /// reason `render_with_request_context` gets it: window-scoped background work an app
    /// arms from here is latched on state only that owner holds, and a poll that cannot see
    /// it re-arms an already-pending chain once per refresh
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏯️ The poll sees the LIVE RUN, exactly as the four render and measure passes above do.
    /// `ArtifactApp::pending_effects` is where a surface starts a run once its work is owed and
    /// finalizes a complete one, and that whole ladder is written over `doc.tool_run()` — handed a
    /// view that always answered `None` it could only ever read "no run": the generation3d preview
    /// run started, ticked, completed and was never finalized, so the next `toolRunStart` a gesture
    /// asked for was a no-op against a run the framework still held and the 3d preview stopped
    /// re-evaluating for the rest of the session
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, contract §3.7). The document itself
    /// stays the COMMITTED snapshot: a poll decides about work over what has landed, never over a
    /// run's own provisional overlay.
    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
        if self.refresh_cache().await.is_err() {
            return Vec::new();
        }
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let snapshot_is_admitted = {
            let (_, snapshot, _, _) = self.cache.as_ref().expect("cache refreshed above");
            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())
        };
        let snapshot_read = if snapshot_is_admitted {
            match self.store.snapshot_read() {
                Ok(snapshot_read) => Some(snapshot_read),
                Err(_) => return Vec::new(),
            }
        } else {
            None
        };
        let instance_operation_owner = self.instance_operation_owner.clone();
        let effects = {
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, tool_machines, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, snapshot_read)
                .await
                .with_instance(instance_operation_owner.clone())
                .with_tool_run(tool_runs.view_for(view.and_then(|view| view.window_id.as_deref())))
                .with_provisional_generation(tool_machines.provisional_generation());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            A::pending_effects(&instance_operation_owner, &doc, &cfg, view).await
        };
        self.stamp_polled_load_document_effects(effects)
    }

    /// 🗂️ Every context menu is organized (D2 of the grouped-context-menu mechanism design) at this
    /// single funnel — a raw-vec emitter is grouped for free, and an emitter that already built its own
    /// `Menu::group(...)` rows is never re-flattened (`organize_context_menu` is idempotent on already-organized input).
    async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> {
        if self.refresh_cache().await.is_err() {
            return Vec::new();
        }
        let interaction_state = self.interaction_state().await;
        let interaction_hover = self.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = match self.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, registry, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: config.as_ref(), window: window_config.as_ref().map(|authority| &authority.snapshot) };
        let items = A::context_menu_with_request_context(request, &doc, &cfg, view_state, &interaction, registry).await;
        ui_wgpu::wgpu::organize_context_menu(items, &|id| registry.category_of(id))
    }

    /// 📤️ ABI-level media delivery for ONE port — the route `AppCommand::MediaOut` reaches
    /// through `plugin_produce_media`.
    ///
    /// 🎞️ Produces one document or declared media output while preserving the initiating pending turn.
    async fn produce_media(&mut self, port: &str, owner: &mut crate::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,crate::app_contract::MediaArtifactOperationFailure> {
        if port == NATURAL_FILE_PORT {
            let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::Payload("natural file exporter is not mounted".into()))?;
            self.refresh_cache().await?;
            let snapshot = self.cache.as_ref().expect("cache refreshed above").1.as_ref();
            let data = A::encode_natural_file(snapshot).await.map_err(MediaArtifactError::Export)?;
            return Ok(Some(MediaArtifact {
                descriptor: MediaArtifactDescriptor {
                    edge_id: None,
                    port_id: Some(port.to_string()),
                    kind_id: Some(codec.format_kind.to_string()),
                    media_type: None,
                    wire: MediaWireFormat::Binary { format_kind: codec.format_kind.to_string() },
                    blob_hash: None,
                },
                data,
            }));
        }
        if port != "artifact:out" {
            let exported = owner.receive_nested::<Media, Result<Option<MediaArtifact>,crate::app_contract::MediaArtifactOperationFailure>>(|slot, original| {
                let media = match ::semio_framework_async::poll::resolve_ready(self.export_media(port,cx)) {
                    Ok(Some(media))=>media,
                    Ok(None)=>return Ok(Ok(None)),
                    Err(error)=>return Ok(Err(error.into())),
                };
                *slot = Some(media);
                let media = slot.as_ref().unwrap();
                let artifact = match &media.payload {
                    MediaPayload::Intrinsic { schema, value } => MediaArtifact::encode_intrinsic(port, media.media_type, schema, value, original).map_err(|error| match error { MediaArtifactError::Value(error) => error, _ => media_projection_error() })?,
                    MediaPayload::Structured { schema, json } => {
                        let (port, schema, text) = copy_media_text_parts(port, schema, json, original)?;
                        original.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), ..Default::default() })?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind: schema }, blob_hash: None }, data: text.into_bytes() }
                    }
                    MediaPayload::Binary { format_kind, blob_hash } => {
                        let (port, format_kind, blob_hash) = copy_media_text_parts(port, format_kind, blob_hash, original)?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind }, blob_hash: Some(blob_hash) }, data: Vec::new() }
                    }
                };
                Ok(Ok(Some(artifact)))
            })??;
            return Ok(exported);
        }
        let archive = PluginApp::document_archive(self).await?;
        whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await.map(Some).map_err(Into::into)
    }

    async fn prepare_media_import(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if let Some((_,original))=self.pending_media_import_permit.as_ref(){
            if *original!=source{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media import requires its retained original receiving source").into())}
            return Ok(!cx.should_yield()&&cx.retained_grant().maximum_items>0&&cx.retained_grant().maximum_depth>0)
        }
        let meta=ActionMeta{actor:self.store.local_actor_id().0.clone(),instance_id:self.live_runtime_instance_id.ok_or_else(||plugin_sdk_fault("media import requires its mounted live instance"))?,view_state:None};
        let Some(permit)=self.admit_reserved_emit_producer("import-media",&meta,cx)?else{return Ok(false)};
        *self.pending_media_import_permit=Some((permit,source));Ok(false)
    }

    async fn consume_media(&mut self, port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> {
        if port != NATURAL_FILE_PORT {
            let schema_now = self.artifact_schema().await.to_string();
            return match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),
                MediaWireFormat::Document { schema } => Err(MediaArtifactError::SchemaMismatch { expected: schema_now, found: schema }),
                MediaWireFormat::Binary { format_kind } => Err(MediaArtifactError::NoImporter(format_kind)),
                MediaWireFormat::Intrinsic { .. } => {
                    let media = {
                        let mut accept = |_| true;
                        let mut control = semio_framework_value::NativeDecodeControl::new(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, &mut accept);
                        artifact.decode_intrinsic(&mut control)?
                    };
                    let instance_id = self.live_runtime_instance_id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "intrinsic media import requires a bound live app instance"))?;
                    let actor = self.store.local_actor_id().0.clone();
                    self.dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx).await.map(|_| MediaConsumption::Applied).map_err(MediaArtifactError::Import)
                }
            };
        }
        let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::NoImporter(port.to_string()))?;
        let MediaWireFormat::Binary { format_kind } = artifact.descriptor.wire else {
            return Err(MediaArtifactError::NoImporter(codec.format_kind.to_string()));
        };
        if format_kind != codec.format_kind {
            return Err(MediaArtifactError::NoImporter(format_kind));
        }
        let instance_id = self.live_runtime_instance_id.ok_or_else(|| MediaArtifactError::Payload("natural file import requires a bound live app instance".into()))?;
        let actor = self.store.local_actor_id().0.clone();
        let media_type = A::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let media = natural_file_input_media(codec, media_type, artifact.data);
        self.dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx)
            .await
            .map(|_| MediaConsumption::Applied)
            .map_err(|fault| MediaArtifactError::Payload(fault.message))
    }

    /// 🎞️ Batch-only adapter. Registered interactive outputs are driven through the same
    /// externally addressable submission API; this loop is not used by UI/workflow entry points.
    async fn export_media(&mut self,port:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Media>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        if self.app_tool_registrations.keys().any(|verb|verb.strip_prefix("export-media:")==Some(port)){
            let operation_id=if let Some(id)=self.current_media_export{if self.media_exports.get(id).is_none_or(|active|active.port!=port){return self.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"current export belongs to a different original port".into()),cx)}id}else{let Some(instance)=self.live_runtime_instance_id else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media requires its mounted instance").into())};if !self.receive_owned_media_export(port,instance,None,cx).await?{return Ok(None)}self.current_media_export.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media publication lost its live operation"))?};
            if self.media_export_preparation.is_some(){
                self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media lost its original supplier"))?.validate(cx)?;
                let grant=cx.retained_grant();self.close_media_submission_refusal(cx,grant)?;return Ok(None)
            }
            return match self.receive_owned_media_poll(operation_id,cx).await?{None|Some(ArtifactMediaExportPoll::Running{..})=>Ok(None),Some(ArtifactMediaExportPoll::Complete(result))=>match result.into_batch_media(){Ok(media)=>Ok(Some(media)),Err(source)=>self.retain_media_submission_refusal(source,cx)},Some(ArtifactMediaExportPoll::Cancelled)=>self.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"media export cancelled".into()),cx),Some(ArtifactMediaExportPoll::Failed(detail))=>self.retain_media_submission_refusal(MediaError::Payload(port.to_string(),detail),cx)};
        }
        self.refresh_cache().await?;
        let render_operation=self.live_render_operation();let parent_document_id=self.store.envelope().id.clone();
        let VcsArtifactApp{app:_,cache,child_content_root,transient_store,instance_operation_owner,..}=self;
        let(_,snapshot,config,history)=cache.as_ref().expect("cache refreshed above");
        let doc=ArtifactView::with_render_context(snapshot.as_ref(),history.as_ref(),ChildContentView::clone(child_content_root),render_operation,parent_document_id,None).await.with_instance(instance_operation_owner.clone());let _cfg=ConfigView{snapshot:config.as_ref(),window:None};let transient=transient_store.current_root();
        match A::export_media_with_request_context(instance_operation_owner,port,&doc,&TransientView{snapshot:transient.as_ref(),window:None}).await{Ok(media)=>Ok(Some(media)),Err(source)=>self.retain_media_submission_refusal(source,cx)}
    }

    async fn submit_media_export(&mut self,port:&str,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let Some(app_instance_id)=self.live_runtime_instance_id else{return self.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()),cx).map(|_|false)};
        self.receive_owned_media_export(port,app_instance_id,Some(receiver),cx).await
    }

    async fn submit_media_export_checked(&mut self,port:&str,expected_parent_document_id:&str,expected_base_revision:u64,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let parent_document_id=self.store.envelope().id.as_str();let content_revision=self.store.content_revision();let base_revision=u64::from_be_bytes(content_revision[..8].try_into().expect("revision lane width"));
        if parent_document_id!=expected_parent_document_id||base_revision!=expected_base_revision{return self.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"media export resource authority is stale".into()),cx).map(|_|false)}
        let Some(app_instance_id)=self.live_runtime_instance_id else{return self.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()),cx).map(|_|false)};
        self.receive_owned_media_export(port,app_instance_id,Some(receiver),cx).await
    }

    async fn poll_media_export(&mut self,handle:crate::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        self.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling lost its original source"))?.validate(cx)?;
        if receiver.is_some(){return Ok(true)}
        if self.media_exports.get(handle.operation_id.0).is_none_or(|active|active.handle.original_loan()!=handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling requires its exact original live handle").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactMediaExportPoll>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Some(poll)=self.receive_owned_media_poll(handle.operation_id.0,cx).await?else{return Ok(false)};
        if matches!(poll,ArtifactMediaExportPoll::Complete(_)){self.media_closures.get_mut(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal media poll lost its original close owner"))?.download_pending=true}
        *receiver=Some(poll);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    async fn cancel_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        self.cancel_owned_media_export(handle).await
    }

    async fn retain_media_export_result(&mut self,handle:crate::media_contract::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<crate::media_contract::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let operation_id=handle.operation_id.0;let identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if handle.parent_document_id.len()>crate::media_contract::ARTIFACT_OUTPUT_METADATA_BYTES{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"download handle document metadata exceeds its original limit").into())}
        self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original source"))?.validate(cx)?;
        if self.media_download_preparation.is_none()&&status.is_some()&&source.is_none(){return if self.segmented_downloads.get(operation_id).is_some_and(|output|output.owns_media_export_loan(handle)){Ok(true)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download lost its exact registry recipient").into())}}
        if self.media_download_preparation.is_none(){
            let active=self.media_closures.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original completed operation"))?;
            if active.handle.original_loan()!=handle{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its exact completed business handle").into())}
            if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if !result.chunks.same_operation(&active.output_chunks){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its original supplied chunks").into())}}
        }
        let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media_download_preparation.is_none(){
            if status.is_some()||!self.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving slot is occupied").into())}
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its retained completion").into())};
            let bytes=size_of::<crate::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            *self.media_download_preparation=Some(crate::media_export::MediaDownloadPreparation{operation_id,source_identity:identity,document_identity:(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()),mime_identity:(result.mime_type.as_ptr()as usize,result.mime_type.len()),chunks_identity:result.chunks.original_identity(),coordinates:(handle.app_instance_id,handle.base_revision,handle.generation),buffers:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),buffers_close:ManuallyDrop::new(None),stage:0,handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),output:ManuallyDrop::new(None),output_close:ManuallyDrop::new(None),published:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media_download_preparation.as_mut().unwrap();if original.operation_id!=operation_id||original.source_identity!=identity||original.coordinates!=(handle.app_instance_id,handle.base_revision,handle.generation)||original.document_identity!=(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download publication requires its unchanged original source and handle").into())}
        if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if original.chunks_identity!=result.chunks.original_identity(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its same actual result root").into())}}
        if original.published{
            if source.is_some()||status.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download has inconsistent original receiving slots").into())}
            let bytes=size_of::<crate::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}self.media_download_preparation.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if original.stage<2{
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download metadata lost its retained result").into())};if original.mime_identity!=(result.mime_type.as_ptr()as usize,result.mime_type.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download requires its unchanged original MIME source").into())}
            let input=if original.stage==0{handle.parent_document_id.as_bytes()}else{result.mime_type.as_bytes()};let output=&mut original.buffers.as_mut().unwrap()[original.stage];
            if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
            if output.capacity()==0&&!input.is_empty(){if grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"download metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.handle.is_none()&&status.is_none(){
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download metadata donation lost its original result").into())};let total_bytes=result.chunks.bytes()as u64;let bytes=size_of::<ArtifactMediaExportHandle>()+size_of::<crate::media_contract::ArtifactMediaExportPublishedStatus>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            let buffers=original.buffers.as_mut().unwrap();let document=String::from_utf8(std::mem::take(&mut buffers[0])).expect("copied original document remains UTF-8");let mime_type=String::from_utf8(std::mem::take(&mut buffers[1])).expect("copied original MIME remains UTF-8");*original.handle=Some(ArtifactMediaExportHandle{app_instance_id:handle.app_instance_id,parent_document_id:document,operation_id:handle.operation_id,base_revision:handle.base_revision,generation:handle.generation});*status=Some(crate::media_contract::ArtifactMediaExportPublishedStatus{mime_type,total_bytes});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.output.is_none(){let _=ArtifactDownloadOutput::receive_media_export_original(&mut original.handle,source,&mut original.output,cx)?;return Ok(false)}
        let bytes=size_of::<ArtifactDownloadOutput>()+2*size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}if !self.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download registry receiving slot changed before publication").into())}
        self.segmented_downloads.insert_admitted(operation_id,original.output.take().unwrap());original.published=true;if let Some(active)=self.media_closures.get_mut(operation_id){active.download_pending=false}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    async fn take_media_export_chunk(&mut self,handle:crate::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if receiver.is_some(){return Ok(true)}let operation_id=handle.operation_id.0;
        if *terminal&&self.segmented_downloads.get(operation_id).is_none(){return Ok(true)}
        self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving lost its original source"))?.validate(cx)?;
        let output=self.segmented_downloads.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving has no exact output"))?;
        if !output.owns_media_export_loan(handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving requires its exact handle").into())}
        if !*terminal{if !output.chunks.take_chunk_original(receiver,terminal,cx)?{return Ok(false)}return Ok(receiver.is_some())}
        if !output.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download EOF retains unread original chunks").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactDownloadOutput>()+size_of::<crate::media_export::MediaDownloadRetirement>();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        if !self.segmented_closures.can_insert(operation_id){return Ok(false)}let output=self.segmented_downloads.remove(operation_id).expect("checked exact original download");self.segmented_closures.insert_admitted(operation_id,crate::media_export::MediaDownloadRetirement::pending(output));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    async fn take_segmented_download_chunk(&mut self, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> {
        let output = self.segmented_downloads.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-segmented-download"), format!("unknown segmented download operation {operation_id}")))?;
        let chunk = output.chunks.take_chunk()?;
        if chunk.is_none() {
            let output =
                self.segmented_downloads.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-authority"), "terminal segmented download changed before exact removal"))?;
            if !output.terminal_is_empty() {
                self.segmented_downloads.insert_admitted(operation_id, output);
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-terminal-not-empty"), "terminal segmented download retained unread chunks"));
            }
            drop(output);
        }
        Ok(chunk)
    }

    async fn import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let log_generation_before = self.log_generation;
        let result = self.dispatch_import_media(port, media, meta, cx).await?;
        Ok(self.finish_recorded(log_generation_before, &format!("import-media:{port}"), result).await)
    }

    async fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
        self.refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
        let render_operation = self.live_render_operation();
        let parent_document_id = self.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let _cfg = ConfigView { snapshot: config.as_ref(), window: None };
        A::media_fingerprint(port, &doc).await
    }
}

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../🪆️child/👁️read/🧪️test-view/🦀️.rs"]
mod child_frame_scene_view;

#[path = "⏪️time-travel/🦀️.rs"]
mod time_travel_turns;
#[path = "⏯️tool-run/🦀️.rs"]
mod tool_run_turns;
#[path = "🛠️tool-machine/🦀️.rs"]
mod tool_machine_turns;
#[path = "⏳️operation-progress/🦀️.rs"]
mod operation_progress_turns;
#[path = "🔄️full-operation/🦀️.rs"]
mod full_operation_turns;
#[path = "🤝️group/🦀️.rs"]
mod child_group_turns;
#[path = "../🪜️close-ladder/🦀️.rs"]
mod close_ladder;
#[path = "../🪜️envelope-ladder/🦀️.rs"]
mod envelope_ladder;
#[path = "../🪜️maintenance-ladder/🦀️.rs"]
mod maintenance_ladder;
include!("../♻️lifecycle/🏪️stores/🦀️.rs");
