//! 🟢️ Energy model mutation — `RenameGlazingMaterial`: Renames one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟢️ `rename-glazing-material` payload. Renames one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-glazing-material")]
pub struct RenameGlazingMaterial {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_glazing_material(id: crate::model::EntityId, new_name: String) -> EnergyModelMutation {
    EnergyModelMutation::RenameGlazingMaterial(RenameGlazingMaterial { id, new_name })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RenameGlazingMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "glazing-material", kind: "rename-glazing-material", record: "RenamedGlazingMaterial" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename Glazing Material of glazing material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
