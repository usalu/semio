//! 🔺️ `trinity.graph` artifact — diff structs + `OperationDiff` impl (constitutional: diff).

use crate::artifacts::jack::{GraphFixture, Node, PropertyValue};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};
use vcs::{CollectionDiff, ItemPatch};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGeometryPatch {
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPatch {
    pub key: String,
    pub value: Option<PropertyValue>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityGraphDiff {
    pub nodes: CollectionDiff<String, NodeGeometryPatch, Node>,
    pub edges: CollectionDiff<String, PropertyPatch, crate::artifacts::jack::Edge>,
    pub node_properties: Vec<ItemPatch<String, PropertyPatch>>,
    pub edge_properties: Vec<ItemPatch<String, PropertyPatch>>,
    /// 📦️ Whole-fixture replacement (preset load, node-graph drag import) — the base the rest of the diff layers onto.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_fixture: Option<GraphFixture>,
    pub recompute_derived: bool,
}

impl OperationDiff<GraphFixture> for TrinityGraphDiff {
    fn apply(&self, projection: &GraphFixture) -> GraphFixture {
        let mut next = self.set_fixture.clone().unwrap_or_else(|| projection.clone());
        for id in &self.nodes.removed {
            remove_node_from_fixture(&mut next, id);
        }
        for patch in &self.nodes.modified {
            if let Some(node) = next.nodes.iter_mut().find(|node| node.id == patch.id) {
                if let Some(name) = &patch.patch.name {
                    node.name = name.clone();
                }
                if let Some(x) = patch.patch.x {
                    node.x = x;
                }
                if let Some(y) = patch.patch.y {
                    node.y = y;
                }
                if let Some(width) = patch.patch.width {
                    node.width = width;
                }
                if let Some(height) = patch.patch.height {
                    node.height = height;
                }
            }
        }
        for node in &self.nodes.added {
            next.nodes.push(node.clone());
        }
        for id in &self.edges.removed {
            next.edges.retain(|edge| edge.id != *id);
        }
        for edge in &self.edges.added {
            next.edges.push(edge.clone());
        }
        for patch in &self.node_properties {
            if let Some(node) = next.nodes.iter_mut().find(|node| node.id == patch.id) {
                match &patch.patch.value {
                    Some(value) => {
                        node.properties.insert(patch.patch.key.clone(), value.clone());
                    }
                    None => {
                        node.properties.remove(&patch.patch.key);
                    }
                }
            }
        }
        for patch in &self.edge_properties {
            if let Some(edge) = next.edges.iter_mut().find(|edge| edge.id == patch.id) {
                match &patch.patch.value {
                    Some(value) => {
                        edge.properties.insert(patch.patch.key.clone(), value.clone());
                    }
                    None => {
                        edge.properties.remove(&patch.patch.key);
                    }
                }
            }
        }
        if self.recompute_derived {
            if let Ok(mut graph) = crate::artifacts::jack::Graph::from_fixture(next.clone()) {
                graph.recompute_derived();
                next = graph.to_fixture();
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if let Some(fixture) = other.set_fixture {
            self.set_fixture = Some(fixture);
        }
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.modified.extend(other.nodes.modified);
        self.nodes.added.extend(other.nodes.added);
        self.edges.removed.extend(other.edges.removed);
        self.edges.added.extend(other.edges.added);
        self.node_properties.extend(other.node_properties);
        self.edge_properties.extend(other.edge_properties);
        self.recompute_derived |= other.recompute_derived;
    }
}

fn remove_node_from_fixture(fixture: &mut GraphFixture, id: &str) {
    fixture.nodes.retain(|node| node.id != id);
    fixture.edges.retain(|edge| crate::artifacts::jack::port_node_id(&edge.source) != Some(id) && crate::artifacts::jack::port_node_id(&edge.target) != Some(id));
    if fixture.root_node_id.as_deref() == Some(id) {
        fixture.root_node_id = None;
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trinity_graph_diff_absorb_merges_fields() {
        let mut diff = TrinityGraphDiff { recompute_derived: false, ..Default::default() };
        let other = TrinityGraphDiff {
            recompute_derived: true,
            nodes: CollectionDiff {
                added: vec![Node { id: "x".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: Default::default(), ports: vec![] }],
                ..Default::default()
            },
            ..Default::default()
        };
        diff.absorb(other);
        assert!(diff.recompute_derived);
        assert_eq!(diff.nodes.added.len(), 1);
    }
}
//#endregion 🧪️Tests
