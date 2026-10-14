//! 🌱 Block3d mutation — `CreateVortexKind`: a new vortex-kind catalog row.

use semio_s_artifact_block_3d_model::{Block3dSnapshot, Block3dVortexKind};
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🌱 `create-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-vortex-kind")]
pub struct CreateVortexKind {
    #[dsl(block)]
    pub vortex_kind: Block3dVortexKind,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_vortex_kind<Op: From<crate::create_vortex_kind::CreateVortexKind>>(vortex_kind: Block3dVortexKind) -> Op {
    Op::from(CreateVortexKind { vortex_kind, index: None })
}

/// 📍️ Builder — like [`create_vortex_kind`] but inserts the row at `index`.
pub fn create_vortex_kind_at<Op: From<crate::create_vortex_kind::CreateVortexKind>>(vortex_kind: Block3dVortexKind, index: u32) -> Op {
    Op::from(CreateVortexKind { vortex_kind, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for CreateVortexKind where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::delete_vortex_kind::DeleteVortexKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex-kind", kind: "create-vortex-kind", record: "CreatedVortexKind" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::create_vortex_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_vortex_kind::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create vortex kind \"{}\"", self.vortex_kind.id), &format!("Wirbelart \"{}\" erstellen", self.vortex_kind.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.vortex_kind.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
