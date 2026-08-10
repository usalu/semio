//! 🧬️ PdfArtifact schema — full artifact state.

// 🔀️ S-6 twin: `crate::artifacts::pdf::schema` now shims to 1.7 (canonical) -- 1.4's own schema
// uses its own standard-local snapshot type directly rather than the shared root re-export.
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub page: PageDoc,
}

impl Default for PdfArtifact {
    fn default() -> Self { Self::from_snapshot(PdfSnapshot::default()) }
}

impl PdfArtifact {
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot { schema: self.schema.clone(), page: self.page.clone() }
    }
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self { schema: snapshot.schema, page: snapshot.page }
    }
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.page = snapshot.page;
    }
}

pub fn pdf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf",
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
