//! 🧬️ AviArtifact schema — full artifact state, mirrors `AviSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviMainHeader, AviSnapshot, AviStream, RiffChunk};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub main_header: AviMainHeader,
    #[state(persistent)]
    #[serde(default)]
    pub streams: Vec<AviStream>,
    #[state(persistent)]
    pub idx1_present: bool,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_chunks: Vec<RiffChunk>,
}

impl AviArtifact {
    pub fn to_snapshot(&self) -> AviSnapshot {
        AviSnapshot { schema: self.schema.clone(), main_header: self.main_header.clone(), streams: self.streams.clone(), idx1_present: self.idx1_present, unknown_chunks: self.unknown_chunks.clone() }
    }
    pub fn from_snapshot(snapshot: AviSnapshot) -> Self {
        Self { schema: snapshot.schema, main_header: snapshot.main_header, streams: snapshot.streams, idx1_present: snapshot.idx1_present, unknown_chunks: snapshot.unknown_chunks }
    }
    pub fn set_snapshot(&mut self, snapshot: AviSnapshot) {
        self.schema = snapshot.schema;
        self.main_header = snapshot.main_header;
        self.streams = snapshot.streams;
        self.idx1_present = snapshot.idx1_present;
        self.unknown_chunks = snapshot.unknown_chunks;
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
