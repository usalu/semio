//! 📐 Puzzle2d mutation — `ResizeTargetRegion`: absolute FINAL-state extent of a region.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 📐 `resize-target-region` payload — absolute FINAL-state extent of a region.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "resize-target-region")]
pub struct ResizeTargetRegion {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_target_region<Op: From<ResizeTargetRegion>>(id: PagedUtf8<{ usize::MAX }>, new_width: f64, new_height: f64) -> Op {
    Op::from(ResizeTargetRegion { id, new_width, new_height })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ResizeTargetRegion
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ResizeTargetRegion>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "target region", kind: "resize-target-region", record: "ResizedTargetRegion" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize target region \"{}\"", self.id), &format!("Größe von Zielregion \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for ResizeTargetRegion
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<ResizeTargetRegion>,
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
