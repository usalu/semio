//! 🛬️ Original preview pages, native tick custody and immutable presentation advance under one caller.
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_tool_run::{ToolRunIdentity,ToolRunState,ToolRunTick};
use semio_framework_tool_run_tick_wire::tick_receive::{ToolRunTickNativeCursor,OriginalTickWireSource};
use semio_framework_tool_run_presentation::{slot::ToolRunPresentationSlot,ToolRunCapturedView};
use semio_framework_tool_run_entities::ToolRunEntityEditGrant;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
fn built<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,demand,|_|semio_framework_value::RetainedWorkOutcome{result:construct(),work:1})}
fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}

struct Owned{presentation:Option<ToolRunPresentationSlot>,tick:Option<ToolRunTick>}
semio_framework_value::artifact_retire_struct!(Owned{presentation,tick});
pub struct OriginalToolRunNativeState{owned:ManuallyDrop<Owned>,cursor:Option<Box<ToolRunTickNativeCursor>>,outcome:Option<semio_framework_job::OriginalNativeReceivingOutcome>,stepped:Option<Result<RetainedCloneStep,semio_framework_value::native_decoding::OriginalReceivingPrimary>>,close:Option<ControlledRetirement<Owned>>,adv_reply:Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,adv_pending:bool,close_reply:Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,close_pending:bool,phase:u8,closing:bool,invalidated:bool}
fn cursor_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<ToolRunTickNativeCursor>>>(),release_bytes:size_of::<ToolRunTickNativeCursor>(),depth:1,..Default::default()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn funds(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d)}
impl OriginalToolRunNativeState{
 pub fn empty()->Self{Self{owned:ManuallyDrop::new(Owned{presentation:None,tick:None}),cursor:None,outcome:None,stepped:None,close:None,adv_reply:None,adv_pending:false,close_reply:None,close_pending:false,phase:0,closing:false,invalidated:false}}
 pub fn initialize(&mut self,tool_id:&str,identity:ToolRunIdentity,state:ToolRunState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.invalidated&&!self.terminal_is_empty(){return Ok(false)}
  if self.closing||self.invalidated{let d=RetirementDemand{copy_bytes:2*size_of::<bool>(),depth:1,..Default::default()};if !funds(cx,d){return Ok(false)}self.closing=false;self.invalidated=false;self.phase=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if self.owned.presentation.is_some(){return Ok(true)}
  let d=RetirementDemand{copy_bytes:tool_id.len()+size_of::<ToolRunPresentationSlot>(),capacity_bytes:tool_id.len(),depth:1,..Default::default()};if !funds(cx,d){return Ok(false)}
  let mut id=String::new();id.try_reserve_exact(tool_id.len()).map_err(|_|invalid("original run identifier backing refused"))?;id.push_str(tool_id);
  self.owned.presentation=Some(ToolRunPresentationSlot::new(id,identity,state));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 pub fn capture(&self,identity:ToolRunIdentity,state:ToolRunState,effect:&mut semio_framework_value::retained_clone::RetainedAllocationEffect<'_>)->Result<Option<(ToolRunCapturedView,RetainedCloneProgress)>,ValueError>{if self.invalidated{return Ok(None)}self.owned.presentation.as_ref().map_or(Ok(None),|presentation|presentation.capture_view(identity,state,effect))}
 pub fn borrowed_view(&self,identity:ToolRunIdentity,state:ToolRunState)->Option<semio_framework_tool_run_presentation::ToolRunView<'_>>{if self.invalidated{return None}self.owned.presentation.as_ref()?.borrowed_view(identity,state)}
 pub fn has_pending_tick(&self)->bool{self.phase!=0||self.cursor.is_some()||self.outcome.is_some()||self.owned.tick.is_some()}
 pub fn request_close(&mut self){self.invalidated=true;}
 pub fn close_requested(&self)->bool{self.invalidated&&!self.terminal_is_empty()}
 pub fn receive(&mut self,source:&dyn OriginalTickWireSource,identity:ToolRunIdentity,state:ToolRunState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.receive_with_domain(source,identity,state,|tick,_|{if !tick.append_ops.is_empty(){return Err(invalid("a mutating original tick requires its installed domain receiver").into())}Ok(Some(tick.retract_to.unwrap_or(0)))},cx)}
 pub fn receive_with_domain(&mut self,source:&dyn OriginalTickWireSource,identity:ToolRunIdentity,state:ToolRunState,mut domain:impl FnMut(&mut ToolRunTick,&mut StepContext<'_>)->Result<Option<u32>,OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(invalid("original run presentation is closing").into())}
  let owned=&mut *self.owned;
  let presentation=owned.presentation.as_mut().ok_or_else(||invalid("original run presentation is absent"))?;
  if !presentation.ready(){advance_presentation(presentation,&mut self.adv_reply,&mut self.adv_pending,cx)?;return Ok(false)}
  match self.phase{
   0=>{if self.cursor.is_none(){let bytes=size_of::<ToolRunTickNativeCursor>();built(cx,&mut self.cursor,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes+size_of::<Option<Box<ToolRunTickNativeCursor>>>(),capacity_bytes:bytes,release_bytes:0,depth:1},||Ok((Box::write(Box::new_uninit(),ToolRunTickNativeCursor::empty()),RetainedCloneProgress{copied_items:1,copied_bytes:bytes+size_of::<Option<Box<ToolRunTickNativeCursor>>>(),retained_capacity_bytes:bytes,..Default::default()})))?;return Ok(false)}let d=ToolRunTickNativeCursor::birth_demands();if funds(cx,d){if let Some(p)=self.cursor.as_mut().ok_or_else(||invalid("tool-run tick cursor is absent"))?.admit(source,cx.retained_grant())?{cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=1;}}},
   1=>{
    if let Some(outcome)=self.outcome.take(){
     cx.settle_receiving_scope()?;let progress=outcome.retained_progress();
     if outcome.source_refusal.is_some()||outcome.call.is_err(){self.stepped=None;return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original tick receiving scope refused").with_retained_progress(progress).into())}
     let step=self.stepped.take().ok_or_else(||invalid("original tick receiving lost its step result"))?.map_err(|primary|semio_framework_job::OriginalTurnFailure{effects:primary.progress,primary:primary.into(),receipt_refusal:None,context_refusal:None,work_refusal:None})?;
     cx.consume_fuel(step.progress().copied_items as u64);return Ok(false)
    }
    if self.cursor.as_ref().ok_or_else(||invalid("tool-run tick cursor is absent"))?.is_complete(){self.phase=2;}
    else{let cursor=self.cursor.as_mut().ok_or_else(||invalid("tool-run tick cursor is absent"))?;let stepped=&mut self.stepped;cx.with_native_receiving(&mut self.outcome,|native,_|{*stepped=Some(cursor.advance(source,native));Ok(())})?;}
   },
   2=>{if let Some((tick,p))=self.cursor.as_mut().ok_or_else(||invalid("tool-run tick cursor is absent"))?.take_output(cx.retained_grant()){owned.tick=Some(tick);cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=3;}},
   3=>{
    let tick=owned.tick.as_ref().ok_or_else(||invalid("original tick is absent"))?;if tick.identity!=identity{return Err(invalid("original tick differs from the current complete run identity").into())}
    if !tick.trace.is_empty(){return Err(invalid("original trace ticks require their retained trace owner").into())}
    let Some(end)=domain(owned.tick.as_mut().ok_or_else(||invalid("original tick is absent"))?,cx)?else{return Ok(false)};
    let d=ToolRunPresentationSlot::tick_admission_demands();if !funds(cx,d){return Ok(false)}
    let mut tick=owned.tick.take();if tick.is_none(){return Err(invalid("original tick is absent").into())}
    match cx.with_original_effect(cx.retained_grant(),|_,effect|tick.take().map(|held|presentation.begin_tick(held,end,state,0.0,0,effect)))?{
     None=>{owned.tick=tick;return Ok(false)},
     Some((None,_))=>return Err(invalid("original tick vanished inside its effect").into()),
     Some((Some(Ok(((),p))),performed))=>{cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));cx.consume_retained(p)?;self.phase=4;},
     Some((Some(Err((error,held))),performed))=>{cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));owned.tick=Some(held);return Err(error.into())}
    }
   },
   4=>{self.phase=5;},
   5=>{let cursor=self.cursor.as_ref().ok_or_else(||invalid("tool-run tick cursor is absent"))?;if cursor.terminal_is_empty(){let d=cursor_release_demands();let step=retired(cx,d,|_|{drop(self.cursor.take());Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;if step.progress().copied_items!=0{self.phase=0;return Ok(true)}return Ok(false)}let d=cursor.close_demands(cx.retained_grant().maximum_copy_bytes)?;let cursor=self.cursor.as_mut().ok_or_else(||invalid("tool-run tick cursor is absent"))?;semio_framework_job::close_original_effect_step(cx,d,|effect|{let result=cursor.close_step(effect).map_err(Into::into);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()})})?;},
   _=>return Err(invalid("original tick receiving phase is invalid").into()),
  }Ok(false)
 }
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.outcome.is_some()||self.stepped.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::OriginalNativeReceivingOutcome>>()+size_of::<Option<Result<RetainedCloneStep,semio_framework_value::native_decoding::OriginalReceivingPrimary>>>(),depth:1,..Default::default()})}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(cursor_release_demands())}else{cursor.close_demands(body)}}
  if let Some(close)=self.close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Owned>>>(),depth:1,..Default::default()})}else{close.selected_demand(body)}}
  if self.owned.presentation.is_some()||self.owned.tick.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Owned>()+size_of::<ControlledRetirement<Owned>>(),depth:1,..Default::default()})}Ok(Default::default())
 }
 /// 🪙️ Runs one quoted close action inside the job-side original effect; an unadmitted quote moves nothing and charges nothing.
 pub fn close_with(&mut self,cx:&mut StepContext<'_>,demand:RetirementDemand)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
  if !funds(cx,demand){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
  if self.outcome.is_some(){cx.settle_receiving_scope()?;}
  if self.outcome.is_none()&&self.cursor.as_ref().is_some_and(|cursor|cursor.frame_pending()){let cursor=self.cursor.as_mut().ok_or_else(||invalid("tool-run tick cursor is absent"))?;let mut settled=None;cx.with_native_receiving(&mut self.outcome,|native,_|{settled=Some(cursor.settle_frame(native));Ok(())})?;if let Some(Err(error))=settled{return Err(error.into())}return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let Some((result,performed))=cx.with_original_effect(cx.retained_grant(),|_,effect|self.close_effect_step(effect))?else{return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})};
  let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));cx.consume_retained(progress)?;
  Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:result?.progress()})
 }
 pub fn close_effect_step(&mut self,effect:&mut semio_framework_value::retained_clone::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let d=self.next_close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=true;
  if self.outcome.is_some()||self.stepped.is_some(){if effect.remaining_work_units()?==0||!effect.performed(1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}let message=self.outcome.take().filter(|outcome|outcome.source_refusal.is_some()||outcome.call.is_err()).map(|outcome|outcome.retained_progress());let failed=matches!(self.stepped.take(),Some(Err(_)));let progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if failed||message.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original tick receiving was refused before its cancelled close").with_retained_progress(progress))}return Ok(RetainedCloneStep::Progress(progress))}
  if let Some(cursor)=self.cursor.as_mut(){if !cursor.terminal_is_empty(){return cursor.close_step(effect)}drop(self.cursor.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  if let Some(close)=self.close.as_mut(){if !close.terminal_is_empty(){{let body=grant.maximum_copy_bytes;if self.close_reply.is_none(){let reply=semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut self.close_reply)?;let done=if self.close_pending{close.recover_step_into(body,effect,reply)?}else{close.step_into(body,effect,reply)?};drop(done);}return match self.close_reply.take(){None=>{self.close_pending=true;Ok(RetainedCloneStep::Progress(Default::default()))},Some(result)=>{self.close_pending=false;result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}}}self.close=None;}
  else if self.owned.presentation.is_some()||self.owned.tick.is_some(){let owned=std::mem::replace(&mut*self.owned,Owned{presentation:None,tick:None});match ControlledRetirement::new(owned){Ok(close)=>self.close=Some(close),Err((error,owned))=>{*self.owned=owned;return Err(error)}}}
  else{return Ok(RetainedCloneStep::Complete(Default::default()))}
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 pub fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.outcome.is_none()&&self.stepped.is_none()&&self.close.is_none()&&self.adv_reply.is_none()&&self.close_reply.is_none()&&!self.adv_pending&&!self.close_pending&&self.owned.presentation.is_none()&&self.owned.tick.is_none()}
}
fn advance_presentation(presentation:&mut ToolRunPresentationSlot,reply:&mut Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,pending:&mut bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 let body=cx.retained_grant().maximum_copy_bytes;
 if !*pending{let d=presentation.next_demands(body)?;if !funds(cx,d){return Ok(())}}
 let fuel=(cx.fuel_remaining().saturating_sub(1)/3)as usize;
 let grant=ToolRunEntityEditGrant{retirement:cx.retained_grant(),comparison:semio_framework_value::retained_clone::ordered_map::BoundedOrdGrant{maximum_items:fuel.min(1),maximum_bytes:fuel.min(64)},maximum_moved_items:fuel.min(1),maximum_moved_bytes:cx.retained_grant().maximum_copy_bytes,maximum_capacity_bytes:cx.retained_grant().maximum_capacity_bytes};
 let recover=*pending;
 let(outcome,performed)=match cx.with_original_effect(grant.retirement,|_,effect|{
  if reply.is_none(){let slot=semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut *reply)?;let done=if recover{presentation.recover_advance_into(body,grant,effect,slot)?}else{presentation.advance_into(body,grant,effect,slot)?};drop(done);}
  Ok::<_,ValueError>(reply.take())
 })?{Some((outcome,performed))=>(outcome,u64::try_from(performed).unwrap_or(u64::MAX)),None=>return Ok(())};
 let(progress,failure)=match outcome{
  Ok(Some(Ok(step)))=>{*pending=false;(step.progress(),None)},
  Ok(Some(Err(error)))=>{*pending=false;(error.retained_progress(),Some(error.into_value_error()))},
  Ok(None)=>{*pending=true;(Default::default(),None)},
  Err(error)=>(error.retained_progress(),Some(error)),
 };
 settle_presentation_turn(cx,progress,performed)?;
 if let Some(error)=failure{return Err(error.into())}
 Ok(())
}
/// ⛽️ Settles one presentation advance: the physical receipt and exactly the work the effect performed, the single fuel charge of the turn.
fn settle_presentation_turn(cx:&mut StepContext<'_>,progress:RetainedCloneProgress,performed:u64)->Result<(),OriginalTurnFailure>{cx.consume_retained(progress)?;cx.consume_fuel(performed);Ok(())}
#[cfg(test)]
mod settle_presentation_turn_tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 /// 🧪️ The fuel a presentation turn loses equals the effect receipt, whatever the compared and moved sizes of its progress were.
 #[test]
 fn presentation_turn_debits_exactly_the_performed_work(){
  let stock=RetainedCloneGrant{maximum_items:1<<10,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(1),Generation(1),stock,||Some(1000),1500,10_000);
  for performed in [0u64,1,3,7]{caller.turn(stock,|cx|{let before=cx.fuel_remaining();settle_presentation_turn(cx,Default::default(),performed)?;assert_eq!(before-cx.fuel_remaining(),performed);Ok(())}).unwrap();}
  caller.retire(stock);
 }
}
impl Drop for OriginalToolRunNativeState{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original run presentation requires funded terminal close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}}}
impl RetireOwned for OriginalToolRunNativeState{type RetirementCursor=Self;fn retirement_seed(self)->Self{self}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for OriginalToolRunNativeState{fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_close_demands(body)}fn close_step(&mut self,effect:&mut semio_framework_value::retained_clone::RetainedAllocationEffect<'_>)->RetirementStep{match self.close_effect_step(effect){Ok(step)=>RetirementStep::Progress(step.progress(),effect.performed_work()),Err(error)=>RetirementStep::Failure(error.into(),effect.performed_work())}}fn terminal_is_empty(&self)->bool{OriginalToolRunNativeState::terminal_is_empty(self)}fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}}

/// 📄️ The descriptor keeps these exact producer pages alive until original receiving and publication finish.
pub struct OriginalPreviewPages<'a>(pub &'a semio_framework_job_payload::RetainedJobPayload);
impl OriginalTickWireSource for OriginalPreviewPages<'_>{
 fn identity(&self)->usize{self.0.original_wire_identity().0}
 fn byte_len(&self)->usize{self.0.len()}
 fn page(&self,index:usize)->Option<&[u8]>{self.0.page(index)}
}
