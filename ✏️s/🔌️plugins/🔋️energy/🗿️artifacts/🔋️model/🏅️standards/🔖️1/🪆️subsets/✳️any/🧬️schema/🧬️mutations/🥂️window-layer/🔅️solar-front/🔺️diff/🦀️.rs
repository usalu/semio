//! 🔺️ Only the authored optical field reaches the existing atomic central applier.
use crate::{diff::{ReferenceEnergyModelDiff as EnergyModelDiff,GlazingMaterialPatch,ModelPatch},EnergyModelSnapshot};
use crate::diff_patch::Rows;
/// 🧭️ Validates the source row and optical domain before producing a sparse field patch.
pub fn diff(payload:&super::ChangeGlazingMaterialSolarReflectanceFront,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{
 let Some(original)=base.model.glazing_materials.iter().find(|row|row.id==payload.id)else{return protocol::MutationOutcome::error("mutation.target-missing",format!("Glazing material {} does not exist.",payload.id.0),[payload.id.0.to_string()]);};
 if !payload.new_solar_reflectance_front.is_finite()||!(0.0..=1.0).contains(&payload.new_solar_reflectance_front){return protocol::MutationOutcome::fatal("mutation.invariant","An optical fraction must be finite and lie in 0..=1.",[payload.id.0.to_string()]);}
 if original.solar_reflectance_front==payload.new_solar_reflectance_front{return protocol::MutationOutcome::empty().warning("mutation.no-op","The optical field already has this value.");}
 protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch{glazing_materials:Rows::modifying(GlazingMaterialPatch{solar_reflectance_front:Some(payload.new_solar_reflectance_front),..GlazingMaterialPatch::of(payload.id)}),..Default::default()}))
}
