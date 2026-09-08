//! 🌈️ Energy model mutation — `ChangeFenestrationVlt`: Sets the visible light transmittance — the daylight fraction the illuminance calculation reads, independent of the solar gain the SHGC governs.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌈️ `change-fenestration-vlt` payload. Sets the visible light transmittance — the daylight fraction the illuminance calculation reads, independent of the solar gain the SHGC governs.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-vlt")]
pub struct ChangeFenestrationVlt {
    pub id: crate::model::EntityId,
    pub new_vlt: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_vlt(id: crate::model::EntityId, new_vlt: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationVlt(ChangeFenestrationVlt { id, new_vlt })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationVlt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-vlt", record: "ChangedFenestrationVlt" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} visible transmittance to {}", self.id.0, self.new_vlt)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
