//! 🧩 Block5d mutation — `AddRepresentationAttribute`: a member of a representation's nested `attributes`.

use semio_s_plugin_block::BlockAttribute;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🧩 `add-representation-attribute` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-representation-attribute")]
pub struct AddRepresentationAttribute {
    pub id: String,
    #[dsl(block)]
    pub attribute: BlockAttribute,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_representation_attribute<Op: From<crate::add_representation_attribute::AddRepresentationAttribute>>(id: String, attribute: BlockAttribute) -> Op {
    Op::from(AddRepresentationAttribute { id, attribute, index: None })
}

/// 📍️ Builder — like [`add_representation_attribute`] but inserts the row at `index`.
pub fn add_representation_attribute_at<Op: From<crate::add_representation_attribute::AddRepresentationAttribute>>(id: String, attribute: BlockAttribute, index: u32) -> Op {
    Op::from(AddRepresentationAttribute { id, attribute, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for AddRepresentationAttribute where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::remove_representation_attribute::RemoveRepresentationAttribute> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-attribute", kind: "add-representation-attribute", record: "AddedRepresentationAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add attribute \"{}\" to representation \"{}\"", self.attribute.key, self.id), &format!("Attribut \"{}\" zu Repräsentation \"{}\" hinzufügen", self.attribute.key, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for AddRepresentationAttribute where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::remove_representation_attribute::RemoveRepresentationAttribute> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::add_representation_attribute::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_representation_attribute::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
