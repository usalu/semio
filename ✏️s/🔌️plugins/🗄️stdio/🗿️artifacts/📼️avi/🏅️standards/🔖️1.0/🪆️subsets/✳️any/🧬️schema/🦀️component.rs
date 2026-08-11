//! 🧬️ AviArtifact schema — full artifact state, mirrors `AviSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviSnapshot, AviRawChunk};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub form_type: String,
    #[state(persistent)]
    #[serde(default)]
    pub chunks: Vec<AviRawChunk>,
}

impl Default for AviArtifact {
    fn default() -> Self { Self::from_snapshot(AviSnapshot::default()) }
}

impl AviArtifact {
    pub fn to_snapshot(&self) -> AviSnapshot {
        AviSnapshot {
            schema: self.schema.clone(),
            form_type: self.form_type.clone(),
            chunks: self.chunks.clone(),
        }
    }
    pub fn from_snapshot(snapshot: AviSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            form_type: snapshot.form_type,
            chunks: snapshot.chunks,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: AviSnapshot) {
        self.schema = snapshot.schema;
        self.form_type = snapshot.form_type;
        self.chunks = snapshot.chunks;
    }
}

pub fn avi_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.avi",
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
