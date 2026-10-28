//! ↩️ Inverse for `ConnectSurfaces` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ConnectSurfaces, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ConnectSurfaces> + From<crate::DisconnectSurfaces>,
{
    Ok((|| {
    if payload.surface_a_id == payload.surface_b_id
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id)
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id)
        || base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id))
    {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.adjacency_pairs.len()) {
        return Vec::new();
    }
    vec![vocabulary::disconnect_surfaces(payload.surface_a_id, payload.surface_b_id)]

    })())
}
//#endregion 🔖️Inverse
