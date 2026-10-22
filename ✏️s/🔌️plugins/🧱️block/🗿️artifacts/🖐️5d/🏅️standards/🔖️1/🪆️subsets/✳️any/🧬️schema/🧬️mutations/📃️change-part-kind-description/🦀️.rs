//! 📃️ Block5d mutation — `ChangePartKindDescription`: the part kind's `description`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 📃️ `change-part-kind-description` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-description")]
pub struct ChangePartKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_description<Op: From<crate::change_part_kind_description::ChangePartKindDescription>>(new_description: String) -> Op {
    Op::from(ChangePartKindDescription { new_description })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangePartKindDescription where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_part_kind_description::ChangePartKindDescription> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-description", record: "ChangedPartKindDescription" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change part kind description", "Bauteilartbeschreibung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for ChangePartKindDescription where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_part_kind_description::ChangePartKindDescription> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_part_kind_description::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_part_kind_description::inverse::inverse(self, base)?
    
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
