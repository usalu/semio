//! 🥃️ Energy model mutation — `ChangeDaylightZoneWindowTransmittance`: Sets the visible transmittance the daylight control sees its window through, as a fraction.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥃️ `change-daylight-zone-window-transmittance` payload. Sets the visible transmittance the daylight control sees its window through, as a fraction.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-window-transmittance")]
pub struct ChangeDaylightZoneWindowTransmittance {
    pub id: crate::model::EntityId,
    pub new_window_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_daylight_zone_window_transmittance(id: crate::model::EntityId, new_window_transmittance: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeDaylightZoneWindowTransmittance(ChangeDaylightZoneWindowTransmittance { id, new_window_transmittance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeDaylightZoneWindowTransmittance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daylight-zone", kind: "change-daylight-zone-window-transmittance", record: "ChangedDaylightZoneWindowTransmittance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change daylight zone {} window transmittance to {:?}", self.id.0, self.new_window_transmittance)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
