//! 🎪 `stdio.json` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::json::schema::diff::JsonDiff;
pub use crate::artifacts::json::schema::mutations::JsonMutation;
pub use crate::artifacts::json::schema::snapshot::JsonSnapshot;
pub use crate::artifacts::json::schema::JsonArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JSON_DOCUMENT_SCHEMA: &str = "stdio.json";

/// 🧬️ Artifact schema descriptor id.
pub const JSON_ARTIFACT_SCHEMA_ID: &str = "s.stdio.json";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.json".into(),
        name: "Json".into(),
        source_format: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
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
/// `.composers(...)` from `standards::v_rfc8259::subsets::any::io::io_registry::entries()` (dissolved
/// out of the former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — that
/// list already carries BOTH the `✳️any` raw composer and the `✳️i-json` composer, so `.composers()`
/// alone covers both); `.schema(...)`/`.inferences(...)` from the schema/inferences descriptors
/// directly; `.languages(...)` from `pilot_languages()` below (same `OnceLock`-leak shape `🔋️energy`'s
/// own `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const fn`-constructible);
/// `.document_codec_bare::<JsonSnapshot, JsonMutation>(...)`; and
/// `crate::artifacts::json::standards::v_rfc8259::subsets::i_json::io::register()` — the ✳️i-json
/// subset's own `register_subset_validator` call, living in `🚪️io/` → `.subset_validators(...)`
/// below, built fresh via `subset_validator_entry_of::<JsonIJsonValidator>()` rather than reaching
/// into that module's private `validator_entry()` OnceLock (left untouched — first artifact in the
/// repo to populate this field, see its own builder doc).
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("json", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::json::schema::json_artifact_schema_descriptor())
        .inferences([crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::inferences::json_artifact_inference_descriptor()])
        .composers(crate::artifacts::json::standards::v_rfc8259::subsets::any::io::io_registry::entries())
        .subset_validators(pilot_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The ✳️i-json subset's `SubsetValidatorEntry`, built once — see `declaration()`'s own doc for
/// why this is a fresh `subset_validator_entry_of::<JsonIJsonValidator>()` call rather than a reuse
/// of `subsets::i_json::io::derived_composition`'s private `validator_entry()`.
fn pilot_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::json::standards::v_rfc8259::subsets::i_json::io::JsonIJsonValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of
/// `standards::v_rfc8259::subsets::any::engine::register_pilot_languages()`'s five `LanguageSpec`s.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.json",
                    extension: Some("json"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.json"),
                },
                dsl::LanguageSpec {
                    id: "stdio.json.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::json::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::json::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.json.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.json.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::json::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::json::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.json.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.json.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.json.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.json.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.json.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::io::io_registry as v_rfc8259;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc8259::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("JsonComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_rfc8259::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
