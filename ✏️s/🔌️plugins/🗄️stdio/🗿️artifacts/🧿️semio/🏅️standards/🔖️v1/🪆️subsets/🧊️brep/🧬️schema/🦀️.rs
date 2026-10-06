//! 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCoedge, BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex, SemioBrepSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<BrepEdge>,
    #[state(artifact)]
    #[value(default)]
    pub loops: Vec<BrepLoop>,
    #[state(artifact)]
    #[value(default)]
    pub faces: Vec<BrepFace>,
    #[state(artifact)]
    #[value(default)]
    pub shells: Vec<BrepShell>,
    #[state(artifact)]
    #[value(default)]
    pub solids: Vec<BrepSolid>,
    #[state(artifact)]
    #[value(default)]
    pub coedges: Vec<BrepCoedge>,
    #[state(artifact)]
    #[value(default)]
    pub next_label: u64,
}

impl Default for SemioBrepArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioBrepSnapshot::default())
    }
}

impl SemioBrepArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioBrepSnapshot {
        SemioBrepSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            loops: self.loops.clone(),
            faces: self.faces.clone(),
            shells: self.shells.clone(),
            solids: self.solids.clone(),
            coedges: self.coedges.clone(),
            next_label: self.next_label,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioBrepSnapshot) -> Self {
        Self { schema: snapshot.schema, vertices: snapshot.vertices, edges: snapshot.edges, loops: snapshot.loops, faces: snapshot.faces, shells: snapshot.shells, solids: snapshot.solids, coedges: snapshot.coedges, next_label: snapshot.next_label }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioBrepSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_brep_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.brep",
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
