//! 🧬️ BinaryArtifact schema — full artifact state.

use crate::artifacts::binary::{BinarySnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.binary` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinaryArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BinaryArtifact {
    fn default() -> Self {
        Self::from_snapshot(BinarySnapshot::default())
    }
}

impl BinaryArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> BinarySnapshot {
        BinarySnapshot {
            schema: self.schema.clone(),
            bytes: self.bytes.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: BinarySnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            bytes: snapshot.bytes,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: BinarySnapshot) {
        self.schema = snapshot.schema;
        self.bytes = snapshot.bytes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.binary`.
pub fn binary_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.binary",
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
