//! 🧪️ Original send-only payloads and provider-refused values retain exact physical custody.
use super::*;
use crate::{observe_retirement_allocations,retirement::{RetireOwned,RetirementCursor,admit_owned_retirement,owned_retirement_birth_bytes,arc_custody::arc_bytes},FactoryPayloadRetirement};
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
struct RefusingFactory{refuse:AtomicBool,calls:AtomicUsize,pointer:usize}
impl FactoryPayloadRetirement for RefusingFactory{
 type CloseState=bool;
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize {std::mem::size_of::<Self::CloseState>()}
 fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
 fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}
 fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
 fn prepare_close_state(&self)->bool{false}
 fn transfer_payload(_:Self,state:&mut bool){*state=true;}
 fn close_state_demands(_: &bool,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{*state=true;Ok(RetainedCloneStep::Complete(Default::default()))}
 fn close_state_terminal_is_empty(state:&bool)->bool{*state}
}
impl ArtifactOwnedValueRetirementFactory<String> for RefusingFactory{
 fn retirement_birth_bytes(&self,_:&String)->usize{owned_retirement_birth_bytes::<String>()}
 fn retire_owned(&self,original:String,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,String)>{self.calls.fetch_add(1,Ordering::SeqCst);if self.pointer!=0{assert_eq!(original.as_ptr()as usize,self.pointer,"custom issuer must receive exact original backing");}if self.refuse.swap(false,Ordering::SeqCst){Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture issuer refuses once"),original))}else{admit_owned_retirement(original,grant)}}
}
#[test]
fn factory_owned_retirement_preserves_original_provider_refusal_and_exact_terminal_allocations(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let mut payload=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);payload.push_str(&text);let pointer=payload.as_ptr();let mut original=Some(payload);let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(RefusingFactory{refuse:AtomicBool::new(true),calls:AtomicUsize::new(0),pointer:0});let strong=Arc::strong_count(&factory);let held=original.as_ref().unwrap().capacity()+arc_bytes::<RefusingFactory>();
  let bytes=FactoryOwnedRetirement::<String>::constructor_capacity_bytes();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};
  for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (result,heap)=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,under));assert!(result.is_err());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(Arc::strong_count(&factory),strong);assert_eq!(heap,(0,0));}
  let (result,(a,r))=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,grant).unwrap());let (mut frame,p)=result.unwrap();assert!(original.is_none());assert_eq!(frame.original.as_ref().unwrap().as_ptr(),pointer);assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert_eq!(a,bytes);let mut births=a;let mut released=r;let mut refusals=0;
  let (_,heap)=observe_retirement_allocations(||drop(factory));assert_eq!(heap,(0,0));
  for turn in 0..1000000{
   let body=copy.as_u64().unwrap()as usize;let (demand,heap)=observe_retirement_allocations(||frame.demands(body).unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for under in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let (step,heap)=observe_retirement_allocations(||frame.close_step(under).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
   let (step,(a,r))=observe_retirement_allocations(||frame.close_step(grant));match step{Ok(step)=>{let p=step.progress();assert!(p.fits(grant));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;},Err(error)=>{assert_eq!(error.message,"original fixture issuer refuses once");assert_eq!(frame.original.as_ref().unwrap().as_ptr(),pointer);assert_eq!((a,r),(0,0));refusals+=1;}}if frame.terminal_is_empty(){break;}assert!(turn<999999);
   if turn==cut.as_u64().unwrap()as usize{let (same,heap)=observe_retirement_allocations(||frame);frame=same;assert_eq!(heap,(0,0));}
  }
  assert_eq!(refusals,1);assert!(frame.terminal_is_empty());let mut erased=Some(frame as Box<dyn ErasedSnapshotRetirement>);let (demand,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(erased.as_ref().unwrap(),0).unwrap());assert_eq!(heap,(0,0));assert_eq!(demand.release_bytes,bytes);let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:demand.depth,..Default::default()};let (small,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut erased,RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant}).unwrap());assert_eq!(small.progress(),Default::default());assert_eq!(heap,(0,0));let (done,(a,r))=observe_retirement_allocations(||crate::close_factory_ticket(&mut erased,grant).unwrap());assert!(done.progress().fits(grant));assert_eq!((done.progress().retained_capacity_bytes,done.progress().released_bytes),(a,r));births+=a;released+=r;assert!(erased.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((erased,original)));assert_eq!(heap,(0,0));
 }}}
}
struct SendOnly{payload:String,marker:std::cell::Cell<u8>}
impl RetireOwned for SendOnly{
 fn retirement(self)->Box<dyn RetirementCursor>{assert_eq!(self.marker.get(),7);self.payload.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.payload.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
#[test]
fn factory_owned_retirement_accepts_actual_send_only_payloads(){
 let mut original=Some(SendOnly{payload:String::with_capacity(8192),marker:std::cell::Cell::new(7)});let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<SendOnly>>=Arc::new(crate::OwnedValueRetirementFactory::<SendOnly>::default());let held=8192+arc_bytes::<crate::OwnedValueRetirementFactory<SendOnly>>();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:FactoryOwnedRetirement::<SendOnly>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()};let (result,(a,r))=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,grant).unwrap());let (frame,p)=result.unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));let mut births=a;let mut released=r;let (_,heap)=observe_retirement_allocations(||drop(factory));assert_eq!(heap,(0,0));let mut frame=Some(frame as Box<dyn ErasedSnapshotRetirement>);
 for turn in 0..1000000{if frame.is_none(){break;}let demand=crate::factory_ticket_demands(frame.as_ref().unwrap(),1).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(1),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let (step,(a,r))=observe_retirement_allocations(||crate::close_factory_ticket(&mut frame,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));births+=a;released+=r;assert!(turn<999999);}
 assert!(frame.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((frame,original)));assert_eq!(heap,(0,0));
}

/// 🔗️ A genuine projected source preserves its original custom issuer on take, cancellation and provider refusal.
#[test]
fn factory_owned_source_preserves_actual_payload_and_custom_issuer() {
    use crate::retained_clone::{RetainedCloneSource, RetainedCloneSourceTake};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔗️source/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut text=String::with_capacity(row["capacity"].as_u64().unwrap()as usize); text.push_str(row["text"].as_str().unwrap());
        let pointer=text.as_ptr();
        let factory=Arc::new(RefusingFactory{refuse:AtomicBool::new(row["refuseOnce"].as_bool().unwrap()),calls:AtomicUsize::new(0),pointer:pointer as usize});
        let exact:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=factory.clone();
        let issuer=Arc::as_ptr(&exact);
        let mut authority=FactoryOwnedSourceAuthority::new(text,exact);
        assert_eq!(authority.get().as_ptr(),pointer);
        let demand=RetainedCloneSource::<String>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<String>>();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::borrowed_constructor_copy_bytes::<FactoryOwnedSourceAuthority<String>>(),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:demand.depth};
        for axis in law["sourceRefusals"].as_array().unwrap() {
            let denied=match axis.as_str().unwrap() { "items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!() };
            let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_borrowed(authority,|owner|owner.get(),denied));
            let(_,returned)=result.err().expect("denied constructor retains exact source pair");authority=returned;assert_eq!(heap,(0,0));assert_eq!(authority.get().as_ptr(),pointer);assert!(std::ptr::addr_eq(Arc::as_ptr(authority.factory()),issuer));
        }
        let (result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_borrowed(authority,|owner|owner.get(),grant));
        let(mut source,receipt)=result.map_err(|(error,_)|error).unwrap();
        assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes)); assert!(receipt.fits(grant));
        assert_eq!(source.borrow().get().as_ptr(),pointer);
        assert!(std::ptr::addr_eq(Arc::as_ptr(source.try_authority().unwrap().factory()),issuer));
        let project_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:crate::retained_clone::RetainedCloneBinding::alias_copy_bytes(),maximum_depth:1,..Default::default()};
        let visits=std::cell::Cell::new(0);
        for axis in law["projectionRefusals"].as_array().unwrap() {
            let denied=match axis.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..project_grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:project_grant.maximum_copy_bytes-1,..project_grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..project_grant},_=>unreachable!()};
            let(result,heap)=observe_retirement_allocations(||source.project_owned(1,|original|{visits.set(visits.get()+1);original.as_str()},denied));
            assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(visits.get(),0,"denied grant must not invoke projection code");
        }
        let mut refusals=0;
        if row["route"]=="transfer" {
            let(mut projection,progress)=source.project_owned(2,|original|original.as_str(),project_grant).unwrap();assert!(progress.fits(project_grant));assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);
            for axis in law["projectionRefusals"].as_array().unwrap(){
                let denied=match axis.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..project_grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:project_grant.maximum_copy_bytes-1,..project_grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..project_grant},_=>unreachable!()};
                let(result,heap)=observe_retirement_allocations(||projection.project(3,|original|{visits.set(visits.get()+1);original},denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(visits.get(),0,"denied nested projection must not invoke callback");
            }
            for turn in 0..10_000{
                let copy=source.next_take_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};
                match source.take_authority(grant){Ok(RetainedCloneSourceTake::Pending(progress))=>assert!(progress.fits(grant)),Ok(RetainedCloneSourceTake::Ready(..))=>panic!("retained projection must delay actual authority transfer"),Err(_)=>{assert_eq!(source.try_authority().unwrap().get().as_ptr(),pointer);break;}}assert!(turn<9_999);
            }
            assert_eq!(factory.calls.load(Ordering::SeqCst),0);
            for turn in 0..10_000{
                if projection.terminal_is_empty(){break;}
                let copy=projection.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:projection.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:projection.next_close_release_byte_demand().unwrap(),maximum_depth:projection.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||projection.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<9_999);
            }
            assert!(projection.terminal_is_empty());assert_eq!(factory.calls.load(Ordering::SeqCst),0);

            let authority=loop {
                let copy=source.next_take_copy_byte_demand().unwrap();
                let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};
                match source.take_authority(grant).unwrap() { RetainedCloneSourceTake::Pending(progress)=>assert!(progress.fits(grant)),RetainedCloneSourceTake::Ready(authority,progress)=>{assert!(progress.fits(grant));break authority;} }
            };
            assert!(source.terminal_is_empty());
            let copy=FactoryOwnedSourceAuthority::<String>::transfer_copy_byte_demand();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()};
            let mut authority=authority;
            for axis in law["transferRefusals"].as_array().unwrap(){
                let denied=match axis.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};
                let(result,heap)=observe_retirement_allocations(||authority.take_original(denied));
                let(_,returned)=result.err().expect("underfunded actual pair transfer refuses");authority=returned;assert_eq!(heap,(0,0));assert_eq!(authority.get().as_ptr(),pointer);assert!(std::ptr::addr_eq(Arc::as_ptr(authority.factory()),issuer));
            }
            let(text,exact,progress)=authority.take_original(grant).map_err(|(error,_)|error).unwrap();assert!(progress.fits(grant));assert!(std::ptr::addr_eq(Arc::as_ptr(&exact),issuer));
            let mut owner=crate::retirement::controlled::ControlledRetirement::new(FactoryOwnedSourceAuthority::new(text,exact)).map_err(|(error,_)|error).unwrap();
            for turn in 0..100_000 {
                if owner.terminal_is_empty(){break;}
                let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                assert!(owner.step(grant).unwrap().progress().fits(grant));assert!(turn<99_999);
            }
            assert!(owner.terminal_is_empty());
        } else {
            for turn in 0..100_000 {
                if source.terminal_is_empty(){break;}
                let copy=source.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
                let(result,heap)=observe_retirement_allocations(||source.close_step(grant));
                match result { Ok(step)=>{assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}, Err(error)=>{assert_eq!(error.message,"original fixture issuer refuses once");assert_eq!(heap,(0,0));refusals+=1;} }
                assert!(turn<99_999);
            }
            assert!(source.terminal_is_empty());
        }
        assert_eq!(factory.calls.load(Ordering::SeqCst),row["expected"]["payloadRetireCalls"].as_u64().unwrap()as usize);
        assert_eq!(refusals,row["expected"]["refusals"].as_u64().unwrap()as usize);
        assert_eq!(Arc::strong_count(&factory),1);
        eprintln!("[DEBUG] Factory original source {} exact payload/issuer identity retained, calls={} refusals={refusals}, terminal source empty",row["name"],factory.calls.load(Ordering::SeqCst));
    }
}
