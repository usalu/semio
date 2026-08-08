//! 🧬️ Jack artifact schema — every field of the artifact with its state class.

use crate::artifacts::jack::{Camera, Edge, Manifest, Node};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full jack artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub name: String,
    #[state(persistent)] pub manifest_id: Option<String>,
    #[state(persistent)] pub manifest: Manifest,
    #[state(persistent)] pub camera: Camera,
    #[state(persistent)] pub nodes: Vec<Node>,
    #[state(persistent)] pub edges: Vec<Edge>,
    #[state(persistent)] pub root_node_id: Option<String>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_fixture_id: String,
    #[state(shared_ui)] pub jack_query: String,
    #[state(shared_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub viewport_camera: Camera,
    #[state(local_ui)] pub jack_result_json: String,
    #[state(local_ui)] pub editor_engagement_input: String,
    #[state(local_ui)] pub graph_engagement_input: String,
    #[state(local_ui)] pub results_engagement_input: String,
    #[state(local_ui)] pub reorganize_epoch: u64,
    #[state(local_ui)] pub editor_selection: Option<JackEditorSelection>,
    #[state(local_ui)] pub revision: u64,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Helpers
/// 🎯️ Ephemeral editor selection range (offsets into the jack query text).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}
//#endregion 🔖️Helpers

//#region 🔖️Conversions
impl Default for JackArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            root_node_id: None,
            selected_node_ids: Vec::new(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            lod_mode_by_window: BTreeMap::new(),
            viewport_camera: Camera::default(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

impl JackArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::jack::JackSnapshot {
        crate::artifacts::jack::JackSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            root_node_id: self.root_node_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::jack::JackSnapshot) -> Self {
        let viewport_camera = snapshot.camera.clone();
        Self {
            schema: snapshot.schema,
            name: snapshot.name,
            manifest_id: snapshot.manifest_id,
            manifest: snapshot.manifest,
            camera: snapshot.camera,
            nodes: snapshot.nodes,
            edges: snapshot.edges,
            root_node_id: snapshot.root_node_id,
            viewport_camera,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::jack::JackSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.manifest_id = snapshot.manifest_id;
        self.manifest = snapshot.manifest;
        self.camera = snapshot.camera;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
        self.root_node_id = snapshot.root_node_id;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.jack` — fifteen handcrafted schema leaves.
pub fn jack_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.trinity.jack",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
