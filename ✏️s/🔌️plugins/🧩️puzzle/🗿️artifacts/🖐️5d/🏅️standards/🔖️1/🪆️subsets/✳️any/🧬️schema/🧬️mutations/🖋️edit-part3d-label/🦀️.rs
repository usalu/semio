//! Puzzle5d mutation — `EditPart3dLabel`: replaces a part's 3D-projection authored display label.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// `edit-part3d-label` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "edit-part3d-label")]
pub struct EditPart3dLabel {
    pub id: String,
    pub new_label: Option<String>,
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for EditPart3dLabel
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<EditPart3dLabel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "part", kind: "edit-part3d-label", record: "EditedPart3dLabel" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit part \"{}\" 3d label", self.id), &format!("3D-Beschriftung von Bauteil \"{}\" bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_part_3d_label<Op>(id: String, new_label: Option<String>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::EditPart3dLabel>,
{
    Op::from(EditPart3dLabel { id, new_label })
}
