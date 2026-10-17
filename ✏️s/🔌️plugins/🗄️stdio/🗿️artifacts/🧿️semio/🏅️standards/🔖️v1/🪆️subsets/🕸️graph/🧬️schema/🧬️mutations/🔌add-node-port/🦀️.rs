//! 🔌 `add-node-port` — attaches one port to a node at a FINAL-state index within that node's
//! `ports` (an intrinsically ordered, anonymous collection nested one level inside `nodes` — mirrors
//! `🔤️text`'s `add-mark` exactly).

use crate::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddNodePort {
    pub node_id: GraphNodeId,
    pub index: usize,
    pub port: SemioGraphPort,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for AddNodePort
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<AddNodePort> + From<crate::schema::mutations::remove_node_port::RemoveNodePort>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "node-port", kind: "add-node-port", record: "AddedNodePort" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add port to node \"{}\" at #{}", self.node_id.value, self.index), &format!("Anschluss zu Knoten \"{}\" an #{} hinzufügen", self.node_id.value, self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
