//! ⚙️ Jack artifact mutation validation, application, inversion and store behavior.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{
    change_data_property, create_edge, create_node, delete_edge, delete_node, move_node, register_trinity_graph_mutation_descriptors, remove_data_property, rename_node, CreateEdge, DeleteNode, RenameNode,
};
use crate::{EntityRef, JackSnapshot, PropertyBag, PropertyValue, TRINITY_GRAPH_SCHEMA};
#[cfg(test)]
use crate::{Edge, Node, Port};
use protocol::Mutation;
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

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
pub fn validate_trinity_graph_operation(operation: &TrinityGraphMutation, fixture: &JackSnapshot) -> Result<(), crate::TrinityRamError> {
    use crate::TrinityRamError;
    let scene = crate::jack_working_scene(fixture);
    match operation {
        TrinityGraphMutation::CreateNode(payload) => {
            let node = &payload.node;
            if scene.nodes.iter().any(|existing| existing.id == node.id) {
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
            if !scene.nodes.iter().any(|node| node.id == payload.id) {
                return Err(TrinityRamError::NodeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::CreateEdge(payload) => {
            let edge = &payload.edge;
            if scene.edges.iter().any(|existing| existing.id == edge.id) {
                return Err(TrinityRamError::EdgeAlreadyExists(edge.id.clone()));
            }
            validate_edge_kind_trinity(&fixture.manifest, &edge.kind)?;
            validate_edge_properties_trinity(&fixture.manifest, &edge.kind, &edge.properties)?;
            let source_node = crate::port_node_id(&edge.source).ok_or_else(|| TrinityRamError::InvalidSourcePortKey(edge.source.clone()))?;
            let target_node = crate::port_node_id(&edge.target).ok_or_else(|| TrinityRamError::InvalidTargetPortKey(edge.target.clone()))?;
            if !scene.nodes.iter().any(|node| node.id == source_node) {
                return Err(TrinityRamError::SourceNodeNotFound(source_node.to_string()));
            }
            if !scene.nodes.iter().any(|node| node.id == target_node) {
                return Err(TrinityRamError::TargetNodeNotFound(target_node.to_string()));
            }
        }
        TrinityGraphMutation::DeleteEdge(payload) => {
            if !scene.edges.iter().any(|edge| edge.id == payload.id) {
                return Err(TrinityRamError::EdgeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::RenameNode(payload) => {
            if !scene.nodes.iter().any(|node| node.id == payload.id) {
                return Err(TrinityRamError::NodeNotFound(payload.id.clone()));
            }
        }
        TrinityGraphMutation::MoveNode(payload) => {
            if !scene.nodes.iter().any(|node| node.id == payload.id) {
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

fn validate_clear_data_property(fixture: &JackSnapshot, entity: &EntityRef, key: &str) -> Result<(), crate::TrinityRamError> {
    use crate::TrinityRamError;
    let scene = crate::jack_working_scene(fixture);
    match entity {
        EntityRef::Node(id) => {
            scene.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
        }
        EntityRef::Edge(id) => {
            scene.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
        }
    }
    let _ = key;
    Ok(())
}

fn validate_set_data_property(fixture: &JackSnapshot, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<(), crate::TrinityRamError> {
    use crate::TrinityRamError;
    let scene = crate::jack_working_scene(fixture);
    let (defs, path_prefix) = match entity {
        EntityRef::Node(id) => {
            let node = scene.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
            (fixture.manifest.node_kind(&node.kind).map(|def| &def.properties[..]), format!("nodes/{id}/properties/{key}"))
        }
        EntityRef::Edge(id) => {
            let edge = scene.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
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

fn validate_node_kind_trinity(manifest: &crate::Manifest, kind: &str) -> Result<(), crate::TrinityRamError> {
    if manifest.node_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::TrinityRamError::UnknownNodeKind { kind: kind.to_string() })
    }
}

fn validate_edge_kind_trinity(manifest: &crate::Manifest, kind: &str) -> Result<(), crate::TrinityRamError> {
    if manifest.edge_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::TrinityRamError::UnknownEdgeKind { kind: kind.to_string() })
    }
}

fn validate_port_kind_trinity(manifest: &crate::Manifest, kind: &str) -> Result<(), crate::TrinityRamError> {
    if manifest.port_kind(kind).is_some() {
        Ok(())
    } else {
        Err(crate::TrinityRamError::UnknownPortKind { kind: kind.to_string() })
    }
}

fn validate_edge_properties_trinity(manifest: &crate::Manifest, kind: &str, properties: &PropertyBag) -> Result<(), crate::TrinityRamError> {
    let Some(def) = manifest.edge_kind(kind) else {
        return validate_edge_kind_trinity(manifest, kind);
    };
    validate_property_bag_trinity(&format!("edges/{kind}/properties"), &def.properties, properties)
}

fn validate_property_bag_trinity(path: &str, defs: &[crate::PropertyDef], bag: &PropertyBag) -> Result<(), crate::TrinityRamError> {
    use crate::{PropertyKind, TrinityRamError};
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

fn property_value_matches_type_trinity(value: &PropertyValue, def: &crate::PropertyDef) -> bool {
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
/// no per-variant hand match here anymore; each kind's real logic lives in its optional `🔺️diff` facet).
pub fn apply_trinity_graph_mutation(snapshot: &mut JackSnapshot, mutation: &TrinityGraphMutation) -> protocol::MutationApplyResult<()> {
    let outcome = mutation.diff(snapshot);
    let next = protocol::MutationDiff::apply(outcome.diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_trinity_graph_mutation(projection: &JackSnapshot, mutation: &TrinityGraphMutation) -> Vec<TrinityGraphMutation> {
    mutation.inverse(projection)
}

/// ▶️ Validates then applies a batch of operations, failing atomically on the first invalid one.
pub fn apply_trinity_graph_mutations(fixture: JackSnapshot, operations: &[TrinityGraphMutation]) -> Result<JackSnapshot, crate::TrinityRamError> {
    let mut snapshot = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        apply_trinity_graph_mutation(&mut snapshot, operation)?;
    }
    Ok(snapshot)
}

/// ▶️ Validates a batch incrementally, then dispatches it as one VCS edit.
pub async fn dispatch_trinity_graph_mutations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphMutation>) -> Result<(), crate::TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut snapshot = store.snapshot()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        apply_trinity_graph_mutation(&mut snapshot, operation)?;
    }
    store.dispatch(ArtifactCommand::Apply { mutations: operations, description: None }).await.map_err(crate::TrinityRamError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
