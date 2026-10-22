//! 🌿️ Block2d mutation — `CreateHandle`: a new rim-handle template.

use semio_s_artifact_block_2d_model::{Block2dHandleTemplate, Block2dSnapshot};
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🌿️ `create-handle` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-handle")]
pub struct CreateHandle {
    #[dsl(block)]
    pub handle: Block2dHandleTemplate,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_handle<Op: From<crate::create_handle::CreateHandle>>(handle: Block2dHandleTemplate) -> Op {
    Op::from(CreateHandle { handle, index: None })
}

/// 📍️ Builder — like [`create_handle`] but inserts the row at `index`.
pub fn create_handle_at<Op: From<crate::create_handle::CreateHandle>>(handle: Block2dHandleTemplate, index: u32) -> Op {
    Op::from(CreateHandle { handle, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for CreateHandle where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::delete_handle::DeleteHandle> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "handle", kind: "create-handle", record: "CreatedHandle" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create handle \"{}\"", self.handle.id), &format!("Griff \"{}\" erstellen", self.handle.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.handle.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for CreateHandle where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::delete_handle::DeleteHandle> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::create_handle::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_handle::inverse::inverse(self, base)?
    
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
