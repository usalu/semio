//! 🩻️ Energy model mutation — `ChangeGlazingMaterialInfraredEmissivity`: Sets both long-wave emissivities of one glazing pane in one step — front and back are one optical facet of the same sheet, and a coating that changes one almost always changes the other.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🩻️ `change-glazing-material-infrared-emissivity` payload. Sets both long-wave emissivities of one glazing pane in one step — front and back are one optical facet of the same sheet, and a coating that changes one almost always changes the other.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-infrared-emissivity")]
pub struct ChangeGlazingMaterialInfraredEmissivity {
    pub id: crate::model::EntityId,
    pub new_infrared_emissivity_front: f64,
    pub new_infrared_emissivity_back: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_glazing_material_infrared_emissivity(id: crate::model::EntityId, new_infrared_emissivity_front: f64, new_infrared_emissivity_back: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGlazingMaterialInfraredEmissivity(ChangeGlazingMaterialInfraredEmissivity { id, new_infrared_emissivity_front, new_infrared_emissivity_back })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGlazingMaterialInfraredEmissivity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-infrared-emissivity", record: "ChangedGlazingMaterialInfraredEmissivity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Glazing Material Infrared Emissivity of glazing material {}", self.id.0), &format!("Verglasungsmaterialinfrarotemissionsgrad von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
