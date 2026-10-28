//! 🛬️ Original preview pages, native tick custody and immutable presentation advance under one caller.
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_tool_run::{ToolRunIdentity,ToolRunState,ToolRunTick};
use semio_framework_tool_run_tick_wire::tick_receive::{ToolRunTickNativeCursor,OriginalTickWireSource};
use semio_framework_tool_run_presentation::{slot::ToolRunPresentationSlot,ToolRunCapturedView};
use semio_framework_tool_run_entities::ToolRunEntityEditGrant;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};

struct Owned{presentation:Option<ToolRunPresentationSlot>,tick:Option<ToolRunTick>}
semio_framework_value::artifact_retire_struct!(Owned{presentation,tick});
pub struct OriginalToolRunNativeState{owned:ManuallyDrop<Owned>,cursor:Option<Box<ToolRunTickNativeCursor>>,close:Option<ControlledRetirement<Owned>>,phase:u8,closing:bool,invalidated:bool}
fn cursor_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<ToolRunTickNativeCursor>>>(),release_bytes:size_of::<ToolRunTickNativeCursor>(),depth:1,..Default::default()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn funds(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d)}
impl OriginalToolRunNativeState{
 pub fn empty()->Self{Self{owned:ManuallyDrop::new(Owned{presentation:None,tick:None}),cursor:None,close:None,phase:0,closing:false,invalidated:false}}
 pub fn initialize(&mut self,tool_id:&str,identity:ToolRunIdentity,state:ToolRunState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.invalidated&&!self.terminal_is_empty(){return Ok(false)}
  if self.closing||self.invalidated{let d=RetirementDemand{copy_bytes:2*size_of::<bool>(),depth:1,..Default::default()};if !funds(cx,d){return Ok(false)}self.closing=false;self.invalidated=false;self.phase=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if self.owned.presentation.is_some(){return Ok(true)}
  let d=RetirementDemand{copy_bytes:tool_id.len()+size_of::<ToolRunPresentationSlot>(),capacity_bytes:tool_id.len(),depth:1,..Default::default()};if !funds(cx,d){return Ok(false)}
  let mut id=String::new();id.try_reserve_exact(tool_id.len()).map_err(|_|invalid("original run identifier backing refused"))?;id.push_str(tool_id);
  self.owned.presentation=Some(ToolRunPresentationSlot::new(id,identity,state));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 pub fn capture(&self,identity:ToolRunIdentity,state:ToolRunState,grant:RetainedCloneGrant)->Result<Option<(ToolRunCapturedView,RetainedCloneProgress)>,ValueError>{if self.invalidated{return Ok(None)}self.owned.presentation.as_ref().map_or(Ok(None),|presentation|presentation.capture_view(identity,state,grant))}
 pub fn borrowed_view(&self,identity:ToolRunIdentity,state:ToolRunState)->Option<semio_framework_tool_run_presentation::ToolRunView<'_>>{if self.invalidated{return None}self.owned.presentation.as_ref()?.borrowed_view(identity,state)}
 pub fn has_pending_tick(&self)->bool{self.phase!=0||self.cursor.is_some()||self.owned.tick.is_some()}
 pub fn request_close(&mut self){self.invalidated=true;}
 pub fn close_requested(&self)->bool{self.invalidated&&!self.terminal_is_empty()}
 pub fn receive(&mut self,source:&dyn OriginalTickWireSource,identity:ToolRunIdentity,state:ToolRunState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.receive_with_domain(source,identity,state,|tick,_|{if !tick.append_ops.is_empty(){return Err(invalid("a mutating original tick requires its installed domain receiver").into())}Ok(Some(tick.retract_to.unwrap_or(0)))},cx)}
 pub fn receive_with_domain(&mut self,source:&dyn OriginalTickWireSource,identity:ToolRunIdentity,state:ToolRunState,mut domain:impl FnMut(&mut ToolRunTick,&mut StepContext<'_>)->Result<Option<u32>,OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(invalid("original run presentation is closing").into())}
  let owned=&mut *self.owned;
  let presentation=owned.presentation.as_mut().ok_or_else(||invalid("original run presentation is absent"))?;
  if !presentation.ready(){advance_presentation(presentation,cx)?;return Ok(false)}
  match self.phase{
   0=>{if self.cursor.is_none(){let bytes=size_of::<ToolRunTickNativeCursor>();semio_framework_job::admit_original_construction(cx,&mut self.cursor,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes+size_of::<Option<Box<ToolRunTickNativeCursor>>>(),capacity_bytes:bytes,release_bytes:0,depth:1},|_|Ok((Box::write(Box::new_uninit(),ToolRunTickNativeCursor::empty()),RetainedCloneProgress{copied_items:1,copied_bytes:bytes+size_of::<Option<Box<ToolRunTickNativeCursor>>>(),retained_capacity_bytes:bytes,..Default::default()})))?;return Ok(false)}let d=ToolRunTickNativeCursor::birth_demands();if funds(cx,d){if let Some(p)=self.cursor.as_mut().unwrap().admit(source,cx.retained_grant())?{cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=1;}}},
   1=>{if self.cursor.as_ref().unwrap().is_complete(){self.phase=2;}else{let result=cx.with_native_receiving(|native|self.cursor.as_mut().unwrap().advance(source,native));let p=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};if let Some(grant)=cx.receiving_issued_grant(){cx.consume_receiving_retained(grant,p)?;}cx.consume_fuel(p.copied_items as u64);result?;}},
   2=>{if let Some((tick,p))=self.cursor.as_mut().unwrap().take_output(cx.retained_grant()){owned.tick=Some(tick);cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=3;}},
   3=>{
    let tick=owned.tick.as_ref().unwrap();if tick.identity!=identity{return Err(invalid("original tick differs from the current complete run identity").into())}
    if !tick.trace.is_empty(){return Err(invalid("original trace ticks require their retained trace owner").into())}
    let Some(end)=domain(owned.tick.as_mut().unwrap(),cx)?else{return Ok(false)};
    let d=ToolRunPresentationSlot::tick_admission_demands();if !funds(cx,d){return Ok(false)}
    let tick=owned.tick.take().unwrap();
    match presentation.begin_tick(tick,end,state,0.0,0,cx.retained_grant()){Ok(((),p))=>{cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=4;},Err((error,tick))=>{owned.tick=Some(tick);return Err(error.into())}}
   },
   4=>{self.phase=5;},
   5=>{let cursor=self.cursor.as_ref().unwrap();if cursor.terminal_is_empty(){let d=cursor_release_demands();let step=semio_framework_job::close_original_retained_step(cx,d,|_|{drop(self.cursor.take().unwrap());Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;if step.progress().copied_items!=0{self.phase=0;return Ok(true)}return Ok(false)}let d=cursor.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|grant|self.cursor.as_mut().unwrap().close_step(grant))?;},
   _=>return Err(invalid("original tick receiving phase is invalid").into()),
  }Ok(false)
 }
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(cursor_release_demands())}else{cursor.close_demands(body)}}
  if let Some(close)=self.close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Owned>>>(),depth:1,..Default::default()})}else{close.selected_demand(body)}}
  if self.owned.presentation.is_some()||self.owned.tick.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Owned>()+size_of::<ControlledRetirement<Owned>>(),depth:1,..Default::default()})}Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.next_close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=true;
  if let Some(cursor)=self.cursor.as_mut(){if !cursor.terminal_is_empty(){return cursor.close_step(grant)}drop(self.cursor.take().unwrap());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  if let Some(close)=self.close.as_mut(){if !close.terminal_is_empty(){return close.step(grant)}self.close=None;}
  else if self.owned.presentation.is_some()||self.owned.tick.is_some(){let owned=std::mem::replace(&mut*self.owned,Owned{presentation:None,tick:None});match ControlledRetirement::new(owned){Ok(close)=>self.close=Some(close),Err((error,owned))=>{*self.owned=owned;return Err(error)}}}
  else{return Ok(RetainedCloneStep::Complete(Default::default()))}
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 pub fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.close.is_none()&&self.owned.presentation.is_none()&&self.owned.tick.is_none()}
}
fn advance_presentation(presentation:&mut ToolRunPresentationSlot,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 let d=presentation.next_demands(cx.retained_grant().maximum_copy_bytes)?;if !funds(cx,d){return Ok(())}
 let fuel=(cx.fuel_remaining().saturating_sub(1)/3)as usize;
 let grant=ToolRunEntityEditGrant{retirement:cx.retained_grant(),comparison:semio_framework_value::retained_clone::ordered_map::BoundedOrdGrant{maximum_items:fuel.min(1),maximum_bytes:fuel.min(64)},maximum_moved_items:fuel.min(1),maximum_moved_bytes:cx.retained_grant().maximum_copy_bytes,maximum_capacity_bytes:cx.retained_grant().maximum_capacity_bytes};
 let result=presentation.advance(grant);let p=match &result{Ok(p)=>RetainedCloneProgress{retained_capacity_bytes:p.retirement.retained_capacity_bytes+p.retained_capacity_bytes,..p.retirement},Err(error)=>error.retained_progress()};cx.consume_retained(p)?;let work=result.as_ref().map_or(1,|p|1u64.saturating_add(p.comparison.compared_items as u64).saturating_add(p.comparison.compared_bytes as u64).saturating_add(p.moved_items as u64));cx.consume_fuel(work);result?;Ok(())
}
impl Drop for OriginalToolRunNativeState{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original run presentation requires funded terminal close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}}}
impl RetireOwned for OriginalToolRunNativeState{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for OriginalToolRunNativeState{fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_close_demands(body)}fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match OriginalToolRunNativeState::close_step(self,grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}fn terminal_is_empty(&self)->bool{OriginalToolRunNativeState::terminal_is_empty(self)}fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}}

/// 📄️ The descriptor keeps these exact producer pages alive until original receiving and publication finish.
pub struct OriginalPreviewPages<'a>(pub &'a semio_framework_job_payload::RetainedJobPayload);
impl OriginalTickWireSource for OriginalPreviewPages<'_>{
 fn identity(&self)->usize{self.0.original_wire_identity().0}
 fn byte_len(&self)->usize{self.0.len()}
 fn page(&self,index:usize)->Option<&[u8]>{self.0.page(index)}
}
