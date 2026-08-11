//! 🧬️ GltfArtifact schema — full artifact state.

use crate::artifacts::gltf::schema::snapshot::{GltfDocument, GltfSourceForm};
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
    pub document: GltfDocument,
    #[state(persistent)]
    #[serde(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(persistent)]
    #[serde(default)]
    pub source_form: GltfSourceForm,
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
            document: self.document.clone(),
            buffers: self.buffers.clone(),
            source_form: self.source_form,
        }
    }

    pub fn from_snapshot(snapshot: GltfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            document: snapshot.document,
            buffers: snapshot.buffers,
            source_form: snapshot.source_form,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: GltfSnapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.buffers = snapshot.buffers;
        self.source_form = snapshot.source_form;
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
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
