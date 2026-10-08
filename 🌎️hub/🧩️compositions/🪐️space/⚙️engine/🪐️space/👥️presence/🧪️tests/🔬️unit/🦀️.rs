use super::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use protocol::{DiffAlgebra, Mutation};

fn camera(x: f64) -> SpaceWindowCamera {
    SpaceWindowCamera { x, y: -x, zoom: 2.0 }
}

fn busy() -> SpacePresence {
    SpacePresence {
        camera: BTreeMap::from([("left".to_string(), camera(1.0)), ("right".to_string(), camera(2.0))]),
        active_node_id: Some("a".into()),
        focused_node_id: Some("b".into()),
        collapsed_node_ids: vec!["c".into(), "d".into()],
        preview_off_node_ids: vec!["e".into()],
    }
}

#[semio_framework_async_macros::async_test]
async fn every_operation_names_only_the_slots_it_owns() {
    let base = busy();
    let diff = SpacePresenceMutation::SetActiveNode { node_id: Some("z".into()) }.diff(&base);
    assert_eq!(diff.diff(), &SpacePresenceDiff { active_node_id: Some(Some("z".into())), ..Default::default() });
    assert!(SpacePresenceMutation::SetActiveNode { node_id: base.active_node_id.clone() }.diff(&base).diff().is_empty());
    let moved = SpacePresenceMutation::SetCamera { window_id: "left".into(), camera: camera(9.0) }.diff(&base);
    assert_eq!(moved.diff().camera, [("left".to_string(), protocol::KeyedRow::Replace(camera(9.0)))].into());
    let added = SpacePresenceMutation::SetCamera { window_id: "middle".into(), camera: camera(9.0) }.diff(&base);
    assert_eq!(added.diff().camera, [("middle".to_string(), protocol::KeyedRow::Insert(camera(9.0)))].into());
}

#[semio_framework_async_macros::async_test]
async fn removing_an_absent_camera_is_a_target_missing_refusal() {
    let outcome = SpacePresenceMutation::RemoveCamera { window_id: "absent".into() }.diff(&busy());
    assert!(outcome.diff().is_empty());
    assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec!["mutation.target-missing"]);
}

#[semio_framework_async_macros::async_test]
async fn every_operation_obeys_the_inverse_sum_law_including_a_middle_camera_row() {
    for base in [busy(), SpacePresence::default()] {
        for mutation in [
            SpacePresenceMutation::SetActiveNode { node_id: None },
            SpacePresenceMutation::SetFocusedNode { node_id: Some("z".into()) },
            SpacePresenceMutation::SetCollapsed { node_ids: vec!["x".into()] },
            SpacePresenceMutation::SetPreviewOff { node_ids: Vec::new() },
            SpacePresenceMutation::SetCamera { window_id: "left".into(), camera: camera(7.0) },
            SpacePresenceMutation::SetCamera { window_id: "middle".into(), camera: camera(7.0) },
        ] {
            assert_mutation_inverse_sum_law(&mutation, &base).await;
        }
    }
    assert_mutation_inverse_sum_law(&SpacePresenceMutation::RemoveCamera { window_id: "left".into() }, &busy()).await;
}

#[semio_framework_async_macros::async_test]
async fn presence_and_operations_round_trip_text_and_binary() {
    store::os_store::test_support::assert_dsl_round_trip(&busy());
    store::os_store::test_support::assert_dsl_pack_equivalence(&busy());
    store::os_store::test_support::assert_op_line_round_trip(&SpacePresenceMutation::SetActiveNode { node_id: Some("a".into()) });
    store::os_store::test_support::assert_op_line_round_trip(&SpacePresenceMutation::SetCollapsed { node_ids: vec!["a".into()] });
    store::os_store::test_support::assert_op_line_round_trip(&SpacePresenceMutation::SetCamera { window_id: "left".into(), camera: camera(3.0) });
    store::os_store::test_support::assert_op_line_round_trip(&SpacePresenceMutation::RemoveCamera { window_id: "left".into() });
}
