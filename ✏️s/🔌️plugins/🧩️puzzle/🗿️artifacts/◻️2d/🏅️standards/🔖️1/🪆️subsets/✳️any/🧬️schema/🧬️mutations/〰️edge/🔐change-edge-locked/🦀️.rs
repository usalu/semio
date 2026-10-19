//! 🧩️ Puzzle2d mutation — `ChangeEdgeLocked`: changes an edge's locked flag.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔗️ `change-edge-locked` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-edge-locked")]
pub struct ChangeEdgeLocked {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_locked: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_edge_locked<Op: From<ChangeEdgeLocked>>(id: PagedUtf8<{ usize::MAX }>, new_locked: Option<bool>) -> Op {
    Op::from(ChangeEdgeLocked { id, new_locked })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ChangeEdgeLocked
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ChangeEdgeLocked>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edge", kind: "change-edge-locked", record: "ChangedEdgeLocked" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change edge \"{}\" locked", self.id), &format!("Sperre von Kante \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for ChangeEdgeLocked
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<ChangeEdgeLocked>,
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
