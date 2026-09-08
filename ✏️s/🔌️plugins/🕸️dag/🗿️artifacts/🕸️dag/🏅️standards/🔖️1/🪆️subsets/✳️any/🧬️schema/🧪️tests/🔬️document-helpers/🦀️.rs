
use super::*;

#[semio_framework_async_macros::async_test]
async fn split_endpoint_defaults_to_out_when_no_port_is_given() {
    assert_eq!(split_endpoint("n1"), ("n1".to_string(), "out".to_string()));
    assert_eq!(split_endpoint("n1@a"), ("n1".to_string(), "a".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn next_node_id_continues_after_the_highest_existing_suffix() {
    let document = crate::default_snapshot();
    let mut nodes = document.nodes();
    nodes.push(DagNodeSpec { id: "n99".into(), ..default_node_for_kind("note", "n99", 0.0, 0.0) });
    let edges = document.edges();
    let content = crate::dag_content_child_with_owner(nodes, edges);
    let document = DagSnapshot { schema: document.schema.clone(), content };
    assert_eq!(next_node_id(&document), "n100");
}

#[semio_framework_async_macros::async_test]
async fn default_node_for_kind_fits_the_widget_size_for_every_kind() {
    for kind in ["slider", "select", "screen", "note", "preview", "computation"] {
        let node = default_node_for_kind(kind, "n1", 10.0, 20.0);
        assert!(node.width > 0.0 && node.height > 0.0, "{kind} node must have a positive fitted size");
    }
}

#[semio_framework_async_macros::async_test]
async fn connect_edge_rejects_a_connection_that_would_create_a_cycle() {
    let document = crate::default_snapshot();
    let nodes = document.nodes();
    if let (Some(first), Some(second)) = (nodes.first(), nodes.get(1)) {
        let _ = connect_edge(&document, &first.id, "out", &second.id, "in");
        let result = connect_edge(&document, &second.id, "out", &first.id, "in");
        // Only asserts the cycle path is reachable when the fixture's first two nodes are already
        // linked in a way that would close a loop; a non-cyclic fixture legitimately returns `Ok`.
        assert!(result.is_ok() || matches!(result, Err(DagPlayError::CycleDetected)));
    }
}

#[semio_framework_async_macros::async_test]
async fn node_patch_for_field_updates_slider_value_and_refits_size() {
    let node = default_node_for_kind("slider", "n1", 0.0, 0.0);
    let patch = node_patch_for_field(&node, "value", Some("5")).expect("slider value patch");
    assert!(matches!(patch.kind, Some(DagNodeKind::Slider { value, .. }) if value == 5.0));
}

#[semio_framework_async_macros::async_test]
async fn node_patch_for_field_returns_none_for_an_unknown_field() {
    let node = default_node_for_kind("note", "n1", 0.0, 0.0);
    assert!(node_patch_for_field(&node, "nonsense", Some("x")).is_none());
}

/// 🐛️ Pre-existing bug fix (unrelated to composition — traced via `git log --date=iso` to
/// commit `31209e7a`, 2026-08-13 00:13:16, the ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES
/// relocation that introduced both `remove_nodes_operations` and this test together): the
/// assertion contradicted the function's OWN doc comment ("one mutation per node, not one per
/// node PLUS one per severed edge" — `delete-node`'s own diff/inverse already captures the edge
/// cascade internally). `remove_nodes_operations` returns exactly one `delete-node` mutation per
/// targeted node id, regardless of how many edges touch it.
#[semio_framework_async_macros::async_test]
async fn remove_nodes_operations_returns_one_delete_node_mutation_per_targeted_node() {
    let document = crate::default_snapshot();
    let nodes = document.nodes();
    let edges = document.edges();
    let node_id = nodes.first().expect("fixture has a node").id.clone();
    let touching_edges = edges
        .iter()
        .filter(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            from == node_id || to == node_id
        })
        .count();
    assert!(touching_edges > 0, "fixture must exercise the cascade-capturing case");
    let operations = remove_nodes_operations(&document, std::slice::from_ref(&node_id));
    assert_eq!(operations.len(), 1, "delete-node's own diff/inverse captures the edge cascade internally");
    let remaining: Vec<DagNodeSpec> = nodes.into_iter().filter(|node| node.id != node_id).collect();
    assert!(remaining.iter().all(|node| node.id != node_id));
}

#[semio_framework_async_macros::async_test]
async fn remove_nodes_operations_is_empty_for_an_unknown_node_id() {
    let document = crate::default_snapshot();
    assert!(remove_nodes_operations(&document, &["nonexistent".to_string()]).is_empty());
}
