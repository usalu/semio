//! 🧬️ JpgArtifact schema — full artifact state.

use crate::artifacts::jpg::schema::snapshot::JpgEntry;
use crate::artifacts::jpg::JpgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.jpg` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<JpgEntry>,
}
//#endregion Artifact

//#region Conversions
impl Default for JpgArtifact {
    fn default() -> Self {
        Self::from_snapshot(JpgSnapshot::default())
    }
}

impl JpgArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> JpgSnapshot {
        JpgSnapshot {
            schema: self.schema.clone(),
            entries: self.entries.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: JpgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            entries: snapshot.entries,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: JpgSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.jpg`.
pub fn jpg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.jpg",
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
