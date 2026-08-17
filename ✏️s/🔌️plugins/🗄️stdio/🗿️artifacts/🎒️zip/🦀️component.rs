//! 🎪 `stdio.zip` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::zip::schema::diff::ZipDiff;
pub use crate::artifacts::zip::schema::mutations::ZipMutation;
pub use crate::artifacts::zip::schema::snapshot::ZipSnapshot;
pub use crate::artifacts::zip::schema::ZipArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_ZIP_DOCUMENT_SCHEMA: &str = "stdio.zip";

/// 🧬️ Artifact schema descriptor id.
pub const ZIP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.zip";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::artifacts::zip::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::register_document_codec(store::ArtifactCodec::of::<ZipSnapshot,
/// ZipMutation>(...))` call. `.composers(...)` reaches the subset IO module's own `io_registry`
/// (the former `⚙️engine`'s `io_registry`, dissolved into `standards::v2_0::subsets::any::io` per
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES), whose `entries()` already
/// aggregates BOTH the `✳️any` and `✳️iso21320` `ComposerEntry` rows — NOT this file's own
/// shadowing `io_registry` below, whose `entries()` returns `&'static [&'static ComposerEntry]`
/// (references) and would silently rebind under a bare call (this ticket's "SILENT REBIND" hazard).
/// `.subset_validators(...)` is new here: the old `register()`'s
/// `crate::artifacts::zip::standards::v2_0::subsets::iso21320::io::register()` call
/// (`register_subset_validator(validator_entry())`) is expressed as data by re-deriving the same
/// `SubsetValidatorEntry` via `subset_validator_entry_of::<ZipIso21320Validator>()` — the identical
/// side-effect-free constructor that file's own (module-private) `validator_entry()` calls, so no
/// visibility widening into `🚪️io/` was needed.
///
/// **NOT covered by any field**: nothing — zip's `register()` never called `register_schema_spec`
/// in the first place (`ZipSnapshot`/`ZipDiff` are hand-rolled, no derivable `RecordSpec` — see
/// the deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("zip", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("zip")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::zip::schema::zip_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::zip::schema::inferences::zip_artifact_inference_descriptor()])
        .composers(crate::artifacts::zip::standards::v2_0::subsets::any::io::io_registry::entries())
        .subset_validators(zip_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<ZipSnapshot, ZipMutation>(STDIO_ZIP_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The `✳️iso21320` subset's `SubsetValidatorEntry`, re-derived (not moved) from the same
/// side-effect-free `subset_validator_entry_of::<ZipIso21320Validator>()` constructor
/// `🚪️io/🦀️component.rs`'s own (module-private) `validator_entry()` calls.
fn zip_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::zip::standards::v2_0::subsets::iso21320::io::ZipIso21320Validator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from the former `crate::artifacts::zip::engine::register_pilot_languages`'s
/// own `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.zip",
                    extension: Some("zip"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::zip::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::zip::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::zip::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::zip::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.zip.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.spr"),
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
        id: "stdio.zip".into(),
        name: "Zip".into(),
        source_format: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::zip::standards::v2_0::subsets::any::io::io_registry as v2_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("ZipComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        let _ = register_composer_entries(v2_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
