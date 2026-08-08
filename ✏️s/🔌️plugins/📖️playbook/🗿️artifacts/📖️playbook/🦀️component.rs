//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! `PlaybookSpec`'s fields and every step/block/expr type are NOT owned here — they live in the shared
//! `playbook` kernel crate (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook`), the same domain model
//! `📋️forms` builds on (see its `🗿️artifacts/📋️forms/🦀️component.rs` for the sibling precedent). This
//! plugin owns only its own document schema id, the `ArtifactKindSpec`, and its app-facing wrappers —
//! this component re-exports the app-facing surface so every sibling taxonomy node (`🔺️diff`, `🔧️op`,
//! `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol instead of reaching into the
//! kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookSpec, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::playbook::create_playbook_play_app`'s `🔖️Manifest` region. Lifted out of the old
/// `playbook_ui::create_playbook_play_app`'s inline builder call, unchanged (`"text.playbook"` as both
/// the media catalogue id and the store schema — playbook keeps the two distinct, unlike forms).
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
