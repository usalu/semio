use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateIdealLoadsSystem),Delete(crate::DeleteIdealLoadsSystem)}
impl From<CreateIdealLoadsSystem> for Operation{fn from(p:CreateIdealLoadsSystem)->Self{Self::Create(p)}}impl From<crate::DeleteIdealLoadsSystem> for Operation{fn from(p:crate::DeleteIdealLoadsSystem)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateIdealLoadsSystem as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteIdealLoadsSystem as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateIdealLoadsSystem as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteIdealLoadsSystem as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_ideal_loads_creation_preserves_zone_optional_capacity_presence_typed_inverse_and_cancelled_sources(){laws::run::<CreateIdealLoadsSystem,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.controls.get().ideal_loads;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateIdealLoadsSystem>(include_str!("🧫️fixtures/🔣️.json"));}
