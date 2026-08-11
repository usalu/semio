//! 🧬️ SemioMeshArtifact schema — full artifact state, mirrors `SemioMeshSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioMesh, SemioMaterial, SemioTexture};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh")]
pub struct SemioMeshArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub meshes: Vec<SemioMesh>,
    #[state(persistent)]
    #[serde(default)]
    pub materials: Vec<SemioMaterial>,
    #[state(persistent)]
    #[serde(default)]
    pub textures: Vec<SemioTexture>,
}

impl Default for SemioMeshArtifact {
    fn default() -> Self { Self::from_snapshot(SemioMeshSnapshot::default()) }
}

impl SemioMeshArtifact {
    pub fn to_snapshot(&self) -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: self.schema.clone(),
            meshes: self.meshes.clone(),
            materials: self.materials.clone(),
            textures: self.textures.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioMeshSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            meshes: snapshot.meshes,
            materials: snapshot.materials,
            textures: snapshot.textures,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioMeshSnapshot) {
        self.schema = snapshot.schema;
        self.meshes = snapshot.meshes;
        self.materials = snapshot.materials;
        self.textures = snapshot.textures;
    }
}

pub fn semio_mesh_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.mesh",
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
