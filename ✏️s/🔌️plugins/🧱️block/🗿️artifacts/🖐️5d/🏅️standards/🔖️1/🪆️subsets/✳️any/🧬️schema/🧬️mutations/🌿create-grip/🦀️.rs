//! 🌿 Block5d mutation — `CreateGrip`: a new rim-grip template.

use semio_s_artifact_block_5d_model::{Block5dGripTemplate, Block5dSnapshot};
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🌿 `create-grip` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-grip")]
pub struct CreateGrip {
    #[dsl(block)]
    pub grip: Block5dGripTemplate,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_grip<Op: From<crate::create_grip::CreateGrip>>(grip: Block5dGripTemplate) -> Op {
    Op::from(CreateGrip { grip, index: None })
}

/// 📍️ Builder — like [`create_grip`] but inserts the row at `index`.
pub fn create_grip_at<Op: From<crate::create_grip::CreateGrip>>(grip: Block5dGripTemplate, index: u32) -> Op {
    Op::from(CreateGrip { grip, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for CreateGrip where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::delete_grip::DeleteGrip> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "grip", kind: "create-grip", record: "CreatedGrip" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create grip \"{}\"", self.grip.id), &format!("Griff \"{}\" erstellen", self.grip.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.grip.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for CreateGrip where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::delete_grip::DeleteGrip> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::create_grip::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_grip::inverse::inverse(self, base)?
    
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
