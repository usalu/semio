//! 🌱️ Guest UI backing is born and registered only inside its admitted original actor Source.
use super::*;
use std::mem::size_of;
use semio_framework_ui_contract::{UiArenaOpening,UiArenaBacking,UiArenaRegistration,ui_value_arena_registry};
pub(super) struct OriginalGuestUiArena{opening:ManuallyDrop<Option<UiArenaOpening>>,backing:ManuallyDrop<Option<UiArenaBacking>>,registration:ManuallyDrop<Option<UiArenaRegistration>>,closing:bool}
impl OriginalGuestUiArena{
 pub(super) fn empty()->Self{Self{opening:ManuallyDrop::new(None),backing:ManuallyDrop::new(None),registration:ManuallyDrop::new(None),closing:false}}
 pub(super) fn ready(&self)->bool{!self.closing&&self.registration.as_ref().is_some_and(UiArenaRegistration::registered)}
 fn metadata(cx:&mut StepContext<'_>,copy:usize,retiring:bool)->Result<bool,OriginalTurnFailure>{let g=cx.retained_grant();if (if retiring{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||g.maximum_items==0||g.maximum_copy_bytes<copy||g.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub(super) fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.closing{return Err(refusal("closing guest UI arena cannot restart admission").into())}if self.ready(){return Ok(true)}
  if let Some(opening)=self.opening.as_mut(){if opening.ready(){let turn=opening.take(cx.retained_grant());let result=match turn.result{Ok(Some((backing,progress)))=>{*self.backing=Some(backing);Ok(RetainedCloneStep::Progress(progress))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;return Ok(false)}if self.backing.is_some()&&opening.terminal_is_empty(){if Self::metadata(cx,size_of::<UiArenaOpening>(),false)?{self.opening.take();}return Ok(false)}let turn=opening.advance(cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}
  if self.backing.is_some(){let turn=UiArenaRegistration::admit(ui_value_arena_registry(),&mut*self.backing,&mut*self.registration,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(self.ready())}
  if Self::metadata(cx,UiArenaOpening::constructor_demand().copy_bytes,false)?{*self.opening=Some(UiArenaOpening::new());}Ok(false)
 }
 pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if !self.closing{if Self::metadata(cx,size_of::<bool>(),true)?{self.closing=true;}return Ok(false)}
  if let Some(registration)=self.registration.as_mut(){if registration.terminal_is_empty(){if Self::metadata(cx,size_of::<UiArenaRegistration>(),true)?{self.registration.take();}return Ok(false)}let turn=registration.close_step(cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}
  if let Some(backing)=self.backing.as_mut(){if backing.terminal_is_empty(){if Self::metadata(cx,size_of::<UiArenaBacking>(),true)?{self.backing.take();}return Ok(false)}let turn=backing.close_step(cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}
  if let Some(opening)=self.opening.as_mut(){if opening.terminal_is_empty(){if Self::metadata(cx,size_of::<UiArenaOpening>(),true)?{self.opening.take();}return Ok(false)}let turn=opening.close_step(cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}Ok(true)
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.opening.is_none()&&self.backing.is_none()&&self.registration.is_none()}
}
impl Drop for OriginalGuestUiArena{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"guest UI lifetime requires original registered backing retirement")}}
