//! ✨️ Energy model mutation — `CreatePvSystem`: Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✨️ `create-pv-system` payload. Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-pv-system")]
pub struct CreatePvSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub dc_capacity_w: f64,
    pub area_m2: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub module_efficiency: f64,
    pub inverter_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_pv_system(index: u32, id: crate::model::EntityId, dc_capacity_w: f64, area_m2: f64, tilt_deg: f64, azimuth_deg: f64, module_efficiency: f64, inverter_efficiency: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreatePvSystem(CreatePvSystem { index, id, dc_capacity_w, area_m2, tilt_deg, azimuth_deg, module_efficiency, inverter_efficiency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreatePvSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "pv-system", kind: "create-pv-system", record: "CreatedPvSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Pv System {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
