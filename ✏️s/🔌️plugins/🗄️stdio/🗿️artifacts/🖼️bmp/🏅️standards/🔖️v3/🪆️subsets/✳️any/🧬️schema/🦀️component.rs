//! 🧬️ BmpArtifact schema — full artifact state.

use crate::artifacts::bmp::BmpSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.bmp` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot {
            schema: self.schema.clone(),
            width: self.width.clone(),            height: self.height.clone(),            pixels: self.pixels.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,            height: snapshot.height,            pixels: snapshot.pixels,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;        self.height = snapshot.height;        self.pixels = snapshot.pixels;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.bmp`.
pub fn bmp_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.bmp",
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
