use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateSurface),Delete(crate::DeleteSurface)}
impl From<CreateSurface> for Operation{fn from(p:CreateSurface)->Self{Self::Create(p)}}impl From<crate::DeleteSurface> for Operation{fn from(p:crate::DeleteSurface)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateSurface as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteSurface as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateSurface as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteSurface as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_surface_creation_preserves_boundary_union_foreign_keys_rank_and_cancelled_sources(){laws::run::<CreateSurface,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.geometry.get().surfaces;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateSurface>(include_str!("🧫️fixtures/🔣️.json"));}
