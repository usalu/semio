use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateConstantSchedule),Delete(crate::DeleteConstantSchedule)}
impl From<CreateConstantSchedule> for Operation{fn from(p:CreateConstantSchedule)->Self{Self::Create(p)}}impl From<crate::DeleteConstantSchedule> for Operation{fn from(p:crate::DeleteConstantSchedule)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateConstantSchedule as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteConstantSchedule as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateConstantSchedule as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteConstantSchedule as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_schedule_creation_checks_all_five_identity_collections_before_rank_and_preserves_nested_output(){laws::run::<CreateConstantSchedule,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.outputs.get().schedules.get().constants;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateConstantSchedule>(include_str!("🧫️fixtures/🔣️.json"));}
