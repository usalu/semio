//! 🧪️ Original send-only payloads and provider-refused values retain exact physical custody.
use super::*;
use crate::{observe_retirement_allocations,RetainedAllocationEffect,RetainedWorkPoll,RetainedWorkOutcome,OriginalRetirementPoll,driven::{poll,leaf,settle_owned,CPU},retirement::{RetireOwned,owned_retirement_birth_bytes,arc_custody::arc_bytes},FactoryPayloadRetirement,FactoryProducerReply,FactoryProducerReplyComplete,OriginalRetirementFailure,OwnedValueRetirementFactory};
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
struct RefusingFactory{refuse:AtomicBool,calls:AtomicUsize,pointer:usize}
impl FactoryPayloadRetirement for RefusingFactory{
 type CloseState=bool;
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize {std::mem::size_of::<Self::CloseState>()}
 fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
 fn close_state_preparation_work_demand(&self,_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:usize,_:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
 fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
 fn prepare_close_state(&self)->bool{false}
 fn transfer_payload(_:Self,state:&mut bool){*state=true;}
 fn close_state_demands(_: &bool,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_state_step(state:&mut bool,_:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{*state=true;let before=effect.performed_work();let _=effect.performed(1);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:effect.performed_work()-before})}
 fn close_state_work_demand(state:&bool,_:usize)->Result<u64,ValueError>{Ok(u64::from(!*state))}
 fn close_state_into<'reply>(state:&mut bool,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if *state{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}*state=true;Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
 unsafe fn close_state_terminal_native_release_safe()->bool{true}
 fn close_state_terminal_is_empty(state:&bool)->bool{*state}
}
impl ArtifactOwnedValueRetirementFactory<String> for RefusingFactory{fn retire_owned_work_demand(&self,_:&String)->Result<u64,crate::ValueError>{Ok(3)}
 fn retirement_birth_bytes(&self,_:&String)->usize{owned_retirement_birth_bytes::<String>()}
 fn retire_owned_demands(&self,value:&String)->Result<RetirementDemand,ValueError>{OwnedValueRetirementFactory::<String>::default().retire_owned_demands(value)}
 fn retire_owned<'reply>(&self,value:&mut Option<String>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{
  let boundary=effect.performed_boundary();self.calls.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1).unwrap());
  if self.pointer!=0{assert_eq!(value.as_ref().unwrap().as_ptr()as usize,self.pointer,"custom issuer must receive exact original backing");}assert!(effect.performed(1).unwrap());
  if self.refuse.swap(false,Ordering::SeqCst){return reply.complete_observed(effect,boundary,Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture issuer refuses once").into()));}
  settle_owned(value,effect,reply,boundary)
 }
 fn recover_retire_owned<'reply>(&self,_:&mut Option<String>,_:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}
/// 🪑️ Admits one resident original into its fixed owner frame inside a funded effect and reports its settled turn, physical heap and marker.
fn admitted<T:Send+'static>(original:&mut Option<T>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,g:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,ValueError>,(usize,usize),Option<usize>){
 let(poll,heap,actual)=leaf(g,CPU,|effect|FactoryOwnedRetirement::admit_original(original,factory,output,effect));
 (match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original admission stayed pending")},heap,actual)
}
/// 🔬️ Borrows the actual owner frame behind its erased ticket.
fn frame<T:Send+'static>(ticket:&Option<Box<dyn ErasedSnapshotRetirement>>)->&FactoryOwnedRetirement<T>{let any:&dyn std::any::Any=&**ticket.as_ref().expect("admitted original frame");any.downcast_ref::<FactoryOwnedRetirement<T>>().expect("the actual owner frame")}
#[test]
fn factory_owned_retirement_preserves_original_provider_refusal_and_exact_terminal_allocations(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let mut payload=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);payload.push_str(&text);let pointer=payload.as_ptr();let mut original=Some(payload);let exact:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(RefusingFactory{refuse:AtomicBool::new(true),calls:AtomicUsize::new(0),pointer:0});let mut factory=Some(exact);let strong=Arc::strong_count(factory.as_ref().unwrap());let held=original.as_ref().unwrap().capacity()+arc_bytes::<RefusingFactory>();
  let bytes=FactoryOwnedRetirement::<String>::constructor_capacity_bytes();let demand=FactoryOwnedRetirement::<String>::original_admission_demands(&original);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};let mut output=None;
  assert_eq!(FactoryOwnedRetirement::<String>::admit_original_source_work_demand(&original),law["originalAdmissionWork"]["success"].as_u64().unwrap());
  for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (turn,heap,_)=admitted(&mut original,&mut factory,&mut output,under);assert_eq!(turn.work,u128::from(law["originalAdmissionWork"]["denied"].as_u64().unwrap()));assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(output.is_none());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(Arc::strong_count(factory.as_ref().unwrap()),strong);assert_eq!(heap,(0,0));}
  let (turn,(a,r),_)=admitted(&mut original,&mut factory,&mut output,grant);assert_eq!(turn.work,u128::from(law["originalAdmissionWork"]["success"].as_u64().unwrap()));let p=turn.result.unwrap().progress();assert_eq!(FactoryOwnedRetirement::<String>::admit_original_source_work_demand(&original),law["originalAdmissionWork"]["denied"].as_u64().unwrap());assert!(original.is_none());assert!(factory.is_none());assert_eq!(frame::<String>(&output).original().unwrap().as_ptr(),pointer);assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert_eq!(a,bytes);let mut births=a;let mut released=r;let mut refusals=0;
  for turn in 0..1000000{
   let body=copy.as_u64().unwrap()as usize;let (demand,heap)=observe_retirement_allocations(||output.as_ref().unwrap().next_demand(body).unwrap());assert_eq!(heap,(0,0));let work=output.as_ref().unwrap().next_computational_work(body).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for under in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let (step,heap)=poll(under,work,|effect|crate::close_owner(output.as_mut().unwrap().as_mut(),body,effect));assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
   let (step,(a,r))=poll(grant,work,|effect|crate::close_owner(output.as_mut().unwrap().as_mut(),body,effect));match step.result{Ok(step)=>{let p=step.progress();assert!(p.fits(grant));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;},Err(error)=>{assert_eq!(error.primary().unwrap().message,"original fixture issuer refuses once");assert_eq!(frame::<String>(&output).original().unwrap().as_ptr(),pointer);assert_eq!((a,r),(0,0));refusals+=1;}}if output.as_ref().unwrap().terminal_is_empty(){break;}assert!(turn<999999);
   if turn==cut.as_u64().unwrap()as usize{let (same,heap)=observe_retirement_allocations(||output);output=same;assert_eq!(heap,(0,0));}
  }
  assert_eq!(refusals,1);assert!(output.as_ref().unwrap().terminal_is_empty());let mut erased=output;let (demand,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(erased.as_ref().unwrap(),0).unwrap());assert_eq!(heap,(0,0));assert_eq!(demand.release_bytes,bytes);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_release_bytes:bytes,maximum_depth:demand.depth,..Default::default()};let (small,heap)=poll(RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},CPU,|effect|crate::close_factory_ticket(&mut erased,0,effect));assert_eq!(small.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));let (done,(a,r))=poll(grant,CPU,|effect|crate::close_factory_ticket(&mut erased,0,effect));let done=done.result.unwrap();assert!(done.progress().fits(grant));assert_eq!((done.progress().retained_capacity_bytes,done.progress().released_bytes),(a,r));births+=a;released+=r;assert!(erased.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((erased,original)));assert_eq!(heap,(0,0));
 }}}
}
struct SendOnly{payload:String,marker:std::cell::Cell<u8>}
impl RetireOwned for SendOnly{
 type RetirementCursor=<String as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{assert_eq!(self.marker.get(),7);self.payload.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
#[test]
fn factory_owned_retirement_accepts_actual_send_only_payloads(){
 let mut original=Some(SendOnly{payload:String::with_capacity(8192),marker:std::cell::Cell::new(7)});let mut factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<SendOnly>>>=Some(Arc::new(crate::OwnedValueRetirementFactory::<SendOnly>::default()));let held=8192+arc_bytes::<crate::OwnedValueRetirementFactory<SendOnly>>();let demand=FactoryOwnedRetirement::<SendOnly>::original_admission_demands(&original);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:FactoryOwnedRetirement::<SendOnly>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()};let mut frame=None;let (result,(a,r),_)=admitted(&mut original,&mut factory,&mut frame,grant);let p=result.result.unwrap().progress();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));let mut births=a;let mut released=r;
 for turn in 0..1000000{if frame.is_none(){break;}let demand=crate::factory_ticket_demands(frame.as_ref().unwrap(),1).unwrap();let work=if frame.as_ref().unwrap().terminal_is_empty(){1}else{frame.as_ref().unwrap().next_computational_work(1).unwrap()};let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(1),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let (step,(a,r))=poll(grant,work,|effect|crate::close_factory_ticket(&mut frame,1,effect));let step=step.result.unwrap();assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));births+=a;released+=r;assert!(turn<999999);}
 assert!(frame.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((frame,original,factory)));assert_eq!(heap,(0,0));
}

/// 🔗️ A genuine projected source preserves its original custom issuer on take, cancellation and provider refusal.
#[test]
fn factory_owned_source_preserves_actual_payload_and_custom_issuer() {
    use semio_framework_value_clone_source::{RetainedCloneSource,RetainedOwnedProjection,RetainedOwnedProjectionAdmission};
    use semio_framework_value_clone_source_testing::{admit_projected,FixtureRecoveryStep};
    use crate::driven::{bank,driven_reply,driven_step,grant_of};
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔗️source/🔣️.json")).unwrap();
    let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:0,maximum_depth:8};
    let value_progress=|step:&Result<RetainedCloneStep,ValueError>|match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
    let retain=|row:semio_framework_value_clone_source::RetainedCloneRef<'_,str>|->RetainedOwnedProjection<str>{
        let mut admission=RetainedOwnedProjectionAdmission::<str>::default();let mut output=None;
        let g=grant_of(admission.demand(row).unwrap());let(turn,heap)=driven_reply(g,CPU,|step:&Result<RetainedCloneStep,ValueError>|match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(row,effect,reply)});assert!(turn.result.unwrap().progress().fits(g));assert_eq!(heap.1,0);
        let g=grant_of(admission.take_demand().unwrap());let(turn,_)=driven_reply(g,CPU,|step:&Result<RetainedCloneStep,ValueError>|match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_take(reply)}else{admission.take(&mut output,effect,reply)});assert!(turn.result.unwrap().progress().fits(g));assert!(admission.terminal_is_empty());output.expect("owned projection published")
    };
    let denied_projection=|row:semio_framework_value_clone_source::RetainedCloneRef<'_,str>,axis:&str|{
        let admission=RetainedOwnedProjectionAdmission::<str>::default();let d=admission.demand(row).unwrap();let g=grant_of(d);
        let denied=match axis{"items"=>RetainedCloneGrant{maximum_items:0,..g},"copy" if g.maximum_copy_bytes>0=>RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-1,..g},"depth"=>RetainedCloneGrant{maximum_depth:0,..g},"copy"=>return,_=>unreachable!()};
        let mut admission=admission;let(turn,heap)=driven_reply(denied,CPU,|step:&Result<RetainedCloneStep,ValueError>|match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(row,effect,reply)});
        assert_eq!(turn.work,0);assert_eq!(turn.result.map_or(RetainedCloneProgress::default(),|step|step.progress()),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert!(admission.terminal_is_empty());
    };
    for row in law["cases"].as_array().unwrap() {
        let mut text=String::with_capacity(row["capacity"].as_u64().unwrap()as usize); text.push_str(row["text"].as_str().unwrap());
        let pointer=text.as_ptr();
        let factory=Arc::new(RefusingFactory{refuse:AtomicBool::new(row["refuseOnce"].as_bool().unwrap()),calls:AtomicUsize::new(0),pointer:pointer as usize});
        let exact:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=factory.clone();
        let issuer=Arc::as_ptr(&exact);
        let mut authority=FactoryOwnedSourceAuthority::new(text,exact);
        assert_eq!(authority.get().as_ptr(),pointer);
        for axis in law["sourceRefusals"].as_array().unwrap() {
            let denied=match axis.as_str().unwrap() { "items"=>RetainedCloneGrant{maximum_items:0,..funded},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:0,..funded},"depth"=>RetainedCloneGrant{maximum_depth:0,..funded},"copy"=>RetainedCloneGrant{maximum_copy_bytes:0,..funded},_=>unreachable!() };
            let mut refused=bank();let outcome=admit_projected::<String,FactoryOwnedSourceAuthority<String>,()>(&mut refused,authority,(),denied);
            let mut recovery=outcome.result.err().expect("underfunded source admission refuses");
            for _ in 0..1000{match recovery.resume(crate::driven::grant_of(RetirementDemand{copy_bytes:65536,capacity_bytes:65536,release_bytes:0,depth:8})){FixtureRecoveryStep::Complete=>break,FixtureRecoveryStep::Progress=>{},other=>panic!("source admission handback {other:?}")}}
            let mut returned=None;assert!(matches!(recovery.finish_into(&mut returned,funded),FixtureRecoveryStep::Complete));let(_,original,_)=returned.unwrap();
            assert_eq!(original.get().as_ptr(),pointer);assert!(std::ptr::addr_eq(Arc::as_ptr(original.factory()),issuer));authority=original;
        }
        let mut admission_bank=bank();let RetainedWorkOutcome{result,work}=admit_projected::<String,FactoryOwnedSourceAuthority<String>,()>(&mut admission_bank,authority,(),funded);assert_eq!(work,u128::from(law["sourceAdmissionWork"].as_u64().unwrap()));
        let(mut source,_)=result.unwrap_or_else(|_|panic!("source admission"));drop(admission_bank);
        assert_eq!(source.borrow().get().as_ptr(),pointer);
        assert!(std::ptr::addr_eq(Arc::as_ptr(source.try_authority().unwrap().factory()),issuer));
        let mut refusals=0;
        if row["route"]=="transfer" {
            let row_ref=source.borrow().project(2,|original|original.as_str());
            for axis in law["projectionRefusals"].as_array().unwrap(){denied_projection(row_ref,axis.as_str().unwrap());}
            let mut projection=retain(row_ref);assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);
            let nested=projection.borrow().unwrap().project(3,|original|original);
            for axis in law["projectionRefusals"].as_array().unwrap(){denied_projection(nested,axis.as_str().unwrap());}
            let mut returned=None;
            for turn in 0..10_000{
                let copy=source.next_take_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};
                let(step,_)=driven_step(grant,CPU,|effect,reply,recover|if recover{source.recover_take_authority_into(&mut returned,grant.maximum_copy_bytes,effect,reply)}else{source.take_authority_into(&mut returned,grant.maximum_copy_bytes,effect,reply)});
                match step.result{Ok(step)=>{assert!(step.progress().fits(grant));assert!(returned.is_none(),"retained projection must delay actual authority transfer");if step.progress()==RetainedCloneProgress::default(){assert_eq!(source.try_authority().unwrap().get().as_ptr(),pointer);break}},Err(_)=>{assert_eq!(source.try_authority().unwrap().get().as_ptr(),pointer);break}}assert!(turn<9_999);
            }
            assert_eq!(factory.calls.load(Ordering::SeqCst),0);
            for turn in 0..10_000{
                if projection.terminal_is_empty(){break;}
                let copy=projection.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:projection.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:projection.next_close_release_byte_demand().unwrap(),maximum_depth:projection.next_close_depth_demand().unwrap()};let(step,heap)=driven_step(grant,CPU,|effect,reply,recover|if recover{projection.recover_close_into(4096,effect,reply)}else{projection.close_into(4096,effect,reply)});let step=step.result.unwrap();assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<9_999);
            }
            assert!(projection.terminal_is_empty());assert_eq!(factory.calls.load(Ordering::SeqCst),0);

            for turn in 0..10_000{
                let copy=source.next_take_copy_byte_demand().unwrap();
                let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};
                let(step,_)=driven_step(grant,CPU,|effect,reply,recover|if recover{source.recover_take_authority_into(&mut returned,grant.maximum_copy_bytes,effect,reply)}else{source.take_authority_into(&mut returned,grant.maximum_copy_bytes,effect,reply)});let step=step.result.unwrap();assert!(step.progress().fits(grant));
                if matches!(step,RetainedCloneStep::Complete(_)){break;}assert!(turn<9_999);
            }
            let mut authority=returned.take().expect("source handed its original authority back");
            assert!(source.terminal_is_empty());
            let grant=grant_of(FactoryOwnedSourceAuthority::<String>::transfer_original_demand());let(mut moved,mut issued)=(None,None);
            for axis in law["transferRefusals"].as_array().unwrap(){
                let denied=match axis.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};
                let(turn,heap)=driven_reply(denied,CPU,value_progress,|effect,reply,_|authority.transfer_original_into(&mut moved,&mut issued,effect,reply));
                assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert!(moved.is_none()&&issued.is_none());assert_eq!(authority.get().as_ptr(),pointer);assert!(std::ptr::addr_eq(Arc::as_ptr(authority.factory()),issuer));
            }
            let(turn,_)=driven_reply(grant,CPU,value_progress,|effect,reply,_|authority.transfer_original_into(&mut moved,&mut issued,effect,reply));assert!(turn.result.unwrap().progress().fits(grant));
            let(text,exact)=(moved.take().expect("actual original moved"),issued.take().expect("exact issuer moved"));assert!(std::ptr::addr_eq(Arc::as_ptr(&exact),issuer));
            let mut owner=crate::retirement::controlled::ControlledRetirement::new(FactoryOwnedSourceAuthority::new(text,exact)).map_err(|(error,_)|error).unwrap();
            for turn in 0..100_000 {
                if owner.terminal_is_empty(){break;}
                let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                let(step,_)=driven_step(grant,CPU,|effect,reply,recover|if recover{owner.recover_step_into(4096,effect,reply)}else{owner.step_into(4096,effect,reply)});assert!(step.result.unwrap().progress().fits(grant));assert!(turn<99_999);
            }
            assert!(owner.terminal_is_empty());assert!(authority.terminal_is_empty());
        } else {
            for turn in 0..100_000 {
                if source.terminal_is_empty(){break;}
                let copy=source.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
                let(result,heap)=driven_step(grant,CPU,|effect,reply,recover|if recover{source.recover_close_into(4096,effect,reply)}else{source.close_into(4096,effect,reply)});
                match result.result { Ok(step)=>{assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}, Err(error)=>{assert_eq!(error.primary().unwrap().message,"original fixture issuer refuses once");assert_eq!(heap,(0,0));refusals+=1;} }
                assert!(turn<99_999);
            }
            assert!(source.terminal_is_empty());
        }
        assert_eq!(factory.calls.load(Ordering::SeqCst),row["expected"]["payloadRetireCalls"].as_u64().unwrap()as usize);
        assert_eq!(refusals,row["expected"]["refusals"].as_u64().unwrap()as usize);
        assert_eq!(Arc::strong_count(&factory),1);
    }
}
