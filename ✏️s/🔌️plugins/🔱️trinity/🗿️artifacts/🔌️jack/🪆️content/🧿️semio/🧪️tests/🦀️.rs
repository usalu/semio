//! 🧪️ The Semio graph owner of a Jack content child is built by its factory, replaced by materialization and read back without a copy.
use super::*;
use jack::standards::v1::subsets::any::schema::content::{jack_content_for_handle, retire_owned_to_terminal, JackContentFactory};
use jack::standards::v1::subsets::any::schema::content_read::JackContentProjection;

fn node(id: &str) -> Node {
    Node { id: id.into(), kind: "k".into(), name: id.into(), x: 1.0, y: 2.0, width: 3.0, height: 4.0, ports: Vec::new(), properties: PropertyBag::new() }
}

fn edge(id: &str, source: &str, target: &str) -> Edge {
    Edge { id: id.into(), kind: "e".into(), source: source.into(), target: target.into(), properties: PropertyBag::new() }
}

#[test]
fn the_factory_child_reads_back_the_working_scene() {
    let child = <SemioJackContent as JackContentFactory>::child_from_working(vec![node("a"), node("b")], vec![edge("e", "a", "b")]);
    let owner = jack_content_for_handle(&child).expect("factory child admits its owner");
    assert_eq!(owner.snapshot().nodes.len(), 2);
    assert_eq!(owner.snapshot().edges.len(), 1);
    assert_eq!(owner.read().projection_counts(), (2, 1));
    drop(owner);
    retire_owned_to_terminal(child).expect("factory child retires");
}

#[test]
fn materialization_replaces_the_retained_graph_under_the_same_address() {
    let mut child = <SemioJackContent as JackContentFactory>::child_from_working(vec![node("a")], Vec::new());
    let address = child.child_id.clone();
    materialize_jack_snapshot(&mut child, jack_content_snapshot_from_working(&[node("a"), node("b"), node("c")], &[edge("e", "a", "c")]));
    assert_eq!(child.child_id, address);
    let owner = jack_content_for_handle(&child).expect("materialized child admits its owner");
    let ids: Vec<&str> = owner.snapshot().nodes.iter().map(|node| node.id.value.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c"]);
    drop(owner);
    retire_owned_to_terminal(child).expect("materialized child retires");
}

#[test]
fn the_working_scene_projects_back_losslessly() {
    let nodes = vec![node("a"), node("b")];
    let edges = vec![edge("e", "a", "b")];
    let (projected_nodes, projected_edges) = working_from_jack_content_snapshot(&jack_content_snapshot_from_working(&nodes, &edges)).expect("projection");
    assert_eq!(projected_nodes, nodes);
    assert_eq!(projected_edges, edges);
}
