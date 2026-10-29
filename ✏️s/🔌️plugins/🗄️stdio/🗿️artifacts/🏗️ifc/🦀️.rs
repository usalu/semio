//! 🎪 `stdio.ifc` artifact — stdio reference format: the schema-owned definition, linking no IFC standard.

/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📜 The schema-owned definition: one kind, two independently versioned standards (`4`, `2x3`).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

/// 🗂️ The formats its owned schema declares.
pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

/// 🧵️ The native codec factories of the artifact; ifc publishes none.
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}
