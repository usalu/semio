//! 🧲️ Energy model mutation — `ChangeFenestrationSurface`: Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧲️ `change-fenestration-surface` payload. Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-surface")]
pub struct ChangeFenestrationSurface {
    pub id: crate::model::EntityId,
    pub new_surface_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_surface(id: crate::model::EntityId, new_surface_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationSurface(ChangeFenestrationSurface { id, new_surface_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-surface", record: "ChangedFenestrationSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move fenestration {} to surface {}", self.id.0, self.new_surface_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
