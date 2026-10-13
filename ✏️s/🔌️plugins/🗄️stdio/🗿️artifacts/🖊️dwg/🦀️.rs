//! 🎪 `stdio.dwg` artifact — stdio reference format: the definition, declaration and registration over the model crate
//! `semio-s-artifact-stdio-dwg-model`. The reader, writer and editor crates (`-dwg-read`, `-dwg-write`, `-dwg-editor`)
//! are named directly by the consumers that need them.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::mutations::DwgMutation;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::STDIO_DWG_DOCUMENT_SCHEMA;

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA);
    codec.extension = "dwg";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.dwg.v1", artifact: "dwg", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "dwg", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration: the ac1024 schema, inference, languages and document codecs, plus the
/// composer entries of both standards (`ac1018` and `ac1024`), which are the only ac1018 registrations that
/// are live (other plugins' composer entries target `StandardId("ac1018")` directly). The ac1024 schema's
/// `register_schema_specs()` has no declaration field and stays on the plugin root's `.setup(..)`.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("dwg", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::dwg_artifact_schema_descriptor())
        .formats(formats)
        .inferences([semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::inferences::dwg_artifact_inference_descriptor()])
        .composers(dwg_combined_composer_entries())
        .languages(pilot_languages())
        .document_codec_bare::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.dwg", standard: semio_framework_artifact_reference::StandardId("ac1018"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.dwg", standard: semio_framework_artifact_reference::StandardId("ac1024"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .try_build()
}

/// 🎹️ `ac1018` + `ac1024` composer entries as one owned `&'static [ComposerEntry]` (both registries own
/// their rows; `ComposerEntry` fields are all `Copy`, so the rebuild is lossless).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_combined_composer_entries() -> &'static [semio_framework_plugin::io::ComposerEntry] {
    use semio_framework_plugin::io::ComposerEntry;
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| semio_s_artifact_stdio_dwg_model::standards::v_ac1018::subsets::any::io::io_registry::entries().iter().chain(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::io_registry::entries().iter()).map(|e| ComposerEntry { writes: e.writes, reads: e.reads, compose: e.compose }).collect())
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dwg",
                    extension: Some("dwg"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dwg"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dwg.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dwg.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dwg.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dwg.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dwg.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dwg.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dwg.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dwg.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.dwg".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Dwg", "Dwg"),
        source_format: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1018::subsets::any::io::io_registry as v_ac1018;
    use semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::io_registry as v_ac1024;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ac1018::entries().iter().chain(v_ac1024::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("DwgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_ac1018::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v_ac1024::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry






#[cfg(test)]
#[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🗂️declaration/🦀️.rs"]
mod sqlite_snapshot_declaration_tests;
