//! 🟡️ Energy model mutation — `ChangeGlazingMaterialSolarTransmittance`: Sets normal-incidence solar transmittance on one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟡️ `change-glazing-material-solar-transmittance` payload. Sets normal-incidence solar transmittance on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-solar-transmittance")]
pub struct ChangeGlazingMaterialSolarTransmittance {
    pub id: crate::model::EntityId,
    pub new_solar_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_glazing_material_solar_transmittance(id: crate::model::EntityId, new_solar_transmittance: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGlazingMaterialSolarTransmittance(ChangeGlazingMaterialSolarTransmittance { id, new_solar_transmittance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGlazingMaterialSolarTransmittance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-solar-transmittance", record: "ChangedGlazingMaterialSolarTransmittance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Glazing Material Solar Transmittance of glazing material {}", self.id.0), &format!("Verglasungsmaterialsolartransmission von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
