//! 🧬️ Note snapshot schema — artifact-lane fields only.

use crate::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted note document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "note.note", layout = "lines")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(statements, block)]
    pub blocks: Vec<NoteBlockNode>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub grid_subdivisions: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub grid_opacity: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub snap_enabled: Option<bool>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub snap_grid_spacing: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub pencil_width: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub eraser_radius: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, NoteImageAsset>,
    /// 🔗️ Forward reference slot — ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`note→R:any`):
    /// a note may point at any other artifact (not a specific composed type), matching layout's
    /// `referenced_model` precedent. Schema/codec-complete, deliberately left inert beyond that (no
    /// mutation dispatch, no resolver read path) — genuinely new capability with no existing UI/
    /// converter to preserve, same honest scope layout's own report used for its analogous slot.
    #[state(artifact)]
    #[link_slot(roles("any"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub linked_artifact: Option<store::ArtifactLink>,
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for NoteSnapshot {
    fn default() -> Self {
        Self {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: String::new(),
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
            linked_artifact: None,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;
//#endregion 🧪️Tests

