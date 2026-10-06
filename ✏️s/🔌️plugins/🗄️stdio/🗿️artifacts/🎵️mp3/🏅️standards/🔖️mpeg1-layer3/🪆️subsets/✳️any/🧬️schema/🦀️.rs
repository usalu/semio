//! 🧬️ Mp3Artifact schema — full artifact state, mirrors `Mp3Snapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3")]
pub struct Mp3Artifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub id3v2: Option<Id3v2Tag>,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<Mp3Frame>,
    #[state(artifact)]
    #[value(default)]
    pub id3v1: Option<Id3v1Tag>,
}

impl Default for Mp3Artifact {
    fn default() -> Self {
        Self::from_snapshot(Mp3Snapshot::default())
    }
}

impl Mp3Artifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> Mp3Snapshot {
        Mp3Snapshot { schema: self.schema.clone(), id3v2: self.id3v2.clone(), frames: self.frames.clone(), id3v1: self.id3v1.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: Mp3Snapshot) -> Self {
        Self { schema: snapshot.schema, id3v2: snapshot.id3v2, frames: snapshot.frames, id3v1: snapshot.id3v1 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: Mp3Snapshot) {
        self.schema = snapshot.schema;
        self.id3v2 = snapshot.id3v2;
        self.frames = snapshot.frames;
        self.id3v1 = snapshot.id3v1;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mp3_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.mp3",
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
//#region 🔖️SampleRateTable
/// 📐️ Sample rate table (Hz), keyed by `(version_id, index)`. Index `3` is reserved. Lives in
/// `🧬️schema/` (not `🚪️io/`, where the frame-header codec that also calls it lives) because
/// `💡️inferences/⏱️duration` needs the same real table for its duration derivation and
/// `🧬️schema` must never depend on `🚪️io` — `🚪️io` depends on `🧬️schema` instead (both call
/// sites reuse this one definition, never re-declare it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sample_rate_hz(version_id: u8, index: u8) -> Option<u32> {
    match (version_id, index) {
        (3, 0) => Some(44_100),
        (3, 1) => Some(48_000),
        (3, 2) => Some(32_000),
        (2, 0) => Some(22_050),
        (2, 1) => Some(24_000),
        (2, 2) => Some(16_000),
        (0, 0) => Some(11_025),
        (0, 1) => Some(12_000),
        (0, 2) => Some(8_000),
        _ => None,
    }
}
//#endregion 🔖️SampleRateTable

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
