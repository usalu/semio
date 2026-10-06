//! 🧬️ AviArtifact schema — full artifact state, mirrors `AviSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::standards::v1_0::subsets::any::schema::snapshot::{AviMainHeader, AviSnapshot, AviStream, RiffChunk};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub main_header: AviMainHeader,
    #[state(artifact)]
    #[value(default)]
    pub streams: Vec<AviStream>,
    #[state(artifact)]
    pub idx1_present: bool,
    #[state(artifact)]
    #[value(default)]
    pub unknown_chunks: Vec<RiffChunk>,
    #[state(artifact)]
    #[value(default)]
    pub hdrl_extra: Vec<RiffChunk>,
}

impl AviArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> AviSnapshot {
        AviSnapshot { schema: self.schema.clone(), main_header: self.main_header.clone(), streams: self.streams.clone(), idx1_present: self.idx1_present, unknown_chunks: self.unknown_chunks.clone(), hdrl_extra: self.hdrl_extra.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: AviSnapshot) -> Self {
        Self { schema: snapshot.schema, main_header: snapshot.main_header, streams: snapshot.streams, idx1_present: snapshot.idx1_present, unknown_chunks: snapshot.unknown_chunks, hdrl_extra: snapshot.hdrl_extra }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: AviSnapshot) {
        self.schema = snapshot.schema;
        self.main_header = snapshot.main_header;
        self.streams = snapshot.streams;
        self.idx1_present = snapshot.idx1_present;
        self.unknown_chunks = snapshot.unknown_chunks;
        self.hdrl_extra = snapshot.hdrl_extra;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn avi_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.avi",
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
