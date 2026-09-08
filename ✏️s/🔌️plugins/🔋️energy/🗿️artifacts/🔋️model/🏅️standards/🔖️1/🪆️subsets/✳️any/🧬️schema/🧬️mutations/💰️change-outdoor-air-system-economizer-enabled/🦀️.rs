//! 💰️ Energy model mutation — `ChangeOutdoorAirSystemEconomizerEnabled`: Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💰️ `change-outdoor-air-system-economizer-enabled` payload. Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-economizer-enabled")]
pub struct ChangeOutdoorAirSystemEconomizerEnabled {
    pub id: crate::model::EntityId,
    pub new_economizer_enabled: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_outdoor_air_system_economizer_enabled(id: crate::model::EntityId, new_economizer_enabled: bool) -> EnergyModelMutation {
    EnergyModelMutation::ChangeOutdoorAirSystemEconomizerEnabled(ChangeOutdoorAirSystemEconomizerEnabled { id, new_economizer_enabled })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeOutdoorAirSystemEconomizerEnabled {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-economizer-enabled", record: "ChangedOutdoorAirSystemEconomizerEnabled" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change outdoor air system {} economizer setting to {:?}", self.id.0, self.new_economizer_enabled)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
