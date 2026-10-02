use super::*;
use crate::default_snapshot;
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_deterministic};
use protocol::Mutation;
use protocol::SemanticMutation;
use store::apply_mutation;

/// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
/// — must agree, in spelling AND in order. The framework never parses Rust, so without this
/// test `KINDS` could drift from the enum and the catalog could keep measuring `🌳️mutate-dag-1`
/// against a vocabulary the artifact no longer has.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = DagMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
    assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"no-mutation\""), "whole-document replace is banned vocabulary here — the catalog must not smuggle it back in");
}

fn round_trip(snapshot: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
    let (forward, _messages) = apply_mutation(snapshot, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    let mut backward = mutation.inverse(snapshot);
    backward.reverse();
    for back in backward {
        let (next, _messages) = apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot");
    forward
}

fn sample_node(id: &str, x: f64, y: f64) -> crate::DagNodeSpec {
    crate::schema::default_node_for_kind("note", id, x, y)
}

#[semio_framework_async_macros::async_test]
async fn create_move_resize_delete_node_round_trip() {
    let snapshot = default_snapshot();
    let node = sample_node("node-99", 5.0, 6.0);
    let added = round_trip(&snapshot, &create_node(node));
    assert!(added.nodes().iter().any(|node| node.id == "node-99"));
    let moved = round_trip(&added, &move_node("node-99".into(), 120.0, 6.0));
    assert_eq!(moved.nodes().iter().find(|node| node.id == "node-99").unwrap().x, 120.0);
    let resized = round_trip(&moved, &resize_node("node-99".into(), 200.0, 80.0));
    assert_eq!(resized.nodes().iter().find(|node| node.id == "node-99").unwrap().width, 200.0);
    let removed = round_trip(&resized, &delete_node("node-99".into()));
    assert!(!removed.nodes().iter().any(|node| node.id == "node-99"));
}

#[semio_framework_async_macros::async_test]
async fn rename_node_cascades_edge_endpoints() {
    let snapshot = default_snapshot();
    let Some(id) = snapshot.nodes().first().map(|node| node.id.clone()) else { return };
    let renamed = round_trip(&snapshot, &rename_node(id.clone(), "renamed-node".into()));
    assert!(renamed.nodes().iter().any(|node| node.id == "renamed-node"));
    assert!(renamed.edges().iter().all(|edge| !edge.source.starts_with(&format!("{id}@")) && !edge.target.starts_with(&format!("{id}@"))));
}

#[semio_framework_async_macros::async_test]
async fn delete_node_severs_and_reconnects_edges() {
    let snapshot = default_snapshot();
    let Some(id) = snapshot.nodes().first().map(|node| node.id.clone()) else { return };
    round_trip(&snapshot, &delete_node(id));
}

#[semio_framework_async_macros::async_test]
async fn reorder_nodes_round_trips() {
    let snapshot = default_snapshot();
    let nodes = snapshot.nodes();
    if nodes.len() < 2 {
        return;
    }
    let mut order: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    order.reverse();
    round_trip(&snapshot, &reorder_nodes(order));
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn create_node_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &create_node(sample_node("node-99", 5.0, 6.0))).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_node_inverse_law() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    assert_mutation_inverse_law(&base, &delete_node(id)).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_node_inverse_law() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    assert_mutation_inverse_law(&base, &rename_node(id, "renamed-node".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn move_node_inverse_law() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    assert_mutation_inverse_law(&base, &move_node(id, 42.0, -8.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn resize_node_inverse_law() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    assert_mutation_inverse_law(&base, &resize_node(id, 200.0, 90.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_disconnect_nodes_inverse_law() {
    let base = default_snapshot();
    let nodes = base.nodes();
    if nodes.len() < 2 {
        return;
    }
    let source = nodes[0].id.clone();
    let target = nodes[1].id.clone();
    assert_mutation_inverse_law(&base, &connect_nodes("edge-99".into(), format!("{source}@out"), format!("{target}@in"), semio_framework_artifact_infinite_dag::EdgeRouteStyle::default(), Default::default())).await;
    if let Some(edge_id) = base.edges().first().map(|edge| edge.id.clone()) {
        assert_mutation_inverse_law(&base, &disconnect_nodes(edge_id)).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn reorder_nodes_inverse_law() {
    let base = default_snapshot();
    let nodes = base.nodes();
    if nodes.len() < 2 {
        return;
    }
    let mut order: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    order.reverse();
    assert_mutation_inverse_law(&base, &reorder_nodes(order)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_node_diff_absorb_law() {
    use protocol::Mutation;
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let d1 = move_node(id.clone(), 10.0, 10.0).diff(&base).diff().clone();
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = move_node(id, 20.0, 30.0).diff(&mid).diff().clone();
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_dag_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in DagMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(DagMutation::kinds().len(), 17);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one `assert_missing_target_is_error`/Fatal/determinism check per verb family this facet
/// implements (create/delete/rename/move/resize/change/replace/reorder/connect/disconnect).
#[semio_framework_async_macros::async_test]
async fn create_node_duplicate_id_is_fatal() {
    let base = default_snapshot();
    let Some(existing_id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let outcome = create_node(sample_node(&existing_id, 0.0, 0.0)).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_node_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &delete_node("ghost-node".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_node_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &rename_node("ghost-node".into(), "x".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn move_node_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &move_node("ghost-node".into(), 1.0, 1.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_node_non_finite_is_fatal() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let outcome = move_node(id, f64::NAN, 0.0).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn resize_node_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &resize_node("ghost-node".into(), 10.0, 10.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn change_node_name_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &change_node_name("ghost-node".into(), "x".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_node_kind_missing_target_is_error() {
    let base = default_snapshot();
    let Some(kind) = base.nodes().first().map(|node| node.kind.clone()) else { return };
    assert_missing_target_is_error(&base, &replace_node_kind("ghost-node".into(), kind)).await;
}

#[semio_framework_async_macros::async_test]
async fn disconnect_nodes_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &disconnect_nodes("ghost-edge".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_nodes_missing_endpoint_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &connect_nodes("edge-99".into(), "ghost-source@out".into(), "ghost-target@in".into(), semio_framework_artifact_infinite_dag::EdgeRouteStyle::default(), Default::default())).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_nodes_self_loop_is_fatal() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let outcome = connect_nodes("edge-99".into(), format!("{id}@out"), format!("{id}@in"), semio_framework_artifact_infinite_dag::EdgeRouteStyle::default(), Default::default()).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn reorder_nodes_duplicate_id_is_fatal() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let outcome = reorder_nodes(vec![id.clone(), id]).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn move_node_diff_is_deterministic() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    assert_outcome_deterministic(&base, &move_node(id, 7.0, 8.0)).await;
}
//#endregion 🔖️OutcomeLaws

/// 🧺️ A retained one-item preparation folds exactly one forward plus one inverse row
/// (`ArtifactStoreOneItemFootprint::for_one_invertible_item`), so every row the editor's removal verbs
/// publish must invert to ONE row: `delete-node` on a connected node inverted to the node plus one row
/// per severed edge and every `removeNode`/`deleteSelection` on it failed `batched item candidate failed
/// its exact fixed fold contract`. The rows' own inverses, undone tail-first, restore the document
/// byte-for-byte (content-child identity is a hash over node and edge ORDER).
#[semio_framework_async_macros::async_test]
async fn removal_rows_are_point_invertible_and_restore_the_document() {
    let base = default_snapshot();
    assert!(!base.edges().is_empty(), "the law needs a connected node");
    for node in base.nodes() {
        let rows = crate::schema::remove_nodes_operations(&base, std::slice::from_ref(&node.id));
        let mut current = base.clone();
        let mut undo = Vec::new();
        for row in &rows {
            let inverse = row.inverse(&current);
            assert_eq!(inverse.len(), 1, "removal row {row:?} must invert to exactly one row, got {inverse:?}");
            current = apply_mutation(&current, row).expect("valid removal row").0;
            undo.extend(inverse);
        }
        assert!(!current.nodes().iter().any(|entry| entry.id == node.id));
        for row in undo.iter().rev() {
            current = apply_mutation(&current, row).expect("valid inverse row").0;
        }
        assert_eq!(current, base, "undoing every removal row of {} restores the document byte-for-byte", node.id);
    }
}

/// 📍️ `create-node`/`connect-nodes` insert at their optional `index`; absent, they append.
#[semio_framework_async_macros::async_test]
async fn positional_create_and_connect_insert_where_they_are_told() {
    let base = default_snapshot();
    let created = apply_mutation(&base, &create_node_at(sample_node("node-first", 0.0, 0.0), 0)).expect("positional create").0;
    assert_eq!(created.nodes().first().map(|node| node.id.clone()), Some("node-first".into()));
    let Some(edge) = base.edges().last().cloned() else { return };
    let disconnected = apply_mutation(&base, &disconnect_nodes(edge.id.clone())).expect("disconnect").0;
    let reconnected = apply_mutation(&disconnected, &connect_nodes_at(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone(), 0)).expect("positional connect").0;
    assert_eq!(reconnected.edges().first().map(|entry| entry.id.clone()), Some(edge.id));
}

//#region 🔖️GestureLeaves
fn slider_base() -> (DagSnapshot, String) {
    let base = default_snapshot();
    let id = "slider-99".to_string();
    let (forward, _) = apply_mutation(&base, &create_node(crate::schema::default_node_for_kind("slider", &id, 0.0, 0.0))).expect("slider node");
    (forward, id)
}

fn slider_numbers(snapshot: &DagSnapshot, id: &str) -> (f64, f64, f64) {
    match snapshot.nodes().into_iter().find(|node| node.id == id).map(|node| node.kind) {
        Some(crate::DagNodeKind::Slider { value, min, max, .. }) => (value, min, max),
        other => panic!("{id} is no slider: {other:?}"),
    }
}

/// ⚖️ LAW: `move-nodes` moves every addressed node by the offset from its BASE position, inverts to ONE absolute
/// `set-node-positions` row (the one-item fold contract), and that row restores the document byte-for-byte.
#[semio_framework_async_macros::async_test]
async fn move_nodes_is_relative_and_inverts_to_one_absolute_row() {
    let base = default_snapshot();
    let nodes = base.nodes();
    let ids: Vec<String> = nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let moved = round_trip(&base, &move_nodes(ids.clone(), 40.0, -12.5));
    for id in &ids {
        let (before, after) = (nodes.iter().find(|node| &node.id == id).expect("base node"), moved.nodes().into_iter().find(|node| &node.id == id).expect("moved node"));
        assert_eq!((after.x, after.y), (before.x + 40.0, before.y - 12.5));
    }
    let inverse = move_nodes(ids.clone(), 40.0, -12.5).inverse(&base);
    assert_eq!(inverse.len(), 1, "a multi-node drag inverts to one row: {inverse:?}");
    assert!(matches!(&inverse[0], DagMutation::SetNodePositions(payload) if payload.ids() == ids));
    assert_mutation_inverse_law(&base, &move_nodes(ids, 40.0, -12.5)).await;
}

/// ⚖️ LAW: the outcome vocabulary of `move-nodes` — missing nodes are a partial Warning, none left is target-missing, a
/// zero offset is a no-op, a malformed target list or a non-finite offset is a Fatal invariant that never applies.
#[semio_framework_async_macros::async_test]
async fn move_nodes_outcomes_follow_the_vocabulary() {
    let base = default_snapshot();
    let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
    let partial = move_nodes(vec![id.clone(), "ghost-node".into()], 5.0, 5.0).diff(&base);
    assert_eq!((partial.worst_level(), partial.messages()[0].code.0.as_str()), (Some(protocol::Severity::Warning), "mutation.partial"));
    assert_missing_target_is_error(&base, &move_nodes(vec!["ghost-node".into()], 5.0, 5.0)).await;
    let zero = move_nodes(vec![id.clone()], 0.0, 0.0).diff(&base);
    assert_eq!((zero.diff(), zero.messages()[0].code.0.as_str()), (&crate::DagDiff::default(), "mutation.no-op"));
    for fatal in [move_nodes(Vec::new(), 1.0, 1.0), move_nodes(vec![id.clone(), id.clone()], 1.0, 1.0), move_nodes(vec![id], f64::NAN, 0.0)] {
        let outcome = fatal.diff(&base);
        assert_fatal_never_applies(&outcome).await;
        assert_eq!(outcome.messages()[0].code.0, "mutation.invariant");
    }
}

/// ⚖️ LAW: `set-node-positions` is absolute, inverts to itself with the BASE positions, and is a no-op where every node
/// already sits.
#[semio_framework_async_macros::async_test]
async fn set_node_positions_is_absolute_and_self_inverse() {
    let base = default_snapshot();
    let nodes = base.nodes();
    let positions: Vec<DagNodePosition> = nodes.iter().take(2).enumerate().map(|(at, node)| DagNodePosition { id: node.id.clone(), x: 10.0 * at as f64, y: -3.5 }).collect();
    let placed = round_trip(&base, &set_node_positions(positions.clone()));
    for position in &positions {
        let node = placed.nodes().into_iter().find(|node| node.id == position.id).expect("placed node");
        assert_eq!((node.x, node.y), (position.x, position.y));
    }
    assert_mutation_inverse_law(&base, &set_node_positions(positions)).await;
    let stay: Vec<DagNodePosition> = nodes.iter().take(1).map(|node| DagNodePosition { id: node.id.clone(), x: node.x, y: node.y }).collect();
    assert_eq!(set_node_positions(stay.clone()).diff(&base).messages()[0].code.0, "mutation.no-op");
    assert!(set_node_positions(stay).inverse(&base).is_empty(), "a placement that moves nothing has no inverse");
    let duplicate = set_node_positions(vec![DagNodePosition { id: "a".into(), x: 0.0, y: 0.0 }, DagNodePosition { id: "a".into(), x: 1.0, y: 1.0 }]).diff(&base);
    assert_fatal_never_applies(&duplicate).await;
}

/// ⚖️ LAW: `set-slider` writes one absolute slider number, inverts to the old one, clamps a value into the range with a
/// `mutation.clamped` Warning, refuses crossing bounds and non-sliders with `mutation.target-mismatch`.
#[semio_framework_async_macros::async_test]
async fn set_slider_is_absolute_and_follows_the_vocabulary() {
    let (base, id) = slider_base();
    let (_, min, max) = slider_numbers(&base, &id);
    let set = round_trip(&base, &set_slider(id.clone(), DagSliderField::Value, 7.5));
    assert_eq!(slider_numbers(&set, &id).0, 7.5);
    assert_mutation_inverse_law(&base, &set_slider(id.clone(), DagSliderField::Max, max + 5.0)).await;
    let clamped = set_slider(id.clone(), DagSliderField::Value, max + 100.0).diff(&base);
    assert_eq!((clamped.worst_level(), clamped.messages()[0].code.0.as_str()), (Some(protocol::Severity::Warning), "mutation.clamped"));
    assert_eq!(slider_numbers(&apply_mutation(&base, &set_slider(id.clone(), DagSliderField::Value, max + 100.0)).expect("clamped").0, &id).0, max);
    for crossing in [set_slider(id.clone(), DagSliderField::Min, max + 1.0), set_slider(id.clone(), DagSliderField::Max, min - 1.0)] {
        assert_eq!(crossing.diff(&base).messages()[0].code.0, "mutation.target-mismatch");
    }
    let Some(note) = base.nodes().into_iter().find(|node| !matches!(node.kind, crate::DagNodeKind::Slider { .. })).map(|node| node.id) else { return };
    assert_eq!(set_slider(note, DagSliderField::Value, 1.0).diff(&base).messages()[0].code.0, "mutation.target-mismatch");
    assert_missing_target_is_error(&base, &set_slider("ghost-node".into(), DagSliderField::Value, 1.0)).await;
    let fatal = set_slider(id, DagSliderField::Min, f64::INFINITY).diff(&base);
    assert_fatal_never_applies(&fatal).await;
}

/// ⚖️ LAW: a snapshot diff expresses position changes as intent — nodes moved by one offset share ONE `move-nodes` leaf
/// (rounding noise of a host's per-node `before + delta` included), different offsets are different leaves.
#[test]
fn snapshot_moves_group_by_offset_into_move_nodes() {
    let before: Vec<crate::DagNodeSpec> = ["a", "b", "c"].iter().enumerate().map(|(at, id)| crate::schema::default_node_for_kind("note", id, 0.1 + at as f64 * 0.6, 0.7)).collect();
    let mut after = before.clone();
    after[0].x += 0.2;
    after[1].x += 0.2;
    after[2].y += 5.0;
    let leaves = dag_move_leaves(&before, &after);
    assert_eq!(leaves.len(), 2, "{leaves:?}");
    assert!(matches!(&leaves[0], DagMutation::MoveNodes(payload) if payload.ids == ["a", "b"] && (payload.dx - 0.2).abs() < 1e-12 && payload.dy == 0.0));
    assert!(matches!(&leaves[1], DagMutation::MoveNodes(payload) if payload.ids == ["c"] && payload.dx == 0.0 && payload.dy == 5.0));
    assert!(dag_move_leaves(&before, &before).is_empty());
}

/// 🏷️ Every gesture leaf is labelled in English and German from its payload.
#[test]
fn gesture_leaves_label_in_both_languages() {
    let resolve = |mutation: &DagMutation| {
        let label = <DagMutation as SemanticMutation<DagSnapshot>>::label(mutation);
        (label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string(), label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string())
    };
    assert_eq!(resolve(&move_nodes(vec!["a".into(), "b".into()], 40.0, -12.5)), ("Move 2 node(s) by (40, -12.5)".into(), "2 Knoten um (40; -12,5) verschieben".into()));
    assert_eq!(resolve(&set_slider("s".into(), DagSliderField::Value, 7.5)), ("Set slider \"s\" value to 7.5".into(), "Wert von Schieberegler \"s\" auf 7,5 setzen".into()));
    assert_eq!(resolve(&set_node_positions(vec![DagNodePosition { id: "a".into(), x: 1.0, y: 2.0 }])), ("Set the positions of 1 node(s)".into(), "Positionen von 1 Knoten setzen".into()));
}
//#endregion 🔖️GestureLeaves
