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

use crate::artifacts::jack::{port_node_id, Edge, JackSnapshot, Node, PropertyValue};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️FlatPosition
/// 🎛 One node's flattened position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackFlatPositionUv {
    pub u: f64,
    pub v: f64,
}

/// 🎛 Flattened `(u, v)` position per node id — covers every connected component, keyed by node id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackFlatPosition {
    pub positions: BTreeMap<String, JackFlatPositionUv>,
}

/// 📐️ Computes `flat-position` directly from `nodes`/`edges`/`root_node_id` — deterministic because
/// both the remaining-node seed pick and each seed's BFS walk are always drawn from `BTreeMap`/
/// `BTreeSet` id order, never from `edges`'/`nodes`' own fixture order.
pub async fn compute_flat_position(snapshot: &JackSnapshot) -> JackFlatPosition {
    let scene = crate::artifacts::jack::jack_working_scene(snapshot);
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

async fn has_incoming_from_remaining(edges: &BTreeMap<String, &Edge>, node_id: &str, remaining: &BTreeSet<String>) -> bool {
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

async fn extend_from_seed(edges: &BTreeMap<String, &Edge>, flat: &mut BTreeMap<String, (f64, f64)>, seed_id: String) {
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
mod tests {
    use super::*;
    use crate::artifacts::jack::{Camera, Manifest, Port, PortDirection, PropertyBag};

    //#region 🧸️Fixtures
    async fn mini_fixture() -> JackSnapshot {
        JackSnapshot::with_content(
            JackSnapshot::SCHEMA.into(),
            "mini".into(),
            Some("nakagin".into()),
            Manifest::nakagin_default(),
            Camera::default(),
            vec![
                Node { id: "root".into(), kind: "Piece".into(), name: "core".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
                Node { id: "child".into(), kind: "Piece".into(), name: "capsule".into(), x: 120.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }] },
            ],
            vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root@out-a".into(),
                target: "child@in-a".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.2));
                    p.insert("v".into(), PropertyValue::Number(-0.6));
                    p
                },
            }],
            Some("root".into()),
        )
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️FlatPositionLaws
    #[semio_framework_async_macros::async_test]
    async fn flat_position_bfs_walks_from_root() {
        let flat = compute_flat_position(&mini_fixture());
        assert_eq!(flat.positions.get("root"), Some(&JackFlatPositionUv { u: 0.0, v: 0.0 }));
        assert_eq!(flat.positions.get("child"), Some(&JackFlatPositionUv { u: 1.2, v: -0.6 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_position_covers_disconnected_components() {
        let fixture = JackSnapshot::with_content(
            JackSnapshot::SCHEMA.into(),
            "disconnected".into(),
            Some("nakagin".into()),
            Manifest::nakagin_default(),
            Camera::default(),
            vec![
                Node { id: "root-a".into(), kind: "Piece".into(), name: "a".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
                Node { id: "child-a".into(), kind: "Piece".into(), name: "a-child".into(), x: 100.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }] },
                Node { id: "root-b".into(), kind: "Piece".into(), name: "b".into(), x: 300.0, y: 200.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
                Node { id: "child-b".into(), kind: "Piece".into(), name: "b-child".into(), x: 400.0, y: 200.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }] },
            ],
            vec![
                Edge { id: "e-a".into(), kind: "Connection".into(), source: "root-a@out".into(), target: "child-a@in".into(), properties: { let mut p = PropertyBag::new(); p.insert("u".into(), PropertyValue::Number(2.0)); p.insert("v".into(), PropertyValue::Number(1.0)); p } },
                Edge { id: "e-b".into(), kind: "Connection".into(), source: "root-b@out".into(), target: "child-b@in".into(), properties: { let mut p = PropertyBag::new(); p.insert("u".into(), PropertyValue::Number(3.0)); p.insert("v".into(), PropertyValue::Number(-1.0)); p } },
            ],
            Some("root-a".into()),
        );
        let flat = compute_flat_position(&fixture);
        assert_eq!(flat.positions.get("child-a"), Some(&JackFlatPositionUv { u: 2.0, v: 1.0 }));
        assert_eq!(flat.positions.get("child-b"), Some(&JackFlatPositionUv { u: 3.0, v: -1.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_position_handles_cycles_without_looping() {
        let fixture = JackSnapshot::with_content(
            JackSnapshot::SCHEMA.into(),
            "cycle".into(),
            Some("nakagin".into()),
            Manifest::nakagin_default(),
            Camera::default(),
            vec![
                Node { id: "a".into(), kind: "Piece".into(), name: "a".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
                Node { id: "b".into(), kind: "Piece".into(), name: "b".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
            ],
            vec![
                Edge { id: "ab".into(), kind: "Connection".into(), source: "a@out".into(), target: "b@out".into(), properties: { let mut p = PropertyBag::new(); p.insert("u".into(), PropertyValue::Number(1.0)); p.insert("v".into(), PropertyValue::Number(0.0)); p } },
                Edge { id: "ba".into(), kind: "Connection".into(), source: "b@out".into(), target: "a@out".into(), properties: PropertyBag::new() },
            ],
            Some("a".into()),
        );
        let flat = compute_flat_position(&fixture);
        assert!(flat.positions.contains_key("a"));
        assert!(flat.positions.contains_key("b"));
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_position_empty_snapshot_yields_default() {
        assert_eq!(compute_flat_position(&JackSnapshot::default()), JackFlatPosition::default());
    }
    //#endregion 🧪️FlatPositionLaws
}
//#endregion 🧪️Tests
