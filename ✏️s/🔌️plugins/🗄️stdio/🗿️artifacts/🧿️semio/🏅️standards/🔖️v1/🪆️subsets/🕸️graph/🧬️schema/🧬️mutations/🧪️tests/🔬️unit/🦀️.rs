use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
use protocol::{Mutation, MutationDiff, SemanticMutation};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioGraphSnapshot {
    SemioGraphSnapshot {
        nodes: vec![
            SemioGraphNode { id: GraphNodeId::new("n1"), kind: "source".into(), label: "Source".into(), position: SemioPoint2 { x: 0.0, y: 0.0 }, width: 0.0, height: 0.0, ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out, category: String::new(), properties: Vec::new() }], properties: vec![] },
            SemioGraphNode { id: GraphNodeId::new("n2"), kind: "sink".into(), label: "Sink".into(), position: SemioPoint2 { x: 10.0, y: 10.0 }, width: 0.0, height: 0.0, ports: vec![], properties: vec![] },
        ],
        edges: vec![SemioGraphEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into(), source_port: None, target_port: None, properties: Vec::new() }],
        ..Default::default()
    }
}

/// 🔧️ `nodes`/`edges` are id-keyed SETS with no user-meaningful display order (this facet's
/// own dispatch doc comment) — `create-node`'s diff always APPENDS, so a cascading
/// `delete-node` inverse that recreates a node deleted from the middle of `base.nodes`
/// legitimately lands it at the end. Comparing snapshots for round-trip fidelity must therefore
/// be order-INSENSITIVE over `nodes`/`edges` (same SET, not same SEQUENCE) — a physical `Vec`
/// is the storage representation, not the domain's equality contract.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sorted_by_id(mut s: SemioGraphSnapshot) -> SemioGraphSnapshot {
    s.nodes.sort_by(|a, b| a.id.value.cmp(&b.id.value));
    s.edges.sort_by(|a, b| a.id.value.cmp(&b.id.value));
    s
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioGraphSnapshot, operation: &SemioGraphMutation) -> SemioGraphSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base).expect("valid retained mutation inverse fixture");
    let mut restored = forward.clone();
    // 🔧️ Each inverse's diff must be computed against the CURRENT (`restored`) state, not the
    // stale pre-operation `base` — a whole-list-replace diff shape reconstructs the entire
    // collection from whatever base it is given, so diffing against the wrong base silently
    // discards the forward mutation's effect instead of undoing it (see `🔤️text`'s own fix).
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(sorted_by_id(restored), sorted_by_id(base.clone()), "inverse must exactly restore the pre-operation fixture (order-insensitive over the id-keyed node/edge sets)");
    forward
}

#[semio_framework_async_macros::async_test]
async fn create_delete_node_round_trips() {
    let base = fixture();
    let new_node = SemioGraphNode { id: GraphNodeId::new("n3"), kind: "extra".into(), label: "Extra".into(), position: SemioPoint2 { x: 5.0, y: 5.0 }, width: 0.0, height: 0.0, ports: vec![], properties: vec![] };

    let create = SemioGraphMutation::CreateNode(create_node::CreateNode {
        id: new_node.id.clone(),
        kind: new_node.kind.clone(),
        label: new_node.label.clone(),
        position: new_node.position.clone(), width: new_node.width, height: new_node.height,
        ports: new_node.ports.clone(),
        properties: new_node.properties.clone(),
        at: None,
    });
    let after_create = round_trip(&base, &create);
    assert_eq!(after_create.nodes.len(), base.nodes.len() + 1);
    assert_eq!(after_create.nodes.last().unwrap(), &new_node);

    let undo = create.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo, vec![SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: new_node.id.clone() })]);

    let delete = SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: GraphNodeId::new("n1") });
    let after_delete = round_trip(&base, &delete);
    assert_eq!(after_delete.nodes.len(), base.nodes.len() - 1);
    assert!(after_delete.edges.is_empty(), "delete-node must cascade-remove every severed edge");
    let mut restored = after_delete;
    for back in delete.inverse(&base).expect("delete-node inverse") {
        restored = back.diff(&restored).diff().apply(&restored).expect("the restore applies");
    }
    assert_eq!(restored, base, "deleting the FIRST node and undoing it restores the exact sequence, not only the set");
}

/// ⚖️ LAW: deleting a node from the MIDDLE of the sets (with edges before and after its own) and undoing it restores the
/// byte-identical pack and therefore the identical content address — content-addressed graph children (trinity, dag) keep
/// their child id across delete → undo, and a time-travel fold over the undo equals the fold before it.
#[semio_framework_async_macros::async_test]
async fn delete_then_undo_restores_byte_identical_snapshot_bytes() {
    use store::ArtifactPack;
    let node = |id: &str, x: f64| SemioGraphNode { id: GraphNodeId::new(id), kind: "k".into(), label: id.to_uppercase(), position: SemioPoint2 { x, y: -x }, width: 2.0, height: 1.0, ports: vec![], properties: vec![SemioValueEntry { key: "w".into(), value: SemioValue::Int { lexeme: "3".into() } }] };
    let edge = |id: &str, source: &str, target: &str| SemioGraphEdge { id: GraphEdgeId::new(id), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: "flow".into(), label: String::new(), source_port: Some("out".into()), target_port: None, properties: Vec::new() };
    let base = SemioGraphSnapshot { nodes: vec![node("a", 0.0), node("b", 1.5), node("c", 3.0)], edges: vec![edge("e1", "a", "c"), edge("e2", "a", "b"), edge("e3", "c", "a"), edge("e4", "b", "c")], ..Default::default() };
    for (mutation, label) in [(SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: GraphNodeId::new("b") }), "delete-node b"), (SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: GraphEdgeId::new("e2") }), "delete-edge e2")] {
        let mut current = mutation.diff(&base).diff().apply(&base).expect("the delete applies");
        for back in mutation.inverse(&base).expect("delete inverse") {
            current = back.diff(&current).diff().apply(&current).expect("the undo applies");
        }
        let (before, after) = (SemioGraphSnapshot::encode_pack(&base), SemioGraphSnapshot::encode_pack(&current));
        assert_eq!(after, before, "{label}: undo must restore byte-identical pack bytes");
        assert_eq!(store::content_id("graph", &after), store::content_id("graph", &before), "{label}: undo must keep the content address");
    }
}

#[semio_framework_async_macros::async_test]
async fn delete_node_of_an_absent_id_has_an_empty_inverse() {
    let base = fixture();
    let delete = SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: GraphNodeId::new("absent") });
    assert!(delete.inverse(&base).expect("valid retained mutation inverse fixture").is_empty(), "deleting an absent node has nothing to undo");
    assert_eq!(delete.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "an absent-id delete is a no-op");
}

/// 🧱️ LAW (audit F3): `delete-node` severs at most the edges its schema-declared inverse rows cover. AT the bound the delete
/// applies and its undo is exactly the declared rows, restoring the snapshot; ONE edge above it the delete is
/// `mutation.target-referenced`, changes nothing and has nothing to undo — the recorded rows never exceed the footprint the
/// store admitted from the declaration.
#[semio_framework_async_macros::async_test]
async fn delete_node_refuses_one_edge_above_its_declared_cascade_bound() {
    let hub = delete_node::DeleteNode { id: GraphNodeId::new("hub") };
    let declared = protocol::MutationLeaf::inverse_rows(&hub);
    let maximum = delete_node::diff::cascade_edges_maximum(&hub);
    assert_eq!(maximum + 1, declared, "the cascade bound is the declared inverse rows less the row that restores the node");
    let node = |id: &str| SemioGraphNode { id: GraphNodeId::new(id), kind: "k".into(), label: id.into(), position: SemioPoint2 { x: 0.0, y: 0.0 }, width: 0.0, height: 0.0, ports: vec![], properties: vec![] };
    let star = |degree: usize| SemioGraphSnapshot {
        nodes: vec![node("hub"), node("rim")],
        edges: (0..degree).map(|index| { let (source, target) = if index % 2 == 0 { ("hub", "rim") } else { ("rim", "hub") }; SemioGraphEdge { id: GraphEdgeId::new(format!("e{index}")), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: "flow".into(), label: String::new(), source_port: None, target_port: None, properties: Vec::new() } }).collect(),
        ..Default::default()
    };
    let delete = SemioGraphMutation::DeleteNode(hub);
    assert_eq!(<SemioGraphMutation as Mutation<SemioGraphSnapshot>>::inverse_rows(&delete), declared, "the aggregate answers the leaf's declared rows");

    let at_bound = star(maximum);
    let applied = delete.diff(&at_bound);
    assert_eq!(applied.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec!["mutation.cascade"], "at the bound the delete applies with its cascade note only");
    let undo = delete.inverse(&at_bound).expect("delete-node inverse at the bound");
    assert_eq!(undo.len(), declared, "at the bound the undo is exactly the declared rows");
    let mut restored = applied.diff().apply(&at_bound).expect("the delete at the bound applies");
    assert_eq!((restored.nodes.len(), restored.edges.len()), (1, 0), "the hub and every incident edge are gone");
    for back in &undo {
        restored = back.diff(&restored).diff().apply(&restored).expect("each undo row applies");
    }
    assert_eq!(restored, at_bound, "the undo at the bound restores the exact snapshot");

    let above = star(maximum + 1);
    let refused = delete.diff(&above);
    let messages = refused.messages();
    assert_eq!(messages.len(), 1, "one edge above the bound is exactly one refusal");
    assert_eq!((messages[0].code.0.as_str(), messages[0].level, messages[0].target.clone()), ("mutation.target-referenced", semio_framework_diagnostic::Severity::Error, vec!["hub".to_string()]), "the refusal names the node with the frozen outcome code");
    assert_eq!(refused.diff().apply(&above).expect("a refused delete carries the empty diff"), above, "a refused delete changes nothing");
    assert!(delete.inverse(&above).expect("delete-node inverse above the bound").is_empty(), "a refused delete has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn delete_node_inverse_is_a_real_multi_mutation_cascade() {
    let base = fixture();
    let delete = SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: GraphNodeId::new("n1") });
    let undo = delete.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo.len(), 2, "inverse must restore the node AND every severed edge");
    assert_eq!(
        undo[0],
        SemioGraphMutation::CreateNode(create_node::CreateNode {
            id: GraphNodeId::new("n1"),
            kind: "source".into(),
            label: "Source".into(),
            position: SemioPoint2 { x: 0.0, y: 0.0 }, width: 0.0, height: 0.0,
            ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out, category: String::new(), properties: Vec::new() }],
            properties: vec![],
            at: Some(0),
        })
    );
    assert_eq!(undo[1], SemioGraphMutation::CreateEdge(create_edge::CreateEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into(), source_port: None, target_port: None, properties: Vec::new(), at: Some(0) }));
}

#[semio_framework_async_macros::async_test]
async fn change_node_kind_and_label_and_move_node_round_trip() {
    let base = fixture();

    let change_kind = SemioGraphMutation::ChangeNodeKind(change_node_kind::ChangeNodeKind { id: GraphNodeId::new("n1"), new_kind: "relay".into() });
    let after = round_trip(&base, &change_kind);
    assert_eq!(after.nodes[0].kind, "relay");

    let change_label = SemioGraphMutation::ChangeNodeLabel(change_node_label::ChangeNodeLabel { id: GraphNodeId::new("n1"), new_label: "Renamed".into() });
    let after = round_trip(&base, &change_label);
    assert_eq!(after.nodes[0].label, "Renamed");

    let move_op = SemioGraphMutation::MoveNode(move_node::MoveNode { id: GraphNodeId::new("n1"), new_position: SemioPoint2 { x: 99.0, y: -1.0 } });
    let after = round_trip(&base, &move_op);
    assert_eq!(after.nodes[0].position, SemioPoint2 { x: 99.0, y: -1.0 });

    let missing = SemioGraphMutation::ChangeNodeKind(change_node_kind::ChangeNodeKind { id: GraphNodeId::new("absent"), new_kind: "x".into() });
    assert!(missing.inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_node_port_round_trips() {
    let base = fixture();
    let port = SemioGraphPort { name: "extra".into(), kind: SemioGraphPortKind::InOut, category: String::new(), properties: Vec::new() };

    let add = SemioGraphMutation::AddNodePort(add_node_port::AddNodePort { node_id: GraphNodeId::new("n2"), index: 0, port: port.clone() });
    let after_add = round_trip(&base, &add);
    assert_eq!(after_add.nodes[1].ports, vec![port.clone()]);

    let undo = add.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo, vec![SemioGraphMutation::RemoveNodePort(remove_node_port::RemoveNodePort { node_id: GraphNodeId::new("n2"), index: 0 })]);

    let remove = SemioGraphMutation::RemoveNodePort(remove_node_port::RemoveNodePort { node_id: GraphNodeId::new("n1"), index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert!(after_remove.nodes[0].ports.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_node_property_round_trips() {
    let base = fixture();
    let property = SemioValueEntry { key: "weight".into(), value: SemioValue::Int { lexeme: "7".into() } };

    let add = SemioGraphMutation::AddNodeProperty(add_node_property::AddNodeProperty { node_id: GraphNodeId::new("n1"), index: 0, property: property.clone() });
    let after_add = round_trip(&base, &add);
    assert_eq!(after_add.nodes[0].properties, vec![property.clone()]);

    let undo = add.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo, vec![SemioGraphMutation::RemoveNodeProperty(remove_node_property::RemoveNodeProperty { node_id: GraphNodeId::new("n1"), key: "weight".into() })]);

    let remove = SemioGraphMutation::RemoveNodeProperty(remove_node_property::RemoveNodeProperty { node_id: GraphNodeId::new("absent"), key: "weight".into() });
    assert!(remove.inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
    assert_eq!(remove.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn create_delete_edge_round_trips() {
    let base = fixture();
    let new_edge = SemioGraphEdge { id: GraphEdgeId::new("e2"), source: GraphNodeId::new("n2"), target: GraphNodeId::new("n1"), kind: "back".into(), label: "Return".into(), source_port: None, target_port: None, properties: Vec::new() };

    let create = SemioGraphMutation::CreateEdge(create_edge::CreateEdge { id: new_edge.id.clone(), source: new_edge.source.clone(), target: new_edge.target.clone(), kind: new_edge.kind.clone(), label: new_edge.label.clone(), source_port: new_edge.source_port.clone(), target_port: new_edge.target_port.clone(), properties: new_edge.properties.clone(), at: None });
    let after_create = round_trip(&base, &create);
    assert_eq!(after_create.edges.last().unwrap(), &new_edge);

    let undo = create.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo, vec![SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: new_edge.id.clone() })]);

    let delete = SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: GraphEdgeId::new("e1") });
    let after_delete = round_trip(&base, &delete);
    assert!(after_delete.edges.is_empty());
    assert_eq!(after_delete.nodes.len(), base.nodes.len(), "delete-edge must not cascade into nodes");
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioGraphMutation::kinds().len(), KINDS.len());
    let mutation = SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: GraphNodeId::new("n1") });
    assert_eq!(mutation.semantics().kind, "delete-node");
    assert_eq!(mutation.semantics().record, "DeletedNode");
    assert_eq!(mutation.target(), vec!["n1".to_string()]);
}

/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-graph`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioGraphMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
