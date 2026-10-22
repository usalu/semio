//! 🌀 Block3d mutation — `CreateVortex`: a new rim-vortex template.

use semio_s_artifact_block_3d_model::{Block3dSnapshot, Block3dVortexTemplate};
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🌀 `create-vortex` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-vortex")]
pub struct CreateVortex {
    #[dsl(block)]
    pub vortex: Block3dVortexTemplate,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_vortex<Op: From<crate::create_vortex::CreateVortex>>(vortex: Block3dVortexTemplate) -> Op {
    Op::from(CreateVortex { vortex, index: None })
}

/// 📍️ Builder — like [`create_vortex`] but inserts the row at `index`.
pub fn create_vortex_at<Op: From<crate::create_vortex::CreateVortex>>(vortex: Block3dVortexTemplate, index: u32) -> Op {
    Op::from(CreateVortex { vortex, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for CreateVortex where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::delete_vortex::DeleteVortex> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex", kind: "create-vortex", record: "CreatedVortex" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create vortex \"{}\"", self.vortex.id), &format!("Wirbel \"{}\" erstellen", self.vortex.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.vortex.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for CreateVortex where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::delete_vortex::DeleteVortex> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::create_vortex::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_vortex::inverse::inverse(self, base)?
    
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
