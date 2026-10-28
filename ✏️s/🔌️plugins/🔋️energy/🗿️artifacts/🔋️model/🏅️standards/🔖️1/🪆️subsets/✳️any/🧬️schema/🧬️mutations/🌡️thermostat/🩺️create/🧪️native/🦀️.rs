use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateThermostat),Delete(crate::DeleteThermostat)}
impl From<CreateThermostat> for Operation{fn from(p:CreateThermostat)->Self{Self::Create(p)}}impl From<crate::DeleteThermostat> for Operation{fn from(p:crate::DeleteThermostat)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateThermostat as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteThermostat as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateThermostat as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteThermostat as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_creation_preserves_append_rank_foreign_first_refusal_and_cancelled_lookups(){laws::run::<CreateThermostat,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.controls.get().thermostats;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateThermostat>(include_str!("🧫️fixtures/🔣️.json"));}
