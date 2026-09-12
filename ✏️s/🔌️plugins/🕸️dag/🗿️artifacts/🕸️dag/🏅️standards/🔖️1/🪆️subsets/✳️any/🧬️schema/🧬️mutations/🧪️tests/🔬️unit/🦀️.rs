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
    register_dag_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in DagMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(DagMutation::kinds().len(), 14);
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
