#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.ifc` artifact — stdio reference format.
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};


/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📜 The schema-owned definition: one kind, two independently versioned standards (`4`, `2x3`) — see [`declaration`].
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "ifc", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("ifc", definition()?, declaration)
}

/// 🧾️ The runtime `ifc` declares for both standards (`4`, `2x3`, independently versioned ids): schemas, format, inference
/// descriptors, composers, document codecs, and the `2x3` model-view-definition validators (`cv20`, `sav`, `cobie`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_stdio_ifc_4_descriptor::standards::v4::subsets::any::schema::descriptor::ifc_artifact_schema_descriptor())
        .schemas([semio_s_artifact_stdio_ifc_2x3_base_descriptor::standards::v2x3::subsets::base::schema::descriptor::ifc2x3_artifact_schema_descriptor()])
        .formats(formats()?)
        .inferences([semio_s_artifact_stdio_ifc_4_inferences::standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor(), semio_s_artifact_stdio_ifc_2x3_base_inferences::standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor()])
        .composers(semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::io_registry::entries())
        .composers(semio_s_artifact_stdio_ifc_2x3_base::standards::v2x3::subsets::base::io::io_registry::entries())
        .composers(io_registry::mvd_entries())
        .document_codec_bare::<semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot, semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::STDIO_IFC_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("4"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("cv20") })
        .document_codec_bare::<semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("sav") })
        .document_codec_bare::<semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("cobie") });
    let builder = semio_s_artifact_stdio_ifc_2x3_cv20::standards::v2x3::subsets::cv20::io::declare(builder);
    let builder = semio_s_artifact_stdio_ifc_2x3_sav::standards::v2x3::subsets::sav::io::declare(builder);
    semio_s_artifact_stdio_ifc_2x3_cobie::standards::v2x3::subsets::cobie::io::declare(builder).try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.ifc".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Ifc", "Ifc"),
        source_format: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::STDIO_IFC_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::STDIO_IFC_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use semio_s_artifact_stdio_ifc_2x3_base::standards::v2x3::subsets::base::io::io_registry as v2x3;
    use semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::io_registry as v4;
    use semio_s_artifact_stdio_ifc_2x3_cobie::standards::v2x3::subsets::cobie::io::Ifc2x3CobieComposer;
    use semio_s_artifact_stdio_ifc_2x3_cv20::standards::v2x3::subsets::cv20::io::Ifc2x3Cv20Composer;
    use semio_s_artifact_stdio_ifc_2x3_sav::standards::v2x3::subsets::sav::io::Ifc2x3SavComposer;
    use semio_framework_plugin::composer_entry_of;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();
    static MVD: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    /// 🧩️ The composers of the three IFC2x3 model view definitions (`cv20`, `sav`, `cobie`), which extend the base
    /// subset and therefore register here, above it.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn mvd_entries() -> &'static [ComposerEntry] {
        MVD.get_or_init(|| vec![composer_entry_of::<Ifc2x3Cv20Composer>(), composer_entry_of::<Ifc2x3SavComposer>(), composer_entry_of::<Ifc2x3CobieComposer>()]).as_slice()
    }

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v4::entries().iter().chain(v2x3::entries().iter()).chain(mvd_entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("IfcComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v4::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v2x3::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(mvd_entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry





