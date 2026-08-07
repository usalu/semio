//! ⚡️ `trinity.graph` artifact — operation enum + apply/backwards laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::jack::diff::{NodeGeometryPatch, PropertyPatch, TrinityGraphDiff};
use crate::artifacts::jack::{EntityRef, GraphFixture, Node, Port, PropertyBag, PropertyValue, TRINITY_GRAPH_SCHEMA};
use protocol::Operation;
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};
use vcs::{apply_operation, CollectionDiff, ItemPatch};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum TrinityGraphOperation {
    CreateNode {
        id: String,
        kind: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        ports: Vec<Port>,
    },
    DeleteNode {
        id: String,
    },
    CreateEdge {
        id: String,
        kind: String,
        source: String,
        target: String,
        properties: PropertyBag,
    },
    DeleteEdge {
        id: String,
    },
    Rename {
        id: String,
        name: String,
    },
    Reposition {
        id: String,
        x: f64,
        y: f64,
    },
    SetDataProperty {
        entity: EntityRef,
        key: String,
        value: PropertyValue,
    },
    ClearDataProperty {
        entity: EntityRef,
        key: String,
    },
    /// 📦️ Replace the whole fixture (preset load, node-graph drag import); the inverse restores the prior fixture.
    SetFixture {
        fixture: GraphFixture,
    },
}

pub type TrinityGraphEnvelope = DocumentEnvelope<GraphFixture, TrinityGraphOperation>;
pub type TrinityGraphStore = DocumentStore<GraphFixture, TrinityGraphOperation>;

pub fn create_trinity_graph_envelope(id: &str, fixture: GraphFixture) -> TrinityGraphEnvelope {
    create_document_envelope(TRINITY_GRAPH_SCHEMA, id, fixture, None)
}

pub fn validate_trinity_graph_operation(operation: &TrinityGraphOperation, fixture: &GraphFixture) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::TrinityRamError;
    match operation {
        TrinityGraphOperation::CreateNode { id, kind, ports, .. } => {
            if fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeAlreadyExists(id.clone()));
            }
            validate_node_kind_trinity(&fixture.manifest, kind)?;
            if let Some(node_def) = fixture.manifest.node_kind(kind) {
                for port in ports {
                    validate_port_kind_trinity(&fixture.manifest, &port.kind)?;
                    if !node_def.port_kinds.is_empty() && !node_def.port_kinds.iter().any(|p| p == &port.kind) {
                        return Err(TrinityRamError::PortKindNotDeclaredOnOperation { node_id: id.clone(), port_id: port.id.clone(), port_kind: port.kind.clone(), node_kind: kind.clone() });
                    }
                }
            }
        }
        TrinityGraphOperation::DeleteNode { id } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => {
            if fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(TrinityRamError::EdgeAlreadyExists(id.clone()));
            }
            validate_edge_kind_trinity(&fixture.manifest, kind)?;
            validate_edge_properties_trinity(&fixture.manifest, kind, properties)?;
            let source_node = crate::artifacts::jack::port_node_id(source).ok_or_else(|| TrinityRamError::InvalidSourcePortKey(source.clone()))?;
            let target_node = crate::artifacts::jack::port_node_id(target).ok_or_else(|| TrinityRamError::InvalidTargetPortKey(target.clone()))?;
            if !fixture.nodes.iter().any(|node| node.id == source_node) {
                return Err(TrinityRamError::SourceNodeNotFound(source_node.to_string()));
            }
            if !fixture.nodes.iter().any(|node| node.id == target_node) {
                return Err(TrinityRamError::TargetNodeNotFound(target_node.to_string()));
            }
        }
        TrinityGraphOperation::DeleteEdge { id } => {
            if !fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(TrinityRamError::EdgeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::Rename { id, .. } | TrinityGraphOperation::Reposition { id, .. } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::SetDataProperty { entity, key, value } => {
            validate_set_data_property(fixture, entity, key, value)?;
        }
        TrinityGraphOperation::ClearDataProperty { entity, key } => {
            validate_clear_data_property(fixture, entity, key)?;
        }
        TrinityGraphOperation::SetFixture { .. } => {}
    }
    Ok(())
}

pub fn apply_trinity_graph_operations(fixture: GraphFixture, operations: &[TrinityGraphOperation]) -> Result<GraphFixture, crate::artifacts::jack::TrinityRamError> {
    let mut projection = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &projection)?;
        projection = apply_operation(&projection, operation);
    }
    Ok(projection)
}

pub fn dispatch_trinity_graph_operations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphOperation>) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut projection = store.projection()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &projection)?;
        projection = apply_operation(&projection, operation);
    }
    store.dispatch(DocumentCommand::Apply { operations, description: None }).map_err(crate::artifacts::jack::TrinityRamError::from)
}

fn validate_clear_data_property(fixture: &GraphFixture, entity: &EntityRef, key: &str) -> Result<(), crate::artifacts::jack::TrinityRamError> {
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

fn validate_set_data_property(fixture: &GraphFixture, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    use crate::artifacts::jack::{PropertyKind, TrinityRamError};
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
    let Some(def) = defs.iter().find(|def| def.name == key) else {
        return Err(TrinityRamError::UnknownPropertyAtPath { path: path_prefix, key: key.to_string() });
    };
    if def.kind == PropertyKind::Derived {
        return Err(TrinityRamError::DerivedPropertyReadonly { path: path_prefix, key: key.to_string() });
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

fn delete_node_snapshot(fixture: &GraphFixture, id: &str) -> (Option<Node>, Vec<crate::artifacts::jack::Edge>) {
    let node = fixture.nodes.iter().find(|node| node.id == id).cloned();
    let edges: Vec<crate::artifacts::jack::Edge> = fixture.edges.iter().filter(|edge| crate::artifacts::jack::port_node_id(&edge.source) == Some(id) || crate::artifacts::jack::port_node_id(&edge.target) == Some(id)).cloned().collect();
    (node, edges)
}

fn entity_property_value(fixture: &GraphFixture, entity: &EntityRef, key: &str) -> Option<PropertyValue> {
    match entity {
        EntityRef::Node(id) => fixture.nodes.iter().find(|node| node.id == *id).and_then(|node| node.properties.get(key).cloned()),
        EntityRef::Edge(id) => fixture.edges.iter().find(|edge| edge.id == *id).and_then(|edge| edge.properties.get(key).cloned()),
    }
}

impl Operation<GraphFixture> for TrinityGraphOperation {
    type Diff = TrinityGraphDiff;

    fn diff(&self, projection: &GraphFixture) -> TrinityGraphDiff {
        match self {
            TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports } => TrinityGraphDiff {
                nodes: CollectionDiff { added: vec![Node { id: id.clone(), kind: kind.clone(), name: name.clone(), x: *x, y: *y, width: *width, height: *height, properties: PropertyBag::new(), ports: ports.clone() }], ..Default::default() },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOperation::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                TrinityGraphDiff {
                    nodes: CollectionDiff { removed: node.as_ref().map(|node| vec![node.id.clone()]).unwrap_or_default(), ..Default::default() },
                    edges: CollectionDiff { removed: edges.iter().map(|edge| edge.id.clone()).collect(), ..Default::default() },
                    recompute_derived: true,
                    ..Default::default()
                }
            }
            TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => TrinityGraphDiff {
                edges: CollectionDiff { added: vec![crate::artifacts::jack::Edge { id: id.clone(), kind: kind.clone(), source: source.clone(), target: target.clone(), properties: properties.clone() }], ..Default::default() },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOperation::DeleteEdge { id } => TrinityGraphDiff { edges: CollectionDiff { removed: vec![id.clone()], ..Default::default() }, recompute_derived: true, ..Default::default() },
            TrinityGraphOperation::Rename { id, name } => {
                TrinityGraphDiff { nodes: CollectionDiff { modified: vec![ItemPatch { id: id.clone(), patch: NodeGeometryPatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() }
            }
            TrinityGraphOperation::Reposition { id, x, y } => {
                TrinityGraphDiff { nodes: CollectionDiff { modified: vec![ItemPatch { id: id.clone(), patch: NodeGeometryPatch { x: Some(*x), y: Some(*y), ..Default::default() } }], ..Default::default() }, ..Default::default() }
            }
            TrinityGraphOperation::SetDataProperty { entity, key, value } => {
                let patch = PropertyPatch { key: key.clone(), value: Some(value.clone()) };
                let recompute = matches!(entity, EntityRef::Edge(_)) && (key == "u" || key == "v");
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff { node_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: key == "flatPosition", ..Default::default() },
                    EntityRef::Edge(id) => TrinityGraphDiff { edge_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: recompute, ..Default::default() },
                }
            }
            TrinityGraphOperation::ClearDataProperty { entity, key } => {
                let patch = PropertyPatch { key: key.clone(), value: None };
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff { node_properties: vec![ItemPatch { id: id.clone(), patch }], ..Default::default() },
                    EntityRef::Edge(id) => TrinityGraphDiff { edge_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: key == "u" || key == "v", ..Default::default() },
                }
            }
            TrinityGraphOperation::SetFixture { fixture } => TrinityGraphDiff { set_fixture: Some(fixture.clone()), recompute_derived: true, ..Default::default() },
        }
    }

    fn backwards(&self, projection: &GraphFixture) -> Vec<Self> {
        match self {
            TrinityGraphOperation::CreateNode { id, .. } => vec![TrinityGraphOperation::DeleteNode { id: id.clone() }],
            TrinityGraphOperation::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                let mut out = Vec::new();
                if let Some(node) = node {
                    out.push(TrinityGraphOperation::CreateNode { id: node.id, kind: node.kind, name: node.name, x: node.x, y: node.y, width: node.width, height: node.height, ports: node.ports });
                    for edge in edges {
                        out.push(TrinityGraphOperation::CreateEdge { id: edge.id, kind: edge.kind, source: edge.source, target: edge.target, properties: edge.properties });
                    }
                }
                out
            }
            TrinityGraphOperation::CreateEdge { id, .. } => vec![TrinityGraphOperation::DeleteEdge { id: id.clone() }],
            TrinityGraphOperation::DeleteEdge { id } => projection
                .edges
                .iter()
                .find(|edge| edge.id == *id)
                .map(|edge| vec![TrinityGraphOperation::CreateEdge { id: edge.id.clone(), kind: edge.kind.clone(), source: edge.source.clone(), target: edge.target.clone(), properties: edge.properties.clone() }])
                .unwrap_or_default(),
            TrinityGraphOperation::Rename { id, .. } => projection.nodes.iter().find(|node| node.id == *id).map(|node| vec![TrinityGraphOperation::Rename { id: id.clone(), name: node.name.clone() }]).unwrap_or_default(),
            TrinityGraphOperation::Reposition { id, .. } => projection.nodes.iter().find(|node| node.id == *id).map(|node| vec![TrinityGraphOperation::Reposition { id: id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
            TrinityGraphOperation::SetDataProperty { entity, key, .. } => {
                let prior = entity_property_value(projection, entity, key);
                match (entity, prior) {
                    (EntityRef::Node(id), Some(old)) => vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node(id.clone()), key: key.clone(), value: old }],
                    (EntityRef::Edge(id), Some(old)) => vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Edge(id.clone()), key: key.clone(), value: old }],
                    (entity, None) => vec![TrinityGraphOperation::ClearDataProperty { entity: entity.clone(), key: key.clone() }],
                }
            }
            TrinityGraphOperation::ClearDataProperty { entity, key } => {
                entity_property_value(projection, entity, key).map(|old| vec![TrinityGraphOperation::SetDataProperty { entity: entity.clone(), key: key.clone(), value: old }]).unwrap_or_default()
            }
            TrinityGraphOperation::SetFixture { .. } => vec![TrinityGraphOperation::SetFixture { fixture: projection.clone() }],
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::{Camera, Manifest, PortDirection};
    use protocol::OperationDiff;

    fn mini_fixture() -> GraphFixture {
        GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
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
            edges: vec![crate::artifacts::jack::Edge {
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
        let op = TrinityGraphOperation::CreateNode {
            id: "new".into(),
            kind: "Piece".into(),
            name: "x".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 40.0,
            ports: vec![Port { id: "p".into(), kind: "Other".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
        };
        let err = validate_trinity_graph_operation(&op, &fixture).expect_err("bad port kind");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::PortKindNotDeclaredOnOperation { .. }));
    }

    #[test]
    fn graph_op_create_edge_rejects_invalid_port_keys() {
        let fixture = mini_fixture();
        let bad_source = TrinityGraphOperation::CreateEdge { id: "e2".into(), kind: "Connection".into(), source: "noAt".into(), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&bad_source, &fixture), Err(crate::artifacts::jack::TrinityRamError::InvalidSourcePortKey(_))));
        let bad_target = TrinityGraphOperation::CreateEdge { id: "e3".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: "noAt".into(), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&bad_target, &fixture), Err(crate::artifacts::jack::TrinityRamError::InvalidTargetPortKey(_))));
    }

    #[test]
    fn graph_op_create_edge_rejects_missing_source_and_target_nodes() {
        let fixture = mini_fixture();
        let missing_source = TrinityGraphOperation::CreateEdge { id: "e2".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("ghost", "out"), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&missing_source, &fixture), Err(crate::artifacts::jack::TrinityRamError::SourceNodeNotFound(_))));
        let missing_target = TrinityGraphOperation::CreateEdge { id: "e3".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: crate::artifacts::jack::port_key("ghost", "in"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&missing_target, &fixture), Err(crate::artifacts::jack::TrinityRamError::TargetNodeNotFound(_))));
    }

    #[test]
    fn graph_op_rejects_duplicate_node_and_edge_ids() {
        let fixture = mini_fixture();
        let dup_node = TrinityGraphOperation::CreateNode { id: "root".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, ports: vec![] };
        assert!(matches!(validate_trinity_graph_operation(&dup_node, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeAlreadyExists(_))));
        let dup_edge = TrinityGraphOperation::CreateEdge { id: "e1".into(), kind: "Connection".into(), source: crate::artifacts::jack::port_key("root", "out-a"), target: crate::artifacts::jack::port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&dup_edge, &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeAlreadyExists(_))));
    }

    #[test]
    fn graph_op_rejects_missing_entities_on_delete_rename_reposition() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::DeleteNode { id: "ghost".into() }, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::DeleteEdge { id: "ghost".into() }, &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::Rename { id: "ghost".into(), name: "x".into() }, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::Reposition { id: "ghost".into(), x: 0.0, y: 0.0 }, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_entity_kind() {
        let mut fixture = mini_fixture();
        fixture.nodes[0].kind = "Ghost".into();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("x".into()) }, &fixture).expect_err("unknown entity kind");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::UnknownEntityKind { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_property_key() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "bogus".into(), value: PropertyValue::Null }, &fixture).expect_err("unknown key");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::UnknownPropertyAtPath { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_type_mismatch() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::Number(1.0) }, &fixture).expect_err("type mismatch");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::PropertyTypeMismatch { .. }));
    }

    #[test]
    fn graph_op_clear_data_property_rejects_missing_entities() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Node("ghost".into()), key: "label".into() }, &fixture), Err(crate::artifacts::jack::TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Edge("ghost".into()), key: "u".into() }, &fixture), Err(crate::artifacts::jack::TrinityRamError::EdgeNotFound(_))));
    }

    #[test]
    fn apply_trinity_graph_operations_applies_valid_sequence_and_rejects_invalid() {
        let fixture = mini_fixture();
        let ok = apply_trinity_graph_operations(fixture.clone(), &[TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("rename applies");
        assert_eq!(ok.nodes.iter().find(|n| n.id == "root").unwrap().name, "renamed");

        let err = apply_trinity_graph_operations(fixture, &[TrinityGraphOperation::DeleteNode { id: "ghost".into() }]).expect_err("missing node");
        assert!(matches!(err, crate::artifacts::jack::TrinityRamError::NodeNotFound(_)));
    }

    #[test]
    fn document_text_round_trip_graph_store() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn dispatch_trinity_graph_operations_noop_on_empty() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        let generation_before = store.generation();
        dispatch_trinity_graph_operations(&mut store, vec![]).expect("empty ops ok");
        assert_eq!(store.generation(), generation_before);
    }

    #[test]
    fn graph_op_reposition_and_rename_undo_restore_prior_values() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Reposition { id: "root".into(), x: 50.0, y: 60.0 }]).expect("reposition");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 50.0);
        store.dispatch(DocumentCommand::Undo).expect("undo reposition");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 0.0);

        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("rename");
        store.dispatch(DocumentCommand::Undo).expect("undo rename");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().name, "core");
    }

    #[test]
    fn graph_op_delete_edge_undo_recreates_edge() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::DeleteEdge { id: "e1".into() }]).expect("delete edge");
        assert!(store.projection().unwrap().edges.is_empty());
        store.dispatch(DocumentCommand::Undo).expect("undo delete edge");
        assert_eq!(store.projection().unwrap().edges.len(), 1);
    }

    #[test]
    fn graph_op_delete_node_undo_restores_node_and_incident_edges() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::DeleteNode { id: "root".into() }]).expect("delete node");
        let projection = store.projection().unwrap();
        assert_eq!(projection.nodes.len(), 1);
        assert!(projection.edges.is_empty());
        store.dispatch(DocumentCommand::Undo).expect("undo delete node");
        let projection = store.projection().unwrap();
        assert_eq!(projection.nodes.len(), 2);
        assert_eq!(projection.edges.len(), 1);
    }

    #[test]
    fn graph_op_set_and_clear_data_property_undo_round_trip() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("first".into()) }]).expect("set");
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("second".into()) }]).expect("set again");
        store.dispatch(DocumentCommand::Undo).expect("undo second set");
        let value = store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));

        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Node("root".into()), key: "label".into() }]).expect("clear");
        assert!(!store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.contains_key("label"));
        store.dispatch(DocumentCommand::Undo).expect("undo clear");
        let value = store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));
    }

    /// 🌱️ `camera` is now a seed-only field on `GraphFixture` (never touched by any operation — see
    /// `nodeGraphViewport`'s runtime-only handling in the jack/rewrite apps), so this only exercises
    /// `SetFixture`'s undo; it no longer asserts camera-as-a-document-operation behavior.
    #[test]
    fn graph_op_set_fixture_undo() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        assert_eq!(store.projection().unwrap().camera, Camera::default());

        let replacement = GraphFixture { name: "replacement".into(), ..mini_fixture() };
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetFixture { fixture: replacement }]).expect("set fixture");
        assert_eq!(store.projection().unwrap().name, "replacement");
        store.dispatch(DocumentCommand::Undo).expect("undo set fixture");
        assert_eq!(store.projection().unwrap().name, "mini");
    }

    #[test]
    fn trinity_graph_diff_apply_uses_set_fixture_as_base_and_recomputes() {
        let base = mini_fixture();
        let mut replacement = base.clone();
        replacement.name = "swapped".into();
        let diff = TrinityGraphDiff { set_fixture: Some(replacement), recompute_derived: true, ..Default::default() };
        let applied = diff.apply(&base);
        assert_eq!(applied.name, "swapped");
        assert!(applied.nodes.iter().any(|n| n.properties.contains_key("flatPosition")));
    }
}
//#endregion 🧪️Tests
