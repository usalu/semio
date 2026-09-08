//! 🫗️ Energy model mutation — `ClearFenestrationGlazingConstruction`: Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫗️ `clear-fenestration-glazing-construction` payload. Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "clear-fenestration-glazing-construction")]
pub struct ClearFenestrationGlazingConstruction {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn clear_fenestration_glazing_construction(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ClearFenestrationGlazingConstruction(ClearFenestrationGlazingConstruction { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ClearFenestrationGlazingConstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "clear", entity: "fenestration", kind: "clear-fenestration-glazing-construction", record: "ClearedFenestrationGlazingConstruction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Clear fenestration {} glazing construction", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
