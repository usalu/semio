//! any edge whose `source`/`target` referenced this handle is severed too).
//! ➖ Puzzle2d mutation — `RemoveNodeHandle`: detaches a rim port from a node (captures cascade —

use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::AddNodeHandle;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// ➖ `remove-node-handle` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-node-handle")]
pub struct RemoveNodeHandle {
    pub node_id: PagedUtf8<{ usize::MAX }>,
    pub handle_id: PagedUtf8<{ usize::MAX }>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_node_handle<Op: From<RemoveNodeHandle>>(node_id: PagedUtf8<{ usize::MAX }>, handle_id: PagedUtf8<{ usize::MAX }>) -> Op {
    Op::from(RemoveNodeHandle { node_id, handle_id })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for RemoveNodeHandle
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<RemoveNodeHandle> + From<AddNodeHandle> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-handle", kind: "remove-node-handle", record: "RemovedNodeHandle" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove handle \"{}\" from node \"{}\"", self.handle_id, self.node_id), &format!("Griff \"{}\" aus Knoten \"{}\" entfernen", self.handle_id, self.node_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.to_string_owner(), self.handle_id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for RemoveNodeHandle
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<RemoveNodeHandle> + From<AddNodeHandle> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>,
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
