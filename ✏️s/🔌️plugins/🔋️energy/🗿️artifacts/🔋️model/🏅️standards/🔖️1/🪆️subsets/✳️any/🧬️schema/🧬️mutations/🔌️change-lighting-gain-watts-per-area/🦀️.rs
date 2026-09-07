//! 🔌️ Energy model mutation — `ChangeLightingGainWattsPerArea`: Sets installed power density (W/m²) on one lighting gain, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔌️ `change-lighting-gain-watts-per-area` payload. Sets installed power density (W/m²) on one lighting gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-watts-per-area")]
pub struct ChangeLightingGainWattsPerArea {
    pub id: crate::model::EntityId,
    pub new_watts_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_lighting_gain_watts_per_area(id: crate::model::EntityId, new_watts_per_area: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeLightingGainWattsPerArea(ChangeLightingGainWattsPerArea { id, new_watts_per_area })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeLightingGainWattsPerArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "lighting-gain", kind: "change-lighting-gain-watts-per-area", record: "ChangedLightingGainWattsPerArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Lighting Gain Watts Per Area of lighting gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
