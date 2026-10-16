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
use semio_framework::{kernel::{ActorId, AppEvent, ArtifactDiff, ArtifactHandle, ArtifactVersion, ClipboardFragment, EditRef, Effect, HistoryEntry, HistoryPatch, HybridLogicalTimestamp, InverseMutation, InvocationId, InvocationResult, KernelMutation, MutationId, SchemaId, UndoGroup, UndoPolicy}, ActionArgDef, ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin, InteractionDefinition, InteractionVerb, MediaForm, ViewModel, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework::onboarding::RECORD_TUTORIAL_ACTION_ID;
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
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuRequest;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowMeasure;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
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
use crate::InstanceExtensions;
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
use crate::tool_jobs::record_typed_operation_unit;
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


pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
    pub(crate) app: A,
    pub(crate) mounted_policy: crate::MountedOwnerPolicyV1,
    /// 📂️ Store-constructor state: the receipts, units and lane of the four original catalogue constructors.
    pub(crate) constructor: ConstructorState<A>,
    /// 📒️ The stores of the instance: document, config, draft, presence, transient, interaction and window owner registries with their one-item publication factories.
    pub(crate) stores: InstanceStores<A>,
    /// 📀️ The peer roster: this actor's colour, the adopted peer presence and the roster publication admission state.
    pub(crate) roster: PeerRoster<A>,
    /// 🪙️ Action ingress: the parser slots that retain unused original arguments and faults until their paid close.
    pub(crate) ingress: ActionIngress,
    /// 🗜️ Local interaction: the framework-owned live query, topology authority, hover and snapshot read returns.
    pub(crate) interaction: LocalInteraction<A>,
    /// ⛓️ Command history: the session command log, the cached history view, backfill marks and the supersede ledger.
    pub(crate) history: CommandHistory<A>,
    /// ✉️ Host events: the utilities, window roster and typed operations a host event dispatch reached.
    pub(crate) host_events: HostEvents,
    /// 🎴️ Transactions: the single pending foreign transaction and its proposal.
    pub(crate) txn: Transactions<A>,
    /// 🧿️ Child composition: the live owned children, their private groups, content roots and retirement registries.
    pub(crate) child_composition: ChildComposition<M>,
    /// 🪬️ Pending reserved work: the reserved commits, render trees, configuration commands and their failure custody.
    pub(crate) pending: PendingReserved<A>,
    /// 🔩️ Tool operations: typed operation outboxes, tool jobs and registrations, cancellations and latest-wins commands.
    pub(crate) tools: ToolOperations<A>,
    /// 🧴️ Media transfers: the media exports, downloads, closures and snapshot retirements of the instance.
    pub(crate) media: MediaTransfers<A>,
    /// 🪞️ Envelope jobs: envelope ingress and decode, store replacement and document archive loads.
    pub(crate) ingest: EnvelopeJobs<A, M>,
    /// 🪠️ Maintenance cursors: the rotation positions of the live-maintenance ladder.
    pub(crate) maintenance: MaintenanceCursors,
    /// 🧽️ Close cursors: the positions and drained flags of the close ladder.
    pub(crate) close: CloseCursors<A, M>,
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
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🔍️ Read-only inspection of this instance's tool run ledger for app-level tests.
    #[cfg(feature = "artifact-app-testing")]
    pub fn tool_run_ledger(&self) -> &ToolRunLedger<A> {
        &self.tool_runs
    }
}

pub use plugin_app_impl::*;
impl<A: ArtifactApp, M: SpaceMember + MemberFactory> Drop for VcsArtifactApp<A, M> {
    fn drop(&mut self) {
        self.tools.tool_cancellations.cancel_scope_generation();
        while let Some((_, id)) = self.pending.pending_reserved.next_id_from(0) {
            if let Some(pending) = self.pending.pending_reserved.remove(id) {
                pending.permit.finish();
            } else {
                break;
            }
        }
        while let Some(commit) = self.pending.reserved_commits.pop_front() {
            commit.permit.finish();
        }
    }
}

//#region 📥️StagedInstanceMounts
#[path = "🏪️stores/📥️staging/⏪️time-travel/🦀️.rs"]
mod stores_time_travel;

#[path = "🏪️stores/📥️staging/⏯️tool-run/🦀️.rs"]
mod stores_tool_run;

#[path = "🏪️stores/📥️staging/📨️dispatch/🦀️.rs"]
mod stores_dispatch;

#[path = "🏪️stores/📥️staging/🖼️render/🦀️.rs"]
mod stores_render;

#[path = "🏪️stores/📥️staging/🚪️ingress/🦀️.rs"]
mod stores_ingress;

#[path = "🏪️stores/📥️staging/🛠️tool-machine/🦀️.rs"]
mod stores_tool_machine;

#[path = "🏪️stores/📥️staging/🧵️instance-root/🦀️.rs"]
mod stores_instance_root;

#[path = "🏪️stores/📥️staging/🧷️command-routes/🦀️.rs"]
mod stores_command_routes;

#[path = "🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs"]
mod stores_typed_operations;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🏪️stores/📥️staging/🪆️child-test-view/🦀️.rs"]
mod stores_child_test_view;

#[path = "🏪️stores/📥️staging/🪜️maintenance-ladder/🦀️.rs"]
mod stores_maintenance_ladder;

#[path = "📜️history/📥️staging/⏪️time-travel/🦀️.rs"]
mod history_time_travel;

#[path = "📜️history/📥️staging/📨️dispatch/🦀️.rs"]
mod history_dispatch;

#[path = "📜️history/📥️staging/🤝️group/🦀️.rs"]
mod history_group;

#[path = "🪆️children/📥️staging/⏪️time-travel/🦀️.rs"]
mod children_time_travel;

#[path = "🪆️children/📥️staging/⏯️tool-run/🦀️.rs"]
mod children_tool_run;

#[path = "🪆️children/📥️staging/📨️dispatch/🦀️.rs"]
mod children_dispatch;

#[path = "🪆️children/📥️staging/🔌️plugin-app-impl/🦀️.rs"]
mod children_plugin_app_impl;

#[path = "🪆️children/📥️staging/🖼️render/🦀️.rs"]
mod children_render;

#[path = "🪆️children/📥️staging/🚪️ingress/🦀️.rs"]
mod children_ingress;

#[path = "🪆️children/📥️staging/🤝️group/🦀️.rs"]
mod children_group;

#[path = "🪆️children/📥️staging/🧵️instance-root/🦀️.rs"]
mod children_instance_root;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🪆️children/📥️staging/🪆️child-test-view/🦀️.rs"]
mod children_child_test_view;

#[path = "👷️jobs/📥️staging/⏯️tool-run/🦀️.rs"]
mod jobs_tool_run;

#[path = "👷️jobs/📥️staging/⏳️operation-progress/🦀️.rs"]
mod jobs_operation_progress;

#[path = "👷️jobs/📥️staging/🎟️lease/🦀️.rs"]
mod jobs_lease;

#[path = "👷️jobs/📥️staging/📨️dispatch/🦀️.rs"]
mod jobs_dispatch;

#[path = "👷️jobs/📥️staging/🔁️document-replacement/🦀️.rs"]
mod jobs_document_replacement;

#[path = "👷️jobs/📥️staging/🔒️close-ladder/🦀️.rs"]
mod jobs_close_ladder;

#[path = "👷️jobs/📥️staging/🖼️render/🦀️.rs"]
mod jobs_render;

#[path = "👷️jobs/📥️staging/🚪️ingress/🦀️.rs"]
mod jobs_ingress;

#[path = "👷️jobs/📥️staging/🤝️group/🦀️.rs"]
mod jobs_group;

#[path = "👷️jobs/📥️staging/🧵️instance-root/🦀️.rs"]
mod jobs_instance_root;

#[path = "👷️jobs/📥️staging/🧷️command-routes/🦀️.rs"]
mod jobs_command_routes;

#[path = "👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs"]
mod jobs_typed_operations;

#[path = "📄️docs/📥️staging/⏪️time-travel/🦀️.rs"]
mod docs_time_travel;

#[path = "📄️docs/📥️staging/🎟️lease/🦀️.rs"]
mod docs_lease;

#[path = "📄️docs/📥️staging/📨️dispatch/🦀️.rs"]
mod docs_dispatch;

#[path = "📄️docs/📥️staging/🔌️plugin-app-impl/🦀️.rs"]
mod docs_plugin_app_impl;

#[path = "📄️docs/📥️staging/🔒️close-ladder/🦀️.rs"]
mod docs_close_ladder;

#[path = "📄️docs/📥️staging/🖼️render/🦀️.rs"]
mod docs_render;

#[path = "📄️docs/📥️staging/🗝️envelope-ladder/🦀️.rs"]
mod docs_envelope_ladder;

#[path = "📄️docs/📥️staging/🚪️ingress/🦀️.rs"]
mod docs_ingress;

#[path = "📄️docs/📥️staging/🧵️instance-root/🦀️.rs"]
mod docs_instance_root;

#[path = "📄️docs/📥️staging/🧷️command-routes/🦀️.rs"]
mod docs_command_routes;

#[path = "📄️docs/📥️staging/🧾️typed-operations/🦀️.rs"]
mod docs_typed_operations;

#[path = "📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs"]
mod docs_maintenance_ladder;

#[path = "🧷️command-routes/📥️staging/🎟️lease/🦀️.rs"]
mod commands_lease;

#[path = "🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs"]
mod commands_dispatch;

#[path = "🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs"]
mod commands_ingress;

#[path = "🧷️command-routes/📥️staging/🤝️group/🦀️.rs"]
mod commands_group;

#[path = "🧷️command-routes/📥️staging/🧵️instance-root/🦀️.rs"]
mod commands_instance_root;

#[path = "🧷️command-routes/🦀️.rs"]
mod command_routes;

#[path = "🧾️typed-operations/📥️staging/🔌️plugin-app-impl/🦀️.rs"]
mod typed_ops_plugin_app_impl;

#[path = "🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs"]
mod typed_ops_tool_machine;

#[path = "🧾️typed-operations/📥️staging/🧵️instance-root/🦀️.rs"]
mod typed_ops_instance_root;

#[path = "🧾️typed-operations/🦀️.rs"]
mod typed_operations;

#[path = "🖼️render/🦀️.rs"]
mod render;

#[path = "../🪜️maintenance-ladder/📥️staging/🏪️lifecycle-stores/🦀️.rs"]
mod ladders_lifecycle_stores;

#[path = "../🪜️maintenance-ladder/📥️staging/🔁️document-replacement/🦀️.rs"]
mod ladders_document_replacement;

#[path = "../🪜️maintenance-ladder/📥️staging/🔄️full-operation/🦀️.rs"]
mod ladders_full_operation;

#[path = "../🪜️maintenance-ladder/📥️staging/🔌️plugin-app-impl/🦀️.rs"]
mod ladders_plugin_app_impl;

#[path = "../🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs"]
mod ladders_close_ladder;

#[path = "../🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs"]
mod ladders_envelope_ladder;

#[path = "../🪜️maintenance-ladder/📥️staging/🚪️ingress/🦀️.rs"]
mod ladders_ingress;

#[path = "../🪜️maintenance-ladder/🦀️.rs"]
mod maintenance_ladder;

#[path = "📥️staging/📨️dispatch/🦀️.rs"]
mod app_dispatch;

#[path = "📥️staging/🔒️close-ladder/🦀️.rs"]
mod app_close_ladder;

#[path = "📥️staging/🚪️ingress/🦀️.rs"]
mod app_ingress;

#[path = "📥️staging/🪜️maintenance-ladder/🦀️.rs"]
mod app_maintenance_ladder;

#[path = "🔌️plugin-app-impl/🦀️.rs"]
mod plugin_app_impl;

#[path = "🧩️extensions/🦀️.rs"]
mod extensions;
pub(crate) use extensions::InstanceExtension;
pub(crate) use children_instance_root::{genesis_member_schema};
pub(crate) use children_plugin_app_impl::{MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY};
pub(crate) use docs_instance_root::{artifact_handle_of, typed_operation_document_is_fresh};
use docs_instance_root::{MediaExportPreparation};
pub(crate) use docs_maintenance_ladder::{item};
pub(crate) use docs_plugin_app_impl::{history_changed_event};
use jobs_document_replacement::{DocumentWindowResetAdmission};
use jobs_instance_root::{OriginalRenderTreeIdentity};
use jobs_lease::{OriginalToolLeaseAdmission};
pub(crate) use ladders_document_replacement::{retire_document_window_registry_demands, retire_document_window_registry_step};
pub use ladders_plugin_app_impl::{MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE, MAINTENANCE_DOCUMENT_DISPLACED_STAGE, MAINTENANCE_STAGES};
pub use plugin_app_impl::*;
#[path = "🏪️stores/📥️staging/🧱️components/🦀️.rs"]
mod stores_components;
#[path = "📜️history/📥️staging/🧱️components/🦀️.rs"]
mod history_components;
#[path = "🪆️children/📥️staging/🧱️components/🦀️.rs"]
mod children_components;
#[path = "👷️jobs/📥️staging/🧱️components/🦀️.rs"]
mod jobs_components;
#[path = "📄️docs/📥️staging/🧱️components/🦀️.rs"]
mod docs_components;
#[path = "../🪜️maintenance-ladder/📥️staging/🧱️components/🦀️.rs"]
mod ladders_components;
pub(crate) use stores_components::{ConstructorState, InstanceStores, PeerRoster, ActionIngress, LocalInteraction};
pub(crate) use history_components::{CommandHistory, HostEvents, Transactions};
pub(crate) use children_components::{ChildComposition};
pub(crate) use jobs_components::{PendingReserved, ToolOperations};
pub(crate) use docs_components::{MediaTransfers, EnvelopeJobs};
pub(crate) use ladders_components::{MaintenanceCursors, CloseCursors};
#[path = "📥️staging/🧬️composer-api/🦀️.rs"]
mod app_composer_api;
//#endregion 📥️StagedInstanceMounts
