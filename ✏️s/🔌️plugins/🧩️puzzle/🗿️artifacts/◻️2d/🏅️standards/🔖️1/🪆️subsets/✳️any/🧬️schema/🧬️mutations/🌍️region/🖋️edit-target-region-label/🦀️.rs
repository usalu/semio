//! 🖋️ Puzzle2d mutation — `EditTargetRegionLabel`: renames a region (`None` clears the label).

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🖋️ `edit-target-region-label` payload — renames a region (`None` clears the label).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "edit-target-region-label")]
pub struct EditTargetRegionLabel {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_label: Option<PagedUtf8<{ usize::MAX }>>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_target_region_label<Op: From<EditTargetRegionLabel>>(id: PagedUtf8<{ usize::MAX }>, new_label: Option<PagedUtf8<{ usize::MAX }>>) -> Op {
    Op::from(EditTargetRegionLabel { id, new_label })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for EditTargetRegionLabel
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<EditTargetRegionLabel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "target region", kind: "edit-target-region-label", record: "EditedTargetRegionLabel" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit target region label \"{}\"", self.id), &format!("Beschriftung von Zielregion \"{}\" bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
