//! 🎩️ Energy model mutation — `ChangeFenestrationOverhangOffset`: Sets how far above the window head the horizontal projection sits, in metres.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎩️ `change-fenestration-overhang-offset` payload. Sets how far above the window head the horizontal projection sits, in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-overhang-offset")]
pub struct ChangeFenestrationOverhangOffset {
    pub id: crate::model::EntityId,
    pub new_overhang_offset_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_overhang_offset(id: crate::model::EntityId, new_overhang_offset_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationOverhangOffset(ChangeFenestrationOverhangOffset { id, new_overhang_offset_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationOverhangOffset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-overhang-offset", record: "ChangedFenestrationOverhangOffset" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} overhang offset to {} m", self.id.0, self.new_overhang_offset_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
