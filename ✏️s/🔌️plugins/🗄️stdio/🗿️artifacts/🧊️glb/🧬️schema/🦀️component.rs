//! 🧬️ GlbArtifact schema — full artifact state.

use crate::artifacts::glb::schema::snapshot::GlbPayload;
use crate::artifacts::glb::GlbSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb")]
pub struct GlbArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub payload: GlbPayload,
}

impl Default for GlbArtifact {
    fn default() -> Self { Self::from_snapshot(GlbSnapshot::default()) }
}

impl GlbArtifact {
    pub fn to_snapshot(&self) -> GlbSnapshot {
        GlbSnapshot { schema: self.schema.clone(), payload: self.payload.clone() }
    }
    pub fn from_snapshot(snapshot: GlbSnapshot) -> Self {
        Self { schema: snapshot.schema, payload: snapshot.payload }
    }
    pub fn set_snapshot(&mut self, snapshot: GlbSnapshot) {
        self.schema = snapshot.schema;
        self.payload = snapshot.payload;
    }
}

pub fn glb_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.glb",
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
