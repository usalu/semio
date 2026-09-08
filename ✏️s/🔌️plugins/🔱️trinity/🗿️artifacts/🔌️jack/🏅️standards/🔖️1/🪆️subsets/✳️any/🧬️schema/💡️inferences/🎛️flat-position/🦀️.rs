//! 🎛 `flat-position` — one named inference: each node's flattened `(u, v)` position, BFS-walked
//! outward from `root_node_id` (or the first node) along `edges`' own `u`/`v` offsets, re-seeding
//! once per remaining disconnected component (preferring a component root with no incoming edge
//! from what is left, so a component that is itself a cycle still terminates). Ported verbatim from
//! the former `Graph::recompute_derived` + its `has_incoming_from_remaining` /
//! `extend_flat_positions_from_seed` helpers — deleted alongside `DerivedPropertyReadonly` and the
//! nakagin manifest's `flatPosition` `"derived"` property declaration (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). A plain
//! whole-snapshot BFS pass — per the family root's own "simple whole-snapshot scalars" guidance,
//! same rationale the sibling `🧭topology` states for itself — so no `InferredField`/incremental
//! caching is needed here either.

use crate::{port_node_id, Edge, JackSnapshot, Node, PropertyValue};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️FlatPosition
/// 🎛 One node's flattened position.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JackFlatPositionUv {
    pub u: f64,
    pub v: f64,
}

/// 🎛 Flattened `(u, v)` position per node id — covers every connected component, keyed by node id.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JackFlatPosition {
    pub positions: BTreeMap<String, JackFlatPositionUv>,
}

/// 📐️ Computes `flat-position` directly from `nodes`/`edges`/`root_node_id` — deterministic because
/// both the remaining-node seed pick and each seed's BFS walk are always drawn from `BTreeMap`/
/// `BTreeSet` id order, never from `edges`'/`nodes`' own fixture order.
pub fn compute_flat_position(snapshot: &JackSnapshot) -> JackFlatPosition {
    let scene = crate::jack_working_scene(snapshot);
    if scene.nodes.is_empty() {
        return JackFlatPosition::default();
    }
    let nodes: BTreeMap<String, &Node> = scene.nodes.iter().map(|node| (node.id.clone(), node)).collect();
    let edges: BTreeMap<String, &Edge> = scene.edges.iter().map(|edge| (edge.id.clone(), edge)).collect();
    let mut flat: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    if let Some(root_id) = snapshot.root_node_id.clone().or_else(|| nodes.keys().next().cloned()) {
        extend_from_seed(&edges, &mut flat, root_id);
    }
    while flat.len() < nodes.len() {
        let remaining: BTreeSet<String> = nodes.keys().filter(|id| !flat.contains_key(*id)).cloned().collect();
        if remaining.is_empty() {
            break;
        }
        let seed = remaining.iter().find(|id| !has_incoming_from_remaining(&edges, id, &remaining)).cloned().unwrap_or_else(|| remaining.iter().next().expect("remaining non-empty").clone());
        extend_from_seed(&edges, &mut flat, seed);
    }
    let positions = flat.into_iter().filter(|(id, _)| nodes.contains_key(id)).map(|(id, (u, v))| (id, JackFlatPositionUv { u, v })).collect();
    JackFlatPosition { positions }
}

fn has_incoming_from_remaining(edges: &BTreeMap<String, &Edge>, node_id: &str, remaining: &BTreeSet<String>) -> bool {
    edges.values().any(|edge| {
        let Some(target_node) = port_node_id(&edge.target) else {
            return false;
        };
        if target_node != node_id {
            return false;
        }
        port_node_id(&edge.source).is_some_and(|source_node| remaining.contains(source_node))
    })
}

fn extend_from_seed(edges: &BTreeMap<String, &Edge>, flat: &mut BTreeMap<String, (f64, f64)>, seed_id: String) {
    if flat.contains_key(&seed_id) {
        return;
    }
    flat.insert(seed_id.clone(), (0.0, 0.0));
    let mut queue = vec![seed_id];
    while let Some(parent_id) = queue.pop() {
        let (pu, pv) = flat.get(&parent_id).copied().unwrap_or((0.0, 0.0));
        let child_edges: Vec<(String, f64, f64)> = edges
            .values()
            .filter_map(|edge| {
                let source_node = port_node_id(&edge.source)?;
                let target_node = port_node_id(&edge.target)?;
                if source_node == parent_id {
                    let u = edge.properties.get("u").and_then(PropertyValue::as_f64).unwrap_or(0.0);
                    let v = edge.properties.get("v").and_then(PropertyValue::as_f64).unwrap_or(0.0);
                    return Some((target_node.to_string(), pu + u, pv + v));
                }
                None
            })
            .collect();
        for (child_id, cu, cv) in child_edges {
            if !flat.contains_key(&child_id) {
                flat.insert(child_id.clone(), (cu, cv));
                queue.push(child_id);
            }
        }
    }
}
//#endregion 🔖️FlatPosition

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
