//! 🧩️ Energy model mutation — `ChangeSurfaceClass`: Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧩️ `change-surface-class` payload. Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-class")]
pub struct ChangeSurfaceClass {
    pub id: crate::model::EntityId,
    pub new_class: crate::model::SurfaceClass,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_class(id: crate::model::EntityId, new_class: crate::model::SurfaceClass) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceClass(ChangeSurfaceClass { id, new_class })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-class", record: "ChangedSurfaceClass" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} class to {:?}", self.id.0, self.new_class)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
