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
pub use crate::artifacts::playbook::diff::{
    PlaybookBlockPatch, PlaybookBlockPatchEntry, PlaybookBlocksDelta, PlaybookDiff, PlaybookStepPatch,
    PlaybookStepPatchEntry, PlaybookStepsDelta, PlaybookStringList,
};
pub use crate::artifacts::playbook::schema::PlaybookArtifact;
pub use crate::artifacts::playbook::snapshot::schema::PlaybookSnapshot;

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
