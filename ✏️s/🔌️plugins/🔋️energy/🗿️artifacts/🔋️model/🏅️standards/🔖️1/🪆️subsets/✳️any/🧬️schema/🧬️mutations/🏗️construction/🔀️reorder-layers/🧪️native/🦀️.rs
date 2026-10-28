use super::*;
#[path="../../../🧳️native/🌱️construction/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
use crate::snapshot::original::EnergyModelSnapshot;
#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]enum Operation{Reorder(ReorderConstructionLayers)}
impl From<ReorderConstructionLayers> for Operation{fn from(p:ReorderConstructionLayers)->Self{Self::Reorder(p)}}
impl protocol::Mutation<EnergyModelSnapshot> for Operation{type Diff=crate::diff::original::EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[<ReorderConstructionLayers as protocol::MutationLeaf>::DESCRIPTOR];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{&<ReorderConstructionLayers as protocol::MutationLeaf>::DESCRIPTOR}}
fn ids(post:&EnergyModelSnapshot)->Vec<u32>{let rows=&post.model.materials.get().constructions;(0..rows.len()).map(|i|rows.get(i).unwrap().id.0).collect()}
#[test]fn original_layer_permutation_preserves_duplicate_multiplicities_exact_inverse_and_cancelled_sources(){laws::run::<ReorderConstructionLayers,Operation>(include_str!("🧫️fixtures/🔣️.json"),ids,|m|match m{Operation::Reorder(p)=>{assert_eq!(p.new_layer_material_ids.iter().map(|v|v.0).collect::<Vec<_>>(),[1,3,1]);p.id.0}});}
#[test]fn original_wide_layer_permutation_uses_paid_heap_sort_work_and_closes_every_partial_sort_source(){laws::run::<ReorderConstructionLayers,Operation>(include_str!("🧫️fixtures/🌊️wide.json"),ids,|m|match m{Operation::Reorder(p)=>{assert_eq!(p.new_layer_material_ids.iter().map(|v|v.0).collect::<Vec<_>>(),(0..75).flat_map(|_|1..17).collect::<Vec<_>>());p.id.0}});}
