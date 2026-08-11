//! 🧬️ SemioObjectArtifact schema — full artifact state, mirrors `SemioObjectSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{SemioObjectNode, SemioObjectSnapshot, SemioValue};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub root: SemioValue,
    #[state(persistent)]
    pub objects: Vec<SemioObjectNode>,
}

impl Default for SemioObjectArtifact {
    fn default() -> Self { Self::from_snapshot(SemioObjectSnapshot::default()) }
}

impl SemioObjectArtifact {
    pub fn to_snapshot(&self) -> SemioObjectSnapshot {
        SemioObjectSnapshot {
            schema: self.schema.clone(),
            root: self.root.clone(),
            objects: self.objects.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioObjectSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            root: snapshot.root,
            objects: snapshot.objects,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioObjectSnapshot) {
        self.schema = snapshot.schema;
        self.root = snapshot.root;
        self.objects = snapshot.objects;
    }
}

pub fn semio_object_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.object",
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
