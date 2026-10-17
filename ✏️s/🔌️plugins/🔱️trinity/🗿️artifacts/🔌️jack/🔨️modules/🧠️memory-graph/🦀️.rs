//! 🧠️ Jack in-memory graph: the `Graph` over a snapshot's working scene with its node, edge and property operations and the scene validation against the compile-time manifest.

use crate::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot;
use crate::standards::v1::subsets::any::schema::content::{JackContentFactory, JackWorkingScene};
use crate::standards::v1::subsets::any::schema::errors::TrinityRamError;
use crate::standards::v1::subsets::any::schema::graph::{port_node_id, Camera, Edge, EntityRef, Node};
use crate::standards::v1::subsets::any::schema::graph_manifest::manifest_by_id;
use crate::standards::v1::subsets::any::schema::manifest::Manifest;
use crate::standards::v1::subsets::any::schema::snapshot::{jack_working_scene, JackSnapshot};
use semio_framework_graph_manifest::{GraphManifest, ManifestValidator, PropertyValue};
use std::collections::{BTreeMap, BTreeSet};

/// 🧠️ In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest_id: Option<String>,
    pub manifest: Manifest,
    pub camera: Camera,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
    pub query: String,
}

impl Graph {
    pub fn from_snapshot(snapshot: JackSnapshot) -> Result<Self, TrinityRamError> {
        let scene = jack_working_scene(&snapshot)?;
        Self::with_scene(snapshot, scene)
    }

    /// 🧸️ The in-memory graph of a parent document over the scene its composed `content` child holds.
    pub fn with_scene(mut snapshot: JackSnapshot, scene: JackWorkingScene) -> Result<Self, TrinityRamError> {
        snapshot.validate_schema()?;
        snapshot.resolve_manifest()?;
        if let Some(id) = snapshot.manifest_id.as_deref() {
            if let Some(gm) = manifest_by_id(id) {
                validate_trinity_scene(&gm, &scene)?;
            }
        }
        let mut nodes = BTreeMap::new();
        for node in scene.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in scene.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self { name: snapshot.name, manifest_id: snapshot.manifest_id, manifest: snapshot.manifest, camera: snapshot.camera, nodes, edges, root_node_id: snapshot.root_node_id, query: snapshot.query })
    }

    pub fn to_snapshot<C:JackContentFactory>(&self) -> JackSnapshot {
        JackSnapshot { query: self.query.clone(), ..JackSnapshot::with_content::<C>(JackSnapshot::SCHEMA.to_string(), self.name.clone(), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: self.nodes.values().cloned().collect(), edges: self.edges.values().cloned().collect() }, self.root_node_id.clone()) }
    }

    pub fn load_json(json: &str) -> Result<Self, TrinityRamError> {
        Self::from_snapshot(JackSnapshot::from_json(json)?)
    }

    pub fn host_snapshot_json<C:JackContentFactory>(&self) -> Result<String, TrinityRamError> {
        self.to_snapshot::<C>().to_json()
    }

    /// 🧩️ Builds a `trinity.graph` snapshot containing only the given node and edge ids.
    pub fn subgraph_snapshot(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> JackQueryGraphSnapshot {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        JackQueryGraphSnapshot { schema: JackSnapshot::SCHEMA.into(), name: format!("{} subgraph", self.name), manifest_id: self.manifest_id.clone(), manifest: self.manifest.clone(), camera: self.camera.clone(), nodes, edges, root_node_id, query: String::new() }
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        if self.nodes.remove(id).is_none() {
            return false;
        }
        let edge_ids: Vec<String> = self.edges.iter().filter(|(_, e)| port_node_id(&e.source) == Some(id) || port_node_id(&e.target) == Some(id)).map(|(id, _)| id.clone()).collect();
        for eid in edge_ids {
            self.edges.remove(&eid);
        }
        if self.root_node_id.as_deref() == Some(id) {
            self.root_node_id = None;
        }
        true
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id.clone(), edge);
    }

    pub fn remove_edge(&mut self, id: &str) -> bool {
        self.edges.remove(id).is_some()
    }

    pub fn set_property(&mut self, entity: EntityRef, key: &str, value: PropertyValue) -> Result<(), TrinityRamError> {
        match entity {
            EntityRef::Node(id) => {
                let node = self.nodes.get_mut(&id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
                node.properties.insert(key.to_string(), value);
            }
            EntityRef::Edge(id) => {
                let edge = self.edges.get_mut(&id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
                edge.properties.insert(key.to_string(), value);
            }
        }
        Ok(())
    }
}

/// 🛡️ Validates trinity snapshot instances against a compile-time graph manifest.
fn validate_trinity_scene(gm: &GraphManifest, scene: &JackWorkingScene) -> Result<(), TrinityRamError> {
    let validator = ManifestValidator::new(gm);
    for node in &scene.nodes {
        validator.validate_node_kind(&node.kind)?;
        validator.validate_node_properties(&node.kind, &node.properties)?;
        if let Some(node_def) = gm.node_kind(&node.kind) {
            for port in &node.ports {
                validator.validate_port_kind(&port.kind)?;
                if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == &port.kind) {
                    return Err(TrinityRamError::PortKindNotDeclaredOnFixture { node_id: node.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                }
            }
        }
    }
    for edge in &scene.edges {
        validator.validate_edge_kind(&edge.kind)?;
        validator.validate_edge_properties(&edge.kind, &edge.properties)?;
    }
    Ok(())
}
