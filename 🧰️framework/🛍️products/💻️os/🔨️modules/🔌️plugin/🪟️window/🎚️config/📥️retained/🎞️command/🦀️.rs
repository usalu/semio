//! 🎞️ The original decoded command keeps native Window preparation and closure across mounted retries.
use crate::{WindowConfigOwnerRegistry,WindowConfigPack,WindowConfigPackSource,WindowConfigPackLoad,WindowConfigPackLoadStep,WindowConfigPackLoadPhase};
use super::{Construction,WindowConfigLoadAdmission};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalClosureFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,RetainedCloneStep,retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}

/// 🎟️ The command owns its exact framed bytes until the registered native owner takes custody.
#[derive(Default)]
pub struct WindowConfigCommandLoad{
 pub pack:ManuallyDrop<Option<WindowConfigPack>>,
 pub pack_close:ManuallyDrop<Option<ControlledRetirement<WindowConfigPack>>>,
 pub load:ManuallyDrop<Option<WindowConfigPackLoad>>,
 pub admission:WindowConfigLoadAdmission,
 pub construction:Construction,
 pub admitted:bool,
 pub source:Option<OriginalSourceBinding>,
 pub failure:ManuallyDrop<Option<OriginalTurnFailure>>,
 pub close_failure:ManuallyDrop<Option<OriginalTurnFailure>>,
 pub compound:ManuallyDrop<Option<OriginalClosureFailure>>,
 pub place:ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<OriginalClosureFailure>>>,
 pub closing:bool,
 pub done:bool,
}
impl WindowConfigCommandLoad{
 /// 🔄️ Advances only the original registered owner under the decoded command's unchanged caller.
 pub fn advance(&mut self,registry:&mut WindowConfigOwnerRegistry,entry:&mut protocol::WindowConfigPackEntry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.source.is_some_and(|source|!source.matches(cx)){return Err(invalid("Window command continuation received a foreign original caller"))}
  if self.done{return Ok(true)}
  if self.compound.is_some()||self.place.is_some(){if !self.place.as_ref().is_some_and(|place|place.ready()){if self.place.is_none(){*self.place=Some(Default::default());}self.place.as_mut().unwrap().prepare(cx)?;return Ok(false)}let place=self.place.as_mut().unwrap();if !place.pending(){let original=&mut *self.compound;place.with_body(cx,|_,slot|{*slot=original.take();Ok(())})?;}let admitted=semio_framework_job::admit_original_domain_failure(place,cx)?;if self.place.as_ref().is_some_and(|place|place.terminal_is_empty()){*self.place=None;}if let Some(failure)=admitted{*self.failure=Some(failure)}return Ok(false)}
  if self.close_failure.is_some(){retired(cx,semio_framework_value::RetirementDemand{copy_bytes:size_of::<OriginalClosureFailure>(),depth:1,..Default::default()},|_|{*self.compound=Some(OriginalClosureFailure{initial:self.failure.take().expect("original primary Window failure"),closure:self.close_failure.take().unwrap()});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<OriginalClosureFailure>(),..Default::default()}))})?;return Ok(false)}
  if cx.is_cancelled()&&!self.closing{let copied=size_of::<OriginalTurnFailure>()+2*size_of::<bool>();retired(cx,semio_framework_value::RetirementDemand{copy_bytes:copied,depth:1,..Default::default()},|_|{*self.failure=Some(invalid("original Window command was cancelled"));self.closing=true;if let Some(load)=self.load.as_mut(){load.request_cancel()}Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))})?;if !self.closing{return Ok(false)}}
  if self.closing{return self.close(registry,cx)}
  let result=self.advance_open(registry,entry,cx);
  match result{Ok(value)=>Ok(value),Err(failure)=>{*self.failure=Some(failure);self.closing=true;Ok(false)}}
 }
 fn advance_open(&mut self,registry:&mut WindowConfigOwnerRegistry,entry:&mut protocol::WindowConfigPackEntry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.construction.resume(){self.construction.settle(cx)?;return Ok(false)}
  if self.source.is_none(){let copied=size_of::<WindowConfigPack>()+size_of::<OriginalSourceBinding>();let binding=OriginalSourceBinding::borrow(cx);self.construction.run_unit(cx,semio_framework_value::RetirementDemand{copy_bytes:copied,depth:1,..Default::default()},|_,_|{*self.pack=Some(WindowConfigPack{window_id:std::mem::take(&mut entry.window_id),window_kind_id:std::mem::take(&mut entry.window_kind_id),source:WindowConfigPackSource::Framed(std::mem::take(&mut entry.envelope_pack))});self.source=Some(binding);Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))})?;return Ok(false)}
  if !self.admitted{self.admitted=registry.admit_retained_load(&mut self.pack,cx,&mut self.admission,&mut self.load)?;return Ok(false)}
  let phase=self.load.as_ref().unwrap().phase();
  if matches!(phase,WindowConfigPackLoadPhase::RetiringDisplacedStore|WindowConfigPackLoadPhase::Complete){self.construction.run_unit(cx,semio_framework_value::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()},|_,_|{self.closing=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))})?;return Ok(false)}
  let load=self.load.as_mut().unwrap();
  if load.phase()==WindowConfigPackLoadPhase::Ready{match registry.commit_retained_load_context(load,cx)?{WindowConfigPackLoadStep::Rejected(_)=>return Err(invalid("original Window publication was refused")),_=>{}}return Ok(false)}
  match registry.advance_retained_load_context(load,cx)?{WindowConfigPackLoadStep::Rejected(_)=>Err(invalid("original native Window load was refused")),WindowConfigPackLoadStep::Complete=>Ok(false),_=>Ok(false)}
 }
 fn close(&mut self,registry:&mut WindowConfigOwnerRegistry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.construction.close(cx)?{return Ok(false)}
  if let Some(load)=self.load.as_mut(){
   if !load.terminal_is_empty(){if let Err(failure)=registry.close_retained_load_context(load,cx){if self.failure.is_none(){*self.failure=Some(failure)}else{*self.close_failure=Some(failure)}}return Ok(false)}
   let copied=size_of::<WindowConfigPackLoad>();retired(cx,semio_framework_value::RetirementDemand{copy_bytes:copied,depth:1,..Default::default()},|_|{self.load.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))})?;return Ok(false)
  }
  if !self.admission.terminal_is_empty(){self.admission.close_step(cx)?;return Ok(false)}
  if self.pack.is_some()||self.pack_close.is_some(){let demand=semio_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.pack,&self.pack_close,cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_effect_step(cx,demand,|effect|semio_plugin_ownership::bounded_ownership::plugin_typed_owner_original(&mut self.pack,&mut self.pack_close,effect))?;return Ok(false)}
  let copied=size_of::<Option<OriginalSourceBinding>>()+size_of::<bool>()+size_of::<Option<OriginalTurnFailure>>();
  retired(cx,semio_framework_value::RetirementDemand{copy_bytes:copied,depth:1,..Default::default()},|_|{self.source=None;self.done=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))})?;
  if !self.done{return Ok(false)}if let Some(failure)=self.failure.take(){return Err(failure)}Ok(true)
 }
 /// 🕳️ Terminal custody contains no pending source, native reader, candidate or original failure.
 pub fn terminal_is_empty(&self)->bool{self.pack.is_none()&&self.pack_close.is_none()&&self.load.is_none()&&self.source.is_none()&&self.failure.is_none()&&self.close_failure.is_none()&&self.compound.is_none()&&self.place.is_none()&&self.admission.terminal_is_empty()&&self.construction.terminal_is_empty()}
}
impl Drop for WindowConfigCommandLoad{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Window command requires native closure before decoded custody ends")}}
fn invalid(message:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,message).into()}


