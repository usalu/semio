//! 🚾️ Energy model mutation — `ChangeWaterSystemPeakFlow`: Sets peak flow (L/s) on one water system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚾️ `change-water-system-peak-flow` payload. Sets peak flow (L/s) on one water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-peak-flow")]
pub struct ChangeWaterSystemPeakFlow {
    pub id: crate::model::EntityId,
    pub new_peak_flow_l_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_water_system_peak_flow(id: crate::model::EntityId, new_peak_flow_l_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeWaterSystemPeakFlow(ChangeWaterSystemPeakFlow { id, new_peak_flow_l_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeWaterSystemPeakFlow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "water-system", kind: "change-water-system-peak-flow", record: "ChangedWaterSystemPeakFlow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Water System Peak Flow of water system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
