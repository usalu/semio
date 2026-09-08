//! 🌇️ Energy model mutation — `ChangeFenestrationShgc`: Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌇️ `change-fenestration-shgc` payload. Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-shgc")]
pub struct ChangeFenestrationShgc {
    pub id: crate::model::EntityId,
    pub new_shgc: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_shgc(id: crate::model::EntityId, new_shgc: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationShgc(ChangeFenestrationShgc { id, new_shgc })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationShgc {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-shgc", record: "ChangedFenestrationShgc" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} SHGC to {}", self.id.0, self.new_shgc)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
