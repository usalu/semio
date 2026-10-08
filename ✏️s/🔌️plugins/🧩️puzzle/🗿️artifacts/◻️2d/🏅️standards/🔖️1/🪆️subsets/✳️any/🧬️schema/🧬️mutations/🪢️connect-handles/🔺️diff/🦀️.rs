//! 🔺️ Sparse diff builder for `ConnectHandles` — an exact ordered insert (never a
//! whole-snapshot capture). No-op when the id already exists in `base`. A payload that states a `tolerance` connects
//! on any base and warns `mutation.precondition-drifted` where its two handles are no longer within it.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::{Puzzle2dEdge, Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_finite,puzzle2d_handle_distance};


//#region 🔖️Diff
pub fn diff(payload: &super::ConnectHandles, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("gap", payload.gap), ("shift", payload.shift), ("rise", payload.rise), ("rotation", payload.rotation), ("turn", payload.turn), ("tilt", payload.tilt), ("x", payload.x), ("y", payload.y), ("tolerance", payload.tolerance.unwrap_or(0.0))]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    if payload.tolerance.is_some_and(|tolerance| tolerance < 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "tolerance must not be negative", vec![payload.id.to_string_owner()]);
    }
    if base.edges.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.id.to_string_owner()])]);
    }
    let edge = Puzzle2dEdge {
        id: payload.id.clone(),
        source: payload.source.clone(),
        target: payload.target.clone(),
        edge_kind: payload.edge_kind.clone(),
        gap: payload.gap,
        shift: payload.shift,
        rise: payload.rise,
        rotation: payload.rotation,
        turn: payload.turn,
        tilt: payload.tilt,
        x: payload.x,
        y: payload.y,
        source_tip: payload.source_tip.clone(),
        target_tip: payload.target_tip.clone(),
        visible: None,
        locked: None,
    };
    let reordered = payload.index.filter(|index| *index < base.edges.len()).map(|index| {
        let mut order: Vec<_> = base.edges.iter().map(|edge| edge.id.clone()).collect();
        order.insert(index, payload.id.clone());
        order
    });
    let delta = Puzzle2dEdgesDelta::adding(edge, reordered);
    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(delta), ..Default::default() }).absorb_messages(drift(payload, base))
}

/// 🧲️ The warning of a recorded proximity that no longer holds on `base`: the payload states a `tolerance`, and its two
/// handles lie farther apart than it — or one of them is on no node. Names both handles.
fn drift(payload: &super::ConnectHandles, base: &Puzzle2dSnapshot) -> Option<protocol::MutationMessage> {
    let tolerance = payload.tolerance?;
    let reason = match puzzle2d_handle_distance(base, &payload.source, &payload.target) {
        Some(distance) if distance <= tolerance => return None,
        Some(distance) => format!("handles {:?} and {:?} are {distance} apart, beyond the tolerance {tolerance} their connection was recorded under", payload.source, payload.target),
        None => format!("handles {:?} and {:?} are not both on a node, so the tolerance {tolerance} their connection was recorded under cannot hold", payload.source, payload.target),
    };
    Some(protocol::MutationMessage::warning(protocol::OutcomeCode::PreconditionDrifted, reason).at(vec![payload.source.to_string_owner(), payload.target.to_string_owner()]))
}
//#endregion 🔖️Diff
