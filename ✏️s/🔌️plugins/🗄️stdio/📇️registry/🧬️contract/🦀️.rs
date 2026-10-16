//! 🧩 Shared, catalog-independent contracts for one stdio artifact package: the facade over the producer crates carved from the former single crate.

pub use pack;
pub use semio_framework_diagnostic as diagnostic;
pub use semio_framework_os_kernel as kernel;
pub use semio_framework_ui_locale as locale;
pub use semio_framework_value as value;

pub use semio_s_artifact_stdio_contract_assembly::{
    artifact_definition_lookup_count, artifact_definition_parse_count, capability_ledger, definition_from_schema, definition_from_schema_with_executables, definition_hierarchy, definition_only_assembly, format_descriptors, native_codec_executables,
    native_codec_factory_receipts, native_codec_hash, registry, runtime_assembly, schema_summary, validate_definition_constraint, ArtifactAssembly, ArtifactContribution, ArtifactExecutable, ArtifactRepresentationClaims, ArtifactSchemaSummary,
    CapabilityCounts, CapabilityLedger, NativeCodecFactory, NativeCodecFactoryReceipt, STDIO_REGISTRY_SCHEMA_DOCUMENTS,
};
pub use semio_s_artifact_stdio_contract_editing::{bounded_native_editing_editor, snapshot_details_editor_support, snapshot_editing_bounded_first_step_tool_proofs, snapshot_editing_command_roster, snapshot_editing_publication_authority};
pub use semio_s_artifact_stdio_contract_list_delta as list_delta;
pub use semio_s_artifact_stdio_contract_list_delta::stdio_list_delta;
pub use semio_s_artifact_stdio_contract_media_export as media_export;
pub use semio_s_artifact_stdio_contract_native_print as native_print;
pub use semio_s_artifact_stdio_contract_part21 as part21;
#[cfg(feature = "testing")]
pub use semio_s_artifact_stdio_contract_testing as testing;
pub use semio_s_artifact_stdio_contract_window_kit::{
    addressed_table_window_kind, example_id_argument, import_media_as_load, load_example_effect, media_duration_ms, media_transport_resource, render_structural_table, require_window_kit_document_revision, revision_addressed_table_window_kind,
    row_revision_addressed_table_window_kind, set_active_example_action, set_active_example_args, set_active_example_description, stable_addressed_table_window_kind, structural_table_window_kind, window_kit_artifact_publication_revision,
    window_kit_canonical_revision, window_kit_document_text_edit, window_kit_index_argument, window_kit_indexed_revision_arguments, window_kit_publication_revision, window_kit_render_publication_revision, window_kit_required_index_argument,
    window_kit_required_text_argument, window_kit_revision_arguments, window_kit_revisioned_cell_arguments, window_kit_revisioned_cell_edit, window_kit_revisioned_editable_cells, window_kit_snapshot_revision, window_kit_splices_argument,
    window_kit_stable_cell_edit, window_kit_text_argument, WindowKitDocumentTextEdit, WindowKitRevisionedCellEdit, WindowKitStableCellEdit, ADD_TABLE_COLUMN_ACTION_ID, ADD_TABLE_ROW_ACTION_ID, REMOVE_TABLE_COLUMN_ACTION_ID,
    REMOVE_TABLE_ROW_ACTION_ID, SET_ACTIVE_EXAMPLE_ACTION_ID, SET_TABLE_HEADER_ACTION_ID,
};
pub use semio_s_artifact_stdio_contract_wire::{
    apply_draft_splices, apply_mutation, apply_mutation_checked, base64_standard, deserialize_double_option, draft_splices_from_json, impl_serde_op_codec, invert_draft_splices, mutation_from_payload_json, mutation_inverse, mutation_payload_json,
    ordered, ordered_by_key, ordered_unique, ordered_unique_descending, semantic_fingerprint, DraftSplice, MutationRefusal,
};

pub mod editing {
    pub use semio_s_artifact_stdio_contract_editing::{
        admit_bounded_native_command, ample_close_grant, bounded_native_edit_bounded_first_step_proofs, build_bounded_native_edit_tool_job, build_snapshot_edit_tool_job, bytes, close_revision_turn, edited_subtree, fixture_mounted_owner_policy,
        is_snapshot_edit_action, raster, register_bounded_native_edit_tool_factory, register_snapshot_edit_tool_factory, revision_close_demand, routed_native_edit_preparation_factory, routed_native_edit_preparation_factory_birth_bytes, rules,
        snapshot_edit_actions, snapshot_edit_actions_with, snapshot_edit_bounded_first_step_proofs, snapshot_edit_event_from_action, snapshot_edit_execution_contract, snapshot_edit_source_is_admitted, snapshot_edit_value_is_admitted,
        snapshot_editing_command_from_action, snapshot_editing_command_id, snapshot_from_edit_source, validate_snapshot_schema_for_dialect, validate_snapshot_value_against_schema, ArtifactPreparationFactory, BoundedNativeEditToolJobFactory,
        BoundedNativeEditingEditor, Carried, EditPlan, EditRules, EntityRule, InsertRule, ItemField, NativeEditPreparationRoute, RemoveRule, RetainedBytesCopy, RetainedTextCopy, RowKey, Selector, SnapshotEditError, SnapshotEditEvent,
        SnapshotEditToolJobFactory, SnapshotEditingCommand, SnapshotEditingEditor, SnapshotEditingNativeCommand, INSERT_SNAPSHOT_VALUE_ACTION_ID, MOVE_SNAPSHOT_VALUE_ACTION_ID, REMOVE_SNAPSHOT_VALUE_ACTION_ID, RENAME_SNAPSHOT_KEY_ACTION_ID,
        REPLACE_SNAPSHOT_SOURCE_ACTION_ID, SET_SNAPSHOT_VALUE_ACTION_ID, SNAPSHOT_EDIT_ACTION_IDS, SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES,
    };
    pub use semio_s_artifact_stdio_contract_editing_details as details;
    pub use semio_s_artifact_stdio_contract_editing_details::{
        render_file_source_editor, render_snapshot_details, render_snapshot_details_provider_revisioned, snapshot_details_split_layout, snapshot_details_window_definition, DslSnapshotDetailsProvider, SnapshotDetailPathSegment,
        SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailsProvider, SNAPSHOT_DETAILS_BODY_KEY, SNAPSHOT_DETAILS_WINDOW_KIND_ID,
    };
    pub use semio_s_artifact_stdio_contract_editing_patch as patch;
    pub use semio_s_artifact_stdio_contract_editing_patch::{
        apply_snapshot_patch, apply_snapshot_patch_checked, apply_snapshot_patch_for_dialect, inverse_snapshot_patch, inverse_snapshot_patches, inverse_snapshot_patches_within, prepare_snapshot_patch, snapshot_patch_from_bytes,
        snapshot_patch_from_hex, snapshot_patch_from_text, snapshot_patch_hex, snapshot_patch_input_schema, snapshot_patch_input_schema_text, snapshot_patch_label, snapshot_patch_text, snapshot_schema_location, SnapshotPatch, SnapshotSchemaLocation,
        SnapshotSchemaResolver, SNAPSHOT_PATCH_MAX_BYTES, SNAPSHOT_PATCH_MAX_INVERSE_PARTS, SNAPSHOT_PATCH_MAX_SEGMENTS,
    };
    pub use semio_s_artifact_stdio_contract_wire::snapshot_edit_source;
}
