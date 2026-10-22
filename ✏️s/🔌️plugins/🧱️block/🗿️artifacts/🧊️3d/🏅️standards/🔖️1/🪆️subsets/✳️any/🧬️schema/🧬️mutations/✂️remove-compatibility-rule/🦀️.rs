//! ✂ Block3d mutation — `RemoveCompatibilityRule`: a compatibility rule attachment.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// ✂ `remove-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-compatibility-rule")]
pub struct RemoveCompatibilityRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_compatibility_rule<Op: From<crate::remove_compatibility_rule::RemoveCompatibilityRule>>(id: String) -> Op {
    Op::from(RemoveCompatibilityRule { id })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for RemoveCompatibilityRule where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_compatibility_rule::AddCompatibilityRule> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "compatibility-rule", kind: "remove-compatibility-rule", record: "RemovedCompatibilityRule" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove compatibility rule \"{}\"", self.id), &format!("Kompatibilitätsregel \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for RemoveCompatibilityRule where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_compatibility_rule::AddCompatibilityRule> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::remove_compatibility_rule::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_compatibility_rule::inverse::inverse(self, base)?
    
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
