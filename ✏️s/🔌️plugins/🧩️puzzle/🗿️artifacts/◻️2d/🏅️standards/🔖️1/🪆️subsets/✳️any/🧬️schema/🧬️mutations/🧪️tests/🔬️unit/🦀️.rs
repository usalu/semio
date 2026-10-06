use super::*;
use crate::PUZZLE_2D_SCHEMA;
use crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot;
use protocol::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

use serde_json::json;

#[test]
fn puzzle2d_delta_ops_are_granular_and_round_trip() {
    let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "anchor": "fixed", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "anchor": "fixed", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
    let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "anchor": "fixed", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
    let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle2dSnapshot>(value.clone()).expect("typed puzzle2d snapshot")).expect("canonical puzzle2d JSON");
    let operations = puzzle2d_document_delta_operations(&before, &after).expect("both sides decode");
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::MoveNode(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::DeleteNode(_))));
    // The typed bridge canonicalizes optional/default JSON fields while preserving the
    // artifact value and every operation's backwards restores the canonical pre-edit value.
    let mut forward = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(Mutation::<Value>::inverse(operation, &forward).expect("valid retained mutation inverse snapshot"));
        forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&after));
    for inverse in inverses.iter().rev() {
        forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
}

#[test]
fn sparse_node_without_anchor_still_emits_create_node() {
    let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [], "edges": [] });
    let after = json!({
        "schema": PUZZLE_2D_SCHEMA,
        "nodes": [{ "id": "n1", "nodeKind": "seed", "shape": "circle", "x": 0.0, "y": 0.0, "text": "n1", "handles": [], "radius": 24.0 }],
        "edges": []
    });
    let operations = puzzle2d_document_delta_operations(&before, &after).expect("both sides decode");
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))), "sparse add must stay granular");
}

//#region 🔖️MutationLaws
#[test]
fn create_delete_node_inverse_law() {
    use crate::{Puzzle2dNode};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &create_node(node.clone(), None)));
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &delete_node("n1".into())));
}

#[test]
fn move_node_inverse_and_absorb_law() {
    use crate::{Puzzle2dNode};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &move_node("n1".into(), 5.0, 6.0)));
    let d1 = move_node("n1".into(), 10.0, 10.0).diff(&with_node).into_parts().0;
    let mid = MutationDiff::<Puzzle2dSnapshot>::apply(&d1, &with_node).expect("valid mutation diff");
    let d2 = move_node("n1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
    ::semio_framework_async::poll::resolve_ready(assert_mutation_diff_absorb_law(&with_node, d1, d2));
}

#[test]
fn node_field_mutations_inverse_law() {
    use crate::{Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), handles: vec![Puzzle2dHandle { id: "h1".into(), ..Default::default() }], ..Default::default() };
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_geometry("n1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_kind("n1".into(), Some("core.capsule".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &edit_node_text("n1".into(), Some("hello".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_icon("n1".into(), Some("star".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &scale_node("n1".into(), Some(2.0))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_visible("n1".into(), Some(false))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_locked("n1".into(), Some(true))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_root("n1".into(), Some(true))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_anchor("n1".into(), Puzzle2dNodeAnchor::Derived)));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &add_node_handle("n1".into(), Puzzle2dHandle { id: "h2".into(), ..Default::default() }, None)));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &remove_node_handle("n1".into(), "h1".into())));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_handle("n1".into(), "h1".into(), Puzzle2dHandle { id: "h1".into(), angle: 1.5, ..Default::default() })));
}

#[test]
fn connect_disconnect_handles_inverse_law() {
    use crate::{Puzzle2dHandle, Puzzle2dNode};
    let base = empty_puzzle2d_snapshot();
    let node_a = Puzzle2dNode { id: "a".into(), handles: vec![Puzzle2dHandle { id: "ha".into(), ..Default::default() }], ..Default::default() };
    let node_b = Puzzle2dNode { id: "b".into(), handles: vec![Puzzle2dHandle { id: "hb".into(), ..Default::default() }], ..Default::default() };
    let mut projection = base.clone();
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&projection, &connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None)));
    let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_handles("e1".into())));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &replace_edge_geometry("e1".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_kind("e1".into(), Some("core.link".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_tips("e1".into(), Some("arrow".into()), Some("dot".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_visible("e1".into(), Some(false))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_locked("e1".into(), Some(true))));
}

/// 🧲️ LAW (design §22.13): a connection that states the tolerance it was recorded under connects on ANY base. Within
/// the tolerance it reports nothing; once a handle moved away, or is on no node, it still adds its edge and warns
/// `mutation.precondition-drifted` naming both handles; a negative or non-finite tolerance is what the schema forbids.
#[test]
fn a_recorded_proximity_connect_warns_once_its_handles_drift_apart() {
    use crate::{Puzzle2dHandle, Puzzle2dNode};
    let node = |id: &str, x: f64, handle: &str, angle: f64| Puzzle2dNode { id: id.into(), x, radius: Some(24.0), handles: vec![Puzzle2dHandle { id: handle.into(), angle, ..Default::default() }], ..Default::default() };
    let mut base = empty_puzzle2d_snapshot();
    base.nodes = vec![node("a", 0.0, "ha", 0.0), node("b", 56.0, "hb", std::f64::consts::PI)];
    assert_eq!(puzzle2d_handle_position(&base, "ha"), Some((24.0, 0.0)), "a circle's handle sits on the rim at its east-zero angle");
    let apart = puzzle2d_handle_distance(&base, "ha", "hb").expect("both handles are on a node");
    assert!((apart - 8.0).abs() < 1e-9, "{apart}");
    assert_eq!(puzzle2d_handle_distance(&base, "ha", "ghost"), None);
    let recorded = connect_handles_in_proximity("e1".into(), "ha".into(), "hb".into(), 12.0);
    let near = recorded.diff(&base);
    assert!(near.messages().is_empty(), "within the tolerance nothing is reported: {:?}", near.messages());
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &recorded));
    let mut moved = base.clone();
    moved.nodes[1].x = 300.0;
    let mut gone = base.clone();
    gone.nodes.remove(1);
    for (state, what) in [(&moved, "moved away"), (&gone, "on no node")] {
        let outcome = recorded.diff(state);
        let reported: Vec<(semio_framework_diagnostic::Severity, &str, Vec<String>)> = outcome.messages().iter().map(|message| (message.level, message.code.0.as_str(), message.target.clone())).collect();
        assert_eq!(reported, vec![(semio_framework_diagnostic::Severity::Warning, "mutation.precondition-drifted", vec!["ha".to_string(), "hb".to_string()])], "{what}");
        assert!(outcome.messages()[0].message.contains("\"ha\"") && outcome.messages()[0].message.contains("\"hb\""), "{what}: the words name both handles: {}", outcome.messages()[0].message);
        let connected = MutationDiff::<Puzzle2dSnapshot>::apply(outcome.diff(), state).expect("a drifted connection still applies");
        assert!(connected.edges.iter().any(|edge| edge.id == "e1" && edge.source == "ha" && edge.target == "hb"), "{what}: the edge is there");
    }
    let unconditional = connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None);
    assert!(unconditional.diff(&moved).messages().is_empty(), "a connection that states no tolerance has no precondition");
    for forbidden in [-1.0, f64::NAN, f64::INFINITY] {
        let outcome = connect_handles_in_proximity("e1".into(), "ha".into(), "hb".into(), forbidden).diff(&base);
        ::semio_framework_async::poll::resolve_ready(assert_fatal_never_applies(&outcome));
        assert_eq!(outcome.messages()[0].code.0, "mutation.invariant", "tolerance {forbidden}");
    }
    let line = <Puzzle2dMutation as protocol::OpText>::print_op(&recorded);
    assert_eq!(<Puzzle2dMutation as protocol::OpText>::parse_op(&line).expect("the text form parses"), recorded, "{line}");
    let bytes = <Puzzle2dMutation as protocol::OpBinary>::encode_op(&recorded).expect("the binary form encodes");
    assert_eq!(<Puzzle2dMutation as protocol::OpBinary>::decode_op(&bytes).expect("the binary form decodes"), recorded);
    let value = semio_framework_value::ToValue::to_value(&unconditional);
    assert!(serde_json::Value::from(value).get("tolerance").is_none(), "a connection with no precondition states none");
}

#[test]
fn delete_node_severs_and_reconnects_edges() {
    use crate::{Puzzle2dHandle, Puzzle2dNode};
    let base = empty_puzzle2d_snapshot();
    let node_a = Puzzle2dNode { id: "a".into(), handles: vec![Puzzle2dHandle { id: "ha".into(), ..Default::default() }], ..Default::default() };
    let node_b = Puzzle2dNode { id: "b".into(), handles: vec![Puzzle2dHandle { id: "hb".into(), ..Default::default() }], ..Default::default() };
    let mut projection = base;
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    assert!(projection.edges.iter().any(|edge| edge.id == "e1"));
    let removed = delete_node("a".into());
    let after = MutationDiff::<Puzzle2dSnapshot>::apply(removed.diff(&projection).diff(), &projection).expect("valid mutation diff");
    assert!(!after.edges.iter().any(|edge| edge.id == "e1"), "delete-node must sever edges touching its handles");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&projection, &removed));
}

#[test]
fn meta_mutations_inverse_law() {
    use crate::{Puzzle2dCompatSpecificity, Puzzle2dKindCatalogs};
    let base = empty_puzzle2d_snapshot();
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &change_manifest_id(Some("manifest-1".into()))));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle)));
    let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle).diff(&base).diff(), &base).expect("valid mutation diff");
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle2dKindCatalogs::default()))));
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle2d_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds().len(), 36);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error};

#[test]
fn missing_target_is_error_per_verb_family() {
    use crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot;
    let base = empty_puzzle2d_snapshot();
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &delete_node("missing".into()))); // delete
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &remove_node_handle("missing".into(), "h0".into()))); // remove
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &change_node_visible("missing".into(), Some(false)))); // change/set/update
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &move_node("missing".into(), 1.0, 1.0))); // move/drag/rotate/scale/resize
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &edit_node_text("missing".into(), Some("x".into())))); // edit/replace
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &disconnect_handles("missing".into())));
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &drag_selection(vec!["missing".into()], 1.0, 1.0)));
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &rotate_selection(vec!["missing".into()], 0.0, 0.0, 1.0)));
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &scale_selection(vec!["missing".into()], 0.0, 0.0, 2.0)));
    // disconnect/unbind
}

#[test]
fn create_duplicate_id_is_fatal_and_never_applies() {
    use crate::{Puzzle2dNode};
    let mut base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n0".into(), ..Default::default() };
    base.nodes.push(node.clone());
    let outcome = create_node(node, None).diff(&base);
    ::semio_framework_async::poll::resolve_ready(assert_fatal_never_applies(&outcome));
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️OutcomeLaws

//#region 🔖️SelectionTransformLaws
/// 🧫️ A board with awkward (non-dyadic) coordinates: a node with a handle, a LOCKED node and a target
/// region — the negated-offset inverse these leaves deliberately do not use would drift on it.
fn selection_board() -> Puzzle2dSnapshot {
    use crate::{Puzzle2dHandle, Puzzle2dNode, Puzzle2dTargetRegion};
    let mut base = empty_puzzle2d_snapshot();
    base.nodes = vec![
        Puzzle2dNode { id: "a".into(), x: 0.1, y: 0.2, handles: vec![Puzzle2dHandle { id: "ha".into(), angle: 0.3, ..Default::default() }], ..Default::default() },
        Puzzle2dNode { id: "b".into(), x: 7.3, y: -2.9, locked: Some(true), ..Default::default() },
    ];
    base.target_regions = vec![Puzzle2dTargetRegion { id: "r".into(), x: 1.7, y: 2.3, width: 3.1, height: 4.9, ..Default::default() }];
    base
}

/// ↩️ Every selection transform inverts EXACTLY through its base-derived absolute setters.
#[test]
fn selection_transforms_invert_exactly_on_awkward_floats() {
    let base = selection_board();
    for mutation in [drag_selection(vec!["a".into(), "r".into()], 0.7, -1.3), rotate_selection(vec!["a".into()], 0.3, 0.9, 0.61), scale_selection(vec!["a".into(), "r".into()], 0.3, 0.9, 1.7)] {
        ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &mutation));
    }
}

/// 🚨️ Non-finite parameters, a non-positive factor and an empty or repeated target set are Fatal
/// `mutation.invariant` with no change — exactly what the payload schemas' hard bounds forbid.
#[test]
fn selection_transforms_refuse_non_finite_and_collapsing_parameters() {
    let base = selection_board();
    for mutation in [
        drag_selection(vec!["a".into()], f64::NAN, 0.0),
        rotate_selection(vec!["a".into()], 0.0, f64::INFINITY, 1.0),
        rotate_selection(vec!["a".into()], 0.0, 0.0, f64::NAN),
        scale_selection(vec!["a".into()], 0.0, 0.0, 0.0),
        scale_selection(vec!["a".into()], 0.0, 0.0, -2.0),
        drag_selection(Vec::new(), 1.0, 1.0),
        rotate_selection(vec!["a".into(), "r".into(), "a".into()], 0.0, 0.0, 1.0),
        scale_selection(vec!["ghost".into(), "ghost".into()], 0.0, 0.0, 2.0),
    ] {
        let outcome = mutation.diff(&base);
        ::semio_framework_async::poll::resolve_ready(assert_fatal_never_applies(&outcome));
        assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal), "{mutation:?} must be Fatal");
        assert_eq!(outcome.messages()[0].code.0, "mutation.invariant", "{mutation:?} breaks the verb family's finite/positive invariant");
    }
}

/// ⚠️ Missing and locked members degrade to `mutation.partial` (one message per reason, ids in payload
/// order) while every survivor still moves.
#[test]
fn selection_transforms_skip_missing_and_locked_members_as_partial() {
    let base = selection_board();
    let outcome = drag_selection(vec!["b".into(), "ghost".into(), "a".into()], 1.0, 2.0).diff(&base);
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Warning));
    let reported: Vec<(&str, Vec<String>)> = outcome.messages().iter().map(|message| (message.code.0.as_str(), message.target.clone())).collect();
    assert_eq!(reported, vec![("mutation.partial", vec!["ghost".to_string()]), ("mutation.partial", vec!["b".to_string()])]);
    let moved = MutationDiff::<Puzzle2dSnapshot>::apply(outcome.diff(), &base).expect("partial drag applies");
    assert_eq!((moved.nodes[0].x, moved.nodes[0].y), (0.1 + 1.0, 0.2 + 2.0), "node a moves by the offset");
    assert_eq!((moved.nodes[1].x, moved.nodes[1].y), (7.3, -2.9), "the locked node stays");
}

/// 🔄️ A rotation skips a target region (axis-aligned by construction) as partial, and a rotation of
/// regions alone has nothing left: `mutation.target-missing`.
#[test]
fn rotating_target_regions_is_partial_and_regions_alone_are_target_missing() {
    let base = selection_board();
    let mixed = rotate_selection(vec!["a".into(), "r".into()], 0.0, 0.0, 1.0).diff(&base);
    assert_eq!(mixed.messages().len(), 1);
    assert_eq!((mixed.messages()[0].code.0.as_str(), mixed.messages()[0].target.clone()), ("mutation.partial", vec!["r".to_string()]));
    assert!(mixed.diff().target_regions.is_none(), "a rotation never patches a target region");
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &rotate_selection(vec!["r".into()], 0.0, 0.0, 1.0)));
    ::semio_framework_async::poll::resolve_ready(assert_missing_target_is_error(&base, &rotate_selection(vec!["b".into()], 0.0, 0.0, 1.0)));
}

/// ⏸️ The identity parameters (zero offset, zero angle, unit factor) are warning-level no-ops.
#[test]
fn identity_selection_transforms_are_no_ops() {
    let base = selection_board();
    for mutation in [drag_selection(vec!["a".into(), "r".into()], 0.0, 0.0), rotate_selection(vec!["a".into()], 0.3, 0.9, 0.0), scale_selection(vec!["a".into(), "r".into()], 0.3, 0.9, 1.0)] {
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.diff(), &Puzzle2dDiff::default(), "{mutation:?}");
        assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec!["mutation.no-op"], "{mutation:?}");
        assert!(inverse_puzzle2d_mutation(&base, &mutation).expect("valid retained mutation inverse snapshot").is_empty(), "{mutation:?}: nothing moved, nothing to undo");
    }
}

/// 🔁️ The diff reads BASE positions: replayed on a base where the node already moved, the drag
/// re-derives from there — the property that makes an edited upstream drag meaningful downstream.
#[test]
fn selection_diff_replays_on_a_moved_base() {
    let base = selection_board();
    let moved_base = MutationDiff::<Puzzle2dSnapshot>::apply(move_node("a".into(), 10.0, 20.0).diff(&base).diff(), &base).expect("move applies");
    let replayed = MutationDiff::<Puzzle2dSnapshot>::apply(drag_selection(vec!["a".into()], 1.0, -1.0).diff(&moved_base).diff(), &moved_base).expect("drag applies");
    assert_eq!((replayed.nodes[0].x, replayed.nodes[0].y), (11.0, 19.0));
}

/// 🗣️ Labels name the count and the parameters in every locale.
#[test]
fn selection_labels_name_count_and_parameters() {
    let label = |mutation: Puzzle2dMutation| serde_json::to_string(&<Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(&mutation)).expect("label serializes");
    let drag = label(drag_selection(vec!["a".into(), "r".into()], 5.0, -2.5));
    assert!(drag.contains("Drag 2 items by (5, -2.5)") && drag.contains("2 Elemente um (5; -2,5) ziehen"), "{drag}");
    let rotate = label(rotate_selection(vec!["a".into()], 0.0, 0.0, std::f64::consts::FRAC_PI_2));
    assert!(rotate.contains("Rotate 1 item by 90°") && rotate.contains("1 Element um 90° drehen"), "{rotate}");
    let scale = label(scale_selection(vec!["a".into()], 0.0, 0.0, 0.5));
    assert!(scale.contains("Scale 1 item by a factor of 0.5") && scale.contains("1 Element um den Faktor 0,5 skalieren"), "{scale}");
}
/// 🧱️ Schema-first: every value a leaf payload schema forbids through a hard bound is a Fatal
/// `mutation.invariant` in that leaf's diff, with the default diff, before the base is consulted — on a
/// board that does hold every addressed record, so the refusal cannot be a missing target in disguise.
#[test]
fn every_bounded_leaf_refuses_what_its_schema_forbids() {
    use crate::{Puzzle2dCatalogHandleKind, Puzzle2dCatalogNodeKind, Puzzle2dHandle, Puzzle2dHandleTemplate, Puzzle2dKindCatalogs, Puzzle2dNode, Puzzle2dTargetRegion};
    let mut base = selection_board();
    base.edges = vec![crate::Puzzle2dEdge { id: "e".into(), source: "ha".into(), target: "ha".into(), ..Default::default() }];
    let handle = |angle: f64, radius: Option<f64>, scale: Option<f64>| Puzzle2dHandle { id: "hn".into(), angle, radius, scale, ..Default::default() };
    let node = |edit: fn(&mut Puzzle2dNode)| {
        let mut node = Puzzle2dNode { id: "n".into(), ..Default::default() };
        edit(&mut node);
        node
    };
    let region = |x: f64, width: f64| Puzzle2dTargetRegion { id: "rn".into(), x, width, ..Default::default() };
    let template = |t: Option<f64>, radius: Option<f64>, angle: f64| Puzzle2dKindCatalogs {
        nodes: vec![Puzzle2dCatalogNodeKind { id: "k".into(), handles: vec![Puzzle2dHandleTemplate { id: "tpl".into(), angle, t, radius, ..Default::default() }], ..Default::default() }],
        ..Default::default()
    };
    let forbidden = [
        move_node("a".into(), f64::NAN, 0.0),
        move_node("a".into(), 0.0, f64::INFINITY),
        move_target_region("r".into(), f64::NEG_INFINITY, 0.0),
        resize_target_region("r".into(), f64::NAN, 1.0),
        replace_node_geometry("a".into(), Some("hexagon".into()), None, None, None),
        replace_node_geometry("a".into(), Some("circle".into()), Some(-4.0), None, None),
        replace_node_geometry("a".into(), Some("rectangle".into()), None, Some(0.0), Some(2.0)),
        replace_node_geometry("a".into(), None, None, None, Some(f64::NAN)),
        scale_node("a".into(), Some(0.0)),
        scale_node("a".into(), Some(-1.5)),
        scale_node("a".into(), Some(f64::INFINITY)),
        create_node(node(|node| node.x = f64::NAN), None),
        create_node(node(|node| node.shape = Some("triangle".into())), None),
        create_node(node(|node| node.radius = Some(0.0)), None),
        create_node(node(|node| node.width = Some(-1.0)), None),
        create_node(node(|node| node.scale = Some(0.0)), None),
        create_node(node(|node| node.handles = vec![Puzzle2dHandle { id: "hx".into(), angle: f64::NAN, ..Default::default() }]), None),
        add_node_handle("a".into(), handle(f64::INFINITY, None, None), None),
        add_node_handle("a".into(), handle(0.0, Some(0.0), None), None),
        replace_node_handle("a".into(), "ha".into(), handle(0.0, None, Some(-1.0))),
        connect_handles("e2".into(), "ha".into(), "ha".into(), None, f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None),
        connect_handles("e2".into(), "ha".into(), "ha".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, f64::INFINITY, None, None),
        replace_edge_geometry("e".into(), 0.0, 0.0, 0.0, f64::NAN, 0.0, 0.0, 0.0, 0.0),
        create_target_region(region(f64::NAN, 1.0), None),
        create_target_region(region(0.0, f64::INFINITY), None),
        replace_kind_catalogs(Some(template(Some(1.5), None, 0.0))),
        replace_kind_catalogs(Some(template(Some(-0.1), None, 0.0))),
        replace_kind_catalogs(Some(template(None, Some(0.0), 0.0))),
        replace_kind_catalogs(Some(template(None, None, f64::NAN))),
        replace_kind_catalogs(Some(Puzzle2dKindCatalogs { handles: vec![Puzzle2dCatalogHandleKind { id: "hk".into(), order: Some(-1), ..Default::default() }], ..Default::default() })),
    ];
    for mutation in forbidden {
        let outcome = mutation.diff(&base);
        ::semio_framework_async::poll::resolve_ready(assert_fatal_never_applies(&outcome));
        let codes: Vec<(semio_framework_diagnostic::Severity, &str)> = outcome.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect();
        assert_eq!(codes, vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant")], "{mutation:?} must be refused as the schema forbids it");
    }
    let admitted = [
        resize_target_region("r".into(), -3.0, 0.0),
        replace_node_geometry("a".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0)),
        scale_node("a".into(), None),
        replace_kind_catalogs(Some(template(Some(1.0), Some(2.0), 0.5))),
    ];
    for mutation in admitted {
        assert!(!mutation.diff(&base).messages().iter().any(|message| message.code.0 == "mutation.invariant"), "{mutation:?} is inside every hard bound its schema declares");
    }
}
//#endregion 🔖️SelectionTransformLaws

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle2dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog


#[test]
fn play_snapshot_pack_shares_the_typed_record_identity_and_round_trips() {
    let spec = <Puzzle2dPlaySnapshot as store::ArtifactPack>::record_spec().expect("play snapshot declares its record spec");
    assert_eq!(store::os_pack::schema_hash(&spec), store::os_pack::schema_hash(&Puzzle2dSnapshot::__dsl_spec()));
    assert_ne!(store::os_pack::schema_hash(&spec), [0u8; 32]);
    let play = Puzzle2dPlaySnapshot::from_typed(Puzzle2dSnapshot::default());
    let bytes = store::ArtifactPack::encode_pack(&play);
    assert_eq!(bytes, store::ArtifactPack::encode_pack(play.typed()));
    assert_eq!(<Puzzle2dPlaySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode play pack"), play);
}
