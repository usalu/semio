use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateSetpointManager),Delete(crate::DeleteSetpointManager)}
impl From<CreateSetpointManager> for Operation{fn from(p:CreateSetpointManager)->Self{Self::Create(p)}}impl From<crate::DeleteSetpointManager> for Operation{fn from(p:crate::DeleteSetpointManager)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateSetpointManager as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteSetpointManager as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateSetpointManager as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteSetpointManager as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_setpoint_creation_preserves_typed_control_optional_schedule_exact_recreation_and_cancelled_sources(){laws::run::<CreateSetpointManager,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.controls.get().setpoint_managers;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateSetpointManager>(include_str!("🧫️fixtures/🔣️.json"));}
