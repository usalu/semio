//! ↩️ Inverse for `ChangePlantLoopDesignFlow` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePlantLoopDesignFlow, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePlantLoopDesignFlow>,
{
    Ok((|| {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_flow_kg_s != payload.new_design_flow_kg_s && !(!payload.new_design_flow_kg_s.is_finite() || payload.new_design_flow_kg_s <= 0.0) => vec![vocabulary::change_plant_loop_design_flow(payload.id, item.design_flow_kg_s)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
