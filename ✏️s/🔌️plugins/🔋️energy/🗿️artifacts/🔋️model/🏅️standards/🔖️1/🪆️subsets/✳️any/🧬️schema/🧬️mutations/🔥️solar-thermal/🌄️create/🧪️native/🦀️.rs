use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateSolarThermalSystem),Delete(crate::DeleteSolarThermalSystem)}
impl From<CreateSolarThermalSystem> for Operation{fn from(p:CreateSolarThermalSystem)->Self{Self::Create(p)}}impl From<crate::DeleteSolarThermalSystem> for Operation{fn from(p:crate::DeleteSolarThermalSystem)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateSolarThermalSystem as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteSolarThermalSystem as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateSolarThermalSystem as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteSolarThermalSystem as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_creation_preserves_field_validation_rank_inverse_and_every_cancelled_prefix(){laws::run::<CreateSolarThermalSystem,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.generation.get().solar_thermal_systems;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateSolarThermalSystem>(include_str!("🧫️fixtures/🔣️.json"));}
