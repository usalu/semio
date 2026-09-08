//! 🪤️ Energy model mutation — `RemoveSpaceListMember`: Takes one space back out of a grouping; the space itself survives.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪤️ `remove-space-list-member` payload. Takes one space back out of a grouping; the space itself survives.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-space-list-member")]
pub struct RemoveSpaceListMember {
    pub id: crate::model::EntityId,
    pub space_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_space_list_member(id: crate::model::EntityId, space_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemoveSpaceListMember(RemoveSpaceListMember { id, space_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveSpaceListMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "space-list", kind: "remove-space-list-member", record: "RemovedSpaceListMember" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove space {} from space list {}", self.space_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
