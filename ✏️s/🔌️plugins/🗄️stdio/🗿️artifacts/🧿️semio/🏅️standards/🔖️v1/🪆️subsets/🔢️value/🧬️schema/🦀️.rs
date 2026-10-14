//! 🧬️ SemioValueArtifact schema — full artifact state, mirrors `SemioValueSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::schema::snapshot::{SemioValue, SemioValueNode, SemioValueSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value")]
pub struct SemioValueArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub root: SemioValue,
    #[state(artifact)]
    pub nodes: Vec<SemioValueNode>,
}

impl Default for SemioValueArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioValueSnapshot::default())
    }
}

impl SemioValueArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: self.schema.clone(), root: self.root.clone(), nodes: self.nodes.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioValueSnapshot) -> Self {
        Self { schema: snapshot.schema, root: snapshot.root, nodes: snapshot.nodes }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioValueSnapshot) {
        self.schema = snapshot.schema;
        self.root = snapshot.root;
        self.nodes = snapshot.nodes;
    }
}

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
