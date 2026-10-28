use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Create(CreateShadingSurface),Delete(crate::DeleteShadingSurface)}
impl From<CreateShadingSurface> for Operation{fn from(p:CreateShadingSurface)->Self{Self::Create(p)}}impl From<crate::DeleteShadingSurface> for Operation{fn from(p:crate::DeleteShadingSurface)->Self{Self::Delete(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<CreateShadingSurface as protocol::MutationLeaf>::DESCRIPTOR,<crate::DeleteShadingSurface as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match self{Self::Create(_)=>&<CreateShadingSurface as protocol::MutationLeaf>::DESCRIPTOR,Self::Delete(_)=>&<crate::DeleteShadingSurface as protocol::MutationLeaf>::DESCRIPTOR}}}
#[test]
fn original_geometry_creation_preserves_optional_schedule_row_target_wide_fragments_and_cancelled_sources(){laws::run::<CreateShadingSurface,Operation>(include_str!("🧫️fixtures/🔣️.json"),|post|{let rows=&post.model.geometry.get().shading_surfaces;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()},|operation|match operation{Operation::Delete(row)=>row.id.0,_=>panic!("creation must retain its original delete inverse")});}

#[test]fn actual_original_row_recreation_retains_typed_fields_rank_and_inverse_custody(){laws::recreate::<CreateShadingSurface>(include_str!("🧫️fixtures/🔣️.json"));}
