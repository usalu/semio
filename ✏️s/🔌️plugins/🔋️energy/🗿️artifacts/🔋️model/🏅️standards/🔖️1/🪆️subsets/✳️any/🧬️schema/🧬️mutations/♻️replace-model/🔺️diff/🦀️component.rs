//! 🔺️ `replace-model` sparse diff construction — writes only `EnergyModelDiff.model_json` from the
//! payload; never touches `schema` or `results_json`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::replace_model::mutation::ReplaceModel;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceModel, _base: &EnergyModelSnapshot) -> EnergyModelDiff {
    EnergyModelDiff { model_json: Some(payload.new_model_json.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
