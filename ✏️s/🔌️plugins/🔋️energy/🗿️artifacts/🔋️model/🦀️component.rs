//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.


pub use crate::artifacts::model::schema::snapshot::EnergyModelSnapshot;
pub use crate::artifacts::model::schema::mutations::EnergyModelMutation;
pub use crate::artifacts::model::schema::diff::EnergyModelDiff;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::model::schema::EnergyModelArtifact;

/// @emoji 🔖️ Document schema / DSL envelope id.


pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// @emoji 🧬️ Artifact schema descriptor id.
pub const ENERGY_MODEL_ARTIFACT_SCHEMA_ID: &str = "s.energy.model";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.🔋️model`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.🔋️model".into(),
        name: "Energy Model".into(),
        source_format: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        component_kind: "energy".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated
/// off `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the old side-effecting `register()`, which the plugin root called
/// unconditionally (energy has no document apps, so there was never a `.setup()` narrowing here to
/// begin with). `crate::artifacts::model::io_registry::register()` (this file's OWN
/// `🚪️DerivedIoRegistry` free fn, below) only ever called `register_composer_entries(v1::entries())`
/// — exactly what `.composers(...)` below now does through `register_all` — so it is dropped here
/// rather than kept as a duplicate call; the outer fn itself is left in place as orphaned dead code
/// (see report), matching the `🗒️note` exemplar's own precedent for its orphaned `io_registry` module.
/// `.composers(...)` below reaches `🚪️io/🦀️component.rs`'s OWN `io_registry` module (the one with
/// the actual `ComposerEntry` rows, distinct from this file's thin wrapper above) by its full path —
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES relocated it off the now-deleted
/// `⚙️engine` (an artifact is a schema + io system, never an engine) into `🚪️io/`, updating this
/// qualified reference in lockstep.
/// `register_document_codec()` — folded into this declaration via `.document_codec_bare::<Snapshot,
/// Mutation>(schema)` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d): `.document_codec::<A:
/// ArtifactApp>()` requires an `ArtifactApp` to bind `A::Snapshot`/`A::Mutation`, and this plugin is a
/// headless library with ZERO apps — there is no `ArtifactApp` to name. `document_codec_bare` is the
/// new sibling closing exactly that gap (see its own doc); the old free fn in `⚙️engine` is deleted
/// with this — nothing else called it.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.model")
        .schema(crate::artifacts::model::standards::v1::subsets::any::schema::energy_model_artifact_schema_descriptor())
        .inferences([crate::artifacts::model::standards::v1::subsets::any::schema::inferences::energy_model_artifact_inference_descriptor()])
        .composers(crate::artifacts::model::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA)
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🗒️note` exemplar's helper of the same shape.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "energy.model",
                    extension: Some("energy"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.op"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("energy.model.diff"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.pack"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.spr"),
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
    use crate::artifacts::model::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("EnergyModelComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
