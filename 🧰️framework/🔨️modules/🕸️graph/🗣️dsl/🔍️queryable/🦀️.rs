//! 🔍️ Queryable graph interface for Jack.

// #region queryable

use crate::GraphDslError;
use semio_framework_graph_manifest::{GraphManifest, PropertyBag, PropertyValue};
use semio_framework_pack_json::Value;
use std::collections::{BTreeMap, BTreeSet};

// #region 🔖️QueryableEdge
/// 🪢️ Edge row exposed to Jack matching.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryableEdge {
    pub id: String,
    pub kind: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub source_port: Option<String>,
    pub target_port: Option<String>,
    pub properties: PropertyBag,
}
// #endregion 🔖️QueryableEdge

// #region 🔖️QueryableGraph
/// 🕸️ Read-only graph surface for Jack query execution.
pub trait QueryableGraph {
    fn manifest(&self) -> Option<&GraphManifest>;
    fn node_ids(&self) -> Vec<String>;
    fn node_kind(&self, id: &str) -> Option<String>;
    fn node_name(&self, id: &str) -> Option<String>;
    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue>;
    fn edges(&self) -> Vec<QueryableEdge>;
    fn subgraph_snapshot_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String>;
}

pub fn manifest_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for id in graph.node_ids() {
        if let Some(kind) = graph.node_kind(id.as_str()) {
            kinds.insert(kind);
        }
    }
    if let Some(manifest) = graph.manifest() {
        for def in &manifest.node_kinds {
            kinds.insert(def.id.clone());
        }
    }
    kinds.into_iter().collect()
}

pub fn manifest_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for edge in graph.edges() {
        kinds.insert(edge.kind.clone());
    }
    if let Some(manifest) = graph.manifest() {
        for def in &manifest.edge_kinds {
            kinds.insert(def.id.clone());
        }
    }
    kinds.into_iter().collect()
}

pub fn manifest_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
    let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
    for id in graph.node_ids() {
        for key in ["label", "text"] {
            if graph.node_property(id.as_str(), key).is_some() {
                props.insert(key.to_string());
            }
        }
        if let Some(PropertyValue::Object(map)) = graph.node_property(id.as_str(), "__all") {
            for key in map.keys() {
                props.insert(key.clone());
            }
        }
    }
    props.into_iter().collect()
}

pub fn manifest_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for edge in graph.edges() {
        if let Some(port) = &edge.source_port {
            kinds.insert(port.clone());
        }
        if let Some(port) = &edge.target_port {
            kinds.insert(port.clone());
        }
    }
    if let Some(manifest) = graph.manifest() {
        for def in &manifest.port_kinds {
            kinds.insert(def.id.clone());
        }
    }
    kinds.into_iter().collect()
}
// #endregion 🔖️QueryableGraph

// #region 🔖️BoardQueryableGraph
fn json_to_property_bag(value: &Value) -> PropertyBag {
    let semio_framework_value::DslValue::Object(entries) = semio_framework_pack_json::to_dsl_value(value) else {
        return PropertyBag::default();
    };
    entries.into_iter().filter_map(|(k, v)| semio_framework_value::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
}

fn split_endpoint(endpoint: &str, handle_to_node: &BTreeMap<String, String>) -> (String, Option<String>) {
    if let Some(node_id) = handle_to_node.get(endpoint) {
        return (node_id.clone(), None);
    }
    if let Some((node, port)) = endpoint.split_once('@') {
        let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
        return (node_id, Some(port.to_string()));
    }
    if let Some((node, port)) = endpoint.rsplit_once(':') {
        let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
        return (node_id, Some(port.to_string()));
    }
    if let Some((node, port)) = endpoint.rsplit_once('.') {
        if handle_to_node.contains_key(endpoint) {
            return (handle_to_node[endpoint].clone(), None);
        }
        return (node.to_string(), Some(port.to_string()));
    }
    (endpoint.to_string(), None)
}

/// 🧩️ Jack query target over board/scene fixture JSON.
pub struct BoardQueryableGraph {
    manifest: Option<GraphManifest>,
    nodes: BTreeMap<String, (String, String, PropertyBag)>,
    edges: Vec<QueryableEdge>,
    raw_fixture: Value,
}

impl BoardQueryableGraph {
    pub fn from_host_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
        let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        if let (Some(manifest), Some(id)) = (&manifest, raw.get("manifestId").and_then(Value::as_str)) {
            if manifest.id != id {
                return Err(GraphDslError::ManifestIdentity { expected: manifest.id.clone(), actual: id.to_string() });
            }
        }
        let mut nodes = BTreeMap::new();
        let mut handle_to_node = BTreeMap::new();
        if let Some(rows) = raw.get("nodes").and_then(|v| v.as_array()) {
            for row in rows {
                let Some(obj) = row.as_object() else { continue };
                let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                let kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                let name = obj.get("text").or_else(|| obj.get("name")).or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id).to_string();
                let mut properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                    Some(v) => json_to_property_bag(v),
                    None => PropertyBag::default(),
                };
                for (key, value) in obj.iter() {
                    if matches!(key, "id" | "nodeKind" | "node_kind" | "kind" | "text" | "name" | "label" | "handles" | "x" | "y" | "shape" | "radius" | "width" | "height" | "userData" | "user_data") {
                        continue;
                    }
                    if let Ok(prop) = <PropertyValue as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)) {
                        properties.insert(key.to_string(), prop);
                    }
                }
                nodes.insert(id.to_string(), (kind, name, properties));
                if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                    for handle in handles {
                        if let Some(hid) = handle.get("id").and_then(|v| v.as_str()) {
                            handle_to_node.insert(hid.to_string(), id.to_string());
                        }
                    }
                }
            }
        }
        let mut edges = Vec::new();
        if let Some(rows) = raw.get("edges").and_then(|v| v.as_array()) {
            for row in rows {
                let Some(obj) = row.as_object() else { continue };
                let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                let Some(source) = obj.get("source").and_then(|v| v.as_str()) else { continue };
                let Some(target) = obj.get("target").and_then(|v| v.as_str()) else { continue };
                let kind = obj.get("edgeKind").or_else(|| obj.get("edge_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                let properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                    Some(v) => json_to_property_bag(v),
                    None => PropertyBag::default(),
                };
                let (source_node_id, source_port) = split_endpoint(source, &handle_to_node);
                let (target_node_id, target_port) = split_endpoint(target, &handle_to_node);
                edges.push(QueryableEdge { id: id.to_string(), kind, source_node_id, target_node_id, source_port, target_port, properties });
            }
        }
        Ok(Self { manifest, nodes, edges, raw_fixture: raw })
    }

    pub fn from_object_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
        let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let mut fixture = raw.clone();
        if fixture.get("nodes").and_then(|v| v.as_array()).is_none() {
            if let Some(objects) = raw.get("objects").and_then(|v| v.as_array()) {
                let nodes: Vec<Value> = objects
                    .iter()
                    .filter_map(|row| {
                        let obj = row.as_object()?;
                        let id = obj.get("id").and_then(|v| v.as_str())?;
                        let kind = obj.get("objectKind").or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("Object");
                        let name = obj.get("name").or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id);
                        Some(Value::Object(semio_framework_pack_json::Object::from_iter([("id".to_string(), Value::from(id)), ("nodeKind".to_string(), Value::from(kind)), ("text".to_string(), Value::from(name))])))
                    })
                    .collect();
                if let Some(object) = fixture.as_object_mut() {
                    object.insert("nodes", Value::Array(nodes));
                }
            }
        }
        Self::from_host_snapshot_json(&semio_framework_pack_json::to_string(&fixture), manifest)
    }


}

impl QueryableGraph for BoardQueryableGraph {
    fn manifest(&self) -> Option<&GraphManifest> {
        self.manifest.as_ref()
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }

    fn node_kind(&self, id: &str) -> Option<String> {
        self.nodes.get(id).map(|(kind, _, _)| kind.clone())
    }

    fn node_name(&self, id: &str) -> Option<String> {
        self.nodes.get(id).map(|(_, name, _)| name.clone())
    }

    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
        let (_, name, properties) = self.nodes.get(id)?;
        match key {
            "id" => Some(PropertyValue::String(id.to_string())),
            "name" | "label" | "text" => Some(PropertyValue::String(name.clone())),
            "kind" => self.node_kind(id).map(PropertyValue::String),
            "__all" => Some(PropertyValue::Object(properties.clone())),
            _ => properties.get(key).cloned(),
        }
    }

    fn edges(&self) -> Vec<QueryableEdge> {
        self.edges.clone()
    }

    fn subgraph_snapshot_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
        let mut fixture = self.raw_fixture.clone();
        if let Some(nodes) = fixture.get_mut("nodes").and_then(|v| v.as_array_mut()) {
            nodes.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| node_ids.contains(id)));
        }
        if let Some(edges) = fixture.get_mut("edges").and_then(|v| v.as_array_mut()) {
            edges.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| edge_ids.contains(id)));
        }
        Some(semio_framework_pack_json::to_string(&fixture))
    }
}
// #endregion 🔖️BoardQueryableGraph
// #endregion queryable
