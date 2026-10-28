use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateFenestration),Delete(crate::DeleteFenestration)}
impl From<CreateFenestration> for Operation{fn from(p:CreateFenestration)->Self{Self::Create(p)}}impl From<crate::DeleteFenestration> for Operation{fn from(p:crate::DeleteFenestration)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateFenestration as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteFenestration as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateFenestration as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteFenestration as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_fenestration_creation_preserves_empty_geometry_source_optional_glazing_and_cancelled_owners(){laws::run::<CreateFenestration,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.geometry.get().fenestrations;(0..rows.len()).map(|i|{let row=rows.get(i).unwrap();assert_eq!(row.vertices_m.get().len(),if row.id.0==3{0}else{3});row.id.0}).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}
