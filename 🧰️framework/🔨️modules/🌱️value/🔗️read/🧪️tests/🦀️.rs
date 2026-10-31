//! 🧪️ Original sparse visitation and physical read ownership share one portable fixture.
use semio_framework_value_read::*;
use semio_framework_value_read::ErasedSnapshotRetirement;
use std::mem::size_of;
use semio_framework_value_read::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::arc_custody::SharedControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use std::sync::Arc;

fn fixture()->serde_json::Value{let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();assert_eq!(schema["const"],f);f}
struct CloseBank{stock:RetainedCloneGrant,spent:RetainedCloneProgress,work:u64,fuel:u64}
impl CloseBank{
 fn new()->Self{let row:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/case.json")).unwrap();let s=&row["stock"];Self{stock:RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},spent:Default::default(),work:0,fuel:s["fuel"].as_u64().unwrap()}}
 fn grant(&self,offer:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:offer.maximum_items.min(self.stock.maximum_items-self.spent.copied_items).min(self.fuel.saturating_sub(self.work)as usize),maximum_copy_bytes:offer.maximum_copy_bytes.min(self.stock.maximum_copy_bytes-self.spent.copied_bytes),maximum_capacity_bytes:offer.maximum_capacity_bytes.min(self.stock.maximum_capacity_bytes-self.spent.retained_capacity_bytes),maximum_release_bytes:offer.maximum_release_bytes.min(self.stock.maximum_release_bytes-self.spent.released_bytes),maximum_depth:offer.maximum_depth.min(self.stock.maximum_depth)}}
 fn debit(&mut self,grant:RetainedCloneGrant,p:RetainedCloneProgress,work:u64,heap:(usize,usize)){assert!(p.fits(grant));assert!(work<=grant.maximum_items as u64);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));self.spent=self.spent.checked_add(p).unwrap();self.work=self.work.checked_add(work).unwrap();assert!(self.spent.fits(self.stock)&&self.work<=self.fuel);}
}
fn finish_source<T:Sync+'static,A:semio_framework_value_read::retirement::RetireOwned+Sync>(mut admission:retained_clone::RetainedCloneSourceAdmission<T,A>,bank:&mut CloseBank)->(retained_clone::RetainedCloneSource<T,A>,usize){
 let mut births=0;for _ in 0..100000{if admission.phase()=="ready"{break}let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let(turn,heap)=observe_retirement_allocations(||admission.advance(g));let p=turn.result.unwrap().progress();bank.debit(g,p,turn.work,heap);assert_eq!(p.copied_bytes,0);births+=heap.0;}
 assert_eq!(admission.phase(),"ready");let mut output=None;let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let(turn,heap)=observe_retirement_allocations(||admission.take_output(&mut output,g));bank.debit(g,turn.result.unwrap().progress(),turn.work,heap);assert!(admission.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(admission)).1,(0,0));(output.unwrap(),births)
}
fn reject_source<T:Sync+'static,A:semio_framework_value_read::retirement::RetireOwned+Sync>(mut admission:retained_clone::RetainedCloneSourceAdmission<T,A>,axis:&str,bank:&mut CloseBank)->A{
 let d=admission.next_demand().unwrap();let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let short=match axis{"items"=>RetainedCloneGrant{maximum_items:0,..g},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g},"depth"=>RetainedCloneGrant{maximum_depth:0,..g},_=>unreachable!()};let(turn,heap)=observe_retirement_allocations(||admission.advance(short));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));let mut output=None;
 for _ in 0..100000{if admission.terminal_is_empty(){break}let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let(turn,heap)=observe_retirement_allocations(||admission.return_inputs(&mut output,g));bank.debit(g,turn.result.unwrap().progress(),turn.work,heap);}assert!(admission.terminal_is_empty());output.unwrap().0
}
fn drain<T:semio_framework_value_read::ErasedSnapshotRetirement>(owner:&mut T,copy:usize,bank:&mut CloseBank)->(usize,usize,usize) {
    let(mut births,mut released,mut copied)=(0,0,0);
    for turn in 0..100000 {
        if owner.terminal_is_empty(){return(births,released,copied);}
        let g=bank.grant(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()});
        let(zero,heap)=observe_retirement_allocations(||owner.close_step(RetainedCloneGrant {maximum_items:0,..g}));assert_eq!(zero.work,0);assert_eq!(zero.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
        for denied in [(g.maximum_capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(1),..g}),(g.maximum_release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:g.maximum_release_bytes.saturating_sub(1),..g})].into_iter().flatten(){let(turn,heap)=observe_retirement_allocations(||owner.close_step(denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
        let(outcome,heap)=observe_retirement_allocations(||owner.close_step(g));let work=outcome.work;let step=outcome.result.unwrap();let p=step.progress();bank.debit(g,p,work,heap);births+=heap.0;released+=heap.1;copied+=p.copied_bytes;assert!(p.copied_items!=0||owner.terminal_is_empty(),"exact read owner stalled at turn {turn}");
    }
    panic!("full read ownership did not close");
}
fn recover_source<A:semio_framework_value_read::retirement::RetireOwned+Sync>(source:&mut semio_framework_value_read::retained_clone::RetainedCloneSource<String,A>,births:&mut usize,released:&mut usize,bank:&mut CloseBank)->A {
    for _ in 0..100000 {
        let demand=source.next_take_demand(4096).unwrap();
        let grant=bank.grant(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth});assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),demand);
        let(turn,heap)=observe_retirement_allocations(||source.take_authority(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(turn.work,0);assert_eq!(heap,(0,0));match turn.result{Ok(semio_framework_value_read::retained_clone::RetainedCloneSourceTake::Pending(progress))=>assert_eq!(progress,Default::default()),Err(_)=>{},_=>panic!("unfunded original read transfer")}assert_eq!(source.next_take_demand(4096).unwrap(),demand);
        let(turn,heap)=observe_retirement_allocations(||source.take_authority(grant));let work=turn.work;let result=turn.result.unwrap();let progress=match &result{semio_framework_value_read::retained_clone::RetainedCloneSourceTake::Pending(progress)|semio_framework_value_read::retained_clone::RetainedCloneSourceTake::Ready(_,progress)=>*progress};bank.debit(grant,progress,work,heap);*births+=heap.0;*released+=heap.1;
        if let semio_framework_value_read::retained_clone::RetainedCloneSourceTake::Ready(original,_)=result{assert!(source.terminal_is_empty());assert!(source.next_take_demand(4096).is_err());return original;}
    }
    panic!("original read capsule handback did not complete");
}
#[test]
fn typed_read_registry_retains_every_original_sparse_wrap_and_starvation_vector() {
    let law=fixture();
    for row in law["cases"].as_array().unwrap() {let mut bank=CloseBank::new();
        let(registry,p)=ReadOwnershipRegistry::<u64>::admit(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadOwnershipRegistry::<u64>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()}).unwrap();assert_eq!(p.retained_capacity_bytes,ReadOwnershipRegistry::<u64>::constructor_capacity_bytes());
        let mut leases:Vec<_>=(0..row["issued"].as_u64().unwrap()).map(|v|Some(registry.try_issue(Arc::new(v),RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("exact read issue")).0)).collect();
        registry.set_cleanup_cursor(row["cursor"].as_u64().unwrap()as usize).unwrap();
        for index in row["returned"].as_array().unwrap(){let mut read=leases[index.as_u64().unwrap()as usize].take().unwrap();drain(&mut read,3,&mut bank);}
        let expected:Vec<Option<u64>>=serde_json::from_value(row["visits"].clone()).unwrap();
        let actual:Vec<_>=expected.iter().map(|_|{let(root,p)=registry.take_returned(RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();assert_eq!(p.copied_items,1);root.map(|root|*root)}).collect();assert_eq!(actual,expected,"{}",row["name"]);
        for read in leases.iter_mut().filter_map(Option::as_mut){drain(read,3,&mut bank);}
        let mut close=SharedControlledRetirement::new(registry);drain(&mut close,3,&mut bank);assert!(close.terminal_is_empty());
    }
}

#[test]
fn typed_read_registry_ctor_return_source_and_cancel_conserve_actual_heap_with_full_grants() {
    let law=fixture();
    for copy in law["workBytes"].as_array().unwrap(){for text in law["texts"].as_array().unwrap(){
        let mut bank=CloseBank::new();let copy=copy.as_u64().unwrap()as usize;let text=text.as_str().unwrap();let capacity=ReadOwnershipRegistry::<String>::constructor_capacity_bytes();
        for denied in [RetainedCloneGrant::default(),RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity-1,maximum_depth:1,..Default::default()},RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity,..Default::default()}]{let(refusal,heap)=observe_retirement_allocations(||ReadOwnershipRegistry::<String>::admit(denied));assert!(refusal.is_err());assert_eq!(heap,(0,0));}
        let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:capacity,maximum_depth:1,..Default::default()};let((registry,p),heap)=observe_retirement_allocations(||ReadOwnershipRegistry::<String>::admit(g).unwrap());assert!(p.fits(g));assert_eq!(heap,(capacity,0));let mut births=capacity;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new(text.to_owned()));let original=heap.0;assert_eq!(heap.1,0);let pointer=Arc::as_ptr(&root);
        let((_,root),heap)=observe_retirement_allocations(||registry.try_issue(root,RetainedCloneGrant::default()).err().unwrap());assert_eq!(Arc::as_ptr(&root),pointer);assert_eq!(heap,(0,0));
        let((mut read,p),heap)=observe_retirement_allocations(||registry.try_issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("typed original root issue")));assert_eq!(heap,(0,0));assert_eq!(p,RetainedCloneProgress {copied_items:1,..Default::default()});assert_eq!(read.get().as_str(),text);assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(read.get()).unwrap()).unwrap(),text);
        let source_bytes=read.source_capacity_bytes();let source_copy=read.source_copy_bytes();
        let admission=read.source_admission().unwrap_or_else(|_|panic!("actual read admission"));let mut capsule=reject_source(admission,"capacity",&mut bank);let mut returned=None;let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut returned,g));bank.debit(g,turn.result.unwrap().progress(),turn.work,heap);read=returned.unwrap();assert_eq!(read.get()as*const String,pointer);
        let(mut source,source_born)=finish_source(read.source_admission().unwrap_or_else(|_|panic!("actual typed read source")),&mut bank);assert_eq!(source_born,source_bytes);assert_eq!(source_copy,0);births+=source_born;assert_eq!(source.borrow().get()as*const String,pointer);assert_eq!(source.borrow().get(),text);
        let mut capsule=recover_source(&mut source,&mut births,&mut released,&mut bank);let handoff=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_depth:1,..Default::default()};assert!(law["sourceHandbackCases"].as_array().unwrap().iter().any(|v|v["axis"]=="copy"&&v["admitted"]==true&&v["maximumCopyBytes"]==0));let mut returned=None;for currency in law["sourceHandbackCases"].as_array().unwrap().iter().filter(|v|v["admitted"]==false){let denied=match currency["axis"].as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..handoff},"depth"=>RetainedCloneGrant{maximum_depth:0,..handoff},_=>unreachable!()};let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut returned,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(returned.is_none());assert_eq!(heap,(0,0));assert_eq!(capsule.get()as*const String,pointer);}
        let handoff=bank.grant(handoff);let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut returned,handoff));let p=turn.result.unwrap().progress();bank.debit(handoff,p,turn.work,heap);assert_eq!(p.copied_bytes,0);assert_eq!(heap,(0,0));let mut read=returned.unwrap();assert_eq!(read.get()as*const String,pointer);
        let(b,r,_)=drain(&mut read,copy,&mut bank);births+=b;released+=r;let mut close=SharedControlledRetirement::lease(registry);let(b,r,_)=drain(&mut close,copy,&mut bank);births+=b;released+=r;assert_eq!(released,original+births);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
    }}
}

#[test]
fn erased_read_authority_retains_typed_roots_and_every_alias_until_full_physical_closure() {
    let law=fixture();
    for count in law["authorityAliases"].as_array().unwrap(){for copy in law["workBytes"].as_array().unwrap(){
        let mut bank=CloseBank::new();let count=count.as_u64().unwrap()as usize;let copy=copy.as_u64().unwrap()as usize;let bytes=ReadAuthority::constructor_capacity_bytes::<String>();
        let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};let(refused,heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g}));assert!(refused.is_err());assert_eq!(heap,(0,0));
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(g).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        assert!(authority.publish_authority(19,[17;32]));assert!(authority.authority_matches(19,[17;32]));assert!(!authority.authority_matches(20,[17;32]));
        let(root,heap)=observe_retirement_allocations(||Arc::new("typed λ root".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);
        let((mut read,p),heap)=observe_retirement_allocations(||authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("erased authority keeps exact concrete registry")));assert_eq!(heap,(0,0));assert_eq!(p.copied_items,1);assert_eq!(read.get()as*const String,pointer);assert!(read.commit_authority_matches(19,[17;32]));
        let(_,heap)=observe_retirement_allocations(||authority.admit_clone(RetainedCloneGrant::default()).err().unwrap());assert_eq!(heap,(0,0));
        let mut aliases:Vec<_>=(0..count).map(|_|{let((alias,p),heap)=observe_retirement_allocations(||authority.admit_clone(RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(0,0));assert_eq!(p,RetainedCloneProgress {copied_items:1,..Default::default()});alias}).collect();
        let(b,r,_)=drain(&mut read,copy,&mut bank);births+=b;released+=r;
        let(b,r,_)=drain(&mut authority,copy,&mut bank);births+=b;released+=r;
        for alias in &mut aliases{let(b,r,_)=drain(alias,copy,&mut bank);births+=b;released+=r;}
        assert_eq!(released,original+births);
    }}
}

#[test]
fn erased_read_admission_preserves_original_pointer_and_cancelled_full_frame_receipts() {
    let law=fixture();
    for copy in law["workBytes"].as_array().unwrap(){for cancel in law["erasedCancelAt"].as_array().unwrap(){
        let mut bank=CloseBank::new();let copy=copy.as_u64().unwrap()as usize;let cancel=cancel.as_u64().unwrap()as usize;
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new("erased λ🙂".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);let(read,_)=authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("original erased read issue"));
        let bytes=ErasedReadLease::constructor_capacity_bytes::<String>();let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};let((_,read),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g}).err().unwrap());assert_eq!(heap,(0,0));assert_eq!(read.get()as*const String,pointer);
        let((mut read,p),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,g).unwrap_or_else(|_|panic!("exact erased read frame admission")));assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert!(read.get::<u64>().is_none());assert_eq!(serde_json::to_value(read.get::<String>().unwrap()).unwrap(),"erased λ🙂");
        for _ in 0..cancel{if read.terminal_is_empty(){break;}let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:read.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:read.next_release_byte_demand().unwrap(),maximum_depth:read.next_depth_demand().unwrap()};let g=bank.grant(g);let(turn,heap)=observe_retirement_allocations(||read.close_step(g));let work=turn.work;let step=turn.result.unwrap();bank.debit(g,step.progress(),work,heap);assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;released+=heap.1;}
        let(b,r,_)=drain(&mut read,copy,&mut bank);births+=b;released+=r;let(b,r,_)=drain(&mut authority,copy,&mut bank);births+=b;released+=r;assert_eq!(released,original+births);
    }}
}

#[test]
fn erased_read_authority_close_order_preserves_original_root_and_full_physical_receipts() {
    let law=fixture();
    for copy in law["workBytes"].as_array().unwrap(){for order in law["closeOrders"].as_array().unwrap(){
        let mut bank=CloseBank::new();let copy=copy.as_u64().unwrap()as usize;let order=order.as_str().unwrap();
        let((mut authority,p),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
        let(root,heap)=observe_retirement_allocations(||Arc::new("last-reader λ🙂".to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);
        let(read,_)=authority.issue(root,RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("close-order original read issue"));
        let((mut read,p),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ErasedReadLease::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("close-order exact frame admission")));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);
        if order=="authority-first"{let(b,r,_)=drain(&mut authority,copy,&mut bank);births+=b;released+=r;assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert_eq!(serde_json::to_value(read.get::<String>().unwrap()).unwrap(),"last-reader λ🙂");}
        let(b,r,_)=drain(&mut read,copy,&mut bank);births+=b;released+=r;
        if order=="read-first"{let(b,r,_)=drain(&mut authority,copy,&mut bank);births+=b;released+=r;}
        assert_eq!(released,original+births);
    }}
}

#[test]
fn erased_read_source_retains_original_id_commit_authority_and_root_after_read_return(){
    let law=fixture();let vector=&law["erasedSource"];
    for copy in law["workBytes"].as_array().unwrap(){
        let mut bank=CloseBank::new();let copy=copy.as_u64().unwrap()as usize;let generation=vector["generation"].as_u64().unwrap();let revision=[vector["revisionOctet"].as_u64().unwrap()as u8;32];
        let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ReadAuthority::constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()};let((mut authority,receipt),heap)=observe_retirement_allocations(||ReadAuthority::admit::<String>(grant).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=(heap.0,heap.1);assert!(authority.publish_authority(generation,revision));
        let(root,heap)=observe_retirement_allocations(||Arc::new(vector["text"].as_str().unwrap().to_owned()));let original=heap.0;let pointer=Arc::as_ptr(&root);let(mut read,_)=authority.issue(root,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("original source read issue"));let id=read.id();
        let bytes=ErasedReadLease::constructor_capacity_bytes::<String>();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        for currency in vector["constructorRefusals"].as_array().unwrap(){let short=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,short));let(_,returned)=result.err().unwrap();read=returned;assert_eq!(heap,(0,0));assert_eq!(read.id(),id);assert_eq!(read.get()as*const String,pointer);assert!(read.commit_authority_matches(generation,revision));}
        let((mut read,receipt),heap)=observe_retirement_allocations(||ErasedReadLease::admit(read,grant).unwrap_or_else(|_|panic!("exact source erased frame")));assert_eq!(heap,(receipt.retained_capacity_bytes,0));born+=heap.0;assert_eq!(read.id(),Some(id));assert!(read.commit_authority_matches(generation,revision));assert!(!read.commit_authority_matches(generation+1,revision));assert!(read.get::<u64>().is_none());
        let(result,heap)=observe_retirement_allocations(||read.source_capacity_bytes::<u64>());assert!(result.is_err());assert_eq!(heap,(0,0));let(result,heap)=observe_retirement_allocations(||read.source_admission::<u64>());let(_,returned)=result.err().unwrap();read=returned;assert_eq!(heap,(0,0));assert_eq!(read.get::<String>().unwrap()as*const String,pointer);
        let(bytes,heap)=observe_retirement_allocations(||read.source_capacity_bytes::<String>().unwrap());assert_eq!(heap,(0,0));let source_copy=read.source_copy_bytes::<String>();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source_copy,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        for currency in vector["constructorRefusals"].as_array().unwrap(){let admission=read.source_admission::<String>().unwrap_or_else(|_|panic!("actual erased read source"));let mut capsule=reject_source(admission,currency.as_str().unwrap(),&mut bank);let mut output=None;let g=bank.grant(RetainedCloneGrant{maximum_items:1,..bank.stock});let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut output,g));bank.debit(g,turn.result.unwrap().progress(),turn.work,heap);read=output.unwrap();assert_eq!(read.id(),Some(id));assert_eq!(read.get::<String>().unwrap()as*const String,pointer);assert!(read.commit_authority_matches(generation,revision));}
        let(mut source,source_born)=finish_source(read.source_admission::<String>().unwrap_or_else(|_|panic!("actual erased read source")),&mut bank);assert_eq!(source_born,bytes);assert_eq!(source_copy,0);born+=source_born;
        let(b,r,_)=drain(&mut authority,copy,&mut bank);born+=b;released+=r;assert!(authority.terminal_is_empty());assert_eq!(source.borrow().get()as*const String,pointer);assert_eq!(source.borrow().get().as_str(),vector["text"].as_str().unwrap());
        let mut capsule=recover_source(&mut source,&mut born,&mut released,&mut bank);let handoff=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_depth:1,..Default::default()};assert!(law["sourceHandbackCases"].as_array().unwrap().iter().any(|v|v["axis"]=="copy"&&v["admitted"]==true&&v["maximumCopyBytes"]==0));let mut returned=None;for currency in law["sourceHandbackCases"].as_array().unwrap().iter().filter(|v|v["admitted"]==false){let denied=match currency["axis"].as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..handoff},"depth"=>RetainedCloneGrant{maximum_depth:0,..handoff},_=>unreachable!()};let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut returned,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(returned.is_none());assert_eq!(heap,(0,0));assert_eq!(capsule.get()as*const String,pointer);}
        let handoff=bank.grant(handoff);let(turn,heap)=observe_retirement_allocations(||capsule.return_read(&mut returned,handoff));let p=turn.result.unwrap().progress();bank.debit(handoff,p,turn.work,heap);assert_eq!(p.copied_bytes,0);assert_eq!(heap,(0,0));let mut read=returned.unwrap();assert_eq!(read.id(),Some(id));assert!(read.commit_authority_matches(generation,revision));assert_eq!(read.get::<String>().unwrap()as*const String,pointer);let(b,r,_)=drain(&mut read,copy,&mut bank);born+=b;released+=r;
        assert!(source.terminal_is_empty());assert_eq!(released,original+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));assert_eq!(observe_retirement_allocations(||drop(read)).1,(0,0));assert_eq!(observe_retirement_allocations(||drop(authority)).1,(0,0));
    }
}
