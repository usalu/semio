//! ✖️ Energy model mutation — `ChangeZoneMultiplier`: Sets how many identical instances of the zone the results are scaled by.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✖️ `change-zone-multiplier` payload. Sets how many identical instances of the zone the results are scaled by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-multiplier")]
pub struct ChangeZoneMultiplier {
    pub id: crate::model::EntityId,
    pub new_multiplier: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_multiplier(id: crate::model::EntityId, new_multiplier: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneMultiplier(ChangeZoneMultiplier { id, new_multiplier })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneMultiplier {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone", kind: "change-zone-multiplier", record: "ChangedZoneMultiplier" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone {} multiplier to {}", self.id.0, self.new_multiplier)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
