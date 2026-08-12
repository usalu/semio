//! 📜️ Imperative artifact — the document entity this plugin's app edits: a `Path` of control-flow
//! `Step`s (`state.set`/`log.print`/`control.if`/`control.while`/`math.add`/…), each addressable by a
//! [`PathRef`] for nested `control.*` bodies (drag-and-drop into blocks).
//!
//! `Path`/`Step` are NOT owned here — they live in the shared kernel crate `imperative_engine`
//! (`✏️s/🔨️modules/📜️imperative`, package `semio-s-kernel-imperative`; **do not confuse this kernel crate
//! with this plugin** — same "imperative" name, different crate, different location, a legitimate
//! dependency this plugin has always had). `Dictionary`/`Registry` come from the framework's
//! `neural_engine` kernel. This component re-exports the app-facing surface so every sibling taxonomy
//! node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `📸️snapshot`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol
//! instead of reaching into either kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry, Value};

/// 🌱️ View of a snapshot seed map as a neural [`Dictionary`] for execution.
pub fn seed_dictionary(seed: &std::collections::BTreeMap<String, Value>) -> Dictionary {
    serde_json::from_value(serde_json::to_value(seed).expect("seed serializes")).expect("seed is a dictionary")
}

/// 🗂️ The `store::ArtifactStore` schema key — deliberately distinct from the snapshot's `schema`
/// field (`"imperative.document"`, the field inside the document itself): this one keys the store envelope.
pub use crate::artifacts::imperative::schema::mutations::ImperativeMutation;

pub use crate::artifacts::imperative::schema::diff::ImperativeDiff;

pub const IMPERATIVE_DOCUMENT_SCHEMA: &str = "imperative.document/v1";

pub use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;

/// 📍️ Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}
//#endregion 🔖️Types

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from a
/// plugin `.setup()` callback. `bootstrap_imperative_runtime()` runs here too, NOT as a §6 registrar
/// (`register_language`/`register_artifact_schema_descriptor`/… all ARE §6 and now live in the builder
/// chain below) but as this artifact's OWN native-module bootstrap
/// (`register_native_imperative_module` × 4 + `register_default_imperative_contributions`) — it has no
/// `ArtifactDeclaration` field because it isn't one of the census's global SDK registrars, it is
/// imperative's private compute-runtime setup. `Once`-guarded, so calling it eagerly here reproduces
/// the old `register()`'s timing exactly (native modules populated before any `ImperativeHost`/
/// `render()` call can observe an empty registry) without adding a second purpose to `.setup()` — see
/// the plugin root's own doc for why `.setup()` stays narrowed to `register_app_schema` alone. Lives at
/// the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) — `declaration()` describes
/// the artifact (kind/schema/io/ownership), it is not engine behaviour.
///
/// ⚠️ DEVIATION from plain move-both: `bootstrap_imperative_runtime()` and `io_registry` did NOT move
/// here with `declaration()` — `bootstrap_imperative_runtime` has a second caller
/// (`ImperativeHost::from_snapshot`) inside `⚙️engine`, and `io_registry` is a real module, not a
/// single-caller helper. Both stayed put and are reached below by their full qualified path instead
/// (`bootstrap_imperative_runtime` widened only to `pub(crate)` to allow that, not `pub`).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    crate::artifacts::imperative::standards::v1::engine::bootstrap_imperative_runtime();
    semio_framework_plugin::ArtifactDeclaration::builder("s.imperative")
        .schema(crate::artifacts::imperative::schema::imperative_artifact_schema_descriptor())
        .inferences([crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::imperative_artifact_inference_descriptor()])
        .composers(crate::artifacts::imperative::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::imperative::ImperativePlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "imperative.document",
                    extension: Some("imperative"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.document"),
                },
                dsl::LanguageSpec {
                    id: "imperative.imperative.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.imperative.op"),
                },
                dsl::LanguageSpec {
                    id: "imperative.imperative.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("imperative.imperative.diff"),
                },
                dsl::LanguageSpec {
                    id: "imperative.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.pack"),
                },
                dsl::LanguageSpec {
                    id: "imperative.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::imperative::create_imperative_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.imperative".into(),
        name: "Imperative".into(),
        source_format: "imperative.document".into(),
        component_kind: "imperative".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Imperative },
        schema: "imperative.document".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("imperative.document") is deliberately NOT
    /// `IMPERATIVE_DOCUMENT_SCHEMA` ("imperative.document/v1") — the former names the artifact kind in
    /// the OS media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "imperative.document");
        assert_eq!(IMPERATIVE_DOCUMENT_SCHEMA, "imperative.document/v1");
    }

    #[test]
    fn default_snapshot_is_empty_with_the_bare_schema() {
        let snapshot = ImperativeSnapshot::default();
        assert_eq!(snapshot.schema, "imperative.document");
        assert!(snapshot.path.steps.is_empty());
        assert!(snapshot.seed.keys().next().is_none());
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::imperative::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("ImperativeComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
