//! 🟤️ Energy model mutation — `ChangeGasMaterialGas`: Sets which fill gas occupies one glazing gap, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟤️ `change-gas-material-gas` payload. Sets which fill gas occupies one glazing gap, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-gas-material-gas")]
pub struct ChangeGasMaterialGas {
    pub id: crate::model::EntityId,
    pub new_gas: crate::model::GasKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_gas_material_gas(id: crate::model::EntityId, new_gas: crate::model::GasKind) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGasMaterialGas(ChangeGasMaterialGas { id, new_gas })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGasMaterialGas {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gas-material", kind: "change-gas-material-gas", record: "ChangedGasMaterialGas" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change Gas Material Gas of gas material {}", self.id.0), &format!("Gasart von Gasfüllung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
