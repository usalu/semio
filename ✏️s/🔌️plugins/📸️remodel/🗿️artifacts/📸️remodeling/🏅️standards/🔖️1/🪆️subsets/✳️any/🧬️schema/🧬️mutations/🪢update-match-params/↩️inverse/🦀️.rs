//! ↩️ Inverse for `UpdateMatchParams` — the OLD `MatchParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMatchParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_match_params(base.params.matching.clone())]
}
//#endregion 🔖️Inverse
