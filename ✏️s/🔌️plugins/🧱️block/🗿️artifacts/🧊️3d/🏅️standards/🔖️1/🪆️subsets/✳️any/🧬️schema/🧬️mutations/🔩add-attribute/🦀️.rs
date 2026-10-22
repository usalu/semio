//! 🔩 Block3d mutation — `AddAttribute`: a free-form key/value attribute attachment.

use semio_s_plugin_block::BlockAttribute;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🔩 `add-attribute` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-attribute")]
pub struct AddAttribute {
    #[dsl(block)]
    pub attribute: BlockAttribute,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_attribute<Op: From<crate::add_attribute::AddAttribute>>(attribute: BlockAttribute) -> Op {
    Op::from(AddAttribute { attribute, index: None })
}

/// 📍️ Builder — like [`add_attribute`] but inserts the row at `index`.
pub fn add_attribute_at<Op: From<crate::add_attribute::AddAttribute>>(attribute: BlockAttribute, index: u32) -> Op {
    Op::from(AddAttribute { attribute, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for AddAttribute where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::remove_attribute::RemoveAttribute> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "attribute", kind: "add-attribute", record: "AddedAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add attribute \"{}\"", self.attribute.key), &format!("Attribut \"{}\" hinzufügen", self.attribute.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.attribute.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for AddAttribute where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::remove_attribute::RemoveAttribute> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::add_attribute::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_attribute::inverse::inverse(self, base)?
    
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
