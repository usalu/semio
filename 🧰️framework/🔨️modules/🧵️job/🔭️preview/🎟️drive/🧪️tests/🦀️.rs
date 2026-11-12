//! 🧪️ Actual native preview turns retain the original receipts, payload loans and physical owners.
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use super::*;
#[derive(Clone,Copy)]
enum Event{Yield,Preview,Checkpoint,Complete,Cancelled,Fault}
struct OriginalPreviewJob{events:&'static[Event],cursor:usize,publication:RetainedJobPublication,delivered:bool,closing:bool}
const FAULT:&[u8]=b"original exact producer fault bytes";
impl OriginalPreviewJob{
 fn demand(&self)->Result<RetirementDemand,ValueError>{if self.publication.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{self.publication.retirement_demands()}}
}
impl InteractiveJob for OriginalPreviewJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{
  fn body<'b>(this:&'b mut OriginalPreviewJob,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'b>>,OriginalTurnFailure>{
   if this.delivered{
    let turn=cx.with_original_effect(cx.retained_grant(),|_,effect|this.publication.close_step(effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},|(turn,_)|turn);
    let step=receive_original_retained_work(cx,turn)?;if matches!(step,RetainedCloneStep::Complete(_)){this.delivered=false;this.cursor+=1;}return Ok(None)
   }
   let event=this.events.get(this.cursor).copied().unwrap_or(Event::Yield);
   match event{
    Event::Yield=>{let result=JobOutcomeBorrow::admit_yield(cx)?;if result.is_some(){this.cursor+=1;}Ok(result)},
    Event::Complete=>JobOutcomeBorrow::admit_complete(cx,None,None).map_err(OriginalTurnFailure::from),
    Event::Cancelled=>JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from),
    event=>{let kind=match event{Event::Preview=>JobPublicationKind::Preview,Event::Checkpoint=>JobPublicationKind::Checkpoint{applied_progress:7},Event::Fault=>JobPublicationKind::Fault,_=>unreachable!()};let result=this.publication.advance_from_source(kind,FAULT,cx)?;if result.is_some(){this.delivered=true;}Ok(result)}
   }
  }
  InteractiveJobStep::from_callback(body(self,cx))
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete=>descriptor.complete(None,None),_=>self.publication.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let demand=self.demand().map_err(OriginalTurnFailure::from)?;
  if !self.publication.terminal_is_empty(){return close_original_effect_step(cx,demand,|effect|semio_framework_value::RetainedWorkPoll::Complete(self.publication.close_step(effect)))}
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  if !self.delivered{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
  self.delivered=false;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(InteractiveJobCloseStep::Pending{progress})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&!self.delivered&&self.publication.terminal_is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.demand()?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand()?.depth)}
}
fn observed<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn policy(row:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:row[0].as_u64().unwrap()as usize,maximum_copy_bytes:row[1].as_u64().unwrap()as usize,maximum_capacity_bytes:row[2].as_u64().unwrap()as usize,maximum_release_bytes:row[3].as_u64().unwrap()as usize,maximum_depth:row[4].as_u64().unwrap()as usize}}
fn stock(admission:RetainedCloneGrant,step:RetainedCloneGrant,turns:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:(admission.maximum_items+step.maximum_items)*turns,maximum_copy_bytes:(admission.maximum_copy_bytes+step.maximum_copy_bytes)*turns,maximum_capacity_bytes:(admission.maximum_capacity_bytes+step.maximum_capacity_bytes)*turns,maximum_release_bytes:(admission.maximum_release_bytes+step.maximum_release_bytes)*turns,maximum_depth:admission.maximum_depth.max(step.maximum_depth)}}
fn original_clock()->Option<u64>{Some(1000)}
fn refusal(error:ValueError)->OriginalTurnFailure{OriginalTurnFailure::from(error)}
enum OriginalPreviewFixtureSession{Typed(BatchJobSession<OriginalPreviewJob>),Erased(ErasedBatchJobSession)}
impl OriginalPreviewFixtureSession{
 fn original(&self)->&dyn OriginalPreviewSession{match self{Self::Typed(session)=>session,Self::Erased(session)=>session}}
 fn original_mut(&mut self)->&mut dyn OriginalPreviewSession{match self{Self::Typed(session)=>session,Self::Erased(session)=>session}}
}
fn admit_batch(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,job:OriginalPreviewJob,params:BatchJobParams,turns:usize)->BatchJobSession<OriginalPreviewJob>{
 let mut job=Some(job);let mut params=Some(params);let mut output=None;let mut admission=WorkerJobSessionAdmission::<OriginalPreviewJob,BatchJobSession<OriginalPreviewJob>>::new();
 for _ in 0..turns{if output.is_some(){break}let funded=caller.remaining_grant();caller.turn(funded,|cx|admission.advance(&mut job,&mut params,&mut output,cx)).unwrap();}
 assert!(admission.terminal_is_empty()&&job.is_none()&&params.is_none());output.expect("actual preview session output")
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
 let turns=law["maximumTurns"].as_u64().unwrap()as usize;let stock=stock(admission,grant,turns);
 for (index,events)in[&[Event::Yield,Event::Complete][..],&[Event::Preview,Event::Checkpoint,Event::Complete][..],&[Event::Cancelled][..],&[Event::Fault][..],&[Event::Yield][..]].into_iter().enumerate(){
  let row=&law["cases"][index];
  let mut caller=OriginalJobFixtureCaller::admit(operation,generation,stock,original_clock,u64::MAX,u64::try_from(turns).unwrap()*16);caller.audit_heap();
  let params=BatchJobParams{operation,generation,cancel:caller.admit_alias(admission),config:BatchDriveConfig{retained:admission,site:"original-preview-driver",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:100000},now_us:original_clock};
  let job=OriginalPreviewJob{events,cursor:0,publication:RetainedJobPublication::new(),delivered:false,closing:false};
  let session=admit_batch(&mut caller,admission,job,params,turns);
  assert_eq!(session.original_identity().unwrap(),Some((operation,generation)));
  let mut session=if erased{
   let mut source=Some(session);let mut erasure=None;
   assert!(caller.turn(grant,|cx|OriginalBatchSessionErasure::try_begin(&mut erasure,&mut source,cx).map_err(refusal)).unwrap().is_some());
   let mut ready=None;for _ in 0..(if partial_cancel{1}else{2}){let(next,progress)=caller.turn(grant,|cx|erasure.as_mut().unwrap().advance(&mut source,cx).map_err(refusal)).unwrap();assert_eq!(progress,caller.receipt());assert_eq!(progress.copied_bytes,0);ready=next;}
   if partial_cancel{
    assert!(source.is_none());caller.cancel().cancel_now();erasure.as_mut().unwrap().begin_close();
    for _ in 0..3{if erasure.as_ref().unwrap().terminal_is_empty(){break}
     for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=observed(||caller.turn(denied,|cx|erasure.as_mut().unwrap().close_step(&mut source,cx).map_err(refusal)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(caller.receipt(),Default::default());assert_eq!(heap,(0,0));}
     let step=caller.turn(grant,|cx|erasure.as_mut().unwrap().close_step(&mut source,cx).map_err(refusal)).unwrap();assert_eq!(step.progress(),caller.receipt());assert_eq!(step.progress().copied_bytes,0);
    }
    assert!(source.is_some()&&erasure.as_ref().unwrap().terminal_is_empty());assert_eq!(observed(||drop(erasure)).1,(0,0));OriginalPreviewFixtureSession::Typed(source.take().unwrap())
   }else{assert!(source.is_none()&&erasure.as_ref().unwrap().terminal_is_empty());assert_eq!(observed(||drop(erasure)).1,(0,0));OriginalPreviewFixtureSession::Erased(ready.unwrap())}
  }else{OriginalPreviewFixtureSession::Typed(session)};
  let original_session=session.original()as*const dyn OriginalPreviewSession as*const ();let limits=JobPreviewLimits{maximum_turns:row["maximumTurns"].as_u64().unwrap_or(law["maximumTurns"].as_u64().unwrap()),maximum_wall_us:law["maximumWallMicros"].as_u64().unwrap()};let mut driver=JobPreviewDriver::new(limits);let mut count=0;let mut semantic_fault=0;
  let verdict=loop{
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observed(||caller.turn(denied,|cx|driver.advance(session.original_mut(),cx).map(|step|matches!(step,JobPreviewStep::Pending))).unwrap());assert!(result);assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());}
   let phase=driver.phase;let mut fault_input=false;
   let result=caller.turn(grant,|cx|{let step=driver.advance(session.original_mut(),cx)?;Ok(match step{JobPreviewStep::Complete(verdict)=>Some(Ok(verdict)),JobPreviewStep::Fault{detail}=>Some(Err((detail as*const RetainedJobPayload as usize,detail.len(),detail.page(0).unwrap().as_ptr()as usize,(0..detail.page_count()).flat_map(|index|detail.page(index).unwrap().to_vec()).collect::<Vec<u8>>()))),JobPreviewStep::Pending=>None})}).unwrap();
   if !matches!(phase,JobPreviewPhase::Run|JobPreviewPhase::Close){assert_eq!(caller.receipt().copied_bytes,0)}
   match result{Some(Ok(verdict))=>break verdict,Some(Err((pointer,length,page,bytes)))=>{semantic_fault+=1;assert_eq!(length,FAULT.len());assert_eq!(bytes,FAULT);let zero_copy=RetainedCloneGrant{maximum_copy_bytes:0,..grant};for _ in 0..3{let(again,heap)=observed(||caller.turn(zero_copy,|cx|{Ok(match driver.advance(session.original_mut(),cx)?{JobPreviewStep::Fault{detail}=>Some((detail as*const RetainedJobPayload as usize,detail.page(0).unwrap().as_ptr()as usize)),_=>None})}).unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(again,Some((pointer,page)),"Fault input must remain borrowed before explicit acceptance");}fault_input=true},None=>{}}
   if fault_input{for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(accepted,heap)=observed(||caller.turn(denied,|cx|driver.accept_fault_input(session.original(),cx).map_err(refusal)).unwrap());assert!(!accepted);assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());}let zero_copy=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let accepted=caller.turn(zero_copy,|cx|driver.accept_fault_input(session.original(),cx).map_err(refusal)).unwrap();assert!(accepted);assert_eq!(caller.receipt(),RetainedCloneProgress{copied_items:1,..Default::default()});}
   count+=1;assert!(count<turns);assert_eq!(session.original()as*const dyn OriginalPreviewSession as*const (),original_session);
  };
  let expected=if partial_cancel{JobPreviewStatus::Cancelled}else{match row["verdict"].as_str().unwrap(){"complete"=>JobPreviewStatus::Complete,"cancelled"=>JobPreviewStatus::Cancelled,"fault"=>JobPreviewStatus::Fault,"budget"=>JobPreviewStatus::Budget,_=>unreachable!()}};assert_eq!(verdict,JobPreviewVerdict{status:expected,outcomes:if partial_cancel{0}else{row["outcomes"].as_u64().unwrap()}});assert_eq!(semantic_fault,usize::from(expected==JobPreviewStatus::Fault));assert!(session.original().terminal_is_empty()&&driver.terminal_is_empty());assert_eq!(session.original().original_identity().unwrap(),None);assert!(session.original_mut().take_checked_out_retained_step_receipt().is_none());assert_eq!(observed(||drop(session)).1,(0,0));
  caller.retire(admission);assert_eq!(observed(||drop(driver)).1,(0,0));
 }
}
