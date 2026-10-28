use super::*;
use crate::CreateSetpointManager;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateSetpointManager),Delete(crate::DeleteSetpointManager)}
impl From<CreateSetpointManager> for Operation{fn from(p:CreateSetpointManager)->Self{Self::Create(p)}}impl From<crate::DeleteSetpointManager> for Operation{fn from(p:crate::DeleteSetpointManager)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateSetpointManager as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteSetpointManager as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateSetpointManager as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteSetpointManager as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_deletion_preserves_actual_row_rank_inverse_and_every_cancelled_prefix(){laws::run::<DeleteSetpointManager,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.controls.get().setpoint_managers;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Create(row)=>{let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let actual=serde_json::Value::from(semio_framework_value::ToValue::to_value(row));assert_eq!(actual,law["inverses"][row.id.0.to_string()]);row.id.0},_=>panic!("deletion must preserve its actual typed create inverse")});}
