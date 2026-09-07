//! ➡️ Energy model mutation — `AddSpaceListMember`: Puts one space into a grouping.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➡️ `add-space-list-member` payload. Puts one space into a grouping.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-space-list-member")]
pub struct AddSpaceListMember {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub space_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_space_list_member(id: crate::model::EntityId, index: u32, space_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddSpaceListMember(AddSpaceListMember { id, index, space_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddSpaceListMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "space-list", kind: "add-space-list-member", record: "AddedSpaceListMember" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add space {} to space list {}", self.space_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
