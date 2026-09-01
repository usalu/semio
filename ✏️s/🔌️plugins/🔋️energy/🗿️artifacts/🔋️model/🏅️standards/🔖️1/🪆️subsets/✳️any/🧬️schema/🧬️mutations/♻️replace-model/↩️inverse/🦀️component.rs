//! ↩️ `replace-model` inverse — restores the pre-replace model (re-serialized from BASE state's
//! working-scene `Model`, via `crate::artifacts::model::energy_model`) — `replace` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::model::mutations::replace_model::ReplaceModel;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ReplaceModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let model = crate::artifacts::model::energy_model(base);
    let new_model_json = pack::json::to_json_string(&model);
    vec![EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json })]
}
//#endregion 🔖️Inverse
