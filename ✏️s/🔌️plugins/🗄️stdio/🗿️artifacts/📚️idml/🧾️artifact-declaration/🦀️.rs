//! 🧾️ IDML publishes its own schema, native event codecs, format and contribution.
use semio_framework_plugin::{ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, PluginAssemblyError};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../📜️artifact-definition.json");

/// 📐️ Builds the exact schema-owned capabilities with executable native factories.
pub fn definition() -> Result<ArtifactDefinition, PluginAssemblyError> {
    let factories = native_codecs();
    let mut executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    executables.push(semio_s_artifact_stdio_contract::ArtifactExecutable::from_function_pointer("s.stdio.idml.mutation.replace-package.v1", execute_replace_package as *const ()));
    executables.push(semio_s_artifact_stdio_contract::ArtifactExecutable::from_function_pointer("s.stdio.idml.mutation.replace-part.v1", execute_replace_part as *const ()));
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

/// 🎞️ Declares the IDML extension and MIME independently of ZIP's own artifact identity.
pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> semio_framework_os_kernel::os_store::serialization::ArtifactCodec {
    let mut codec = semio_framework_os_kernel::bare_artifact_codec_with_owners::<crate::IdmlSnapshot, crate::IdmlMutation,crate::issuer::IdmlCodecOwners>(crate::DOCUMENT_SCHEMA);
    codec.extension = "idml";
    codec
}

/// 🏭️ Owns the actual IDML snapshot and mutation factory used by caller-authored assembly.
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.idml.v1", artifact: "idml", kind: artifact_kind, codec: native_codec }]
}

/// 📇️ Exposes the headless package kind with both explicit language descriptors.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: crate::ARTIFACT_KIND.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("InDesign Package", "InDesign-Paket"),
        source_format: crate::DOCUMENT_SCHEMA.into(), component_kind: "stdio".into(), dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: crate::DOCUMENT_SCHEMA.into(), export_formats: vec![], import_formats: vec![], export_stdio_kinds: vec![], import_stdio_kinds: vec![],
    }
}

/// 🔖️ Registers the four-facet descriptor and real IDML event codec under its own dialect.
pub fn declaration(definition: ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
    ArtifactDeclaration::builder(definition)
        .schema(crate::schema::descriptor::schema_descriptor())
        .formats(formats()?)
        .document_codec_bare_with_owners::<crate::IdmlSnapshot, crate::IdmlMutation,crate::issuer::IdmlCodecOwners>(crate::DOCUMENT_SCHEMA, Dialect { artifact_kind: crate::ARTIFACT_KIND, standard: StandardId(crate::STANDARD), subset: SubsetId("*") })
        .try_build()
}

/// 🧩️ Returns a runtime declaration for a caller-selected artifact contribution.
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("idml", definition()?, declaration)
}

/// 📦️ Keeps package selection explicit while publishing all owned definition and codec factories.
pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "idml", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

fn execute_replace_package(base: &crate::IdmlSnapshot, document: crate::IdmlSnapshot) -> semio_framework_os_kernel::MutationOutcome<crate::IdmlDiff> {
    semio_framework_os_kernel::ColdMutation::diff(&crate::IdmlMutation::ReplacePackage { document }, base)
}

fn execute_replace_part(base: &crate::IdmlSnapshot, path: String, bytes: Vec<u8>) -> semio_framework_os_kernel::MutationOutcome<crate::IdmlDiff> {
    semio_framework_os_kernel::ColdMutation::diff(&crate::IdmlMutation::ReplacePart { path, bytes }, base)
}
