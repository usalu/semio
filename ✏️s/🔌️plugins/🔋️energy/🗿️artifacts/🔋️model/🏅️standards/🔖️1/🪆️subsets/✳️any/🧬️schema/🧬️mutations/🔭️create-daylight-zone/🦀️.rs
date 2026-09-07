//! 🔭️ Energy model mutation — `CreateDaylightZone`: Puts a daylighting control on one zone: an illuminance target the electric lighting dims toward, a glare index limit and the visible transmittance the control sees the window through.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔭️ `create-daylight-zone` payload. Puts a daylighting control on one zone: an illuminance target the electric lighting dims toward, a glare index limit and the visible transmittance the control sees the window through.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-daylight-zone")]
pub struct CreateDaylightZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub illuminance_target_lux: f64,
    pub glare_limit: f64,
    pub window_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_daylight_zone(id: crate::model::EntityId, zone_id: crate::model::EntityId, illuminance_target_lux: f64, glare_limit: f64, window_transmittance: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateDaylightZone(CreateDaylightZone { id, zone_id, illuminance_target_lux, glare_limit, window_transmittance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateDaylightZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "daylight-zone", kind: "create-daylight-zone", record: "CreatedDaylightZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create daylight zone {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
