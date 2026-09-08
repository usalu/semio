
use super::*;
use crate::Step;
use std::collections::BTreeMap as StdBTreeMap;

fn step(id: &str, bodies: StdBTreeMap<String, Path>) -> Step {
    Step { id: id.into(), kind: "noop".into(), params: Default::default(), bodies }
}

#[semio_framework_async_macros::async_test]
async fn a_flat_path_orders_steps_at_depth_zero() {
    let path = Path { steps: vec![step("a", StdBTreeMap::new()), step("b", StdBTreeMap::new())] };
    let topology = compute_procedure_topology(&path);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&0));
    assert_eq!(topology.node_count, 2);
    assert!(topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn a_nested_body_step_sits_one_depth_deeper_and_still_counts() {
    let inner = Path { steps: vec![step("inner", StdBTreeMap::new())] };
    let mut bodies = StdBTreeMap::new();
    bodies.insert("then".to_string(), inner);
    let path = Path { steps: vec![step("outer", bodies)] };
    let topology = compute_procedure_topology(&path);
    assert_eq!(topology.topo_order, vec!["outer".to_string(), "inner".to_string()]);
    assert_eq!(topology.depth.get("outer"), Some(&0));
    assert_eq!(topology.depth.get("inner"), Some(&1));
    assert_eq!(topology.node_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn an_empty_path_is_the_zero_topology() {
    assert_eq!(compute_procedure_topology(&Path::new()), ProcedureTopology::default());
}
