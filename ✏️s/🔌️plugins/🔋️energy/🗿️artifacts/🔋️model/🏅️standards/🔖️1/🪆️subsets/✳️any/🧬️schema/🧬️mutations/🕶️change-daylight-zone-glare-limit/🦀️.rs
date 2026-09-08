//! 🕶️ Energy model mutation — `ChangeDaylightZoneGlareLimit`: Sets the maximum discomfort glare index the control tolerates.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕶️ `change-daylight-zone-glare-limit` payload. Sets the maximum discomfort glare index the control tolerates.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-glare-limit")]
pub struct ChangeDaylightZoneGlareLimit {
    pub id: crate::model::EntityId,
    pub new_glare_limit: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_daylight_zone_glare_limit(id: crate::model::EntityId, new_glare_limit: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeDaylightZoneGlareLimit(ChangeDaylightZoneGlareLimit { id, new_glare_limit })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeDaylightZoneGlareLimit {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daylight-zone", kind: "change-daylight-zone-glare-limit", record: "ChangedDaylightZoneGlareLimit" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change daylight zone {} glare limit to {:?}", self.id.0, self.new_glare_limit)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
