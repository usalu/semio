//! 🟣️ Energy model mutation — `ChangeGasMaterialThickness`: Sets gap width (m) on one gas material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟣️ `change-gas-material-thickness` payload. Sets gap width (m) on one gas material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-gas-material-thickness")]
pub struct ChangeGasMaterialThickness {
    pub id: crate::model::EntityId,
    pub new_thickness_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_gas_material_thickness(id: crate::model::EntityId, new_thickness_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGasMaterialThickness(ChangeGasMaterialThickness { id, new_thickness_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGasMaterialThickness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gas-material", kind: "change-gas-material-thickness", record: "ChangedGasMaterialThickness" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Gas Material Thickness of gas material {}", self.id.0), &format!("Dicke von Gasfüllung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
