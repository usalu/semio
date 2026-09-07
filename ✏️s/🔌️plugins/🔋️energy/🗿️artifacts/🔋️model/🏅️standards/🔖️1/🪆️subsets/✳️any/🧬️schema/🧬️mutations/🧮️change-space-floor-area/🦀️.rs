//! 🧮️ Energy model mutation — `ChangeSpaceFloorArea`: Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧮️ `change-space-floor-area` payload. Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-space-floor-area")]
pub struct ChangeSpaceFloorArea {
    pub id: crate::model::EntityId,
    pub new_floor_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_space_floor_area(id: crate::model::EntityId, new_floor_area_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSpaceFloorArea(ChangeSpaceFloorArea { id, new_floor_area_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSpaceFloorArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "space", kind: "change-space-floor-area", record: "ChangedSpaceFloorArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change space {} floor area to {} m²", self.id.0, self.new_floor_area_m2)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
