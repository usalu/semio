//! 📝️ Note artifact — the document entity this plugin's app edits: an infinite-canvas block tree
//! (text/image/table/math/ink/group blocks).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::note::create_note_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.note".into(),
        name: "2D Note".into(),
        source_format: "note.document".into(),
        component_kind: "note".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Document },
        schema: "note.document".into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Domain
pub const NOTE_DOCUMENT_SCHEMA: &str = "note.document";

/// 🎥️ Camera pose — ephemeral view state that lives in `crate::apps::note::config::NoteConfig`, never in
/// `NoteSnapshot`, so it stays out of undo history and off the operation channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for NoteCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NoteBlockNode {
    #[serde(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        paragraphs: Vec<NoteTextParagraph>,
        font_size: f64,
        font_weight: String,
        align: String,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        image_key: String,
    },
    #[serde(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[serde(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[dsl(lang = "tex")]
        tex: String,
        display_mode: bool,
    },
    #[serde(rename = "stroke", rename_all = "camelCase")]
    #[dsl(key = "stroke")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[dsl(statements, block)]
        children: Vec<NoteBlockNode>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "r")]
pub struct NoteTextRun {
    #[dsl(positional)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "p")]
pub struct NoteTextParagraph {
    pub runs: Vec<NoteTextRun>,
}

pub fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct NoteTableCell {
    #[dsl(positional)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

pub use crate::artifacts::note::snapshot::schema::NoteSnapshot;

//#endregion 🔖️Domain

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` and `NOTE_DOCUMENT_SCHEMA` are deliberately the
    /// same string here (note has no separate "fixture" store schema, unlike shooting) — pinned so a
    /// future edit can't silently diverge them without noticing.
    #[test]
    fn artifact_kind_schema_matches_the_store_schema() {
        assert_eq!(artifact_kind().schema, NOTE_DOCUMENT_SCHEMA);
    }

    #[test]
    fn note_document_round_trips_assets_and_grid_settings() {
        let mut document = NoteSnapshot {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "empty".into(),
            title: None,
            blocks: Vec::new(),
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: Some(4.0),
            grid_opacity: Some(0.35),
            snap_enabled: Some(false),
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: Some(12.0),
            assets: BTreeMap::new(),
        };
        document.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: Some(10.0), height: Some(20.0) });
        document.grid_subdivisions = Some(6.0);
        document.grid_opacity = Some(0.5);
        let json_text = serde_json::to_string(&document).unwrap();
        let parsed: NoteSnapshot = serde_json::from_str(&json_text).unwrap();
        assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
        assert_eq!(parsed.grid_subdivisions, Some(6.0));
        assert_eq!(parsed.grid_opacity, Some(0.5));
    }
}
//#endregion 🧪️Tests
