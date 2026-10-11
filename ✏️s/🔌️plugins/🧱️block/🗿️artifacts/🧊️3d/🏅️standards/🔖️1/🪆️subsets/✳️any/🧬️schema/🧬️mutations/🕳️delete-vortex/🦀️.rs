//! 🕳 Block3d mutation — `DeleteVortex`: a rim-vortex template.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🕳 `delete-vortex` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-vortex")]
pub struct DeleteVortex {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_vortex(id: String) -> Block3dMutation {
    Block3dMutation::DeleteVortex(DeleteVortex { id })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for DeleteVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "vortex", kind: "delete-vortex", record: "DeletedVortex" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete vortex \"{}\"", self.id), &format!("Wirbel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
