//! 📦️ Explicit public schema registry surface from the defining component.

#[path = "../../🦀️.rs"]
mod component;

pub use component::{
    assembly,
    AppSchemaDescriptor, AppSchemaRegistry, ArtifactInferenceDescriptor, ArtifactInferenceRegistry,
    ArtifactSchemaDescriptor, ArtifactSchemaRegistry, FacetLeaves, SchemaDescriptorRegistryError,
    SchemaExport, SchemaExportEntries, SchemaExportEntry, SchemaExportRegistry, SchemaExportRegistryError,
    SchemaFormat, SchemaResolveError, ScopeSchemaExports, RESERVED_FACET_EXPORT_IDS,
    app_schema_catalog_len, app_schema_descriptor_registered, artifact_inference_catalog_len,
    artifact_inference_descriptor_registered, artifact_schema_catalog_len, artifact_schema_descriptor_registered,
    preflight_app_schema_descriptors, preflight_artifact_inference_descriptors, preflight_artifact_schema_descriptors,
    register_app_schema_descriptor, register_app_schema_descriptors, register_artifact_inference_descriptor,
    register_artifact_inference_descriptors, register_artifact_schema_descriptor, register_artifact_schema_descriptors,
    register_referenced_schema_documents, register_scope_facet_leaves, register_scope_schema_exports,
    registered_referenced_schema_documents, resolve_schema_export, schema_export_catalog_entries,
    scope_schema_exports_registered, scope_schema_facets_registered, with_app_schema_catalog,
    with_app_schema_registry, with_artifact_inference_catalog, with_artifact_inference_registry,
    with_artifact_schema_catalog, with_artifact_schema_registry, with_schema_export_registry,
};
