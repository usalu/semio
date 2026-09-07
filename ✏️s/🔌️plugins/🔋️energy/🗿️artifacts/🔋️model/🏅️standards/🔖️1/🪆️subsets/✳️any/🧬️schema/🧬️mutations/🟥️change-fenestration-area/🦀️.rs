//! 🟥️ Energy model mutation — `ChangeFenestrationArea`: Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟥️ `change-fenestration-area` payload. Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-area")]
pub struct ChangeFenestrationArea {
    pub id: crate::model::EntityId,
    pub new_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_area(id: crate::model::EntityId, new_area_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationArea(ChangeFenestrationArea { id, new_area_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-area", record: "ChangedFenestrationArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} area to {} m²", self.id.0, self.new_area_m2)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
