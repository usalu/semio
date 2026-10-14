//! 🟤️ Energy model mutation — `ChangeGasMaterialGas`: Sets which fill gas occupies one glazing gap, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟤️ `change-gas-material-gas` payload. Sets which fill gas occupies one glazing gap, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-gas-material-gas")]
pub struct ChangeGasMaterialGas {
    pub id: crate::model::EntityId,
    pub new_gas: crate::model::GasKind,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_gas_material_gas<Op: From<ChangeGasMaterialGas>>(id: crate::model::EntityId, new_gas: crate::model::GasKind) -> Op {
    ChangeGasMaterialGas { id, new_gas }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeGasMaterialGas
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeGasMaterialGas>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gas-material", kind: "change-gas-material-gas", record: "ChangedGasMaterialGas" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Gas Material Gas of gas material {}", self.id.0), &format!("Gasart von Gasfüllung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
