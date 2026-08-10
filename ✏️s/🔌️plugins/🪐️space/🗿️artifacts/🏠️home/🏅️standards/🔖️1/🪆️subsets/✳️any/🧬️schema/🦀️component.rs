//! 🧬️ S Home artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full S Home launcher artifact state across persistent and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub catalog_generation: u64,
    #[state(local_ui)]
    pub active_panel_tab: String,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SHomeArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::home::S_HOME_DOCUMENT_SCHEMA.into(),
            catalog_generation: 0,
            active_panel_tab: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl SHomeArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::home::SHomeSnapshot {
        crate::artifacts::home::SHomeSnapshot {
            schema: self.schema.clone(),
            catalog_generation: self.catalog_generation,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::home::SHomeSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            catalog_generation: snapshot.catalog_generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::home::SHomeSnapshot) {
        self.schema = snapshot.schema;
        self.catalog_generation = snapshot.catalog_generation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.home` — twenty handcrafted schema leaves.
pub fn home_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.space.home",
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
