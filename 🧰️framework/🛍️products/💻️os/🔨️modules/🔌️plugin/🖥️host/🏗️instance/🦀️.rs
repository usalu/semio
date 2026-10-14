//! 🏭️ A retained Host opening borrows the root's actual context and returns the callee's unchanged execution receipt.
use super::{CompiledHandle,GuestInstance,GuestInstanceState,GuestRuntimes,RuntimeActorId,owned_compiled};
use semio_framework_plugin_interpreter_runtime::{OwnedActorStartup,OwnedActorStartupCall,OwnedGuestInstance};
use semio_framework_actor::{OriginalGuestExecutionBudget,RetainedTurnInput};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
/// 🪢️ The actual kernel actor and partial Core stay owned until a funded final move or original cleanup.
pub struct GuestActorOpening{actor:RuntimeActorId,owner:Option<OwnedActorStartup>}
impl GuestActorOpening{
 /// 🔭️ Borrows the admitted Core memory while startup owns every physical resource.
 pub fn original_memory(&self,index:u32)->Option<&[u8]>{self.owner.as_ref()?.original_resources()?.memory(index)}
 pub fn actor(&self)->RuntimeActorId{self.actor}
 pub fn requires_execution(&self)->bool{self.owner.as_ref().is_some_and(OwnedActorStartup::requires_execution)}
 pub fn ready(&self)->bool{self.owner.as_ref().is_some_and(OwnedActorStartup::ready)}
 pub fn terminal_is_empty(&self)->bool{self.owner.is_none()}
}
impl GuestRuntimes{
 pub fn begin_original_actor_step(&self,compiled:&CompiledHandle,actor:RuntimeActorId,cx:&mut StepContext<'_>)->Result<Option<GuestActorOpening>,OriginalTurnFailure>{
  let Self::Owned(runtime)=self else{return Err(refused("Host startup requires an implemented original phased backend").into())};let compiled=owned_compiled(compiled).map_err(|_|refused("Host startup has no original Owned artifact"))?;if cx.is_cancelled()||!metadata(cx,size_of::<RuntimeActorId>())?{return Ok(None)};Ok(runtime.begin_actor_startup(compiled,cx)?.map(|owner|GuestActorOpening{actor,owner:Some(owner)}))
 }
 pub fn advance_original_actor_step(&self,owner:&mut GuestActorOpening,compiled:&CompiledHandle,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<OwnedActorStartupCall,OriginalTurnFailure>{
  let Self::Owned(_)=self else{return Err(refused("Host startup requires its same original phased backend").into())};owner.owner.as_mut().ok_or_else(||refused("Host startup continuation has no original owner"))?.advance_step(owned_compiled(compiled).map_err(|_|refused("Host startup has no original Owned artifact"))?,cx,execution)
 }
 pub fn take_original_actor_step(&self,owner:&mut Option<GuestActorOpening>,compiled:&CompiledHandle,input:RetainedTurnInput,cx:&mut StepContext<'_>)->Result<Option<GuestInstance>,OriginalTurnFailure>{
  let Self::Owned(runtime)=self else{return Err(refused("Host startup requires its same original phased backend").into())};let state=owner.as_mut().ok_or_else(||refused("Host startup transfer has no retained owner"))?;let compiled=owned_compiled(compiled).map_err(|_|refused("Host startup has no original Owned artifact"))?;state.owner.as_ref().ok_or_else(||refused("Host startup transfer has no retained callee"))?.validate_original_context(compiled,cx)?;if input.operation!=cx.operation().0||input.generation!=cx.generation().0||Some(input.epoch)!=cx.original_source_epoch()||input.grant!=cx.retained_grant(){return Err(refused("Host startup transfer differs from its actual root input").into())}if !state.ready(){return Ok(None)}let copy=size_of::<GuestInstance>().checked_sub(size_of::<OwnedGuestInstance>()).ok_or_else(||refused("Host startup wrapper extent is invalid"))?;if !metadata(cx,copy)?{return Ok(None)}let input=RetainedTurnInput{grant:cx.retained_grant(),..input};let Some(instance)=runtime.take_actor_startup_step(&mut state.owner,compiled,input,cx)?else{return Ok(None)};let actor=state.actor;owner.take();Ok(Some(GuestInstance{actor,state:GuestInstanceState::Owned(instance)}))
 }
 pub fn close_original_actor_step(&self,owner:&mut Option<GuestActorOpening>,compiled:&CompiledHandle,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(state)=owner.as_mut()else{return Ok(true)};let Self::Owned(_)=self else{return Err(refused("Host startup cleanup requires its same original phased backend").into())};if let Some(startup)=state.owner.as_mut(){if !startup.close_step(owned_compiled(compiled).map_err(|_|refused("Host startup has no original Owned artifact"))?,cx)?{return Ok(false)}}if !metadata(cx,size_of::<RuntimeActorId>())?{return Ok(false)}state.owner.take();owner.take();Ok(true)
 }
 pub fn close_original_instance_step(&self,instance:&mut GuestInstance,compiled:&CompiledHandle,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if matches!(instance.state,GuestInstanceState::Closed){return Ok(true)}let Self::Owned(runtime)=self else{return Err(refused("Host instance cleanup requires its original phased backend").into())};let compiled=owned_compiled(compiled).map_err(|_|refused("Host cleanup has no original Owned artifact"))?;let GuestInstanceState::Owned(state)=&instance.state else{return Err(refused("Host cleanup has no actual Owned instance").into())};runtime.validate_original_actor_retirement(state,compiled,cx)?;if !metadata(cx,size_of::<GuestInstanceState>())?{return Ok(false)}let GuestInstanceState::Owned(state)=std::mem::replace(&mut instance.state,GuestInstanceState::Closed)else{unreachable!()};let mut owner=Some(state);let result=runtime.close_original_actor_step(&mut owner,compiled,cx);if let Some(state)=owner{instance.state=GuestInstanceState::Owned(state);}result
 }
}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
fn metadata(cx:&mut StepContext<'_>,copy:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

