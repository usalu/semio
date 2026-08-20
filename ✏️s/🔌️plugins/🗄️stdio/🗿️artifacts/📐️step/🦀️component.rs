//! 🎪 `stdio.step` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::step::schema::diff::StepDiff;
pub use crate::artifacts::step::schema::mutations::StepMutation;
pub use crate::artifacts::step::schema::snapshot::StepSnapshot;
pub use crate::artifacts::step::schema::StepArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_STEP_DOCUMENT_SCHEMA: &str = "stdio.step";

/// 🧬️ Artifact schema descriptor id.
pub const STEP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.step";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::artifacts::step::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<StepSnapshot, StepMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows, already the full `any` + `cc1`..`cc6` union) by its full path through the `engine`
/// shim (`📦️glue.rs`'s `pub mod engine { pub use super::standards::v_ap214::engine::*; }`) —
/// deliberately NOT this file's own `io_registry` module below, whose `entries()` returns
/// `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare call
/// (this ticket's "SILENT REBIND" hazard).
///
/// `register_subset_validators()` — step's ONE extra call beyond schema/inferences/languages/codec
/// — fans out to each of the six `cc1`..`cc6` conformance-class subsets' own `io::register()`,
/// each of which is exactly one `register_subset_validator(...)` call (confirmed by reading
/// `🪆️subsets/✳️cc1/🚪️io/🦀️component.rs`, identical shape for cc2-cc6). That IS
/// `.subset_validators(...)`'s own job, so it is folded in here rather than left imperative — each
/// subset's `SubsetValidator` type (`StepCc1Validator`..`StepCc6Validator`) is `pub`, reached via
/// `subset_validator_entry_of::<T>()` the same way each subset's own `register()` builds its entry,
/// just combined into one owned `&'static [SubsetValidatorEntry]` slice here since `.subset_
/// validators()` takes one slice, not six separate calls. `dialect.artifact_kind` on all six is
/// `"s.stdio.step"` (verified against `🪆️subsets/✳️cc1/🚪️io/🦀️component.rs`'s own `DIALECT_SELF`),
/// matching this declaration's `kind` exactly — the ownership check in `register_all` holds.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("step", definition, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("step")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .await.schema(crate::artifacts::step::schema::step_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::step::schema::inferences::step_artifact_inference_descriptor()])
        .composers(crate::artifacts::step::engine::io_registry::entries())
        .subset_validators(step_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The six `ap214` conformance-class `SubsetValidator`s, combined — see `declaration()`'s own
/// doc for why this exists as one owned slice instead of six separate `register_subset_validator`
/// calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn step_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    use semio_framework_plugin::subset_validator_entry_of;
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc1::io::StepCc1Validator>(),
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc2::io::StepCc2Validator>(),
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc3::io::StepCc3Validator>(),
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc4::io::StepCc4Validator>(),
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc5::io::StepCc5Validator>(),
                subset_validator_entry_of::<crate::artifacts::step::standards::v_ap214::subsets::cc6::io::StepCc6Validator>(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::artifacts::step::standards::
/// v_ap214::engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.step",
                    extension: Some("step"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.step"),
                },
                dsl::LanguageSpec {
                    id: "stdio.step.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::step::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::step::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.step.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.step.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::step::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::step::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.step.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.step.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.step.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.step.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.step.spr"),
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
        id: "stdio.step".into(),
        name: "Step".into(),
        source_format: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::step::standards::v_ap214::engine::io_registry as v_ap214;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ap214::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("StepComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_composer_entries(v_ap214::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
