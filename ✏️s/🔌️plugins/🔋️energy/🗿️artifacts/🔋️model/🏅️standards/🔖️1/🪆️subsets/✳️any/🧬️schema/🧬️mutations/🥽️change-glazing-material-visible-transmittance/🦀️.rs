//! 🥽️ Energy model mutation — `ChangeGlazingMaterialVisibleTransmittance`: Sets normal-incidence visible transmittance on one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥽️ `change-glazing-material-visible-transmittance` payload. Sets normal-incidence visible transmittance on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-visible-transmittance")]
pub struct ChangeGlazingMaterialVisibleTransmittance {
    pub id: crate::model::EntityId,
    pub new_visible_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_glazing_material_visible_transmittance(id: crate::model::EntityId, new_visible_transmittance: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGlazingMaterialVisibleTransmittance(ChangeGlazingMaterialVisibleTransmittance { id, new_visible_transmittance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGlazingMaterialVisibleTransmittance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-visible-transmittance", record: "ChangedGlazingMaterialVisibleTransmittance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Glazing Material Visible Transmittance of glazing material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
