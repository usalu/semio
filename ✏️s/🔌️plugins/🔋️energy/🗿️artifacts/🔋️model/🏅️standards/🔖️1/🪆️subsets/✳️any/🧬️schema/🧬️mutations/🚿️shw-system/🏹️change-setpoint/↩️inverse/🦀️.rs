//! ↩️ Inverse for `ChangeShwSystemSetpoint` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeShwSystemSetpoint, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeShwSystemSetpoint>,
{
    Ok((|| {
    match base.model.shw_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.setpoint_c != payload.new_setpoint_c && !(!payload.new_setpoint_c.is_finite() || !(0.0..=100.0).contains(&payload.new_setpoint_c)) => vec![vocabulary::change_shw_system_setpoint(payload.id, item.setpoint_c)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
