use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreatePeopleGain),Delete(crate::DeletePeopleGain)}
impl From<CreatePeopleGain> for Operation{fn from(p:CreatePeopleGain)->Self{Self::Create(p)}}impl From<crate::DeletePeopleGain> for Operation{fn from(p:crate::DeletePeopleGain)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreatePeopleGain as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeletePeopleGain as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreatePeopleGain as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeletePeopleGain as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_foreign_creation_preserves_each_actual_failed_key_rank_inverse_and_cancelled_lookup(){laws::run::<CreatePeopleGain,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.loads.get().people;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreatePeopleGain>(include_str!("🧫️fixtures/🔣️.json"));}
