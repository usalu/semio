use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateWeeklySchedule),Delete(crate::DeleteWeeklySchedule)}
impl From<CreateWeeklySchedule> for Operation{fn from(p:CreateWeeklySchedule)->Self{Self::Create(p)}}impl From<crate::DeleteWeeklySchedule> for Operation{fn from(p:crate::DeleteWeeklySchedule)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateWeeklySchedule as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteWeeklySchedule as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateWeeklySchedule as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteWeeklySchedule as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_weekly_creation_preserves_seven_fixed_native_days_original_foreign_keys_and_cancelled_sources(){laws::run::<CreateWeeklySchedule,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.outputs.get().schedules.get().weekly;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateWeeklySchedule>(include_str!("🧫️fixtures/🔣️.json"));}
