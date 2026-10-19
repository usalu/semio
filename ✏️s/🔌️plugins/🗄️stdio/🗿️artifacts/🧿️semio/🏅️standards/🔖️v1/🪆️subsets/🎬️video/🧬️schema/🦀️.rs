//! 🧬️ SemioVideoArtifact schema — full artifact state, mirrors `SemioVideoSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::schema::snapshot::{SemioVideoSnapshot, SemioVideoStream};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video")]
pub struct SemioVideoArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub streams: Vec<SemioVideoStream>,
}

impl Default for SemioVideoArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioVideoSnapshot::default())
    }
}

impl SemioVideoArtifact {
    pub fn to_snapshot(&self) -> SemioVideoSnapshot {
        SemioVideoSnapshot { schema: self.schema.clone(), streams: self.streams.clone() }
    }
    pub fn from_snapshot(snapshot: SemioVideoSnapshot) -> Self {
        Self { schema: snapshot.schema, streams: snapshot.streams }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioVideoSnapshot) {
        self.schema = snapshot.schema;
        self.streams = snapshot.streams;
    }
}

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
