//! ↩️ `replace-model` inverse — restores the pre-replace `model_json` from BASE state; `replace` is
//! its own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::model::mutations::replace_model::mutation::ReplaceModel;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ReplaceModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    vec![EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json: base.model_json.clone() })]
}
//#endregion 🔖️Inverse
