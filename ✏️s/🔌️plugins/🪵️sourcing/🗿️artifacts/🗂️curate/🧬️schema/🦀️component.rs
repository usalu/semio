//! 🧬️ Curate artifact schema — every field of the artifact with its state class.

use crate::artifacts::curate::{CuratedItem, Filters, ObjectKind};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full curate artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateArtifact {
    #[state(persistent)]
    pub stock: Vec<ObjectKind>,
    #[state(persistent)]
    pub curated: Vec<CuratedItem>,
    #[state(local_ui)]
    pub filters: Filters,
    #[state(shared_ui)]
    pub selected_object_id: Option<String>,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for CurateArtifact {
    fn default() -> Self {
        Self {
            stock: Vec::new(),
            curated: Vec::new(),
            filters: Filters::default(),
            selected_object_id: None,
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

impl CurateArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::curate::CurateSnapshot {
        crate::artifacts::curate::CurateSnapshot {
            stock: self.stock.clone(),
            curated: self.curated.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::curate::CurateSnapshot) -> Self {
        Self {
            stock: snapshot.stock,
            curated: snapshot.curated,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::curate::CurateSnapshot) {
        self.stock = snapshot.stock;
        self.curated = snapshot.curated;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sourcing.curate` — fifteen handcrafted schema leaves.
pub fn curate_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.sourcing.curate",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
