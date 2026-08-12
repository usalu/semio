//! ➕ Puzzle3d mutation — `AddObjectVortex`: attaches a new rim vortex to an object.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dSnapshot, Puzzle3dVortex};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-object-vortex` payload — owner object id + new vortex payload at an optional
/// FINAL-state `index` (`None` appends). A duplicate `vortex.id` on the same object is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-object-vortex")]
pub struct AddObjectVortex {
    pub object_id: String,
    #[dsl(block)]
    pub vortex: Puzzle3dVortex,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_object_vortex(object_id: String, vortex: Puzzle3dVortex, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::AddObjectVortex(AddObjectVortex { object_id, vortex, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for AddObjectVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "object-vortex", kind: "add-object-vortex", record: "AddedObjectVortex" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add vortex \"{}\" to object \"{}\"", self.vortex.id, self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex.id.clone()]
    }
}
//#endregion 🔖️Mutation
