//! 🪦 Puzzle2d mutation — `DeleteTargetRegion`: removes an id-keyed fill target region.

use crate::standards::v1::subsets::any::schema::mutations::create_target_region::CreateTargetRegion;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🪦 `delete-target-region` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-target-region")]
pub struct DeleteTargetRegion {
    pub id: PagedUtf8<{ usize::MAX }>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_target_region<Op: From<DeleteTargetRegion>>(id: PagedUtf8<{ usize::MAX }>) -> Op {
    Op::from(DeleteTargetRegion { id })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for DeleteTargetRegion
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<DeleteTargetRegion> + From<CreateTargetRegion>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "target region", kind: "delete-target-region", record: "DeletedTargetRegion" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete target region \"{}\"", self.id), &format!("Zielregion \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for DeleteTargetRegion
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<DeleteTargetRegion> + From<CreateTargetRegion>,
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
