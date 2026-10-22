//! 🥅️ Render-independent framework facade: explicit re-exports of the carved producer crates under the module paths the framework always exposed (`kernel`, `manifest`, `action_bus`, `interaction`, `onboarding`, `history_edit`, `io_schema`, `sqlite_snapshot`, `mesh_io`).

pub mod kernel {
    pub use semio_framework_kernel::{ActionContext, ActionRequest, ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, ActorUiPatchReceipt, ArtifactDiff, ArtifactMergeKind, BrokerHooks, CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES, CLIPBOARD_METADATA_MAX_WIRE_BYTES, CLIPBOARD_PASTE_MAX_WIRE_BYTES, CLIPBOARD_TEXT_MAX_BYTES, COLD_PAIR_MAXIMUM_BYTES, COLD_PAIR_MAXIMUM_PAGES, COLD_PAIR_PAGE_MAXIMUM_BYTES, COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES, ClipboardError, ClipboardFragment, ColdDocumentPairApplied, ColdDocumentPairCursor, ColdDocumentPairFrontier, ColdDocumentPairHeader, ColdDocumentPairPage, ColdPairIngressStatus, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandContext, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, Diagnostic, Effect, Event, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, FixedCommandPage, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, InvocationResult, JobPlacement, MessageEndpoint, OriginalJobBudget, OriginalJobCallFailure, OriginalJobCancelReply, OriginalJobOutcome, OriginalJobOutputAckReply, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputState, OriginalJobSource, OriginalJobSourceCursor, OriginalJobStartReply, OriginalJobStepReply, OriginalActorTurnWork, OriginalJobTurnLoan, PagedCommand, PagedCommandReader, PasteAnchor, PastePlacement, PresenceHistoryEdit, PresenceHistoryEditStage, PresencePeer, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView, RejectedCommandBuild, RejectedCommandBuildRegistry, RetainedTurnInput, RetainedTurnReceipt, Severity, TurnResult, UndoPolicy, WindowOutput, decode_presence_history_edit, decode_presence_peer, encode_presence_history_edit, encode_presence_peer, return_content, return_message, spawned_job_completed_event};
    pub use semio_framework_kernel_history::{FRAMEWORK_FAULT_NOTICE_LABELS, FaultNotice, HISTORY_NOTICE_LABELS, HISTORY_REPROJECTION_LABELS, HistoryEntry, HistoryMutationEntry, HistoryMutationMessage, HistoryPatch, HistoryReprojection, HistoryReprojectionKind, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, HistoryTransaction, fault_notice, framework_fault_notice, history_notice, history_reprojection_status};
    pub use semio_framework_kernel_identity::{ActionDef, ActionId, ActionInvocation, ActivationEvent, ActorId, AppEvent, AppInstanceId, Appearance, ArtifactHandle, ArtifactId, ArtifactKind, ArtifactVersion, AssetHandle, BrokerCapabilityGrant, Budget, Capability, CapabilityChange, CapabilityGrant, CapabilityId, CapabilityRequest, CapabilityRequirement, CapabilityToken, CommandId, CommandInvocation, EditRef, ExtensionDescriptor, FailureAction, HybridLogicalTimestamp, InverseMutation, InvocationId, KernelMutation, MutationId, PhysicalSize, PluginInstanceId, QuotaBreach, QuotaSchema, QuotaTree, RequestId, RequestOutcome, Rights, SchemaId, SchemaVersion, Scope, TurnStatus, UndoGroup, Usage, WindowEvent, WindowHandle, WindowInput, WindowKindDef, WindowKindId, extensions_extending, scope_capabilities_to_parent};
    pub use semio_framework_kernel_media_jobs::{FRAMEWORK_RESERVED_JOB_KIND, IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_NAME, IMPORT_ARGUMENT_PAYLOAD, IMPORT_ARGUMENT_TOTAL, IMPORT_CHUNK_BYTES, IMPORT_STAGING_MAXIMUM_BYTES, IMPORT_STAGING_MAXIMUM_CHUNKS, IMPORT_STAGING_RUNS, IconRenderExportItem, ImportArguments, ImportChunk, ImportStaging, ImportStagingRefusal, MEDIA_EXPORT_BASE64_ENCODING, MEDIA_EXPORT_UTF8_ENCODING, MEDIA_VIDEO_RENDER_CAPABILITY, MediaExportEncodingError, SPAWNED_JOB_DEADLINE_MS, SPAWNED_JOB_FUEL, SPAWNED_JOB_STEP_CEILING, SpawnedJobCompletion, SpawnedJobDriveError, SpawnedJobStep, VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE, VIDEO_RENDER_PROGRAM_MAXIMUM_FPS, VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES, VIDEO_RENDER_PROGRAM_SCHEMA, VideoRenderEncoderTier, VideoRenderImage, VideoRenderJobEvent, VideoRenderJobEventError, VideoRenderJobLedger, VideoRenderJobOutcome, VideoRenderJobRow, VideoRenderOp, VideoRenderPath, VideoRenderProgram, VideoRenderProgramError, VideoRenderRun, VideoRenderScene, import_chunk_arguments, import_payload_chunks, media_export_bytes, spawned_job_completion, video_render_crop_is_admitted, video_render_image_url_is_admitted, video_render_path_is_well_formed};
    pub use semio_framework_kernel_ui::{PresenceUpdate, UI_TURN_PATCHES_MAXIMUM, UI_TURN_PATCH_BUDGET_BYTES, UI_TURN_PATCH_OP_WIRE_ENVELOPE_BYTES, UI_TURN_PATCH_OWNER_BYTES, UI_TURN_PATCH_RETIRE_SLOTS, UI_TURN_PATCH_TRANSPORT_SLOTS, UI_TURN_PATCH_WIRE_ENVELOPE_BYTES, UiDirtyScope, UiDirtySection, UiPatch, UiPatchOp, UiTurnPatchTransfer, UiTurnPatchTransportLease, UiTurnPatchTransportProducer, UiTurnPatchTransportProgress, UiTurnPatchTransportStep, UiTurnPatches, UiTurnPatchesIntoIter, close_ui_turn_patch_owner_one, close_ui_turn_patch_owner_with_grant, close_ui_turn_patch_transport_one, close_ui_turn_patch_transport_session_one, close_ui_turn_patch_transport_with_grant, ui_patch_turn_bytes};
}

pub mod manifest {
    pub use crate::kernel;
    pub use semio_framework_kernel_history::{FaultNoticeDefinition, FaultNoticeError, fault_notice_definitions, fault_notice_placeholders, fault_notice_text, fill_fault_notice, is_fault_notice_code, validate_fault_notices};
    pub use semio_framework_manifest_actions::{ActionDefinition, ActionKind, ActionSemantics, ApprovalMode, CLEAR_SELECTION_ACTION_ID, CapabilityAudience, CapabilityEffects, CapabilityExecution, CapabilityPolicy, CommandDefinition, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, ExecutionClass, HISTORY_COMMAND_FILTER_ALL, HISTORY_COMMAND_FILTER_ONLY_MUTATIONS, HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS, HOST_EVENT_ACTION_ID, HOST_EVENT_ARG_KIND, HOST_EVENT_ARG_WINDOW_ID, HOST_EVENT_KIND_BLUR, HOST_EVENT_KIND_CAPTURE_LOST, HOST_EVENT_KIND_RETIRING, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, IdempotencyMode, InitialWindowUtilityInput, InitialWindowUtilityResolution, InteractionVerb, InteractiveJobClassification, InteractiveJobClassificationError, NATURAL_FILE_PORT_ID, NOTE_SHELL_COMMAND_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, OsDefinition, PreviewMode, REVERT_TO_COMMAND_ACTION_ID, ResourceSelector, SAVE_ARTIFACT_FILE_ACTION_ID, SELECT_ALL_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, SWITCH_ALTERNATIVE_ACTION_ID, SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, ToolDefinition, UndoMode, UtilityDefinition, clipboard_action_definitions, default_action_icon_id, derive_audience, document_transfer_action_definitions, history_action_definitions, host_event_action_definition, interaction_declared_refresh_scope, interaction_verb_surface_scope, note_shell_command_action_definition, resolve_audience, resolve_command_audience, resolve_initial_window_utility, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, validate_interactive_job_classification};
    pub use semio_framework_manifest_address::{ActionAddress, ActionInvocation, ActionRef, CommandAddress, CommandInvocation, CommandInvocationView, CommandOwnerAddress, Keybinding, Platform, PlatformKeybinding, ToolRef, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID, UtilityRef, child_element_id, element_id_segment, is_element_id, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id};
    pub use semio_framework_manifest_app_surface::{AppDefinition, AppRef, CommandGrammar, CommandVariantSpec, ConfigSpec, DIALOG_CHOICE_ARG, DialogChoice, DialogDefinition, ExampleDefinition, ModeDefinition, Modes, NonEmptyVec, PanelGroup, PanelTabDefinition, PanelTabKind, ReservedContribution, WindowKindDefinition, WindowKinds, action_arg_requires_choice, app_breadcrumb, app_window_label, asset_name_segment, effective_action_args, example_body_asset_name, example_body_asset_prefix, examples_for_app, examples_for_dialect, interaction_action_definitions, interaction_window_bodies_by_domain, invalid_action_choice_args, missing_required_args, panel_leaf_body_keys, parse_surface_app_id, reserved_app_actions, reserved_audience_violation, reserved_editor_dialogs, reserved_fixed_audience, reserved_window_actions, resolve_app_breadcrumb, resolve_layout_for_mode, resolve_mode_tools, resolve_window_actions, surface_app_id, unresolved_action_args, window_kind_actions};
    pub use semio_framework_manifest_arg_schema::{ActionArgControl, ActionArgDef, ActionArgNumberFacets, ActionArgOption, AppRole, ArgFormat, ArgPresentation, ArgSchema, NumberScale, OptionSource, REFERENCE_ID_INTEGER_MAX, ReferenceIdType, SnapSource, action_arg_unit_symbol, interaction_entity_kind, reference_id_text};
    pub use semio_framework_manifest_contribution::{ArtifactContributionDescriptor, ContributedInferenceMetadata, ContributedMutationMetadata, ContributedMutationSemantics, INFERENCE_ARTIFACT_PACK_BASE64, InferenceArtifactBinding, InferenceCommitBinding, InferencePayloadContract, PluginDependency, ProgramContributionEntry, SPACE_INDEX_TOPIC, SpaceIndexContribution, TopicContribution, Version, VersionParseError, VersionPin, VersionPinParseError, parse_contributions, tree_pin};
    pub use semio_framework_manifest_media::{AppIo, ArtifactKindSpec, ArtifactPresentation, HostedArtifactKind, Media, MediaClass, MediaCompat, MediaConverter, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, OsMediaCapability, PortMultiplicity, media_types_compatible};
    pub use semio_framework_manifest_mutation_inputs::{FRAMEWORK_INPUT_SCHEMA_DOCUMENTS, InputSchemaAudit, InputSchemaError, InputSchemaErrorCode, InputSchemaResolver, input_label_glossary, mutation_input_audit, mutation_input_defs, mutation_input_instance, registered_input_schema_document};
    pub use semio_framework_manifest_plugin::{ASSEMBLY_FAILED_PLUGIN_ID, AgentContributions, ArtifactKindChoice, AssetDeclaration, CanonicalDescriptorValue, ComposerEntryDescriptor, ContributionSet, DependencyGraphError, DescriptorEntry, ExecutionMode, ExecutionProtocol, ExtensionPointDeclaration, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, PackageDescriptor, PackageRole, PluginManifest, SurfaceAppChoice, artifact_kind_choices, decode_artifact_kind_choice, decode_surface_app_choice, dependents, encode_artifact_kind_choice, encode_surface_app_choice, resolve_load_order};
    pub use semio_framework_manifest_view_model::{MAX_SURFACE_BODY_KEY_BYTES, MAX_SURFACE_VIEW_CONTEXT_BYTES, PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_DEPTH, PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR, PUBLIC_INVOCATION_STRING_BYTES, TreeWindowRequest, UI_REFRESH_SECTION_BODY_KEYS, UI_REFRESH_SECTION_KEYS, UiRefreshSection, VIEW_CONTEXT_IDENTIFIER_CHARS, VIEW_CONTEXT_IDENTIFIER_FIELDS, VIEW_CONTEXT_LONG_STRING_CHARS, VIEW_CONTEXT_LONG_STRING_FIELDS, VIEW_CONTEXT_SESSION_IDENTITY_FIELDS, VIEW_CONTEXT_TRACE_CURSOR_ENTRIES, VIEW_CONTEXT_TRACE_CURSOR_FIELDS, VIEW_CONTEXT_TREE_WINDOWS, VIEW_CONTEXT_TREE_WINDOW_FIELDS, VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS, VIEW_CONTEXT_UTILITY_ENTRIES, VIEW_CONTEXT_WINDOW_INSTANCES, ViewModel, ViewSessionIdentity, ViewWindowInstance, public_invocation_char_cost, public_invocation_string_pages};
}

pub use semio_framework_action_bus as action_bus;
pub use semio_framework_interaction as interaction;

//#region 🧩️ReservedFeatures
pub use semio_framework_time_travel_actions as history_edit;
pub use semio_framework_onboarding as onboarding;

/// 🧩️ The framework features that reserve actions or dialogs on every hosting app, in injection order: the history-edit
/// verbs join the app roster after the history lane, the tool run actions join every window kind after the interaction
/// verbs. A host adds its own contributions (such as audience rules) after these.
pub const FRAMEWORK_RESERVED_CONTRIBUTIONS: [manifest::ReservedContribution; 2] = [semio_framework_time_travel_actions::RESERVED, semio_framework_tool_run_actions::RESERVED];
//#endregion 🧩️ReservedFeatures

pub use semio_framework_ui_declaration::IconName;
pub use semio_framework_ui_locale::{Locale, Terminology};
pub use semio_framework_io_schema as io_schema;
pub use semio_framework_io_sqlite_snapshot as sqlite_snapshot;
pub use action_bus::{
    ActionBus, ErasedToolJob, ToolCancellationPolicy, ToolDispatchError, ToolExecutionContract, ToolExecutionShape, ToolFactoryKey, ToolFreshnessPolicy, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
    ToolPayload, ToolRegistrationError, ToolWireAdmission,
};
pub use semio_framework_value::{dsl_value,DslValue};
pub use semio_framework_diagnostic::Diagnostic;
pub use semio_framework_diagnostic::Fault;
pub use semio_framework_diagnostic::FaultCause;
pub use semio_framework_diagnostic::FaultCode;
pub use semio_framework_diagnostic::FaultFrom;
pub use semio_framework_diagnostic::FaultOrigin;
pub use semio_framework_diagnostic::FaultParams;
pub use semio_framework_diagnostic::is_fault_param_name;
pub use semio_framework_diagnostic::FaultScope;
pub use semio_framework_diagnostic::Severity;
pub use semio_framework_diagnostic::TextError;
pub use semio_framework_diagnostic::TextSpan;
// 🛂️ The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod manifest`.
// 🔺️ Mesh geometry data, primitive construction, and Obj/Glb/Stl codecs are dissolved into a
// dedicated engine crate (consumed only from artifact facet code / engine-to-engine callers such
// as brep tessellation) — no longer part of this framework module's own re-export surface.
pub use semio_framework_mesh_engine::io as mesh_io;
pub use semio_framework_mesh_engine::{mesh_box, mesh_cone, mesh_cylinder, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_ico_sphere, mesh_plane, mesh_torus, mesh_uv_sphere, MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation, MeshTexture, PolygonMeshSource, validate_polygon_mesh_attributes, validate_mesh_surface_assets, validate_mesh_attribute, MeshData, MeshCanonicalDecoder, MeshCanonicalIntakeSource};
pub use semio_framework_artifact_reference::{ArtifactDialect, Dialect, StandardId, SubsetId};
pub use semio_framework_action_bus::platform::{PanelVisibility, Platform, PlatformSpec};
pub use kernel::{
    decode_presence_peer,
    encode_presence_peer,
    ActionContext,
    ActionDef,
    ActionId,
    ActionInvocation,
    ActionRequest,
    ActorId,
    AppEvent,
    AppInstanceId,
    Appearance,
    ArtifactDiff,
    ArtifactHandle,
    ArtifactId,
    ArtifactKind,
    ArtifactVersion,
    AssetHandle,
    Capability,
    CapabilityGrant,
    CapabilityRequirement,
    CapabilityToken,
    CommandContext,
    CommandId,
    CommandInvocation,
    Effect,
    HybridLogicalTimestamp,
    IconRenderExportItem,
    InverseMutation,
    InvocationId,
    InvocationResult,
    KernelMutation,
    MutationId,
    PhysicalSize,
    PluginInstanceId,
    PresencePeer,
    PresenceToolRun,
    PresenceToolRunState,
    PresenceUi,
    PresenceViewKind,
    PresenceWindowView,
    // 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet A3-kernel-types: `RequestId`
    // is the completion-correlation id every `req`-carrying `Effect` variant now needs at its call
    // site — re-exported here so plugin call sites can name it as `semio_framework::RequestId` /
    // `semio_framework_plugin::RequestId` without a separate import.
    RequestId,
    Rights,
    SchemaId,
    SchemaVersion,
    Scope,
    UndoGroup,
    UndoPolicy,
    WindowEvent,
    WindowHandle,
    WindowInput,
    WindowKindDef,
    WindowKindId,
    WindowOutput,
};

pub use semio_framework_abi::{ABI_MAX_BODY_BYTES, ABI_MAX_EVENT_CODE, ABI_MAX_IN_FLIGHT_HANDLES, ABI_MAX_IN_FLIGHT_REQUESTS, ABI_MAX_MESSAGE_BYTES, ABI_MAX_OPERATION_CODE, ABI_MAX_PAGES_PER_TRANSFER, ABI_MAX_PAGE_BYTES, ABI_MAX_TRANSFER_BYTES, ABI_SCHEMA_JSON, ABI_VERSION, AbiBytes, AbiCancelOutcome, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage, AbiMessageBytes, AbiOperation, AbiPage, AbiPageBytes, AbiPageReader, AbiPageWriter, AbiPort, AbiPortPoll, AbiPortRejection, AbiRejectedBytes, AbiRejectedPage, AbiReply, AbiReplyLedger, AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, RetainedCloneGrant, decode_abi_message, encode_abi_message};
pub use semio_framework_interaction::{DomainHover, DomainSelection, DomainTopology, GranularityDefinition, HierarchyProvider, HoverInput, HoverSpec, InteractionDefinition, InteractionOutline, InteractionRef, InteractionState, InteractionTarget, InteractionTopology, MergeMode, PresenceDomain, PresenceInteraction, SelectionInput, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, gesture, next_hover, next_selection, schema, validate_state};
pub use semio_framework_kernel_history::{FaultNoticeDefinition, FaultNoticeError, fault_notice_definitions, fault_notice_placeholders, fault_notice_text, fill_fault_notice, is_fault_notice_code, validate_fault_notices};
pub use semio_framework_manifest_actions::{ActionDefinition, ActionKind, ActionSemantics, ApprovalMode, CLEAR_SELECTION_ACTION_ID, CapabilityAudience, CapabilityEffects, CapabilityExecution, CapabilityPolicy, CommandDefinition, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, ExecutionClass, HISTORY_COMMAND_FILTER_ALL, HISTORY_COMMAND_FILTER_ONLY_MUTATIONS, HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS, HOST_EVENT_ACTION_ID, HOST_EVENT_ARG_KIND, HOST_EVENT_ARG_WINDOW_ID, HOST_EVENT_KIND_BLUR, HOST_EVENT_KIND_CAPTURE_LOST, HOST_EVENT_KIND_RETIRING, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, IdempotencyMode, InitialWindowUtilityInput, InitialWindowUtilityResolution, InteractionVerb, InteractiveJobClassification, InteractiveJobClassificationError, NATURAL_FILE_PORT_ID, NOTE_SHELL_COMMAND_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, OsDefinition, PreviewMode, REVERT_TO_COMMAND_ACTION_ID, ResourceSelector, SAVE_ARTIFACT_FILE_ACTION_ID, SELECT_ALL_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, SWITCH_ALTERNATIVE_ACTION_ID, SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, ToolDefinition, UndoMode, UtilityDefinition, clipboard_action_definitions, default_action_icon_id, derive_audience, document_transfer_action_definitions, history_action_definitions, host_event_action_definition, interaction_declared_refresh_scope, interaction_verb_surface_scope, note_shell_command_action_definition, resolve_audience, resolve_command_audience, resolve_initial_window_utility, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, validate_interactive_job_classification};
pub use semio_framework_manifest_address::{ActionAddress, ActionRef, CommandAddress, CommandInvocationView, CommandOwnerAddress, Keybinding, PlatformKeybinding, ToolRef, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID, UtilityRef, child_element_id, element_id_segment, is_element_id, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id};
pub use semio_framework_manifest_app_surface::{AppDefinition, AppRef, CommandGrammar, CommandVariantSpec, ConfigSpec, DIALOG_CHOICE_ARG, DialogChoice, DialogDefinition, ExampleDefinition, ModeDefinition, Modes, NonEmptyVec, PanelGroup, PanelTabDefinition, PanelTabKind, ReservedContribution, WindowKindDefinition, WindowKinds, action_arg_requires_choice, app_breadcrumb, app_window_label, asset_name_segment, effective_action_args, example_body_asset_name, example_body_asset_prefix, examples_for_app, examples_for_dialect, interaction_action_definitions, interaction_window_bodies_by_domain, invalid_action_choice_args, missing_required_args, panel_leaf_body_keys, parse_surface_app_id, reserved_app_actions, reserved_audience_violation, reserved_editor_dialogs, reserved_fixed_audience, reserved_window_actions, resolve_app_breadcrumb, resolve_layout_for_mode, resolve_mode_tools, resolve_window_actions, surface_app_id, unresolved_action_args, window_kind_actions};
pub use semio_framework_manifest_arg_schema::{ActionArgControl, ActionArgDef, ActionArgNumberFacets, ActionArgOption, AppRole, ArgFormat, ArgPresentation, ArgSchema, NumberScale, OptionSource, REFERENCE_ID_INTEGER_MAX, ReferenceIdType, SnapSource, action_arg_unit_symbol, interaction_entity_kind, reference_id_text};
pub use semio_framework_manifest_contribution::{ArtifactContributionDescriptor, ContributedInferenceMetadata, ContributedMutationMetadata, ContributedMutationSemantics, INFERENCE_ARTIFACT_PACK_BASE64, InferenceArtifactBinding, InferenceCommitBinding, InferencePayloadContract, PluginDependency, ProgramContributionEntry, SPACE_INDEX_TOPIC, SpaceIndexContribution, TopicContribution, Version, VersionParseError, VersionPin, VersionPinParseError, parse_contributions, tree_pin};
pub use semio_framework_manifest_media::{AppIo, ArtifactKindSpec, ArtifactPresentation, HostedArtifactKind, Media, MediaClass, MediaCompat, MediaConverter, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, OsMediaCapability, PortMultiplicity, media_types_compatible};
pub use semio_framework_manifest_mutation_inputs::{FRAMEWORK_INPUT_SCHEMA_DOCUMENTS, InputSchemaAudit, InputSchemaError, InputSchemaErrorCode, InputSchemaResolver, input_label_glossary, mutation_input_audit, mutation_input_defs, mutation_input_instance, registered_input_schema_document};
pub use semio_framework_manifest_plugin::{ASSEMBLY_FAILED_PLUGIN_ID, AgentContributions, ArtifactKindChoice, AssetDeclaration, CanonicalDescriptorValue, ComposerEntryDescriptor, ContributionSet, DependencyGraphError, DescriptorEntry, ExecutionMode, ExecutionProtocol, ExtensionPointDeclaration, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, PackageDescriptor, PackageRole, PluginManifest, SurfaceAppChoice, artifact_kind_choices, decode_artifact_kind_choice, decode_surface_app_choice, dependents, encode_artifact_kind_choice, encode_surface_app_choice, resolve_load_order};
pub use semio_framework_manifest_view_model::{MAX_SURFACE_BODY_KEY_BYTES, MAX_SURFACE_VIEW_CONTEXT_BYTES, PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_DEPTH, PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR, PUBLIC_INVOCATION_STRING_BYTES, TreeWindowRequest, UI_REFRESH_SECTION_BODY_KEYS, UI_REFRESH_SECTION_KEYS, UiRefreshSection, VIEW_CONTEXT_IDENTIFIER_CHARS, VIEW_CONTEXT_IDENTIFIER_FIELDS, VIEW_CONTEXT_LONG_STRING_CHARS, VIEW_CONTEXT_LONG_STRING_FIELDS, VIEW_CONTEXT_SESSION_IDENTITY_FIELDS, VIEW_CONTEXT_TRACE_CURSOR_ENTRIES, VIEW_CONTEXT_TRACE_CURSOR_FIELDS, VIEW_CONTEXT_TREE_WINDOWS, VIEW_CONTEXT_TREE_WINDOW_FIELDS, VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS, VIEW_CONTEXT_UTILITY_ENTRIES, VIEW_CONTEXT_WINDOW_INSTANCES, ViewModel, ViewSessionIdentity, ViewWindowInstance, public_invocation_char_cost, public_invocation_string_pages};

//#region 🧬️SchemaMetadata
#[cfg(test)]
#[path = "../../🔨️modules/🧬️schema/📽️projection/🦀️.rs"]
pub mod schema_metadata;
//#endregion 🧬️SchemaMetadata

#[cfg(test)]
#[path = "../../🔨️modules/🧬️schema/📽️projection/🧪️tests/🦀️.rs"]
mod typegen_tests;

#[cfg(test)]
#[path = "../../🔨️modules/🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs"]
mod framework_reserved_tests;
