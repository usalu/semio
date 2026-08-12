//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! Step/block/expr records live in the shared kernel `playbook` crate; this plugin owns
//! `PlaybookSnapshot`, `PlaybookArtifact`, facet schemas, and app-facing wrappers.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use crate::playbook::{
    PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS,
    PLAYBOOK_DOCUMENT_SCHEMA,
};
pub use crate::artifacts::playbook::schema::diff::{
    PlaybookBlockPatch, PlaybookBlockPatchEntry, PlaybookBlocksDelta, PlaybookDiff, PlaybookStepPatch,
    PlaybookStepPatchEntry, PlaybookStepsDelta, PlaybookStringList,
};
pub use crate::artifacts::playbook::schema::PlaybookArtifact;
pub use crate::artifacts::playbook::schema::snapshot::PlaybookSnapshot;
pub use crate::artifacts::playbook::schema::mutations::PlaybookMutation;

pub const PLAYBOOK_ARTIFACT_SCHEMA_ID: &str = "s.playbook.playbook";

/// 📸️ Default persisted playbook document for new stores and demos.
pub fn empty_playbook_snapshot() -> PlaybookSnapshot {
    PlaybookSnapshot::default()
}

/// 🧱️ Flattens all blocks across steps — delegates to the kernel helper.
pub fn flatten_playbook_blocks(snapshot: &PlaybookSnapshot) -> Vec<PlaybookBlock> {
    crate::playbook::flatten_playbook_blocks(&snapshot.as_kernel())
        .into_iter()
        .cloned()
        .collect()
}
//#endregion 🔖️Types

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::apps::playbook::config::schema::register_app_schema()` is the
/// one exception, still called from this file's own `.setup()`: it registers the `PlaybookPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Lives at the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) —
/// `declaration()` describes the artifact (kind/schema/io/ownership), it is not engine behaviour.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.playbook")
        .schema(crate::artifacts::playbook::schema::playbook_artifact_schema_descriptor())
        .inferences([crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::playbook_artifact_inference_descriptor()])
        .composers(crate::artifacts::playbook::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::playbook::PlaybookPlayApp>()
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
                    id: "playbook.playbook",
                    extension: Some("playbook"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook.op"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("playbook.playbook.diff"),
                },
                dsl::LanguageSpec {
                    id: "playbook.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.pack"),
                },
                dsl::LanguageSpec {
                    id: "playbook.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::playbook::create_playbook_play_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.playbook".into(),
        name: "Playbook".into(),
        source_format: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        component_kind: "playbook".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_uses_the_playbook_media_kind_as_both_id_and_schema() {
        assert_eq!(artifact_kind().id, "text.playbook");
        assert_eq!(artifact_kind().schema, PLAYBOOK_DOCUMENT_SCHEMA);
    }

    #[test]
    fn block_fields_roundtrip() {
        let json = r#"{
            "id":"b1",
            "label":"Panel Count",
            "kind":"number",
            "required":true,
            "min":4,
            "max":64,
            "step":1,
            "unit":"panels"
        }"#;
        let block: PlaybookBlock = serde_json::from_str(json).expect("block json");
        assert_eq!(block.min, Some(4.0));
        assert_eq!(block.unit.as_deref(), Some("panels"));
        assert!(block.required.unwrap_or(false));
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::playbook::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("PlaybookComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
