//! 🌲️ Energy model mutation — `CreateOutdoorAirSystem`: Puts an outdoor air system on one air loop: the minimum outdoor air flow it always draws and whether its economizer may open beyond that.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌲️ `create-outdoor-air-system` payload. Puts an outdoor air system on one air loop: the minimum outdoor air flow it always draws and whether its economizer may open beyond that.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-outdoor-air-system")]
pub struct CreateOutdoorAirSystem {
    pub id: crate::model::EntityId,
    pub air_loop_id: crate::model::EntityId,
    pub min_oa_flow_m3_s: f64,
    pub economizer_enabled: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_outdoor_air_system(id: crate::model::EntityId, air_loop_id: crate::model::EntityId, min_oa_flow_m3_s: f64, economizer_enabled: bool) -> EnergyModelMutation {
    EnergyModelMutation::CreateOutdoorAirSystem(CreateOutdoorAirSystem { id, air_loop_id, min_oa_flow_m3_s, economizer_enabled })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateOutdoorAirSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "outdoor-air-system", kind: "create-outdoor-air-system", record: "CreatedOutdoorAirSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create outdoor air system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
