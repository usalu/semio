//! 🔌️ Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM plugins.

pub use semio_framework_os_kernel::io;

#[cfg(any(test, feature = "artifact-app-testing"))]
pub use semio_framework_plugin_testing_laws::with_authoring_identity;
//#region 📣️FacadeMacroMounts
#[macro_use]
#[path = "📣️macros/🌱️wp1-window/🦀️.rs"]
mod macros_wp1_window;
#[macro_use]
#[path = "📣️macros/📜️wp2-declaration/🦀️.rs"]
mod macros_wp2_declaration;
#[macro_use]
#[path = "📣️macros/⚛️wp4-reactor/🦀️.rs"]
mod macros_wp4_reactor;
//#endregion 📣️FacadeMacroMounts
pub use semio_framework_plugin_mounted_owner::{MountedOwnerIdentityV1, MountedOwnerJobReceiptV1, MountedOwnerPhaseV1, MountedOwnerPolicyV1, MountedOwnerTurnV1};

#[cfg(test)] use semio_framework_ui_locale::{Label, LabelText};

/// 🗣️ `#[derive(dsl::DslRecord)]` expands to `::dsl::…` paths, so the crate root must bind that name
/// — the same `extern crate semio_framework_os_kernel as dsl;` every technology crate declares.
/// Without it the plugin-owned `world-3d` records (`WorldSunConfig`, `WorldProjectionConfig`) could
/// not implement `DslField`, and every artifact embedding one in a `WindowConfigOwner::State` would
/// have to carry a hand-mirrored copy instead.
extern crate semio_framework_os_kernel as dsl;

#[doc(hidden)]
pub use semio_framework_async as __async;

//#region 🧮️HeapWitness
/// 🧮️ Weighs what ONE reactor turn RETAINS, in this crate's own test binary only. The wasm harness
/// (`🖥️host/🧪️tests/🔬️poll-turn-memory`) measured 4 437 B of guest linear memory per `poll` turn
/// against a live instance — app-independent and render-independent — and the guest has one fixed
/// linear memory with no process to restart. `semio_framework_trace::HeapWitness` is the shared
/// instrument (two relaxed atomics per allocation, TEST BINARIES ONLY) the puzzle-3d and
/// generation3d editors already install; the turn-retention law below reads it.
#[cfg(test)]
#[global_allocator]
static PLUGIN_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
//#endregion 🧮️HeapWitness

#[doc(hidden)]
#[path = "🧪️tests/🧬️generated-test-contracts/🦀️.rs"]
mod generated_test_contracts;

#[doc(hidden)]
pub use semio_framework_plugin_abi::component_persistent_local;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub mod owned_abi {
    pub use semio_framework_plugin_abi::owned_abi::*;

    /// 📬️ Releases one guest allocation after the reactor-context receipt registries had their chance to claim it; the plain release is abi's.
    pub unsafe fn deallocate(pointer: u32, length: u32) {
        if crate::reactor::original_actor_context::exports::fixed_receipt_pointer(pointer,length)||crate::reactor::original_actor_context::exports::release_owned_job_receipt(pointer,length) { return; }
        unsafe { semio_framework_plugin_abi::owned_abi::deallocate(pointer, length) }
    }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use wasip2::component_export_anchor;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn component_export_anchor() {}

pub use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleFailure,PluginLifecyclePrimary};
//#region 🏛️OwnershipFacet
pub use semio_framework_job::StepContext;
// 🧭️ Mount hubs: the module mounts that used to follow live in one file per work package of PLAN-PLUGIN-CRATES.md,
// spliced into this crate root, so every `crate::<module>` path is unchanged.
include!("🪝️mounts/🌱️wp1-foundation/🦀️.rs");
include!("🪝️mounts/📜️wp2-declaration/🦀️.rs");
include!("🪝️mounts/👥️wp3-owners/🦀️.rs");
include!("🪝️mounts/⚛️wp4-reactor/🦀️.rs");
include!("🪝️mounts/⏯️wp5-features/🦀️.rs");
include!("🪝️mounts/🧱️wp6-instance-tier1/🦀️.rs");
include!("🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs");
//#region 🏛️OwnershipFacet
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use semio_framework_plugin_abi::wasip2;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub use semio_framework_plugin_abi::__semio_export_component_guest;
//#endregion 🏛️OwnershipFacet
//#region 🪟️WindowFacet
pub(crate) use semio_framework_plugin_window_transient::window_transient;
#[doc(hidden)]
pub use semio_framework_plugin_window_transient::window_transient_transfer;
//#endregion 🪟️WindowFacet
//#region 📜️DeclarationFacet
//#endregion 📜️DeclarationFacet
//#region ⚛️ReactorFacet
#[path = "⚛️reactor/🏃️runtime/🦀️.rs"]
pub mod plugin_runtime;
//#endregion ⚛️ReactorFacet

//#endregion 🏛️OwnershipFacet
//#region 👷️JobsFacet
#[cfg(feature="reserved-producer-testing")]
pub use tool_contract::original_reserved_producer_laws;
//#endregion 👷️JobsFacet
//#region 🧵️InstanceFacet
//#endregion 🧵️InstanceFacet

pub mod app {
    //! 🧩️ The crate's public app vocabulary: declarative app builder, plugin traits and runtime types, composed from the facet modules that own them.
    pub use crate::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
    pub use super::child_emit_preparation::{ChildEmitPreparation,ChildEmitPreparationStep,OwnedChildEmit};
    pub use semio_framework_plugin_ownership::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, bounded_transient_store_disposer, transient_store_disposer};
    pub use super::window_config::{
        WindowConfigNativePairReceiving,window_config_native_pair_demands,admit_window_config_native_pair,bounded_window_config_preparation_source_birth, bounded_window_config_preparation_factory, bounded_window_config_store_disposer,bounded_window_config_store_disposer_birth, bounded_window_config_store_owners, RejectedWindowConfigEmission, WindowConfigAuthorityIdentity, WindowConfigBorrow, WindowConfigMutation, WindowConfigOwner, WindowConfigOwnerRegistry, WindowConfigPack,
        WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep, WindowConfigApplyCursor, WindowConfigApplyEdit, WindowConfigApplyMutation, WindowConfigReplaceCursor, WindowConfigReplaceEdit, WindowConfigReplaceMutation, WindowConfigSnapshot,
    };
    pub use super::window_transient::{TransientView, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientOwnerRegistry, WindowTransientSnapshot};
    /// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
    pub use semio_framework::AppDefinition;
    pub use semio_framework_ui_contract::BuiltNode;
    pub use semio_framework_ui_runtime::{ComponentTree, TreeNode};
    /// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
    /// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
    /// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
    /// `semio_framework_os`'s artifact catalog registry share one definition.
    pub use semio_framework::{ArtifactKindSpec, OsMediaCapability};
    /// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
    /// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
    /// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
    /// The legacy format enum was retired in ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
    /// Binary` now carries a plain format kind id string.
    pub use semio_framework::MediaWireFormat;
    /// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
    /// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
    /// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
    pub use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
    /// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
    /// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
    /// without a direct `semio-framework-core` dependency.
    pub use semio_framework::{MediaClass, MediaType};
    /// ⚠️ IO error alias from the framework media stack.
    pub use semio_framework::mesh_io::IoError;
    /// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
    /// 2). Defined in `semio_framework` so plugins and the OS product
    /// share one definition without an inverted dependency; re-exported here verbatim.
    pub use {semio_framework_os_kernel::io::io_compose_via,semio_framework_os_kernel::io::dialects_for,semio_framework_os_kernel::io::io_dispatch,semio_framework_os_kernel::io::io_keys_for,semio_framework_os_kernel::io::resolve,semio_framework_os_kernel::io::list_composer_entries,semio_framework_os_kernel::io::register_composer_entries,semio_framework_os_kernel::io::register_subset_validator,semio_framework_os_kernel::io::set_io_fallback_dispatcher,semio_framework_os_kernel::io::subset_validator_entry_of,semio_framework_os_kernel::io::wire_artifact_compose,semio_framework_os_kernel::io::wire_decode_composed_artifact,semio_framework_os_kernel::io::wire_list_composer_entries,semio_framework_os_kernel::io::Analysis,semio_framework_os_kernel::io::AnalyzeSource,semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::AsyncComposeFn,semio_framework_os_kernel::io::ComposeError,semio_framework_os_kernel::io::ComposeFuture,semio_framework_os_kernel::io::ComposeSource,semio_framework_os_kernel::io::ComposedArtifact,semio_framework_os_kernel::io::ComposerEntry,semio_framework_os_kernel::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_os_kernel::io::ErasedComposeSource,semio_framework_os_kernel::io::IoDirection,semio_framework_os_kernel::io::IoFallback,semio_framework_os_kernel::io::IoFallbackDispatcher,semio_framework_os_kernel::io::IoKey,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::IoResolveError,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_os_kernel::io::SubsetValidator,semio_framework_os_kernel::io::SubsetValidatorEntry,semio_framework_os_kernel::io::WireComposeSource,semio_framework_os_kernel::io::WireComposedArtifact};
    pub use super::inference_gateway::wire_artifact_infer;
    /// 🧾️ The body-wide node budget and its fixed-node reserve live ONCE, in the UI contract, because
    /// the React host spends the same number from the other side of the wire
    /// (`🌳️Tree/🟦️.tsx`'s `capTreeWindowRequests`). See
    /// [`semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET`] for the cost model both sides
    /// implement; [`TreeWindows`] is this side of it.
    pub use semio_framework_ui_contract::{TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_FIXED_NODE_HEADROOM, TREE_WINDOW_PATH_SEPARATOR};
    pub use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
    pub use crate::local_interaction::mutations::{InteractionConfigMutation, SetInteractionState};
    /// 🙋️ Explicit actor identity for standalone local births and fixtures.
    pub use protocol::LOCAL_ACTOR_ID;
    pub use super::completion_fault::ArtifactCompletionFault;
    pub use store::SnapshotReadProjection as ArtifactContextRoot;
    pub use super::owned_context::ArtifactOwnedContextHandle;
    pub use store::ArtifactStoreInitializationAuthority;
    pub use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
    pub use super::builder::{NeedsLabel, NeedsVersion, PluginBuilder, Ready};
    pub use semio_framework_plugin_ownership::bounded_ownership::*;
    pub use semio_framework_plugin_ownership::sdk_fault::*;
    pub use super::document_kit::*;
    pub use super::image_kit::*;
    pub use super::media_kit::*;
    pub use super::mesh_kit::*;
    pub use super::table_kit::*;
    pub use super::text_kit::*;
    pub use super::tree_kit::*;
    pub use super::tree_kit::tree_item;
    pub use super::window_kits::*;
    pub use super::window_surface::*;
    pub use super::app_builder::*;
    pub use super::app_contract::*;
    pub use super::app_emission::*;
    pub use super::app_views::*;
    pub use super::app_views::PeerMark;
    pub use super::artifact_declaration::*;
    pub use super::artifact_identity::*;
    pub use super::artifact_inference::*;
    pub use super::artifact_io::*;
    pub use super::artifact_registration::*;
    pub use super::surface_contract::*;
    pub use super::child_view::*;
    pub use super::gesture_contract::*;
    pub use super::history_view::*;
    pub use super::media_contract::*;
    pub use super::tool_contract::*;
    pub use super::tool_run_contract::*;
    pub use super::child_content::*;
    pub use super::document_ingress::*;
    pub use super::peer_roster::*;
    pub use semio_framework_plugin_ownership::store_lifecycle::*;
    pub use super::store_publication::*;
    pub use super::command_history::*;
    pub use super::reserved_jobs::*;
    pub use super::tool_cancellation::*;
    pub use super::tool_jobs::*;
    pub use super::artifact_instance::*;
    pub use super::plugin_app::*;
    pub use super::plugin_program::*;
    pub use super::declarations;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub use super::app_testing::*;
}

/// 🧩️ Cross-crate closure vocabulary required by `dyn_enum_close!` for `PluginApp`.
pub mod plugin_app_close_prelude {
    pub use crate::app::*;
    pub use crate::app::{tree_item, PeerMark};
    pub use semio_framework::kernel::ActionId;
    pub use semio_framework::kernel::*;
    pub use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandInvocation as ManifestCommandInvocation};
    pub use semio_framework::*;
    pub use semio_framework_os_kernel as protocol;
    pub use semio_framework_os_kernel as store;
    pub use semio_framework_plugin_decl_app;
    pub use semio_framework_plugin_decl_views;
    pub use semio_framework_plugin_ownership;
    pub use semio_framework_tool_run::ToolRunTraceCursor;
    pub use semio_framework_ui_contract::tree_item as tree_item_builder;
    pub use semio_framework_ui_contract::*;
    pub use semio_framework_ui_runtime::{ComponentTree, TreeNode};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
    pub use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, WindowEngagement, WindowMeasure};
}

#[cfg(any(test, feature = "artifact-app-testing"))]
pub use app::artifact_app_laws;
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use app::authoring_law;
pub use app::time_travel::{history_mutation_entry, HistoryMutationPage, HistoryMutationPages, SupersedeLedger, SupersedeRecord, SupersedeRole, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelEditor, TimeTravelEditorPanel, TimeTravelLedger, TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};
pub use app::tool_run::{is_tool_run_action_id, ToolRunActionOutcome, ToolRunDriver, ToolRunLedger, ToolRunTickReceipt};
pub use app::ActionFactory;
pub use semio_framework::{ErasedToolJob, IconName, Locale, Terminology};
pub use semio_framework_ui_locale::LocalizedLabel;
/// 🔀️ `dyn_enum_close!` for `PluginApp` resolves its captured dispatch macro through this root export.
#[doc(hidden)]
pub use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
pub use app::{FRAMEWORK_TOOL_RUN_BODY_KEY, OriginalToolRunJobPortRetirement, ToolRunJob, ToolRunJobPort, ToolRunJobPurpose, ToolRunJobRequest, ToolRunRetargetableJob, ToolRunTraceKeys, ToolRunView, ToolRunPollView, is_time_travel_action_id};
pub use app::{AsyncTaskSource,AsyncTaskRun};
pub use app::{activation_target, resolve_labels, row_action, row_target, selection_ids, tree_group, tree_item, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, LabelAxes};
pub use app::{
    artifact_app_apply_ops,
    artifact_app_genesis_pair,
    artifact_app_replay_envelopes,
    artifact_inference_service,
    artifact_pair_snapshot,
    bounded_config_store_disposer,
    bounded_config_store_one_item_preparation_factory,
    bounded_config_store_owners,
    bounded_document_store_disposer,
    bounded_document_store_initialization_job,
    bounded_document_store_owners,
    bounded_transient_preparation_factory,
    bounded_transient_root_retirement_factory,
    bounded_transient_store_disposer,
    WindowConfigNativePairReceiving,window_config_native_pair_demands,admit_window_config_native_pair,bounded_window_config_preparation_source_birth,
    bounded_window_config_preparation_factory,
    bounded_window_config_store_disposer,bounded_window_config_store_disposer_birth,
    bounded_window_config_store_owners,
    built_text_node,
    built_text_to_component_tree,
    built_to_component_tree,
    built_to_tree,
    composer_entry_of,
    delete_selection,
    deserializer_entry_of,
    history_alternative_views,
    infer_artifact,
    inference_child,
    inference_child_dependency,
    inference_child_dependency_parts,
    list_artifact_inference_services,
    node_graph_delete_selection_spec,
    nothing_selected,
    paged_text_carrier,
    publish_document_store_candidate_if_authoritative,
    register_artifact_inference_service,

    scene_surface,
    section_component_tree,
    selection_count_phrase,
    selection_domains_from_surface,
    serializer_entry_of,
    transient_store_disposer,
    wire_list_artifact_inference_services,
    ActionMeta,
    AlternativeFact,
    AlternativeView,
    App,
    AppActionRegistry,
    AppBuilder,
    AppInstance,
    AppOperationContext,
    AppRenderOperationContext,
    MountedSnapshotReadPreparation,
    ArtifactAnalysis,
    ArtifactAnalyzer,
    ArtifactApp,
    ArtifactBoundedFirstStepProof,
    ArtifactBuilder,
    SnapshotBuilder,
    ArtifactCapability,
    NaturalFileCodec,
    NaturalFileDecodeCursor,
    NaturalFileDecodeOwner,
    NaturalFileDecodeProgress,
    NaturalFileDecodeStep,
    ArtifactCapabilityKind,
    ArtifactCapabilityRow,
    ArtifactChildren,
    ArtifactComposer,
    ArtifactComposition,
    ArtifactDeclaration,
    ArtifactDecomposer,
    ArtifactDefinition,
    ArtifactDefinitionError,
    ArtifactDefinitionRegistry,
    ArtifactDeserializer,
    ArtifactDocumentPayload,
    OriginalArtifactDocumentReceiving,
    ArtifactDocumentStoreDisposer,
    ArtifactDownloadOutput,
    ArtifactEditor,
    ArtifactEnvelopeDecodeOperationHandle,
    ArtifactEnvelopeDecodeOperationPoll,
    ArtifactExecutableIdentity,
    ArtifactIdentity,
    ArtifactIdentityClaim,
    ArtifactIdentityNamespace,
    ArtifactInference,
    ArtifactInferenceDemands,
    ArtifactInferenceCommitBinding,
    ArtifactInferenceDocumentBinding,
    ArtifactInferenceExecution,
    ArtifactInferenceExecutionStep,
    ArtifactInferenceStepContext,
    ArtifactInferenceExecutionError,
    ArtifactInferenceExecutionRequest,
    ArtifactInferencePayloadContract,
    ArtifactInferenceRegistrationError,
    ArtifactInferenceService,
    ArtifactInferenceServiceKey,
    ArtifactInferenceServiceMetadata,
    ArtifactInferenceServiceRegistry,
    ArtifactInstanceOperationOwner,
    ArtifactInstanceOperationOwnerHandle,
    ArtifactInferenceSessionAccess,
    ArtifactInstanceOperationOwnerAliasRetirement,
    ArtifactKindSpec,
    ArtifactLocale,
    ArtifactLocalization,
    ArtifactMediaExportCompletion,
    ArtifactMediaExportCredit,
    ArtifactMediaExportHandle,
    ArtifactMediaExportHandleLoan,
    ArtifactMediaExportJobRequest,
    ArtifactMediaExportPoll,
    ArtifactMediaExportPublishedStatus,
    ArtifactMediaExportResult,
    ArtifactMime,
    ArtifactOutputChunks,
    ArtifactOwnedDisposer,
    ArtifactOwnedToolJobFactory,
    ArtifactOwnedToolJobRequest,
    ArtifactOriginalToolCommand,
    ArtifactReservedJob,
    ArtifactReservedToolInput,
    ArtifactReservedToolJob,
    ArtifactReservedToolJobRequest,
    ArtifactRuntimeCapabilityRequirement,
    ArtifactSerializer,
    ArtifactSnapshotCloseLease,
    ArtifactStoreInitializationAuthority,
    ArtifactStoreInitializationJob,
    ArtifactToolCompletion,
    ArtifactToolCompletionValue,
    ArtifactToolCompletionRejection,
    ArtifactDownloadCompletionRejection,
    ArtifactBoundedToolFault,
    ArtifactCompletionFault,
    ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract,
    ArtifactToolPublicationLane,
    ArtifactView,
    ArtifactPendingView,
    ArtifactViewer,
    AsyncComposeFn,
    BuiltNode,
    // 🧸️👥️🫧️ Composition child-read seam plus the two ephemeral state lanes.
    ChildContentView,
    ChildContentReadRetirement,
    ComponentTree,
    ComposeFuture,
    Confidence,
    ConfigView,
    DecomposeSource,
    Decomposition,
    DerivedArtifactAnalyzer,
    DerivedArtifactBuilder,
    DerivedArtifactComposer,
    DerivedArtifactParts,
    DerivedArtifactSpec,
    Dialect,
    DraftView,
    Editor,
    EditorApp,
    Emit,
    EphemeralEmit,
    ExampleSource,
    ExampleSourceBody,
    ExtensionInvocation,
    FlowExtensionContribution,
    FlowExtensionContributionKind,
    FlowExtensionDeclaration,
    FlowExtensionDescriptor,
    FlowExtensionExecutableIdentity,
    FlowExtensionManifest,
    HistoryView,
    HostEvent,
    HostMediaExecutableIdentity,
    HostMediaHandlerDeclaration,
    HostMediaHandlerDescriptor,
    HostMediaHandlerKind,
    HostMediaRuntimeError,
    InteractionView,
    InteractionWrite,
    KeybindingSpec,
    MediaClass,
    MediaType,
    Menu,
    MeshDocumentImporter,
    MeshImportRequest,
    MeshImportResult,
    MeshView,
    MeshWindowKit,
    ModeSpec,
    NoChildren,
    NoConfig,
    NoConfigMutation,
    NoDraft,
    NoDraftMutation,
    NoPresence,
    NoPresenceMutation,
    NoTransient,
    NoTransientMutation,
    NodeGraphDeleteDispatch,
    OsMediaCapability,
    OwnedDocumentMemberIngress,
    PanelTabSpec,
    PanelTreeBuilder,
    Plugin,
    PluginApp,
    PluginAssemblyError,
    OriginalRenderFailure,
    OriginalRenderResult,
    NeedsLabel,
    NeedsVersion,
    PluginBuilder,
    Ready,
    PluginLifecycleStep,
    PluginProgram,
    PresenceView,
    SelectionKind,
    StandardId,
    SubsetId,
    ToolCommandCatalog,
    TransientView,
    TreeNode,
    TwoDSvgDocumentRenderer,
    TwoDSvgExportRequest,
    TwoDSvgExportResult,
    UiAssemblyResult,
    VcsArtifactApp,
    OriginalVcsApp,
    ViewEmit,
    Viewer,
    ViewerApp,
    WindowConfigApplyCursor,
    WindowConfigApplyEdit,
    WindowConfigApplyMutation,
    WindowConfigMutation,
    WindowConfigOwner,
    WindowConfigOwnerRegistry,
    WindowConfigBorrow,
    WindowConfigAuthorityIdentity,
    WindowConfigPack,
    WindowConfigPackLoad,
    WindowConfigPackLoadDiagnostic,
    WindowConfigPackLoadPhase,
    WindowConfigPackLoadProgress,
    WindowConfigPackLoadStep,
    WindowConfigSnapshot,
    WindowKindSpec,
    WindowKit,
    WindowTransientMutation,
    WindowTransientOwner,
    WindowTransientOwnerBundle,
    WindowTransientOwnerRegistry,
    WindowTransientSnapshot,
    WireArtifactInferenceBudget,
    WireArtifactInferenceCacheMode,
    WireArtifactInferenceDiagnostic,
    WireArtifactInferenceMetadata,
    WireArtifactInferenceProvenance,
    WireArtifactInferenceRequest,
    WireArtifactInferenceResult,
    WireInferenceArtifactBinding,
    WireInferenceCommitBinding,
    WireInferencePayloadContract,
    ARTIFACT_INFERENCE_WIRE_VERSION,
    CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES,
    MAINTENANCE_STAGES,
    NATURAL_FILE_PORT,
};
pub use app::{
    take_arena_unbuilt_rows, tree_window_indexed_item, tree_window_indexed_section, tree_window_item, tree_window_section, tree_window_section_or_placeholder, ui_node_list, TreeSlice, TreeWindows, TREE_WINDOW_BODY_NODE_BUDGET,
    TREE_WINDOW_DEFAULT_ROWS, TREE_WINDOW_FIXED_NODE_HEADROOM, TREE_WINDOW_PATH_SEPARATOR,
};
pub use engagement::{engagement_token_matches, strip_engagement_prefix};
pub use semio_framework_tool_machine::{TypingCommit, TypingFold, TYPING_BUFFER_ARG, TYPING_COMMIT_ARG, TYPING_IDLE_MS};
// 🧬️ A2 (design-abi.md §4): `host_port`'s re-export is deleted along with the module (see the
// "Replace, never wrap" note above `pub mod engagement`). `host::now_ms` replaces `host_now_ms` —
// `✏️s/🔌️plugins/🪐️space` is the one measured external caller (design-abi.md §0) and will be
// repointed during its own W3 migration wave, not here (SDK crate stays frozen during W3 per
// `important.md`'s sequencing constraints).
pub use plugin_runtime::{
    extension_activate, extension_cancel_close, extension_close_step, extension_deactivate, extension_invoke, extension_manifest, extension_retirement_demands, extension_resume_close, extension_terminal_is_empty, install_extension_bundle,
    install_plugin_bundle, install_plugin_bundle_result, plugin_artifact_apply_ops, plugin_artifact_genesis, plugin_artifact_pack_schema_hash, plugin_artifact_print_mirror, plugin_artifact_replay_envelopes, plugin_attach_backbone,
    plugin_cancel_media_export, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_poll_media_export, plugin_retain_media_export_result, plugin_submit_media_export,
    plugin_take_media_export_chunk, plugin_take_segmented_download_chunk, ExtensionBundle, ExtensionManifest, ExtensionResourceOwner, ExtensionInvokeStep,
};
pub use semio_framework::*;
pub use semio_framework::{MediaForm, MediaPortDirection, MediaPortSpec};
pub use semio_framework_ui_contract::{
    ActionBinding, ActionId, Buildable, Component, HasBase, HasChildren, Label as UiLabel, RowAction, RowActionPlacement, RowActionRefusal, RowTarget, Trigger, UiFixedList, UiFixedMap, UiListBuilder, UiMapBuilder, UiPublicationRevision, UiText, UiValue,
};
pub use world3d_host::{
    apply_world3d_projection_action, apply_world3d_sun_action, default_world3d_selection, merge_world_selection_ids, mesh_kind_from_json, scene_lane_hash, world3d_camera_projection_json, world3d_default_camera, world3d_environment_json,
    world3d_mesh_id_from_url, world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_projection_action_moves_pose, world3d_projection_measures, world3d_projection_pose,
    world3d_projection_spec_json, world3d_scene, world3d_selection_json, world3d_selection_json_with_granularity, world3d_sun_measures, SceneLanePayload, SceneLaneRef, SelectionSet, World3dScene, World3dSceneLane, WorldProjectionConfig,
    World3dAngle, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dDimension, World3dHighlight, World3dLeader, World3dLegendTick, World3dMarker, World3dMarkerShape, World3dModellingOptions, World3dPickGranularity, World3dPickTargets, World3dScalarDomain, World3dScalarField, World3dScalarLegend, World3dScalarRange, World3dSection, World3dSectionCap, World3dSubElementStyle, World3dText, World3dTone, WORLD3D_ANNOTATIONS_MAX, WorldSunConfig, WORLD3D_SCENE_LANE_BODY_KEYS, WORLD3D_SCENE_LANE_FIELDS, WORLD3D_SCENE_LANE_KEY_PREFIX, WORLD3D_SCENE_LANE_NAMES, WORLD3D_SCENE_LANE_OPTIONAL,
};
/// 📐️ The renderer-neutral window, engagement, measure, context-menu and panel-tab vocabulary plugins name through this
/// crate, re-exported from its owner `semio-framework-ui-layout`.
pub use semio_framework_ui_layout::layout::{
    create_default_layout, create_named_layout, create_stack_layout, create_tab_stack_layout, create_window_layout, ActionDescriptor, ContextMenuHit, ContextMenuItemSpec, ContextMenuRequest, ContextMenuSelectionGroup,
    ContextMenuSurfaceTarget, ContextMenuTextContext, MeasureSelectItem, NamedLayout, UiMenuRef, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible,
    WindowEngagementSlot, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, WindowOptions,
    FRAMEWORK_HISTORY_BODY_KEY, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
/// 🧰️ The utility-rail category plugins declare utilities under, from `semio-framework-ui-layout`.
pub use semio_framework_ui_layout::utilities::UtilityCategory;
/// 📜️ The renderer-neutral surface kinds, table cells, tree actions, inspector helpers and world3d scene JSON helpers plugins
/// name through this crate, re-exported from their owner `semio-framework-ui-declaration`.
pub use semio_framework_ui_declaration::ui::{
    text_identifier_occurrences_json, ui_inspector_all_equal, ui_inspector_mixed_number, ui_inspector_mixed_text, world3d_camera_json, world3d_chunking_json, world3d_fit_json, BlockPaletteEntry, SurfaceKind, TableCell,
    UiTreeActionPlacement, UiTreeItemAction, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
/// 🎬️ The scene records plugins publish through this crate (the world3d family is re-exported by `world3d_host` above),
/// from their owner `semio-framework-ui-scene`.
pub use semio_framework_ui_scene::{
    BlockListBlock, BlockListPaletteEntry, BlockListScene, BlockListSelectionTarget, BlockListStep, Board2dScene, Canvas2dFraming, Canvas2dScene, IconRenderScene, InkCanvasInteractionDomain, InkCanvasScene,
    NodeGraphEdgeRecord, NodeGraphHover, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, Paint2dScene, TableScene, TextEditorScene, TextSplice, TextSpliceComposition, TiledMapScene, TEXT_SPLICE_CONTEXT_SCALARS,
};

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle_result($bundle);
    };
}

/// 🧬️ Declares uniform artifact lifecycle types from schema and IO hooks. UNIFIED-COMPOSABLE-
/// ARTIFACT-SYSTEM (C1, Task 5): accepts an optional trailing `children: $ty` field naming a real
/// `ArtifactChildren` impl (`$ty::Snapshot` must equal `<$construction as ArtifactBuilder>::Snapshot`,
/// enforced by `DerivedArtifactSpec::Children`'s own bound) — when present, `$composer`'s `reads()`
/// gains each declared slot's dialect and `compose()` routes matching sources through
/// `$ty::compose_from_children` (see `DerivedArtifactComposer`'s own doc comment for the mechanism;
/// this macro only has to name the type, the behavior lives centrally there). Omitting `children`
/// (every pre-C1 invocation) defaults `type Children` to `NoChildren<Snapshot>` — a leaf artifact
/// needs zero changes to keep compiling.
#[macro_export]
macro_rules! derive_artifact_facets {
    (
        $visibility:vis spec $spec:ident {
            construction: $construction:ty,
            analysis: $analysis:ty,
            composition: $composition:ty
            $(, children: $children:ty)? $(,)?
        }
        builder: $builder:ident,
        analyzer: $analyzer:ident,
        composer: $composer:ident $(,)?
    ) => {
        $visibility struct $spec;

        impl $crate::DerivedArtifactSpec for $spec {
            type Snapshot = <$construction as $crate::ArtifactBuilder>::Snapshot;
            type Mutation = <$construction as $crate::ArtifactBuilder>::Mutation;
            type Diff = <$construction as $crate::ArtifactBuilder>::Diff;
            type Construction = $construction;
            type Analysis = $analysis;
            type Composition = $composition;
            type Children = $crate::derive_artifact_facets!(@children_ty <$construction as $crate::ArtifactBuilder>::Snapshot $(, $children)?);
        }

        #[derive(Clone, Debug, Default)]
        #[allow(dead_code, reason = "grammar-smoke-test invocations never construct this — see comment above")]
        $visibility struct $builder($crate::DerivedArtifactBuilder<$spec>);

        impl $crate::ArtifactBuilder for $builder {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            type Mutation = <$spec as $crate::DerivedArtifactSpec>::Mutation;
            type Diff = <$spec as $crate::DerivedArtifactSpec>::Diff;

            fn empty() -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::empty()) }
            fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_snapshot(snapshot)) }
            fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_text(text)?)) }
            fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_binary(bytes)?)) }
            fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) { let (builder, outcome) = <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::mutate(self.0, mutation); (Self(builder), outcome) }
            fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> { <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::absorb(self.0, diff).map(Self) }
            fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> { <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::build(self.0) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never construct this — see comment above $builder")]
        $visibility struct $analyzer;

        impl $crate::ArtifactAnalyzer for $analyzer {
            type Parts = <$analysis as $crate::ArtifactAnalysis>::Parts;
            const DIALECT: $crate::Dialect = <$analysis as $crate::ArtifactAnalysis>::DIALECT;
            fn sniff(source: &$crate::io::AnalyzeSource<'_>) -> $crate::io::Confidence { <$analysis as $crate::ArtifactAnalysis>::sniff(source) }
            fn analyze(sources: &[$crate::io::AnalyzeSource<'_>]) -> $crate::io::Analysis<Self::Parts> { <$analysis as $crate::ArtifactAnalysis>::analyze(sources) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never call the inherent duplicates — see comment above $builder")]
        impl $analyzer {
            pub fn sniff(source: &$crate::io::AnalyzeSource<'_>) -> $crate::io::Confidence { <Self as $crate::ArtifactAnalyzer>::sniff(source) }
            pub fn analyze(sources: &[$crate::io::AnalyzeSource<'_>]) -> $crate::io::Analysis<<Self as $crate::ArtifactAnalyzer>::Parts> { <Self as $crate::ArtifactAnalyzer>::analyze(sources) }
        }

        $visibility struct $composer;

        impl $crate::ArtifactComposer for $composer {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            const WRITES: $crate::Dialect = <$composition as $crate::ArtifactComposition>::WRITES;
            fn reads() -> &'static [$crate::Dialect] { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::reads() }
            fn compose(sources: &[$crate::io::ComposeSource<'_>]) -> Result<$crate::io::Composition<Self::Snapshot>, $crate::io::ComposeError> { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::compose(sources) }
        }

        #[allow(dead_code, reason = "grammar-smoke-test invocations never call the inherent duplicate — see comment above $builder")]
        impl $composer {
            pub fn compose(sources: &[$crate::io::ComposeSource<'_>]) -> Result<$crate::io::Composition<<Self as $crate::ArtifactComposer>::Snapshot>, $crate::io::ComposeError> { <Self as $crate::ArtifactComposer>::compose(sources) }
        }
    };
    (@children_ty $snapshot:ty) => { $crate::app::NoChildren<$snapshot> };
    (@children_ty $snapshot:ty, $children:ty) => { $children };
}

/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5) — `ArtifactChildren`/`DerivedArtifactSpec::Children`
/// tests. Placed here (rather than beside `🧬️DerivedArtifactFacets` near the top of the file) so
/// `derive_artifact_facets!` is already in scope textually — `macro_rules!` visibility is
/// definition-order-sensitive within a crate for a bare (non-`$crate`-qualified) invocation.
#[cfg(test)]
#[path = "🧪️tests/🔬️derived-artifact-children/🦀️.rs"]
mod derived_artifact_children_tests;

/// 🪆️ Whether a subset owns snapshot/diff/mutation types or derives behavior from a base dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubsetKind {
    Owning,
    Derived,
}

/// 🪆️ Declares one subset's lifecycle facets, runtime registration, and inline conformance tests.
#[macro_export]
macro_rules! subset {
    (
        $vis:vis owning dialect $artifact:literal / $standard:literal / $subset:literal {
            spec $spec:ident {
                construction: $construction:ty,
                analysis: $analysis:ty,
                composition: $composition:ty,
            }
            builder: $builder:ident,
            analyzer: $analyzer:ident,
            composer: $composer:ident,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(validator: $validator:ty,)?
            $(examples: [$($example:expr),+ $(,)?],)?
        }
    ) => {
        $crate::derive_artifact_facets! {
            $vis spec $spec {
                construction: $construction,
                analysis: $analysis,
                composition: $composition,
            }
            builder: $builder,
            analyzer: $analyzer,
            composer: $composer,
        }

        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Owning;
            static REGISTERED: Once = Once::new();
            $(static VALIDATOR_ENTRY: OnceLock<$crate::io::SubsetValidatorEntry> = OnceLock::new();)?

            $(fn validator_entry() -> &'static $crate::io::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init($crate::io::subset_validator_entry_of::<$validator>)
            })?

            pub async fn register() {
                REGISTERED.call_once(|| {
                    let mut entries = vec![$crate::composer_entry_of::<$composer>()];
                    $(entries.extend([$($io_entry),+]);)?
                    $crate::io::register_composer_entries(&entries).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");
                    $($crate::io::register_subset_validator(validator_entry()).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");)?
                });
            }

            $(pub const EXAMPLES: &'static [$crate::ExampleSource] = &[$($example),+];)?

            $crate::__semio_subset_owning_conformance_tests!($composition);
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };

    (
        $vis:vis derived dialect $artifact:literal / $standard:literal / $subset:literal {
            validator: $validator:ty,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(positive: [$($pos_example:expr),+ $(,)?],)?
            $(negative: [$($neg_example:expr),+ $(,)?],)?
        }
    ) => {
        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Derived;
            static REGISTERED: Once = Once::new();
            static VALIDATOR_ENTRY: OnceLock<$crate::io::SubsetValidatorEntry> = OnceLock::new();

            fn validator_entry() -> &'static $crate::io::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init($crate::io::subset_validator_entry_of::<$validator>)
            }

            pub async fn register() {
                REGISTERED.call_once(|| {
                    $crate::io::register_subset_validator(validator_entry()).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error");
                    $( $crate::io::register_composer_entries(&[$($io_entry),+]).expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error"); )?
                });
            }

            $(pub const POSITIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($pos_example),+];)?
            $(pub const NEGATIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($neg_example),+];)?

            $crate::__semio_subset_derived_conformance_tests!($validator);
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️subset-macro/🦀️.rs"]
mod subset_macro_tests;


#[path="🎞️media/📤️snapshot/🦀️.rs"]
pub mod native_snapshot_export;

pub use semio_framework_tool_run_presentation::ToolRunView as OriginalToolRunView;
