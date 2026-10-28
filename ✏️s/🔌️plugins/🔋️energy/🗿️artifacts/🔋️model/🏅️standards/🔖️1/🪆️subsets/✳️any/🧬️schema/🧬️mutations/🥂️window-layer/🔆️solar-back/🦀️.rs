//! 🔆️ A semantic intent changes only the named glazing optical field.
use semio_framework_value_derive::{FromValue as FromValueDerive,ToValue as ToValueDerive};
/// 🪟️ Solar back reflectance is addressed by the original glazing row identity.
#[derive(Clone,Debug,PartialEq,ToValueDerive,FromValueDerive,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf, semio_framework_value::NativeReceive,semio_s_artifact_energy_model_native_derive::EnergyNativeMutation,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all="camelCase")]
#[dsl(keyword="change-glazing-material-solar-reflectance-back")]
pub struct ChangeGlazingMaterialSolarReflectanceBack{pub id:crate::model::EntityId,pub new_solar_reflectance_back:f64}
/// 🎯️ Constructs the explicit optical field intent.
pub fn change_glazing_material_solar_reflectance_back<Op: From<ChangeGlazingMaterialSolarReflectanceBack>>(id:crate::model::EntityId,new_solar_reflectance_back:f64)->Op{ChangeGlazingMaterialSolarReflectanceBack{id,new_solar_reflectance_back}.into()}
