//! 🔺️ Sparse diff builder for `ChangeOutdoorAirSystemAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, OutdoorAirSystemPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeOutdoorAirSystemAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Outdoor air system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.air_loops.iter().any(|item| item.id == payload.new_air_loop_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.new_air_loop_id.0), [payload.new_air_loop_id.0.to_string()]);
    }
    if existing.air_loop_id == payload.new_air_loop_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Outdoor air system {} already has that air loop.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { outdoor_air_systems: Rows::modifying(OutdoorAirSystemPatch { air_loop_id: Some(payload.new_air_loop_id), ..OutdoorAirSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
