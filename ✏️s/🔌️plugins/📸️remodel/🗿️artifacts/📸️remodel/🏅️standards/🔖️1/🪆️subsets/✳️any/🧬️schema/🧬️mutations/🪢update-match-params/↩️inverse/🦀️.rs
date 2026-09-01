//! ↩️ Inverse for `UpdateMatchParams` — the OLD `MatchParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMatchParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_match_params(base.params.matching.clone())]
}
//#endregion 🔖️Inverse
