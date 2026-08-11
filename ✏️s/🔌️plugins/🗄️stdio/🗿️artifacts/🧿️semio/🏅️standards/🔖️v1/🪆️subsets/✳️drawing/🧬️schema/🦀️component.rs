//! 🧬️ SemioDrawingArtifact schema — full artifact state, mirrors `SemioDrawingSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawStyle, SemioDrawingSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing")]
pub struct SemioDrawingArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub canvas: DrawCanvas,
    #[state(persistent)]
    #[serde(default)]
    pub styles: Vec<DrawStyle>,
    #[state(persistent)]
    #[serde(default)]
    pub layers: Vec<DrawLayer>,
}

impl Default for SemioDrawingArtifact {
    fn default() -> Self { Self::from_snapshot(SemioDrawingSnapshot::default()) }
}

impl SemioDrawingArtifact {
    pub fn to_snapshot(&self) -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: self.schema.clone(),
            canvas: self.canvas,
            styles: self.styles.clone(),
            layers: self.layers.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioDrawingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            canvas: snapshot.canvas,
            styles: snapshot.styles,
            layers: snapshot.layers,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioDrawingSnapshot) {
        self.schema = snapshot.schema;
        self.canvas = snapshot.canvas;
        self.styles = snapshot.styles;
        self.layers = snapshot.layers;
    }
}

pub fn semio_drawing_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.drawing",
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
