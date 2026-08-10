//! 🧬️ Note artifact schema — every field of the artifact with its state class.

use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full note artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub blocks: Vec<NoteBlockNode>,
    #[state(persistent)] pub grid_visible: Option<bool>,
    #[state(persistent)] pub grid_spacing: Option<f64>,
    #[state(persistent)] pub grid_subdivisions: Option<f64>,
    #[state(persistent)] pub grid_opacity: Option<f64>,
    #[state(persistent)] pub snap_enabled: Option<bool>,
    #[state(persistent)] pub snap_grid_spacing: Option<f64>,
    #[state(persistent)] pub pencil_width: Option<f64>,
    #[state(persistent)] pub eraser_radius: Option<f64>,
    #[state(persistent)] pub assets: BTreeMap<String, NoteImageAsset>,
    #[state(shared_ui)] pub selected_block_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_block_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for NoteArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::note::NoteSnapshot::default())
    }
}

impl NoteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::note::NoteSnapshot {
        crate::artifacts::note::NoteSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            blocks: self.blocks.clone(),
            grid_visible: self.grid_visible,
            grid_spacing: self.grid_spacing,
            grid_subdivisions: self.grid_subdivisions,
            grid_opacity: self.grid_opacity,
            snap_enabled: self.snap_enabled,
            snap_grid_spacing: self.snap_grid_spacing,
            pencil_width: self.pencil_width,
            eraser_radius: self.eraser_radius,
            assets: self.assets.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::note::NoteSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            blocks: snapshot.blocks,
            grid_visible: snapshot.grid_visible,
            grid_spacing: snapshot.grid_spacing,
            grid_subdivisions: snapshot.grid_subdivisions,
            grid_opacity: snapshot.grid_opacity,
            snap_enabled: snapshot.snap_enabled,
            snap_grid_spacing: snapshot.snap_grid_spacing,
            pencil_width: snapshot.pencil_width,
            eraser_radius: snapshot.eraser_radius,
            assets: snapshot.assets,
            ..Self::default_ui()
        }
    }

    fn default_ui() -> Self {
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
            selected_block_ids: Vec::new(),
            active_utility_id: "selectDirect".into(),
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            hovered_block_id: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::note::NoteSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.blocks = snapshot.blocks;
        self.grid_visible = snapshot.grid_visible;
        self.grid_spacing = snapshot.grid_spacing;
        self.grid_subdivisions = snapshot.grid_subdivisions;
        self.grid_opacity = snapshot.grid_opacity;
        self.snap_enabled = snapshot.snap_enabled;
        self.snap_grid_spacing = snapshot.snap_grid_spacing;
        self.pencil_width = snapshot.pencil_width;
        self.eraser_radius = snapshot.eraser_radius;
        self.assets = snapshot.assets;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.note.note` — fifteen handcrafted schema leaves.
pub fn note_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.note.note",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
