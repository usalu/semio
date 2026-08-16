//! 🎪 `stdio.pptx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::pptx::schema::diff::PptxDiff;
pub use crate::artifacts::pptx::schema::mutations::PptxMutation;
pub use crate::artifacts::pptx::schema::snapshot::PptxSnapshot;
pub use crate::artifacts::pptx::schema::PptxArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PPTX_DOCUMENT_SCHEMA: &str = "stdio.pptx";

/// 🧬️ Artifact schema descriptor id.
pub const PPTX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pptx";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::artifacts::pptx::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::register_document_codec(store::ArtifactCodec::of::<PptxSnapshot,
/// PptxMutation>(...))` call. `.composers(...)` reaches the engine's own `io_registry` (through the
/// `engine` shim), whose `entries()` already aggregates the `✳️any`/`✳️strict`/`✳️transitional`
/// `ComposerEntry` rows — NOT this file's own shadowing `io_registry` below, whose `entries()`
/// returns `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare
/// call (this ticket's "SILENT REBIND" hazard). `.subset_validators(...)` re-derives the two
/// `SubsetValidatorEntry` rows the old `register()`'s `✳️strict`/`✳️transitional` `io::register()`
/// calls used to install, via the same side-effect-free `subset_validator_entry_of::<V>()`
/// constructor those (module-private) `validator_entry()` fns call — no visibility widening into
/// `🚪️io/` needed.
///
/// **NOT covered by any field**: nothing — pptx's `register()` never called `register_schema_spec`
/// (`PptxSnapshot`/`PptxDiff`/`PptxMutation` are hand-rolled, no derivable `RecordSpec` — see the
/// deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("pptx", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("pptx")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::pptx::schema::pptx_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::inferences::pptx_artifact_inference_descriptor()])
        .composers(crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::io_registry::entries())
        .subset_validators(pptx_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<PptxSnapshot, PptxMutation>(STDIO_PPTX_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The `✳️strict`/`✳️transitional` subsets' `SubsetValidatorEntry` rows, re-derived (not moved)
/// from the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️component.rs` (module-private) `validator_entry()` calls.
fn pptx_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::io::PptxStrictValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::io::PptxTransitionalValidator>(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from `crate::artifacts::pptx::engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.pptx",
                    extension: Some("pptx"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::pptx::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pptx::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pptx"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pptx.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::pptx::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pptx::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pptx.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pptx.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::pptx::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pptx::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.pptx.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pptx.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pptx.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pptx.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pptx.spr"),
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
        id: "stdio.pptx".into(),
        name: "Pptx".into(),
        source_format: STDIO_PPTX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::io_registry as v_ecma_376;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ecma_376::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PptxComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ecma_376::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
