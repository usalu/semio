//! 📇️ The full semio catalog: the aggregator's artifact with every subset conversion of the family declared, the
//! contribution the stdio hub selects for its full catalog.

/// 🔗️ Every conversion of the semio family, each subset's import before its export.
//#region 🔗️Conversions
pub const ALL: &[semio_s_artifact_stdio_semio_base::SemioConversion] = &[
    semio_s_artifact_stdio_semio_animation_import::CONVERSION,
    semio_s_artifact_stdio_semio_animation_export::CONVERSION,
    semio_s_artifact_stdio_semio_audio_import::CONVERSION,
    semio_s_artifact_stdio_semio_audio_export::CONVERSION,
    semio_s_artifact_stdio_semio_brep_import::CONVERSION,
    semio_s_artifact_stdio_semio_brep_export::CONVERSION,
    semio_s_artifact_stdio_semio_cad_import::CONVERSION,
    semio_s_artifact_stdio_semio_cad_export::CONVERSION,
    semio_s_artifact_stdio_semio_document_import::CONVERSION,
    semio_s_artifact_stdio_semio_document_export::CONVERSION,
    semio_s_artifact_stdio_semio_drawing_import::CONVERSION,
    semio_s_artifact_stdio_semio_drawing_export::CONVERSION,
    semio_s_artifact_stdio_semio_flow_import::CONVERSION,
    semio_s_artifact_stdio_semio_flow_export::CONVERSION,
    semio_s_artifact_stdio_semio_image_import::CONVERSION,
    semio_s_artifact_stdio_semio_image_export::CONVERSION,
    semio_s_artifact_stdio_semio_mesh_import::CONVERSION,
    semio_s_artifact_stdio_semio_mesh_export::CONVERSION,
    semio_s_artifact_stdio_semio_model_import::CONVERSION,
    semio_s_artifact_stdio_semio_model_export::CONVERSION,
    semio_s_artifact_stdio_semio_presentation_import::CONVERSION,
    semio_s_artifact_stdio_semio_presentation_export::CONVERSION,
    semio_s_artifact_stdio_semio_value_import::CONVERSION,
    semio_s_artifact_stdio_semio_value_export::CONVERSION,
    semio_s_artifact_stdio_semio_video_import::CONVERSION,
    semio_s_artifact_stdio_semio_video_export::CONVERSION,
];
//#endregion 🔗️Conversions

/// 🧾️ The semio artifact declared with every conversion of the family.
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_semio::declaration(definition, ALL)
}

/// 🗂️ The runtime assembly of the full semio artifact.
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("semio", semio_s_artifact_stdio_semio::definition()?, declaration)
}

/// 🧵️ The native codec factories of the full catalog; semio publishes none.
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}

/// 📇️ The full-catalog contribution of the semio artifact.
pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "semio", schema: semio_s_artifact_stdio_semio::ARTIFACT_DEFINITION_SCHEMA, definition: semio_s_artifact_stdio_semio::definition, assembly, formats: semio_s_artifact_stdio_semio::formats, native_codecs }
}
