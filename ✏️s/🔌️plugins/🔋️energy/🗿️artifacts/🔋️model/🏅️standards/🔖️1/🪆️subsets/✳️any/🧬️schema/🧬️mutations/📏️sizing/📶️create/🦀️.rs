//! 📶️ Energy model mutation — `CreateSizingObject`: Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📶️ `create-sizing-object` payload. Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-sizing-object")]
pub struct CreateSizingObject {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub sizing_type: crate::model::SizingType,
    pub design_day_type: crate::model::DesignDayType,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_sizing_object<Op: From<CreateSizingObject>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, sizing_type: crate::model::SizingType, design_day_type: crate::model::DesignDayType, index: Option<u32>) -> Op {
    CreateSizingObject { id, zone_id, sizing_type, design_day_type, index }.into()
}
//#endregion 🔖️Mutation
