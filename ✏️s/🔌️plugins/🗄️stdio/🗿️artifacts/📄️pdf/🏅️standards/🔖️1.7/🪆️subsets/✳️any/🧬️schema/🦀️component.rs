//! 🧬️ PdfArtifact schema (1.7) — full artifact state.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfDictEntry, PdfInfo, PdfIndirectObject, PdfPage, PdfSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub declared_version: String,
    #[state(persistent)]
    #[serde(default)]
    pub pages: Vec<PdfPage>,
    #[state(persistent)]
    #[serde(default)]
    pub info: PdfInfo,
    #[state(persistent)]
    #[serde(default)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(persistent)]
    #[serde(default)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfArtifact {
    fn default() -> Self { Self::from_snapshot(PdfSnapshot::default()) }
}

impl PdfArtifact {
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot {
            schema: self.schema.clone(),
            declared_version: self.declared_version.clone(),
            pages: self.pages.clone(),
            info: self.info.clone(),
            objects: self.objects.clone(),
            trailer: self.trailer.clone(),
        }
    }
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            declared_version: snapshot.declared_version,
            pages: snapshot.pages,
            info: snapshot.info,
            objects: snapshot.objects,
            trailer: snapshot.trailer,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.declared_version = snapshot.declared_version;
        self.pages = snapshot.pages;
        self.info = snapshot.info;
        self.objects = snapshot.objects;
        self.trailer = snapshot.trailer;
    }
}

pub fn pdf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf.1.7",
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
