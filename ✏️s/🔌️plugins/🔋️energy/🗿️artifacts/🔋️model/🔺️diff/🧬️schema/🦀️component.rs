//! 🧬️ EnergyModel diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the energy-model artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::model::schema::EnergyModelArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub model_json: Option<String>,
    #[state(preview)]
    pub results_json: Option<String>,
}
//#endregion 🔖️Diff
