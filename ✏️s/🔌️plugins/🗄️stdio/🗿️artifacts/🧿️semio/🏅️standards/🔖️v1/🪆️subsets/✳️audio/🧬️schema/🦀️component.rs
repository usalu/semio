//! 🧬️ SemioAudioArtifact schema — full artifact state, mirrors `SemioAudioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub sample_rate: u32,
    #[state(persistent)]
    #[serde(default)]
    pub format: SemioAudioFormat,
    #[state(persistent)]
    #[serde(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(persistent)]
    #[serde(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioArtifact {
    fn default() -> Self { Self::from_snapshot(SemioAudioSnapshot::default()) }
}

impl SemioAudioArtifact {
    pub fn to_snapshot(&self) -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            schema: self.schema.clone(),
            sample_rate: self.sample_rate,
            format: self.format,
            channels: self.channels.clone(),
            tags: self.tags.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioAudioSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            sample_rate: snapshot.sample_rate,
            format: snapshot.format,
            channels: snapshot.channels,
            tags: snapshot.tags,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioAudioSnapshot) {
        self.schema = snapshot.schema;
        self.sample_rate = snapshot.sample_rate;
        self.format = snapshot.format;
        self.channels = snapshot.channels;
        self.tags = snapshot.tags;
    }
}

pub fn semio_audio_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.audio",
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
