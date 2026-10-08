//! 🔺️ Sparse diff builder for `ChangeHumidistatZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, HumidistatPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHumidistatZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Humidistat {} already has that zone.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { humidistats: Rows::modifying(HumidistatPatch { zone_id: Some(payload.new_zone_id), ..HumidistatPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
