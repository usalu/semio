//! 🎪 `stdio.obj` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::obj::schema::diff::ObjDiff;
pub use crate::artifacts::obj::schema::mutations::ObjMutation;
pub use crate::artifacts::obj::schema::snapshot::ObjSnapshot;
pub use crate::artifacts::obj::schema::ObjArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_OBJ_DOCUMENT_SCHEMA: &str = "stdio.obj";

/// 🧬️ Artifact schema descriptor id.
pub const OBJ_ARTIFACT_SCHEMA_ID: &str = "s.stdio.obj";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::artifacts::obj::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<ObjSnapshot, ObjMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows) by its full path through the `engine` shim (`📦️glue.rs`'s `pub mod engine { pub use
/// super::standards::v3_0::engine::*; }`) — deliberately NOT this file's own `io_registry` module
/// below, whose `entries()` returns `&'static [&'static ComposerEntry]` (references) and would
/// silently rebind under a bare call (this ticket's "SILENT REBIND" hazard).
///
/// **NOT covered by any `ArtifactDeclaration` field**: the engine's `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, a registry distinct from `.languages()`'s
/// `dsl::register_language` — same gap `🗜️deflate`'s own exemplar documents). No field here closes
/// it, so it is not invented and the call is not dropped — it must survive on the plugin root's
/// `.setup(crate::artifacts::obj::engine::register_schema_specs)` alongside this declaration's
/// `.artifact(...)`, exactly this ticket's own W1d precedent (puzzle's B2 OS-media-bridge case) for
/// a genuine, narrowly scoped registration gap.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("obj", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("obj")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::obj::schema::obj_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::obj::schema::inferences::obj_artifact_inference_descriptor()])
        .composers(crate::artifacts::obj::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<ObjSnapshot, ObjMutation>(STDIO_OBJ_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::artifacts::obj::standards::v3_0::
/// engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.obj",
                    extension: Some("obj"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::obj::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::obj::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.obj"),
                },
                dsl::LanguageSpec {
                    id: "stdio.obj.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::obj::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::obj::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::obj::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::obj::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.obj.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.obj.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::obj::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::obj::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.obj.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.obj.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.obj.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.obj.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::obj::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::obj::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.obj.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.obj".into(),
        name: "Obj".into(),
        source_format: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::obj::standards::v3_0::engine::io_registry as v3_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v3_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("ObjComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v3_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
