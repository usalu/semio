//! 🧬️ DwgArtifact schema — full artifact state.

use crate::artifacts::dwg::DwgSnapshot;
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
            // 🚧️ ac1018 is a legacy shim (nothing real behind it, per Decision #5) — it never ran
            // the real ac1024 D1/D2 decode pipeline, so it has no structural insight to carry.
            sections: Vec::new(),
            decode_status: Default::default(),
        }
    }

    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            bytes: snapshot.bytes,
            section_names: snapshot.section_names,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.bytes = snapshot.bytes;
        self.section_names = snapshot.section_names;
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
