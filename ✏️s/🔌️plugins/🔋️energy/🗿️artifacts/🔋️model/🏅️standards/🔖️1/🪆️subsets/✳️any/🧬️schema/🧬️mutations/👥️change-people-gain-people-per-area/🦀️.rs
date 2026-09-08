//! 👥️ Energy model mutation — `ChangePeopleGainPeoplePerArea`: Sets occupant density (people/m²) on one people gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 👥️ `change-people-gain-people-per-area` payload. Sets occupant density (people/m²) on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-people-per-area")]
pub struct ChangePeopleGainPeoplePerArea {
    pub id: crate::model::EntityId,
    pub new_people_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_people_gain_people_per_area(id: crate::model::EntityId, new_people_per_area: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePeopleGainPeoplePerArea(ChangePeopleGainPeoplePerArea { id, new_people_per_area })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePeopleGainPeoplePerArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-people-per-area", record: "ChangedPeopleGainPeoplePerArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change People Gain People Per Area of people gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
