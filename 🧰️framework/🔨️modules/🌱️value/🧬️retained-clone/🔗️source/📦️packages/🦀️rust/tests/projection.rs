//! 🧪️ The original sealed source and projection reply remain resident through the paid action.
use semio_framework_value_clone_source::{RetainedCloneSourceAdmission,OriginalProjectionFrame,RetainedCloneStep,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetainedAllocationEffect,ValueError};
use semio_framework_value_receiving::span::effect::OriginalEffectAction;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

fn turn(bank:&mut EffectBank,demand:RetirementDemand,work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>)->RetainedCloneStep{
 let grant=bank.loan(1,demand.copy_bytes);
 let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,work,|effect|RetainedWorkPoll::Complete(operation(effect))));
 let RetainedWorkPoll::Complete(outcome)=poll else{panic!("original leaf unexpectedly pending")};
 let step=outcome.result.expect("original source turn");
 if let Some(actual)=actual{assert_eq!(actual,heap.0)}
 assert_eq!(bank.take_admitted(),Some(true));
 bank.post(step.progress(),grant,heap);
 step
}

fn publish_original_source<T:Sync+'static,A:semio_framework_value_retirement::RetireOwned+Sync,B:semio_framework_value_retirement::RetireOwned>(bank:&mut EffectBank,admission:&mut semio_framework_value_clone_source::RetainedCloneSourceAdmission<T,A,B>,output:&mut Option<semio_framework_value_clone_source::RetainedCloneSource<T,A>>,maximum:u64){for _ in 0..maximum{if admission.terminal_is_empty(){return}let demand=admission.take_output_demand();assert!(demand.copy_bytes<=4096);let work=admission.take_output_work_demand();turn(bank,demand,work,|effect|admission.take_output(output,effect));}assert!(admission.terminal_is_empty(),"original source publication exceeded its preserved turn bound");}
fn projection_turn<T:Sync+'static,A:semio_framework_value_retirement::RetireOwned+Sync+semio_framework_value_clone_source::OriginalSourceProjection<T>>(bank:&mut EffectBank,selected:&mut OriginalProjectionFrame<T,A>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,ValueError>>,grant:semio_framework_value_clone_source::RetainedCloneGrant,work:u64)->RetainedCloneStep{let mut observed=(0usize,0usize);let mut reply=None;for _ in 0..1024{let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,work,|effect|{let before=effect.performed_work();let mut context=selected.context().unwrap();let ready=selected.authority().unwrap().drive(&mut context,effect,action).unwrap();let actual=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:actual})}else{RetainedWorkPoll::Pending{work:actual}}}));observed.0+=heap.0;observed.1+=heap.1;if let Some(actual)=actual{assert_eq!(actual,observed.0)}if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());selected.authority().unwrap().finish(action).unwrap();let step=reply.take().unwrap().unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,observed);return step}}panic!("original projection action remained pending")}

type CloseFailure=semio_framework_value_retirement::OriginalRetirementFailure;
#[derive(Clone,Copy)]struct SourceClose;
impl semio_framework_value_control_contract::OriginalEffectProducer for SourceClose{type Output=RetainedCloneStep;type Failure=CloseFailure;type Context<'a,'b>=&'a mut semio_framework_value_clone_source::RetainedCloneSource<u64,u64> where 'b:'a;fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'b:'a{None}fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.close_into(4096,effect,reply)}fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.recover_close_into(4096,effect,reply)}}
#[derive(Clone,Copy)]struct ProjectionClose;
impl semio_framework_value_control_contract::OriginalEffectProducer for ProjectionClose{type Output=RetainedCloneStep;type Failure=CloseFailure;type Context<'a,'b>=&'a mut semio_framework_value_clone_source::RetainedOwnedProjection<u64> where 'b:'a;fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'b:'a{None}fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.close_into(4096,effect,reply)}fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.recover_close_into(4096,effect,reply)}}
fn close_event<'a,'b,P>(bank:&mut EffectBank,context:&mut P::Context<'a,'b>,authority:&mut semio_framework_value_control_contract::OriginalEffectAuthority<P>,grant:semio_framework_value_clone_source::RetainedCloneGrant,units:u64)->RetainedCloneStep where 'b:'a,P:semio_framework_value_control_contract::OriginalEffectProducer<Output=RetainedCloneStep,Failure=CloseFailure>{let mut action=None;let mut reply=None;let mut observed=(0usize,0usize);for _ in 0..1024{let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let ready=authority.drive(context,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work})}else{RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;if let Some(actual)=actual{assert_eq!(actual,observed.0)}if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.take().unwrap().unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,observed);return step}}panic!("original borrowed close remained pending")}
fn finish_close_authority<'a,'b,P>(bank:&mut EffectBank,context:&P::Context<'a,'b>,authority:&mut Option<semio_framework_value_control_contract::OriginalEffectAuthority<P>>) where 'b:'a,P:semio_framework_value_control_contract::OriginalEffectProducer<Output=RetainedCloneStep,Failure=CloseFailure>+Copy{let demand=semio_framework_value_control_contract::OriginalEffectAuthority::<P>::producer_transfer_demand().unwrap();let mut producer=None;turn(bank,demand,1,|effect|{let result=semio_framework_value_control_contract::OriginalEffectAuthority::transfer_producer(authority,context,&mut producer,effect);RetainedWorkOutcome{result:result.result.map(|p|match p{Some(p)=>RetainedCloneStep::Complete(p),None=>RetainedCloneStep::Progress(Default::default())}),work:result.work}});let demand=RetirementDemand{copy_bytes:std::mem::size_of::<Option<P>>(),depth:1,..Default::default()};turn(bank,demand,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());producer.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});}

fn projected()->(EffectBank,RetainedCloneSourceAdmission<u64,u64>,serde_json::Value){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));
 let mut admission=RetainedCloneSourceAdmission::<u64,u64>::new(73,());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){
  if admission.phase()=="projectionStart"{break}
  let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();
  turn(&mut bank,demand,work,|effect|admission.advance(effect));
 }
 assert_eq!(admission.phase(),"projectionStart");
 let original=admission.original_authority().unwrap() as *const u64;
 let mut frame:Option<OriginalProjectionFrame<u64,u64>>=None;
 let demand=admission.begin_projection_demand();
 turn(&mut bank,demand,1,|effect|admission.begin_projection(&mut frame,effect));
 let selected=frame.as_mut().expect("resident projection frame");
 let demand=selected.projection_demand();let work=selected.projection_work_demand().unwrap();
 let mut action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>=None;
 let grant=bank.loan(1,demand.copy_bytes);projection_turn(&mut bank,selected,&mut action,grant,work);
 let demand=selected.producer_transfer_demand().unwrap();
 turn(&mut bank,demand,1,|effect|{let outcome=selected.transfer_producer(effect);RetainedWorkOutcome{result:outcome.result.map(|progress|match progress{Some(progress)=>RetainedCloneStep::Complete(progress),None=>RetainedCloneStep::Progress(Default::default())}),work:outcome.work}});
 let demand=admission.projection_answer_demand(selected).unwrap();
 turn(&mut bank,demand,2,|effect|admission.accept_projection(selected,effect));
 let demand=admission.finish_projection_demand();turn(&mut bank,demand,1,|effect|admission.finish_projection(&mut frame,effect));
 assert_eq!(admission.original_authority().unwrap() as *const u64,original);
 (bank,admission,fixture)
}

#[test]
fn original_projection_owner_survives_settlement_and_cancellation(){
 let (mut bank,mut admission,fixture)=projected();
 let demand=admission.discard_projection_demand();turn(&mut bank,demand,1,|effect|admission.discard_projection(effect));
 let mut returned=None;
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){
  if admission.terminal_is_empty(){break}
  let demand=admission.next_return_demand().unwrap();let work=admission.return_work_demand().unwrap();
  turn(&mut bank,demand,work,|effect|admission.return_inputs(&mut returned,effect));
 }
 assert!(admission.terminal_is_empty());assert_eq!(returned,Some((73,())));
}

/// 🧪️ Every unpublished lease phase returns the same original owner under the real wallet.
#[test]
fn unpublished_lease_phases_recover_the_original_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();
 for row in law["cancelPhases"].as_array().unwrap(){let target=row.as_str().unwrap();
  let (mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()==target{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
  assert_eq!(admission.phase(),target);let demand=admission.discard_projection_demand();turn(&mut bank,demand,1,|effect|admission.discard_projection(effect));
  let mut returned=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.terminal_is_empty(){break}let demand=admission.next_return_demand().unwrap();let work=admission.return_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.return_inputs(&mut returned,effect));}
  assert!(admission.terminal_is_empty());assert_eq!(returned,Some((73,())));
 }
}

/// 🧪️ A producer-recorded refusal survives its real callback unwind without replay.
#[test]
fn original_projection_refusal_survives_recorded_unwind(){
 use semio_framework_value_clone_source::{OriginalProjectionReply,OriginalSourceProjection,ValueRefusalKind};
 use semio_framework_value_control_contract::OriginalEffectReplyComplete;
 use semio_framework_value_retirement::RetireOwned;
 use std::sync::atomic::{AtomicUsize,Ordering};
 static CALLS:AtomicUsize=AtomicUsize::new(0);
 static RECOVERS:AtomicUsize=AtomicUsize::new(0);
 struct Probe(u64);
 impl RetireOwned for Probe{
  type RetirementCursor=<u64 as RetireOwned>::RetirementCursor;
  fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}
  fn controlled_retirement_supported()->bool{true}
 }
 impl OriginalSourceProjection<u64> for Probe{
  fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
  fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{
   CALLS.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1)?);
   let recorded=reply.complete(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original projected refusal")));drop(recorded);panic!("producer interrupted after recording the original refusal")
  }
  fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{RECOVERS.fetch_add(1,Ordering::SeqCst);Ok(reply.pending())}
 }
 CALLS.store(0,Ordering::SeqCst);RECOVERS.store(0,Ordering::SeqCst);
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut admission=RetainedCloneSourceAdmission::<u64,Probe>::new(Probe(73),());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="projectionStart"{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
 assert_eq!(admission.phase(),"projectionStart");let original=admission.original_authority().unwrap() as *const Probe;
 let mut frame=None;let demand=admission.begin_projection_demand();turn(&mut bank,demand,1,|effect|admission.begin_projection(&mut frame,effect));
 let selected=frame.as_mut().unwrap();let demand=selected.projection_demand();let work=selected.projection_work_demand().unwrap();let grant=bank.loan(1,demand.copy_bytes);
 let mut action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>=None;
 let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||bank.run(grant,work,|effect|{
  let mut context=selected.context().unwrap();let _=selected.authority().unwrap().drive(&mut context,effect,&mut action);
  RetainedWorkPoll::<RetainedCloneStep,ValueError>::Pending{work:0}
 })));
 assert!(interrupted.is_err());assert_eq!(CALLS.load(Ordering::SeqCst),1);assert_eq!(RECOVERS.load(Ordering::SeqCst),0);
 projection_turn(&mut bank,selected,&mut action,grant,work);
 assert_eq!(CALLS.load(Ordering::SeqCst),1);assert_eq!(RECOVERS.load(Ordering::SeqCst),0);
 let d=selected.producer_transfer_demand().unwrap();turn(&mut bank,d,1,|effect|{let result=selected.transfer_producer(effect);RetainedWorkOutcome{result:result.result.map(|progress|match progress{Some(progress)=>RetainedCloneStep::Complete(progress),None=>RetainedCloneStep::Progress(Default::default())}),work:result.work}});
 let d=admission.projection_answer_demand(selected).unwrap();turn(&mut bank,d,2,|effect|admission.accept_projection(selected,effect));let d=admission.finish_projection_demand();turn(&mut bank,d,1,|effect|admission.finish_projection(&mut frame,effect));
 assert_eq!(admission.original_authority().unwrap() as *const Probe,original);assert_eq!(admission.phase(),"projectionFailure");let(pointer,heap)=observe_retirement_allocations(||admission.projection_failure().unwrap().message.as_ptr());assert_eq!(heap,(0,0));let mut error=None;let d=admission.take_projection_failure_demand();turn(&mut bank,d,1,|effect|admission.take_projection_failure(&mut error,effect));
 let error=error.unwrap();assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.message.as_ptr(),pointer);assert_eq!(error.message.as_ptr(),"original projected refusal".as_ptr());
 let mut returned=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.terminal_is_empty(){break}let d=admission.next_return_demand().unwrap();let work=admission.return_work_demand().unwrap();turn(&mut bank,d,work,|effect|admission.return_inputs(&mut returned,effect));}assert!(admission.terminal_is_empty());assert_eq!(returned.unwrap().0.0,73);
}

/// 🧷️ Captured original aliases outlive source closure and publish without replaying a getter.
#[test]
fn owned_projection_admission_retains_the_root_until_paid_output_publication(){
 use semio_framework_value_clone_source::{RetainedOwnedProjectionAdmission,RetainedOwnedProjection};
 use semio_framework_value_control_contract::OriginalEffectReply;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();
 let (mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
 assert_eq!(admission.phase(),"ready");let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());assert!(admission.terminal_is_empty());let mut source=source.unwrap();let pointer=source.borrow().get() as *const u64;let mut capture=RetainedOwnedProjectionAdmission::<u64>::default();let mut output:Option<RetainedOwnedProjection<u64>>=None;let mut reply=None;
 for denied in 0..2{let demand=capture.demand(source.borrow()).unwrap();let work=capture.work_demand(source.borrow()).unwrap();let mut grant=bank.loan(1,demand.copy_bytes);let maximum=if denied==0{0}else{work};if denied==1{grant.maximum_copy_bytes=0}let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,maximum,|effect|{let before=effect.performed_work();drop(capture.admit(source.borrow(),effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("native denial cannot become pending")};assert_eq!(outcome.work,u128::from(law["ownedAlias"]["deniedWork"].as_u64().unwrap()));let step=outcome.result.unwrap();assert_eq!(actual,None);assert_eq!(heap,(0,0));assert_eq!(bank.take_admitted(),None);bank.post(step.progress(),grant,heap);assert!(capture.terminal_is_empty());assert!(capture.borrow().is_err());assert!(output.is_none());assert_eq!(source.borrow().get() as *const u64,pointer);}
 let demand=capture.demand(source.borrow()).unwrap();let work=capture.work_demand(source.borrow()).unwrap();turn(&mut bank,demand,work,|effect|{let before=effect.performed_work();drop(capture.admit(source.borrow(),effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before}});assert!(!capture.terminal_is_empty());assert!(output.is_none());assert_eq!(capture.borrow().unwrap().get() as *const u64,pointer);
 let mut source_close=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let demand=source.next_close_demand(4096).unwrap();let work=source.next_close_work_demand(4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);close_event(&mut bank,&mut &mut source,source_close.as_mut().unwrap(),grant,work);}
 assert!(source.terminal_is_empty());finish_close_authority(&mut bank,&&mut source,&mut source_close);drop(source);assert!(law["ownedAlias"]["sourceMayCloseBeforePublish"].as_bool().unwrap());assert_eq!(*capture.borrow().unwrap().get(),law["ownedAlias"]["value"].as_u64().unwrap());assert_eq!(capture.borrow().unwrap().get() as *const u64,pointer);
 let demand=capture.take_demand().unwrap();let work=capture.take_work_demand().unwrap();assert_eq!(work,law["ownedAlias"]["publishWork"].as_u64().unwrap());turn(&mut bank,demand,work,|effect|{let before=effect.performed_work();drop(capture.take(&mut output,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before}});assert!(capture.terminal_is_empty());let projection=output.as_mut().unwrap();assert_eq!(projection.borrow().unwrap().get() as *const u64,pointer);assert_eq!(*projection.borrow().unwrap().get(),law["ownedAlias"]["value"].as_u64().unwrap());
 let mut projection_close=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(ProjectionClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if projection.terminal_is_empty(){break}let demand=projection.close_demand(4096).unwrap();let work=projection.close_work_demand(4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);close_event(&mut bank,&mut &mut *projection,projection_close.as_mut().unwrap(),grant,work);}
 assert!(projection.terminal_is_empty());finish_close_authority(&mut bank,&&mut *projection,&mut projection_close);let demand=RetirementDemand{copy_bytes:std::mem::size_of::<Option<RetainedOwnedProjection<u64>>>(),depth:1,..Default::default()};turn(&mut bank,demand,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});assert!(output.is_none());
}

#[test]
fn borrowed_close_recovers_recorded_terminal_child_refusal_without_replay(){
 use semio_framework_value_control_contract as core;
 use semio_framework_value_retirement::{self as retirement,RetireOwned};
 use retirement::retirement::{RetirementCursor,RetirementStep};
 use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
 struct Original{error:Option<core::ValueError>,calls:Arc<AtomicUsize>,recoveries:Arc<AtomicUsize>}
 struct Cursor(Original);
 impl RetireOwned for Original{
  type RetirementCursor=retirement::retirement::RetirementFieldsPair<Cursor,retirement::retirement::Leaf<u64>>;
  fn retirement_seed(self)->Self::RetirementCursor{retirement::retirement::RetirementFieldsPair::new(Cursor(self),retirement::retirement::leaf(73))}
  fn controlled_retirement_supported()->bool{true}
 }
 impl RetirementCursor for Cursor{
  fn next_computational_work(&self,_:usize)->Result<u64,core::ValueError>{Ok(u64::from(self.0.error.is_some()))}
  fn next_demand(&self,_:usize)->Result<core::RetirementDemand,core::ValueError>{Ok(core::RetirementDemand{copy_bytes:usize::from(self.0.error.is_some())*std::mem::size_of::<Option<core::ValueError>>(),depth:usize::from(self.0.error.is_some()),..Default::default()})}
  fn close_into<'reply>(&mut self,body:usize,effect:&mut core::RetainedAllocationEffect<'_>,reply:core::OriginalEffectReply<'reply,RetirementStep,retirement::OriginalRetirementFailure>)->Result<core::OriginalEffectReplyComplete<'reply,RetirementStep,retirement::OriginalRetirementFailure>,core::ValueError>{
   assert!(effect.invocation().is_some());self.0.calls.fetch_add(1,Ordering::Relaxed);assert!(effect.performed(1)?);let error=self.0.error.take().expect("actual original refusal");let _recorded=reply.complete(Err(error.into()));panic!("terminal child after its recorded refusal")
  }
  fn recover_close_into<'reply>(&mut self,_body:usize,_:&mut core::RetainedAllocationEffect<'_>,reply:core::OriginalEffectReply<'reply,RetirementStep,retirement::OriginalRetirementFailure>)->Result<core::OriginalEffectReplyComplete<'reply,RetirementStep,retirement::OriginalRetirementFailure>,core::ValueError>{self.0.recoveries.fetch_add(1,Ordering::Relaxed);Ok(reply.pending())}
  fn terminal_is_empty(&self)->bool{self.0.error.is_none()}
  fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
  fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
  fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
 }
 struct Work{stock:u128}
 impl core::OriginalWorkRecipient for Work{
  type Context<'a,'b>=(&'a core::OriginalSourceTurnIssuer,&'a core::OriginalSourceTurn) where 'b:'a;
  fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),core::ValueError> where 'b:'a{c.0.validate(c.1).map_err(Into::into)}
  fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<core::OriginalSourceTurnIdentity> where 'b:'a{Some(c.1.identity())}
  fn admits<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkQuote<'r>,_:u64,n:u64)->core::OriginalWorkQuoteComplete<'r> where 'b:'a{q.complete(Ok(u128::from(n)<=self.stock))}
  fn settle_partial<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkPartialCheck<'r>)->core::OriginalWorkPartialCheckComplete<'r> where 'b:'a{self.stock=self.stock.checked_sub(q.delta()).expect("actual finite close stock");q.complete(Ok(()))}
  fn recover_partial<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkPartialCheck<'r>)->core::OriginalWorkPartialCheckComplete<'r> where 'b:'a{q.pending()}
 }
 struct Producer{owner:retirement::retirement::controlled::ControlledRetirement<Original>,source:core::OriginalSourceTurnIdentity}
 impl core::OriginalEffectProducer for Producer{
  type Output=core::RetainedCloneStep;type Failure=retirement::OriginalRetirementFailure;type Context<'a,'b>=(&'a core::OriginalSourceTurnIssuer,&'a core::OriginalSourceTurn) where 'b:'a;
  fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),core::ValueError> where 'b:'a{c.0.validate(c.1).map_err(core::ValueError::from)?;if c.1.identity()!=self.source{return Err(core::ValueError::invariant("foreign original close Source"))}Ok(())}
  fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<core::OriginalSourceTurnIdentity> where 'b:'a{Some(c.1.identity())}
  fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut core::RetainedAllocationEffect<'_>,reply:core::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<core::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,core::ValueError> where 'b:'a{self.owner.step_into(4096,effect,reply)}
  fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut core::RetainedAllocationEffect<'_>,reply:core::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<core::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,core::ValueError> where 'b:'a{self.owner.recover_step_into(4096,effect,reply)}
 }
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎟️admission/projection.json")).unwrap();let law=&fixture["borrowedClose"];let issuer=core::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let calls=Arc::new(AtomicUsize::new(0));let recoveries=Arc::new(AtomicUsize::new(0));let error=core::ValueError::new(core::ValueRefusalKind::Canceled,law["refusal"].as_str().unwrap().to_owned());let pointer=error.message.as_ptr();let mut producer=core::OriginalEffectAuthority::new(Producer{owner:retirement::retirement::controlled::ControlledRetirement::retain(Original{error:Some(error),calls:calls.clone(),recoveries:recoveries.clone()}),source:turn.identity()});let fuel=law["fuel"].as_u64().unwrap();let mut payer=core::OriginalWorkAuthority::new(Work{stock:u128::from(fuel)});let mut interrupted=false;let mut returned=false;let mut used=0u128;
 payer.with_loan(&mut (&issuer,&turn),|loan|{
  let mut loan=loan.reborrow();let mut work=core::OriginalWork::empty();work.recipient=Some(&mut loan);
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){
   if producer.inspect(|p|p.owner.terminal_is_empty()){break}
   let demand=producer.inspect(|p|p.owner.next_demand(65536).unwrap());let maximum=producer.inspect(|p|p.owner.next_computational_work(65536).unwrap());let grant=core::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let(mut action,mut receipt,mut marker,mut invocation,mut proof)=(None,None,None,None,None);work.begin_turn();assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut total=0u128;let mut output=None;
   loop{
    work.issue_performed_for_action(action,&mut receipt).unwrap();let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut effect=core::RetainedAllocationEffect::new(grant,maximum,&mut marker,receipt.as_mut().unwrap());producer.drive(&mut (&issuer,&turn),&mut effect,&mut invocation)}));
    work.claim_performed_for_action(action,&mut receipt).unwrap();if work.partial_pending_for_action(action).unwrap(){let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert!(partial.source_refusal().is_none());total+=partial.delta();used+=partial.delta()}
    match result{Err(_)=>{assert!(!interrupted);interrupted=true;assert_eq!(marker,None);assert_eq!(calls.load(Ordering::Relaxed),1);assert!(producer.finish(&mut invocation).is_err())},Ok(result)=>{assert!(result.unwrap());assert!(invocation.as_mut().unwrap().transfer_reply(&mut output).unwrap());break}}
   }
   assert!(work.commit_for_action(total,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap();producer.finish(&mut invocation).unwrap();match output.take().unwrap(){Err(error)=>{assert!(!returned);assert_eq!(error.primary().unwrap().message.as_ptr(),pointer);returned=true;assert!(interrupted)},Ok(_)=>{}}
  }
 }).unwrap();
 assert!(producer.inspect(|p|p.owner.terminal_is_empty()));assert!(producer.into_producer().is_ok());assert!(interrupted&&returned);assert_eq!(calls.load(Ordering::Relaxed),law["receiveCalls"].as_u64().unwrap()as usize);assert_eq!(recoveries.load(Ordering::Relaxed),law["recoverCalls"].as_u64().unwrap()as usize);assert_eq!(payer.inspect(|p|p.stock)+used,u128::from(fuel));assert!(!payer.has_live_actions())
}

/// 🧪️ Cancellation keeps its original input resident until the separately paid caller handoff.
#[test]
fn staged_source_cancellation_returns_the_original_input_without_temporary_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();
 for cut in law["stagedCancelCuts"].as_array().unwrap(){let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut staged=semio_framework_value_clone_source::StagedSourceAdmission::<u64,u64>::owned(73);let mut source=None;
  for _ in 0..cut.as_u64().unwrap(){let d=staged.next_demand().unwrap();let work=staged.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.advance(&mut source,effect));}
  let mut input=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if staged.terminal_is_empty(){break}let d=staged.close_demand().unwrap();let work=staged.close_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.close_into(&mut input,&mut source,0,effect));}
  assert!(staged.terminal_is_empty());assert_eq!(input,Some(73));assert!(source.is_none());
 }
}

/// 🧷️ Cancelling an idle captured producer returns its actual owner without invoking projection.
#[test]
fn staged_projection_cancellation_returns_the_uninvoked_original_producer(){
 use semio_framework_value_clone_source::{StagedSourceAdmission,StagedProjectionSelection};
 use semio_framework_value_clone_source::{OriginalProjectionReply,OriginalSourceProjection};
 use semio_framework_value_control_contract::OriginalEffectReplyComplete;
 use semio_framework_value_retirement::RetireOwned;
 use std::sync::atomic::{AtomicUsize,Ordering};
 static CALLS:AtomicUsize=AtomicUsize::new(0);
 struct Probe(u64);
 impl RetireOwned for Probe{type RetirementCursor=<u64 as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}fn controlled_retirement_supported()->bool{true}}
 impl OriginalSourceProjection<u64> for Probe{
  fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
  fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{CALLS.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1)?);Ok(reply.complete(Ok(&self.0)))}
  fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{Ok(reply.pending())}
 }
 CALLS.store(0,Ordering::SeqCst);

 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();let original=law["stagedProjection"]["input"].as_u64().unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut staged=StagedSourceAdmission::<u64,Probe>::borrowed(Probe(original));let(mut source,mut frame)=(None,None);
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if matches!(staged.projection_selection(&frame).unwrap(),Some(StagedProjectionSelection::Begin)){break}let d=staged.next_demand().unwrap();let work=staged.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.advance(&mut source,effect));}
 assert!(matches!(staged.projection_selection(&frame).unwrap(),Some(StagedProjectionSelection::Begin)));
 let d=staged.projection_demand(&frame,false).unwrap();let work=staged.projection_work_demand(&frame,false).unwrap();turn(&mut bank,d,work,|effect|match staged.projection_step(&mut frame,false,effect){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("pure producer capture cannot be pending")});
 assert!(matches!(staged.projection_selection(&frame).unwrap(),Some(StagedProjectionSelection::Drive)));
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if frame.is_none(){break}let d=staged.projection_demand(&frame,true).unwrap();let work=staged.projection_work_demand(&frame,true).unwrap();turn(&mut bank,d,work,|effect|match staged.projection_step(&mut frame,true,effect){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("idle original producer has no pending action")});}
 assert!(frame.is_none());assert!(staged.projection_result().is_none());let mut input=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if staged.terminal_is_empty(){break}let d=staged.close_demand().unwrap();let work=staged.close_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.close_into(&mut input,&mut source,0,effect));}
 assert!(staged.terminal_is_empty());assert_eq!(input.unwrap().0,original);assert!(source.is_none());assert_eq!(CALLS.load(Ordering::SeqCst),law["stagedProjection"]["idleCancelCalls"].as_u64().unwrap()as usize);
}

/// 🧭️ The staged wrapper transfers its real recorded projection only after the original action settles.
#[test]
fn staged_projection_success_publishes_after_original_action_settles(){
 use semio_framework_value_clone_source::{StagedSourceAdmission,StagedProjectionSelection};
 use semio_framework_value_clone_source::{OriginalProjectionReply,OriginalSourceProjection};
 use semio_framework_value_control_contract::OriginalEffectReplyComplete;
 use semio_framework_value_retirement::RetireOwned;
 use std::sync::atomic::{AtomicUsize,Ordering};
 static CALLS:AtomicUsize=AtomicUsize::new(0);
 struct Probe(u64);
 impl RetireOwned for Probe{type RetirementCursor=<u64 as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}fn controlled_retirement_supported()->bool{true}}
 impl OriginalSourceProjection<u64> for Probe{
  fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
  fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{CALLS.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1)?);Ok(reply.complete(Ok(&self.0)))}
  fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,u64>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<u64>,ValueError>,ValueError>{Ok(reply.pending())}
 }
 CALLS.store(0,Ordering::SeqCst);

 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();let original=law["stagedProjection"]["input"].as_u64().unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut staged=StagedSourceAdmission::<u64,Probe>::borrowed(Probe(original));let(mut source,mut frame)=(None,None);
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if matches!(staged.projection_selection(&frame).unwrap(),Some(StagedProjectionSelection::Begin)){break}let d=staged.next_demand().unwrap();let work=staged.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.advance(&mut source,effect));}
 let d=staged.projection_demand(&frame,false).unwrap();let work=staged.projection_work_demand(&frame,false).unwrap();turn(&mut bank,d,work,|effect|match staged.projection_step(&mut frame,false,effect){RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("capture remained pending")});
 assert!(staged.projection_result().is_none());let selected=staged.projection_frame(&mut frame).unwrap();let demand=selected.projection_demand();let work=selected.projection_work_demand().unwrap();let mut action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>=None;let grant=bank.loan(1,demand.copy_bytes);projection_turn(&mut bank,selected,&mut action,grant,work);assert!(staged.projection_result().is_none());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if frame.is_none(){break}let d=staged.projection_demand(&frame,false).unwrap();let work=staged.projection_work_demand(&frame,false).unwrap();turn(&mut bank,d,work,|effect|match staged.projection_step(&mut frame,false,effect){RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("settled projection remained pending")});}
 assert!(frame.is_none());assert_eq!(staged.projection_result(),Some(true));let d=staged.projection_result_demand().unwrap();let work=staged.projection_result_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.discard_projection(effect));let mut input=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if staged.terminal_is_empty(){break}let d=staged.close_demand().unwrap();let work=staged.close_work_demand().unwrap();turn(&mut bank,d,work,|effect|staged.close_into(&mut input,&mut source,0,effect));}assert!(staged.terminal_is_empty());assert_eq!(input.unwrap().0,original);assert!(source.is_none());assert_eq!(CALLS.load(Ordering::SeqCst),law["stagedProjection"]["successCalls"].as_u64().unwrap()as usize);
}


/// 🪪️ An unbound original reference reports its real check work to the same finite Source bank.
#[test]
fn original_unbound_binding_check_uses_its_actual_effect(){
 use semio_framework_value_clone_source::{RetainedCloneBinding,RetainedCloneRef};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();let (mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,d,work,|effect|admission.advance(effect));}
 let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());assert!(admission.terminal_is_empty());let mut source=source.unwrap();let binding:Option<RetainedCloneBinding>=None;assert_eq!(RetainedCloneRef::<u64>::check_binding_work(&binding),law["bindingCheck"]["unboundWork"].as_u64().unwrap());
 for (maximum,expected) in [(0,None),(1,Some(false))]{let grant=bank.loan(1,0);let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,maximum,|effect|RetainedWorkPoll::Complete(source.borrow().check_binding_paid(&binding,effect))));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("pure binding check cannot be pending")};assert_eq!(outcome.work,u128::from(if maximum==0{law["bindingCheck"]["deniedWork"].as_u64().unwrap()}else{law["bindingCheck"]["unboundWork"].as_u64().unwrap()}));assert_eq!(outcome.result.unwrap(),expected);assert!(actual.is_none());assert_eq!(heap,(0,0));assert_eq!(bank.take_admitted(),if maximum==0{None}else{Some(true)});bank.post(Default::default(),grant,heap);}

 let mut binding=None;let d=source.borrow().binding_demand(&binding).unwrap().unwrap();let units=source.borrow().binding_work_demand(&binding).unwrap();turn(&mut bank,d,units,|effect|{let t=source.borrow().bind(&mut binding,effect);RetainedWorkOutcome{result:t.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:t.work}});assert!(binding.is_some());assert_eq!(RetainedCloneRef::<u64>::check_binding_work(&binding),law["bindingCheck"]["boundWork"].as_u64().unwrap());
 for (maximum,expected) in [(4,None),(5,Some(true))]{let grant=bank.loan(1,0);let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,maximum,|effect|RetainedWorkPoll::Complete(source.borrow().check_binding_paid(&binding,effect))));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("pure bound check cannot be pending")};assert_eq!(outcome.work,if maximum==4{0}else{u128::from(law["bindingCheck"]["boundWork"].as_u64().unwrap())});assert_eq!(outcome.result.unwrap(),expected);assert!(actual.is_none());assert_eq!(heap,(0,0));assert_eq!(bank.take_admitted(),if maximum==4{None}else{Some(true)});bank.post(Default::default(),grant,heap);}

 let (mut foreign_bank,mut foreign_admission,foreign_fixture)=projected();for _ in 0..foreign_fixture["maximumTurns"].as_u64().unwrap(){if foreign_admission.phase()=="ready"{break}let d=foreign_admission.next_demand().unwrap();let units=foreign_admission.advance_work_demand().unwrap();turn(&mut foreign_bank,d,units,|effect|foreign_admission.advance(effect));}
 let mut foreign=None;publish_original_source(&mut foreign_bank,&mut foreign_admission,&mut foreign,foreign_fixture["maximumTurns"].as_u64().unwrap());assert!(foreign_admission.terminal_is_empty());let mut foreign=foreign.unwrap();
 let grant=bank.loan(1,0);let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,5,|effect|RetainedWorkPoll::Complete(foreign.borrow().check_binding_paid(&binding,effect))));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("pure foreign check cannot be pending")};assert_eq!(outcome.work,u128::from(law["bindingCheck"]["foreignWork"].as_u64().unwrap()));assert_eq!(outcome.result.unwrap_err().kind,semio_framework_value_clone_source::ValueRefusalKind::InvariantViolated);assert!(actual.is_none());assert_eq!(heap,(0,0));assert_eq!(bank.take_admitted(),Some(true));bank.post(Default::default(),grant,heap);
 let mut foreign_close=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));for _ in 0..foreign_fixture["maximumTurns"].as_u64().unwrap(){if foreign.terminal_is_empty(){break}let d=foreign.next_close_demand(4096).unwrap();let units=foreign.next_close_work_demand(4096).unwrap();let grant=foreign_bank.loan(1,d.copy_bytes);close_event(&mut foreign_bank,&mut &mut foreign,foreign_close.as_mut().unwrap(),grant,units);}assert!(foreign.terminal_is_empty());finish_close_authority(&mut foreign_bank,&&mut foreign,&mut foreign_close);
 {
  use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity};
  use semio_framework_value_retirement::OriginalRetirementFailure;
  #[derive(Clone,Copy)] struct Close;
  impl OriginalEffectProducer for Close{
   type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut Option<RetainedCloneBinding> where 'b:'a;
   fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
   fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
   fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{RetainedCloneBinding::close_into(c,4096,effect,reply)}
   fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{RetainedCloneBinding::recover_close_into(c,4096,effect,reply)}
  }
  let mut authority=Some(OriginalEffectAuthority::new(Close));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if binding.is_none(){break}let demand=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let units=RetainedCloneBinding::close_work_demand(&binding,4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);let mut action=None;let mut work=0u128;let mut recorded=None;let mut observed_heap=(0,0);for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let recorded=authority.as_mut().unwrap().drive(&mut &mut binding,effect,&mut action).unwrap();let work=effect.performed_work()-before;if recorded{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneStep::Progress(Default::default())),work})}else{RetainedWorkPoll::Pending{work}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}observed_heap.0+=heap.0;observed_heap.1+=heap.1;work+=poll.work();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut recorded).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();break}}
   let step=recorded.take().expect("actual binding close reply").unwrap();assert_eq!(bank.take_admitted(),if work==0{None}else{Some(true)});bank.post(step.progress(),grant,observed_heap);
  }assert!(binding.is_none());finish_close_authority(&mut bank,&&mut binding,&mut authority);
 }
 let mut source_close=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let demand=source.next_close_demand(4096).unwrap();let work=source.next_close_work_demand(4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);close_event(&mut bank,&mut &mut source,source_close.as_mut().unwrap(),grant,work);}finish_close_authority(&mut bank,&&mut source,&mut source_close);
 assert!(source.terminal_is_empty());
}

/// 🧪️ The actual source closes through a resident borrowed producer and the same finite wallet.
#[test]
fn original_source_borrowed_close_keeps_its_actual_reply(){
 let (mut bank,mut admission,fixture)=projected();
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
 let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());let mut source=source.unwrap();let oracle:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();assert_eq!(*source.borrow().get(),oracle["sourceClose"]["value"].as_u64().unwrap());let mut authority=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let demand=source.next_close_demand(4096).unwrap();let units=source.next_close_work_demand(4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);let mut action=None;let mut reply=None;let mut work=0;let mut observed=(0,0);
  for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let ready=authority.as_mut().unwrap().drive(&mut &mut source,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneStep::Progress(Default::default())),work})}else{RetainedWorkPoll::Pending{work}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}observed.0+=heap.0;observed.1+=heap.1;work+=poll.work();if let RetainedWorkPoll::Complete(result)=poll{result.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();break}}
  let step=reply.take().expect("original source close reply").unwrap();assert_eq!(bank.take_admitted(),if work==0{None}else{Some(true)});bank.post(step.progress(),grant,observed);
 }
 assert_eq!(source.terminal_is_empty(),oracle["sourceClose"]["terminal"].as_bool().unwrap());finish_close_authority(&mut bank,&&mut source,&mut authority);
}

/// 🧪️ Paid staged retirement returns and retires each actual original frontier.
#[test]
fn staged_source_retirement_uses_the_same_borrowed_bank(){
 use semio_framework_value_clone_source::{OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalRetirementFailure,StagedSourceAdmission};
 use semio_framework_value_retirement::{RetireOwned,retirement::{RetirementCursor,RetirementStep}};
 type Cursor=<StagedSourceAdmission<u64,u64> as RetireOwned>::RetirementCursor;
 struct Close;
 impl OriginalEffectProducer for Close{type Output=RetirementStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut Cursor where 'b:'a;fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.close_into(4096,effect,reply)}fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.recover_close_into(4096,effect,reply)}}
  fn event(bank:&mut EffectBank,cursor:&mut Cursor,authority:&mut semio_framework_value_control_contract::OriginalEffectAuthority<Close>,grant:semio_framework_value_clone_source::RetainedCloneGrant,units:u64)->(RetirementStep,(usize,usize),u128){let mut action=None;let mut reply=None;let mut total=0u128;let mut observed=(0usize,0usize);for _ in 0..1024{let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut &mut *cursor,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work})}else{RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;if let Some(actual)=actual{assert_eq!(actual,observed.0)}total+=poll.work();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.take().unwrap().unwrap();let progress=match &step{RetirementStep::Progress(progress,work)=>{assert_eq!(*work,total);*progress},RetirementStep::Complete(work)=>{assert_eq!(*work,total);Default::default()},RetirementStep::Failure(_,_)=>panic!("staged retirement retained a failure"),RetirementStep::Pending(_)=>panic!("recorded retirement reply was pending"),_=>panic!("staged wrapper returned an unselected child")};assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,grant,observed);return(step,observed,total)}}panic!("staged retirement action remained pending")}
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/projection.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();for cut in law["stagedRetirement"]["cuts"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()).chain(std::iter::once(u64::MAX)){let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut staged=Some(StagedSourceAdmission::<u64,u64>::owned(law["stagedRetirement"]["input"].as_u64().unwrap()));let mut output=None;if cut==u64::MAX{assert!(law["stagedRetirement"]["readyCut"].as_bool().unwrap());let mut frame=None;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if matches!(staged.as_ref().unwrap().projection_selection(&frame).unwrap(),Some(semio_framework_value_clone_source::StagedProjectionSelection::Begin)){break}let owner=staged.as_mut().unwrap();let d=owner.next_demand().unwrap();let work=owner.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|owner.advance(&mut output,effect));assert!(output.is_none())}let owner=staged.as_mut().unwrap();let d=owner.projection_demand(&frame,false).unwrap();let work=owner.projection_work_demand(&frame,false).unwrap();turn(&mut bank,d,work,|effect|match owner.projection_step(&mut frame,false,effect){RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("staged projection capture pending")});let selected=owner.projection_frame(&mut frame).unwrap();let d=selected.projection_demand();let work=selected.projection_work_demand().unwrap();let grant=bank.loan(1,d.copy_bytes);let mut action=None;projection_turn(&mut bank,selected,&mut action,grant,work);for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if frame.is_none(){break}let d=owner.projection_demand(&frame,false).unwrap();let work=owner.projection_work_demand(&frame,false).unwrap();turn(&mut bank,d,work,|effect|match owner.projection_step(&mut frame,false,effect){RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("staged projection settlement pending")});}assert!(frame.is_none());assert_eq!(owner.projection_result(),Some(true));for _ in 0..4{let d=owner.next_demand().unwrap();let work=owner.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|owner.advance(&mut output,effect));assert!(output.is_none())}}else{for _ in 0..cut{let owner=staged.as_mut().unwrap();let d=owner.next_demand().unwrap();let work=owner.next_work_demand().unwrap();turn(&mut bank,d,work,|effect|owner.advance(&mut output,effect));assert!(output.is_none())}}let mut cursor=None;let demand=RetirementDemand{copy_bytes:std::mem::size_of::<Option<StagedSourceAdmission<u64,u64>>>()+std::mem::size_of::<Option<Cursor>>(),depth:1,..Default::default()};turn(&mut bank,demand,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());cursor=Some(staged.take().unwrap().retirement_seed());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});let mut authority=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(Close));let body=4096;let d=cursor.as_ref().unwrap().next_demand(body).unwrap();let units=cursor.as_ref().unwrap().next_computational_work(body).unwrap();let denied=bank.loan(0,0);let(step,heap,work)=event(&mut bank,cursor.as_mut().unwrap(),authority.as_mut().unwrap(),denied,units);assert_eq!(work,u128::from(law["stagedRetirement"]["deniedWork"].as_u64().unwrap()));assert!(matches!(step,RetirementStep::Progress(progress,0)if progress==Default::default()));assert_eq!(heap,(0,0));assert_eq!(cursor.as_ref().unwrap().next_demand(body).unwrap(),d);for _ in 0..1024{if cursor.as_ref().unwrap().terminal_is_empty(){break}let d=cursor.as_ref().unwrap().next_demand(body).unwrap();let units=cursor.as_ref().unwrap().next_computational_work(body).unwrap();let grant=bank.loan(1,d.copy_bytes);let(_,heap,_)=event(&mut bank,cursor.as_mut().unwrap(),authority.as_mut().unwrap(),grant,units);assert!(heap.0<=grant.maximum_capacity_bytes&&heap.1<=grant.maximum_release_bytes)}assert_eq!(cursor.as_ref().unwrap().terminal_is_empty(),law["stagedRetirement"]["ownerRetired"].as_bool().unwrap());let d=semio_framework_value_control_contract::OriginalEffectAuthority::<Close>::producer_transfer_demand().unwrap();let mut producer=None;turn(&mut bank,d,1,|effect|{let context=&mut *cursor.as_mut().unwrap();let result=semio_framework_value_control_contract::OriginalEffectAuthority::transfer_producer(&mut authority,&context,&mut producer,effect);RetainedWorkOutcome{result:result.result.map(|p|match p{Some(p)=>RetainedCloneStep::Complete(p),None=>RetainedCloneStep::Progress(Default::default())}),work:result.work}});let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<Close>>(),depth:1,..Default::default()};turn(&mut bank,d,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());producer.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<Cursor>>(),depth:1,..Default::default()};turn(&mut bank,d,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(cursor.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});}
}

#[test]
fn live_original_witness_returns_binding_without_retirement_scaffolding(){
 use semio_framework_value_clone_source::{RetainedCloneBinding,RetainedCloneRef};
 use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/bound-close.json")).unwrap();
 let(mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let units=admission.advance_work_demand().unwrap();turn(&mut bank,d,units,|effect|admission.advance(effect));}
 let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());let mut source=source.unwrap();let original=source.borrow().get()as *const u64;let mut binding=None;
 let d=source.borrow().binding_demand(&binding).unwrap().unwrap();let units=source.borrow().binding_work_demand(&binding).unwrap();turn(&mut bank,d,units,|effect|{let out=source.borrow().bind(&mut binding,effect);RetainedWorkOutcome{result:out.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:out.work}});
 assert!(RetainedCloneBinding::bound_close_available(&binding));
 #[derive(Clone,Copy)]struct Release;
 impl OriginalEffectProducer for Release{
  type Output=RetainedCloneStep;type Failure=CloseFailure;type Context<'a,'b>=(&'a mut Option<RetainedCloneBinding>,RetainedCloneRef<'a,u64>)where 'b:'a;
  fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
  fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
  fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{RetainedCloneBinding::close_bound_into(c.0,c.1,4096,effect,reply)}
  fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{Ok(reply.pending())}
 }
 let mut authority=Some(OriginalEffectAuthority::new(Release));let d=RetainedCloneBinding::bound_close_demand(&binding,source.borrow(),4096).unwrap();let grant=bank.loan(1,d.copy_bytes);
 for(g,work)in[(semio_framework_value_clone_source::RetainedCloneGrant{maximum_items:0,..grant},1),(semio_framework_value_clone_source::RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant},1),(grant,0)]{let before=bank.wallet().remaining_work();let step=close_event(&mut bank,&mut(&mut binding,source.borrow()),authority.as_mut().unwrap(),g,work);assert_eq!(step.progress(),Default::default());assert_eq!(bank.wallet().remaining_work(),before);assert!(RetainedCloneBinding::bound_close_available(&binding));assert_eq!(source.borrow().get()as *const u64,original);}
 let before=bank.wallet().remaining_work();let step=close_event(&mut bank,&mut(&mut binding,source.borrow()),authority.as_mut().unwrap(),grant,law["work"].as_u64().unwrap());let p=step.progress();assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(p.retained_capacity_bytes,law["allocations"].as_u64().unwrap()as usize);assert_eq!(p.released_bytes,law["releases"].as_u64().unwrap()as usize);assert_eq!(before-bank.wallet().remaining_work(),law["work"].as_u64().unwrap());assert!(binding.is_none());assert_eq!(source.borrow().get()as *const u64,original);assert_eq!(*source.borrow().get(),law["sourceValue"].as_u64().unwrap());finish_close_authority(&mut bank,&(&mut binding,source.borrow()),&mut authority);
 let mut close=Some(OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let d=source.next_close_demand(4096).unwrap();let work=source.next_close_work_demand(4096).unwrap();let grant=bank.loan(1,d.copy_bytes);close_event(&mut bank,&mut &mut source,close.as_mut().unwrap(),grant,work);}assert!(source.terminal_is_empty());finish_close_authority(&mut bank,&&mut source,&mut close);
}

#[test]
fn captured_projection_header_clear_is_separately_funded(){
 use semio_framework_value_clone_source::{RetainedOwnedProjectionAdmission,RetainedCloneRef};
 use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/captured-close.json")).unwrap();
 let(mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,d,work,|effect|admission.advance(effect));}
 let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());let mut source=source.unwrap();let pointer=source.borrow().get()as *const u64;
 let mut capture=RetainedOwnedProjectionAdmission::<u64>::default();let d=capture.demand(source.borrow()).unwrap();let work=capture.work_demand(source.borrow()).unwrap();let mut reply=None;turn(&mut bank,d,work,|effect|{let before=effect.performed_work();drop(capture.admit(source.borrow(),effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before}});
 #[derive(Clone,Copy)]struct Release;
 impl OriginalEffectProducer for Release{
  type Output=RetainedCloneStep;type Failure=CloseFailure;type Context<'a,'b>=(&'a mut RetainedOwnedProjectionAdmission<u64>,RetainedCloneRef<'a,u64>)where 'b:'a;
  fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
  fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
  fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.0.close_bound_into(c.1,4096,effect,reply)}
  fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.0.recover_close_bound_into(c.1,4096,effect,reply)}
 }
 let mut authority=Some(OriginalEffectAuthority::new(Release));let d=capture.bound_close_demand(source.borrow(),4096).unwrap();let g=bank.loan(1,d.copy_bytes);let before=bank.wallet().remaining_work();let step=close_event(&mut bank,&mut(&mut capture,source.borrow()),authority.as_mut().unwrap(),g,law["aliasWork"].as_u64().unwrap());assert!(matches!(step,RetainedCloneStep::Progress(_)));assert_eq!(before-bank.wallet().remaining_work(),1);assert!(!capture.terminal_is_empty());assert!(capture.borrow().is_err());
 let d=capture.bound_close_demand(source.borrow(),4096).unwrap();assert_eq!(d.copy_bytes,std::mem::size_of::<Option<std::ptr::NonNull<u64>>>());let g=bank.loan(1,d.copy_bytes);
 for(issued,work)in[(semio_framework_value_clone_source::RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},1),(g,0)]{let before=bank.wallet().remaining_work();let step=close_event(&mut bank,&mut(&mut capture,source.borrow()),authority.as_mut().unwrap(),issued,work);assert_eq!(step.progress(),Default::default());assert_eq!(before,bank.wallet().remaining_work());assert!(!capture.terminal_is_empty());}
 let step=close_event(&mut bank,&mut(&mut capture,source.borrow()),authority.as_mut().unwrap(),g,law["headerWork"].as_u64().unwrap());assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(step.progress().copied_bytes,d.copy_bytes);assert!(capture.terminal_is_empty());assert_eq!(source.borrow().get()as *const u64,pointer);finish_close_authority(&mut bank,&(&mut capture,source.borrow()),&mut authority);
 let mut close=Some(OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let d=source.next_close_demand(4096).unwrap();let work=source.next_close_work_demand(4096).unwrap();let g=bank.loan(1,d.copy_bytes);close_event(&mut bank,&mut &mut source,close.as_mut().unwrap(),g,work);}assert!(source.terminal_is_empty());finish_close_authority(&mut bank,&&mut source,&mut close);
}


#[test]
fn borrowed_handback_keeps_original_through_denial_and_returns_it(){
 use semio_framework_value_clone_source::{RetainedCloneSource,RetainedCloneGrant};
 use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity};
 type Source=RetainedCloneSource<u64,u64>;
 struct Context<'a>{source:&'a mut Source,output:&'a mut Option<u64>,turn:OriginalSourceTurnIdentity}
 #[derive(Clone,Copy)]struct Handback{turn:OriginalSourceTurnIdentity}
 impl OriginalEffectProducer for Handback{type Output=RetainedCloneStep;type Failure=CloseFailure;type Context<'a,'b>=Context<'a> where 'b:'a;fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.turn!=self.turn{return Err(ValueError::invariant("handback Source changed"))}Ok(())}fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.turn)}fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.source.take_authority_into(c.output,4096,effect,reply)}fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.source.recover_take_authority_into(c.output,4096,effect,reply)}}
 fn event(bank:&mut EffectBank,source:&mut Source,output:&mut Option<u64>,authority:&mut OriginalEffectAuthority<Handback>,grant:RetainedCloneGrant,units:u64)->RetainedCloneStep{let mut action=None;let mut reply=None;let mut observed=(0usize,0usize);for _ in 0..1024{let((poll,actual),heap)=observe_retirement_allocations(||bank.run_with_source(grant,units,|effect,witness|{let before=effect.performed_work();let mut context=Context{source,output,turn:witness.identity()};let ready=authority.drive(&mut context,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work})}else{RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;if let Some(actual)=actual{assert_eq!(actual,observed.0)}if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.take().unwrap().unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,observed);return step}}panic!("original borrowed handback remained pending")}
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/borrowed-handback.json")).unwrap();let(mut bank,mut admission,fixture)=projected();for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,d,work,|effect|admission.advance(effect));}let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());let mut source=source.unwrap();let original=source.borrow().get() as *const u64;let mut output=None;let mut authority=None;let(poll,actual)=bank.run_with_source(RetainedCloneGrant::default(),0,|_,witness|{authority=Some(OriginalEffectAuthority::new(Handback{turn:witness.identity()}));RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:0})});assert!(matches!(poll,RetainedWorkPoll::Complete(_)));assert!(actual.is_none());
 let d=source.next_take_demand(4096).unwrap();let work=source.next_take_work_demand(4096).unwrap();let grant=bank.loan(1,d.copy_bytes);let denied=RetainedCloneGrant{maximum_items:0,..grant};let before=bank.wallet().remaining_work();let step=event(&mut bank,&mut source,&mut output,authority.as_mut().unwrap(),denied,work);assert_eq!(step.progress(),Default::default());assert_eq!(before-bank.wallet().remaining_work(),law["deniedWork"].as_u64().unwrap());assert!(output.is_none());assert_eq!(source.borrow().get() as *const u64,original);
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let d=source.next_take_demand(4096).unwrap();let work=source.next_take_work_demand(4096).unwrap();let grant=bank.loan(1,d.copy_bytes);event(&mut bank,&mut source,&mut output,authority.as_mut().unwrap(),grant,work);}assert!(source.terminal_is_empty());assert_eq!(output,Some(law["value"].as_u64().unwrap()));let d=OriginalEffectAuthority::<Handback>::producer_transfer_demand().unwrap();let grant=bank.loan(1,d.copy_bytes);let mut producer=None;let((poll,_),heap)=observe_retirement_allocations(||bank.run_with_source(grant,1,|effect,witness|{let context=Context{source:&mut source,output:&mut output,turn:witness.identity()};RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut authority,&context,&mut producer,effect))}));let RetainedWorkPoll::Complete(completed)=poll else{panic!("handback producer transfer pending")};let progress=completed.result.unwrap().unwrap();assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,heap);let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<Handback>>(),depth:1,..Default::default()};turn(&mut bank,d,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());producer.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:effect.performed_work()-before}});
}

/// 🧭️ Native identity projection lends the original owned value without an implementer callback.
#[test]
fn owned_identity_projection_uses_the_actual_owner_and_funded_native_effect(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/owned-identity.json")).unwrap();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut admission=RetainedCloneSourceAdmission::<u64,u64>::new(law["value"].as_u64().unwrap(),());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="projectionStart"{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
 assert_eq!(admission.phase(),"projectionStart");let original=admission.original_authority().unwrap()as*const u64;let demand=admission.owned_projection_demand().unwrap();assert!(demand.copy_bytes<=law["maximumCopyBytes"].as_u64().unwrap()as usize);
 for (copy,work) in [(demand.copy_bytes,0),(demand.copy_bytes-1,1)]{let grant=bank.loan(1,copy);let before=*bank.wallet().original_work();let((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,work,|effect|RetainedWorkPoll::Complete(admission.project_owned(effect))));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("closed native identity pending")};let step=outcome.result.unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(outcome.work,0);assert_eq!(actual,None);assert_eq!(heap,(0,0));assert_eq!(*bank.wallet().original_work(),before);assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,heap);assert_eq!(admission.phase(),"projectionStart");}
 let before=*bank.wallet().original_work();turn(&mut bank,demand,1,|effect|admission.project_owned(effect));assert_eq!(before-*bank.wallet().original_work(),1);assert_eq!(admission.projection_result(),Some(true));
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.phase()=="ready"{break}let demand=admission.next_demand().unwrap();let work=admission.advance_work_demand().unwrap();turn(&mut bank,demand,work,|effect|admission.advance(effect));}
 let mut source=None;publish_original_source(&mut bank,&mut admission,&mut source,fixture["maximumTurns"].as_u64().unwrap());let mut source=source.unwrap();assert_eq!(source.borrow().get()as*const u64,original);assert_eq!(*source.borrow().get(),73);
 let mut authority=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let d=source.next_close_demand(4096).unwrap();let units=source.next_close_work_demand(4096).unwrap();let g=bank.loan(1,d.copy_bytes);close_event(&mut bank,&mut&mut source,authority.as_mut().unwrap(),g,units);}assert!(source.terminal_is_empty());finish_close_authority(&mut bank,&&mut source,&mut authority);
}

/// 🪜️ Every native admission seed and identity frontier returns its original value on cancellation.
#[test]
fn staged_owned_identity_preserves_every_early_cancellation_frontier(){
 use semio_framework_value_clone_source::StagedSourceAdmission;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/owned-identity.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️admission/case.json")).unwrap();
 let mut published=0usize;for cut in law["cancellationCuts"].as_array().unwrap(){let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));let mut admission=StagedSourceAdmission::<u64,u64>::owned(73);let(mut source,mut returned)=(None,None);
  for _ in 0..cut.as_u64().unwrap(){if admission.terminal_is_empty(){break}let demand=admission.next_owned_demand().unwrap();let units=admission.next_owned_work_demand().unwrap();assert!(demand.copy_bytes<=4096);turn(&mut bank,demand,units,|effect|admission.advance_owned(&mut source,effect));}
  for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if admission.terminal_is_empty(){break}let demand=admission.owned_close_demand().unwrap();let units=admission.owned_close_work_demand().unwrap();assert!(demand.copy_bytes<=4096);turn(&mut bank,demand,units,|effect|admission.close_owned_into(&mut returned,&mut source,effect));}
  assert!(admission.terminal_is_empty());if let Some(mut source)=source{published+=1;assert!(returned.is_none());assert_eq!(*source.borrow().get(),73);let mut authority=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(SourceClose));for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let demand=source.next_close_demand(4096).unwrap();let units=source.next_close_work_demand(4096).unwrap();let grant=bank.loan(1,demand.copy_bytes);close_event(&mut bank,&mut&mut source,authority.as_mut().unwrap(),grant,units);}assert!(source.terminal_is_empty());finish_close_authority(&mut bank,&&mut source,&mut authority);}else{assert_eq!(returned,Some(73));let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<u64>>(),depth:1,..Default::default()};turn(&mut bank,d,1,|effect|{assert!(effect.performed(1).unwrap());returned.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}});}
 }assert!(published>0&&published<law["cancellationCuts"].as_array().unwrap().len());
}
