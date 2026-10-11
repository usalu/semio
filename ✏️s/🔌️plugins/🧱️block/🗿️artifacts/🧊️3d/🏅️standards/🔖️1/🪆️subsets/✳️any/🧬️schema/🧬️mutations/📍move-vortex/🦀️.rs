//! 📍 Block3d mutation — `MoveVortex`: a vortex's position + facing direction together.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 📍 `move-vortex` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-vortex")]
pub struct MoveVortex {
    pub id: String,
    pub new_position: [f64; 3],
    pub new_direction: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_vortex(id: String, new_position: [f64; 3], new_direction: [f64; 3]) -> Block3dMutation {
    Block3dMutation::MoveVortex(MoveVortex { id, new_position, new_direction })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for MoveVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vortex", kind: "move-vortex", record: "MovedVortex" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move vortex \"{}\"", self.id), &format!("Wirbel \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
