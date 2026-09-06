//! 🌬️ Energy model mutation — `ChangeZoneConditioned`: Sets whether the zone is served by heating or cooling equipment at all.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌬️ `change-zone-conditioned` payload. Sets whether the zone is served by heating or cooling equipment at all.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-conditioned")]
pub struct ChangeZoneConditioned {
    pub id: crate::model::EntityId,
    pub new_conditioned: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_conditioned(id: crate::model::EntityId, new_conditioned: bool) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneConditioned(ChangeZoneConditioned { id, new_conditioned })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneConditioned {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone", kind: "change-zone-conditioned", record: "ChangedZoneConditioned" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone {} conditioned to {}", self.id.0, self.new_conditioned)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
