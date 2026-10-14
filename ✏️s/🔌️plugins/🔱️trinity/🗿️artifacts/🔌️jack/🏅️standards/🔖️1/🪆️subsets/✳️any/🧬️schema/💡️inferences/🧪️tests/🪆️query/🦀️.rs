//! 🧪️ Native inference shares the language-neutral graph and independent Graphology witnesses.
use super::*;
use crate::{infer_jack_flat_position, infer_jack_topology};
use serde_json::Value;
use std::cell::Cell;
#[global_allocator]
static ORIGINAL_QUERY_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

struct FixtureGraph {
    identity: [String; 5],
    nodes: Vec<String>,
    edges: Vec<(String, String, String, f64, f64)>,
    reads: Cell<usize>,
}

impl JackGraphQuery for FixtureGraph {
    fn identity(&self) -> ChildRefFields<'_> {
        ChildRefFields { child_id: &self.identity[0], artifact_id: &self.identity[1], artifact_kind: &self.identity[2], standard: &self.identity[3], subset: &self.identity[4] }
    }
    fn node_count(&self) -> usize { self.reads.set(self.reads.get() + 1); self.nodes.len() }
    fn node_id(&self, index: usize) -> Option<&str> { self.nodes.get(index).map(String::as_str) }
    fn edge_count(&self) -> usize { self.edges.len() }
    fn edge(&self, index: usize) -> Option<JackGraphEndpoint<'_>> { self.edges.get(index).map(|(id, source, target, _, _)| JackGraphEndpoint { id, source, target }) }
    fn displacement(&self, id: &str) -> Result<(f64, f64), ValueError> { self.edges.iter().find(|edge| edge.0 == id).map(|edge| (edge.3, edge.4)).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "absent fixture edge")) }
}

fn fixture() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🪆️query/🔣️.json")).unwrap() }
fn graph(case: &Value) -> FixtureGraph {
    let fixture = fixture();
    let identity = &fixture["identity"];
    FixtureGraph {
        identity: [identity["childId"].as_str().unwrap().into(), identity["target"]["artifactId"].as_str().unwrap().into(), identity["target"]["dialect"]["artifactKind"].as_str().unwrap().into(), identity["target"]["dialect"]["standard"].as_str().unwrap().into(), identity["target"]["dialect"]["subset"].as_str().unwrap().into()],
        nodes: case["nodes"].as_array().unwrap().iter().map(|node| node.as_str().unwrap().into()).collect(),
        edges: case["edges"].as_array().unwrap().iter().map(|edge| (edge["id"].as_str().unwrap().into(), edge["source"].as_str().unwrap().into(), edge["target"].as_str().unwrap().into(), edge["u"].as_f64().unwrap(), edge["v"].as_f64().unwrap())).collect(),
        reads: Cell::new(0),
    }
}

#[test]
fn graph_query_neutral_fixture_parity() {
    for case in fixture()["cases"].as_array().unwrap() {
        let owner = graph(case);
        let result = infer_jack_topology(owner.identity(), &owner).unwrap();
        assert_eq!(result.topo_order, case["topology"]["topoOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect::<Vec<_>>());
        assert_eq!(result.cycle_free, case["topology"]["cycleFree"].as_bool().unwrap());
        assert_eq!(u64::from(result.node_count), case["topology"]["nodeCount"].as_u64().unwrap());
        for (id, expected) in case["topology"]["depth"].as_object().unwrap() { assert_eq!(u64::from(result.depth[id]), expected.as_u64().unwrap()); }
        assert_eq!(result.depth.len(), case["topology"]["depth"].as_object().unwrap().len());
        let flat = infer_jack_flat_position(owner.identity(), case["rootNodeId"].as_str(), &owner).unwrap();
        assert_eq!(flat.positions.len(), case["positions"].as_object().unwrap().len());
        for (id, expected) in case["positions"].as_object().unwrap() { assert_eq!((flat.positions[id].u, flat.positions[id].v), (expected["u"].as_f64().unwrap(), expected["v"].as_f64().unwrap())); }
    }
    println!("[DEBUG] Native Jack graph query matched shared topology/depth/displacement witnesses");
}

#[test]
fn graph_query_checks_every_selected_coordinate_before_entity_read() {
    let input=fixture();
    let owner = graph(&input["cases"][1]);
    for coordinate in 0..5 {
        let mut identity = owner.identity.clone();
        identity[coordinate].push('!');
        let selected = ChildRefFields { child_id: &identity[0], artifact_id: &identity[1], artifact_kind: &identity[2], standard: &identity[3], subset: &identity[4] };
        let (refusal,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained_jack_graph(selected,&owner));
        assert!(refusal.is_err());
        assert_eq!((heap.requested_bytes as u64,heap.released_bytes as u64),(input["identityRefusal"]["requestedBytes"].as_u64().unwrap(),input["identityRefusal"]["releasedBytes"].as_u64().unwrap()));
        assert!(infer_jack_topology(selected, &owner).is_err());
        assert!(infer_jack_flat_position(selected, None, &owner).is_err());
        assert_eq!(owner.reads.get() as u64,input["identityRefusal"]["entityReads"].as_u64().unwrap());
    }
}

#[test]
fn graph_query_refuses_duplicate_ids_endpoints_and_unretained_root() {
    let mut owner = graph(&fixture()["cases"][1]);
    assert!(infer_jack_flat_position(owner.identity(), Some("absent"), &owner).is_err());
    owner.nodes.push(owner.nodes[0].clone());
    assert!(infer_jack_topology(owner.identity(), &owner).is_err());
    owner.nodes.pop();
    owner.edges.push(owner.edges[0].clone());
    assert!(infer_jack_topology(owner.identity(), &owner).is_err());
    owner.edges.pop();
    owner.edges[0].1 = "absent".into();
    assert!(infer_jack_topology(owner.identity(), &owner).is_err());
}

#[test]
fn rich_read_preserves_literal_components_and_presented_geometry() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/👁️rich-read/🔣️.json")).unwrap();
    for value in fixture["nodes"].as_array().unwrap(){
        let node=crate::JackContentNodeView{id:value["id"].as_str().unwrap(),kind:value["kind"].as_str().unwrap(),label:value["label"].as_str().unwrap(),x:value["x"].as_f64().unwrap(),y:value["y"].as_f64().unwrap(),width:value["width"].as_f64().unwrap(),height:value["height"].as_f64().unwrap(),port_count:value["ports"].as_array().unwrap().len()};
        assert_eq!(node.presented_label(),value["expectedLabel"].as_str().unwrap());
        assert_eq!(node.presented_extent(),(value["expectedWidth"].as_f64().unwrap(),value["expectedHeight"].as_f64().unwrap()));
        assert!(node.id.contains('@'));
        let port=&value["ports"][0];
        let view=crate::JackContentPortView{id:port["id"].as_str().unwrap(),label:port["label"].as_str().unwrap(),inbound:port["inbound"].as_bool().unwrap(),outbound:port["outbound"].as_bool().unwrap()};
        assert!(view.inbound&&view.outbound&&view.id.contains('@'));
    }
    println!("[DEBUG] Native rich Jack views retained literal endpoint components and both port directions");
}
