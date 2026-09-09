use super::*;

fn sample_node(id: &str) -> DagNodeSpec {
    crate::schema::default_node_for_kind("note", id, 0.0, 0.0)
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_create_node() {
    store::os_store::test_support::assert_op_line_round_trip(&create_node(sample_node("node-1")));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_move_node() {
    store::os_store::test_support::assert_op_line_round_trip(&move_node("node-1".into(), 5.0, 6.0));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_node_operator_kind_none() {
    store::os_store::test_support::assert_op_line_round_trip(&change_node_operator_kind("node-1".into(), None));
}

/// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<DagMutation> {
    vec![
        create_node(sample_node("node-1")),
        delete_node("node-1".into()),
        rename_node("node-1".into(), "node-2".into()),
        change_node_name("node-1".into(), "Renamed".into()),
        move_node("node-1".into(), 5.0, 6.0),
        resize_node("node-1".into(), 200.0, 90.0),
        change_node_icon("node-1".into(), "emoji:🎚️".into()),
        change_node_abbreviation("node-1".into(), "Nd".into()),
        change_node_operator_kind("node-1".into(), Some("math.add".into())),
        change_node_operator_kind("node-1".into(), None),
        replace_node_kind("node-1".into(), sample_node("node-1").kind),
        replace_node_properties("node-1".into(), PropertyBag::default()),
        reorder_nodes(vec!["node-2".into(), "node-1".into()]),
        connect_nodes("edge-1".into(), "node-1@out".into(), "node-2@in".into(), EdgeRouteStyle::default(), PropertyBag::default()),
        disconnect_nodes("edge-1".into()),
    ]
}
