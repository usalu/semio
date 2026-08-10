//! 🧬️ BcfArtifact schema — full artifact state.

use crate::artifacts::bcf::schema::snapshot::{BcfEntry, BcfTopic};
use crate::artifacts::bcf::BcfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.bcf` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf")]
pub struct BcfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<BcfEntry>,
    #[state(persistent)]
    #[serde(default)]
    pub topics: Vec<BcfTopic>,
}
//#endregion Artifact

//#region Conversions
impl Default for BcfArtifact {
    fn default() -> Self {
        Self::from_snapshot(BcfSnapshot::default())
    }
}

impl BcfArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> BcfSnapshot {
        BcfSnapshot {
            schema: self.schema.clone(),
            entries: self.entries.clone(),
            topics: self.topics.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: BcfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            entries: snapshot.entries,
            topics: snapshot.topics,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: BcfSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
        self.topics = snapshot.topics;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.bcf`.
pub fn bcf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.bcf",
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
//#endregion Descriptor
