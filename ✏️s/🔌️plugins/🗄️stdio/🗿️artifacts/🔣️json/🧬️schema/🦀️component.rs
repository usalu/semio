//! 🧬️ JsonArtifact schema — full artifact state.

use crate::artifacts::json::{JsonSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.json` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub value: serde_json::Value,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for JsonArtifact {
    fn default() -> Self {
        Self::from_snapshot(JsonSnapshot::default())
    }
}

impl JsonArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> JsonSnapshot {
        JsonSnapshot {
            schema: self.schema.clone(),
            value: self.value.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: JsonSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            value: snapshot.value,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: JsonSnapshot) {
        self.schema = snapshot.schema;
        self.value = snapshot.value;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.json`.
pub fn json_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.json",
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
