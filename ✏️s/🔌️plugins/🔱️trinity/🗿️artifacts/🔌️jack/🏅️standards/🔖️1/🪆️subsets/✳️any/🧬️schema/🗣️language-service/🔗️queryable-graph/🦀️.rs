//! 🔗️ Jack queryable graph: the borrowed and owned `QueryableGraph` adapters of the jack graph for the graph query language.

use super::*;

fn trinity_jack_manifest() -> &'static semio_framework_graph_manifest::GraphManifest {
    use std::sync::OnceLock;
    static MANIFEST: OnceLock<semio_framework_graph_manifest::GraphManifest> = OnceLock::new();
    MANIFEST.get_or_init(|| crate::standards::v1::subsets::any::schema::graph_manifest::manifest_by_id("nakagin").expect("nakagin manifest"))
}

fn trinity_queryable_edges(graph: &Graph) -> Vec<QueryableEdge> {
    graph
        .edges
        .values()
        .filter_map(|edge| {
            let source_node_id = port_node_id(&edge.source)?.to_string();
            let target_node_id = port_node_id(&edge.target)?.to_string();
            Some(QueryableEdge {
                id: edge.id.clone(),
                kind: edge.kind.clone(),
                source_node_id,
                target_node_id,
                source_port: port_port_id(&edge.source).map(str::to_string),
                target_port: port_port_id(&edge.target).map(str::to_string),
                properties: edge.properties.clone(),
            })
        })
        .collect()
}

pub struct TrinityQueryableGraph<'a>(pub &'a Graph);

impl QueryableGraph for TrinityQueryableGraph<'_> {
    fn manifest(&self) -> Option<&semio_framework_graph_manifest::GraphManifest> {
        Some(trinity_jack_manifest())
    }

    fn node_ids(&self) -> Vec<String> {
        self.0.nodes.keys().cloned().collect()
    }

    fn node_kind(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.kind.clone())
    }

    fn node_name(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.name.clone())
    }

    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
        let node = self.0.nodes.get(id)?;
        match key {
            "id" => Some(PropertyValue::String(id.to_string())),
            "name" | "label" | "text" => Some(PropertyValue::String(node.name.clone())),
            "kind" => Some(PropertyValue::String(node.kind.clone())),
            "__all" => Some(PropertyValue::Object(node.properties.clone())),
            _ => node.properties.get(key).cloned(),
        }
    }

    fn edges(&self) -> Vec<QueryableEdge> {
        trinity_queryable_edges(self.0)
    }

    fn subgraph_snapshot_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
        self.0.subgraph_snapshot(node_ids, edge_ids).to_json().ok()
    }
}

pub struct OwnedTrinityQueryableGraph(pub Graph);

impl QueryableGraph for OwnedTrinityQueryableGraph {
    fn manifest(&self) -> Option<&semio_framework_graph_manifest::GraphManifest> {
        Some(trinity_jack_manifest())
    }

    fn node_ids(&self) -> Vec<String> {
        self.0.nodes.keys().cloned().collect()
    }

    fn node_kind(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.kind.clone())
    }

    fn node_name(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.name.clone())
    }

    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
        let node = self.0.nodes.get(id)?;
        match key {
            "id" => Some(PropertyValue::String(id.to_string())),
            "name" | "label" | "text" => Some(PropertyValue::String(node.name.clone())),
            "kind" => Some(PropertyValue::String(node.kind.clone())),
            "__all" => Some(PropertyValue::Object(node.properties.clone())),
            _ => node.properties.get(key).cloned(),
        }
    }

    fn edges(&self) -> Vec<QueryableEdge> {
        trinity_queryable_edges(&self.0)
    }

    fn subgraph_snapshot_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
        self.0.subgraph_snapshot(node_ids, edge_ids).to_json().ok()
    }
}
