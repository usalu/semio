//! 🎪 `stdio.las` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::las::schema::diff::LasDiff;
pub use crate::artifacts::las::schema::mutations::LasMutation;
pub use crate::artifacts::las::schema::snapshot::LasSnapshot;
pub use crate::artifacts::las::schema::LasArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_LAS_DOCUMENT_SCHEMA: &str = "stdio.las";

/// 🧬️ Artifact schema descriptor id.
pub const LAS_ARTIFACT_SCHEMA_ID: &str = "s.stdio.las";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::artifacts::las::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<LasSnapshot, LasMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows) by its full path through the `engine` shim (`🦀️.rs`'s `pub mod engine { pub use
/// super::standards::v1_0::engine::*; }`) — deliberately NOT this file's own `io_registry` module
/// below, whose `entries()` returns `&'static [&'static ComposerEntry]` (references) and would
/// silently rebind under a bare call (this ticket's "SILENT REBIND" hazard).
///
/// **NOT covered by any `ArtifactDeclaration` field**: the engine's `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, a registry distinct from `.languages()`'s
/// `dsl::register_language` — same gap `🗜️deflate`'s own exemplar documents). No field here closes
/// it, so it is not invented and the call is not dropped — it must survive on the plugin root's
/// `.setup(crate::artifacts::las::engine::register_schema_specs)` alongside this declaration's
/// `.artifact(...)`, exactly this ticket's own W1d precedent (puzzle's B2 OS-media-bridge case) for
/// a genuine, narrowly scoped registration gap.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("las", definition, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("las")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::las::schema::las_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::las::schema::inferences::las_artifact_inference_descriptor()])
        .composers(crate::artifacts::las::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<LasSnapshot, LasMutation>(STDIO_LAS_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::artifacts::las::standards::v1_0::
/// engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.las",
                    extension: Some("las"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::las::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::las::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.las"),
                },
                dsl::LanguageSpec {
                    id: "stdio.las.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::las::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::las::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::las::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::las::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.las.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.las.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::las::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::las::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.las.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.las.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.las.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.las.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::las::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::las::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.las.spr"),
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
        id: "stdio.las".into(),
        name: "Las".into(),
        source_format: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::las::standards::v1_0::engine::io_registry as v1_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("LasComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_composer_entries(v1_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
