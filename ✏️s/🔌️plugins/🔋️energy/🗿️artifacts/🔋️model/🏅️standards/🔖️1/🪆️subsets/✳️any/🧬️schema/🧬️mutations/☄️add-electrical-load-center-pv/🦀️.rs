//! ☄️ Energy model mutation — `AddElectricalLoadCenterPv`: Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☄️ `add-electrical-load-center-pv` payload. Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-electrical-load-center-pv")]
pub struct AddElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_electrical_load_center_pv(id: crate::model::EntityId, index: u32, pv_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddElectricalLoadCenterPv(AddElectricalLoadCenterPv { id, index, pv_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddElectricalLoadCenterPv {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "electrical-load-center", kind: "add-electrical-load-center-pv", record: "AddedElectricalLoadCenterPv" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add pv system {} to electrical load center {}", self.pv_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
