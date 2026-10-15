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

#[path = "🚪️ingress/🦀️.rs"]
mod ingress;
#[path = "📨️dispatch/🦀️.rs"]
mod dispatch;
#[path = "🖼️render/🦀️.rs"]
mod render;
#[path = "🧷️command-routes/🦀️.rs"]
mod command_routes;
#[path = "🧾️typed-operations/🦀️.rs"]
mod typed_operations;
#[path = "🔌️plugin-app-impl/🦀️.rs"]
mod plugin_app_impl;
pub use plugin_app_impl::*;

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
