//! 🟠️ Energy model mutation — `ChangeGlazingMaterialConductivity`: Sets pane conductivity (W/m·K) on one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟠️ `change-glazing-material-conductivity` payload. Sets pane conductivity (W/m·K) on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-conductivity")]
pub struct ChangeGlazingMaterialConductivity {
    pub id: crate::model::EntityId,
    pub new_conductivity_w_m_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_glazing_material_conductivity(id: crate::model::EntityId, new_conductivity_w_m_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGlazingMaterialConductivity(ChangeGlazingMaterialConductivity { id, new_conductivity_w_m_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGlazingMaterialConductivity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-conductivity", record: "ChangedGlazingMaterialConductivity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Glazing Material Conductivity of glazing material {}", self.id.0), &format!("Verglasungsmaterialleitfähigkeit von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
