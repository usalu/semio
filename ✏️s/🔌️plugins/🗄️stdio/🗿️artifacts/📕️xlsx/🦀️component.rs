//! 🎪 `stdio.xlsx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::xlsx::schema::snapshot::XlsxSnapshot;
pub use crate::artifacts::xlsx::schema::XlsxArtifact;
pub use crate::artifacts::xlsx::schema::diff::XlsxDiff;
pub use crate::artifacts::xlsx::schema::mutations::XlsxMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XLSX_DOCUMENT_SCHEMA: &str = "stdio.xlsx";

/// 🧬️ Artifact schema descriptor id.
pub const XLSX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xlsx";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::artifacts::xlsx::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::register_document_codec(store::ArtifactCodec::of::<XlsxSnapshot,
/// XlsxMutation>(...))` call. `.composers(...)` reaches the engine's own `io_registry` (through the
/// `engine` shim), whose `entries()` already aggregates the `✳️any`/`✳️strict`/`✳️transitional`
/// `ComposerEntry` rows — NOT this file's own shadowing `io_registry` below, whose `entries()`
/// returns `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare
/// call (this ticket's "SILENT REBIND" hazard). `.subset_validators(...)` re-derives the two
/// `SubsetValidatorEntry` rows the old `register()`'s `✳️strict`/`✳️transitional` `io::register()`
/// calls used to install, via the same side-effect-free `subset_validator_entry_of::<V>()`
/// constructor those (module-private) `validator_entry()` fns call — no visibility widening into
/// `🚪️io/` needed.
///
/// **NOT covered by any field**: nothing — xlsx's `register()` never called `register_schema_spec`
/// (`XlsxSnapshot`/`XlsxDiff`/`XlsxMutation` are hand-rolled, no derivable `RecordSpec` — see the
/// deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(XLSX_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::xlsx::schema::xlsx_artifact_schema_descriptor())
        .inferences([crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::inferences::xlsx_artifact_inference_descriptor()])
        .composers(crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::io_registry::entries())
        .subset_validators(xlsx_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA)
        .build()
}

/// 🛡️ The `✳️strict`/`✳️transitional` subsets' `SubsetValidatorEntry` rows, re-derived (not moved)
/// from the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️component.rs` (module-private) `validator_entry()` calls.
fn xlsx_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<
                    crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::io::XlsxStrictValidator,
                >(),
                semio_framework_plugin::subset_validator_entry_of::<
                    crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::io::XlsxTransitionalValidator,
                >(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from `crate::artifacts::xlsx::engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.xlsx",
                    extension: Some("xlsx"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::xlsx::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xlsx::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::xlsx::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xlsx::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::xlsx::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xlsx::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.xlsx.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.spr"),
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
        id: "stdio.xlsx".into(),
        name: "Xlsx".into(),
        source_format: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::io_registry as v_ecma_376;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ecma_376::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("XlsxComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ecma_376::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
