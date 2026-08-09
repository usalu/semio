//! 🧬️ EnergyModel artifact schema — every field with its state class.

use crate::artifacts::model::{EnergyModelSnapshot, ENERGY_MODEL_ARTIFACT_SCHEMA_ID};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full energy-model artifact across persistent and preview classes (no UI app).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelArtifact {
    #[state(persistent)]
    pub schema: String,
    /// 🏢️ Opaque JSON of `crate::Model` — building inputs that persist.
    #[state(persistent)]
    pub model_json: String,
    /// 📋️ Opaque JSON of `crate::Results` — recomputed by the BEM engine; never persisted.
    #[state(preview)]
    pub results_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for EnergyModelArtifact {
    fn default() -> Self {
        Self::from_snapshot(EnergyModelSnapshot::default())
    }
}

impl EnergyModelArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> EnergyModelSnapshot {
        EnergyModelSnapshot {
            schema: self.schema.clone(),
            model_json: self.model_json.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving preview empty.
    pub fn from_snapshot(snapshot: EnergyModelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            model_json: snapshot.model_json,
            results_json: String::new(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: EnergyModelSnapshot) {
        self.schema = snapshot.schema;
        self.model_json = snapshot.model_json;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.energy.model` — fifteen handcrafted schema leaves.
pub fn energy_model_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: ENERGY_MODEL_ARTIFACT_SCHEMA_ID,
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
