//! 🔏 Puzzle2d mutation — `ChangeTargetRegionLocked`: changes a region's locked flag.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔏 `change-target-region-locked` payload — changes a region's locked flag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-target-region-locked")]
pub struct ChangeTargetRegionLocked {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_target_region_locked<Op: From<ChangeTargetRegionLocked>>(id: PagedUtf8<{ usize::MAX }>, new_locked: bool) -> Op {
    Op::from(ChangeTargetRegionLocked { id, new_locked })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ChangeTargetRegionLocked
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ChangeTargetRegionLocked>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "target region", kind: "change-target-region-locked", record: "ChangedTargetRegionLocked" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change target region locked \"{}\"", self.id), &format!("Sperre von Zielregion \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for ChangeTargetRegionLocked
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<ChangeTargetRegionLocked>,
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
