//! 🧬️ StepArtifact schema — full artifact state.

use crate::artifacts::step::StepSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepArtifact {
    #[state(persistent)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph — the actual persisted state.
    #[state(persistent)]
    #[serde(default)]
    pub document: crate::artifacts::step::engine::part21::Part21Document,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StepArtifact {
    fn default() -> Self {
        Self::from_snapshot(StepSnapshot::default())
    }
}

impl StepArtifact {
    pub fn to_snapshot(&self) -> StepSnapshot {
        StepSnapshot {
            schema: self.schema.clone(),
            document: self.document.clone(),
        }
    }

    pub fn from_snapshot(snapshot: StepSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            document: snapshot.document,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: StepSnapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
    }

    /// 🧐️ Derived BrepMesh analyzer view — computed on demand, never stored.
    pub fn brep_mesh(&self) -> crate::artifacts::step::engine::brep::BrepMeshView {
        crate::artifacts::step::engine::brep::analyze_brep_mesh(&self.document)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn step_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.step",
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
