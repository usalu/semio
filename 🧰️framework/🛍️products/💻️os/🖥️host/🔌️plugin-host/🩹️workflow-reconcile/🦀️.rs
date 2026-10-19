//! 🩹️ Workflow reconcile of the plugin host: the graph reconcile that drops orphan edges, type mismatches, duplicate wires, cycles and unbound parameter bindings. See [`super`].

use super::*;

// #region host
//#region 🔖️GraphReconcile
/// 🧵️ Post-materialization workflow integrity pass, invoked explicitly by
/// `OsWorkflowStore::snapshot_with_conflicts` (NOT through `Mutation::reconcile` — the kernel
/// `workflow::WorkflowMutation` inherits that trait hook's no-op default, since the two rules that
/// used to run alongside these purely-structural ones need the os-core plugin/artifact registry the
/// kernel crate doesn't have; see `workflow::WorkflowMutation`'s own doc). Runs, in order: (1) drop
/// edges whose source/target node or port no longer exists (a concurrent delete tombstone wins over
/// the wiring), (2) drop edges whose port types no longer match (a concurrent re-typing wins over
/// the wiring), (3) dedupe edges with identical endpoints down to the lexicographically smallest id
/// (deterministic across peers replaying the same operation log), (4) break any cycle the previous
/// rules left behind, (5) drop parameter bindings whose target config field or parameter type no
/// longer validates. Each rule operates on the edge/binding set the previous one produced. Reports
/// through `protocol::MutationMessage` (the old free-form diagnostic struct's replacement,
/// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10), always at
/// `Warning`: every rule here is corrective (the drop always happens, regardless of this
/// message's level — unlike a `Mutation::diff` outcome, `Fatal`/`Error`'s "no change applied" laws
/// don't bind a post-merge reconcile pass), so `Warning` is the level that means "the state
/// changed to survive a conflict, and the change is worth surfacing" rather than "rejected"
/// (`Error`/`Fatal`) or "a routine side effect of an otherwise-successful op" (`Info`).
pub(super) fn reconcile_workflow_snapshot(mut document: workflow::WorkflowSnapshot) -> (workflow::WorkflowSnapshot, Vec<protocol::MutationMessage>) {
    let mut conflicts = Vec::new();
    let mut edges = std::mem::take(&mut document.graph.edges);
    let node_by_id: HashMap<&str, &workflow::WorkflowNode> = document.graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();

    //#region OrphanEdgeDrop
    edges.retain(|edge| {
        let source_ok = node_by_id.get(edge.source_node_id.as_str()).is_some_and(|node| node.outputs.iter().any(|port| port.id == edge.source_port_id));
        let target_ok = node_by_id.get(edge.target_node_id.as_str()).is_some_and(|node| node.inputs.iter().any(|port| port.id == edge.target_port_id));
        if source_ok && target_ok {
            true
        } else {
            conflicts.push(protocol::MutationMessage {
                level: semio_framework_diagnostic::Severity::Warning,
                code: semio_framework_diagnostic::FaultCode::new("workflow/edge-orphaned"),
                message: format!("edge {} references a node or port that no longer exists ({}:{} -> {}:{})", edge.id, edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id),
                target: vec![edge.id.clone()],
                op_index: None,
            });
            false
        }
    });
    //#endregion OrphanEdgeDrop

    //#region TypeMismatchDrop
    edges.retain(|edge| {
        let Some(source_port) = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)) else {
            return false;
        };
        let Some(target_port) = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)) else {
            return false;
        };
        match workflow::negotiate_media_contract(source_port, target_port) {
            Ok(contract) if contract == edge.contract => true,
            Ok(_) => {
                conflicts.push(protocol::MutationMessage {
                    level: semio_framework_diagnostic::Severity::Warning,
                    code: semio_framework_diagnostic::FaultCode::new("workflow/edge-type-mismatch"),
                    message: format!("edge {} contract stale: no longer matches negotiated port types", edge.id),
                    target: vec![edge.id.clone()],
                    op_index: None,
                });
                false
            }
            Err(reason) => {
                conflicts.push(protocol::MutationMessage {
                    level: semio_framework_diagnostic::Severity::Warning,
                    code: semio_framework_diagnostic::FaultCode::new("workflow/edge-type-mismatch"),
                    message: format!("edge {} connects ports whose types no longer match: {reason}", edge.id),
                    target: vec![edge.id.clone()],
                    op_index: None,
                });
                false
            }
        }
    });
    //#endregion TypeMismatchDrop

    //#region DuplicateWireDedupe
    let mut smallest_id_for_wire: HashMap<(String, String, String, String), String> = HashMap::new();
    for edge in &edges {
        let wire = (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        smallest_id_for_wire
            .entry(wire)
            .and_modify(|smallest| {
                if edge.id < *smallest {
                    *smallest = edge.id.clone();
                }
            })
            .or_insert_with(|| edge.id.clone());
    }
    edges.retain(|edge| {
        let wire = (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        smallest_id_for_wire.get(&wire).is_some_and(|smallest| smallest == &edge.id)
    });
    //#endregion DuplicateWireDedupe

    //#region CycleDrop
    edges = drop_workflow_cycle_edges(edges, &mut conflicts);
    //#endregion CycleDrop

    //#region ParameterBindingValidation
    let parameters = document.parameters.clone();
    let nodes = document.graph.nodes.clone();
    document.parameter_bindings.retain(|binding| {
        let Some(node) = nodes.iter().find(|node| node.id == binding.node_id) else { return true };
        let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) else { return true };
        let Some(parameter_type) = parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == binding.parameter_id).map(workflow_parameter_type_of) else { return true };
        match resolve_kernel_future(workflow::validate_workflow_parameter_config_binding(binding, &parameter_type, &registration.config)) {
            Ok(()) => true,
            Err(conflict) => {
                conflicts.push(conflict);
                false
            }
        }
    });
    document.graph = workflow::sync_workflow_parameter_ports(&document.graph, &document.parameter_bindings);
    //#endregion ParameterBindingValidation

    document.graph.edges = edges;
    (document, conflicts)
}

/// 🎛️ Maps a `workflow::WorkflowParameter` to its `WorkflowParameterType` tag — needed here
/// to type-check a binding's parameter against its target config field's `ArgSchema`.
fn workflow_parameter_type_of(parameter: &workflow::WorkflowParameter) -> workflow::WorkflowParameterType {
    match parameter {
        workflow::WorkflowParameter::Numeric { .. } => workflow::WorkflowParameterType::Numeric,
        workflow::WorkflowParameter::Categorical { .. } => workflow::WorkflowParameterType::Categorical,
        workflow::WorkflowParameter::Toggle { .. } => workflow::WorkflowParameterType::Toggle,
        workflow::WorkflowParameter::Text { .. } => workflow::WorkflowParameterType::Text,
    }
}

/// 🌀️ Repeatedly finds a cycle in `edges` (by node-id adjacency) and drops the participating
/// edge with the highest array index — a deterministic proxy for "newest edit" since
/// `reconcile_workflow_snapshot` only receives the materialized `WorkflowSnapshot` by value, not
/// per-edge `HybridLogicalTimestamp`s from the edit log. `apply_workflow_operation`'s `ConnectPorts`
/// handler appends new edges to the end of the vec, so a higher index approximates a later edit;
/// true HLT-based tie-breaking would need this pass to also see edit history, not just the document.
fn drop_workflow_cycle_edges(mut edges: Vec<workflow::WorkflowEdge>, conflicts: &mut Vec<protocol::MutationMessage>) -> Vec<workflow::WorkflowEdge> {
    while let Some(cycle_node_ids) = find_workflow_cycle_participants(&edges) {
        let newest_cycle_edge_index = edges.iter().enumerate().filter(|(_, edge)| cycle_node_ids.contains(&edge.source_node_id) && cycle_node_ids.contains(&edge.target_node_id)).map(|(index, _)| index).max();
        let Some(newest_cycle_edge_index) = newest_cycle_edge_index else { break };
        let dropped = edges.remove(newest_cycle_edge_index);
        conflicts.push(protocol::MutationMessage {
            level: semio_framework_diagnostic::Severity::Warning,
            code: semio_framework_diagnostic::FaultCode::new("workflow/edge-cycle"),
            message: format!("edge {} was dropped to break a cycle in the workflow", dropped.id),
            target: vec![dropped.id.clone()],
            op_index: None,
        });
    }
    edges
}

/// 🔍️ DFS cycle detection adapted from `workflow::validate_workflow`'s check, but returning
/// the participant node ids of the first cycle found (rather than just an error string) so the
/// caller can identify which edges are eligible for dropping.
fn find_workflow_cycle_participants(edges: &[workflow::WorkflowEdge]) -> Option<HashSet<String>> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for edge in edges {
        node_ids.insert(edge.source_node_id.clone());
        node_ids.insert(edge.target_node_id.clone());
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visited = HashSet::new();
    for node_id in &node_ids {
        if visited.contains(node_id) {
            continue;
        }
        let mut stack: Vec<String> = Vec::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        if let Some(cycle) = dfs_find_workflow_cycle(node_id, &adjacency, &mut stack, &mut on_stack, &mut visited) {
            return Some(cycle);
        }
    }
    None
}

fn dfs_find_workflow_cycle(node_id: &str, adjacency: &HashMap<String, Vec<String>>, stack: &mut Vec<String>, on_stack: &mut HashSet<String>, visited: &mut HashSet<String>) -> Option<HashSet<String>> {
    if on_stack.contains(node_id) {
        let start = stack.iter().position(|id| id == node_id).unwrap_or(0);
        return Some(stack[start..].iter().cloned().collect());
    }
    if visited.contains(node_id) {
        return None;
    }
    stack.push(node_id.to_string());
    on_stack.insert(node_id.to_string());
    for next in adjacency.get(node_id).into_iter().flatten() {
        if let Some(cycle) = dfs_find_workflow_cycle(next, adjacency, stack, on_stack, visited) {
            return Some(cycle);
        }
    }
    stack.pop();
    on_stack.remove(node_id);
    visited.insert(node_id.to_string());
    None
}
//#endregion 🔖️GraphReconcile
// #endregion host
