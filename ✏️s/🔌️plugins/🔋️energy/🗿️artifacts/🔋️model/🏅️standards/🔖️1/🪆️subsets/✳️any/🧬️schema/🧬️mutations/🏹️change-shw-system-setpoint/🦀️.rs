//! 🏹️ Energy model mutation — `ChangeShwSystemSetpoint`: Sets tank setpoint (°C) on one service hot water system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏹️ `change-shw-system-setpoint` payload. Sets tank setpoint (°C) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-setpoint")]
pub struct ChangeShwSystemSetpoint {
    pub id: crate::model::EntityId,
    pub new_setpoint_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_shw_system_setpoint(id: crate::model::EntityId, new_setpoint_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeShwSystemSetpoint(ChangeShwSystemSetpoint { id, new_setpoint_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeShwSystemSetpoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-setpoint", record: "ChangedShwSystemSetpoint" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Shw System Setpoint of service hot water system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
