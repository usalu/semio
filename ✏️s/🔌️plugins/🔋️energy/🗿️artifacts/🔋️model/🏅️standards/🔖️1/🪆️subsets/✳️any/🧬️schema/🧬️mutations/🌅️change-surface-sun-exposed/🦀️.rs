//! 🌅️ Energy model mutation — `ChangeSurfaceSunExposed`: Sets whether the outside face receives beam and diffuse solar at all.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌅️ `change-surface-sun-exposed` payload. Sets whether the outside face receives beam and diffuse solar at all.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-sun-exposed")]
pub struct ChangeSurfaceSunExposed {
    pub id: crate::model::EntityId,
    pub new_sun_exposed: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_sun_exposed(id: crate::model::EntityId, new_sun_exposed: bool) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceSunExposed(ChangeSurfaceSunExposed { id, new_sun_exposed })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceSunExposed {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-sun-exposed", record: "ChangedSurfaceSunExposed" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} sun exposure to {}", self.id.0, self.new_sun_exposed)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
