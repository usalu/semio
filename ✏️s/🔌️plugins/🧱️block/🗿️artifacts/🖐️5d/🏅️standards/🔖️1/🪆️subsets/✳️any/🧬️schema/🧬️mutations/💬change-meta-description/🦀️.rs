//! 💬 Block5d mutation — `ChangeMetaDescription`: the editing-session `meta.description`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 💬 `change-meta-description` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-meta-description")]
pub struct ChangeMetaDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_meta_description<Op: From<crate::change_meta_description::ChangeMetaDescription>>(new_description: String) -> Op {
    Op::from(ChangeMetaDescription { new_description })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangeMetaDescription where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_meta_description::ChangeMetaDescription> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "meta", kind: "change-meta-description", record: "ChangedMetaDescription" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change meta description", "Metabeschreibung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for ChangeMetaDescription where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_meta_description::ChangeMetaDescription> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_meta_description::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_meta_description::inverse::inverse(self, base)?
    
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
