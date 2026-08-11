//! 🧬️ SemioModelArtifact schema — full artifact state, mirrors `SemioModelSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ModelRelation, SemioModelElement, SemioModelSnapshot, SpatialNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model")]
pub struct SemioModelArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub spatial: Vec<SpatialNode>,
    #[state(persistent)]
    #[serde(default)]
    pub elements: Vec<SemioModelElement>,
    #[state(persistent)]
    #[serde(default)]
    pub relations: Vec<ModelRelation>,
}

impl Default for SemioModelArtifact {
    fn default() -> Self { Self::from_snapshot(SemioModelSnapshot::default()) }
}

impl SemioModelArtifact {
    pub fn to_snapshot(&self) -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: self.schema.clone(),
            spatial: self.spatial.clone(),
            elements: self.elements.clone(),
            relations: self.relations.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioModelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            spatial: snapshot.spatial,
            elements: snapshot.elements,
            relations: snapshot.relations,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioModelSnapshot) {
        self.schema = snapshot.schema;
        self.spatial = snapshot.spatial;
        self.elements = snapshot.elements;
        self.relations = snapshot.relations;
    }
}

pub fn semio_model_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.model",
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
