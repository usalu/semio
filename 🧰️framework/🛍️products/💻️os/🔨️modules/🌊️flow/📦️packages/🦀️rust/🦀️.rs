//! 🌊️ OS flow family facade — re-exports the pre-split public API of the document, catalogue, registry, bridge, host, drawing, vcs, geometry, mesh and wasm session producers.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow_extension_sdk;

//#region 🔖️KernelModuleAliases
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_vcs;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
//#endregion 🔖️KernelModuleAliases

//#region 🔖️InfiniteAlias
/// ♾️ Flow components use `crate::infinite::{board,canvas}` paths.
pub use semio_framework_os_infinite as infinite;
//#endregion 🔖️InfiniteAlias

//#region 🔖️DagCanvasNeural
pub use crate::infinite::board::ports::directed_dag as dag;
pub use semio_framework_canvas as canvas;
pub use neural_engine as neural;
//#endregion 🔖️DagCanvasNeural

pub use semio_framework_os_flow_vcs::forms;
pub use semio_framework_os_flow_operators::{bridge, catalogue, registry};
pub use semio_framework_os_flow_host as host;
pub use semio_framework_os_flow_drawing as drawing;
pub use semio_framework_os_flow_vcs as vcs;
pub use semio_framework_os_flow_mesh as mesh;
pub use semio_framework_os_flow_geometry as geometry;

// 🌉️ `wasm_session`'s `flow_bridge_*` linear-memory ABI is the browser wasm-pack SDK entry point
// (its own `ui_webgpu::{CanvasMetrics, SurfaceId, SurfaceGeneration}` imports are canvas-surface
// types); nothing in `semio-s-plugin-flow`'s own `plugin_exports!` guest dispatch calls it
// (confirmed by repo-wide grep). `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so a
// bare arch gate would not exclude the shipped WASI component target — narrowed to keep native
// (the producers' own `#[cfg(test)]` suites over the bridge protocol) and browser wasm, drop only wasip2,
// matching `os-kernel-host-crates-split.md`'s target-table shape. RUNTIME-DEPENDENCY-ELIMINATION ticket
// 26/09/01.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use semio_framework_os_flow_wasm as wasm_session;

pub mod extensions {
    pub use semio_framework_os_flow_operators::extensions::schema;
    pub use semio_framework_os_flow_extensions as wasm;
}

pub use semio_framework_os_flow_drawing::{
    Affine2D, boolean_segments_json, dispose_drawing, DrawingEngine, DrawingHandle, DrawingKernel,
    DrawingKind, DrawingNode, DrawingScene, DrawingStore, export_pdf_json, export_svg_json,
    FillStyle, GradientStop, LineCap, LineJoin, render_scene_json, retain_drawing_handles,
    SceneNode, StrokeStyle, trace_bitmap_json, with_drawing_kernel,
};

pub use semio_framework_os_flow_operators::{
    CatalogueGroup, CatalogueItem, CatalogueSection, drain_flow_extension_registry_retirements, flow_app_catalogue, flow_app_catalogue_json,
    flow_app_catalogue_json_shared, flow_catalogue_sections, FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_BYTES, FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_PAGES, flow_extension_invocation_address, flow_extension_registry,
    flow_extension_registry_generation, FLOW_EXTENSION_REGISTRY_TEST_LOCK, FLOW_LOD_MODE_AUTOMATIC, flow_neuron_kind_info_map, flow_neuron_kind_infos_json, flow_operator_catalogue_json,
    flow_operator_catalogue_records, flow_operator_registry, flow_palette_catalogue_sections, FlowAppCatalogue, FlowExtensionAddressMiss, FlowExtensionInfo,
    FlowExtensionManifestRejection, FlowExtensionSpec, host_flow_extension_contributions_pending_bytes, install_builtin_flow_extensions, install_flow_extension, install_flow_extension_manifest,
    installed_flow_extensions, installed_flow_extensions_shared, lock_flow_extension_registry_for_test, next_flow_extension_registry_retirement_demands, register_linked_flow_extension_installer, reset_host_flow_extension_contributions_pages,
    retire_flow_extension_registries_step, seed, seed_flow_eval_node_cache, sync_host_flow_extension_contributions, sync_host_flow_extension_contributions_page, test_flow_registry_retirement_step,
    uninstall_flow_extension, unregister_linked_flow_extension_installer,
};

pub use semio_framework_os_flow_vcs::{
    FLOW_VCS_DEADLINE_MILLISECONDS, FLOW_VCS_FEATURES, FLOW_VCS_MAX_BYTES, FLOW_VCS_MAX_CONTROLS, FLOW_VCS_MAX_DEPTH, FLOW_VCS_MAX_EVENTS,
    FLOW_VCS_MAX_HISTORY, FLOW_VCS_MAX_ITEMS, FLOW_VCS_MAX_OPERATIONS, FLOW_VCS_MAX_OUTPUTS, FLOW_VCS_MAX_PAGES, FlowRetainedVcs,
    FlowSurfaceOwner, FlowVcsAuthority, FlowVcsCloseDemands, FlowVcsCloseFailure, FlowVcsClosePhase, FlowVcsCredits,
    FlowVcsFault, FlowVcsGrant, FlowVcsHandle, FlowVcsPage, FlowVcsPoll, FlowVcsResourceFingerprint,
    FlowVcsSource, forms_bridge,
};

pub use semio_framework_os_flow_extensions::{
    build_manifest_json, cancel_all_evaluations, cancel_evaluation, cancel_inference_evaluation, command_json, EVALUATE_ENVELOPE_SCHEMA,
    evaluate_function_json, evaluate_invoke_json, evaluate_json, EVALUATE_STEP_BUDGET, evaluate_step_envelope_cold, EVALUATE_STEP_WALL_MICROS,
    EvaluateRequest, evaluation_dependency_identity, evaluation_progress, evaluation_retirement_demands, evaluation_retirement_pending, EvaluationExternalResult,
    EvaluationFailure, EvaluationInputPreparation, EvaluationInvokeCursor, EvaluationReply, EvaluationRequestContextInput, ExtensionEvaluationResources,
    flow_extension_topic_contribution, flow_inference_dependency_json, FlowExtensionCommand, FlowExtensionContributes, FlowExtensionManifest, FlowExtensionSetting,
    FlowExtensionWidget, integer_setting_default, retire_cancelled_evaluations_close_step,
};

pub use semio_framework_os_flow_host_core::{
    FLOW_SHARED_PAYLOAD_PART_SEPARATOR, FLOW_SHARED_PAYLOAD_PREFIX, FlowCoreError, FlowEvaluationStep, FlowHost, FlowHostClosePhase,
    FlowHostRetirement, FlowHostRetirementState, FlowWheelPlan, port_value_types_compatible, PortSide, resolve_flow_shared_payload,
    resolve_flow_shared_payload_parts, retire_flow_shared_payloads, widget_port_value_types,
};

pub use semio_framework_os_flow_host_read::{
    FlowCurrentOutput, FlowCurrentOutputRead, FlowPreviewSource, FlowPreviewSourceTransfer, FlowPreviewTessellationRequest, FlowPreviewTessellationWriter,
    FlowSourceRead, FlowSourceReadFailure, FlowSourceReadRequest, FlowSourceReadStep, FlowSourceReadTarget,
};

pub use semio_framework_os_flow_host_session::{
    ExtensionEvaluateFault, flow_backed_node_graph_extras, flow_eval_inline_continuation_fits, FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US, flow_eval_publication_for, flow_eval_publication_ledger,
    flow_eval_step_ledger, flow_eval_tick_budget, FLOW_EVAL_TICK_ELAPSED_CEILING_US, FLOW_EVAL_TICK_STEP_BUDGET, flow_host_with_session, FlowBackedNodeGraphExtras,
    FlowEvalPublication, FlowEvalPublicationLedger, FlowEvalSession, FlowEvalSessionState, FlowEvalStepLedger, FlowEvaluationReplyAddress,
    FlowEvaluationReplyIntakeSource, FlowEvaluationReplyReceiver, FlowInvocationIdentity, FlowInvocationOrigin, FlowInvocationOriginLease, FlowPreviewCompletedAddress,
    FlowPreviewCompletedIntakeSource, FlowPreviewCompletedReceiver, FlowPreviewLedgerFold, FlowSnapshotIntakeFailure, io, merge_unanswered_eval_entries,
    NodeEvalStatus, preview_tessellate_node_hash, PreviewChainStatus, PreviewEvalOutcome, PreviewEvalPhase, PreviewEvalProgress,
    PreviewEvalStatus, PreviewTessellateOutcome, PreviewTessellatePhase, PreviewTessellateProgress, PreviewTessellateStatus, record_flow_eval_step,
    reset_flow_eval_publication_ledger, reset_flow_eval_step_ledger, retire_invocation_reply_cold, unserved_flow_operator_kinds,
};
