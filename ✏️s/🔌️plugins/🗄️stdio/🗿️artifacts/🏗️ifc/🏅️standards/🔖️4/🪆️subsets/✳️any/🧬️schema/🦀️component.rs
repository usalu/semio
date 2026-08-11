//! 🧬️ IfcArtifact schema — full artifact state. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this used to duplicate
//! `IfcSnapshot`'s prior worst-offender defect (`document: step::engine::part21::Part21Document`
//! verbatim) — now mirrors `IfcSnapshot`'s own typed `header`/`entities` fields.

use crate::artifacts::ifc::schema::snapshot::{IfcHeader, IfcEntity};
use crate::artifacts::ifc::IfcSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcArtifact {
    #[state(persistent)]
    pub schema: String,
    /// 📦️ The full, lossless IFC4 graph in IFC's own typed model — the actual persisted state.
    #[state(persistent)]
    #[serde(default)]
    pub header: IfcHeader,
    #[state(persistent)]
    #[serde(default)]
    pub entities: Vec<IfcEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for IfcArtifact {
    fn default() -> Self {
        Self::from_snapshot(IfcSnapshot::default())
    }
}

impl IfcArtifact {
    pub fn to_snapshot(&self) -> IfcSnapshot {
        IfcSnapshot {
            schema: self.schema.clone(),
            header: self.header.clone(),
            entities: self.entities.clone(),
        }
    }

    pub fn from_snapshot(snapshot: IfcSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header: snapshot.header,
            entities: snapshot.entities,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: IfcSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.entities = snapshot.entities;
    }

    /// 🏛️ Derived spatial-structure/placement/pset analyzer view — computed on demand, never
    /// stored; builds the shared generic Part-21 graph on the fly via `to_part21_document`
    /// (the analyzer's own relationship-graph traversal still walks that generic shape).
    pub fn spatial(&self) -> crate::artifacts::ifc::engine::spatial::SpatialAnalysis {
        let document = crate::artifacts::ifc::schema::snapshot::to_part21_document(&self.to_snapshot());
        crate::artifacts::ifc::engine::spatial::analyze_spatial(&document)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn ifc_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc",
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
