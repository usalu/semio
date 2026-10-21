//! 🧪️ Actual original4096 native rows keep their payload pointers through every page birth and interruption.
use super::*;
use crate::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[derive(Debug,PartialEq,Eq)]
struct Row{key:u64,body:[u8;88]}
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(v,h)=crate::observe_retirement_allocations(body);(v,h)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:17,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64}}
fn close<T,const N:usize>(owner:&mut PagedList<T,N>,g:RetainedCloneGrant)->usize{
 while !owner.is_empty(){assert!(owner.next_pop_depth_demand().unwrap()<=g.maximum_depth);let(_,h)=observe(||owner.truncate_retired_last().unwrap());assert_eq!(h,(0,0));assert!(RetainedCloneProgress{copied_items:1,..Default::default()}.fits(g))}
 let mut released=0;while !owner.terminal_is_empty(){let demand=owner.next_release_allocation_bytes().unwrap();assert!(owner.next_release_depth_demand().unwrap()<=g.maximum_depth);if demand>0{let(denied,h)=observe(||owner.release_empty_page(demand-1).unwrap());assert!(!denied.progressed);assert_eq!(h,(0,0))}let(p,h)=observe(||owner.release_empty_page(g.maximum_release_bytes).unwrap());assert!(p.progressed);assert_eq!(h,(0,p.released_allocation_bytes));assert!(RetainedCloneProgress{copied_items:1,released_bytes:p.released_allocation_bytes,..Default::default()}.fits(g));released+=h.1}released
}
#[test]
fn original_paged_list_reservation_preserves_every4096_pointer_and_partial_final_extent(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(size_of::<Row>(),law["rowBytes"].as_u64().unwrap()as usize);let g=grant();
 for cut in law["interruptAfter"].as_array().unwrap(){let cut=cut.as_u64().unwrap()as usize;let mut owner=PagedList::<Row,4096>::empty();let mut pointers=Vec::with_capacity(4096);let mut oracle=std::collections::VecDeque::with_capacity(4096);let mut born=0;
  for key in 0..cut as u64{while !owner.has_reserved_slot(){let bytes=owner.next_allocation_bytes().unwrap();let depth=owner.next_reserve_depth_demand().unwrap();assert!(bytes<=g.maximum_capacity_bytes);assert!(depth<=g.maximum_depth);assert_eq!(owner.next_reserve_copy_byte_demand().unwrap(),0);let before=(owner.len(),owner.capacity(),owner.allocated_bytes());for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:depth-1,..g},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..g}]{let(p,h)=observe(||owner.reserve_one_funded(denied).unwrap());assert_eq!(p,Default::default());assert_eq!(h,(0,0));assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes()),before)}let(p,h)=observe(||owner.reserve_one_funded(RetainedCloneGrant{maximum_copy_bytes:0,..g}).unwrap());assert!(p.fits(g));assert_eq!(p.copied_items,1);assert_eq!(p.copied_bytes,0);assert_eq!(h,(p.retained_capacity_bytes,0));assert_eq!(p.retained_capacity_bytes,bytes);born+=h.0;for(index,pointer)in pointers.iter().enumerate(){assert_eq!(owner.backing_ptr(index),Some(*pointer));}}
   let mut incoming=Some(Row{key,body:[key as u8;88]});assert!(owner.next_push_depth_demand().unwrap()<=g.maximum_depth);let(p,h)=observe(||owner.adopt_reserved(&mut incoming).unwrap());assert!(p.progressed);assert_eq!(p.placed_bytes,0);assert_eq!(h,(0,0));assert!(incoming.is_none());assert!(RetainedCloneProgress{copied_items:1,..Default::default()}.fits(g));pointers.push(owner.backing_ptr(owner.len()-1).unwrap());oracle.push_back(key);
  }
  assert_eq!(owner.iter().map(|row|row.key).collect::<Vec<_>>(),oracle.iter().copied().collect::<Vec<_>>());for(index,pointer)in pointers.iter().enumerate(){assert_eq!(owner.backing_ptr(index),Some(*pointer));assert_eq!(owner[index].body,[index as u8;88]);}let released=close(&mut owner,g);assert_eq!(born,released);let(_,h)=observe(||drop(owner));assert_eq!(h,(0,0));
 }
 let mut partial=PagedList::<Row,4096>::empty();let mut born=0;while partial.capacity()<13{let(p,h)=observe(||partial.reserve_capacity_one_funded(13,g).unwrap());assert!(p.fits(g));assert_eq!(h,(p.retained_capacity_bytes,0));assert_eq!(p.copied_bytes,0);born+=h.0;}for key in 0..13{let mut input=Some(Row{key,body:[0;88]});partial.adopt_reserved(&mut input).unwrap();}let pointer=partial.backing_ptr(0).unwrap();let(denied,h)=observe(||partial.reserve_one_funded(g));assert!(denied.is_err());assert_eq!(h,(0,0));assert_eq!(partial.backing_ptr(0),Some(pointer));assert_eq!(partial.capacity(),13);assert_eq!(born,close(&mut partial,g));let(_,h)=observe(||drop(partial));assert_eq!(h,(0,0));
 let mut too_wide=PagedList::<[u8;65537],1>::empty();let(p,h)=observe(||too_wide.reserve_one_funded(g).unwrap());assert_eq!(p.copied_bytes,0);assert_eq!(h,(p.retained_capacity_bytes,0));let born=h.0;let(p,h)=observe(||too_wide.reserve_one_funded(g).unwrap());assert_eq!(p,Default::default());assert_eq!(h,(0,0));assert_eq!(too_wide.capacity(),0);assert_eq!(born,close(&mut too_wide,g));let(_,h)=observe(||drop(too_wide));assert_eq!(h,(0,0));
}

struct NoMetadataAllocation;
unsafe impl PageAllocation for NoMetadataAllocation{fn reserve<T>(_:&mut Vec<T>,_:usize)->Result<(),std::collections::TryReserveError>{Ok(())}}
struct PartialPayloadAllocation;
unsafe impl PageAllocation for PartialPayloadAllocation{fn reserve<T>(owner:&mut Vec<T>,slots:usize)->Result<(),std::collections::TryReserveError>{owner.try_reserve_exact(slots/2)}}

#[test]
fn allocation_policy_refuses_empty_metadata_without_hidden_push_allocation(){
 let mut owner=PagedList::<u64,4>::empty();let demand=owner.next_allocation_bytes().unwrap();let(error,heap)=observe(||owner.reserve_one_using::<NoMetadataAllocation>(demand).unwrap_err());assert_eq!(error.kind,PagedListRefusalKind::InvariantViolated);assert_eq!(error.allocated_bytes,0);assert_eq!(heap,(0,0));assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes()),(0,0,0));assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
}

#[test]
fn allocation_policy_quarantines_partial_payload_until_exact_funded_release(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let policy=&law["allocationPolicy"];let row=&policy["cases"][1];assert_eq!(size_of::<u64>(),policy["elementBytes"].as_u64().unwrap()as usize);let mut owner=PagedList::<u64,4>::empty();let metadata=owner.next_allocation_bytes().unwrap();let(p,heap)=observe(||owner.reserve_one(metadata).unwrap());assert_eq!(heap,(p.allocated_bytes,0));let root=owner.root.as_ptr();let requested=owner.next_allocation_bytes().unwrap();assert_eq!(requested,policy["requestedSlots"].as_u64().unwrap()as usize*size_of::<u64>());
 let(error,heap)=observe(||owner.reserve_one_using::<PartialPayloadAllocation>(requested).unwrap_err());let retained=row["retainedBytes"].as_u64().unwrap()as usize;assert_eq!(error.kind,PagedListRefusalKind::InvariantViolated);assert_eq!(error.allocated_bytes,retained);assert_eq!(heap,(retained,0));assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes()),(0,0,metadata+retained));assert!(!owner.has_reserved_slot());assert_eq!(owner.root.as_ptr(),root);
 let(error,heap)=observe(||owner.reserve_one(requested).unwrap_err());assert_eq!(error.allocated_bytes,0);assert_eq!(heap,(0,0));assert_eq!(owner.root.as_ptr(),root);assert_eq!(owner.next_release_allocation_bytes().unwrap(),retained);assert_eq!(owner.next_release_depth_demand().unwrap(),3);let(p,heap)=observe(||owner.release_empty_page(retained-1).unwrap());assert!(!p.progressed);assert_eq!(heap,(0,0));assert_eq!(owner.allocated_bytes(),metadata+retained);
 let(p,heap)=observe(||owner.release_empty_page(retained).unwrap());assert!(p.progressed);assert_eq!(p.released_allocation_bytes,retained);assert_eq!(heap,(0,retained));assert_eq!((owner.capacity(),owner.allocated_bytes()),(0,metadata));assert_eq!(owner.next_release_depth_demand().unwrap(),2);assert_eq!(owner.root.as_ptr(),root);assert_eq!(close(&mut owner,grant()),metadata);assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
}

#[test]
fn allocation_policy_partial_payload_hands_back_without_hidden_disposal(){
 let mut owner=PagedList::<u64,4>::empty();let metadata=owner.next_allocation_bytes().unwrap();owner.reserve_one(metadata).unwrap();let requested=owner.next_allocation_bytes().unwrap();let(error,heap)=observe(||owner.reserve_one_using::<PartialPayloadAllocation>(requested).unwrap_err());let retained=error.allocated_bytes;assert_eq!(retained,2*size_of::<u64>());assert_eq!(heap,(retained,0));let mut parent=semio_framework_value_allocation_return::ParentAllocationReturn::<2>::try_new(65536,131072).unwrap();
 let(p,heap)=observe(||owner.return_empty_page(&mut parent,0).unwrap());assert!(!p.progressed);assert_eq!(heap,(0,0));assert_eq!(parent.retained_bytes(),0);let(p,heap)=observe(||owner.return_empty_page(&mut parent,1).unwrap());assert!(p.progressed);assert_eq!(p.returned_allocation_bytes,retained);assert_eq!(heap,(0,0));assert_eq!(parent.retained_bytes(),retained);assert_eq!(owner.allocated_bytes(),metadata);assert_eq!(close(&mut owner,grant()),metadata);
 let(_,heap)=observe(||parent.close_step(1,retained-1));assert_eq!(heap,(0,0));assert_eq!(parent.retained_bytes(),retained);let(_,heap)=observe(||parent.close_step(1,retained));assert_eq!(heap,(0,retained));assert!(parent.terminal_is_empty());let(_,heap)=observe(||drop((owner,parent)));assert_eq!(heap,(0,0));
}
