//! 🩻️ Block 2D artifact — the document entity the ◻2d app edits (constitutional: general). Edits
//! exactly one `NodeKind`: its identity, rim presentation, and the `HandleKind` templates placed on
//! that rim.

use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub const BLOCK_2D_SCHEMA: &str = "block.2d";

// #region 🔖️Document
/// 🔵️ The node's own rim presentation — mirrors `Puzzle2dNode`'s shape fields, minus placement (a
/// kind definition has no x/y — those belong to the puzzle assembly, not the definition).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dPresentation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
}

/// 🔘️ One handle-kind catalog row this node kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// 🌱️ One rim-handle template — where a handle of `handle_kind` sits on the node's rim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleTemplate {
    pub id: String,
    #[dsl(refs = "handle_kind")]
    pub handle_kind: String,
    #[dsl(angle = "rad")]
    pub angle: f64,
    pub radius: f64,
}

//#region 🔖️Snapshot
pub use crate::artifacts::block2d::snapshot::schema::Block2dSnapshot;
//#endregion 🔖️Snapshot

// #endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `2d.block` declaration, stitched into
/// `crate::apps::block2d::create_block2d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.block".into(),
        name: "Node Kind".into(),
        source_format: BLOCK_2D_SCHEMA.into(),
        component_kind: "block2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_2D_SCHEMA.into(),
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
    fn artifact_kind_declares_the_2d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.block");
        assert_eq!(kind.schema, BLOCK_2D_SCHEMA);
        assert_eq!(kind.component_kind, "block2d");
    }
}
//#endregion 🧪️Tests
