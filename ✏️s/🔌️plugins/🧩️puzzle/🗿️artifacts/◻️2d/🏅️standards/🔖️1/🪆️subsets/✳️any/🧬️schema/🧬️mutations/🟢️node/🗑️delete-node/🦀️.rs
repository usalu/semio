//! touching one of this node's handles is severed too, re-`connect-handles`ed by the inverse).
//! 🗑 Puzzle2d mutation — `DeleteNode`: removes an id-keyed node (captures cascade — any edge

use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🗑 `delete-node` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-node")]
pub struct DeleteNode {
    pub id: PagedUtf8<{ usize::MAX }>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_node<Op: From<DeleteNode>>(id: PagedUtf8<{ usize::MAX }>) -> Op {
    Op::from(DeleteNode { id })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for DeleteNode
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<DeleteNode> + From<CreateNode> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete node \"{}\"", self.id), &format!("Knoten \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for DeleteNode
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<DeleteNode> + From<CreateNode> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
 {
fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

