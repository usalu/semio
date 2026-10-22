//! 🔚 `remove-node-port` — detaches one port from a node, addressed by BASE-state
//! `{node_id, index}`.

use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNodePort {
    pub node_id: GraphNodeId,
    pub index: usize,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for RemoveNodePort
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<RemoveNodePort> + From<crate::schema::mutations::add_node_port::AddNodePort>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-port", kind: "remove-node-port", record: "RemovedNodePort" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove port #{} from node \"{}\"", self.index, self.node_id.value), &format!("Anschluss #{} aus Knoten \"{}\" entfernen", self.index, self.node_id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioGraphSnapshot, Op> for RemoveNodePort
where
    Op: protocol::ColdMutation<SemioGraphSnapshot, ReferenceDiff = crate::schema::diff::SemioGraphDiff> + From<RemoveNodePort> + From<crate::schema::mutations::add_node_port::AddNodePort>,
{
    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioGraphSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
