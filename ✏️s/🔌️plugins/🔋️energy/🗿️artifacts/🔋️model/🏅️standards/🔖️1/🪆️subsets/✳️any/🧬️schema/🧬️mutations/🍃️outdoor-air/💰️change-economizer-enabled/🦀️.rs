//! 💰️ Energy model mutation — `ChangeOutdoorAirSystemEconomizerEnabled`: Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💰️ `change-outdoor-air-system-economizer-enabled` payload. Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-economizer-enabled")]
pub struct ChangeOutdoorAirSystemEconomizerEnabled {
    pub id: crate::model::EntityId,
    pub new_economizer_enabled: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_economizer_enabled<Op: From<ChangeOutdoorAirSystemEconomizerEnabled>>(id: crate::model::EntityId, new_economizer_enabled: bool) -> Op {
    ChangeOutdoorAirSystemEconomizerEnabled { id, new_economizer_enabled }.into()
}
//#endregion 🔖️Mutation
