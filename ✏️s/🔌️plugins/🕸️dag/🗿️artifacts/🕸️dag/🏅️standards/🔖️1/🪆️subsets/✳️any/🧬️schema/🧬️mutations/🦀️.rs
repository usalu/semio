//! 🧬️ DAG artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DagSnapshot>` and
//! `impl protocol::SemanticMutation<DagSnapshot>` from those payloads — no hand-written apply/diff/
//! inverse dispatch here, and no bridge into `semio_framework_artifact_infinite_dag::DagMutation` (the
//! foreign kernel port type) either — see `📝️text/🦀️.rs` for the local `DagMutationDsl`
//! mirror that replaced it.

use crate::diff::DagDiff;
use crate::DagSnapshot;

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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
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
/// the `dag-1-any` mutation catalog (`../../🔣️oracle.json`) declares and `🌳️mutate-dag-1`'s
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
/// `🌳️mutate-dag-1`'s own `Examples` payloads carry — into a real [`DagMutation`]. The test adapter
/// cannot reach `serde_json` (the generated host links only `semio-repo-test-host` and this crate)
/// and cannot name this crate's private `protocol`/`store` extern-crate aliases either, so the
/// bridge belongs here rather than there.
pub fn decode_dag_mutation_json(text: &str) -> Result<DagMutation, String> {
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
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
/// `🌱create-node/🧪️tests/🚫️rejects-a-duplicate-node-id/🦀️.rs::before` does with
/// its exact child owner. Exposed here because seeding from the committed payload is what keeps the
/// vector free of any transcription: the seeded node IS the mutation JSON's own `node`.
pub fn seed_dag_working_scene_with(snapshot: &mut DagSnapshot, mutation: &DagMutation) -> bool {
    let DagMutation::CreateNode(payload) = mutation else { return false };
    snapshot.content.set_local_owner(std::sync::Arc::new(crate::DagWorkingScene { nodes: vec![payload.node.clone()], edges: Vec::new() }));
    true
}
//#endregion 🌉️ExternalCodecBridge

/// ▶️ Applies `mutation` via its diff.
pub fn apply_dag_mutation(snapshot: &mut DagSnapshot, mutation: &DagMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_dag_mutation(snapshot: &DagSnapshot, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as protocol::Mutation<DagSnapshot>>::inverse(mutation, snapshot)
}

/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set — the re-expression the
/// former whole-collection and whole-document replacement call sites (whole-fixture paste,
/// auto-reorganize) now go through instead of a snapshot swap. Doesn't detect node id renames
/// (shows as a delete+create pair); `🎮️commands/➕️add-node::rename_dag_node` uses the dedicated
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
