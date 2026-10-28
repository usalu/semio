use super::*;
use crate::CreateConstruction;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateConstruction),Delete(crate::DeleteConstruction)}
impl From<CreateConstruction> for Operation{fn from(p:CreateConstruction)->Self{Self::Create(p)}}impl From<crate::DeleteConstruction> for Operation{fn from(p:crate::DeleteConstruction)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateConstruction as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteConstruction as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateConstruction as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteConstruction as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_combined_deletion_rejects_actual_glazing_references_and_preserves_every_cancelled_child(){for fixture in[include_str!("🧫️fixtures/🔣️.json"),include_str!("🧫️fixtures/🔗️referenced.json")]{laws::run::<DeleteConstruction,Operation>(fixture,|post|{let rows=&post.model.materials.get().constructions;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Create(row)=>{assert_eq!(row.index,if row.id.0==1{0}else{1});assert_eq!(row.name,"Original layers");assert!(row.layer_material_ids.is_empty());row.id.0},_=>panic!("deletion must preserve its actual typed create inverse")});}}
