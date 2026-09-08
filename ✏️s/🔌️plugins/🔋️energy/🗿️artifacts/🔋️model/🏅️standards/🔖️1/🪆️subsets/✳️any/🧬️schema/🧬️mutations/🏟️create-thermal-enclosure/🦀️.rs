//! 🏟️ Energy model mutation — `CreateThermalEnclosure`: Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏟️ `create-thermal-enclosure` payload. Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-thermal-enclosure")]
pub struct CreateThermalEnclosure {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_thermal_enclosure(index: u32, id: crate::model::EntityId, name: String, zone_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateThermalEnclosure(CreateThermalEnclosure { index, id, name, zone_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateThermalEnclosure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "thermal-enclosure", kind: "create-thermal-enclosure", record: "CreatedThermalEnclosure" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Thermal Enclosure {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
