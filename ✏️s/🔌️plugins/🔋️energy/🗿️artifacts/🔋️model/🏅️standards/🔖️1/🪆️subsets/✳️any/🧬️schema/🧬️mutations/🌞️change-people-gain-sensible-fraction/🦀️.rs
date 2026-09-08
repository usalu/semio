//! 🌞️ Energy model mutation — `ChangePeopleGainSensibleFraction`: Sets sensible fraction on one people gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌞️ `change-people-gain-sensible-fraction` payload. Sets sensible fraction on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-sensible-fraction")]
pub struct ChangePeopleGainSensibleFraction {
    pub id: crate::model::EntityId,
    pub new_sensible_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_people_gain_sensible_fraction(id: crate::model::EntityId, new_sensible_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePeopleGainSensibleFraction(ChangePeopleGainSensibleFraction { id, new_sensible_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePeopleGainSensibleFraction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-sensible-fraction", record: "ChangedPeopleGainSensibleFraction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change People Gain Sensible Fraction of people gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
