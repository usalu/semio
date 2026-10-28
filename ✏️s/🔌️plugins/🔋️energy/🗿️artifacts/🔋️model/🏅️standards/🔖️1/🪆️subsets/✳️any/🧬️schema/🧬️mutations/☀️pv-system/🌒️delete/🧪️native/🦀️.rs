use super::*;
use crate::CreatePvSystem;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreatePvSystem),Delete(crate::DeletePvSystem)}
impl From<CreatePvSystem> for Operation{fn from(p:CreatePvSystem)->Self{Self::Create(p)}}impl From<crate::DeletePvSystem> for Operation{fn from(p:crate::DeletePvSystem)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreatePvSystem as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeletePvSystem as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreatePvSystem as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeletePvSystem as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_referenced_deletion_reads_wide_keys_before_removal_and_preserves_all_cancelled_sources(){for fixture in[include_str!("🧫️fixtures/🔣️.json"),include_str!("🧫️fixtures/🔗️referenced.json")]{laws::run::<DeletePvSystem,Operation>(fixture,|post|{let rows=&post.model.generation.get().pv_systems;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Create(row)=>{assert_eq!(row.index,if row.id.0==1{0}else{1});assert_eq!(row.dc_capacity_w,1000.);row.id.0},_=>panic!("deletion must preserve its actual typed create inverse")});}}
