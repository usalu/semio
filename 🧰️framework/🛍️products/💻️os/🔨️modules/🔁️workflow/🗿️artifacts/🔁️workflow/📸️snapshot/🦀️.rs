//! 📸️ Workflow snapshot: the `os.workflow` persisted artifact with its bindings, snapshot validation and native snapshot codecs.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

use semio_framework_artifact_workflow_workflow_graph::*;
use semio_framework::{MediaForm, MediaPortSpec, MediaType, MediaWireFormat};
use std::collections::{HashMap, HashSet};

//#region 🔖️WorkflowSnapshot

/// 🔗️ Binds a declared [`WorkflowInput`] slot onto one node's in-port.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowInputBinding {
    pub input_id: String,
    pub node_id: String,
    pub port_id: String,
}

/// 📤️ Names where a node's out-port materializes in the output collection — `path_template` like
/// `"renders/{node}/{input.stem}.{ext}"` (resolved at run time by W5's `SpaceRunner`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowOutputBinding {
    pub node_id: String,
    pub port_id: String,
    pub path_template: String,
}

/// 🕸️ The `os.workflow` persisted artifact — a non-destructive pipeline of connected apps (`graph`),
/// its parameters/bindings, and its declared collection-level inputs/outputs. Absorbs os-core's
/// dissolved `OsSnapshot` (`programs` moved to `space::SpaceSnapshot`, `active_plugin_id`/
/// `active_alternative_id` become space-app session state — see `## The inversion` in the plan).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[artifact(id = "os.workflow")]
pub struct WorkflowSnapshot {
    pub schema: String,
    #[dsl(block)]
    pub graph: Workflow,
    #[dsl(statements)]
    pub parameters: Vec<WorkflowParameter>,
    #[dsl(table)]
    pub parameter_bindings: Vec<WorkflowParameterBinding>,
    pub inputs: Vec<WorkflowInput>,
    #[dsl(table)]
    pub input_bindings: Vec<WorkflowInputBinding>,
    #[dsl(table)]
    pub output_bindings: Vec<WorkflowOutputBinding>,
}

pub async fn empty_workflow_snapshot() -> WorkflowSnapshot {
    WorkflowSnapshot { schema: S_WORKFLOW_SCHEMA.into(), graph: empty_workflow().await, parameters: Vec::new(), parameter_bindings: Vec::new(), inputs: Vec::new(), input_bindings: Vec::new(), output_bindings: Vec::new() }
}

//#region 🔖️HandcraftedWorkflowSnapshotCodecs



//#endregion 🔖️HandcraftedWorkflowSnapshotCodecs

impl semio_framework_schema_composition::ArtifactCompositionFields for WorkflowSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_entity_id(parameter: &WorkflowParameter) -> &str {
    workflow_parameter_id(parameter)
}

/// ✅️ Extends [`validate_workflow`] with the two `WorkflowSnapshot`-level checks that need the
/// declared `inputs`/`input_bindings`/`output_bindings` (pure/registry-free, unlike os-core's own
/// `validate_workflow` wrapper which layers on the contract-renegotiation check): (1) a required node
/// in-port must have EITHER an incoming edge XOR a `WorkflowInputBinding` targeting it — never both,
/// never neither; (2) every `input_bindings`/`output_bindings` entry must resolve to a real
/// `WorkflowInput`/node+port.
pub async fn validate_workflow_snapshot(document: &WorkflowSnapshot) -> WorkflowValidation {
    let mut validation = validate_workflow(&document.graph).await;

    let input_ids: HashSet<&str> = document.inputs.iter().map(|input| input.id.as_str()).collect();
    for binding in &document.input_bindings {
        if !input_ids.contains(binding.input_id.as_str()) {
            validation.errors.push(format!("input binding targets unknown input '{}'", binding.input_id));
        }
    }

    let node_by_id: HashMap<&str, &WorkflowNode> = document.graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    for binding in &document.output_bindings {
        let resolves = node_by_id.get(binding.node_id.as_str()).is_some_and(|node| node.outputs.iter().any(|port| port.id == binding.port_id));
        if !resolves {
            validation.errors.push(format!("output binding targets unknown node/port '{}:{}'", binding.node_id, binding.port_id));
        }
    }

    for node in &document.graph.nodes {
        for port in &node.inputs {
            if !port.spec.required {
                continue;
            }
            let has_edge = document.graph.edges.iter().any(|edge| edge.target_node_id == node.id && edge.target_port_id == port.id);
            let has_binding = document.input_bindings.iter().any(|binding| binding.node_id == node.id && binding.port_id == port.id);
            if has_edge && has_binding {
                validation.errors.push(format!("required port {} on node {} has both a wire and an input binding", port.id, node.id));
            } else if !has_edge && !has_binding {
                validation.errors.push(format!("required port {} on node {} has neither a wire nor an input binding", port.id, node.id));
            }
        }
    }

    validation.ok = validation.errors.is_empty();
    validation
}
//#endregion 🔖️WorkflowSnapshot

#[path = "../🚪️io/🦀️.rs"]
pub mod io;
