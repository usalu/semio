//! 🧬️ LasArtifact schema — full artifact state.

use crate::artifacts::las::LasSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<crate::artifacts::las::schema::snapshot::MeshVertex>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LasArtifact {
    fn default() -> Self {
        Self::from_snapshot(LasSnapshot::default())
    }
}

impl LasArtifact {
    pub fn to_snapshot(&self) -> LasSnapshot {
        LasSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
        }
    }

    pub fn from_snapshot(snapshot: LasSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: LasSnapshot) {
        self.schema = snapshot.schema;
        self.vertices = snapshot.vertices;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn las_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.las",
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
    }
}
//#endregion 🔖️Descriptor
