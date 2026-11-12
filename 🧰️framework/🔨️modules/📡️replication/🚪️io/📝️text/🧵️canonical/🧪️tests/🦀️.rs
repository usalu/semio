//! 🧪️ Actual persistent protocol field projections and granted closure match independent JSON bytes.
use super::*;
use semio_framework_pack_json::ArtifactCanonicalJsonTreeCursor;
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant}};
use crate::retained_sources::{stage,drained,effected,projected};
use semio_framework_trace::observe_heap_allocations_on_this_thread;

fn grant(demand:RetirementDemand)->RetainedCloneGrant {
 assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
 RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
#[test]
fn persistent_protocol_canonical_fields_preserve_original_projection_pointers_and_granted_heap() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let expected=row["expected"].as_str().unwrap().as_bytes();
  let mut pauses:Vec<Option<usize>>=fixture["pauses"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize)).collect();pauses.push(None);
  for pause in pauses {
   let mut edit:Edit<bool>=semio_framework_pack_json::from_json_str(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   edit.id.reserve(8192);if let Some(meta)=edit.mutation_meta.first_mut(){meta.label.as_mut().unwrap().reserve(8192);}
   let id_pointer=edit.id.as_ptr();let forwards_pointer=edit.forwards.as_ptr();
    let mut source=stage(edit);



   let original=source.borrow();assert_eq!(original.get().id.as_ptr(),id_pointer);assert_eq!(original.get().forwards.as_ptr(),forwards_pointer);
   let(fields,heap)=observe_heap_allocations_on_this_thread(||{
    let owner=original.get();let id=owner.canonical_tree_child(0).unwrap();
    assert_eq!(id as*const dyn ArtifactCanonicalJsonTree as*const (),&owner.id as*const String as*const ());
    let forwards_ordinal=if owner.actor.is_some(){2}else{1};let forwards=owner.canonical_tree_child(forwards_ordinal).unwrap();
    assert_eq!(forwards as*const dyn ArtifactCanonicalJsonTree as*const (),&owner.forwards as*const Vec<bool> as*const ());
    if let Some(meta)=owner.mutation_meta.first(){let field=meta.canonical_tree_child(11).unwrap();assert_eq!(field as*const dyn ArtifactCanonicalJsonTree as*const (),&meta.transaction as*const Option<TransactionRef> as*const ());}
    owner.canonical_tree_node().unwrap()
   });assert!(matches!(fields,Node::Object(_)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let projection=projected(source.borrow().project(0,|owner:&Edit<bool>|owner as&dyn ArtifactCanonicalJsonTree));
   let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();
   let((mut cursor,progress),heap)=observe_heap_allocations_on_this_thread(||ArtifactCanonicalJsonTreeCursor::admit(projection,grant(demand)).unwrap_or_else(|(error,_)|panic!("original protocol tree: {error}")));
   assert_eq!(progress.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let(zero,_,spent)=effected(RetainedCloneGrant::default(),|effect|{let step=cursor.advance(&mut[0],effect)?;Ok((step.written_bytes,step.ownership))});assert_eq!(zero,0);assert_eq!(spent,(0,0));
   let mut bytes=Vec::new();let mut turns=0;
   while !cursor.terminal_is_empty(){
    assert!(turns<100000);if pause==Some(turns){cursor.begin_close();}
    let(demand,heap)=observe_heap_allocations_on_this_thread(||cursor.next_demand().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let admitted=grant(demand);let mut output=[0xa5];
    let(written,ownership,_)=effected(admitted,|effect|{let step=cursor.advance(&mut output,effect)?;Ok((step.written_bytes,step.ownership))});assert!(ownership.progress().fits(admitted));
    bytes.extend_from_slice(&output[..written]);turns+=1;
   }
   if pause.is_none(){assert_eq!(bytes,expected);}else{assert_eq!(bytes,expected[..bytes.len()]);}
   let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   drained(&mut source);
   assert!(source.terminal_is_empty());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }
}
