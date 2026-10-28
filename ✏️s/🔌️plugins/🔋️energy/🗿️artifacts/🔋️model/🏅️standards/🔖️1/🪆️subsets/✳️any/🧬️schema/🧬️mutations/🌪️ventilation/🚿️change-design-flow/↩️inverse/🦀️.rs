//! ↩️ Inverse for `ChangeMechanicalVentilationDesignFlow` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeMechanicalVentilationDesignFlow, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeMechanicalVentilationDesignFlow>,
{
    Ok((|| {
    match base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_flow_m3_s != payload.new_design_flow_m3_s && !(!payload.new_design_flow_m3_s.is_finite() || payload.new_design_flow_m3_s < 0.0) => {
            vec![vocabulary::change_mechanical_ventilation_design_flow(payload.id, item.design_flow_m3_s)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
