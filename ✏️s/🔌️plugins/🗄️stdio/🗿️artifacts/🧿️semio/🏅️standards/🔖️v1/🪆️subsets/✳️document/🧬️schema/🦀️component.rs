//! 🧬️ SemioDocumentArtifact schema — full artifact state, mirrors `SemioDocumentSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot, DocBlock, DocStyle};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
    #[state(persistent)]
    #[serde(default)]
    pub styles: Vec<DocStyle>,
}

impl Default for SemioDocumentArtifact {
    fn default() -> Self { Self::from_snapshot(SemioDocumentSnapshot::default()) }
}

impl SemioDocumentArtifact {
    pub fn to_snapshot(&self) -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: self.schema.clone(),
            blocks: self.blocks.clone(),
            styles: self.styles.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioDocumentSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            blocks: snapshot.blocks,
            styles: snapshot.styles,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioDocumentSnapshot) {
        self.schema = snapshot.schema;
        self.blocks = snapshot.blocks;
        self.styles = snapshot.styles;
    }
}

pub fn semio_document_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.document",
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
