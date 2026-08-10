//! 🧬️ MdArtifact schema — full artifact state.

use crate::artifacts::md::{MdSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.md` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    PLACEHOLDER_PUB_VALUE PLACEHOLDER_VALUE_TYPE,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for MdArtifact {
    fn default() -> Self {
        Self::from_snapshot(MdSnapshot::default())
    }
}

impl MdArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> MdSnapshot {
        MdSnapshot {
            schema: self.schema.clone(),
            PLACEHOLDER_VALUE_COLON PLACEHOLDER_SELF_VALUE.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: MdSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            PLACEHOLDER_VALUE_COLON snapshot.value,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: MdSnapshot) {
        self.schema = snapshot.schema;
        PLACEHOLDER_SELF_VALUE = snapshot.value;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.md`.
pub fn md_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.md",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            md_schema: include_str!("🔣️component.md"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            md_schema: include_str!("📸️snapshot/🔣️component.md"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            md_schema: include_str!("🔺️diff/🔣️component.md"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
