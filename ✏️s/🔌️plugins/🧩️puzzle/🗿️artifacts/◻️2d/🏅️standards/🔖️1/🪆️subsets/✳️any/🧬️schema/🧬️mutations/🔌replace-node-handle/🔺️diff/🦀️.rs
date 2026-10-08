//! 🔺️ Sparse diff builder for `ReplaceNodeHandle` — patches the presentation fields of one handle inside the owner
//! node. An absent node or an absent handle is `mutation.target-missing`; a replacement that keeps every field is the
//! `mutation.no-op` warning. A replacement keeps the addressed handle's id.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dHandlePatch, Puzzle2dHandlesDelta, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_handle_invariant;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_handle_invariant(&payload.new_handle) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.node_id.to_string_owner(), payload.handle_id.to_string_owner()]);
    }
    if payload.new_handle.id != payload.handle_id {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a replacement handle keeps the addressed handle's id", vec![payload.node_id.to_string_owner(), payload.handle_id.to_string_owner()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.to_string_owner()]);
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.to_string_owner()]);
    };
    let next = &payload.new_handle;
    let patch = Puzzle2dHandlePatch {
        handle_kind: (next.handle_kind != handle.handle_kind).then(|| next.handle_kind.clone()),
        angle: (next.angle != handle.angle).then_some(next.angle),
        radius: (next.radius != handle.radius).then_some(next.radius),
        color: (next.color != handle.color).then(|| next.color.clone()),
        icon_kind: (next.icon_kind != handle.icon_kind).then(|| next.icon_kind.clone()),
        scale: (next.scale != handle.scale).then_some(next.scale),
        visible: (next.visible != handle.visible).then_some(next.visible),
        locked: (next.locked != handle.locked).then_some(next.locked),
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.node_id.to_string_owner()])]);
    }
    let node_patch = Puzzle2dNodePatch { handles: Some(Puzzle2dHandlesDelta::patching(payload.handle_id.clone(), patch)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle2dDiff { nodes: Some(Puzzle2dNodesDelta::patching(payload.node_id.clone(), node_patch)), ..Default::default() })
}
//#endregion 🔖️Diff
