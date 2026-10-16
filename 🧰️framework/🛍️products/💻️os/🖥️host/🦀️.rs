//! 🖥️ Plugin-based OS kernel: hot-swappable WASM plugins, workflow, document VCS.
#[cfg(test)] use semio_framework_artifact_reference::{ArtifactDialect,Dialect,StandardId,SubsetId};

#[path = "🔌️plugin-host/🦀️.rs"]
pub mod host;

#[path = "🗃️document-backbone/🦀️.rs"]
pub mod backbone;

#[path = "🧾️media-export-simple/🦀️.rs"]
pub mod media_export_simple;

pub use semio_framework_os_host_codec_abi as codec_abi;
pub use semio_framework_os_host_instance as instance;
pub use semio_framework_os_host_media_raster as media_export_raster;
pub use semio_framework_os_host_registry as registry;
pub use semio_framework_os_host_workflow as workflow;

pub use crate::space::*;
pub use crate::workflow::{
    apply_flow_host_snapshot_to_os_workflow, apply_workflow_operation, build_os_workflow_operator_infos, create_default_workflow_parameter, empty_workflow, empty_workflow_snapshot, export_os_app_instance_media_kind, import_os_app_instance_media_kind,
    negotiate_media_contract, os_media_export_extension_for_format_kind, os_media_neuron_kind_for_node, os_resource_media_capability, os_workflow_to_flow_host_snapshot_json, os_workflow_to_node_graph_payload, patch_workflow_parameter,
    placeholder_media_contract, plan_workflow, sync_workflow_parameter_ports, validate_workflow, validate_workflow_parameter_config_binding, validate_workflow_snapshot, workflow_node_for_app, workflow_parameter_id,
    workflow_parameter_id_from_port_id, workflow_parameter_name, workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, OsMediaCapability, OsWorkflowCamera, OsWorkflowNodeGraphPayload, OsWorkflowOperatorInfo, Workflow,
    WorkflowDelivery, WorkflowEdge, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowMutation, WorkflowNode, WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch,
    WorkflowParameterType, WorkflowPosition, WorkflowSnapshot, WorkflowValidation, OS_MEDIA_FLOW_MODULE_ID, OS_SPACE_SCHEMA, OS_WORKFLOW_VFS_ROOT_ID, S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
};
#[cfg(not(target_arch = "wasm32"))]
pub use backbone::{open_file_space_backbone, open_folder_space_backbone};
pub use host::{
    create_backbone_document, create_os_space, decode_backbone_payload, delete_os_space, encode_backbone_payload, export_backbone_dsl, export_backbone_pack, export_os_space_dsl, export_os_space_pack, import_os_space_from_dsl,
    import_os_space_from_pack, list_os_space_catalog_entries, load_os_space_document, materialize_backbone_snapshot, seed_os_space_catalog_if_empty, BackboneDocument, LoadedProgram, OsBackbonePort, OsBackbonePorts, OsCollectionDocument,
    OsSpaceCatalogEntry, OsSpaceDocument, OsSpaceStore, OsWorkflowArtifactDocument, OsWorkflowStore, PluginHost, ProgramHotSwapEvent, OS_HOME_VFS_ROOT_ID, OS_SPACE_BACKBONE_URI_PREFIX,
};
pub use instance::{
    apply_parameter_values_to_snapshot, create_default_os_parameter, create_os_artifact_id, create_os_id, is_parameter_port_id, materialize_os_app_instance_document_json, media_port_id_for_spec, media_port_spec_id,
    os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter, resolve_parameter_values_for_instance, set_json_pointer_value, OsArtifactRef, OsInstanceState,
    OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType, OS_PARAMETER_PORT_PREFIX,
};
pub use media_export_raster::{
    media_accept_filter_kinds, rasterize_svg_to_png_base64, register_2d_export_handlers, register_mesh_exporter, register_mesh_importer, register_os_media_export_handler_kind, register_os_media_import_handler_kind, svg_to_polylines,
    OsMediaExportResult,
};
pub use media_export_simple::{map_points_svg, pages_rects_svg, title_card_svg, wrap_svg};
pub use registry::{
    list_os_artifact_descriptors, os_app_primary_output_kind, os_app_registration, os_artifact_descriptor, os_artifact_dialect, register_app_io, register_artifact_descriptor, register_artifact_descriptors, resolve_os_app_definition,
    try_os_artifact_descriptor, workflow_palette, AppPaletteEntry, OsAppRegistration, OsArtifactDescriptor, OsArtifactKindId, PluginRegistry,
};
pub use semio_framework::*;
pub use store::{os_store::store_schema::document_backbone_ref, os_store::backbone::set_host_backbone_port, os_store::store_schema::ArtifactBackboneRef, os_store::store_schema::ArtifactCommand, os_store::backbone::LocalStorageBackbonePort, os_store::backbone::MemoryBackbonePort};
pub use ui_wgpu::wgpu::*;
pub use vcs::{Author, Checkpoint, VcsError};
