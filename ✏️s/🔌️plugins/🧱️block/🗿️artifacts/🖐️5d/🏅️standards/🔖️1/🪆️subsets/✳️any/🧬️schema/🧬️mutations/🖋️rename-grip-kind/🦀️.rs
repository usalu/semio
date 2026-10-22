//! 🖋 Block5d mutation — `RenameGripKind`: a grip-kind catalog row's `name`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🖋 `rename-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-grip-kind")]
pub struct RenameGripKind {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_grip_kind<Op: From<crate::rename_grip_kind::RenameGripKind>>(id: String, new_name: String) -> Op {
    Op::from(RenameGripKind { id, new_name })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for RenameGripKind where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::rename_grip_kind::RenameGripKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "grip-kind", kind: "rename-grip-kind", record: "RenamedGripKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename grip kind \"{}\" to \"{}\"", self.id, self.new_name), &format!("Griffart \"{}\" in \"{}\" umbenennen", self.id, self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for RenameGripKind where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::rename_grip_kind::RenameGripKind> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::rename_grip_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_grip_kind::inverse::inverse(self, base)?
    
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
