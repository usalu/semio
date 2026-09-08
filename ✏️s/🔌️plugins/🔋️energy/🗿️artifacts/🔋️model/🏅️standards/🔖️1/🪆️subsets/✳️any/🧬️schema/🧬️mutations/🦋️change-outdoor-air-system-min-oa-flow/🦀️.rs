//! 🦋️ Energy model mutation — `ChangeOutdoorAirSystemMinOaFlow`: Sets the outdoor air flow the system always draws, in m³/s.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🦋️ `change-outdoor-air-system-min-oa-flow` payload. Sets the outdoor air flow the system always draws, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-min-oa-flow")]
pub struct ChangeOutdoorAirSystemMinOaFlow {
    pub id: crate::model::EntityId,
    pub new_min_oa_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_outdoor_air_system_min_oa_flow(id: crate::model::EntityId, new_min_oa_flow_m3_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeOutdoorAirSystemMinOaFlow(ChangeOutdoorAirSystemMinOaFlow { id, new_min_oa_flow_m3_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeOutdoorAirSystemMinOaFlow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-min-oa-flow", record: "ChangedOutdoorAirSystemMinOaFlow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change outdoor air system {} minimum outdoor air flow to {:?}", self.id.0, self.new_min_oa_flow_m3_s)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
