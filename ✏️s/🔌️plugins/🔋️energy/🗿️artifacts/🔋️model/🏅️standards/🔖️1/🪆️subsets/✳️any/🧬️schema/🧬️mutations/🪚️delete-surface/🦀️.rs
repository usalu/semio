//! 🪚️ Energy model mutation — `DeleteSurface`: Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪚️ `delete-surface` payload. Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-surface")]
pub struct DeleteSurface {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_surface(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteSurface(DeleteSurface { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "surface", kind: "delete-surface", record: "DeletedSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete surface {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
