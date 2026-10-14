//! 🌱️ Block2d mutation — `CreateHandleKind`: a new handle-kind catalog row.

use semio_s_artifact_block_2d_model::{Block2dHandleKind, Block2dSnapshot};
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🌱️ `create-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-handle-kind")]
pub struct CreateHandleKind {
    #[dsl(block)]
    pub handle_kind: Block2dHandleKind,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_handle_kind<Op: From<crate::create_handle_kind::CreateHandleKind>>(handle_kind: Block2dHandleKind) -> Op {
    Op::from(CreateHandleKind { handle_kind, index: None })
}

/// 📍️ Builder — like [`create_handle_kind`] but inserts the row at `index`.
pub fn create_handle_kind_at<Op: From<crate::create_handle_kind::CreateHandleKind>>(handle_kind: Block2dHandleKind, index: u32) -> Op {
    Op::from(CreateHandleKind { handle_kind, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for CreateHandleKind where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::delete_handle_kind::DeleteHandleKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "handle-kind", kind: "create-handle-kind", record: "CreatedHandleKind" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::create_handle_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_handle_kind::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create handle kind \"{}\"", self.handle_kind.id), &format!("Griffart \"{}\" erstellen", self.handle_kind.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.handle_kind.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
