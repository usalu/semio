//! 🧬️ SemioAudioArtifact schema — full artifact state, mirrors `SemioAudioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub sample_rate: u32,
    #[state(artifact)]
    #[value(default)]
    pub format: SemioAudioFormat,
    #[state(artifact)]
    #[value(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(artifact)]
    #[value(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioAudioSnapshot::default())
    }
}

impl SemioAudioArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioAudioSnapshot {
        SemioAudioSnapshot { schema: self.schema.clone(), sample_rate: self.sample_rate, format: self.format, channels: self.channels.clone(), tags: self.tags.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioAudioSnapshot) -> Self {
        Self { schema: snapshot.schema, sample_rate: snapshot.sample_rate, format: snapshot.format, channels: snapshot.channels, tags: snapshot.tags }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioAudioSnapshot) {
        self.schema = snapshot.schema;
        self.sample_rate = snapshot.sample_rate;
        self.format = snapshot.format;
        self.channels = snapshot.channels;
        self.tags = snapshot.tags;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_audio_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.audio",
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
