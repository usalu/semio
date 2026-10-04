//! 🎛️ `set-node-property` — sets the value of one EXISTING keyed property of a node to an absolute typed value (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §19.3, §20.15). Composed editors (dag, reasoning/wires) keep their
//! per-node fields as keyed graph properties, so a field edit is this one leaf and history edits the typed value directly;
//! a key the node lacks is attached by `add-node-property` and detached by `remove-node-property`.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Payload
/// 🎛️ `set-node-property` payload — the node, the key of its property and the value that property takes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetNodeProperty {
    pub node_id: GraphNodeId,
    pub key: String,
    pub value: SemioValue,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for SetNodeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-property", kind: "set-node-property", record: "SetNodeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (key, owner) = (&self.key, &self.node_id.value);
        match crate::standards::v1::subsets::value::schema::snapshot::semio_value_scalar_label(&self.value) {
            Some((en, de)) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of node \"{owner}\" to {en}"), &format!("\"{key}\" von Knoten \"{owner}\" auf {de} setzen")),
            None => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of node \"{owner}\""), &format!("\"{key}\" von Knoten \"{owner}\" setzen")),
        }
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone()]
    }
}
//#endregion 🔖️Payload
