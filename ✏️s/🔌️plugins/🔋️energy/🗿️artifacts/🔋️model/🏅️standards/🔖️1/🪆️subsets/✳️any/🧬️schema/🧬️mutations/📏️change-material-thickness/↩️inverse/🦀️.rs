//! ↩️ Inverse for `ChangeMaterialThickness` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMaterialThickness, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.thickness_m != payload.new_thickness_m && !(!payload.new_thickness_m.is_finite() || payload.new_thickness_m <= 0.0) => vec![vocabulary::change_material_thickness(payload.id, item.thickness_m)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
