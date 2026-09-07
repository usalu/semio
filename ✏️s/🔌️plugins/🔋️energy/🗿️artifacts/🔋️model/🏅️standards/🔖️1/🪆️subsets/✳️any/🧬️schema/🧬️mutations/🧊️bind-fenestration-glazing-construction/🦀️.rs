//! 🧊️ Energy model mutation — `BindFenestrationGlazingConstruction`: Points the fenestration's optional glazing slot at an existing layered construction, which then supersedes `uValueWM2k`/`shgc`/`vlt`. This is the schema seam the ticket's oracle comparison needed: with only the three scalars a semio→EnergyPlus translation can emit nothing richer than `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧊️ `bind-fenestration-glazing-construction` payload. Points the fenestration's optional glazing slot at an existing layered construction, which then supersedes `uValueWM2k`/`shgc`/`vlt`. This is the schema seam the ticket's oracle comparison needed: with only the three scalars a semio→EnergyPlus translation can emit nothing richer than `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "bind-fenestration-glazing-construction")]
pub struct BindFenestrationGlazingConstruction {
    pub id: crate::model::EntityId,
    pub construction_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn bind_fenestration_glazing_construction(id: crate::model::EntityId, construction_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::BindFenestrationGlazingConstruction(BindFenestrationGlazingConstruction { id, construction_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for BindFenestrationGlazingConstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "fenestration", kind: "bind-fenestration-glazing-construction", record: "BoundFenestrationGlazingConstruction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Bind fenestration {} to glazing construction {}", self.id.0, self.construction_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
