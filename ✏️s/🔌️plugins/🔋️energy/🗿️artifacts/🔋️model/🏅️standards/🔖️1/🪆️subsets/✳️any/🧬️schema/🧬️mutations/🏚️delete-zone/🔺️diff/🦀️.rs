//! 🔺️ Sparse diff builder for `DeleteZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.spaces.iter().any(|item| item.zone_id == payload.id) || base.model.surfaces.iter().any(|item| item.zone_id == payload.id) || base.model.people.iter().any(|item| item.zone_id == payload.id) || base.model.lighting.iter().any(|item| item.zone_id == payload.id) || base.model.equipment.iter().any(|item| item.zone_id == payload.id) || base.model.thermostats.iter().any(|item| item.zone_id == payload.id) || base.model.humidistats.iter().any(|item| item.zone_id == payload.id) || base.model.ideal_loads.iter().any(|item| item.zone_id == payload.id) || base.model.zone_equipment.iter().any(|item| item.zone_id == payload.id) || base.model.infiltrations.iter().any(|item| item.zone_id == payload.id) || base.model.mechanical_ventilations.iter().any(|item| item.zone_id == payload.id) || base.model.sizing_objects.iter().any(|item| item.zone_id == payload.id) || base.model.daylight_zones.iter().any(|item| item.zone_id == payload.id) || base.model.room_air_models.iter().any(|item| item.zone_id == payload.id) || base.model.thermal_enclosures.iter().any(|item| item.zone_ids.contains(&payload.id)) || base.model.air_loops.iter().any(|item| item.terminal_zone_ids.contains(&payload.id)) || base.model.airflow_network.as_ref().is_some_and(|network| network.zone_node_ids.iter().any(|(zone, _)| *zone == payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} is still referenced by another entity; delete or reassign those first.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.zones.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
