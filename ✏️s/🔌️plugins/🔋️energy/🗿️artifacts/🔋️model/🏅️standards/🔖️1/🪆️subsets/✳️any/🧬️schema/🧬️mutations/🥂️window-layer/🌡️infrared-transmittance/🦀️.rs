//! 🌡️ A semantic intent changes only the named glazing optical field.
use semio_framework_value_derive::{FromValue as FromValueDerive,ToValue as ToValueDerive};
/// 🪟️ Infrared transmittance is addressed by the original glazing row identity.
#[derive(Clone,Debug,PartialEq,ToValueDerive,FromValueDerive,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf, semio_framework_value::NativeReceive,semio_s_artifact_energy_model_native_derive::EnergyNativeMutation,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all="camelCase")]
#[dsl(keyword="change-glazing-material-infrared-transmittance")]
pub struct ChangeGlazingMaterialInfraredTransmittance{pub id:crate::model::EntityId,pub new_infrared_transmittance:f64}
/// 🎯️ Constructs the explicit optical field intent.
pub fn change_glazing_material_infrared_transmittance<Op: From<ChangeGlazingMaterialInfraredTransmittance>>(id:crate::model::EntityId,new_infrared_transmittance:f64)->Op{ChangeGlazingMaterialInfraredTransmittance{id,new_infrared_transmittance}.into()}
