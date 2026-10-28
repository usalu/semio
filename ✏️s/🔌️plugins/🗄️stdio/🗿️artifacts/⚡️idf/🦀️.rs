//! ⚡️ EnergyPlus 25.2 IDF format identity, owned by Stdio and composed by the energy model.

pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 🪪️ Validates the authored IDF identity through the shared Stdio contract.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

/// 📄️ Publishes the canonical `.idf` extension and EnergyPlus revision to format selectors.
pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

/// 🧩️ IDF has a format definition; energy owns the typed physical-model conversion.
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_only_assembly("idf", definition()?)
}

/// 📇️ Allows a deployment to select the format without claiming a generic document editor.
pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution {
        identity: "idf", schema: ARTIFACT_DEFINITION_SCHEMA, definition_constraint: None,
        definition, assembly, formats, native_codecs: Vec::new,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn official_format_definition_validates_and_exposes_idf() {
        super::definition().unwrap();
        let formats = super::formats().unwrap();
        assert!(!formats.is_empty());
    }
}
