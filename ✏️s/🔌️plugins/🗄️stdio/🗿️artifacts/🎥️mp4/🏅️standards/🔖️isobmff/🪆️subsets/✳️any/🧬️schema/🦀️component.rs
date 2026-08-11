//! 🧬️ Mp4Artifact schema — full artifact state, mirrors `Mp4Snapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Snapshot, Mp4RawBox};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp4")]
pub struct Mp4Artifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub major_brand: String,
    #[state(persistent)]
    pub minor_version: u32,
    #[state(persistent)]
    pub track_count: u32,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_boxes: Vec<Mp4RawBox>,
}

impl Default for Mp4Artifact {
    fn default() -> Self { Self::from_snapshot(Mp4Snapshot::default()) }
}

impl Mp4Artifact {
    pub fn to_snapshot(&self) -> Mp4Snapshot {
        Mp4Snapshot {
            schema: self.schema.clone(),
            major_brand: self.major_brand.clone(),
            minor_version: self.minor_version.clone(),
            track_count: self.track_count.clone(),
            unknown_boxes: self.unknown_boxes.clone(),
        }
    }
    pub fn from_snapshot(snapshot: Mp4Snapshot) -> Self {
        Self {
            schema: snapshot.schema,
            major_brand: snapshot.major_brand,
            minor_version: snapshot.minor_version,
            track_count: snapshot.track_count,
            unknown_boxes: snapshot.unknown_boxes,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: Mp4Snapshot) {
        self.schema = snapshot.schema;
        self.major_brand = snapshot.major_brand;
        self.minor_version = snapshot.minor_version;
        self.track_count = snapshot.track_count;
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
