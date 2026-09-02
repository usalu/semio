//! 🧬️ DAG artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DagSnapshot>` and
//! `impl protocol::SemanticMutation<DagSnapshot>` from those payloads — no hand-written apply/diff/
//! inverse dispatch here, and no bridge into `infinite_board_port_directed_dag::DagMutation` (the
//! foreign kernel port type) either — see `📝️text/🦀️.rs` for the local `DagMutationDsl`
//! mirror that replaced it.

use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type DagEnvelope = store::ArtifactEnvelope<DagSnapshot, DagMutation>;
pub type DagStore = store::ArtifactStore<DagSnapshot, DagMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic DAG document mutation vocabulary: id-keyed node create/delete/rename/move/resize/
/// change-<field>/replace-<payload>/reorder, plus relationship connect/disconnect between node
/// ports. The old generic id-keyed-collection wrapper variants for nodes/edges, and the old
/// whole-collection and whole-document replacement variants, are all gone with no direct
/// replacement — whole-collection/whole-document replace is not an in-history mutation (see
/// `crate::editor::dag::DagPlayApp` no longer overriding `whole_document_operation`; use
/// `store::ArtifactStore::reset` for a real whole-document load).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations, Serialize, Deserialize)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = DagSnapshot, diff = DagDiff, schema = "dag.dag")]
pub enum DagMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    RenameNode(RenameNode),
    ChangeNodeName(ChangeNodeName),
    MoveNode(MoveNode),
    ResizeNode(ResizeNode),
    ChangeNodeIcon(ChangeNodeIcon),
    ChangeNodeAbbreviation(ChangeNodeAbbreviation),
    ChangeNodeOperatorKind(ChangeNodeOperatorKind),
    ReplaceNodeKind(ReplaceNodeKind),
    ReplaceNodeProperties(ReplaceNodeProperties),
    ReorderNodes(ReorderNodes),
    ConnectNodes(ConnectNodes),
    DisconnectNodes(DisconnectNodes),
}
//#endregion 🔖️Mutations

pub use super::change_node_abbreviation::mutation::{change_node_abbreviation, ChangeNodeAbbreviation};
pub use super::change_node_icon::mutation::{change_node_icon, ChangeNodeIcon};
pub use super::change_node_name::mutation::{change_node_name, ChangeNodeName};
pub use super::change_node_operator_kind::mutation::{change_node_operator_kind, ChangeNodeOperatorKind};
pub use super::connect_nodes::mutation::{connect_nodes, ConnectNodes};
pub use super::create_node::mutation::{create_node, CreateNode};
pub use super::delete_node::mutation::{delete_node, DeleteNode};
pub use super::disconnect_nodes::mutation::{disconnect_nodes, DisconnectNodes};
pub use super::move_node::mutation::{move_node, MoveNode};
pub use super::rename_node::mutation::{rename_node, RenameNode};
pub use super::reorder_nodes::mutation::{reorder_nodes, ReorderNodes};
pub use super::replace_node_kind::mutation::{replace_node_kind, ReplaceNodeKind};
pub use super::replace_node_properties::mutation::{replace_node_properties, ReplaceNodeProperties};
pub use super::resize_node::mutation::{resize_node, ResizeNode};

/// 🏷️ Kebab-case spelling of every [`DagMutation`] variant, in declaration order — the vocabulary
/// the `dag-1-any` mutation catalog (`../../🔣️oracle.json`) declares and `mutate-dag-1`'s
/// exhaustive case measures itself against. There is deliberately no `no-mutation` and no
/// `set-snapshot`: whole-collection and whole-document replacement are banned vocabulary here (see
/// the enum's own docstring) and reach the store through `ArtifactStore::reset` instead.
/// [`kinds_match_the_enum_and_the_catalog`] keeps this list honest against the enum, since the
/// framework never parses Rust.
pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "rename-node",
    "change-node-name",
    "move-node",
    "resize-node",
    "change-node-icon",
    "change-node-abbreviation",
    "change-node-operator-kind",
    "replace-node-kind",
    "replace-node-properties",
    "reorder-nodes",
    "connect-nodes",
    "disconnect-nodes",
];

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "moveNode", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification vectors and
/// `mutate-dag-1`'s own `Examples` payloads carry — into a real [`DagMutation`]. The test adapter
/// cannot reach `serde_json` (the generated host links only `semio-repo-test-host` and this crate)
/// and cannot name this crate's private `protocol`/`store` extern-crate aliases either, so the
/// bridge belongs here rather than there.
pub fn decode_dag_mutation_json(text: &str) -> Result<DagMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ [`apply_dag_mutation`]'s reporting, non-async twin: applies `mutation` in place and returns
/// every diagnostic it raised as `(code, severity, target)` triples. [`apply_dag_mutation`] discards
/// them and is `async`, and this vocabulary's whole committed specification-vector suite is a
/// REJECTION suite — `mutation.target-missing`, `mutation.duplicate-id`, `mutation.invariant` —
/// so the diagnostics are the evidence, not a side channel.
pub fn apply_dag_mutation_reporting(snapshot: &mut DagSnapshot, mutation: &DagMutation) -> Vec<(String, String, Vec<String>)> {
    let outcome = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level), message.target.clone())).collect()
}

/// ↩️ [`inverse_dag_mutation`]'s non-async twin — the mutation's OWN computed undo steps, which is
/// what an `inverse-<kind>` scenario has to apply for the metamorphic law to mean anything.
pub fn inverse_dag_mutation_steps(mutation: &DagMutation, base: &DagSnapshot) -> Vec<DagMutation> {
    <DagMutation as protocol::Mutation<DagSnapshot>>::inverse(mutation, base)
}
/// 🌱 Resolves `snapshot`'s composed `s.stdio.semio.graph` child to a working scene holding exactly
/// the node a `create-node` payload carries, and reports whether it did. `create-node` is the one
/// verb in this vocabulary with NO rejection branch on an empty scene, so its committed
/// `mutation.duplicate-id` vector is only reachable once the id it collides with is actually
/// present — which is precisely what
/// `🌱create-node/🧪️tests/rejects-a-duplicate-node-id/🦀️.rs::before` does with
/// its exact child owner. Exposed here because seeding from the committed payload is what keeps the
/// vector free of any transcription: the seeded node IS the mutation JSON's own `node`.
pub fn seed_dag_working_scene_with(snapshot: &mut DagSnapshot, mutation: &DagMutation) -> bool {
    let DagMutation::CreateNode(payload) = mutation else { return false };
    snapshot.content.set_local_owner(std::sync::Arc::new(crate::artifacts::dag::DagWorkingScene { nodes: vec![payload.node.clone()], edges: Vec::new() }));
    true
}
//#endregion 🌉️ExternalCodecBridge

/// ▶️ Applies `mutation` via its diff.
pub async fn apply_dag_mutation(snapshot: &mut DagSnapshot, mutation: &DagMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

pub async fn inverse_dag_mutation(snapshot: &DagSnapshot, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as protocol::Mutation<DagSnapshot>>::inverse(mutation, snapshot)
}

/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set — the re-expression the
/// former whole-collection and whole-document replacement call sites (whole-fixture paste,
/// auto-reorganize) now go through instead of a snapshot swap. Doesn't detect node id renames
/// (shows as a delete+create pair); `🎮️commands/🔧️add-node::rename_dag_node` uses the dedicated
/// `rename-node` mutation directly for that gesture instead of this generic differ.
pub async fn dag_snapshot_mutations(before: &DagSnapshot, after: &DagSnapshot) -> Vec<DagMutation> {
    let before_nodes = before.nodes();
    let after_nodes = after.nodes();
    let before_edges = before.edges();
    let after_edges = after.edges();
    let mut mutations = Vec::new();
    for node in &before_nodes {
        if !after_nodes.iter().any(|entry| entry.id == node.id) {
            mutations.push(delete_node(node.id.clone()));
        }
    }
    for node in &after_nodes {
        match before_nodes.iter().find(|entry| entry.id == node.id) {
            None => mutations.push(create_node(node.clone())),
            Some(prior) => {
                if prior.name != node.name {
                    mutations.push(change_node_name(node.id.clone(), node.name.clone()));
                }
                if prior.x != node.x || prior.y != node.y {
                    mutations.push(move_node(node.id.clone(), node.x, node.y));
                }
                if prior.width != node.width || prior.height != node.height {
                    mutations.push(resize_node(node.id.clone(), node.width, node.height));
                }
                if prior.icon != node.icon {
                    mutations.push(change_node_icon(node.id.clone(), node.icon.clone()));
                }
                if prior.abbreviation != node.abbreviation {
                    mutations.push(change_node_abbreviation(node.id.clone(), node.abbreviation.clone()));
                }
                if prior.operator_kind != node.operator_kind {
                    mutations.push(change_node_operator_kind(node.id.clone(), node.operator_kind.clone()));
                }
                if prior.kind != node.kind {
                    mutations.push(replace_node_kind(node.id.clone(), node.kind.clone()));
                }
                if prior.properties != node.properties {
                    mutations.push(replace_node_properties(node.id.clone(), node.properties.clone()));
                }
            }
        }
    }
    for edge in &before_edges {
        if !after_edges.iter().any(|entry| entry.id == edge.id) {
            mutations.push(disconnect_nodes(edge.id.clone()));
        }
    }
    for edge in &after_edges {
        match before_edges.iter().find(|entry| entry.id == edge.id) {
            None => mutations.push(connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone())),
            Some(prior) if prior.source != edge.source || prior.target != edge.target || prior.route_style != edge.route_style || prior.properties != edge.properties => {
                mutations.push(disconnect_nodes(edge.id.clone()));
                mutations.push(connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone()));
            }
            Some(_) => {}
        }
    }
    mutations
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dag::default_snapshot;
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_deterministic};
    use protocol::Mutation;
    use protocol::SemanticMutation;
    use vcs::apply_mutation;

    /// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
    /// — must agree, in spelling AND in order. The framework never parses Rust, so without this
    /// test `KINDS` could drift from the enum and the catalog could keep measuring `mutate-dag-1`
    /// against a vocabulary the artifact no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = DagMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"no-mutation\""), "whole-document replace is banned vocabulary here — the catalog must not smuggle it back in");
    }

    async fn round_trip(snapshot: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
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

    async fn sample_node(id: &str, x: f64, y: f64) -> crate::artifacts::dag::DagNodeSpec {
        crate::artifacts::dag::schema::default_node_for_kind("note", id, x, y)
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
        assert_mutation_inverse_law(&base, &create_node(sample_node("node-99", 5.0, 6.0)));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &delete_node(id));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &rename_node(id, "renamed-node".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &move_node(id, 42.0, -8.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &resize_node(id, 200.0, 90.0));
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
        assert_mutation_inverse_law(&base, &connect_nodes("edge-99".into(), format!("{source}@out"), format!("{target}@in"), infinite_board_port_directed_dag::EdgeRouteStyle::default(), Default::default()));
        if let Some(edge_id) = base.edges().first().map(|edge| edge.id.clone()) {
            assert_mutation_inverse_law(&base, &disconnect_nodes(edge_id));
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
        assert_mutation_inverse_law(&base, &reorder_nodes(order));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        let d1 = move_node(id.clone(), 10.0, 10.0).diff(&base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = move_node(id, 20.0, 30.0).diff(&mid).diff().clone();
        assert_mutation_diff_absorb_law(&base, d1, d2);
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
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_node_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &delete_node("ghost-node".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_node_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &rename_node("ghost-node".into(), "x".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &move_node("ghost-node".into(), 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_non_finite_is_fatal() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        let outcome = move_node(id, f64::NAN, 0.0).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_node_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &resize_node("ghost-node".into(), 10.0, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_node_name_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &change_node_name("ghost-node".into(), "x".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_node_kind_missing_target_is_error() {
        let base = default_snapshot();
        let Some(kind) = base.nodes().first().map(|node| node.kind.clone()) else { return };
        assert_missing_target_is_error(&base, &replace_node_kind("ghost-node".into(), kind));
    }

    #[semio_framework_async_macros::async_test]
    async fn disconnect_nodes_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &disconnect_nodes("ghost-edge".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_nodes_missing_endpoint_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &connect_nodes("edge-99".into(), "ghost-source@out".into(), "ghost-target@in".into(), infinite_board_port_directed_dag::EdgeRouteStyle::default(), Default::default()));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_nodes_self_loop_is_fatal() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        let outcome = connect_nodes("edge-99".into(), format!("{id}@out"), format!("{id}@in"), infinite_board_port_directed_dag::EdgeRouteStyle::default(), Default::default()).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_nodes_duplicate_id_is_fatal() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        let outcome = reorder_nodes(vec![id.clone(), id]).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_node_diff_is_deterministic() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_outcome_deterministic(&base, &move_node(id, 7.0, 8.0));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
