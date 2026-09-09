//! 🔺️ Sparse diff builder for `ChangeInfiltrationFlowPerExteriorArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationFlowPerExteriorArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_flow_per_exterior_area_m3_s_m2.is_finite() || payload.new_flow_per_exterior_area_m3_s_m2 < 0.0 {
        return protocol::MutationOutcome::error(
            "mutation.invariant",
            format!("Infiltration {}: flow per exterior area (m³/s·m²) must be a finite non-negative value, got {}.", payload.id.0, payload.new_flow_per_exterior_area_m3_s_m2),
            [payload.id.0.to_string()],
        );
    }
    if existing.flow_per_exterior_area_m3_s_m2 == payload.new_flow_per_exterior_area_m3_s_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this flow per exterior area (m³/s·m²): {}.", payload.id.0, payload.new_flow_per_exterior_area_m3_s_m2));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.flow_per_exterior_area_m3_s_m2 = payload.new_flow_per_exterior_area_m3_s_m2;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
