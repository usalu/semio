//! 🎪 `stdio.csv` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::csv::schema::diff::CsvDiff;
pub use crate::artifacts::csv::schema::mutations::CsvMutation;
pub use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
pub use crate::artifacts::csv::schema::CsvArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_CSV_DOCUMENT_SCHEMA: &str = "stdio.csv";

/// 🧬️ Artifact schema descriptor id.
pub const CSV_ARTIFACT_SCHEMA_ID: &str = "s.stdio.csv";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.csv".into(),
        name: "Csv".into(),
        source_format: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling an imperative `register()` before `Plugin::builder` was even
/// constructed, mirroring the `🔋️energy`/`🗒️note` exemplars. Call order, in `.builder()` order below:
/// `.composers(...)` from `standards::v_rfc4180::subsets::any::io::io_registry::entries()` (dissolved
/// out of the former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES);
/// `.schema(...)`/`.inferences(...)`; `.languages(...)` from `pilot_languages()` below (same
/// `OnceLock`-leak shape `🔋️energy`'s own `pilot_languages()` uses, since `dsl::LanguageSpec` isn't
/// `const fn`-constructible); `.document_codec_bare::<CsvSnapshot, CsvMutation>(...)`. Unlike
/// `📄txt`/`💾️binary`, this artifact's declaration never calls `register_schema_specs` —
/// `CsvSnapshot`/`CsvDiff` don't carry the `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]`
/// `register_schema_specs` needs, per txt's own doc ("unlike json/csv...") — so there is no
/// uncovered call left behind here.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("csv", definition, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("csv")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::csv::schema::csv_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::inferences::csv_artifact_inference_descriptor()])
        .composers(crate::artifacts::csv::standards::v_rfc4180::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<CsvSnapshot, CsvMutation>(STDIO_CSV_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of `standards::v_rfc4180::subsets::
/// any::engine::register_pilot_languages()`'s five `LanguageSpec`s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.csv",
                    extension: Some("csv"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.csv"),
                },
                dsl::LanguageSpec {
                    id: "stdio.csv.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.csv.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.csv.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.csv.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.csv.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.csv.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.csv.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.csv.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::csv::standards::v_rfc4180::subsets::any::io::io_registry as v_rfc4180;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc4180::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("CsvComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_composer_entries(v_rfc4180::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
