//! 🧭️ Topology and edges: topological node order and edge media-kind validation.
//!
//! Holds the `🔖️Topology`, `🔖️EdgeValidation` regions of the module; the root re-exports its items.

use super::*;

//#region 🔖️Topology
/// 🔢️ Deterministic topological order (Kahn's algorithm, lexicographically-smallest-ready-node-first)
/// over `graph`'s nodes. `Err(RunError::Cycle)` names whichever nodes never became ready —
/// `workflow::validate_workflow` should be called first to reject cycles with a friendlier message;
/// this is the runner's authoritative order once that check has passed.
///
/// 🧷️ Not redundant with `workflow::plan_workflow`: that function only walks the downstream closure
/// of an already-dirty node set to emit `WorkflowDelivery` records, and its own topological helper is
/// private to the kernel crate. The runner needs a total Kahn order over EVERY node (dirty, clean, or
/// edge-less) to decide host-open/execution sequence deterministically — a genuinely different job.
pub(super) fn topological_order(graph: &Workflow) -> Result<Vec<String>, RunError> {
    let mut indegree: BTreeMap<String, usize> = graph.nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        *indegree.entry(edge.target_node_id.clone()).or_insert(0) += 1;
        outgoing.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut ready: BTreeSet<String> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| id.clone()).collect();
    let mut order = Vec::with_capacity(graph.nodes.len());
    while let Some(node_id) = ready.iter().next().cloned() {
        ready.remove(&node_id);
        order.push(node_id.clone());
        for next in outgoing.get(&node_id).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(next) {
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(next.clone());
                }
            }
        }
    }
    if order.len() != graph.nodes.len() {
        let unreached: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).filter(|id| !order.contains(id)).collect();
        return Err(RunError::Cycle(unreached));
    }
    Ok(order)
}
//#endregion 🔖️Topology

//#region 🔖️EdgeValidation
/// ✅️ Full connect-time edge validation, called by both `plan` (the `--dry` path never opens a
/// single node) and `run`: every edge's endpoints must resolve to real ports; the producer's
/// `MediaType` must be `media_types_compatible` with the consumer's (typed rejection, no more
/// stringly `artifact_kind` equality); a negotiated `contract.conversion` must have a registered
/// converter (fail-closed — an edge nobody can actually run is worse than one caught here at plan
/// time); and every input port's `required`/`multiplicity` constraint must hold across the WHOLE
/// node, not just one edge (so it's checked in a second pass once every edge's incoming count is known).
pub(super) async fn validate_edge_kinds(graph: &Workflow) -> Result<(), RunError> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming_count: HashMap<(&str, &str), usize> = HashMap::new();

    for edge in &graph.edges {
        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
        let target_node = *node_by_id.get(edge.target_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.target_node_id.clone()))?;
        let source_port = source_node.outputs.iter().find(|port| port.id == edge.source_port_id).ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.source_node_id.clone(), port_id: edge.source_port_id.clone() })?;
        let target_port = target_node.inputs.iter().find(|port| port.id == edge.target_port_id).ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.target_node_id.clone(), port_id: edge.target_port_id.clone() })?;

        if matches!(media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type).await, MediaCompat::Reject) {
            return Err(RunError::Incompatible { edge_id: edge.id.clone(), produced: source_port.spec.media_type, accepted: target_port.spec.media_type });
        }

        if let Some((from, to)) = edge.contract.conversion {
            if !media_converter_registered(edge.contract.media_type.class, from, to) {
                return Err(RunError::UnregisteredConversion { edge_id: edge.id.clone(), from, to });
            }
        }

        *incoming_count.entry((edge.target_node_id.as_str(), edge.target_port_id.as_str())).or_insert(0) += 1;
    }

    for node in &graph.nodes {
        for port in &node.inputs {
            let count = incoming_count.get(&(node.id.as_str(), port.id.as_str())).copied().unwrap_or(0);
            if port.spec.required && count == 0 {
                return Err(RunError::MissingRequiredInput { node_id: node.id.clone(), port_id: port.id.clone() });
            }
            if matches!(port.spec.multiplicity, PortMultiplicity::One) && count > 1 {
                return Err(RunError::MultiplicityViolation { node_id: node.id.clone(), port_id: port.id.clone(), count });
            }
        }
    }
    Ok(())
}
//#endregion 🔖️EdgeValidation
