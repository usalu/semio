//! 🧪️ Original sparse visitation and physical read ownership share one portable fixture.
use super::*;
use crate::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::shared::SharedControlledRetirement,value::observe_retirement_allocations};
use std::sync::Arc;

fn drain<T:crate::ErasedSnapshotRetirement>(owner:&mut T,copy:usize)->(usize,usize,usize) {
    let(mut births,mut released,mut copied)=(0,0,0);
    for turn in 0..100000 {
        if owner.terminal_is_empty(){return(births,released,copied);}
        let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let(zero,heap)=observe_retirement_allocations(||owner.close_step(RetainedCloneGrant {maximum_items:0,..g}).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
        for denied in [(g.maximum_capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(1),..g}),(g.maximum_release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:g.maximum_release_bytes.saturating_sub(1),..g})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
        let(step,heap)=observe_retirement_allocations(||owner.close_step(g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));births+=heap.0;released+=heap.1;copied+=p.copied_bytes;assert!(p.copied_items!=0||owner.terminal_is_empty(),"exact read owner stalled at turn {turn}");
    }
    panic!("full read ownership did not close");
}

#[test]
fn typed_read_registry_retains_every_original_sparse_wrap_and_starvation_vector() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let(registry,p)=ReadOwnershipRegistry::<u64>::admit(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadOwnershipRegistry::<u64>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()}).unwrap();assert_eq!(p.retained_capacity_bytes,ReadOwnershipRegistry::<u64>::constructor_capacity_bytes());
        let mut leases:Vec<_>=(0..row["issued"].as_u64().unwrap()).map(|v|Some(registry.try_issue(Arc::new(v),RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("exact read issue")).0)).collect();
        registry.set_cleanup_cursor(row["cursor"].as_u64().unwrap()as usize).unwrap();
        for index in row["returned"].as_array().unwrap(){let mut read=leases[index.as_u64().unwrap()as usize].take().unwrap();drain(&mut read,3);}
        let expected:Vec<Option<u64>>=serde_json::from_value(row["visits"].clone()).unwrap();
        let actual:Vec<_>=expected.iter().map(|_|{let(root,p)=registry.take_returned(RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();assert_eq!(p.copied_items,1);root.map(|root|*root)}).collect();assert_eq!(actual,expected,"{}",row["name"]);
        for read in leases.iter_mut().filter_map(Option::as_mut){drain(read,3);}
        let mut close=SharedControlledRetirement::new(registry);drain(&mut close,3);assert!(close.terminal_is_empty());
    }
    eprintln!("[DEBUG] Typed read registry preserved all original sparse/wrap/starvation vectors");
}

#[test]
fn typed_read_registry_ctor_return_source_and_cancel_conserve_actual_heap_with_full_grants() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for copy in law["workBytes"].as_array().unwrap(){for text in law["texts"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let text=text.as_str().unwrap();let capacity=ReadOwnershipRegistry::<String>::constructor_capacity_bytes();
        for denied in [RetainedCloneGrant::default(),RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity-1,maximum_depth:1,..Default::default()},RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity,..Default::default()}]{let(refusal,heap)=observe_retirement_allocations(||ReadOwnershipRegistry::<String>::admit(denied));assert!(refusal.is_err());assert_eq!(heap,(0,0));}
        let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity,maximum_depth:1,..Default::default()};let((registry,p),heap)=observe_retirement_allocations(||ReadOwnershipRegistry::<String>::admit(g).unwrap());assert!(p.fits(g));assert_eq!(heap,(capacity,0));let mut births=capacity;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new(text.to_owned()));let original=heap.0;assert_eq!(heap.1,0);let pointer=Arc::as_ptr(&root);
        let((_,root),heap)=observe_retirement_allocations(||registry.try_issue(root,RetainedCloneGrant::default()).err().unwrap());assert_eq!(Arc::as_ptr(&root),pointer);assert_eq!(heap,(0,0));
        let((mut read,p),heap)=observe_retirement_allocations(||registry.try_issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("typed original root issue")));assert_eq!(heap,(0,0));assert_eq!(p,RetainedCloneProgress {copied_items:1,..Default::default()});assert_eq!(read.get().as_str(),text);assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(read.get()).unwrap()).unwrap(),text);
        let source_bytes=read.source_capacity_bytes();let(_,heap)=observe_retirement_allocations(||read.admit_source(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:source_bytes-1,maximum_depth:1,..Default::default()}).err().unwrap());assert_eq!(heap,(0,0));assert_eq!(read.get()as*const String,pointer);
        let((mut source,p),heap)=observe_retirement_allocations(||read.admit_source(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:source_bytes,maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert_eq!(source.borrow().get(),text);
        let(b,r,_)=drain(&mut read,copy);births+=b;released+=r;
        let mut close=SharedControlledRetirement::lease(registry);let(b,r,_)=drain(&mut close,copy);births+=b;released+=r;assert!(released<original+births);
        for turn in 0..100000 {if source.terminal_is_empty(){break;}let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||source.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;released+=heap.1;assert!(step.progress().copied_items!=0||source.terminal_is_empty(),"source cancellation stalled {turn}");}
        assert!(source.terminal_is_empty());assert_eq!(released,original+births);eprintln!("[DEBUG] Typed read full ownership copy={copy} original={original} births={births} physical={released}");
    }}
}

#[test]
fn erased_read_authority_retains_typed_roots_and_every_alias_until_full_physical_closure() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for count in law["authorityAliases"].as_array().unwrap(){for copy in law["workBytes"].as_array().unwrap(){
        let count=count.as_u64().unwrap()as usize;let copy=copy.as_u64().unwrap()as usize;let bytes=ReadAuthority::constructor_capacity_bytes::<String>();
        let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};let(refused,heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g}));assert!(refused.is_err());assert_eq!(heap,(0,0));
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(g).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        assert!(authority.publish_authority(19,[17;32]));assert!(authority.authority_matches(19,[17;32]));assert!(!authority.authority_matches(20,[17;32]));
        let(root,heap)=observe_retirement_allocations(||Arc::new("typed λ root".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);
        let((mut read,p),heap)=observe_retirement_allocations(||authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("erased authority keeps exact concrete registry")));assert_eq!(heap,(0,0));assert_eq!(p.copied_items,1);assert_eq!(read.get()as*const String,pointer);assert!(read.commit_authority_matches(19,[17;32]));
        let(_,heap)=observe_retirement_allocations(||authority.admit_clone(RetainedCloneGrant::default()).err().unwrap());assert_eq!(heap,(0,0));
        let mut aliases:Vec<_>=(0..count).map(|_|{let((alias,p),heap)=observe_retirement_allocations(||authority.admit_clone(RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(0,0));assert_eq!(p,RetainedCloneProgress {copied_items:1,..Default::default()});alias}).collect();
        let(b,r,_)=drain(&mut read,copy);births+=b;released+=r;
        let(b,r,_)=drain(&mut authority,copy);births+=b;released+=r;
        for alias in &mut aliases{let(b,r,_)=drain(alias,copy);births+=b;released+=r;}
        assert_eq!(released,original+births);eprintln!("[DEBUG] Erased typed read authority aliases={count} copy={copy} original={original} births={births} physical={released}");
    }}
}

#[test]
fn erased_read_admission_preserves_original_pointer_and_cancelled_full_frame_receipts() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for copy in law["workBytes"].as_array().unwrap(){for cancel in law["erasedCancelAt"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let cancel=cancel.as_u64().unwrap()as usize;
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new("erased λ🙂".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);let(read,_)=authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("original erased read issue"));
        let bytes=ErasedReadLease::constructor_capacity_bytes::<String>();let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};let((_,read),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g}).err().unwrap());assert_eq!(heap,(0,0));assert_eq!(read.get()as*const String,pointer);
        let((mut read,p),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,g).unwrap_or_else(|_|panic!("exact erased read frame admission")));assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert!(read.get::<u64>().is_none());assert_eq!(serde_json::to_value(read.get::<String>().unwrap()).unwrap(),"erased λ🙂");
        for _ in 0..cancel{if read.terminal_is_empty(){break;}let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:read.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:read.next_release_byte_demand().unwrap(),maximum_depth:read.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||read.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;released+=heap.1;}
        let(b,r,_)=drain(&mut read,copy);births+=b;released+=r;let(b,r,_)=drain(&mut authority,copy);births+=b;released+=r;assert_eq!(released,original+births);
        eprintln!("[DEBUG] Erased read physical cancellation copy={copy} at={cancel} original={original} births={births} physical={released}");
    }}
}

#[test]
fn erased_read_authority_close_order_preserves_original_root_and_full_physical_receipts() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for copy in law["workBytes"].as_array().unwrap(){for order in law["closeOrders"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let order=order.as_str().unwrap();
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new("last-reader λ🙂".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);
        let(read,_)=authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("close-order original read issue"));
        let((mut read,p),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ErasedReadLease::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("close-order exact frame admission")));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);
        if order=="authority-first"{let(b,r,_)=drain(&mut authority,copy);births+=b;released+=r;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert_eq!(serde_json::to_value(read.get::<String>().unwrap()).unwrap(),"last-reader λ🙂");}
        let(b,r,_)=drain(&mut read,copy);births+=b;released+=r;
        if order=="read-first"{let(b,r,_)=drain(&mut authority,copy);births+=b;released+=r;}
        assert_eq!(released,original+births);eprintln!("[DEBUG] Erased read close order={order} copy={copy} original={original} births={births} physical={released}");
    }}
}

#[test]
fn erased_read_source_retains_original_id_commit_authority_and_root_after_read_return(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let vector=&law["erasedSource"];
    for copy in law["workBytes"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let generation=vector["generation"].as_u64().unwrap();let revision=[vector["revisionOctet"].as_u64().unwrap()as u8;32];
        let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()};let((mut authority,receipt),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(grant).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=(heap.0,heap.1);assert!(authority.publish_authority(generation,revision));
        let(root,heap)=observe_retirement_allocations(||Arc::new(vector["text"].as_str().unwrap().to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);let(mut read,_)=authority.issue(root,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("original source read issue"));let id=read.id();
        let bytes=ErasedReadLease::constructor_capacity_bytes::<String>();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        for currency in vector["constructorRefusals"].as_array().unwrap(){let short=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,short));let(_,returned)=result.err().unwrap();read=returned;assert_eq!(heap,(0,0));assert_eq!(read.id(),id);assert_eq!(read.get()as*const String,pointer);assert!(read.commit_authority_matches(generation,revision));}
        let((mut read,receipt),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,grant).unwrap_or_else(|_|panic!("exact source erased frame")));assert_eq!(heap,(receipt.retained_capacity_bytes,0));born+=heap.0;assert_eq!(read.id(),Some(id));assert!(read.commit_authority_matches(generation,revision));assert!(!read.commit_authority_matches(generation+1,revision));assert!(read.get::<u64>().is_none());
        let(result,heap)=observe_retirement_allocations(||read.source_capacity_bytes::<u64>());assert!(result.is_err());assert_eq!(heap,(0,0));let(result,heap)=observe_retirement_allocations(||read.admit_source::<u64>(grant));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(read.get::<String>().unwrap()as*const String,pointer);
        let(bytes,heap)=observe_retirement_allocations(||read.source_capacity_bytes::<String>().unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        for currency in vector["constructorRefusals"].as_array().unwrap(){let short=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=observe_retirement_allocations(||read.admit_source::<String>(short));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(read.id(),Some(id));assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert!(read.commit_authority_matches(generation,revision));}
        let((mut source,receipt),heap)=observe_retirement_allocations(||read.admit_source::<String>(grant).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,0));born+=heap.0;
        let(b,r,_)=drain(&mut read,copy);born+=b;released+=r;assert!(read.terminal_is_empty());assert!(read.id().is_none());let(b,r,_)=drain(&mut authority,copy);born+=b;released+=r;assert!(authority.terminal_is_empty());assert_eq!(source.borrow().get()as*const String,pointer);assert_eq!(source.borrow().get().as_str(),vector["text"].as_str().unwrap());
        for _ in 0..100000{if source.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
        assert!(source.terminal_is_empty());assert_eq!(released,original+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));assert_eq!(observe_retirement_allocations(||drop(read)).1,(0,0));assert_eq!(observe_retirement_allocations(||drop(authority)).1,(0,0));eprintln!("[DEBUG] Erased source pins original id={id:?} generation={generation} pointer through read+authority return copy={copy} original={original} born={born} physical={released}");
    }
}
