//! 🌱 Block5d mutation — `CreateGripKind`: a new grip-kind catalog row.

use semio_s_artifact_block_5d_model::{Block5dGripKind, Block5dSnapshot};
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🌱 `create-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-grip-kind")]
pub struct CreateGripKind {
    #[dsl(block)]
    pub grip_kind: Block5dGripKind,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_grip_kind<Op: From<crate::create_grip_kind::CreateGripKind>>(grip_kind: Block5dGripKind) -> Op {
    Op::from(CreateGripKind { grip_kind, index: None })
}

/// 📍️ Builder — like [`create_grip_kind`] but inserts the row at `index`.
pub fn create_grip_kind_at<Op: From<crate::create_grip_kind::CreateGripKind>>(grip_kind: Block5dGripKind, index: u32) -> Op {
    Op::from(CreateGripKind { grip_kind, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for CreateGripKind where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::delete_grip_kind::DeleteGripKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "grip-kind", kind: "create-grip-kind", record: "CreatedGripKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create grip kind \"{}\"", self.grip_kind.id), &format!("Griffart \"{}\" erstellen", self.grip_kind.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.grip_kind.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for CreateGripKind where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::delete_grip_kind::DeleteGripKind> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::create_grip_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_grip_kind::inverse::inverse(self, base)?
    
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
