//! 🌐️ Energy model mutation — `ChangeFenestrationUValue`: Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌐️ `change-fenestration-u-value` payload. Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-u-value")]
pub struct ChangeFenestrationUValue {
    pub id: crate::model::EntityId,
    pub new_u_value_w_m2k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_u_value(id: crate::model::EntityId, new_u_value_w_m2k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationUValue(ChangeFenestrationUValue { id, new_u_value_w_m2k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationUValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-u-value", record: "ChangedFenestrationUValue" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} U-value to {} W/(m²·K)", self.id.0, self.new_u_value_w_m2k)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
