//! 🎪 `stdio.jpg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::artifacts::jpg::schema::diff::JpgDiff;
pub use crate::artifacts::jpg::schema::mutations::JpgMutation;
pub use crate::artifacts::jpg::schema::snapshot::JpgSnapshot;
pub use crate::artifacts::jpg::schema::JpgArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JPG_DOCUMENT_SCHEMA: &str = "stdio.jpg";

/// 🧬️ Artifact schema descriptor id.
pub const JPG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.jpg";

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const JPG_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
pub const JPG_BASELINE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("baseline") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.jpg".into(),
        name: "Jpg".into(),
        source_format: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::artifacts::jpg::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// rows — ✳️any + ✳️baseline already folded into one list there) by its FULLY QUALIFIED path,
/// never the bare `io_registry::entries()` shortcut that would silently rebind to this file's own
/// shadowing `io_registry` module below (repo-wide "silent rebind" hazard this ticket tracks —
/// that module returns `&[&ComposerEntry]`, a different type, and is left in place as orphaned
/// dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). The baseline subset's
/// `SubsetValidator` (`✳️baseline/🚪️io`'s own `JpgBaselineValidator`, previously registered via
/// `⚙️engine::register()`'s trailing `subsets::baseline::io::register()` call) is re-derived here
/// via `subset_validator_entry_of::<JpgBaselineValidator>()` rather than reused from that file's
/// own private `validator_entry()` cache (not `pub`) — same erasure helper, fresh instance, same
/// registry effect. `register_schema_specs()` (this engine's own, real, `#[cfg]`-unconditional
/// `pub fn`) is deliberately dropped, not re-wired: its body is a genuine no-op (`{}`, see that
/// fn's own doc comment — `JpgSnapshot`/`JpgDiff`/`JpgMutation` all fail `dsl`'s derive machinery
/// on hand-rolled fields, so it never calls `dsl::registry::register_schema_spec` at all), so
/// dropping the call changes zero runtime behaviour. `⚙️engine` itself is untouched — this only
/// REFERENCES what it already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("jpg", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("jpg")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::jpg_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::inferences::jpg_artifact_inference_descriptor()])
        .composers(crate::artifacts::jpg::standards::v_jfif_1_01::engine::io_registry::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<JpgSnapshot, JpgMutation>(STDIO_JPG_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ Re-derives the ✳️baseline subset's `SubsetValidatorEntry` — see `declaration()`'s own doc for
/// why this calls `subset_validator_entry_of` directly instead of reusing the private cache in
/// `✳️baseline/🚪️io`.
fn declared_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::io::JpgBaselineValidator>()]).as_slice()
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
                    id: "stdio.jpg",
                    extension: Some("jpg"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.jpg"),
                },
                dsl::LanguageSpec {
                    id: "stdio.jpg.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.jpg.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.jpg.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.jpg.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.jpg.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.jpg.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.jpg.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.jpg.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::jpg::standards::v_jfif_1_01::engine::io_registry as v_jfif_1_01;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_jfif_1_01::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("JpgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub fn register() {
        let _ = register_composer_entries(v_jfif_1_01::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
