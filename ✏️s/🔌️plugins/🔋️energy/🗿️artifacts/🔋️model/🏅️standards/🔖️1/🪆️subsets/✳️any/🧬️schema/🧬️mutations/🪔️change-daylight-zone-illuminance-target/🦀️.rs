//! 🪔️ Energy model mutation — `ChangeDaylightZoneIlluminanceTarget`: Sets the working-plane illuminance the electric lighting dims toward, in lux.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪔️ `change-daylight-zone-illuminance-target` payload. Sets the working-plane illuminance the electric lighting dims toward, in lux.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-illuminance-target")]
pub struct ChangeDaylightZoneIlluminanceTarget {
    pub id: crate::model::EntityId,
    pub new_illuminance_target_lux: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_daylight_zone_illuminance_target(id: crate::model::EntityId, new_illuminance_target_lux: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeDaylightZoneIlluminanceTarget(ChangeDaylightZoneIlluminanceTarget { id, new_illuminance_target_lux })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeDaylightZoneIlluminanceTarget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daylight-zone", kind: "change-daylight-zone-illuminance-target", record: "ChangedDaylightZoneIlluminanceTarget" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change daylight zone {} illuminance target to {:?}", self.id.0, self.new_illuminance_target_lux)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
