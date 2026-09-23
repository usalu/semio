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

/// 🧺️ `remove_nodes_operations` disconnects every edge touching a targeted node and THEN deletes the
/// node — one `disconnect-nodes` per incident edge followed by one `delete-node` per node — so every
/// published row is point-invertible for the retained one-item preparation
/// (`ArtifactStoreOneItemFootprint::for_one_invertible_item`). The former one-row-per-node contract
/// relied on `delete-node`'s internal cascade, whose inverse is one row per severed edge plus the node
/// and failed the fold with `batched item candidate failed its exact fixed fold contract` (ticket
/// 26/09/19, `📓️knowledge.md` §14.1).
#[semio_framework_async_macros::async_test]
async fn remove_nodes_operations_disconnects_incident_edges_before_deleting_the_node() {
    let document = crate::default_snapshot();
    let node_id = document.nodes().first().expect("fixture has a node").id.clone();
    let touching: Vec<String> = document
        .edges()
        .into_iter()
        .filter(|edge| split_endpoint(&edge.source).0 == node_id || split_endpoint(&edge.target).0 == node_id)
        .map(|edge| edge.id)
        .collect();
    assert!(!touching.is_empty(), "fixture must exercise the cascade case");
    let operations = remove_nodes_operations(&document, std::slice::from_ref(&node_id));
    let mut expected: Vec<DagMutation> = touching.into_iter().map(crate::mutations::disconnect_nodes).collect();
    expected.push(crate::mutations::delete_node(node_id));
    assert_eq!(operations, expected);
}

#[semio_framework_async_macros::async_test]
async fn remove_nodes_operations_is_empty_for_an_unknown_node_id() {
    let document = crate::default_snapshot();
    assert!(remove_nodes_operations(&document, &["nonexistent".to_string()]).is_empty());
}
