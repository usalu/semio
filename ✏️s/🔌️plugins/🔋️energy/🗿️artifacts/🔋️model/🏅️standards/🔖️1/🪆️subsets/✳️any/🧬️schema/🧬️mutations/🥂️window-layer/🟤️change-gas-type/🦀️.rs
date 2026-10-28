//! 🟤️ Energy model mutation — `ChangeGasMaterialGas`: Sets which fill gas occupies one glazing gap, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟤️ `change-gas-material-gas` payload. Sets which fill gas occupies one glazing gap, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
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
//#endregion 🔖️Mutation
