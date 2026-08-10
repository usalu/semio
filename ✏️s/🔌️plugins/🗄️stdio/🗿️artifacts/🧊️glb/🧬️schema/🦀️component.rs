//! 🧬️ GlbArtifact schema — full artifact state.

use crate::artifacts::glb::schema::snapshot::GlbEntry;
use crate::artifacts::glb::GlbSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.glb` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb")]
pub struct GlbArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<GlbEntry>,
}
//#endregion Artifact

//#region Conversions
impl Default for GlbArtifact {
    fn default() -> Self {
        Self::from_snapshot(GlbSnapshot::default())
    }
}

impl GlbArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> GlbSnapshot {
        GlbSnapshot {
            schema: self.schema.clone(),
            entries: self.entries.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: GlbSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            entries: snapshot.entries,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: GlbSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.glb`.
pub fn glb_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.glb",
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
