//! 🎟️ The admitting Original Source owns every arena opening, published registration and physical backing.
use super::*;
use semio_framework_job::{OriginalNativeLease,OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedWorkOutcome,RetirementDemand,retained_clone::retained_clone_grant_admits};

/// 🏡️ This allocation-free input becomes a native Source recipient before its first opening exists.
pub struct UiArenaLifetime{registry:&'static UiArenaRegistry,opening:Option<UiArenaOpening>,backing:Option<UiArenaBacking>,registration:Option<UiArenaRegistration>,closing:bool}
impl UiArenaLifetime{
 pub const fn new(registry:&'static UiArenaRegistry)->Self{Self{registry,opening:None,backing:None,registration:None,closing:false}}
 pub fn admit_source(input:&mut Option<Self>,output:&mut Option<OriginalNativeLease<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.admit_original_native(input,output)}
 pub fn opening_reserve()->Result<(usize,RetirementDemand,u64),semio_framework_value::ValueError>{let(mut items,mut d,mut work)=UiArenaOpening::opening_reserve()?;d.copy_bytes+=UiArenaRegistration::constructor_demand().copy_bytes+size_of::<Option<UiArenaOpening>>()-size_of::<UiArenaOpening>()+size_of::<Option<UiArenaBacking>>();items+=2;work+=2;Ok((items,d,work))}
 pub fn closure_reserve()->Result<(usize,RetirementDemand,u64),semio_framework_value::ValueError>{let(mut items,mut d,mut work)=UiArenaRegistration::closure_reserve()?;d.copy_bytes+=size_of::<bool>()+size_of::<Option<UiArenaRegistration>>()+size_of::<Option<UiArenaBacking>>()+size_of::<Option<UiArenaOpening>>();items+=4;work+=4;Ok((items,d,work))}
 pub fn ready(&self)->bool{!self.closing&&self.registration.as_ref().is_some_and(UiArenaRegistration::registered)}
 pub fn advance_on_source(lease:&mut OriginalNativeLease<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut ready=false;lease.with_native(cx,|home,cx|{home.advance(cx)?;ready=home.ready();Ok(())})?;Ok(ready)}
 /// 🛡️ Only the original lifetime owner can recover its poisoned registry borrow.
 pub fn recover_on_source(lease:&mut OriginalNativeLease<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut recovered=false;lease.with_native(cx,|home,cx|{let owner=home.registration.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("UI arena recovery requires its registered owner"))?;if !fits(cx,UiArenaRegistration::recovery_demand(),false){return Ok(())}let turn=owner.recover_borrow_step(cx.retained_grant());recovered=matches!(&turn.result,Ok(semio_framework_value::RetainedCloneStep::Complete(_)));settle(turn,cx)})?;Ok(recovered)}
 fn advance(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"UI arena lifetime is closing").into())}if self.ready()||cx.should_yield(){return Ok(())}
  if self.opening.is_none(){let d=RetirementDemand{copy_bytes:size_of::<Option<UiArenaOpening>>(),depth:1,..Default::default()};if !fits(cx,d,false){return Ok(())}cx.consume_retained(progress(d))?;cx.consume_fuel(1);self.opening=Some(UiArenaOpening::new());return Ok(())}
  let opening=self.opening.as_mut().expect("admitted opening remains in the native Source recipient");
  if self.backing.is_none(){if !opening.ready(){if !fits(cx,opening.demands()?,false){return Ok(())}return settle(opening.advance(cx.retained_grant()),cx)}let d=RetirementDemand{copy_bytes:UiArenaOpening::take_demand().copy_bytes+size_of::<Option<UiArenaBacking>>(),depth:1,..Default::default()};if !fits(cx,d,false)||cx.retained_grant().maximum_items<2||cx.fuel_remaining()<2{return Ok(())}let turn=opening.take(cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok(Some((backing,p)))=>{self.backing=Some(backing);cx.consume_retained(p.checked_add(progress(RetirementDemand{copy_bytes:size_of::<Option<UiArenaBacking>>(),depth:1,..Default::default()}))?)?;cx.consume_fuel(1);},Ok(None)=>{},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}return Ok(())}
  if !fits(cx,UiArenaRegistration::constructor_demand(),false){return Ok(())}settle(UiArenaRegistration::admit(self.registry,&mut self.backing,&mut self.registration,cx.retained_grant()),cx)
 }
}
fn fits(cx:&StepContext<'_>,d:RetirementDemand,retiring:bool)->bool{!(if retiring{cx.retirement_should_yield()}else{cx.should_yield()})&&cx.fuel_remaining()>0&&retained_clone_grant_admits(cx.retained_grant(),d)}
fn progress(d:RetirementDemand)->semio_framework_value::RetainedCloneProgress{semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn settle(turn:RetainedWorkOutcome,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.consume_fuel(turn.work);match turn.result{Ok(step)=>{cx.consume_retained(step.progress())?;Ok(())},Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}}
impl OriginalNativeRetirement for UiArenaLifetime{
 fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.closing{let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};if !fits(cx,d,true){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);self.closing=true;return Ok(false)}
  if let Some(registration)=self.registration.as_mut(){if registration.terminal_is_empty(){let d=RetirementDemand{copy_bytes:size_of::<Option<UiArenaRegistration>>(),depth:1,..Default::default()};if !fits(cx,d,true){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);self.registration.take();return Ok(false)}if !fits(cx,registration.close_demands()?,true){return Ok(false)}settle(registration.close_step(cx.retained_grant()),cx)?;return Ok(false)}
  if let Some(backing)=self.backing.as_mut(){if backing.terminal_is_empty(){let d=RetirementDemand{copy_bytes:size_of::<Option<UiArenaBacking>>(),depth:1,..Default::default()};if !fits(cx,d,true){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);self.backing.take();return Ok(false)}if !fits(cx,backing.close_demands()?,true){return Ok(false)}settle(backing.close_step(cx.retained_grant()),cx)?;return Ok(false)}
  if let Some(opening)=self.opening.as_mut(){if opening.terminal_is_empty(){let d=RetirementDemand{copy_bytes:size_of::<Option<UiArenaOpening>>(),depth:1,..Default::default()};if !fits(cx,d,true){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);self.opening.take();return Ok(false)}if !fits(cx,opening.close_demands()?,true){return Ok(false)}settle(opening.close_step(cx.retained_grant()),cx)?;return Ok(false)}
  Ok(true)
 }
 fn terminal_is_empty(&self)->bool{self.registration.is_none()&&self.backing.is_none()&&self.opening.is_none()}
}
impl Drop for UiArenaLifetime{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"UI arena lifetime requires its actual Source-funded registration and physical closure")}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
