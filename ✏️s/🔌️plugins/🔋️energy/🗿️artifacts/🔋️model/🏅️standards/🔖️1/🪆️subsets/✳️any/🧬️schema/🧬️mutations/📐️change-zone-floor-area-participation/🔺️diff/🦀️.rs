//! 🔺️ Sparse diff builder for `ChangeZoneFloorAreaParticipation` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneFloorAreaParticipation, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.part_of_total_floor_area == payload.new_part_of_total_floor_area {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has partOfTotalFloorArea={}.", payload.id.0, payload.new_part_of_total_floor_area));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.part_of_total_floor_area = payload.new_part_of_total_floor_area;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
