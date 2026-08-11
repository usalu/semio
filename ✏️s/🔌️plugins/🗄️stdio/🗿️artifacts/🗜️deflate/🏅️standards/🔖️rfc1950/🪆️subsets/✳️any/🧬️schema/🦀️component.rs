//! 🧬️ DeflateArtifact schema — full artifact state.

use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::{DeflateSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.deflate` artifact state — mirrors `DeflateSnapshot`'s typed RFC1950 fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate")]
pub struct DeflateArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub compression_method: u8,
    #[state(persistent)]
    #[serde(default)]
    pub window_bits: u8,
    #[state(persistent)]
    #[serde(default)]
    pub compression_level_hint: DeflateLevelHint,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<u32>,
    #[state(persistent)]
    #[serde(default)]
    pub payload: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DeflateArtifact {
    fn default() -> Self {
        Self::from_snapshot(DeflateSnapshot::default())
    }
}

impl DeflateArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DeflateSnapshot {
        DeflateSnapshot {
            schema: self.schema.clone(),
            compression_method: self.compression_method,
            window_bits: self.window_bits,
            compression_level_hint: self.compression_level_hint,
            dict_id: self.dict_id,
            payload: self.payload.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: DeflateSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            compression_method: snapshot.compression_method,
            window_bits: snapshot.window_bits,
            compression_level_hint: snapshot.compression_level_hint,
            dict_id: snapshot.dict_id,
            payload: snapshot.payload,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DeflateSnapshot) {
        self.schema = snapshot.schema;
        self.compression_method = snapshot.compression_method;
        self.window_bits = snapshot.window_bits;
        self.compression_level_hint = snapshot.compression_level_hint;
        self.dict_id = snapshot.dict_id;
        self.payload = snapshot.payload;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.deflate`.
pub fn deflate_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.deflate",
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
