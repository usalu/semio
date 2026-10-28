use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateZone),Delete(crate::DeleteZone)}
impl From<CreateZone> for Operation{fn from(p:CreateZone)->Self{Self::Create(p)}}impl From<crate::DeleteZone> for Operation{fn from(p:crate::DeleteZone)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateZone as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteZone as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateZone as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteZone as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn actual_create_zone_leaf_preserves_semantic_validation_rank_inverse_and_original_cancellation(){laws::run::<CreateZone,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.geometry.get().zones;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("create must retain its actual delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateZone>(include_str!("🧫️fixtures/🔣️.json"));}
