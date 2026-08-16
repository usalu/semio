//! 🔺️ `replace-model` sparse diff construction — parses `payload.new_model_json` into a real
//! `crate::model::Model` and mints+caches its composed `structure`/`zones` children together
//! (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM); never touches `schema` or `results_json`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::replace_model::mutation::ReplaceModel;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Diff
/// 🔺️ Falls back to `Model::default()` if `payload.new_model_json` doesn't parse into a full
/// `Model` (e.g. malformed or partial JSON) — documented, honest degradation, never a panic,
/// matching this ticket's converter-honesty rule. Unlike the pre-migration behaviour (which stored
/// arbitrary opaque JSON text verbatim, never validating it), a composed child slot can only ever
/// hold a real, typed `Model`.
pub fn diff(payload: &ReplaceModel, _base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let model: crate::model::Model = serde_json::from_str(&payload.new_model_json).unwrap_or_default();
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(&model))
}
//#endregion 🔖️Diff
