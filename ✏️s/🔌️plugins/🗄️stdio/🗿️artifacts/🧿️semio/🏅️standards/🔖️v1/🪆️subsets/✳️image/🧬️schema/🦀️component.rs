//! 🧬️ SemioImageArtifact schema — full artifact state, mirrors `SemioImageSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{
    SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub colorspace: SemioColorspace,
    #[state(persistent)]
    #[serde(default)]
    pub bit_depth: u8,
    #[state(persistent)]
    #[serde(default)]
    pub frames: Vec<SemioImageFrame>,
    #[state(persistent)]
    #[serde(default)]
    pub icc: Option<Vec<u8>>,
    #[state(persistent)]
    #[serde(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageArtifact {
    fn default() -> Self { Self::from_snapshot(SemioImageSnapshot::default()) }
}

impl SemioImageArtifact {
    pub fn to_snapshot(&self) -> SemioImageSnapshot {
        SemioImageSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            colorspace: self.colorspace,
            bit_depth: self.bit_depth,
            frames: self.frames.clone(),
            icc: self.icc.clone(),
            metadata: self.metadata.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioImageSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            colorspace: snapshot.colorspace,
            bit_depth: snapshot.bit_depth,
            frames: snapshot.frames,
            icc: snapshot.icc,
            metadata: snapshot.metadata,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioImageSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.colorspace = snapshot.colorspace;
        self.bit_depth = snapshot.bit_depth;
        self.frames = snapshot.frames;
        self.icc = snapshot.icc;
        self.metadata = snapshot.metadata;
    }
}

pub fn semio_image_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.image",
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
