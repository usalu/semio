//! 🌗️ Energy model mutation — `DeleteDaylightZone`: Removes the daylighting control from its zone. Refused when no control carries the id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌗️ `delete-daylight-zone` payload. Removes the daylighting control from its zone. Refused when no control carries the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-daylight-zone")]
pub struct DeleteDaylightZone {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_daylight_zone(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteDaylightZone(DeleteDaylightZone { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteDaylightZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "daylight-zone", kind: "delete-daylight-zone", record: "DeletedDaylightZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete daylight zone {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
