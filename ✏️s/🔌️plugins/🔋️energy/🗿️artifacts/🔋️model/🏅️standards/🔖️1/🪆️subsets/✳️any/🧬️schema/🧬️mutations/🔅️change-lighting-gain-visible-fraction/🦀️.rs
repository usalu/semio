//! 🔅️ Energy model mutation — `ChangeLightingGainVisibleFraction`: Sets visible fraction on one lighting gain, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔅️ `change-lighting-gain-visible-fraction` payload. Sets visible fraction on one lighting gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-visible-fraction")]
pub struct ChangeLightingGainVisibleFraction {
    pub id: crate::model::EntityId,
    pub new_visible_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_lighting_gain_visible_fraction(id: crate::model::EntityId, new_visible_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeLightingGainVisibleFraction(ChangeLightingGainVisibleFraction { id, new_visible_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeLightingGainVisibleFraction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "lighting-gain", kind: "change-lighting-gain-visible-fraction", record: "ChangedLightingGainVisibleFraction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Lighting Gain Visible Fraction of lighting gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
