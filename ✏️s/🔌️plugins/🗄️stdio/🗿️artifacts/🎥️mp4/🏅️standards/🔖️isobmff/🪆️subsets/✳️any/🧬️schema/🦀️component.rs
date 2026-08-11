//! 🧬️ Mp4Artifact schema — full artifact state, mirrors `Mp4Snapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Box, Mp4Ftyp, Mp4Snapshot, Mp4Track};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4")]
pub struct Mp4Artifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub ftyp: Mp4Ftyp,
    #[state(persistent)]
    #[serde(default)]
    pub tracks: Vec<Mp4Track>,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_boxes: Vec<Mp4Box>,
}

impl Mp4Artifact {
    pub fn to_snapshot(&self) -> Mp4Snapshot {
        Mp4Snapshot {
            schema: self.schema.clone(),
            ftyp: self.ftyp.clone(),
            tracks: self.tracks.clone(),
            unknown_boxes: self.unknown_boxes.clone(),
        }
    }
    pub fn from_snapshot(snapshot: Mp4Snapshot) -> Self {
        Self { schema: snapshot.schema, ftyp: snapshot.ftyp, tracks: snapshot.tracks, unknown_boxes: snapshot.unknown_boxes }
    }
    pub fn set_snapshot(&mut self, snapshot: Mp4Snapshot) {
        self.schema = snapshot.schema;
        self.ftyp = snapshot.ftyp;
        self.tracks = snapshot.tracks;
        self.unknown_boxes = snapshot.unknown_boxes;
    }
}

pub fn mp4_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.mp4",
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
