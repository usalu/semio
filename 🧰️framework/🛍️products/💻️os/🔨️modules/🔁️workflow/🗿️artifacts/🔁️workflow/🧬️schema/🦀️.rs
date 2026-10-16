//! 🧬️ Workflow schema: the mutation vocabulary, operation application, the diff algebra and the native mutation codecs, with the artifact's behavioural laws.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

use semio_framework_artifact_workflow_workflow_graph::*;
use semio_framework_artifact_workflow_workflow_snapshot::*;

//#region 🔖️WorkflowMutation
#[path = "🧬️mutations/🦀️.rs"]
mod workflow_mutations;
pub use workflow_mutations::{
    AddInput, AddNode, AddParameter, BindInput, BindOutput, BindParameterField, ChangeParameter, ConnectPorts, DisconnectEdge, MoveNode, MoveNodes, RemoveInput, RemoveNode, RemoveParameter, RenameNode, SetNodePositions, UnbindInput, UnbindOutput,
    UnbindParameterField, UpdateNodePorts, WorkflowMutation, WorkflowNodePosition,
};

pub fn apply_workflow_operation(document: &WorkflowSnapshot, operation: &WorkflowMutation) -> WorkflowSnapshot {
    let mut next = document.clone();
    match operation {
        WorkflowMutation::AddNode(AddNode { node }) => {
            let node = sync_workflow_node_parameter_ports(node, &next.parameter_bindings);
            next.graph.nodes.push(node);
        }
        WorkflowMutation::RemoveNode(RemoveNode { node_id }) => {
            next.parameter_bindings.retain(|binding| binding.node_id != *node_id);
            next.input_bindings.retain(|binding| binding.node_id != *node_id);
            next.output_bindings.retain(|binding| binding.node_id != *node_id);
            next.graph.nodes.retain(|node| node.id != *node_id);
            next.graph.edges.retain(|edge| edge.source_node_id != *node_id && edge.target_node_id != *node_id);
        }
        WorkflowMutation::ConnectPorts(ConnectPorts { edge }) => next.graph.edges.push(edge.clone()),
        WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id }) => next.graph.edges.retain(|edge| edge.id != *edge_id),
        WorkflowMutation::MoveNode(MoveNode { node_id, x, y }) => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        WorkflowMutation::RenameNode(RenameNode { node_id, label }) => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.label = label.clone();
                }
            }
        }
        WorkflowMutation::AddParameter(AddParameter { parameter }) => next.parameters.push((**parameter).clone()),
        WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id }) => {
            next.parameters.retain(|parameter| workflow_parameter_entity_id(parameter) != *parameter_id);
            next.parameter_bindings.retain(|binding| binding.parameter_id != *parameter_id);
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id, parameter }) => {
            for entry in &mut next.parameters {
                if workflow_parameter_entity_id(entry) == *parameter_id {
                    *entry = (**parameter).clone();
                }
            }
        }
        WorkflowMutation::BindParameterField(BindParameterField { binding }) => {
            next.parameter_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.field_path == binding.field_path));
            next.parameter_bindings.push(binding.clone());
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id, field_path }) => {
            next.parameter_bindings.retain(|binding| !(binding.node_id == *node_id && binding.field_path == *field_path));
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}) => {
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::AddInput(AddInput { input }) => next.inputs.push(input.clone()),
        WorkflowMutation::RemoveInput(RemoveInput { input_id }) => {
            next.inputs.retain(|input| input.id != *input_id);
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindInput(BindInput { binding }) => {
            next.input_bindings.retain(|entry| entry.input_id != binding.input_id);
            next.input_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindInput(UnbindInput { input_id }) => {
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindOutput(BindOutput { binding }) => {
            next.output_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.port_id == binding.port_id));
            next.output_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindOutput(UnbindOutput { node_id, port_id }) => {
            next.output_bindings.retain(|binding| !(binding.node_id == *node_id && binding.port_id == *port_id));
        }
        WorkflowMutation::MoveNodes(MoveNodes { node_ids, dx, dy }) => {
            for node in next.graph.nodes.iter_mut().filter(|node| node_ids.contains(&node.id)) {
                node.x += *dx;
                node.y += *dy;
            }
        }
        WorkflowMutation::SetNodePositions(SetNodePositions { positions }) => {
            for position in positions {
                for node in next.graph.nodes.iter_mut().filter(|node| node.id == position.node_id) {
                    node.x = position.x;
                    node.y = position.y;
                }
            }
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum WorkflowDiff {
    #[default]
    Empty,
    AddNode {
        node: WorkflowNode,
    },
    RemoveNode {
        node_id: String,
    },
    ConnectPorts {
        edge: WorkflowEdge,
    },
    DisconnectEdge {
        edge_id: String,
    },
    MoveNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PlaceNodes {
        positions: Vec<WorkflowNodePosition>,
    },
    PatchNode {
        node_id: String,
        label: String,
    },
    AddParameter {
        parameter: WorkflowParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: WorkflowParameter,
    },
    BindParameterField {
        binding: WorkflowParameterBinding,
    },
    UnbindParameterField {
        node_id: String,
        field_path: String,
    },
    SyncNodePorts,
    DeclareInput {
        input: WorkflowInput,
    },
    RemoveInput {
        input_id: String,
    },
    BindInput {
        binding: WorkflowInputBinding,
    },
    UnbindInput {
        input_id: String,
    },
    BindOutput {
        binding: WorkflowOutputBinding,
    },
    UnbindOutput {
        node_id: String,
        port_id: String,
    },
    Sequence {
        steps: Vec<WorkflowDiff>,
    },
}

impl WorkflowDiff {
    fn atoms(&self) -> Vec<&WorkflowDiff> {
        match self {
            WorkflowDiff::Empty => Vec::new(),
            WorkflowDiff::Sequence { steps } => steps.iter().flat_map(WorkflowDiff::atoms).collect(),
            atom => vec![atom],
        }
    }

    fn sequence(mut atoms: Vec<WorkflowDiff>) -> WorkflowDiff {
        match atoms.len() {
            0 => WorkflowDiff::Empty,
            1 => atoms.pop().expect("single workflow diff step"),
            _ => WorkflowDiff::Sequence { steps: atoms },
        }
    }

    /// ↩️ The steps that undo this one atom, in application order, read from `base` or, for a row the diff itself introduced, from the
    /// `earlier` atoms' rows. Nothing is applied; an atom that edits a slot an earlier atom of the same diff already edited is inverted
    /// step by step by the store, which absorbs the per-step inverses.
    fn undo_rows(&self, base: &WorkflowSnapshot, earlier: &[&WorkflowDiff]) -> Vec<WorkflowDiff> {
        match self {
            WorkflowDiff::Empty | WorkflowDiff::SyncNodePorts | WorkflowDiff::Sequence { .. } => Vec::new(),
            WorkflowDiff::AddNode { node } => vec![WorkflowDiff::RemoveNode { node_id: node.id.clone() }],
            WorkflowDiff::RemoveNode { node_id } => {
                let Some(node) = base.graph.nodes.iter().find(|node| node.id == *node_id).or_else(|| earlier.iter().rev().find_map(|atom| if let WorkflowDiff::AddNode { node } = atom { (node.id == *node_id).then_some(node) } else { None })) else { return Vec::new() };
                let mut steps = vec![WorkflowDiff::AddNode { node: node.clone() }];
                steps.extend(base.output_bindings.iter().rev().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowDiff::BindOutput { binding: binding.clone() }));
                steps.extend(base.input_bindings.iter().rev().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowDiff::BindInput { binding: binding.clone() }));
                steps.extend(base.parameter_bindings.iter().rev().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowDiff::BindParameterField { binding: binding.clone() }));
                steps.extend(base.graph.edges.iter().rev().filter(|edge| edge.source_node_id == *node_id || edge.target_node_id == *node_id).map(|edge| WorkflowDiff::ConnectPorts { edge: edge.clone() }));
                steps
            }
            WorkflowDiff::ConnectPorts { edge } => vec![WorkflowDiff::DisconnectEdge { edge_id: edge.id.clone() }],
            WorkflowDiff::DisconnectEdge { edge_id } => base.graph.edges.iter().find(|edge| edge.id == *edge_id).or_else(|| earlier.iter().rev().find_map(|atom| if let WorkflowDiff::ConnectPorts { edge } = atom { (edge.id == *edge_id).then_some(edge) } else { None })).map(|edge| vec![WorkflowDiff::ConnectPorts { edge: edge.clone() }]).unwrap_or_default(),
            WorkflowDiff::MoveNode { node_id, .. } => base.graph.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![WorkflowDiff::MoveNode { node_id: node_id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
            WorkflowDiff::PlaceNodes { positions } => vec![WorkflowDiff::PlaceNodes {
                positions: positions
                    .iter()
                    .filter_map(|position| base.graph.nodes.iter().find(|node| node.id == position.node_id).map(|node| WorkflowNodePosition { node_id: position.node_id.clone(), x: node.x, y: node.y }))
                    .collect(),
            }],
            WorkflowDiff::PatchNode { node_id, .. } => base.graph.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![WorkflowDiff::PatchNode { node_id: node_id.clone(), label: node.label.clone() }]).unwrap_or_default(),
            WorkflowDiff::AddParameter { parameter } => vec![WorkflowDiff::RemoveParameter { parameter_id: workflow_parameter_entity_id(parameter).to_string() }],
            WorkflowDiff::RemoveParameter { parameter_id } => {
                let Some(parameter) = base.parameters.iter().find(|parameter| workflow_parameter_entity_id(parameter) == parameter_id).or_else(|| earlier.iter().rev().find_map(|atom| if let WorkflowDiff::AddParameter { parameter } = atom { (workflow_parameter_entity_id(parameter) == parameter_id).then_some(parameter) } else { None })) else { return Vec::new() };
                let mut steps = vec![WorkflowDiff::AddParameter { parameter: parameter.clone() }];
                steps.extend(base.parameter_bindings.iter().rev().filter(|binding| binding.parameter_id == *parameter_id).map(|binding| WorkflowDiff::BindParameterField { binding: binding.clone() }));
                steps
            }
            WorkflowDiff::PatchParameter { parameter_id, parameter } => base
                .parameters
                .iter()
                .find(|entry| workflow_parameter_entity_id(entry) == parameter_id)
                .map(|prior| vec![WorkflowDiff::PatchParameter { parameter_id: workflow_parameter_entity_id(parameter).to_string(), parameter: prior.clone() }])
                .unwrap_or_default(),
            WorkflowDiff::BindParameterField { binding } => vec![match base.parameter_bindings.iter().find(|entry| entry.node_id == binding.node_id && entry.field_path == binding.field_path) {
                Some(prior) => WorkflowDiff::BindParameterField { binding: prior.clone() },
                None => WorkflowDiff::UnbindParameterField { node_id: binding.node_id.clone(), field_path: binding.field_path.clone() },
            }],
            WorkflowDiff::UnbindParameterField { node_id, field_path } => base
                .parameter_bindings
                .iter()
                .find(|binding| binding.node_id == *node_id && binding.field_path == *field_path)
                .map(|binding| vec![WorkflowDiff::BindParameterField { binding: binding.clone() }])
                .unwrap_or_default(),
            WorkflowDiff::DeclareInput { input } => vec![WorkflowDiff::RemoveInput { input_id: input.id.clone() }],
            WorkflowDiff::RemoveInput { input_id } => {
                let Some(input) = base.inputs.iter().find(|input| input.id == *input_id).or_else(|| earlier.iter().rev().find_map(|atom| if let WorkflowDiff::DeclareInput { input } = atom { (input.id == *input_id).then_some(input) } else { None })) else { return Vec::new() };
                let mut steps = vec![WorkflowDiff::DeclareInput { input: input.clone() }];
                steps.extend(base.input_bindings.iter().rev().filter(|binding| binding.input_id == *input_id).map(|binding| WorkflowDiff::BindInput { binding: binding.clone() }));
                steps
            }
            WorkflowDiff::BindInput { binding } => vec![match base.input_bindings.iter().find(|entry| entry.input_id == binding.input_id) {
                Some(prior) => WorkflowDiff::BindInput { binding: prior.clone() },
                None => WorkflowDiff::UnbindInput { input_id: binding.input_id.clone() },
            }],
            WorkflowDiff::UnbindInput { input_id } => base.input_bindings.iter().find(|binding| binding.input_id == *input_id).map(|binding| vec![WorkflowDiff::BindInput { binding: binding.clone() }]).unwrap_or_default(),
            WorkflowDiff::BindOutput { binding } => vec![match base.output_bindings.iter().find(|entry| entry.node_id == binding.node_id && entry.port_id == binding.port_id) {
                Some(prior) => WorkflowDiff::BindOutput { binding: prior.clone() },
                None => WorkflowDiff::UnbindOutput { node_id: binding.node_id.clone(), port_id: binding.port_id.clone() },
            }],
            WorkflowDiff::UnbindOutput { node_id, port_id } => base
                .output_bindings
                .iter()
                .find(|binding| binding.node_id == *node_id && binding.port_id == *port_id)
                .map(|binding| vec![WorkflowDiff::BindOutput { binding: binding.clone() }])
                .unwrap_or_default(),
        }
    }

    fn apply_atom(&self, document: &WorkflowSnapshot) -> protocol::MutationApplyResult<WorkflowSnapshot> {
        match self {
            WorkflowDiff::Empty | WorkflowDiff::SyncNodePorts => {}
            WorkflowDiff::Sequence { .. } => return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "workflow diff sequences apply step by step")),
            WorkflowDiff::AddNode { node } => {
                if document.graph.nodes.iter().any(|entry| entry.id == node.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow node identity already exists").at(["nodes", node.id.as_str()]));
                }
            }
            WorkflowDiff::RemoveNode { node_id } | WorkflowDiff::MoveNode { node_id, .. } | WorkflowDiff::PatchNode { node_id, .. } => {
                if !document.graph.nodes.iter().any(|node| node.id == *node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow node does not exist").at(["nodes", node_id.as_str()]));
                }
            }
            WorkflowDiff::PlaceNodes { positions } => {
                if let Some(position) = positions.iter().find(|position| !document.graph.nodes.iter().any(|node| node.id == position.node_id)) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow node does not exist").at(["nodes", position.node_id.as_str()]));
                }
            }
            WorkflowDiff::ConnectPorts { edge } => {
                if document.graph.edges.iter().any(|entry| entry.id == edge.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow edge identity already exists").at(["edges", edge.id.as_str()]));
                }
                let source = match document.graph.nodes.iter().find(|node| node.id == edge.source_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source node does not exist").at(["edges", edge.id.as_str(), "sourceNodeId"])),
                };
                if !source.outputs.iter().any(|port| port.id == edge.source_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source port does not exist").at(["edges", edge.id.as_str(), "sourcePortId"]));
                }
                let target = match document.graph.nodes.iter().find(|node| node.id == edge.target_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target node does not exist").at(["edges", edge.id.as_str(), "targetNodeId"])),
                };
                if !target.inputs.iter().any(|port| port.id == edge.target_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target port does not exist").at(["edges", edge.id.as_str(), "targetPortId"]));
                }
            }
            WorkflowDiff::DisconnectEdge { edge_id } => {
                if !document.graph.edges.iter().any(|edge| edge.id == *edge_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge does not exist").at(["edges", edge_id.as_str()]));
                }
            }
            WorkflowDiff::AddParameter { parameter } => {
                let parameter_id = workflow_parameter_entity_id(parameter);
                if document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow parameter identity already exists").at(["parameters", parameter_id]));
                }
            }
            WorkflowDiff::RemoveParameter { parameter_id } => {
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").at(["parameters", parameter_id.as_str()]));
                }
            }
            WorkflowDiff::PatchParameter { parameter_id, parameter } => {
                if !document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").at(["parameters", parameter_id.as_str()]));
                }
                let new_id = workflow_parameter_entity_id(parameter);
                if new_id != parameter_id && document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == new_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "patched workflow parameter identity already exists").at(["parameters", new_id]));
                }
            }
            WorkflowDiff::BindParameterField { binding } => {
                if !document.graph.nodes.iter().any(|node| node.id == binding.node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "parameter binding node does not exist").at(["parameterBindings", binding.node_id.as_str()]));
                }
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == binding.parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow parameter does not exist").at(["parameterBindings", binding.parameter_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindParameterField { node_id, field_path } => {
                if !document.parameter_bindings.iter().any(|binding| binding.node_id == *node_id && binding.field_path == *field_path) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter binding does not exist").at(["parameterBindings", node_id.as_str(), field_path.as_str()]));
                }
            }
            WorkflowDiff::DeclareInput { input } => {
                if document.inputs.iter().any(|entry| entry.id == input.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow input identity already exists").at(["inputs", input.id.as_str()]));
                }
            }
            WorkflowDiff::RemoveInput { input_id } => {
                if !document.inputs.iter().any(|input| input.id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input does not exist").at(["inputs", input_id.as_str()]));
                }
            }
            WorkflowDiff::BindInput { binding } => {
                if !document.inputs.iter().any(|input| input.id == binding.input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow input does not exist").at(["inputBindings", binding.input_id.as_str()]));
                }
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding node does not exist").at(["inputBindings", binding.node_id.as_str()])),
                };
                if !node.inputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding port does not exist").at(["inputBindings", binding.node_id.as_str(), binding.port_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindInput { input_id } => {
                if !document.input_bindings.iter().any(|binding| binding.input_id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input binding does not exist").at(["inputBindings", input_id.as_str()]));
                }
            }
            WorkflowDiff::BindOutput { binding } => {
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding node does not exist").at(["outputBindings", binding.node_id.as_str()])),
                };
                if !node.outputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding port does not exist").at(["outputBindings", binding.node_id.as_str(), binding.port_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindOutput { node_id, port_id } => {
                if !document.output_bindings.iter().any(|binding| binding.node_id == *node_id && binding.port_id == *port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow output binding does not exist").at(["outputBindings", node_id.as_str(), port_id.as_str()]));
                }
            }
        }
        let operation = match self {
            WorkflowDiff::Empty | WorkflowDiff::Sequence { .. } => return Ok(document.clone()),
            WorkflowDiff::AddNode { node } => WorkflowMutation::AddNode(AddNode { node: node.clone() }),
            WorkflowDiff::RemoveNode { node_id } => WorkflowMutation::RemoveNode(RemoveNode { node_id: node_id.clone() }),
            WorkflowDiff::ConnectPorts { edge } => WorkflowMutation::ConnectPorts(ConnectPorts { edge: edge.clone() }),
            WorkflowDiff::DisconnectEdge { edge_id } => WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge_id.clone() }),
            WorkflowDiff::MoveNode { node_id, x, y } => WorkflowMutation::MoveNode(MoveNode { node_id: node_id.clone(), x: *x, y: *y }),
            WorkflowDiff::PlaceNodes { positions } => WorkflowMutation::SetNodePositions(SetNodePositions { positions: positions.clone() }),
            WorkflowDiff::PatchNode { node_id, label } => WorkflowMutation::RenameNode(RenameNode { node_id: node_id.clone(), label: label.clone() }),
            WorkflowDiff::AddParameter { parameter } => WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(parameter.clone()) }),
            WorkflowDiff::RemoveParameter { parameter_id } => WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: parameter_id.clone() }),
            WorkflowDiff::PatchParameter { parameter_id, parameter } => WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: parameter_id.clone(), parameter: Box::new(parameter.clone()) }),
            WorkflowDiff::BindParameterField { binding } => WorkflowMutation::BindParameterField(BindParameterField { binding: binding.clone() }),
            WorkflowDiff::UnbindParameterField { node_id, field_path } => WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() }),
            WorkflowDiff::SyncNodePorts => WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}),
            WorkflowDiff::DeclareInput { input } => WorkflowMutation::AddInput(AddInput { input: input.clone() }),
            WorkflowDiff::RemoveInput { input_id } => WorkflowMutation::RemoveInput(RemoveInput { input_id: input_id.clone() }),
            WorkflowDiff::BindInput { binding } => WorkflowMutation::BindInput(BindInput { binding: binding.clone() }),
            WorkflowDiff::UnbindInput { input_id } => WorkflowMutation::UnbindInput(UnbindInput { input_id: input_id.clone() }),
            WorkflowDiff::BindOutput { binding } => WorkflowMutation::BindOutput(BindOutput { binding: binding.clone() }),
            WorkflowDiff::UnbindOutput { node_id, port_id } => WorkflowMutation::UnbindOutput(UnbindOutput { node_id: node_id.clone(), port_id: port_id.clone() }),
        };
        Ok(apply_workflow_operation(document, &operation))
    }
}

impl protocol::MutationDiff<WorkflowSnapshot> for WorkflowDiff {
    fn apply(&self, document: &WorkflowSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WorkflowSnapshot> {
        let mut current = document.clone();
        for atom in self.atoms() {
            current = atom.apply_atom(&current)?;
        }
        Ok(current)
    }

    fn absorb(&mut self, other: Self) {
        let mut atoms: Vec<WorkflowDiff> = std::mem::take(self).atoms().into_iter().cloned().collect();
        atoms.extend(other.atoms().into_iter().cloned());
        *self = WorkflowDiff::sequence(atoms);
    }
}

impl protocol::DiffAlgebra<WorkflowSnapshot> for WorkflowDiff {
    fn inverse(&self, base: &WorkflowSnapshot) -> Self {
        let atoms = self.atoms();
        let rows: Vec<Vec<WorkflowDiff>> = atoms.iter().enumerate().map(|(index, atom)| atom.undo_rows(base, &atoms[..index])).collect();
        WorkflowDiff::sequence(rows.into_iter().rev().flatten().collect())
    }

    fn is_empty(&self) -> bool {
        self.atoms().is_empty()
    }
}

//#endregion 🔖️WorkflowMutation

//#region 🔖️WorkflowMutationOpText



//#endregion 🔖️WorkflowMutationOpText


#[path = "../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
mod binary_mutations;
#[path = "../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
mod text_mutations;
