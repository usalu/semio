//! 🔺️ Sparse diff builder for `RenameSpaceList` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SpaceListPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameSpaceList, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.space_lists.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space list {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A space list name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.space_lists.iter().any(|other| other.id != payload.id && other.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another space list is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Space list {} already carries this name: {}.", payload.id.0, payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { space_lists: Rows::modifying(SpaceListPatch { name: Some(payload.new_name.clone()), ..SpaceListPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
