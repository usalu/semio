//! 🏳️ Energy model mutation — `RenameSurface`: Sets one surface's identity field; a name another surface already holds is refused because every surface-level report keys on it.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏳️ `rename-surface` payload. Sets one surface's identity field; a name another surface already holds is refused because every surface-level report keys on it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-surface")]
pub struct RenameSurface {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_surface(id: crate::model::EntityId, new_name: String) -> EnergyModelMutation {
    EnergyModelMutation::RenameSurface(RenameSurface { id, new_name })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RenameSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "surface", kind: "rename-surface", record: "RenamedSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename surface {} to \"{}\"", self.id.0, self.new_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
