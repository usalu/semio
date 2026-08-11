//! 🧬️ WavArtifact schema — full artifact state, mirrors `WavSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavSnapshot, WavFmt, WavData, RiffChunk};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub fmt: WavFmt,
    #[state(persistent)]
    pub data: WavData,
    #[state(persistent)]
    #[serde(default)]
    pub other_chunks: Vec<RiffChunk>,
}

impl Default for WavArtifact {
    fn default() -> Self { Self::from_snapshot(WavSnapshot::default()) }
}

impl WavArtifact {
    pub fn to_snapshot(&self) -> WavSnapshot {
        WavSnapshot {
            schema: self.schema.clone(),
            fmt: self.fmt.clone(),
            data: self.data.clone(),
            other_chunks: self.other_chunks.clone(),
        }
    }
    pub fn from_snapshot(snapshot: WavSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            fmt: snapshot.fmt,
            data: snapshot.data,
            other_chunks: snapshot.other_chunks,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: WavSnapshot) {
        self.schema = snapshot.schema;
        self.fmt = snapshot.fmt;
        self.data = snapshot.data;
        self.other_chunks = snapshot.other_chunks;
    }
}

pub fn wav_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.wav",
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
