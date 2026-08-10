//! 🧬️ GifArtifact schema (89a) — full artifact state, mirrors `GifSnapshot`'s frame/GCE/loop model.

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifFrame, GifSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
pub struct GifArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub loop_count: Option<u16>,
    #[state(persistent)]
    #[serde(default)]
    pub frames: Vec<GifFrame>,
}

impl Default for GifArtifact {
    fn default() -> Self { Self::from_snapshot(GifSnapshot::default()) }
}

impl GifArtifact {
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, loop_count: self.loop_count, frames: self.frames.clone() }
    }
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, loop_count: snapshot.loop_count, frames: snapshot.frames }
    }
    pub fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.loop_count = snapshot.loop_count;
        self.frames = snapshot.frames;
    }
}

pub fn gif_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gif.89a",
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
