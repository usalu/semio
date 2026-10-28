use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateDailySchedule),Delete(crate::DeleteDailySchedule)}
impl From<CreateDailySchedule> for Operation{fn from(p:CreateDailySchedule)->Self{Self::Create(p)}}impl From<crate::DeleteDailySchedule> for Operation{fn from(p:crate::DeleteDailySchedule)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateDailySchedule as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteDailySchedule as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateDailySchedule as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteDailySchedule as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_daily_creation_preserves_twenty_four_values_paired_limits_original_inverse_and_cancelled_sources(){laws::run::<CreateDailySchedule,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.outputs.get().schedules.get().daily;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateDailySchedule>(include_str!("🧫️fixtures/🔣️.json"));}
