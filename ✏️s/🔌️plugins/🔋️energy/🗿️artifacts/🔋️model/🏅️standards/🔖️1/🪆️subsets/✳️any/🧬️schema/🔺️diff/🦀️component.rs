//! 🧬️ EnergyModel diff schema — sparse field delta over the artifact.

use crate::artifacts::model::{EnergyStructureChild, EnergyZonesChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the energy-model artifact. `structure`/`zones` are always-present
/// slots (never absent, only ever replaced) — single-`Option`, matching `mathematical`'s/`forms`'s
/// diff shape. `referenced_model` uses the optional-slot double-`Option` shape (outer = "did the
/// presence/identity change", inner = "is it now present") per the migration recipe's §8
/// convention, matching `layout`'s own `referenced_model` diff field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::model::schema::EnergyModelArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub structure: Option<EnergyStructureChild>,
    #[state(artifact)]
    pub zones: Option<EnergyZonesChild>,
    #[state(artifact)]
    pub referenced_model: Option<Option<store::ArtifactLink>>,
    #[state(artifact)]
    pub results_json: Option<String>,
}
//#endregion 🔖️Diff
