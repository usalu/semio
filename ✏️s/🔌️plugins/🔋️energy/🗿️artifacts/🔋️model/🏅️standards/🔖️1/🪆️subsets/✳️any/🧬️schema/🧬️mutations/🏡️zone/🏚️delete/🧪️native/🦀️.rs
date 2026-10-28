use super::*;
use crate::CreateZone;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateZone),Delete(crate::DeleteZone)}
impl From<CreateZone> for Operation{fn from(p:CreateZone)->Self{Self::Create(p)}}impl From<crate::DeleteZone> for Operation{fn from(p:crate::DeleteZone)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateZone as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteZone as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateZone as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteZone as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_zone_deletion_checks_all_seventeen_native_reference_sources_before_removal(){for fixture in[include_str!("🧫️fixtures/🔣️.json"),include_str!("🧫️fixtures/🔗️referenced.json")]{laws::run::<DeleteZone,Operation>(fixture,|post|{let rows=&post.model.geometry.get().zones;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Create(row)=>{assert_eq!(row.index,Some(if row.id.0==1{0}else{1}));assert!(row.volume_m3>0.);row.id.0},_=>panic!("deletion must preserve its actual typed create inverse")});}}
