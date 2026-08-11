//! 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex, SemioBrepSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<BrepEdge>,
    #[state(persistent)]
    #[serde(default)]
    pub loops: Vec<BrepLoop>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<BrepFace>,
    #[state(persistent)]
    #[serde(default)]
    pub shells: Vec<BrepShell>,
    #[state(persistent)]
    #[serde(default)]
    pub solids: Vec<BrepSolid>,
}

impl Default for SemioBrepArtifact {
    fn default() -> Self { Self::from_snapshot(SemioBrepSnapshot::default()) }
}

impl SemioBrepArtifact {
    pub fn to_snapshot(&self) -> SemioBrepSnapshot {
        SemioBrepSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            loops: self.loops.clone(),
            faces: self.faces.clone(),
            shells: self.shells.clone(),
            solids: self.solids.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioBrepSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
            edges: snapshot.edges,
            loops: snapshot.loops,
            faces: snapshot.faces,
            shells: snapshot.shells,
            solids: snapshot.solids,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioBrepSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

pub fn semio_brep_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.brep",
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
