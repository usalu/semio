//! ⚡️ `trinity.graph` artifact — semantic document mutation dispatch enum + validation laws
//! (constitutional: op). Every variant is a single-field tuple wrapping a handcrafted
//! `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/` triad leaves);
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<JackSnapshot>` and
//! `impl protocol::SemanticMutation<JackSnapshot>` from those payloads — no hand-written
//! diff/inverse dispatch here. Whole-fixture replace (the old `SetFixture`) is banned; loading a
//! preset/import routes through `HostEffect::LoadDocument` (see `apps::jack::reset_document_effect`),
//! never through this enum.

use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyValue, TRINITY_GRAPH_SCHEMA};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

//#region 🔖️Mutations
/// 🧮️ Semantic trinity graph mutation vocabulary: id-keyed node/edge create+delete (cascade-capturing),
/// rename/move on nodes, and a generic node-or-edge data-property change/remove pair.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = JackSnapshot, diff = JackDiff, schema = "s.trinity.jack")]
pub enum TrinityGraphMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    CreateEdge(CreateEdge),
    DeleteEdge(DeleteEdge),
    RenameNode(RenameNode),
    MoveNode(MoveNode),
    ChangeDataProperty(ChangeDataProperty),
    RemoveDataProperty(RemoveDataProperty),
}
//#endregion 🔖️Mutations

pub use super::change_data_property::mutation::{change_data_property, ChangeDataProperty};
pub use super::create_edge::mutation::{create_edge, CreateEdge};
pub use super::create_node::mutation::{create_node, CreateNode};
pub use super::delete_edge::mutation::{delete_edge, DeleteEdge};
pub use super::delete_node::mutation::{delete_node, DeleteNode};
pub use super::move_node::mutation::{move_node, MoveNode};
pub use super::remove_data_property::mutation::{remove_data_property, RemoveDataProperty};
pub use super::rename_node::mutation::{rename_node, RenameNode};

//#region 🔖️Store
pub type TrinityGraphEnvelope = ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>;
pub type TrinityGraphStore = ArtifactStore<JackSnapshot, TrinityGraphMutation>;

pub fn create_trinity_graph_envelope(id: &str, fixture: JackSnapshot) -> TrinityGraphEnvelope {
    create_document_envelope(TRINITY_GRAPH_SCHEMA, id, fixture, None)
}
//#endregion 🔖️Store

//#region 🔖️Validation
/// 🛡️ Pre-flight manifest/reference validation for one operation against `fixture` — distinct from
/// `diff`/`inverse` (which assume a validated operation); kept centralized because it cross-checks
/// against the compile-time `Manifest`, not a single sparse-diff concern.
pub fn validate_trinity_graph_operation(operation: &TrinityGraphMutation, fixture: &JackSnapshot) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::TrinityRamError;
    match operation {
        TrinityGraphMutation::CreateNode(payload) => {
            let node = &payload.node;
            if fixture.nodes.iter().any(|existing| existing.id == node.id) {
                return Err(TrinityRamError::NodeAlreadyExists(node.id.clone()));
            }
            validate_node_kind_trinity(&fixture.manifest, &node.kind)?;
            if let Some(node_def) = fixture.manifest.node_kind(&node.kind) {
                for port in &node.ports {
                    validate_port_kind_trinity(&fixture.manifest, &port.kind)?;
                    if !node_def.port_kinds.is_empty() && !node_def.port_kinds.iter().any(|p| p == &port.kind) {
                        return Err(TrinityRamError::PortKindNotDeclaredOnMutation { node_id: node.id.clone(), port_id: port.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                    }
                }
            }
        }
        TrinityGraphMutation::DeleteNode(payload) => {
            if !fixture.nodes.iter().any(|node| node.id == payload.id) {
                return Err(TrinityRamError::NodeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::CreateEdge(payload) => {
            let edge = &payload.edge;
            if fixture.edges.iter().any(|existing| existing.id == edge.id) {
                return Err(TrinityRamError::EdgeAlreadyExists(edge.id.clone()));
            }
            validate_edge_kind_trinity(&fixture.manifest, &edge.kind)?;
            validate_edge_properties_trinity(&fixture.manifest, &edge.kind, &edge.properties)?;
            let source_node = crate::artifacts::jack::port_node_id(&edge.source).ok_or_else(|| TrinityRamError::InvalidSourcePortKey(edge.source.clone()))?;
            let target_node = crate::artifacts::jack::port_node_id(&edge.target).ok_or_else(|| TrinityRamError::InvalidTargetPortKey(edge.target.clone()))?;
            if !fixture.nodes.iter().any(|node| node.id == source_node) {
                return Err(TrinityRamError::SourceNodeNotFound(source_node.to_string()));
            }
            if !fixture.nodes.iter().any(|node| node.id == target_node) {
                return Err(TrinityRamError::TargetNodeNotFound(target_node.to_string()));
            }
        }
        TrinityGraphMutation::DeleteEdge(payload) => {
            if !fixture.edges.iter().any(|edge| edge.id == payload.id) {
                return Err(TrinityRamError::EdgeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::RenameNode(payload) => {
            if !fixture.nodes.iter().any(|node| node.id == payload.id) {
                return Err(TrinityRamError::NodeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::MoveNode(payload) => {
            if !fixture.nodes.iter().any(|node| node.id == payload.id) {
                return Err(TrinityRamError::NodeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::ChangeDataProperty(payload) => {
            validate_set_data_property(fixture, &payload.entity, &payload.key, &payload.new_value)?;
        }
        TrinityGraphMutation::RemoveDataProperty(payload) => {
            validate_clear_data_property(fixture, &payload.entity, &payload.key)?;
        }
    }
    Ok(())
}

fn validate_clear_data_property(fixture: &JackSnapshot, entity: &EntityRef, key: &str) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::TrinityRamError;
    match entity {
        EntityRef::Node(id) => {
            fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
        }
        EntityRef::Edge(id) => {
            fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
        }
    }
    let _ = key;
    Ok(())
}

fn validate_set_data_property(fixture: &JackSnapshot, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::TrinityRamError;
    let (defs, path_prefix) = match entity {
        EntityRef::Node(id) => {
            let node = fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
            (fixture.manifest.node_kind(&node.kind).map(|def| &def.properties[..]), format!("nodes/{id}/properties/{key}"))
        }
        EntityRef::Edge(id) => {
            let edge = fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
            (fixture.manifest.edge_kind(&edge.kind).map(|def| &def.properties[..]), format!("edges/{id}/properties/{key}"))
        }
    };
    let Some(defs) = defs else {
        return Err(TrinityRamError::UnknownEntityKind { path: path_prefix });
    };
    if !defs.iter().any(|def| def.name == key) {
        return Err(TrinityRamError::UnknownPropertyAtPath { path: path_prefix, key: key.to_string() });
    }
    let mut bag = PropertyBag::new();
    bag.insert(key.to_string(), value.clone());
    validate_property_bag_trinity(&path_prefix, defs, &bag)
}

fn validate_node_kind_trinity(manifest: &crate::artifacts::jack::Manifest, kind: &str) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if manifest.node_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::artifacts::jack::TrinityRamError::UnknownNodeKind { kind: kind.to_string() })
    }
}

fn validate_edge_kind_trinity(manifest: &crate::artifacts::jack::Manifest, kind: &str) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if manifest.edge_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::artifacts::jack::TrinityRamError::UnknownEdgeKind { kind: kind.to_string() })
    }
}

fn validate_port_kind_trinity(manifest: &crate::artifacts::jack::Manifest, kind: &str) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if manifest.port_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::artifacts::jack::TrinityRamError::UnknownPortKind { kind: kind.to_string() })
    }
}

fn validate_edge_properties_trinity(manifest: &crate::artifacts::jack::Manifest, kind: &str, properties: &PropertyBag) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    let Some(def) = manifest.edge_kind(kind) else {
        return validate_edge_kind_trinity(manifest, kind);
    };
    validate_property_bag_trinity(&format!("edges/{kind}/properties"), &def.properties, properties)
}

fn validate_property_bag_trinity(path: &str, defs: &[crate::artifacts::jack::PropertyDef], bag: &PropertyBag) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::{PropertyKind, TrinityRamError};
    for def in defs {
        if def.kind == PropertyKind::Derived {
            continue;
        }
        let Some(value) = bag.get(&def.name) else {
            continue;
        };
        if !property_value_matches_type_trinity(value, def) {
            return Err(TrinityRamError::PropertyTypeMismatch { path: path.to_string(), name: def.name.clone(), value_type: def.value_type.id() });
        }
    }
    for key in bag.keys() {
        if !defs.iter().any(|def| def.name == *key) {
            return Err(TrinityRamError::UnknownPropertyInBag { path: path.to_string(), key: key.clone() });
        }
    }
    Ok(())
}

fn property_value_matches_type_trinity(value: &PropertyValue, def: &crate::artifacts::jack::PropertyDef) -> bool {
    match value {
        PropertyValue::Null => def.value_type.id() == "null",
        PropertyValue::Bool(_) => def.value_type.id() == "boolean",
        PropertyValue::Number(_) => {
            let id = def.value_type.id();
            id == "decimal" || id == "integer" || id == "number"
        }
        PropertyValue::String(_) => {
            let id = def.value_type.id();
            id == "string" || id == "text"
        }
        PropertyValue::Object(_) => {
            let id = def.value_type.id();
            id.starts_with("schema:") || id == "object"
        }
        PropertyValue::Array(_) => def.value_type.id() == "array",
    }
}
//#endregion 🔖️Validation

//#region 🔖️BatchHelpers
/// ▶️ Diff-based apply of one mutation — thin `Mutation::diff` + `MutationDiff::apply` delegate (P6:
/// no per-variant hand match here anymore; each kind's real logic lives in its own triad `🔺️diff` leaf).
pub fn apply_trinity_graph_mutation(snapshot: &mut JackSnapshot, mutation: &TrinityGraphMutation) {
    let diff = mutation.diff(snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
}

pub fn inverse_trinity_graph_mutation(projection: &JackSnapshot, mutation: &TrinityGraphMutation) -> Vec<TrinityGraphMutation> {
    mutation.inverse(projection)
}

/// ▶️ Validates then applies a batch of operations, failing atomically on the first invalid one.
pub fn apply_trinity_graph_mutations(fixture: JackSnapshot, operations: &[TrinityGraphMutation]) -> Result<JackSnapshot, crate::artifacts::jack::TrinityRamError> {
    let mut snapshot = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        apply_trinity_graph_mutation(&mut snapshot, operation);
    }
    Ok(snapshot)
}

/// ▶️ Validates a batch incrementally, then dispatches it as one VCS edit.
pub fn dispatch_trinity_graph_mutations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphMutation>) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut snapshot = store.snapshot()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        apply_trinity_graph_mutation(&mut snapshot, operation);
    }
    store
        .dispatch(ArtifactCommand::Apply { mutations: operations, description: None })
        .map_err(crate::artifacts::jack::TrinityRamError::from)
        .map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use crate::artifacts::jack::{Camera, Manifest, PortDirection};
    use protocol::MutationDiff;

    fn mini_fixture() -> JackSnapshot {
        JackSnapshot {
            schema: JackSnapshot::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            edges: vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root@out-a".into(),
                target: "child@in-a".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.2));
                    p.insert("v".into(), PropertyValue::Number(-0.6));
                    p
                },
            }],
        }
    }

    fn mini_node(id: &str, x: f64, y: f64, ports: Vec<Port>) -> Node {
        Node { id: id.into(), kind: "Piece".into(), name: id.into(), x, y, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports }
    }

    #[test]
    fn graph_op_rejects_port_kind_not_declared_on_operation() {
        let mut fixture = mini_fixture();
        fixture.manifest = Manifest {
            node_kinds: vec![math::graph::manifest::TrinityNodeKindDef { name: "Piece".into(), properties: vec![], port_kinds: vec!["Connector".into()] }],
            edge_kinds: vec![math::graph::manifest::TrinityEdgeKindDef { name: "Connection".into(), properties: vec![] }],
            port_kinds: vec![
                math::graph::manifest::TrinityPortKindDef { name: "Connector".into(), direction: PortDirection::Out, properties: vec![] },
                math::graph::manifest::TrinityPortKindDef { name: "Other".into(), direction: PortDirection::In, properties: vec![] },
            ],
        };
        let op = create_node(mini_node("new", 0.0, 0.0, vec![Port { id: "p".into(), kind: "Other".into(), direction: PortDirection::In, properties: PropertyBag::new() }]));
        let err = validate_trinity_graph_operation(&op, &fixture).expect_err("bad port kind");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::PortKindNotDeclaredOnMutation { .. }));
    }

    #[test]
    fn graph_op_create_edge_rejects_invalid_port_keys() {
        let fixture = mini_fixture();
        let bad_source = create_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: "noAt".into(), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() });
        assert!(matches!(validate_trinity_graph_operation(&bad_source, &fixture), Err(crate::artifacts::jack::TrinityRamError::InvalidSourcePortKey(_))));
        let bad_target = create_edge(Edge { id: "e3".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: "noAt".into(), properties: PropertyBag::new() });
        assert!(matches!(validate_trinity_graph_operation(&bad_target, &fixture), Err(crate::artifacts::jack::TrinityRamError::InvalidTargetPortKey(_))));
    }

    #[test]
    fn graph_op_create_edge_rejects_missing_source_and_target_nodes() {
        let fixture = mini_fixture();
        let missing_source = create_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("ghost", "out"), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() });
        assert!(matches!(validate_trinity_graph_operation(&missing_source, &fixture), Err(crate::artifacts::jack::TrinityRamError::SourceNodeNotFound(_))));
        let missing_target = create_edge(Edge { id: "e3".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: crate::artifacts::jack::port_key("ghost", "in"), properties: PropertyBag::new() });
        assert!(matches!(validate_trinity_graph_operation(&missing_target, &fixture), Err(crate::artifacts::jack::TrinityRamError::TargetNodeNotFound(_))));
    }

    #[test]
    fn graph_op_rejects_duplicate_node_and_edge_ids() {
        let fixture = mini_fixture();
        let dup_node = create_node(mini_node("root", 0.0, 0.0, vec![]));
        assert!(matches!(validate_trinity_graph_operation(&dup_node, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeAlreadyExists(_))));
        let dup_edge = create_edge(Edge { id: "e1".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() });
        assert!(matches!(validate_trinity_graph_operation(&dup_edge, &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeAlreadyExists(_))));
    }

    #[test]
    fn graph_op_rejects_missing_entities_on_delete_rename_reposition() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&delete_node("ghost".into()), &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&delete_edge("ghost".into()), &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&rename_node("ghost".into(), "x".into()), &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&move_node("ghost".into(), 0.0, 0.0), &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_entity_kind() {
        let mut fixture = mini_fixture();
        fixture.nodes[0].kind = "Ghost".into();
        let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("x".into())), &fixture).expect_err("unknown entity kind");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::UnknownEntityKind { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_property_key() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "bogus".into(), PropertyValue::Null), &fixture).expect_err("unknown key");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::UnknownPropertyAtPath { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_type_mismatch() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::Number(1.0)), &fixture).expect_err("type mismatch");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::PropertyTypeMismatch { .. }));
    }

    #[test]
    fn graph_op_clear_data_property_rejects_missing_entities() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&remove_data_property(EntityRef::Node("ghost".into()), "label".into()), &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&remove_data_property(EntityRef::Edge("ghost".into()), "u".into()), &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeNotFound(_))));
    }

    #[test]
    fn apply_trinity_graph_mutations_applies_valid_sequence_and_rejects_invalid() {
        let fixture = mini_fixture();
        let ok = apply_trinity_graph_mutations(fixture.clone(), &[rename_node("root".into(), "renamed".into())]).expect("rename applies");
        assert_eq!(ok.nodes.iter().find(|n| n.id == "root").unwrap().name, "renamed");

        let err = apply_trinity_graph_mutations(fixture, &[delete_node("ghost".into())]).expect_err("missing node");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::NodeNotFound(_)));
    }

    #[test]
    fn document_text_round_trip_graph_store() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_mutations(&mut store, vec![rename_node("root".into(), "renamed".into())]).expect("apply");
        ::store::os_store::test_support::assert_document_text_round_trip(&store);
        ::store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn dispatch_trinity_graph_mutations_noop_on_empty() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        let generation_before = store.generation();
        dispatch_trinity_graph_mutations(&mut store, vec![]).expect("empty ops ok");
        assert_eq!(store.generation(), generation_before);
    }

    #[test]
    fn graph_op_reposition_and_rename_undo_restore_prior_values() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_mutations(&mut store, vec![move_node("root".into(), 50.0, 60.0)]).expect("reposition");
        assert_eq!(store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 50.0);
        store.dispatch(ArtifactCommand::Undo).expect("undo reposition");
        assert_eq!(store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 0.0);

        dispatch_trinity_graph_mutations(&mut store, vec![rename_node("root".into(), "renamed".into())]).expect("rename");
        store.dispatch(ArtifactCommand::Undo).expect("undo rename");
        assert_eq!(store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().name, "core");
    }

    #[test]
    fn graph_op_delete_edge_undo_recreates_edge() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_mutations(&mut store, vec![delete_edge("e1".into())]).expect("delete edge");
        assert!(store.snapshot().unwrap().edges.is_empty());
        store.dispatch(ArtifactCommand::Undo).expect("undo delete edge");
        assert_eq!(store.snapshot().unwrap().edges.len(), 1);
    }

    #[test]
    fn graph_op_delete_node_undo_restores_node_and_incident_edges() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_mutations(&mut store, vec![delete_node("root".into())]).expect("delete node");
        let projection = store.snapshot().unwrap();
        assert_eq!(projection.nodes.len(), 1);
        assert!(projection.edges.is_empty());
        store.dispatch(ArtifactCommand::Undo).expect("undo delete node");
        let projection = store.snapshot().unwrap();
        assert_eq!(projection.nodes.len(), 2);
        assert_eq!(projection.edges.len(), 1);
    }

    #[test]
    fn graph_op_set_and_clear_data_property_undo_round_trip() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_mutations(&mut store, vec![change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("first".into()))]).expect("set");
        dispatch_trinity_graph_mutations(&mut store, vec![change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("second".into()))]).expect("set again");
        store.dispatch(ArtifactCommand::Undo).expect("undo second set");
        let value = store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));

        dispatch_trinity_graph_mutations(&mut store, vec![remove_data_property(EntityRef::Node("root".into()), "label".into())]).expect("clear");
        assert!(!store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.contains_key("label"));
        store.dispatch(ArtifactCommand::Undo).expect("undo clear");
        let value = store.snapshot().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_trinity_graph_mutation_descriptors();
        for kind in <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds().len(), 8);
    }
}
//#endregion 🧪️Tests
