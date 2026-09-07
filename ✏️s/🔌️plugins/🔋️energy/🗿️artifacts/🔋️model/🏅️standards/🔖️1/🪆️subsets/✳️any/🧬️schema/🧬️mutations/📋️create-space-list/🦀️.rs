//! 📋️ Energy model mutation — `CreateSpaceList`: Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📋️ `create-space-list` payload. Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-space-list")]
pub struct CreateSpaceList {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub space_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_space_list(index: u32, id: crate::model::EntityId, name: String, space_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateSpaceList(CreateSpaceList { index, id, name, space_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSpaceList {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "space-list", kind: "create-space-list", record: "CreatedSpaceList" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Space List {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
