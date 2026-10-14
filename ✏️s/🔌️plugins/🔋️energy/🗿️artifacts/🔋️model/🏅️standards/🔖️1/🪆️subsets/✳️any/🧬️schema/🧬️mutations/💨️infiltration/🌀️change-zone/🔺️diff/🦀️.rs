//! 🔺️ Sparse diff builder for `ChangeInfiltrationZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, InfiltrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Infiltration {} already carries this zone reference: {}.", payload.id.0, payload.new_zone_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { infiltrations: Rows::modifying(InfiltrationPatch { zone_id: Some(payload.new_zone_id), ..InfiltrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
