//! 🧬️ EnergyModel artifact — mutation dispatch.

use crate::artifacts::model::diff::diff_set_snapshot;
use crate::artifacts::model::{EnergyModelDiff, EnergyModelSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for an `EnergyModelSnapshot`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum EnergyModelMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: EnergyModelSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, producing the next document.
pub fn apply_energy_model_mutation(
    snapshot: &mut EnergyModelSnapshot,
    mutation: &EnergyModelMutation,
) {
    match mutation {
        EnergyModelMutation::NoMutation => {}
        EnergyModelMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<EnergyModelSnapshot> for EnergyModelMutation {
    type Diff = EnergyModelDiff;

    fn diff(&self, _base: &EnergyModelSnapshot) -> Self::Diff {
        match self {
            EnergyModelMutation::NoMutation => EnergyModelDiff::default(),
            EnergyModelMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<Self> {
        match self {
            EnergyModelMutation::NoMutation => vec![EnergyModelMutation::NoMutation],
            EnergyModelMutation::SetSnapshot { .. } => {
                vec![EnergyModelMutation::SetSnapshot {
                    snapshot: base.clone(),
                }]
            }
        }
    }
}
//#endregion 🔖️MutationTrait
