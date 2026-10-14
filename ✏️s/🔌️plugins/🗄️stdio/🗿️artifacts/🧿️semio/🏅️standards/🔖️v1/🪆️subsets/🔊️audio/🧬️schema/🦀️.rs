//! 🧬️ SemioAudioArtifact schema — full artifact state, mirrors `SemioAudioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
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

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
