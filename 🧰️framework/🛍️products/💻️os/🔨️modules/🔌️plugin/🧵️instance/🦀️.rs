//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework_plugin_reactor_app::LAST_MAINTENANCE_STAGE;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
#[cfg(test)]
use semio_framework_plugin_job::reserved_jobs::FrameworkCutJob;
use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_SLOTS;
use semio_framework_plugin_mounted_owner::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use semio_framework_plugin_window_config::{WindowConfigOwnerRegistry, WindowConfigPack};
use semio_framework_plugin_window_transient::window_transient::{WindowTransientOwnerRegistry, WindowTransientSnapshot};
use semio_framework_value::DslValue;
use protocol::OpText;
use semio_framework::kernel::{FixedCommandPage, UiDirtyScope};
use semio_framework::manifest::CommandAddress as ManifestCommandAddress;
use semio_framework::kernel::{ActorId, AppEvent, Effect, HistoryPatch, InvocationResult};
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin, MediaForm, ViewModel};
use semio_framework_tool_run::ToolRunTraceCursor;
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
use semio_framework_ui_runtime::ComponentTree;
use std::collections::{BTreeMap, HashMap};
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ConfigStore, create_config_envelope, MemberFactory, Mutation, NoMembers, ReferenceSpaceMember};
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::{ArtifactCommand, ArtifactEnvelope};
use store::os_store::composition_coordinator::CompositionCoordinator;
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
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework::AppRole;
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_document_media::copy_media_text_parts;
use semio_framework_plugin_decl_app::media_projection_error;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_BUSY_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_CHANGED_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_FAILED_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_HISTORY_INVALID_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_INCOMPLETE_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_MEMBER_INVALID_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_TOO_LARGE_CODE;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE;
use semio_framework_plugin_decl_app::EphemeralSnapshot;
use semio_framework_plugin_document_media::MediaArtifact;
use semio_framework_plugin_decl_app::MediaArtifactDescriptor;
use semio_framework_plugin_decl_app::MediaArtifactError;
use semio_framework_plugin_decl_app::MediaConsumption;
use semio_framework_plugin_decl_app::NATURAL_FILE_PORT;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::TransactionPrepareOutcome;
use semio_framework_plugin_decl_app::document_load_fault;
use semio_framework_plugin_decl_app::natural_file_input_media;
use semio_framework_plugin_window_kits::validate_ui_dispatch_classification;
use semio_framework_plugin_decl_app::whole_document_archive;
use semio_framework_plugin_document_media::whole_document_media;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_views::ArtifactView;
use semio_framework_plugin_decl_views::ConfigView;
use semio_framework_plugin_decl_views::InteractionView;
use semio_framework_plugin_window_transient::window_transient::TransientView;
use semio_framework_plugin_decl_registry::artifact_registration::PluginRuntimeRegistry;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::ChildMemberRegistrationError;
use semio_framework_plugin_child::ChildMemberRetirement;
use semio_framework_plugin_child::OwnedDocumentMemberIngress;
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_framework_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoad;
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoadState;
use semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle;
use semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll;
use semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind;
use semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase;
use semio_framework_plugin_decl_media::ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES;
use semio_framework_plugin_decl_media::ArtifactDownloadOutput;
use semio_framework_plugin_decl_media::ArtifactMediaExportHandle;
use semio_framework_plugin_decl_media::ArtifactMediaExportPoll;
use semio_framework_plugin_peer::PeerRosterPublication;
use semio_framework_plugin_peer::PresenceRosterAdmission;
use semio_framework_plugin_peer::PresenceRosterOutcome;
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness;
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job;
use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_composition_publication::ComposedGestureResult;
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use semio_framework_plugin_operation::ToolCancellationHandle;
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicContract;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_plugin_job::EmitWire;
use semio_framework_plugin_job::PendingTransaction;
/// 🪝️ The instance extensions every mounted instance consults, in route, refusal and maintenance order (see [`InstanceExtension`]).
pub type InstanceExtensions = (semio_framework_plugin_tool_run_turns::ToolRunExtension, (semio_framework_plugin_time_travel_turns::TimeTravelExtension, ()));
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_plugin_job::TransactionProposalDraft;
use semio_framework_plugin_job::TypedOperationCompletion;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework_plugin_job::TypedOperationResultPage;
use semio_framework_plugin_job::TypedOperationResultToken;
use semio_framework_plugin_job::TypedOperationUiProgress;
use semio_framework_plugin_job::lifted_full_operation_job::TypedCommandFullOperationJobFactory;
use semio_framework_plugin_window_kits::merge_ui_values;
use semio_framework_plugin_window_transient::built_to_component_tree;
#[cfg(test)]
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job_counted;


pub struct VcsArtifactApp<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory = NoMembers> {
    pub app: A,
    pub mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub original_controller_close:Option<semio_framework_plugin_instance_initialization::services::OriginalControllerClose<A>>,
    /// 📂️ Store-constructor state: the receipts, units and lane of the four original catalogue constructors.
    pub constructor: ConstructorState<A>,
    /// 📒️ The stores of the instance: document, config, draft, presence, transient, interaction and window owner registries with their one-item publication factories.
    pub stores: InstanceStores<A>,
    /// 📀️ The peer roster: this actor's colour, the adopted peer presence and the roster publication admission state.
    pub roster: PeerRoster<A>,
    /// 🪙️ Action ingress: the parser slots that retain unused original arguments and faults until their paid close.
    pub ingress: ActionIngress,
    pub original_typing:semio_framework_plugin_instance_typed_ops::original_typing::OriginalTypingDispatch<A>,
    pub original_emission:semio_framework_plugin_job::original_emission::OriginalEmissionSource<A>,
    pub original_typed_command:semio_framework_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>,
    pub original_typed_tag:std::mem::ManuallyDrop<Option<semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag>>,
    pub original_typed_tag_close:semio_framework_value::retained_clone::RetainedCloneClose,
    /// 🗜️ Local interaction: the framework-owned live query, topology authority, hover and snapshot read returns.
    pub interaction: LocalInteraction<A>,
    /// ⛓️ Command history: the session command log, the cached history view, backfill marks and the supersede ledger.
    pub history: CommandHistory<A>,
    /// ✉️ Host events: the utilities, window roster and typed operations a host event dispatch reached.
    pub host_events: HostEvents,
    /// 🎴️ Transactions: the single pending foreign transaction and its proposal.
    pub txn: Transactions<A>,
    /// 🧿️ Child composition: the live owned children, their private groups, content roots and retirement registries.
    pub child_composition: ChildComposition<M>,
    /// 🪬️ Pending reserved work: the reserved commits, render trees, configuration commands and their failure custody.
    pub pending: PendingReserved<A>,
    /// 🔩️ Tool operations: typed operation outboxes, tool jobs and registrations, cancellations and latest-wins commands.
    pub tools: ToolOperations<A>,
    /// 🧴️ Media transfers: the media exports, downloads, closures and snapshot retirements of the instance.
    pub media: MediaTransfers<A>,
    /// 🪞️ Envelope jobs: envelope ingress and decode, store replacement and document archive loads.
    pub ingest: EnvelopeJobs<A, M>,
    /// 🪠️ Maintenance cursors: the rotation positions of the live-maintenance ladder.
    pub maintenance: MaintenanceCursors,
    /// 🧽️ Close cursors: the positions and drained flags of the close ladder.
    pub close: CloseCursors<A, M>,
    /// ⏯️ The ephemeral local-only tool run ledger (contract §3.3): provisional document ops folded
    /// into an overlay the renderer reads, never touching `store` until the one-edit finalize.
    pub tool_runs: ToolRunLedger<A>,
    /// ⏪️ The ephemeral local-only history-edit session (design §7 of ticket
    /// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): drafts previewed and replayed beside the store, never touching it
    /// until the one finalize commit.
    pub time_travel: TimeTravelLedger<A>,
    /// 🎚️ Every window's open continuous-control press (design §13.1): provisional leaves overlaid on every render,
    /// published as ONE transaction on the release, zero trace on any host abort.
    pub tool_machines: ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🔍️ Read-only inspection of this instance's tool run ledger for app-level tests.
    #[cfg(feature = "artifact-app-testing")]
    pub fn tool_run_ledger(&self) -> &ToolRunLedger<A> {
        &self.tool_runs
    }
}

pub use crate::plugin_app_impl::{FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS, settle_framework_reserved_admission, settle_framework_reserved_commit};
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use crate::plugin_app_impl::drive_framework_reserved_spawn_like_host;
pub use semio_framework_ui_contract::*;
impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory> Drop for VcsArtifactApp<A, M> {
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




















































use semio_framework_plugin_instance_render as render;









#[path = "📥️staging/📨️dispatch/🦀️.rs"]
mod app_dispatch;

#[path = "📥️staging/🔒️close-ladder/🦀️.rs"]
mod app_close_ladder;

#[path = "📥️staging/🚪️ingress/🦀️.rs"]
mod app_ingress;

#[path = "📥️staging/🪜️maintenance-ladder/🦀️.rs"]
mod app_maintenance_ladder;

#[path = "🔌️plugin-app-impl/🦀️.rs"]
#[doc(hidden)]
pub mod plugin_app_impl;






use semio_framework_plugin_instance_dispatch::dispatch_emit;


















#[path = "📥️staging/🏗️ctx-builders/🦀️.rs"]
mod app_ctx_builders;
pub use semio_framework_plugin_instance_extension::{ExtensionCtx, ExtensionRef, InstanceExtension};
pub use semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema;
pub use semio_framework_plugin_instance_children::children_plugin_app_impl::MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY;
pub use semio_framework_plugin_instance_docs::docs_instance_root::{artifact_handle_of, typed_operation_document_is_fresh};
pub use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
pub use semio_framework_plugin_instance_docs::docs_plugin_app_impl::history_changed_event;
use semio_framework_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;
pub use semio_framework_plugin_instance_ladders::ladders_document_replacement::{retire_document_window_registry_demands, retire_document_window_registry_step};
pub use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::{MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE, MAINTENANCE_DOCUMENT_DISPLACED_STAGE, MAINTENANCE_STAGES};
pub use semio_framework_plugin_instance_stores::stores_components::{ConstructorState, InstanceStores, PeerRoster, ActionIngress, LocalInteraction};
pub use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
pub use semio_framework_plugin_instance_children::children_components::ChildComposition;
pub use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
pub use semio_framework_plugin_instance_docs::docs_components::{MediaTransfers, EnvelopeJobs};
pub use semio_framework_plugin_instance_ladders::ladders_components::{MaintenanceCursors, CloseCursors};
#[path = "📥️staging/🧬️composer-api/🦀️.rs"]
mod app_composer_api;
pub use semio_framework_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
pub use semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx;
pub use semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx;
pub use semio_framework_plugin_time_travel_turns::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
pub use semio_framework_plugin_tool_run_turns::tool_run_bundle::{ToolRunCtx, ToolRunRef};
pub use semio_framework_plugin_instance_commands::command_bundle::CommandCtx;
pub use semio_framework_plugin_instance_ladders::ladder_bundle::{LadderCtx, LadderRef};
pub use semio_framework_plugin_instance_render::render_history_view::history_command;
//#endregion 📥️StagedInstanceMounts
