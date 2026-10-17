//! 🧪️ Actual native preview turns retain the original receipts, payload loans and physical owners.
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use super::*;
use std::mem::MaybeUninit;
#[derive(Clone,Copy)]
enum Event{Yield,Preview,Checkpoint,Complete,Cancelled,Fault}
struct OriginalPreviewJob{events:&'static[Event],cursor:usize,publication:RetainedJobPublication,delivered:bool,closing:bool}
const FAULT:&[u8]=b"original exact producer fault bytes";
struct OriginalPreviewInitializer{events:Option<&'static[Event]>,stage:u8,target:Option<usize>,publication_present:bool}
unsafe impl WorkerJobInitializer<OriginalPreviewJob> for OriginalPreviewInitializer{
 fn initialization_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 unsafe fn initialize_step(&mut self,target:*mut MaybeUninit<OriginalPreviewJob>,grant:RetainedCloneGrant)->Result<WorkerJobInitializationStep,ValueError>{
  if self.target.is_some_and(|original|original!=target as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"preview initializer requires its same original final native target"))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
  self.target=Some(target as usize);let target=target.cast::<OriginalPreviewJob>();
  unsafe{match self.stage{
   0=>std::ptr::addr_of_mut!((*target).events).write(self.events.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview event source is absent"))?),
   1=>std::ptr::addr_of_mut!((*target).cursor).write(0),
   2=>{std::ptr::addr_of_mut!((*target).publication).write(RetainedJobPublication::new());self.publication_present=true;},
   3=>std::ptr::addr_of_mut!((*target).delivered).write(false),
   4=>std::ptr::addr_of_mut!((*target).closing).write(false),
   5=>self.publication_present=false,
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview initializer already transferred its final native fields"))
  }}self.stage+=1;Ok(WorkerJobInitializationStep{progress:RetainedCloneProgress{copied_items:1,..Default::default()},complete:self.stage==6})
 }
 fn initialization_input_is_empty(&self)->bool{self.stage==6&&self.events.is_none()&&!self.publication_present}
 fn partial_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:usize::from(!self.partial_terminal_is_empty()),..Default::default()})}
 unsafe fn close_partial_step(&mut self,target:*mut MaybeUninit<OriginalPreviewJob>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.target.is_some_and(|original|original!=target as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"preview partial close requires its same original final native target"))}
  if self.partial_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.publication_present{unsafe{let publication=std::ptr::addr_of_mut!((*target.cast::<OriginalPreviewJob>()).publication);assert!((*publication).terminal_is_empty());std::ptr::drop_in_place(publication);}self.publication_present=false;}else{self.events.take();}
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 fn partial_terminal_is_empty(&self)->bool{self.events.is_none()&&!self.publication_present}
}

impl OriginalPreviewJob{
 fn demand(&self)->Result<RetirementDemand,ValueError>{if self.publication.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{self.publication.retirement_demands()}}
}
impl InteractiveJob for OriginalPreviewJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  if self.delivered{let step=self.publication.close_step(cx.retained_grant())?;cx.consume_retained(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.delivered=false;self.cursor+=1;}return Ok(None)}
  let event=self.events.get(self.cursor).copied().unwrap_or(Event::Yield);
  match event{
   Event::Yield=>{let result=JobOutcomeBorrow::admit_yield(cx)?;if result.is_some(){self.cursor+=1;}Ok(result)},
   Event::Complete=>JobOutcomeBorrow::admit_complete(cx,None,None).map_err(OriginalTurnFailure::from),
   Event::Cancelled=>JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from),
   event=>{let kind=match event{Event::Preview=>JobPublicationKind::Preview,Event::Checkpoint=>JobPublicationKind::Checkpoint{applied_progress:7},Event::Fault=>JobPublicationKind::Fault,_=>unreachable!()};let result=self.publication.advance_from_source(kind,FAULT,cx)?;if result.is_some(){self.delivered=true;}Ok(result)}
  }
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete=>descriptor.complete(None,None),_=>self.publication.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let grant=cx.retained_grant();let demand=self.demand().map_err(OriginalTurnFailure::from)?;
  if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let step=if !self.publication.terminal_is_empty(){match self.publication.close_step(grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}}else if self.delivered{self.delivered=false;InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}}}else{InteractiveJobCloseStep::Complete{progress:Default::default()}};
  cx.consume_retained(step.progress()).map_err(OriginalTurnFailure::from)?;if step.progress()!=Default::default(){cx.consume_fuel(1);}Ok(step)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&!self.delivered&&self.publication.terminal_is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.demand()?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.depth)}
}
fn observed<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[track_caller]
fn physical(progress:RetainedCloneProgress,heap:(usize,usize),grant:RetainedCloneGrant){assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap)}
fn policy(row:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:row[0].as_u64().unwrap()as usize,maximum_copy_bytes:row[1].as_u64().unwrap()as usize,maximum_capacity_bytes:row[2].as_u64().unwrap()as usize,maximum_release_bytes:row[3].as_u64().unwrap()as usize,maximum_depth:row[4].as_u64().unwrap()as usize}}
fn original_clock()->Option<u64>{Some(1000)}
enum OriginalPreviewFixtureSession{Typed(BatchJobSession<OriginalPreviewJob>),Erased(ErasedBatchJobSession)}
impl OriginalPreviewFixtureSession{
 fn original(&self)->&dyn OriginalPreviewSession{match self{Self::Typed(session)=>session,Self::Erased(session)=>session}}
 fn original_mut(&mut self)->&mut dyn OriginalPreviewSession{match self{Self::Typed(session)=>session,Self::Erased(session)=>session}}
}
#[test]
fn original_preview_driver_preserves_native_receipts_fault_loan_and_paid_closure(){
 original_preview_corpus(false,false);
}
#[test]
fn original_erased_preview_driver_preserves_native_receipts_fault_loan_and_paid_closure(){
 original_preview_corpus(true,false);
}
#[test]
fn original_cancelled_preview_erasure_returns_original_source_before_paid_closure(){original_preview_corpus(true,true);}
fn original_preview_corpus(erased:bool,partial_cancel:bool){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let admission=policy(&law["admissionGrant"]);let grant=policy(&law["stepGrant"]);let operation=OperationId(law["originalIdentity"][0].as_u64().unwrap());let generation=Generation(law["originalIdentity"][1].as_u64().unwrap());assert_eq!(law["faultSource"].as_str().unwrap().as_bytes(),FAULT);
 for (index,events)in[&[Event::Yield,Event::Complete][..],&[Event::Preview,Event::Checkpoint,Event::Complete][..],&[Event::Cancelled][..],&[Event::Fault][..],&[Event::Yield][..]].into_iter().enumerate(){
  let row=&law["cases"][index];let((cancel,birth),heap)=observed(||CancelToken::admit_root(admission).unwrap().unwrap());physical(birth,heap,admission);let mut born=heap.0;let mut released=heap.1;let((mut authority,birth),heap)=observed(||JobPayloadAuthority::admit(operation,generation,admission).unwrap().unwrap());physical(birth,heap,admission);born+=heap.0;released+=heap.1;let mut sequence=0;let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,admission),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let(alias,heap)=observed(||cx.admit_original_cancel_alias().unwrap().unwrap());physical(cx.retained_progress(),heap,admission);drop(cx);
  let mut original=OriginalPreviewInitializer{events:Some(events),stage:0,target:None,publication_present:false};let mut params=Some(BatchJobParams{operation,generation,cancel:alias,config:BatchDriveConfig{retained:admission,site:"original-preview-driver",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:100000},now_us:original_clock});let mut recipient=Default::default();let mut control=WorkerJobAdmissionContext::new(operation,generation,StepBudget::new(1,u64::MAX,admission),original_clock,&mut recipient).unwrap();let((mut preparation,progress),heap)=observed(||WorkerJobSessionPreparation::try_begin(&mut original,&mut params,&mut control).unwrap().unwrap());drop(control);assert_eq!(recipient,progress);physical(progress,heap,admission);born+=heap.0;released+=heap.1;let mut prepared=None;let mut preparation_turns=0;while prepared.is_none(){let mut recipient=Default::default();let mut control=WorkerJobAdmissionContext::new(operation,generation,StepBudget::new(1,u64::MAX,admission),original_clock,&mut recipient).unwrap();let((ready,progress),heap)=observed(||preparation.advance(&mut original,&mut params,&mut control).unwrap());drop(control);assert_eq!(recipient,progress);physical(progress,heap,admission);born+=heap.0;released+=heap.1;prepared=ready;preparation_turns+=1;assert!(preparation_turns<10000);}assert!(preparation.terminal_is_empty()&&original.initialization_input_is_empty()&&params.is_none());assert_eq!(observed(||drop(preparation)).1,(0,0));let original_native_job=original.target.unwrap();let mut recipient=Default::default();let mut control=WorkerJobAdmissionContext::new(operation,generation,StepBudget::new(1,u64::MAX,admission),original_clock,&mut recipient).unwrap();let((mut session,progress),heap)=observed(||BatchJobSession::admit_prepared(&mut prepared,&mut control).unwrap().unwrap());drop(control);assert!(prepared.is_none());assert_eq!(recipient,progress);physical(progress,heap,admission);assert_eq!(heap,(0,0));assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!(session.original_identity().unwrap(),Some((operation,generation)));let mut session=if erased{let mut source=Some(session);let mut erasure=None;let mut recipient=Default::default();let mut control=WorkerJobAdmissionContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),original_clock,&mut recipient).unwrap();let(progress,heap)=observed(||OriginalBatchSessionErasure::try_begin(&mut erasure,&mut source,&mut control).unwrap().unwrap());drop(control);assert_eq!(recipient,progress);physical(progress,heap,grant);born+=heap.0;released+=heap.1;let mut ready=None;for _ in 0..(if partial_cancel{1}else{2}){let mut recipient=Default::default();let mut control=WorkerJobAdmissionContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),original_clock,&mut recipient).unwrap();let(next,heap)=observed(||erasure.as_mut().unwrap().advance(&mut source,&mut control).unwrap());drop(control);assert_eq!(recipient,next.1);physical(next.1,heap,grant);assert_eq!(next.1.copied_bytes,0);born+=heap.0;released+=heap.1;ready=next.0;}if partial_cancel{assert!(source.is_none());cancel.cancel_now();erasure.as_mut().unwrap().begin_close();for _ in 0..3{if erasure.as_ref().unwrap().terminal_is_empty(){break}for (fuel,deadline,denied) in [(0,u64::MAX,grant),(1,1000,grant),(1,u64::MAX,RetainedCloneGrant{maximum_items:0,..grant}),(1,u64::MAX,RetainedCloneGrant{maximum_depth:0,..grant})]{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(fuel,deadline,denied),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();assert!(cx.is_cancelled());let(step,heap)=observed(||erasure.as_mut().unwrap().close_step(&mut source,&mut cx).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(cx.retained_progress(),Default::default());assert_eq!(heap,(0,0));}let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();assert!(!cx.admission_is_open()&&cx.retirement_is_open());let(step,heap)=observed(||erasure.as_mut().unwrap().close_step(&mut source,&mut cx).unwrap());physical(step.progress(),heap,grant);assert_eq!(step.progress(),cx.retained_progress());assert_eq!(step.progress().copied_bytes,0);born+=heap.0;released+=heap.1;}assert!(source.is_some()&&erasure.as_ref().unwrap().terminal_is_empty());assert_eq!(observed(||drop(erasure)).1,(0,0));OriginalPreviewFixtureSession::Typed(source.take().unwrap())}else{assert!(source.is_none()&&erasure.as_ref().unwrap().terminal_is_empty());assert_eq!(observed(||drop(erasure)).1,(0,0));OriginalPreviewFixtureSession::Erased(ready.unwrap())}}else{OriginalPreviewFixtureSession::Typed(session)};let original_session=session.original()as*const dyn OriginalPreviewSession as*const ();let limits=JobPreviewLimits{maximum_turns:row["maximumTurns"].as_u64().unwrap_or(law["maximumTurns"].as_u64().unwrap()),maximum_wall_us:law["maximumWallMicros"].as_u64().unwrap()};let mut driver=JobPreviewDriver::new(limits);let mut turns=0;let mut semantic_fault=0;let mut total_copy=0;
  let verdict=loop{
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,denied),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let(result,heap)=observed(||driver.advance(session.original_mut(),&mut cx).unwrap());assert!(matches!(result,JobPreviewStep::Pending));assert_eq!(heap,(0,0));assert_eq!(cx.retained_progress(),Default::default());}
   let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let phase=driver.phase;let(result,heap)=observed(||driver.advance(session.original_mut(),&mut cx).unwrap());let progress=cx.retained_progress();if (progress.retained_capacity_bytes,progress.released_bytes)!=heap{}physical(progress,heap,grant);born+=heap.0;released+=heap.1;total_copy+=progress.copied_bytes;if phase!=JobPreviewPhase::Run{assert_eq!(progress.copied_bytes,0)}drop(cx);
   let mut fault_input=false;match result{JobPreviewStep::Complete(verdict)=>break verdict,JobPreviewStep::Fault{detail}=>{semantic_fault+=1;assert_eq!(detail.len(),FAULT.len());let pointer=detail as*const _;let page=detail.page(0).unwrap().as_ptr();let mut offset=0;for index in 0..detail.page_count(){let bytes=detail.page(index).unwrap();assert_eq!(bytes,&FAULT[offset..offset+bytes.len()]);offset+=bytes.len();}assert_eq!(offset,FAULT.len());for _ in 0..3{let mut receipt=Default::default();let zero_copy=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,zero_copy),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let(result,heap)=observed(||driver.advance(session.original_mut(),&mut cx).unwrap());assert_eq!(heap,(0,0));assert_eq!(cx.retained_progress(),Default::default());match result{JobPreviewStep::Fault{detail}=>{assert_eq!(detail as*const _,pointer);assert_eq!(detail.page(0).unwrap().as_ptr(),page)},_=>panic!("Fault input must remain borrowed before explicit acceptance")}}fault_input=true},JobPreviewStep::Pending=>{}}
   if fault_input{for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,denied),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let(accepted,heap)=observed(||driver.accept_fault_input(session.original(),&mut cx).unwrap());assert!(!accepted);assert_eq!(heap,(0,0));assert_eq!(cx.retained_progress(),Default::default());}let mut receipt=Default::default();let zero_copy=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,zero_copy),&cancel,original_clock,&mut sequence,&mut receipt,&authority).unwrap();let(accepted,heap)=observed(||driver.accept_fault_input(session.original(),&mut cx).unwrap());assert!(accepted);physical(cx.retained_progress(),heap,zero_copy);assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,..Default::default()});}
   turns+=1;assert!(turns<10000);assert_eq!(session.original()as*const dyn OriginalPreviewSession as*const (),original_session);
  };
  let expected=if partial_cancel{JobPreviewStatus::Cancelled}else{match row["verdict"].as_str().unwrap(){"complete"=>JobPreviewStatus::Complete,"cancelled"=>JobPreviewStatus::Cancelled,"fault"=>JobPreviewStatus::Fault,"budget"=>JobPreviewStatus::Budget,_=>unreachable!()}};assert_eq!(verdict,JobPreviewVerdict{status:expected,outcomes:if partial_cancel{0}else{row["outcomes"].as_u64().unwrap()}});assert_eq!(semantic_fault,usize::from(expected==JobPreviewStatus::Fault));assert!(session.original().terminal_is_empty()&&driver.terminal_is_empty());assert_eq!(session.original().original_identity().unwrap(),None);assert!(session.original_mut().take_checked_out_retained_step_receipt().is_none());assert_eq!(observed(||drop(session)).1,(0,0));let(step,heap)=observed(||authority.close_step(admission).unwrap());physical(step.progress(),heap,admission);born+=heap.0;released+=heap.1;assert!(authority.terminal_is_empty());let mut root=semio_framework_async::CancelTokenRetirement::from_token(cancel);while !root.terminal_is_empty(){let(step,heap)=observed(||root.close_step(RetainedCloneGrant{maximum_copy_bytes:0,..admission}).unwrap());physical(step.progress(),heap,admission);born+=heap.0;released+=heap.1;}assert_eq!(born,released);assert_eq!(observed(||{drop(authority);drop(root);drop(driver)}).1,(0,0));
 }
}

