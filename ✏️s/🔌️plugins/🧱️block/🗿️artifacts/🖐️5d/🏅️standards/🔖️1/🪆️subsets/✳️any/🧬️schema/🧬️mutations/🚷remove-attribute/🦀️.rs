//! 🚷 Block5d mutation — `RemoveAttribute`: a free-form key/value attribute attachment.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🚷 `remove-attribute` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-attribute")]
pub struct RemoveAttribute {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_attribute<Op: From<crate::remove_attribute::RemoveAttribute>>(key: String) -> Op {
    Op::from(RemoveAttribute { key })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for RemoveAttribute where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::add_attribute::AddAttribute> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "attribute", kind: "remove-attribute", record: "RemovedAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove attribute \"{}\"", self.key), &format!("Attribut \"{}\" entfernen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for RemoveAttribute where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::add_attribute::AddAttribute> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::remove_attribute::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_attribute::inverse::inverse(self, base)?
    
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
