//! 🧪️ Every actual family payload preserves original projections and independently encoded Serde bytes.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneSource,RetainedCloneGrant}};
use semio_framework_trace::observe_heap_allocations_on_this_thread;
const CASES:[&str;36]=[
    include_str!("../../../../🧫️fixtures/🧬️mutations/⚓change-node-anchor/⚓️derives/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/✂️disconnect-handles/✂️severs/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/⚠️skips-locked-ghost/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/✏️edit-node-text/✏️recodes/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/➕add-node-handle/➕️adds/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/➖remove-node-handle/➖removes-middle-handle/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🆔change-manifest-id/📦️repoints/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️appends-region-2/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🌟change-node-root/🌳️promotes/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🌱create-node/🌱️appends/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🎨change-node-icon/🎨️swaps-a-capsule-icon/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🏗️change-node-kind/🏗️rekinds/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🏷️change-edge-kind/🏷️kinds/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/👀change-edge-visible/🙈️hides/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/👁️change-node-visible/🙈️hides-a-capsule/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/💔disconnect-kind-compatibility/💔️withdraws/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/📍move-node/📍️moves/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/📏scale-node/📏️doubles-node-a/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/📚replace-kind-catalogs/📇️installs/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔄️rotate-selection/⚠️skips-locked-ghost/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔌replace-node-handle/🔌️rekinds/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔏change-target-region-locked/🔏️locks/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔐change-edge-locked/🔒️locks/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🔒change-node-locked/🔒️locks/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🖇️change-edge-tips/🔀️swaps-edge-1-tips/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🖋️edit-target-region-label/🖋️renames/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🗑️delete-node/🗑️removes-middle-node/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🚀move-target-region/🚀️slides-region-1/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🤝connect-kind-compatibility/🤝️adds/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️circle/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🧮replace-edge-geometry/📍️repositions-edge-1/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🪢️connect-handles/⏸️keeps/🦠️mutation/🔣️.json"),
    include_str!("../../../../🧫️fixtures/🧬️mutations/🪦delete-target-region/🪦️removes-middle-region/🦠️mutation/🔣️.json"),
];
fn grant(demand:RetirementDemand)->RetainedCloneGrant{
 assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
 RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
#[test]
fn puzzle_canonical_native_all_mutation_fields_preserve_serde_originals_and_granted_heap(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut covered=std::collections::BTreeSet::new();
 for (ordinal,input) in CASES.iter().enumerate(){
  let raw:serde_json::Value=serde_json::from_str(input).unwrap();assert_eq!(raw["mutation"],fixture["cases"][ordinal]["tag"]);assert!(covered.insert(raw["mutation"].as_str().unwrap().to_owned()));
  for pause in [Some(0),Some(1),Some(3),Some(17),Some(129),None]{
   let mutation:Puzzle2dMutation=serde_json::from_str(input).unwrap();let oracle=serde_json::to_vec(&mutation).unwrap();
   let birth=RetainedCloneSource::<Puzzle2dMutation>::owned_constructor_demand::<()>();let full=grant(birth);
   let((mut source,receipt),heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(mutation,(),full).unwrap_or_else(|(error,_,_)|panic!("original Puzzle source: {error}")));assert!(receipt.fits(full));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));
   let(_,heap)=observe_heap_allocations_on_this_thread(||{
    let owner=source.borrow();let owner=owner.get();
    match owner{
     Puzzle2dMutation::MoveNode(payload)=>assert_eq!(owner.canonical_tree_child(1).unwrap()as*const dyn ArtifactCanonicalJsonTree as*const(),&payload.id as*const _ as*const()),
     Puzzle2dMutation::CreateNode(payload)=>assert_eq!(owner.canonical_tree_child(1).unwrap()as*const dyn ArtifactCanonicalJsonTree as*const(),&payload.node as*const _ as*const()),
     _=>{}
    }
   });assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let projection=source.project_owned(0,|owner|owner as&dyn ArtifactCanonicalJsonTree);let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();let(mut cursor,receipt)=ArtifactCanonicalJsonTreeCursor::admit(projection,grant(demand)).unwrap_or_else(|(error,_)|panic!("original Puzzle traversal: {error}"));assert_eq!(receipt.copied_bytes,demand.copy_bytes);
   let mut output=Vec::new();let mut turns=0;
   while !cursor.terminal_is_empty(){
    assert!(turns<100000,"Puzzle payload ordinal {ordinal} traversal did not settle");if pause==Some(turns){cursor.begin_close();}
    let(demand,heap)=observe_heap_allocations_on_this_thread(||cursor.next_demand().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let full=grant(demand);
    let(zero,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut[0],RetainedCloneGrant::default()).unwrap());assert_eq!(zero.ownership.progress(),Default::default());assert_eq!(zero.written_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let mut byte=[0xa5];let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut byte,full).unwrap());assert!(step.ownership.progress().fits(full));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.ownership.progress().retained_capacity_bytes,step.ownership.progress().released_bytes));assert!(step.written_bytes<=1);output.extend_from_slice(&byte[..step.written_bytes]);turns+=1;
   }
   if pause.is_none(){assert_eq!(output,oracle);}else{assert_eq!(output,oracle[..output.len()]);}
   let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   for turn in 0..100000{
    if source.terminal_is_empty(){break;}
    let copy=source.next_close_copy_byte_demand().unwrap();let full=grant(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
    let(step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(full).unwrap());assert!(step.progress().fits(full));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||source.terminal_is_empty());
   }
   assert!(source.terminal_is_empty());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] actual Puzzle canonical payload ordinal={ordinal} pause={pause:?} turns={turns}, independent Serde exact prefix, original ID/node projections, native paged frontier,4096 and terminalDrop0");
  }
 }
 assert_eq!(covered.len(),36);
}
