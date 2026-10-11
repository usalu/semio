//! 🔖 Block3d mutation — `AddRepresentationTag`: a member of a representation's `tags` set.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🔖 `add-representation-tag` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-representation-tag")]
pub struct AddRepresentationTag {
    pub id: String,
    pub tag: String,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_representation_tag(id: String, tag: String) -> Block3dMutation {
    Block3dMutation::AddRepresentationTag(AddRepresentationTag { id, tag, index: None })
}

/// 📍️ Builder — like [`add_representation_tag`] but inserts the row at `index`.
pub fn add_representation_tag_at(id: String, tag: String, index: u32) -> Block3dMutation {
    Block3dMutation::AddRepresentationTag(AddRepresentationTag { id, tag, index: Some(index) })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddRepresentationTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-tag", kind: "add-representation-tag", record: "AddedRepresentationTag" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add tag \"{}\" to representation \"{}\"", self.tag, self.id), &format!("Tag \"{}\" zu Repräsentation \"{}\" hinzufügen", self.tag, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
