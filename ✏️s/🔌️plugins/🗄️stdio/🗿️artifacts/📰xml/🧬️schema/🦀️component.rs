//! 🧬️ XmlArtifact schema — full artifact state.

use crate::artifacts::xml::{XmlSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.xml` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    PLACEHOLDER_PUB_VALUE PLACEHOLDER_VALUE_TYPE,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for XmlArtifact {
    fn default() -> Self {
        Self::from_snapshot(XmlSnapshot::default())
    }
}

impl XmlArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> XmlSnapshot {
        XmlSnapshot {
            schema: self.schema.clone(),
            PLACEHOLDER_VALUE_COLON PLACEHOLDER_SELF_VALUE.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: XmlSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            PLACEHOLDER_VALUE_COLON snapshot.value,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: XmlSnapshot) {
        self.schema = snapshot.schema;
        PLACEHOLDER_SELF_VALUE = snapshot.value;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.xml`.
pub fn xml_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.xml",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            xml_schema: include_str!("🔣️component.xml"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            xml_schema: include_str!("📸️snapshot/🔣️component.xml"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            xml_schema: include_str!("🔺️diff/🔣️component.xml"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
