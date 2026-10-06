//! 🧬️ WavArtifact schema — full artifact state, mirrors `WavSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub fmt: WavFmt,
    #[state(artifact)]
    pub data: WavData,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "crate::standards::riff_pcm::subsets::any::schema::snapshot::is_zero_byte")]
    pub fmt_pad_byte: u8,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "crate::standards::riff_pcm::subsets::any::schema::snapshot::is_zero_byte")]
    pub data_pad_byte: u8,
    #[state(artifact)]
    #[value(default)]
    pub other_chunks: Vec<RiffChunk>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub chunk_order: Vec<WavChunkRef>,
}

impl Default for WavArtifact {
    fn default() -> Self {
        Self::from_snapshot(WavSnapshot::default())
    }
}

impl WavArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> WavSnapshot {
        WavSnapshot { schema: self.schema.clone(), fmt: self.fmt.clone(), data: self.data.clone(), fmt_pad_byte: self.fmt_pad_byte, data_pad_byte: self.data_pad_byte, other_chunks: self.other_chunks.clone(), chunk_order: self.chunk_order.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: WavSnapshot) -> Self {
        Self { schema: snapshot.schema, fmt: snapshot.fmt, data: snapshot.data, fmt_pad_byte: snapshot.fmt_pad_byte, data_pad_byte: snapshot.data_pad_byte, other_chunks: snapshot.other_chunks, chunk_order: snapshot.chunk_order }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: WavSnapshot) {
        self.schema = snapshot.schema;
        self.fmt = snapshot.fmt;
        self.data = snapshot.data;
        self.fmt_pad_byte = snapshot.fmt_pad_byte;
        self.data_pad_byte = snapshot.data_pad_byte;
        self.other_chunks = snapshot.other_chunks;
        self.chunk_order = snapshot.chunk_order;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wav_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.wav",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
