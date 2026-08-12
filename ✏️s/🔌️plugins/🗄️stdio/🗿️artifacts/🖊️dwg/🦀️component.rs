//! 🎪 `stdio.dwg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::dwg::schema::snapshot::DwgSnapshot;
pub use crate::artifacts::dwg::schema::snapshot::{DwgDecodeStatus, DwgSection, DwgSectionPage};
pub use crate::artifacts::dwg::schema::DwgArtifact;
pub use crate::artifacts::dwg::schema::diff::DwgDiff;
pub use crate::artifacts::dwg::schema::mutations::DwgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DWG_DOCUMENT_SCHEMA: &str = "stdio.dwg";

/// 🧬️ Artifact schema descriptor id.
pub const DWG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dwg";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::artifacts::dwg::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. `dwg` is
/// TWO standards (`ac1018`, `ac1024`), but `crate::artifacts::dwg::engine` (the `📦️glue.rs` shim,
/// `pub use super::standards::v_ac1024::engine::*;`) is a PLAIN glob re-export of ac1024 ALONE —
/// unlike `ifc`'s own shim (which locally overrides `register()` to call both standards
/// explicitly), dwg's shim never calls `v_ac1018::engine::register()` at all. Repo-wide grep
/// confirms zero callers of that ac1018 free fn anywhere — ac1018's own schema/inference/
/// languages/codec registrations are genuinely dead code today (ac1018 was superseded by real
/// R2004+ decode per Decision #5; the shim's own doc comment says so). So this declaration mirrors
/// `crate::artifacts::dwg::engine::register()`'s ACTUAL (ac1024-only) schema/inference/languages/
/// codec exactly — nothing ac1018-only is dropped, because nothing ac1018-only was ever running.
///
/// **Composers are the one place both standards ARE live**: `crate::artifacts::dwg::engine::
/// register()`'s first line is `crate::artifacts::dwg::io_registry::register()` — THIS file's own
/// root `io_registry` module below, which unions `v_ac1018::engine::io_registry::entries()` AND
/// `v_ac1024::engine::io_registry::entries()`. `.composers()` needs one owned `&'static
/// [ComposerEntry]`, and that root `io_registry::entries()` returns `&'static [&'static
/// ComposerEntry]` (references — the SILENT REBIND shape this ticket warns about, and also the
/// wrong type for `.composers()` regardless). `dwg_combined_composer_entries()` below re-
/// materializes both engines' OWN owned `io_registry::entries()` (`&'static [ComposerEntry]` each,
/// fully qualified through `standards::v_ac1018`/`v_ac1024::engine::io_registry`, never through the
/// shim or the shadowing root module) into one new owned slice — same entries, same `writes`/
/// `reads`/`compose` fn pointers, just recombined to satisfy `.composers()`'s type. `ComposerEntry`
/// has no `#[derive(Clone)]` but every field (`Dialect: Copy`, `&'static [Dialect]`, `fn(...)  ->
/// ...`) is individually `Copy`, so this is a lossless field-for-field rebuild, not a fabrication.
///
/// **NOT covered by any `ArtifactDeclaration` field**: the ac1024 engine's `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, a registry distinct from `.languages()`'s `dsl::
/// register_language` — same gap `🗜️deflate`'s own exemplar documents). No field here closes it, so
/// it is not invented and the call is not dropped — it must survive on the plugin root's
/// `.setup(crate::artifacts::dwg::engine::register_schema_specs)` alongside this declaration's
/// `.artifact(...)`, exactly this ticket's own W1d precedent (puzzle's B2 OS-media-bridge case).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(DWG_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::dwg::schema::dwg_artifact_schema_descriptor())
        .inferences([crate::artifacts::dwg::schema::inferences::dwg_artifact_inference_descriptor()])
        .composers(dwg_combined_composer_entries())
        .languages(pilot_languages())
        .document_codec_bare::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA)
        .build()
}

/// 🎹️ `ac1018` + `ac1024` engine composer entries, re-materialized as one owned `&'static
/// [ComposerEntry]` — see `declaration()`'s own doc for why this exists instead of a bare
/// `.composers()` call.
fn dwg_combined_composer_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
    use semio_framework_plugin::ComposerEntry;
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            crate::artifacts::dwg::standards::v_ac1018::engine::io_registry::entries()
                .iter()
                .chain(crate::artifacts::dwg::standards::v_ac1024::engine::io_registry::entries().iter())
                .map(|e| ComposerEntry { writes: e.writes, reads: e.reads, compose: e.compose })
                .collect()
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role, INCLUDING the `Some("bin")` document
/// extension — not a typo, copied as-is) from `crate::artifacts::dwg::standards::v_ac1024::engine::
/// register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.dwg",
                    extension: Some("bin"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::dwg::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::dwg::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::dwg::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::dwg::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::dwg::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::dwg::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.dwg.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::dwg::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::dwg::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.spr"),
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
        id: "stdio.dwg".into(),
        name: "Dwg".into(),
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
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::dwg::standards::v_ac1018::engine::io_registry as v_ac1018;
    use crate::artifacts::dwg::standards::v_ac1024::engine::io_registry as v_ac1024;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ac1018::entries().iter().chain(v_ac1024::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("DwgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ac1018::entries());
        register_composer_entries(v_ac1024::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
