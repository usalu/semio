//! 🧰️ Energy model mutation — `ChangeSurfaceConstruction`: Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧰️ `change-surface-construction` payload. Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-construction")]
pub struct ChangeSurfaceConstruction {
    pub id: crate::model::EntityId,
    pub new_construction_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_construction(id: crate::model::EntityId, new_construction_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceConstruction(ChangeSurfaceConstruction { id, new_construction_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceConstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-construction", record: "ChangedSurfaceConstruction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} construction to {}", self.id.0, self.new_construction_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
