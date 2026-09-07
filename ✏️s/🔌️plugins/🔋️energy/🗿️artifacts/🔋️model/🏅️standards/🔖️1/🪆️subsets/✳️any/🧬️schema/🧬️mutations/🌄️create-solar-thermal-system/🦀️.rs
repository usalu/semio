//! 🌄️ Energy model mutation — `CreateSolarThermalSystem`: Adds one solar thermal collector loop: aperture, conversion efficiency, the buffer it charges, and the plane the incident-solar model integrates over.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌄️ `create-solar-thermal-system` payload. Adds one solar thermal collector loop: aperture, conversion efficiency, the buffer it charges, and the plane the incident-solar model integrates over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-solar-thermal-system")]
pub struct CreateSolarThermalSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub collector_area_m2: f64,
    pub efficiency: f64,
    pub storage_volume_m3: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_solar_thermal_system(index: u32, id: crate::model::EntityId, collector_area_m2: f64, efficiency: f64, storage_volume_m3: f64, tilt_deg: f64, azimuth_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateSolarThermalSystem(CreateSolarThermalSystem { index, id, collector_area_m2, efficiency, storage_volume_m3, tilt_deg, azimuth_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSolarThermalSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "solar-thermal-system", kind: "create-solar-thermal-system", record: "CreatedSolarThermalSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Solar Thermal System {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
