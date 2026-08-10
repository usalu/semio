//! 🧬️ DwgArtifact schema — full artifact state.

use crate::artifacts::dwg::DwgSnapshot;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{DwgDecodeStatus, DwgSection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub version: String,
    #[state(persistent)]
    #[serde(default)]
    pub bytes: Vec<u8>,
    #[state(persistent)]
    #[serde(default)]
    pub section_names: Vec<String>,
    #[state(persistent)]
    #[serde(default)]
    pub sections: Vec<DwgSection>,
    #[state(persistent)]
    #[serde(default)]
    pub decode_status: DwgDecodeStatus,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DwgArtifact {
    fn default() -> Self {
        Self::from_snapshot(DwgSnapshot::default())
    }
}

impl DwgArtifact {
    pub fn to_snapshot(&self) -> DwgSnapshot {
        DwgSnapshot {
            schema: self.schema.clone(),
            version: self.version.clone(),
            bytes: self.bytes.clone(),
            section_names: self.section_names.clone(),
            sections: self.sections.clone(),
            decode_status: self.decode_status,
        }
    }

    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            bytes: snapshot.bytes,
            section_names: snapshot.section_names,
            sections: snapshot.sections,
            decode_status: snapshot.decode_status,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.bytes = snapshot.bytes;
        self.section_names = snapshot.section_names;
        self.sections = snapshot.sections;
        self.decode_status = snapshot.decode_status;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn dwg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dwg",
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
