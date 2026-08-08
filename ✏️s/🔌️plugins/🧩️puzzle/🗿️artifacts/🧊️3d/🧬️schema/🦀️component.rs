//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume, Puzzle3dSnapshot, PUZZLE_3D_SCHEMA};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle3d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub domain: String,
    #[state(persistent)] pub meta: Puzzle3dMeta,
    #[state(persistent)] pub objects: Vec<Puzzle3dObject>,
    #[state(persistent)] pub attractions: Vec<Puzzle3dAttraction>,
    #[state(persistent)] pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[state(persistent)] pub references: Vec<Puzzle3dReference>,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub selected_vortex_ids: Vec<String>,
    #[state(shared_ui)] pub selected_attraction_ids: Vec<String>,
    #[state(shared_ui)] pub selected_target_volume_ids: Vec<String>,
    #[state(shared_ui)] pub selected_reference_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub camera_position_x: f64,
    #[state(local_ui)] pub camera_position_y: f64,
    #[state(local_ui)] pub camera_position_z: f64,
    #[state(local_ui)] pub camera_target_x: f64,
    #[state(local_ui)] pub camera_target_y: f64,
    #[state(local_ui)] pub camera_target_z: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_spacing: f64,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: u32,
    #[state(local_ui)] pub lod_automatic: bool,
    #[state(local_ui)] pub lod_depth_variable: bool,
    #[state(local_ui)] pub lod_manual: f64,
    #[state(local_ui)] pub proximity_radius: f64,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub runtime_extras_json: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_vortex_full_id: Option<String>,
    #[state(preview)] pub hovered_kind_id: Option<String>,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle3dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle3dSnapshot::default())
    }
}

impl Puzzle3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle3dSnapshot {
        Puzzle3dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            meta: self.meta.clone(),
            objects: self.objects.clone(),
            attractions: self.attractions.clone(),
            target_volumes: self.target_volumes.clone(),
            references: self.references.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Puzzle3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            meta: snapshot.meta,
            objects: snapshot.objects,
            attractions: snapshot.attractions,
            target_volumes: snapshot.target_volumes,
            references: snapshot.references,
            selected_object_ids: Vec::new(),
            selected_vortex_ids: Vec::new(),
            selected_attraction_ids: Vec::new(),
            selected_target_volume_ids: Vec::new(),
            selected_reference_ids: Vec::new(),
            active_utility_id: "select".into(),
            camera_position_x: 0.0,
            camera_position_y: 0.0,
            camera_position_z: 0.0,
            camera_target_x: 0.0,
            camera_target_y: 0.0,
            camera_target_z: 0.0,
            camera_zoom: 1.0,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            grid_visible: true,
            grid_snap_enabled: false,
            grid_spacing: 1.0,
            overlap_budget: 0.0,
            fill_count: 0,
            brush_candidate_index: 0,
            lod_automatic: true,
            lod_depth_variable: false,
            lod_manual: 1.0,
            proximity_radius: 0.75,
            locale: "en-US".into(),
            runtime_extras_json: "{}".into(),
            hovered_object_id: None,
            hovered_vortex_full_id: None,
            hovered_kind_id: None,
            preview_seq: 0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle3dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.meta = snapshot.meta;
        self.objects = snapshot.objects;
        self.attractions = snapshot.attractions;
        self.target_volumes = snapshot.target_volumes;
        self.references = snapshot.references;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle3d` — fifteen handcrafted schema leaves.
pub fn puzzle3d_artifact_schema_descriptor() -> artifact_schema::ArtifactSchemaDescriptor {
    artifact_schema::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
        artifact: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: artifact_schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: artifact_schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

