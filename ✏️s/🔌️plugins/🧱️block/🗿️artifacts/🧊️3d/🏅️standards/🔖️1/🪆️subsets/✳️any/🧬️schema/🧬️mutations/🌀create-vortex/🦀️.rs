//! 🌀 Block3d mutation — `CreateVortex`: a new rim-vortex template.

use crate::{Block3dSnapshot, Block3dVortexTemplate};
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🌀 `create-vortex` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-vortex")]
pub struct CreateVortex {
    #[dsl(block)]
    pub vortex: Block3dVortexTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_vortex(vortex: Block3dVortexTemplate) -> Block3dMutation {
    Block3dMutation::CreateVortex(CreateVortex { vortex })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex", kind: "create-vortex", record: "CreatedVortex" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create vortex \"{}\"", self.vortex.id), &format!("Wirbel \"{}\" erstellen", self.vortex.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.vortex.id.clone()]
    }
}
//#endregion 🔖️Mutation
