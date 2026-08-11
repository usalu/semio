//! 🧬️ Mp3Artifact schema — full artifact state, mirrors `Mp3Snapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Snapshot, Id3v2Header};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3")]
pub struct Mp3Artifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub id3v2: Option<Id3v2Header>,
    #[state(persistent)]
    #[serde(default)]
    pub frames_raw: Vec<u8>,
}

impl Default for Mp3Artifact {
    fn default() -> Self { Self::from_snapshot(Mp3Snapshot::default()) }
}

impl Mp3Artifact {
    pub fn to_snapshot(&self) -> Mp3Snapshot {
        Mp3Snapshot {
            schema: self.schema.clone(),
            id3v2: self.id3v2.clone(),
            frames_raw: self.frames_raw.clone(),
        }
    }
    pub fn from_snapshot(snapshot: Mp3Snapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id3v2: snapshot.id3v2,
            frames_raw: snapshot.frames_raw,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: Mp3Snapshot) {
        self.schema = snapshot.schema;
        self.id3v2 = snapshot.id3v2;
        self.frames_raw = snapshot.frames_raw;
    }
}

pub fn mp3_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.mp3",
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
