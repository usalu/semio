//! 🎛️ `set-node-property` — sets the value of one EXISTING keyed property of a node to an absolute typed value (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §19.3, §20.15). Composed editors (dag, reasoning/wires) keep their
//! per-node fields as keyed graph properties, so a field edit is this one leaf and history edits the typed value directly;
//! a key the node lacks is attached by `add-node-property` and detached by `remove-node-property`.

use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;

//#region 🔖️Payload
/// 🎛️ `set-node-property` payload — the node, the key of its property and the value that property takes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetNodeProperty {
    pub node_id: GraphNodeId,
    pub key: String,
    pub value: SemioValue,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for SetNodeProperty
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<SetNodeProperty>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-property", kind: "set-node-property", record: "SetNodeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (key, owner) = (&self.key, &self.node_id.value);
        match semio_s_artifact_stdio_semio_value_model::schema::snapshot::semio_value_scalar_label(&self.value) {
            Some((en, de)) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of node \"{owner}\" to {en}"), &format!("\"{key}\" von Knoten \"{owner}\" auf {de} setzen")),
            None => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set \"{key}\" of node \"{owner}\""), &format!("\"{key}\" von Knoten \"{owner}\" setzen")),
        }
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone()]
    }
}
//#endregion 🔖️Payload
