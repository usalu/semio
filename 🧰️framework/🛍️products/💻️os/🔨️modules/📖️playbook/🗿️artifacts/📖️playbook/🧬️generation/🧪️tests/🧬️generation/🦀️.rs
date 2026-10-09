//! 🧪️ Original generation leases conserve physical backing under independent retirement grants.
use super::*;
use crate::os_store as store;
use store::ErasedSnapshotRetirement;
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
thread_local! {static EVENTS:Cell<Option<(usize,usize)>>=const {Cell::new(None)};}
struct ObservedAllocator;
fn event(allocated:usize,released:usize){let _=EVENTS.try_with(|events|{if let Some((a,r))=events.get(){events.set(Some((a+allocated,r+released)));}});}
unsafe impl GlobalAlloc for ObservedAllocator {
 unsafe fn alloc(&self,layout:Layout)->*mut u8{let p=unsafe{System.alloc(layout)};if !p.is_null(){event(layout.size(),0)}p}
 unsafe fn alloc_zeroed(&self,layout:Layout)->*mut u8{let p=unsafe{System.alloc_zeroed(layout)};if !p.is_null(){event(layout.size(),0)}p}
 unsafe fn realloc(&self,p:*mut u8,layout:Layout,size:usize)->*mut u8{let next=unsafe{System.realloc(p,layout,size)};if !next.is_null(){event(size,layout.size())}next}
 unsafe fn dealloc(&self,p:*mut u8,layout:Layout){event(0,layout.size());unsafe{System.dealloc(p,layout)}}
}
#[global_allocator]static ALLOCATOR:ObservedAllocator=ObservedAllocator;
fn observe<T>(action:impl FnOnce()->T)->(T,(usize,usize)){EVENTS.with(|events|assert!(events.replace(Some((0,0))).is_none()));let result=action();let heap=EVENTS.with(|events|events.replace(None).unwrap());(result,heap)}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()}
fn root(value:&serde_json::Value)->GenerationPlayRoot{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn quoted_grant(owner:&GenerationRootRetirement,baseline:usize)->RetainedCloneGrant{let copy=baseline.max(owner.next_copy_byte_demand().unwrap());RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}}
fn drain(mut owner:GenerationRootRetirement,baseline:usize)->(usize,usize,usize){
 let(mut allocated,mut released,mut turns)=(0,0,0);
 for _ in 0..2_000_000{
  if owner.terminal_is_empty(){break}let grant=quoted_grant(&owner,baseline);
  let(step,heap)=observe(||owner.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
  if let Some(depth)=grant.maximum_depth.checked_sub(1){let(result,heap)=observe(||owner.close_step(RetainedCloneGrant{maximum_depth:depth,..grant}));assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert_eq!(heap,(0,0));assert_eq!(owner.next_depth_demand().unwrap(),grant.maximum_depth);}
  for under in [grant.maximum_capacity_bytes.checked_sub(1).map(|n|RetainedCloneGrant{maximum_capacity_bytes:n,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|n|RetainedCloneGrant{maximum_release_bytes:n,..grant})].into_iter().flatten(){let(step,heap)=observe(||owner.close_step(under).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
  if let Some(copy)=owner.next_copy_byte_demand().unwrap().checked_sub(1){let under=RetainedCloneGrant{maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),..grant};let(result,heap)=observe(||owner.close_step(under));match result{Ok(step)=>{let progress=step.progress();assert!(progress.fits(under));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));allocated+=heap.0;released+=heap.1;turns+=1;},Err(error)=>{assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::WorkLimit);assert_eq!(heap,(0,0));}}}
  if owner.terminal_is_empty(){break}let grant=quoted_grant(&owner,baseline);
  let(step,heap)=observe(||owner.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));allocated+=heap.0;released+=heap.1;turns+=1;
 }
 assert!(owner.terminal_is_empty(),"original generation custody failed to close");let(step,heap)=observe(||owner.close_step(Default::default()).unwrap());assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));(allocated,released,turns)
}
#[test]
fn generation_root_current_physical_final_owner_and_weak_backpressure(){
 let fixture=fixture();let policy:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
 for baseline in policy["baselines"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()){
  let(root,construction)=observe(||root(&fixture["generation"]));let pointer=Arc::as_ptr(root.0.as_ref().unwrap());let weak=Arc::downgrade(root.0.as_ref().unwrap());let(copy,heap)=observe(||root.clone());assert_eq!(heap,(0,0));assert!(root.same_allocation(&copy));assert_eq!(semio_framework_pack_json::to_json_string(&root),semio_framework_pack_json::to_json_string(&copy));
  let(mut first,heap)=observe(||root.into_retirement());assert_eq!(heap,(0,0));let original=quoted_grant(&first,baseline as usize);let(step,heap)=observe(||first.close_step(original).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(&weak.upgrade().unwrap()),pointer);drop(weak);
  let(first_birth,first_release,_)=drain(first,baseline as usize);assert_eq!((first_birth,first_release),(0,0));assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&copy)).unwrap(),fixture["generation"]);
  let(last,heap)=observe(||copy.into_retirement());assert_eq!(heap,(0,0));let(born,released,turns)=drain(last,baseline as usize);assert_eq!(construction.0+born,construction.1+released);eprintln!("[DEBUG] Generation original per-root lease baseline={baseline} initial={:?} childBirth={born} released={released} turns={turns} aliasRelease=0 terminalDrop=0",construction);
 }
}
#[test]
fn generation_values_ranked_source_shares_payload_and_retires_every_alias_order(){
 let fixture=fixture();let policy=&fixture["rankedValues"];let expected:Vec<String>=serde_json::from_value(policy["expectedKeys"].clone()).unwrap();
 for order in policy["aliasReleaseOrders"].as_array().unwrap(){
  let(roots,construction)=observe(||{let values:crate::PlaybookValues=policy["entries"].as_array().unwrap().iter().map(|entry|(entry[0].as_str().unwrap().to_owned(),entry[1].clone().into())).collect();let shared=values.clone();for(index,key)in expected.iter().enumerate(){let a=values.entry_at_rank(index).unwrap();let b=shared.entry_at_rank(index).unwrap();assert_eq!(a.0,key);assert!(std::ptr::eq(a.0,b.0));assert!(std::ptr::eq(a.1,b.1));}let root=GenerationPlayRoot::from(GenerationPlayState{generations:vec![crate::FormGeneration{id:"ranked".into(),name:"Ranked".into(),values}],selected_generation_id:None,preview_text:None});let second=GenerationPlayRoot::from(GenerationPlayState{generations:vec![crate::FormGeneration{id:"ranked".into(),name:"Ranked".into(),values:shared}],selected_generation_id:None,preview_text:None});let third=root.clone();[Some(root),Some(second),Some(third)]});
  let mut roots=roots;let(mut born,mut released)=(0,0);for index in order.as_array().unwrap(){let(close,heap)=observe(||roots[index.as_u64().unwrap()as usize].take().unwrap().into_retirement());assert_eq!(heap,(0,0));let(a,r,_)=drain(close,1);born+=a;released+=r;}assert_eq!(construction.0+born,construction.1+released);eprintln!("[DEBUG] Generation ranked alias order={order} original={:?} childBirth={born} actualRelease={released}",construction);
 }
}
#[test]
fn generation_root_cold_builder_refuses_shared_mutation_without_cloning(){
 let mut root=GenerationPlayRoot::default();root.cold_builder_mut().unwrap().preview_text=Some("mutable cold builder".into());let shared=root.clone();assert_eq!(root.cold_builder_mut().unwrap_err(),"playbook.generation-root-shared");assert!(root.same_allocation(&shared));drain(root.into_retirement(),1);drain(shared.into_retirement(),1);
}
#[test]
fn generation_root_live_final_owner_and_unclosed_cursor_reject_drop_without_double_panic(){
 let fixture=fixture();assert!(std::panic::catch_unwind(||drop(root(&fixture["generation"]))).is_err());assert!(std::panic::catch_unwind(||drop(root(&fixture["generation"]).into_retirement())).is_err());let root=root(&fixture["generation"]);assert!(std::thread::spawn(move||{let _retirement=root.into_retirement();panic!("primary generation lifecycle fault");}).join().is_err());
}
