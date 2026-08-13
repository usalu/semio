//! 🧬️ DAG artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DagSnapshot>` and
//! `impl protocol::SemanticMutation<DagSnapshot>` from those payloads — no hand-written apply/diff/
//! inverse dispatch here, and no bridge into `infinite_board_port_directed_dag::DagMutation` (the
//! foreign kernel port type) either — see `📝️text/🦀️component.rs` for the local `DagMutationDsl`
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
/// `crate::apps::dag::DagPlayApp` no longer overriding `whole_document_operation`; use
/// `store::ArtifactStore::reset` for a real whole-document load).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
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

/// ▶️ Applies `mutation` via its diff.
pub fn apply_dag_mutation(snapshot: &mut DagSnapshot, mutation: &DagMutation) {
    use store::MutationDiff;
    *snapshot = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(mutation, snapshot).apply(snapshot);
}

pub fn inverse_dag_mutation(snapshot: &DagSnapshot, mutation: &DagMutation) -> Vec<DagMutation> {
    use store::Mutation;
    <DagMutation as protocol::Mutation<DagSnapshot>>::inverse(mutation, snapshot)
}

/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set — the re-expression the
/// former whole-collection and whole-document replacement call sites (whole-fixture paste,
/// auto-reorganize) now go through instead of a snapshot swap. Doesn't detect node id renames
/// (shows as a delete+create pair); `🎮️commands/🔧️add-node::rename_dag_node` uses the dedicated
/// `rename-node` mutation directly for that gesture instead of this generic differ.
pub fn dag_snapshot_mutations(before: &DagSnapshot, after: &DagSnapshot) -> Vec<DagMutation> {
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
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::Mutation;
    use protocol::SemanticMutation;
    use vcs::apply_mutation;

    fn round_trip(snapshot: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
        let forward = apply_mutation(snapshot, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(snapshot) {
            restored = apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot");
        forward
    }

    fn sample_node(id: &str, x: f64, y: f64) -> crate::artifacts::dag::DagNodeSpec {
        crate::artifacts::dag::schema::default_node_for_kind("note", id, x, y)
    }

    #[test]
    fn create_move_resize_delete_node_round_trip() {
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

    #[test]
    fn rename_node_cascades_edge_endpoints() {
        let snapshot = default_snapshot();
        let Some(id) = snapshot.nodes().first().map(|node| node.id.clone()) else { return };
        let renamed = round_trip(&snapshot, &rename_node(id.clone(), "renamed-node".into()));
        assert!(renamed.nodes().iter().any(|node| node.id == "renamed-node"));
        assert!(renamed.edges().iter().all(|edge| !edge.source.starts_with(&format!("{id}@")) && !edge.target.starts_with(&format!("{id}@"))));
    }

    #[test]
    fn delete_node_severs_and_reconnects_edges() {
        let snapshot = default_snapshot();
        let Some(id) = snapshot.nodes().first().map(|node| node.id.clone()) else { return };
        round_trip(&snapshot, &delete_node(id));
    }

    #[test]
    fn reorder_nodes_round_trips() {
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
    #[test]
    fn create_node_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &create_node(sample_node("node-99", 5.0, 6.0)));
    }

    #[test]
    fn delete_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &delete_node(id));
    }

    #[test]
    fn rename_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &rename_node(id, "renamed-node".into()));
    }

    #[test]
    fn move_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &move_node(id, 42.0, -8.0));
    }

    #[test]
    fn resize_node_inverse_law() {
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        assert_mutation_inverse_law(&base, &resize_node(id, 200.0, 90.0));
    }

    #[test]
    fn connect_disconnect_nodes_inverse_law() {
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

    #[test]
    fn reorder_nodes_inverse_law() {
        let base = default_snapshot();
        let nodes = base.nodes();
        if nodes.len() < 2 {
            return;
        }
        let mut order: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
        order.reverse();
        assert_mutation_inverse_law(&base, &reorder_nodes(order));
    }

    #[test]
    fn move_node_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let Some(id) = base.nodes().first().map(|node| node.id.clone()) else { return };
        let d1 = move_node(id.clone(), 10.0, 10.0).diff(&base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = move_node(id, 20.0, 30.0).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_dag_mutation_descriptors();
        for kind in DagMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(DagMutation::kinds().len(), 14);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
