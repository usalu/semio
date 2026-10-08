//! 🔺️ Sparse diff builder for `ChangeAirLoopReturnNode` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelAirLoopPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeAirLoopReturnNode, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_return_node_id == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A return node id must be at least one.".to_string(), [payload.id.0.to_string()]);
    }
    if existing.return_node_id == payload.new_return_node_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Air loop {} already has that return node.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { air_loops: Rows::modifying(ModelAirLoopPatch { return_node_id: Some(payload.new_return_node_id), ..ModelAirLoopPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
