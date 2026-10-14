//! 🔖 Block5d mutation — `AddRepresentationTag`: a member of a representation's `tags` set.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

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
pub fn add_representation_tag<Op: From<crate::add_representation_tag::AddRepresentationTag>>(id: String, tag: String) -> Op {
    Op::from(AddRepresentationTag { id, tag, index: None })
}

/// 📍️ Builder — like [`add_representation_tag`] but inserts the row at `index`.
pub fn add_representation_tag_at<Op: From<crate::add_representation_tag::AddRepresentationTag>>(id: String, tag: String, index: u32) -> Op {
    Op::from(AddRepresentationTag { id, tag, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for AddRepresentationTag where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::remove_representation_tag::RemoveRepresentationTag> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-tag", kind: "add-representation-tag", record: "AddedRepresentationTag" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::add_representation_tag::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_representation_tag::inverse::inverse(self, base)?
    
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

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
