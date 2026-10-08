//! ⚙️ Jack artifact mutation validation, application, inversion and store behavior.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::{EntityRef, Graph, GraphEffect, JackSnapshot, PropertyBag, PropertyValue, TRINITY_GRAPH_SCHEMA};
use protocol::Mutation;
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type TrinityGraphEnvelope = ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>;
pub type TrinityGraphStore = ArtifactStore<JackSnapshot, TrinityGraphMutation>;

pub fn create_trinity_graph_envelope(id: &str, snapshot: JackSnapshot) -> TrinityGraphEnvelope {
    create_document_envelope(TRINITY_GRAPH_SCHEMA, id, snapshot, None)
}

/// 🔐️ Opens a Jack store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `TrinityGraphStore::new`
/// can be read but never mutated, undone or closed. The editor app installs the same catalog
/// through `build_document_store_owners`; every standalone store (tests, the rewriting bridge)
/// goes through here instead.


/// 🔚 A standalone Jack store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedTrinityGraphStore(pub(crate) TrinityGraphStore);

impl OwnedTrinityGraphStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Jack document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedTrinityGraphStore {
    type Target = TrinityGraphStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedTrinityGraphStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedTrinityGraphStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🔖️Validation
/// 🛡️ Pre-flight validation of one parent-lane operation against `snapshot` — the query must fit its document bound.
pub fn validate_trinity_graph_operation(operation: &TrinityGraphMutation, _snapshot: &JackSnapshot) -> Result<(), crate::TrinityRamError> {
    let TrinityGraphMutation::SetQuery(payload) = operation;
    if payload.value.len() > crate::JACK_QUERY_MAXIMUM_BYTES {
        return Err(crate::TrinityRamError::QueryTooLarge { bytes: payload.value.len(), maximum: crate::JACK_QUERY_MAXIMUM_BYTES });
    }
    Ok(())
}

/// 🛡️ Manifest/reference validation of one query effect against the working `graph` — cross-checks the compile-time
/// `Manifest`, so a query never publishes a content leaf of an undeclared kind or a dangling endpoint.
pub fn validate_graph_effect(effect: &GraphEffect, graph: &Graph) -> Result<(), crate::TrinityRamError> {
    use crate::TrinityRamError;
    match effect {
        GraphEffect::CreateNode(node) => {
            if graph.nodes.contains_key(&node.id) {
                return Err(TrinityRamError::NodeAlreadyExists(node.id.clone()));
            }
            validate_node_kind_trinity(&graph.manifest, &node.kind)?;
            if let Some(node_def) = graph.manifest.node_kind(&node.kind) {
                for port in &node.ports {
                    validate_port_kind_trinity(&graph.manifest, &port.kind)?;
                    if !node_def.port_kinds.is_empty() && !node_def.port_kinds.iter().any(|p| p == &port.kind) {
                        return Err(TrinityRamError::PortKindNotDeclaredOnMutation { node_id: node.id.clone(), port_id: port.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                    }
                }
            }
        }
        GraphEffect::DeleteNode(id) | GraphEffect::RenameNode { id, .. } | GraphEffect::MoveNode { id, .. } => {
            if !graph.nodes.contains_key(id) {
                return Err(TrinityRamError::NodeNotFound(id.clone()));
            }
        }
        GraphEffect::CreateEdge(edge) => {
            if graph.edges.contains_key(&edge.id) {
                return Err(TrinityRamError::EdgeAlreadyExists(edge.id.clone()));
            }
            validate_edge_kind_trinity(&graph.manifest, &edge.kind)?;
            validate_edge_properties_trinity(&graph.manifest, &edge.kind, &edge.properties)?;
            let source_node = crate::port_node_id(&edge.source).ok_or_else(|| TrinityRamError::InvalidSourcePortKey(edge.source.clone()))?;
            let target_node = crate::port_node_id(&edge.target).ok_or_else(|| TrinityRamError::InvalidTargetPortKey(edge.target.clone()))?;
            if !graph.nodes.contains_key(source_node) {
                return Err(TrinityRamError::SourceNodeNotFound(source_node.to_string()));
            }
            if !graph.nodes.contains_key(target_node) {
                return Err(TrinityRamError::TargetNodeNotFound(target_node.to_string()));
            }
        }
        GraphEffect::DeleteEdge(id) => {
            if !graph.edges.contains_key(id) {
                return Err(TrinityRamError::EdgeNotFound(id.clone()));
            }
        }
        GraphEffect::SetProperty { entity, key, value } => validate_set_data_property(graph, entity, key, value)?,
        GraphEffect::RemoveProperty { entity, .. } => validate_entity(graph, entity)?,
    }
    Ok(())
}

/// ▶️ Validates then applies query effects to `graph`, failing atomically on the first invalid one.
pub fn apply_graph_effects(graph: &mut Graph, effects: &[GraphEffect]) -> Result<(), crate::TrinityRamError> {
    for effect in effects {
        validate_graph_effect(effect, graph)?;
        match effect {
            GraphEffect::CreateNode(node) => graph.add_node(node.clone()),
            GraphEffect::DeleteNode(id) => {
                graph.remove_node(id);
            }
            GraphEffect::CreateEdge(edge) => graph.add_edge(edge.clone()),
            GraphEffect::DeleteEdge(id) => {
                graph.remove_edge(id);
            }
            GraphEffect::RenameNode { id, name } => graph.node_mut(id).expect("validated node").name = name.clone(),
            GraphEffect::MoveNode { id, x, y } => {
                let node = graph.node_mut(id).expect("validated node");
                node.x = *x;
                node.y = *y;
            }
            GraphEffect::SetProperty { entity, key, value } => graph.set_property(entity.clone(), key, value.clone())?,
            GraphEffect::RemoveProperty { entity, key } => {
                let bag = match entity {
                    EntityRef::Node(id) => graph.nodes.get_mut(id).map(|node| &mut node.properties),
                    EntityRef::Edge(id) => graph.edges.get_mut(id).map(|edge| &mut edge.properties),
                };
                bag.expect("validated entity").remove(key);
            }
        }
    }
    Ok(())
}

fn validate_entity(graph: &Graph, entity: &EntityRef) -> Result<(), crate::TrinityRamError> {
    match entity {
        EntityRef::Node(id) if !graph.nodes.contains_key(id) => Err(crate::TrinityRamError::NodeNotFound(id.clone())),
        EntityRef::Edge(id) if !graph.edges.contains_key(id) => Err(crate::TrinityRamError::EdgeNotFound(id.clone())),
        _ => Ok(()),
    }
}

fn validate_set_data_property(graph: &Graph, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<(), crate::TrinityRamError> {
    use crate::TrinityRamError;
    let (defs, path_prefix) = match entity {
        EntityRef::Node(id) => {
            let node = graph.nodes.get(id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
            (graph.manifest.node_kind(&node.kind).map(|def| &def.properties[..]), format!("nodes/{id}/properties/{key}"))
        }
        EntityRef::Edge(id) => {
            let edge = graph.edges.get(id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
            (graph.manifest.edge_kind(&edge.kind).map(|def| &def.properties[..]), format!("edges/{id}/properties/{key}"))
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
pub use crate::apply_trinity_graph_mutation;

pub fn inverse_trinity_graph_mutation(projection: &JackSnapshot, mutation: &TrinityGraphMutation) -> Result<Vec<TrinityGraphMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

/// ▶️ Validates then applies a batch of operations, failing atomically on the first invalid one.
pub fn apply_trinity_graph_mutations(snapshot: JackSnapshot, operations: &[TrinityGraphMutation]) -> Result<JackSnapshot, crate::TrinityRamError> {
    let mut snapshot = snapshot;
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
    store.dispatch(ArtifactCommand::Apply { mutations: operations, transaction: None }).await.map_err(crate::TrinityRamError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
