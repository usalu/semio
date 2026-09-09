use super::*;
use crate::{Edge, Node, Port, PortDirection, PropertyBag};

//#region 🧸️Fixtures
fn node(id: &str) -> Node {
    Node {
        id: id.into(),
        kind: "Piece".into(),
        name: id.into(),
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
        properties: PropertyBag::new(),
        ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }, Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
    }
}

fn edge(id: &str, source: &str, target: &str) -> Edge {
    Edge { id: id.into(), kind: "Connection".into(), source: source.into(), target: target.into(), properties: PropertyBag::new() }
}

fn chain_snapshot() -> JackSnapshot {
    // root -e1- mid -e2- leaf: a 3-node chain.
    JackSnapshot::with_content(
        "trinity.graph".into(),
        "chain".into(),
        None,
        Default::default(),
        Default::default(),
        vec![node("root"), node("mid"), node("leaf")],
        vec![edge("e1", "root@out", "mid@in"), edge("e2", "mid@out", "leaf@in")],
        Some("root".into()),
    )
}
//#endregion 🧸️Fixtures

//#region 🧪️TopologyLaws
#[semio_framework_async_macros::async_test]
async fn chain_is_cycle_free_with_increasing_depth() {
    let topology = compute_topology(&chain_snapshot());
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
    assert_eq!(topology.topo_order, vec!["root".to_string(), "mid".to_string(), "leaf".to_string()]);
    assert_eq!(topology.depth.get("root"), Some(&0));
    assert_eq!(topology.depth.get("mid"), Some(&1));
    assert_eq!(topology.depth.get("leaf"), Some(&2));
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_reported_as_not_cycle_free() {
    let snapshot = chain_snapshot();
    let mut edges = snapshot.edges();
    edges.push(edge("e3", "leaf@out", "root@in"));
    let snapshot = JackSnapshot::with_content(snapshot.schema.clone(), snapshot.name.clone(), snapshot.manifest_id.clone(), snapshot.manifest.clone(), snapshot.camera.clone(), snapshot.nodes(), edges, snapshot.root_node_id.clone());
    let topology = compute_topology(&snapshot);
    assert!(!topology.cycle_free);
    assert!(topology.topo_order.is_empty(), "every node in the 3-cycle has nonzero indegree");
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_yields_default_topology() {
    assert_eq!(compute_topology(&JackSnapshot::default()), JackTopology::default());
}
//#endregion 🧪️TopologyLaws
