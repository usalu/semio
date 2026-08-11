//! 🧬️ PngArtifact schema — full artifact state (mirrors `PngSnapshot` field-for-field; see
//! `zip_artifact_schema_descriptor`/`ZipArtifact` for the established repo pattern this follows).

use crate::artifacts::png::schema::snapshot::{
    PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims,
    PngRgb, PngSrgbIntent, PngTextChunk, PngTimestamp, PngTransparency,
};
use crate::artifacts::png::PngSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    pub bit_depth: u8,
    #[state(persistent)]
    pub color_type: PngColorType,
    #[state(persistent)]
    pub interlace: bool,
    #[state(persistent)]
    #[serde(default)]
    pub plte: Option<Vec<PngRgb>>,
    #[state(persistent)]
    #[serde(default)]
    pub trns: Option<PngTransparency>,
    #[state(persistent)]
    #[serde(default)]
    pub gama: Option<u32>,
    #[state(persistent)]
    #[serde(default)]
    pub chrm: Option<PngChromaticities>,
    #[state(persistent)]
    #[serde(default)]
    pub srgb: Option<PngSrgbIntent>,
    #[state(persistent)]
    #[serde(default)]
    pub phys: Option<PngPhysicalDims>,
    #[state(persistent)]
    #[serde(default)]
    pub time: Option<PngTimestamp>,
    #[state(persistent)]
    #[serde(default)]
    pub bkgd: Option<PngBackground>,
    #[state(persistent)]
    #[serde(default)]
    pub text_chunks: Vec<PngTextChunk>,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
    #[state(persistent)]
    #[serde(default)]
    pub chunk_order: Vec<PngChunkMarker>,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_chunks: Vec<PngChunk>,
}

impl Default for PngArtifact {
    fn default() -> Self { Self::from_snapshot(PngSnapshot::default()) }
}

impl PngArtifact {
    pub fn to_snapshot(&self) -> PngSnapshot {
        PngSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            bit_depth: self.bit_depth,
            color_type: self.color_type,
            interlace: self.interlace,
            plte: self.plte.clone(),
            trns: self.trns.clone(),
            gama: self.gama,
            chrm: self.chrm,
            srgb: self.srgb,
            phys: self.phys,
            time: self.time,
            bkgd: self.bkgd.clone(),
            text_chunks: self.text_chunks.clone(),
            pixels: self.pixels.clone(),
            chunk_order: self.chunk_order.clone(),
            unknown_chunks: self.unknown_chunks.clone(),
        }
    }
    pub fn from_snapshot(snapshot: PngSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            bit_depth: snapshot.bit_depth,
            color_type: snapshot.color_type,
            interlace: snapshot.interlace,
            plte: snapshot.plte,
            trns: snapshot.trns,
            gama: snapshot.gama,
            chrm: snapshot.chrm,
            srgb: snapshot.srgb,
            phys: snapshot.phys,
            time: snapshot.time,
            bkgd: snapshot.bkgd,
            text_chunks: snapshot.text_chunks,
            pixels: snapshot.pixels,
            chunk_order: snapshot.chunk_order,
            unknown_chunks: snapshot.unknown_chunks,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: PngSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

pub fn png_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.png",
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
