use super::*;
use crate::{DagHostSnapshotEdge, DagScene};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{apply_semio_graph_mutation, inverse_semio_graph_mutation};

fn demo() -> DagScene {
    crate::examples::demo::scene()
}

#[semio_framework_async_macros::async_test]
async fn split_endpoint_defaults_to_out_when_no_port_is_given() {
    assert_eq!(split_endpoint("n1"), ("n1".to_string(), "out".to_string()));
    assert_eq!(split_endpoint("n1@a"), ("n1".to_string(), "a".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn next_node_id_continues_after_the_highest_existing_suffix() {
    let mut scene = demo();
    scene.nodes.push(DagNodeSpec { id: "n99".into(), ..default_node_for_kind("note", "n99", 0.0, 0.0) });
    assert_eq!(next_node_id(&scene), "n100");
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
    let mut scene = DagScene { nodes: vec![default_node_for_kind("note", "a", 0.0, 0.0), default_node_for_kind("note", "b", 0.0, 0.0)], edges: Vec::new() };
    scene.edges.push(connect_edge(&scene, "a", "out", "b", "in").expect("a acyclic connection"));
    assert!(matches!(connect_edge(&scene, "b", "out", "a", "in"), Err(DagPlayError::CycleDetected)));
}

/// 🩹️ A slider field write is the ABSOLUTE `set-node-property` of that kind field (plus `resize-node` only when the widget
/// refits), and folding it onto the composed content yields the node with the new value.
#[semio_framework_async_macros::async_test]
async fn node_field_leaves_set_the_slider_value_on_the_child_content() {
    let node = default_node_for_kind("slider", "n1", 0.0, 0.0);
    let leaves = node_field_leaves(&node, "value", "5");
    assert!(matches!(leaves.first(), Some(SemioGraphMutation::SetNodeProperty(set)) if set.key == "value"), "{leaves:?}");
    let mut content = crate::dag_content_snapshot(&DagScene { nodes: vec![node], edges: Vec::new() });
    for leaf in &leaves {
        assert!(apply_semio_graph_mutation(&mut content, leaf).messages().is_empty(), "{leaf:?}");
    }
    assert!(matches!(crate::dag_scene_of_content(&content).nodes[0].kind, DagNodeKind::Slider { value, .. } if value == 5.0));
}

#[semio_framework_async_macros::async_test]
async fn node_field_leaves_are_empty_for_an_unknown_field_or_an_unchanged_value() {
    let note = default_node_for_kind("note", "n1", 0.0, 0.0);
    assert!(node_field_leaves(&note, "nonsense", "x").is_empty());
    let slider = default_node_for_kind("slider", "n2", 0.0, 0.0);
    assert!(node_field_leaves(&slider, "value", "3").is_empty(), "the default slider value is 3");
}

/// 🧺️ `remove_nodes_leaves` deletes every edge touching a targeted node and THEN the node, so every published row is
/// point-invertible; undoing them in reverse restores the composed content byte for byte.
#[semio_framework_async_macros::async_test]
async fn remove_nodes_leaves_delete_incident_edges_first_and_undo_exactly() {
    use store::ArtifactPack;
    let scene = demo();
    let node_id = scene.nodes.first().expect("fixture has a node").id.clone();
    let touching = scene.edges.iter().filter(|edge| split_endpoint(&edge.source).0 == node_id || split_endpoint(&edge.target).0 == node_id).count();
    assert!(touching > 0, "fixture must exercise the cascade case");
    let leaves = remove_nodes_leaves(&scene, std::slice::from_ref(&node_id));
    assert_eq!(leaves.len(), touching + 1);
    assert!(leaves[..touching].iter().all(|leaf| matches!(leaf, SemioGraphMutation::DeleteEdge(_))) && matches!(leaves.last(), Some(SemioGraphMutation::DeleteNode(_))));
    let base = crate::dag_content_snapshot(&scene);
    let mut content = base.clone();
    let mut undo = Vec::new();
    for leaf in &leaves {
        undo.push(inverse_semio_graph_mutation(leaf, &content).expect("inverse"));
        assert_eq!(undo.last().map(Vec::len), Some(1), "each row is point-invertible");
        apply_semio_graph_mutation(&mut content, leaf);
    }
    for step in undo.into_iter().rev().flatten() {
        apply_semio_graph_mutation(&mut content, &step);
    }
    assert_eq!(content.encode_pack(), base.encode_pack(), "undo restores the content bytes");
}

#[semio_framework_async_macros::async_test]
async fn remove_nodes_leaves_are_empty_for_an_unknown_node_id() {
    assert!(remove_nodes_leaves(&demo(), &["nonexistent".to_string()]).is_empty());
}

/// 🌉️ The graph codec round-trips every node and edge of the bundled demo exactly: native slots + typed properties.
#[semio_framework_async_macros::async_test]
async fn the_graph_codec_round_trips_the_demo_scene() {
    let scene = demo();
    assert_eq!(crate::dag_scene_of_content(&crate::dag_content_snapshot(&scene)), scene);
    let bare = DagHostSnapshotEdge { id: "e9".into(), source: "a".into(), target: "b@in".into(), ..Default::default() };
    assert_eq!(crate::dag_edge_of_graph(&crate::dag_graph_edge(&bare)), bare, "a bare endpoint keeps no port");
}
