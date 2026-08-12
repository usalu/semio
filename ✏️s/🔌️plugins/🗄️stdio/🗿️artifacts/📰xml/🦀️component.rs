//! 🎪 `stdio.xml` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::xml::schema::snapshot::XmlSnapshot;
pub use crate::artifacts::xml::schema::XmlArtifact;
pub use crate::artifacts::xml::schema::diff::XmlDiff;
pub use crate::artifacts::xml::schema::mutations::XmlMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XML_DOCUMENT_SCHEMA: &str = "stdio.xml";

/// 🧬️ Artifact schema descriptor id.
pub const XML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xml";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.xml".into(),
        name: "Xml".into(),
        source_format: STDIO_XML_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling `crate::artifacts::xml::engine::register()` imperatively before
/// `Plugin::builder` was even constructed, mirroring the `🔋️energy`/`🗒️note` exemplars and this
/// artifact's own `🔣️json` sibling (same shape: an `✳️any` + one dependent subset). `crate::
/// artifacts::xml::standards::v1_0::subsets::any::engine::register()` (stdio's own `⚙️engine` —
/// UNTOUCHED, per this ticket's rule that stdio's engines stay a public surface other plugins reach
/// into) called, in call order: `io_registry::register()` → `.composers(...)` below, the same
/// `standards::v1_0::subsets::any::engine::io_registry::entries()` this artifact's own root
/// `io_registry` module already wraps — that list already carries BOTH the `✳️any` raw composer and
/// the `✳️valid` composer, so `.composers()` alone covers both; `register_artifact_schema()`/
/// `register_artifact_inferences()` → `.schema(...)`/`.inferences(...)`; `register_pilot_languages()`
/// → `.languages(...)`, replicated verbatim below (same `OnceLock`-leak shape `🔋️energy`'s own
/// `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const fn`-constructible);
/// `register_document_codec` → `.document_codec_bare::<XmlSnapshot, XmlMutation>(...)`; and
/// `crate::artifacts::xml::standards::v1_0::subsets::valid::io::register()` — the ✳️valid subset's
/// own `register_subset_validator` call, living in `🚪️io/` (not `⚙️engine/`, so freely editable) →
/// `.subset_validators(...)` below, built fresh via
/// `subset_validator_entry_of::<XmlValidValidator>()` rather than reaching into that module's private
/// `validator_entry()` OnceLock (left untouched — same pattern `🔣️json`'s `declaration()` establishes
/// for this field). `standards::v1_0::subsets::any::engine::register()` itself is left in place, now
/// orphaned/uncalled — deleting it means editing `⚙️engine/`, off-limits here.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.stdio.xml")
        .schema(crate::artifacts::xml::schema::xml_artifact_schema_descriptor())
        .inferences([crate::artifacts::xml::standards::v1_0::subsets::any::schema::inferences::xml_artifact_inference_descriptor()])
        .composers(crate::artifacts::xml::standards::v1_0::subsets::any::engine::io_registry::entries())
        .subset_validators(pilot_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<XmlSnapshot, XmlMutation>(STDIO_XML_DOCUMENT_SCHEMA)
        .build()
}

/// 🛡️ The ✳️valid subset's `SubsetValidatorEntry`, built once — see `declaration()`'s own doc for why
/// this is a fresh `subset_validator_entry_of::<XmlValidValidator>()` call rather than a reuse of
/// `subsets::valid::io::derived_composition`'s private `validator_entry()`.
fn pilot_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::xml::standards::v1_0::subsets::valid::io::XmlValidValidator>()])
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of `standards::v1_0::subsets::any::
/// engine::register_pilot_languages()`'s five `LanguageSpec`s.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.xml",
                    extension: Some("xml"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::xml::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xml::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xml"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xml.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::xml::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xml::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::xml::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xml::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xml.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xml.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::xml::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::xml::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.xml.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xml.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xml.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xml.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::xml::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::xml::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xml.spr"),
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
    use crate::artifacts::xml::standards::v1_0::subsets::any::engine::io_registry as v1_0;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("XmlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
