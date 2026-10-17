//! ✅️ Validation and planning of the workflow graph: the structural validator, the delivery record, the topological order and the delivery plan. See [`super`].

use super::*;

//#region 🔖️WorkflowValidator
#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowValidation {
    pub ok: bool,
    pub errors: Vec<String>,
}

/// ✅️ Validates workflow connectivity and cycle freedom. Ported down from
/// `framework/product/os/core`'s `workflow` module (`validate_workflow`); the edge-contract
/// re-negotiation check that lived alongside it there (re-running `negotiate_media_contract` against
/// the live artifact registry) stays in os-core for now — it needs the artifact-kind registry, which
/// doesn't exist at this layer yet. A later work package re-adds it here once contract negotiation
/// itself moves down.
pub async fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
    let mut errors = Vec::new();
    let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.clone()).collect();
    for edge in &graph.edges {
        if !node_ids.contains(&edge.source_node_id) {
            errors.push(format!("missing source node {}", edge.source_node_id));
        }
        if !node_ids.contains(&edge.target_node_id) {
            errors.push(format!("missing target node {}", edge.target_node_id));
        }
    }

    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    async fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visiting: &mut HashSet<String>, visited: &mut HashSet<String>, errors: &mut Vec<String>) {
        if visited.contains(node_id) {
            return;
        }
        if visiting.contains(node_id) {
            errors.push(format!("cycle detected at {node_id}"));
            return;
        }
        visiting.insert(node_id.to_string());
        for next in adjacency.get(node_id).into_iter().flatten() {
            Box::pin(dfs(next, adjacency, visiting, visited, errors)).await;
        }
        visiting.remove(node_id);
        visited.insert(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors).await;
    }
    WorkflowValidation { ok: errors.is_empty(), errors }
}
//#endregion 🔖️WorkflowValidator

//#region 🔖️WorkflowPlanner
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkflowDelivery {
    pub edge_id: String,
    pub producer_node_id: String,
    pub producer_port_id: String,
    pub consumer_node_id: String,
    pub consumer_port_id: String,
}

/// 🧭️ Post-order DFS reversed into a topological node order (source before target); same
/// recursive shape as `validate_workflow`'s cycle-detection DFS, but collects the traversal order
/// instead of flagging revisits (the graph is validated acyclic before planning runs).
async fn workflow_topological_node_order(graph: &Workflow) -> Vec<String> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    async fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, order: &mut Vec<String>) {
        if !visited.insert(node_id.to_string()) {
            return;
        }
        for next in adjacency.get(node_id).into_iter().flatten() {
            Box::pin(dfs(next, adjacency, visited, order)).await;
        }
        order.push(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visited, &mut order).await;
    }
    order.reverse();
    order
}

/// 🚚️ Plans one [`WorkflowDelivery`] per edge in the downstream closure of `dirty_node_ids`,
/// propagating dirtiness onto each edge's consumer node so multi-hop chains (A→B→C) resolve in a
/// single topological pass. Pure/side-effect-free — callers own applying the deliveries.
pub async fn plan_workflow(graph: &Workflow, dirty_node_ids: &HashSet<String>) -> Vec<WorkflowDelivery> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut edges_by_source: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        edges_by_source.entry(edge.source_node_id.as_str()).or_default().push(edge);
    }
    let order = workflow_topological_node_order(graph).await;
    let mut dirty = dirty_node_ids.clone();
    let mut deliveries = Vec::new();
    for node_id in &order {
        let Some(node) = node_by_id.get(node_id.as_str()) else { continue };
        if !dirty.contains(node.id.as_str()) {
            continue;
        }
        for edge in edges_by_source.get(node_id.as_str()).into_iter().flatten() {
            let Some(target_node) = node_by_id.get(edge.target_node_id.as_str()) else { continue };
            deliveries.push(WorkflowDelivery { edge_id: edge.id.clone(), producer_node_id: node.id.clone(), producer_port_id: edge.source_port_id.clone(), consumer_node_id: target_node.id.clone(), consumer_port_id: edge.target_port_id.clone() });
            dirty.insert(target_node.id.clone());
        }
    }
    deliveries
}
//#endregion 🔖️WorkflowPlanner
