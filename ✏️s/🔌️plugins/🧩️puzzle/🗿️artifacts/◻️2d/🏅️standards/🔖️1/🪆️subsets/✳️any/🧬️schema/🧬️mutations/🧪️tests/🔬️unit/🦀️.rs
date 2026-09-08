
use super::*;
use crate::PUZZLE_2D_SCHEMA;
use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

use serde_json::json;

#[test]
fn puzzle2d_delta_ops_are_granular_and_round_trip() {
    let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "anchor": "fixed", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "anchor": "fixed", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
    let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "anchor": "fixed", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
    let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle2dSnapshot>(value.clone()).expect("typed puzzle2d fixture")).expect("canonical puzzle2d JSON");
    let operations = puzzle2d_document_delta_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::MoveNode(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::DeleteNode(_))));
    // The typed bridge canonicalizes optional/default JSON fields while preserving the
    // artifact value and every operation's backwards restores the canonical pre-edit value.
    let mut forward = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(Mutation::<Value>::inverse(operation, &forward));
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
    let operations = puzzle2d_document_delta_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))), "sparse add must stay granular");
}

//#region 🔖️MutationLaws
#[test]
fn create_delete_node_inverse_law() {
    use crate::{Puzzle2dNode, schema::empty_puzzle2d_snapshot};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_node(node.clone(), None)));
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &delete_node("n1".into())));
}

#[test]
fn move_node_inverse_and_absorb_law() {
    use crate::{Puzzle2dNode, schema::empty_puzzle2d_snapshot};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &move_node("n1".into(), 5.0, 6.0)));
    let d1 = move_node("n1".into(), 10.0, 10.0).diff(&with_node).into_parts().0;
    let mid = MutationDiff::<Puzzle2dSnapshot>::apply(&d1, &with_node).expect("valid mutation diff");
    let d2 = move_node("n1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
    semio_framework::io::resolve_ready(assert_mutation_diff_absorb_law(&with_node, d1, d2));
}

#[test]
fn node_field_mutations_inverse_law() {
    use crate::{Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor, schema::empty_puzzle2d_snapshot};
    let base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n1".into(), handles: vec![Puzzle2dHandle { id: "h1".into(), ..Default::default() }], ..Default::default() };
    let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_geometry("n1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_kind("n1".into(), Some("core.capsule".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &edit_node_text("n1".into(), Some("hello".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_icon("n1".into(), Some("star".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &scale_node("n1".into(), Some(2.0))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_visible("n1".into(), Some(false))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_locked("n1".into(), Some(true))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_root("n1".into(), Some(true))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_anchor("n1".into(), Puzzle2dNodeAnchor::Derived)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &add_node_handle("n1".into(), Puzzle2dHandle { id: "h2".into(), ..Default::default() }, None)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &remove_node_handle("n1".into(), "h1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_handle("n1".into(), "h1".into(), Puzzle2dHandle { id: "h1".into(), angle: 1.5, ..Default::default() })));
}

#[test]
fn connect_disconnect_handles_inverse_law() {
    use crate::{Puzzle2dHandle, Puzzle2dNode, schema::empty_puzzle2d_snapshot};
    let base = empty_puzzle2d_snapshot();
    let node_a = Puzzle2dNode { id: "a".into(), handles: vec![Puzzle2dHandle { id: "ha".into(), ..Default::default() }], ..Default::default() };
    let node_b = Puzzle2dNode { id: "b".into(), handles: vec![Puzzle2dHandle { id: "hb".into(), ..Default::default() }], ..Default::default() };
    let mut projection = base.clone();
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None)));
    let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_handles("e1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &replace_edge_geometry("e1".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_kind("e1".into(), Some("core.link".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_tips("e1".into(), Some("arrow".into()), Some("dot".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_visible("e1".into(), Some(false))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_locked("e1".into(), Some(true))));
}

#[test]
fn delete_node_severs_and_reconnects_edges() {
    use crate::{Puzzle2dHandle, Puzzle2dNode, schema::empty_puzzle2d_snapshot};
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
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &removed));
}

#[test]
fn meta_mutations_inverse_law() {
    use crate::{Puzzle2dCompatSpecificity, Puzzle2dKindCatalogs, schema::empty_puzzle2d_snapshot};
    let base = empty_puzzle2d_snapshot();
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_manifest_id(Some("manifest-1".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle)));
    let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle2dKindCatalogs::default()))));
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds().len(), 26);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

#[test]
fn missing_target_is_error_per_verb_family() {
    use crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot;
    let base = empty_puzzle2d_snapshot();
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &delete_node("missing".into()))); // delete
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &remove_node_handle("missing".into(), "h0".into()))); // remove
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &change_node_visible("missing".into(), Some(false)))); // change/set/update
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &move_node("missing".into(), 1.0, 1.0))); // move/drag/rotate/scale/resize
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &edit_node_text("missing".into(), Some("x".into())))); // edit/replace
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &disconnect_handles("missing".into())));
    // disconnect/unbind
}

#[test]
fn create_duplicate_id_is_fatal_and_never_applies() {
    use crate::{Puzzle2dNode, schema::empty_puzzle2d_snapshot};
    let mut base = empty_puzzle2d_snapshot();
    let node = Puzzle2dNode { id: "n0".into(), ..Default::default() };
    base.nodes.push(node.clone());
    let outcome = create_node(node, None).diff(&base);
    semio_framework::io::resolve_ready(assert_fatal_never_applies(&outcome));
    assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️OutcomeLaws

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
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
