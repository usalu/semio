//! 🧬️ Draw artifact schema — every field of the artifact with its state class.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode, DRAW_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full draw artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub layers: Vec<DrawLayerNode>,
    #[state(persistent)] pub assets: BTreeMap<String, DrawImageAsset>,
    #[state(persistent)] pub artboard: Option<DrawArtboard>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DrawArtifact {
    fn default() -> Self {
        Self {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }),
            selected_ids: Vec::new(),
            active_utility_id: "selectDirect".into(),
            engagement_input: String::new(),
            camera_x: 512.0,
            camera_y: 512.0,
            camera_zoom: 0.75,
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl DrawArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::draw::DrawSnapshot {
        crate::artifacts::draw::DrawSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            layers: self.layers.clone(),
            assets: self.assets.clone(),
            artboard: self.artboard.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::draw::DrawSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            layers: snapshot.layers,
            assets: snapshot.assets,
            artboard: snapshot.artboard,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::draw::DrawSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
        self.artboard = snapshot.artboard;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.draw.draw` — twenty handcrafted schema leaves.
pub fn draw_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.draw.draw",
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
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
