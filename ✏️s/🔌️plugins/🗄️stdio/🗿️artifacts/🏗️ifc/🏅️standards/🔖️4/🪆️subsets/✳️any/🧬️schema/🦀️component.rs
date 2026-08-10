//! 🧬️ IfcArtifact schema — full artifact state.

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
    #[state(persistent)]
    #[serde(default)]
    pub brep: crate::artifacts::ifc::schema::snapshot::BrepMesh,
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
            brep: self.brep.clone(),
        }
    }

    pub fn from_snapshot(snapshot: IfcSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            brep: snapshot.brep,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: IfcSnapshot) {
        self.schema = snapshot.schema;
        self.brep = snapshot.brep;
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
    }
}
//#endregion 🔖️Descriptor
