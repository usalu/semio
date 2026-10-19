//! 🏗️ Puzzle2d mutation — `ChangeNodeKind`: changes a node's `node_kind` catalog reference.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🏗️ `change-node-kind` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-kind")]
pub struct ChangeNodeKind {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_node_kind: Option<PagedUtf8<{ usize::MAX }>>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind<Op: From<ChangeNodeKind>>(id: PagedUtf8<{ usize::MAX }>, new_node_kind: Option<PagedUtf8<{ usize::MAX }>>) -> Op {
    Op::from(ChangeNodeKind { id, new_node_kind })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ChangeNodeKind
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ChangeNodeKind>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-kind", record: "ChangedNodeKind" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node \"{}\" kind", self.id), &format!("Art von Knoten \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for ChangeNodeKind
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<ChangeNodeKind>,
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
