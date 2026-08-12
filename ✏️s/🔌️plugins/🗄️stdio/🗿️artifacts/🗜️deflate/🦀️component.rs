//! 🎪 `stdio.deflate` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::deflate::schema::snapshot::DeflateSnapshot;
pub use crate::artifacts::deflate::schema::DeflateArtifact;
pub use crate::artifacts::deflate::schema::diff::DeflateDiff;
pub use crate::artifacts::deflate::schema::mutations::DeflateMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DEFLATE_DOCUMENT_SCHEMA: &str = "stdio.deflate";

/// 🧬️ Artifact schema descriptor id.
pub const DEFLATE_ARTIFACT_SCHEMA_ID: &str = "s.stdio.deflate";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::artifacts::deflate::engine::register()`, which the
/// plugin root called unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🔋️energy`'s `s.model` exemplar exactly: a headless library artifact with zero `ArtifactApp`s,
/// so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<DeflateSnapshot,
/// DeflateMutation>(...))`. `.composers(...)` reaches the ENGINE's own `io_registry` (returns
/// `&'static [ComposerEntry]`, owned rows) by its full path through the `engine` shim
/// (`📦️glue.rs`'s `pub mod engine { pub use super::standards::v_rfc1950::engine::*; }`) —
/// deliberately NOT this file's own `io_registry` module below, whose `entries()` returns
/// `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare call
/// (see this ticket's "SILENT REBIND" hazard).
///
/// **NOT covered by any `ArtifactDeclaration` field**: the engine's `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, the P2-M3 `FullResolver` insertion API — a registry
/// distinct from `.languages()`'s `dsl::register_language`). No field here closes it, so it is not
/// invented and the call is not dropped — it survives on the plugin root's `.setup(...)`, exactly
/// this ticket's own W1d precedent (puzzle's B2 OS-media-bridge case) for a genuine, narrowly
/// scoped registration gap.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(DEFLATE_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::deflate::schema::deflate_artifact_schema_descriptor())
        .inferences([crate::artifacts::deflate::schema::inferences::deflate_artifact_inference_descriptor()])
        .composers(crate::artifacts::deflate::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<DeflateSnapshot, DeflateMutation>(STDIO_DEFLATE_DOCUMENT_SCHEMA)
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::artifacts::deflate::engine::
/// register_pilot_languages`'s own `dsl::register_language(...)` call bodies — same ids, same
/// grammar/protocol paths, same `passthrough_hooks` calls.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.deflate",
                    extension: Some("zz"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.deflate"),
                },
                dsl::LanguageSpec {
                    id: "stdio.deflate.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::deflate::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::deflate::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.deflate.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.deflate.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::deflate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::deflate::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.deflate.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.deflate.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.deflate.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.deflate.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.deflate.spr"),
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
        id: "stdio.deflate".into(),
        name: "Deflate".into(),
        source_format: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::deflate::standards::v_rfc1950::engine::io_registry as v_rfc1950;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc1950::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("DeflateComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_rfc1950::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
