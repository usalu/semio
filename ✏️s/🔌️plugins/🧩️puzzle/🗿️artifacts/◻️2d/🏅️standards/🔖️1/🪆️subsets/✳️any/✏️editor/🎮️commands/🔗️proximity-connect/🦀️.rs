//! 🔗️ `proximity-connect` command, and the proximity search the node drop and `translateSelection` share.

use crate::editor::puzzle2d::{
    fixture_nodes, new_edge_id, puzzle2d_handle_world_position, puzzle2d_kinds_compatible, puzzle2d_node_reach, puzzle2d_occupied_handles, puzzle2d_push_edge, Puzzle2dActionCtx, PUZZLE2D_PROXIMITY_CONNECT_MAX, PUZZLE2D_PROXIMITY_GESTURE_MAX,
};
use serde_json::{json, Value};
use std::collections::HashSet;

/// 🧲️ One admitted proximity pair: an open handle of the moved node and the stationary open handle
/// it snapped onto.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle2dProximityPair {
    pub moved: String,
    pub peer: String,
}

/// 🔎️ Every handle of `node` as `(id, kind, x, y)`, skipping hidden and locked handles.
fn open_handles(node: &Value) -> Vec<(String, String, f64, f64)> {
    node.get("handles")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|handle| handle.get("hidden").and_then(Value::as_bool) != Some(true) && handle.get("locked").and_then(Value::as_bool) != Some(true))
        .filter_map(|handle| {
            let id = handle.get("id").and_then(Value::as_str)?.to_string();
            let kind = handle.get("handleKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let (x, y) = puzzle2d_handle_world_position(node, handle);
            Some((id, kind, x, y))
        })
        .collect()
}

fn centre(node: &Value) -> (f64, f64) {
    (node.get("x").and_then(Value::as_f64).unwrap_or(0.0), node.get("y").and_then(Value::as_f64).unwrap_or(0.0))
}

/// 🔎️ The proximity search: for every unoccupied handle of `node_id`, the NEAREST unoccupied handle
/// of another node within `radius` whose kind `meta.kindCompatibility` admits. The search is
/// spatially bounded before any handle is placed — every handle of a node lies inside the circle of
/// [`puzzle2d_node_reach`] around its centre, so a node whose circle misses the query ball is
/// rejected on its centre alone — and yields at most [`PUZZLE2D_PROXIMITY_CONNECT_MAX`] pairs, which
/// is what `extent` prices. Each handle is claimed at most once across the whole result.
pub fn puzzle2d_proximity_pairs(fixture: &Value, node_id: &str, radius: f64) -> Vec<Puzzle2dProximityPair> {
    let radius = radius.max(0.0);
    if radius == 0.0 {
        return Vec::new();
    }
    let nodes = fixture_nodes(fixture);
    let Some(moved) = nodes.iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id)) else {
        return Vec::new();
    };
    if moved.get("hidden").and_then(Value::as_bool) == Some(true) || moved.get("locked").and_then(Value::as_bool) == Some(true) {
        return Vec::new();
    }
    let moved_handles = open_handles(moved);
    if moved_handles.is_empty() {
        return Vec::new();
    }
    let (moved_x, moved_y) = centre(moved);
    let moved_reach = puzzle2d_node_reach(moved);
    let mut claimed = puzzle2d_occupied_handles(fixture);
    let mut reach: Vec<&Value> = Vec::new();
    for node in nodes {
        if node.get("id").and_then(Value::as_str) == Some(node_id) || node.get("hidden").and_then(Value::as_bool) == Some(true) {
            continue;
        }
        let (x, y) = centre(node);
        let bound = radius + moved_reach + puzzle2d_node_reach(node);
        if (x - moved_x).powi(2) + (y - moved_y).powi(2) > bound * bound {
            continue;
        }
        reach.push(node);
    }
    let peers: Vec<(String, String, f64, f64)> = reach.into_iter().flat_map(open_handles).collect();
    let squared = radius * radius;
    let mut pairs: Vec<Puzzle2dProximityPair> = Vec::new();
    for (id, kind, x, y) in moved_handles {
        if pairs.len() >= PUZZLE2D_PROXIMITY_CONNECT_MAX {
            break;
        }
        if claimed.contains(&id) {
            continue;
        }
        let best = peers
            .iter()
            .filter(|(peer_id, peer_kind, _, _)| !claimed.contains(peer_id) && *peer_id != id && puzzle2d_kinds_compatible(fixture, &kind, peer_kind))
            .map(|(peer_id, _, peer_x, peer_y)| (peer_id, (peer_x - x).powi(2) + (peer_y - y).powi(2)))
            .filter(|(_, distance)| *distance <= squared)
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(peer_id, _)| peer_id.clone());
        if let Some(peer) = best {
            claimed.insert(id.clone());
            claimed.insert(peer.clone());
            pairs.push(Puzzle2dProximityPair { moved: id, peer });
        }
    }
    pairs
}

/// 🧲️ Applies [`puzzle2d_proximity_pairs`] for every id in `node_ids`, splicing one edge per pair
/// until the gesture's [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] budget is spent — the fixed figure
/// `extent` prices, so a whole-selection move never claims more work than a single-node drop. The
/// stationary peer stays `source`, so the pre-existing structure remains the resolution root exactly
/// as puzzle3d's relocate auto-attract keeps it. Returns the number of edges created.
pub fn puzzle2d_proximity_connect(fixture: &mut Value, node_ids: &[String], radius: f64) -> usize {
    let mut created = 0usize;
    let mut seen: HashSet<String> = HashSet::new();
    for node_id in node_ids {
        if !seen.insert(node_id.clone()) {
            continue;
        }
        for pair in puzzle2d_proximity_pairs(fixture, node_id, radius) {
            if created >= PUZZLE2D_PROXIMITY_GESTURE_MAX {
                return created;
            }
            let id = new_edge_id(fixture);
            puzzle2d_push_edge(fixture, json!({ "id": id, "source": pair.peer, "target": pair.moved }));
            created += 1;
        }
    }
    created
}

/// 📡️ The programmatic twin of the drop-time auto-connect: connects one node's (or the live
/// selection's) open handles to whatever compatible open handle is inside `radius`.
pub fn proximity_connect(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let explicit: Vec<String> = args
        .and_then(|value| value.get("nodeId").or_else(|| value.get("id")))
        .and_then(Value::as_str)
        .map(|id| vec![id.to_string()])
        .unwrap_or_else(|| args.and_then(|value| value.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default());
    let node_ids = if explicit.is_empty() { ctx.selected_ids() } else { explicit };
    if node_ids.is_empty() {
        return;
    }
    let radius = args.and_then(|value| value.get("radius")).and_then(Value::as_f64).filter(|radius| radius.is_finite()).unwrap_or(ctx.scene.runtime.proximity_radius);
    puzzle2d_proximity_connect(&mut ctx.scene.fixture, &node_ids, radius);
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
