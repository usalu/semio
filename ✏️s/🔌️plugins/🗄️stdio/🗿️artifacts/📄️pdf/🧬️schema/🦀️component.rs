//! 🧬️ PdfArtifact schema — full artifact state.

use crate::artifacts::pdf::schema::snapshot::PdfEntry;
use crate::artifacts::pdf::PdfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.pdf` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<PdfEntry>,
}
//#endregion Artifact

//#region Conversions
impl Default for PdfArtifact {
    fn default() -> Self {
        Self::from_snapshot(PdfSnapshot::default())
    }
}

impl PdfArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot {
            schema: self.schema.clone(),
            entries: self.entries.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            entries: snapshot.entries,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.pdf`.
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
    }
}
//#endregion Descriptor
