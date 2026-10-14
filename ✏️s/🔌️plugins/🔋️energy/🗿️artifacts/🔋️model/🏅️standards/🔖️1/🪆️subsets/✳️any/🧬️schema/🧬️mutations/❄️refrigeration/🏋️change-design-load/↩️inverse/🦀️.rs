//! ↩️ Inverse for `ChangeRefrigerationSystemDesignLoad` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeRefrigerationSystemDesignLoad, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeRefrigerationSystemDesignLoad>,
{
    Ok((|| {
    match base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_load_w != payload.new_design_load_w && !(!payload.new_design_load_w.is_finite() || payload.new_design_load_w <= 0.0) => vec![vocabulary::change_refrigeration_system_design_load(payload.id, item.design_load_w)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
