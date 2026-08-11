//! 🧬️ JpgArtifact schema — full artifact state.

use crate::artifacts::jpg::JpgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

/// 🎪️ Reduced UI-editable view: identity + the raster the user is directly manipulating. Ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION killed the shared
/// `RasterImage` wrapper (jpg/png/tiff each copy-pasted it) — `width`/`height`/`pixels` are
/// first-class fields here, matching `JpgSnapshot`'s own shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub width: u32,
    #[state(persistent)]
    #[serde(default)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for JpgArtifact {
    fn default() -> Self { Self::from_snapshot(JpgSnapshot::default()) }
}

impl JpgArtifact {
    pub fn to_snapshot(&self) -> JpgSnapshot {
        // 🎪️ `JpgArtifact` is the reduced UI-editable view (schema+raster only) — it never
        // carries frame/table data, so `frame`/`sof_marker`/`arithmetic`/`quant_tables`/
        // `huffman_tables`/etc. fall back to `JpgSnapshot::default()`'s "no decoded frame" state.
        JpgSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, pixels: self.pixels.clone(), ..JpgSnapshot::default() }
    }
    pub fn from_snapshot(snapshot: JpgSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, pixels: snapshot.pixels }
    }
    pub fn set_snapshot(&mut self, snapshot: JpgSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.pixels = snapshot.pixels;
    }
}

pub fn jpg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.jpg",
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
