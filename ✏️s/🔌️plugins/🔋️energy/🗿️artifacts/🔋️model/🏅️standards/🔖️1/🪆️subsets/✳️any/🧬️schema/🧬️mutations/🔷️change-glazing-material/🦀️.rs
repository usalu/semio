//! 🔷️ Energy model mutation — `ChangeGlazingMaterialThickness`: Sets pane thickness (m) on one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔷️ `change-glazing-material-thickness` payload. Sets pane thickness (m) on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-thickness")]
pub struct ChangeGlazingMaterialThickness {
    pub id: crate::model::EntityId,
    pub new_thickness_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_glazing_material_thickness(id: crate::model::EntityId, new_thickness_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGlazingMaterialThickness(ChangeGlazingMaterialThickness { id, new_thickness_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGlazingMaterialThickness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-thickness", record: "ChangedGlazingMaterialThickness" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Glazing Material Thickness of glazing material {}", self.id.0), &format!("Dicke von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
