//! 🧬️ Puzzle5d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dKindCatalogs, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dSnapshot, PUZZLE_5D_SCHEMA};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle5d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub domain: String,
    #[state(persistent)] pub label: Option<String>,
    #[state(persistent)] pub meta: Puzzle5dMeta,
    #[state(persistent)] pub kind_catalogs: Option<Puzzle5dKindCatalogs>,
    #[state(persistent)] pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[state(persistent)] pub parts: Vec<Puzzle5dPart>,
    #[state(persistent)] pub fasteners: Vec<Puzzle5dFastener>,
    #[state(shared_ui)] pub selected_part_ids: Vec<String>,
    #[state(shared_ui)] pub selected_grip_ids: Vec<String>,
    #[state(shared_ui)] pub selected_fastener_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub camera2d_x: f64,
    #[state(local_ui)] pub camera2d_y: f64,
    #[state(local_ui)] pub camera2d_zoom: f64,
    #[state(local_ui)] pub camera3d_position_x: f64,
    #[state(local_ui)] pub camera3d_position_y: f64,
    #[state(local_ui)] pub camera3d_position_z: f64,
    #[state(local_ui)] pub camera3d_target_x: f64,
    #[state(local_ui)] pub camera3d_target_y: f64,
    #[state(local_ui)] pub camera3d_target_z: f64,
    #[state(local_ui)] pub camera3d_zoom: f64,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub suggestion_offset: f64,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: u32,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub runtime_extras_json: String,
    #[state(preview)] pub hovered_part_id: Option<String>,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle5dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle5dSnapshot::default())
    }
}

impl Puzzle5dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle5dSnapshot {
        Puzzle5dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            label: self.label.clone(),
            meta: self.meta.clone(),
            kind_catalogs: self.kind_catalogs.clone(),
            kind_compatibility: self.kind_compatibility.clone(),
            parts: self.parts.clone(),
            fasteners: self.fasteners.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Puzzle5dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            label: snapshot.label,
            meta: snapshot.meta,
            kind_catalogs: snapshot.kind_catalogs,
            kind_compatibility: snapshot.kind_compatibility,
            parts: snapshot.parts,
            fasteners: snapshot.fasteners,
            selected_part_ids: Vec::new(),
            selected_grip_ids: Vec::new(),
            selected_fastener_ids: Vec::new(),
            active_utility_id: "select".into(),
            camera2d_x: 0.0,
            camera2d_y: 0.0,
            camera2d_zoom: 1.0,
            camera3d_position_x: 0.0,
            camera3d_position_y: 0.0,
            camera3d_position_z: 0.0,
            camera3d_target_x: 0.0,
            camera3d_target_y: 0.0,
            camera3d_target_z: 0.0,
            camera3d_zoom: 1.0,
            selection_method: "rectangle".into(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            suggestion_offset: 80.0,
            overlap_budget: 0.0,
            fill_count: 0,
            brush_candidate_index: 0,
            lod_mode: "automatic".into(),
            locale: "en-US".into(),
            runtime_extras_json: "{}".into(),
            hovered_part_id: None,
            preview_seq: 0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle5dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.label = snapshot.label;
        self.meta = snapshot.meta;
        self.kind_catalogs = snapshot.kind_catalogs;
        self.kind_compatibility = snapshot.kind_compatibility;
        self.parts = snapshot.parts;
        self.fasteners = snapshot.fasteners;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle5d` — fifteen handcrafted schema leaves.
pub fn puzzle5d_artifact_schema_descriptor() -> artifact_schema::ArtifactSchemaDescriptor {
    artifact_schema::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
        artifact: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: artifact_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: artifact_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

