//! 🎪 `stdio.svg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::svg::schema::snapshot::SvgSnapshot;
pub use crate::artifacts::svg::schema::SvgArtifact;
pub use crate::artifacts::svg::schema::diff::SvgDiff;
pub use crate::artifacts::svg::schema::mutations::SvgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SVG_DOCUMENT_SCHEMA: &str = "stdio.svg";

/// 🧬️ Artifact schema descriptor id.
pub const SVG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.svg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.svg".into(),
        name: "Svg".into(),
        source_format: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::artifacts::svg::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// rows — ✳️any + ✳️tiny + ✳️basic already folded into one list there) by its FULLY QUALIFIED path,
/// never the bare `io_registry::entries()` shortcut that would silently rebind to this file's own
/// shadowing `io_registry` module below (repo-wide "silent rebind" hazard this ticket tracks —
/// that module returns `&[&ComposerEntry]`, a different type, and is left in place as orphaned
/// dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). Both subset dialects'
/// `SubsetValidator`s (`✳️tiny`/`✳️basic`'s own `SvgTinyValidator`/`SvgBasicValidator`, previously
/// registered via `⚙️engine::register()`'s two trailing `subsets::{tiny,basic}::io::register()`
/// calls) are re-derived here via `subset_validator_entry_of::<…>()` rather than reused from those
/// files' own private `validator_entry()` caches (neither is `pub`) — same erasure helper, fresh
/// instances, same registry effect. `⚙️engine` itself is untouched — this only REFERENCES what it
/// already exposes.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(SVG_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::svg::standards::v1_1::subsets::any::schema::svg_artifact_schema_descriptor())
        .inferences([crate::artifacts::svg::standards::v1_1::subsets::any::schema::inferences::svg_artifact_inference_descriptor()])
        .composers(crate::artifacts::svg::standards::v1_1::engine::io_registry::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA)
        .build()
}

/// 🛡️ Re-derives the ✳️tiny and ✳️basic subsets' `SubsetValidatorEntry`s — see `declaration()`'s own
/// doc for why this calls `subset_validator_entry_of` directly instead of reusing either subset's
/// own private cache.
fn declared_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<
                    crate::artifacts::svg::standards::v1_1::subsets::tiny::io::SvgTinyValidator,
                >(),
                semio_framework_plugin::subset_validator_entry_of::<
                    crate::artifacts::svg::standards::v1_1::subsets::basic::io::SvgBasicValidator,
                >(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.svg",
                    extension: Some("svg"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.svg"),
                },
                dsl::LanguageSpec {
                    id: "stdio.svg.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.svg.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.svg.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.svg.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.svg.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.svg.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.svg.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.svg.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::svg::standards::v1_1::engine::io_registry as v1_1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("SvgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
