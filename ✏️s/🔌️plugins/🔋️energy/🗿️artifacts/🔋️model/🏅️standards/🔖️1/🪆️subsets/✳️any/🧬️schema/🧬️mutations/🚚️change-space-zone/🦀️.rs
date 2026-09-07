//! 🚚️ Energy model mutation — `ChangeSpaceZone`: Reassigns one space to another existing zone. `Space::zone_id` is a foreign key between two flat collections, not a recursive parent field, so this is the `change` verb rather than a hierarchy move.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚚️ `change-space-zone` payload. Reassigns one space to another existing zone. `Space::zone_id` is a foreign key between two flat collections, not a recursive parent field, so this is the `change` verb rather than a hierarchy move.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-space-zone")]
pub struct ChangeSpaceZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_space_zone(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSpaceZone(ChangeSpaceZone { id, new_zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSpaceZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "space", kind: "change-space-zone", record: "ChangedSpaceZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move space {} to zone {}", self.id.0, self.new_zone_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
