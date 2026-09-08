//! 🐋️ Energy model mutation — `ChangeFenestrationFinOffset`: Sets how far beside the window jambs the two vertical projections stand, in metres.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐋️ `change-fenestration-fin-offset` payload. Sets how far beside the window jambs the two vertical projections stand, in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-fin-offset")]
pub struct ChangeFenestrationFinOffset {
    pub id: crate::model::EntityId,
    pub new_fin_offset_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_fin_offset(id: crate::model::EntityId, new_fin_offset_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationFinOffset(ChangeFenestrationFinOffset { id, new_fin_offset_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationFinOffset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-fin-offset", record: "ChangedFenestrationFinOffset" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} fin offset to {} m", self.id.0, self.new_fin_offset_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
