//! 🧬️ SemioTextArtifact schema — full artifact state, mirrors `SemioTextSnapshot` field for
//! field (see `🖼️image`'s `SemioImageArtifact` for the precedent this follows).

use crate::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text")]
pub struct SemioTextArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub runs: Vec<SemioTextRun>,
}

impl Default for SemioTextArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioTextSnapshot::default())
    }
}

impl SemioTextArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioTextSnapshot {
        SemioTextSnapshot { schema: self.schema.clone(), runs: self.runs.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioTextSnapshot) -> Self {
        Self { schema: snapshot.schema, runs: snapshot.runs }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioTextSnapshot) {
        self.schema = snapshot.schema;
        self.runs = snapshot.runs;
    }
}

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
