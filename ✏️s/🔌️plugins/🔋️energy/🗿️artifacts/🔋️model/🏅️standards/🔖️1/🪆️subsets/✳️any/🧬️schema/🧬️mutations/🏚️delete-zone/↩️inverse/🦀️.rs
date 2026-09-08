//! ↩️ Inverse for `DeleteZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.zones.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.spaces.iter().any(|item| item.zone_id == payload.id) || base.model.surfaces.iter().any(|item| item.zone_id == payload.id) || base.model.people.iter().any(|item| item.zone_id == payload.id) || base.model.lighting.iter().any(|item| item.zone_id == payload.id) || base.model.equipment.iter().any(|item| item.zone_id == payload.id) || base.model.thermostats.iter().any(|item| item.zone_id == payload.id) || base.model.humidistats.iter().any(|item| item.zone_id == payload.id) || base.model.ideal_loads.iter().any(|item| item.zone_id == payload.id) || base.model.zone_equipment.iter().any(|item| item.zone_id == payload.id) || base.model.infiltrations.iter().any(|item| item.zone_id == payload.id) || base.model.mechanical_ventilations.iter().any(|item| item.zone_id == payload.id) || base.model.sizing_objects.iter().any(|item| item.zone_id == payload.id) || base.model.daylight_zones.iter().any(|item| item.zone_id == payload.id) || base.model.room_air_models.iter().any(|item| item.zone_id == payload.id) || base.model.thermal_enclosures.iter().any(|item| item.zone_ids.contains(&payload.id)) || base.model.air_loops.iter().any(|item| item.terminal_zone_ids.contains(&payload.id)) || base.model.airflow_network.as_ref().is_some_and(|network| network.zone_node_ids.iter().any(|(zone, _)| *zone == payload.id)) || false {
        return Vec::new();
    }
    vec![vocabulary::create_zone(existing.id, existing.name.clone(), existing.volume_m3, existing.multiplier, existing.conditioned, existing.part_of_total_floor_area)]
}
//#endregion 🔖️Inverse
