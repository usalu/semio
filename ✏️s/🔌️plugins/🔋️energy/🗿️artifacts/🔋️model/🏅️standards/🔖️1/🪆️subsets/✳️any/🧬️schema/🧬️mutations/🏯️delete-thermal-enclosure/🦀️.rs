//! 🏯️ Energy model mutation — `DeleteThermalEnclosure`: Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏯️ `delete-thermal-enclosure` payload. Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-thermal-enclosure")]
pub struct DeleteThermalEnclosure {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_thermal_enclosure(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteThermalEnclosure(DeleteThermalEnclosure { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteThermalEnclosure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "thermal-enclosure", kind: "delete-thermal-enclosure", record: "DeletedThermalEnclosure" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Thermal Enclosure {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
