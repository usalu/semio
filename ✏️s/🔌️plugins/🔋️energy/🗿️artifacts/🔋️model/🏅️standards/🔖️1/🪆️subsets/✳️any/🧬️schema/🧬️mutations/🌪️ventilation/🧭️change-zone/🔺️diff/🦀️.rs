//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MechanicalVentilationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Mechanical Ventilation {} already carries this zone reference: {}.", payload.id.0, payload.new_zone_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { mechanical_ventilations: Rows::modifying(MechanicalVentilationPatch { zone_id: Some(payload.new_zone_id), ..MechanicalVentilationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
