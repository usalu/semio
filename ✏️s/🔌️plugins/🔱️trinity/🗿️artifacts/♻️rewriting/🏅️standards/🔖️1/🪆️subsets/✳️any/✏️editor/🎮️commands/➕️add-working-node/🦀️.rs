//! ➕️ Trinity Rewriting app command — `add-working-node`: the guest's own add-node verb on the working (before) graph (design §13.3
//! of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING: adding a node is never a `nodeGraphEdit` row). It yields ONE `create-node`
//! leaf of the composed `workingGraph` child (design §20.15) carrying the first free `n<k>` id, the kind (the given one, else the
//! first node kind the graph's resolved manifest declares, else the kind of its first node), the name (the given one, else the id)
//! and the position.

use crate::content::{read, working_child_emit};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::app::ChildContentView;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{create_node::CreateNode, SemioGraphMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;

/// 🆔️ The first `n<k>` id (counting from the node count) none of `held` carries.
fn first_free_node_id(held: &[String]) -> String {
    (held.len()..).map(|k| format!("n{k}")).find(|id| !held.contains(id)).unwrap_or_default()
}

/// ➕️ ONE `create-node` leaf at (`x`, `y`) on the rule's working child; a non-finite position, a kind the resolved manifest does not
/// declare and a graph that names no node kind are refused by name.
pub(crate) fn add_working_node_command(state: &RewritingSnapshot, children: &ChildContentView, kind: Option<&str>, name: Option<&str>, x: f64, y: f64) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    if !x.is_finite() || !y.is_finite() {
        return Err(invalid("addWorkingNode needs a finite position".into()));
    }
    let work = read(state, children)?;
    let manifest = crate::editor::rewriting::resolved_working_manifest(state);
    let declared = manifest.as_ref().and_then(|manifest| manifest.node_kinds.first().map(|kind| kind.name.clone()));
    let kind = kind.map(str::trim).filter(|kind| !kind.is_empty()).map(str::to_string).or(declared).or_else(|| work.nodes.first().map(|node| node.kind.clone())).ok_or_else(|| invalid("the working graph names no node kind a node could carry".into()))?;
    if manifest.as_ref().is_some_and(|manifest| manifest.node_kind(&kind).is_none()) {
        return Err(invalid(format!("the working graph's manifest declares no node kind “{kind}”")));
    }
    let id = first_free_node_id(&work.nodes.iter().map(|node| node.id.value.clone()).collect::<Vec<_>>());
    let label = name.map(str::trim).filter(|name| !name.is_empty()).map_or_else(|| id.clone(), str::to_string);
    let leaf = SemioGraphMutation::CreateNode(CreateNode { id: GraphNodeId::new(id), kind, label, position: SemioPoint2 { x, y }, width: 0.0, height: 0.0, ports: Vec::new(), properties: Vec::new(), at: None });
    Ok(working_child_emit(state, &[leaf]))
}
