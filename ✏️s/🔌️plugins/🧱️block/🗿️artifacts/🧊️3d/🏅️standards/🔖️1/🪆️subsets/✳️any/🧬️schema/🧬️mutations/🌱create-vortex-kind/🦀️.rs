//! 🌱 Block3d mutation — `CreateVortexKind`: a new vortex-kind catalog row.

use crate::{Block3dSnapshot, Block3dVortexKind};
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🌱 `create-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
pub fn create_vortex_kind(vortex_kind: Block3dVortexKind) -> Block3dMutation {
    Block3dMutation::CreateVortexKind(CreateVortexKind { vortex_kind, index: None })
}

/// 📍️ Builder — like [`create_vortex_kind`] but inserts the row at `index`.
pub fn create_vortex_kind_at(vortex_kind: Block3dVortexKind, index: u32) -> Block3dMutation {
    Block3dMutation::CreateVortexKind(CreateVortexKind { vortex_kind, index: Some(index) })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex-kind", kind: "create-vortex-kind", record: "CreatedVortexKind" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
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
