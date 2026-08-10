//! 🧬️ GltfArtifact schema — full artifact state.

use crate::artifacts::gltf::GltfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<crate::artifacts::gltf::schema::snapshot::MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub document: serde_json::Value,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GltfArtifact {
    fn default() -> Self {
        Self::from_snapshot(GltfSnapshot::default())
    }
}

impl GltfArtifact {
    pub fn to_snapshot(&self) -> GltfSnapshot {
        GltfSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),            document: self.document.clone(),
        }
    }

    pub fn from_snapshot(snapshot: GltfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,            document: snapshot.document,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: GltfSnapshot) {
        self.schema = snapshot.schema;
        self.vertices = snapshot.vertices;        self.document = snapshot.document;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn gltf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gltf",
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
