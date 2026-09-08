//! 🌘️ Energy model mutation — `RemoveElectricalLoadCenterPv`: Detaches one PV system from a load centre; the PV system itself survives, unattached.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌘️ `remove-electrical-load-center-pv` payload. Detaches one PV system from a load centre; the PV system itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-pv")]
pub struct RemoveElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_electrical_load_center_pv(id: crate::model::EntityId, pv_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemoveElectricalLoadCenterPv(RemoveElectricalLoadCenterPv { id, pv_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveElectricalLoadCenterPv {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "electrical-load-center", kind: "remove-electrical-load-center-pv", record: "RemovedElectricalLoadCenterPv" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove pv system {} from electrical load center {}", self.pv_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
