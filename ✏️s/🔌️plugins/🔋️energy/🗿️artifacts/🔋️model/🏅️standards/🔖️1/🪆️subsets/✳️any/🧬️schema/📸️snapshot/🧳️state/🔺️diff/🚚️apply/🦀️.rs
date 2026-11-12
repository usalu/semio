//! 🚚️ Genuine Zone sparse preparation validates original identity and physics before the shared central writes.
use super::{StateZonePatch,StateZone,EntityId};
use protocol::{ApplyCapability,MutationMessage,RetainedDiffFragment,RetainedDiffApply,RetainedDiffPreflightAdvance,retained_mutation_demand_fits,retained::{Done,Reply,RetainedPatchDiffApply,conclude}};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::size_of;

/// 🚦️ Zone authored fields validate once before the shared sparse staging and central capability exchange.
pub struct StateZoneApply{inner:RetainedPatchDiffApply<StateZone,StateZonePatch>,checked:bool,refused:bool}
impl RetainedDiffFragment<StateZone> for StateZonePatch{type FragmentApply=StateZoneApply;}
impl RetainedDiffApply<StateZone,StateZonePatch> for StateZoneApply{
 fn new()->Self{Self{inner:RetainedPatchDiffApply::new(),checked:false,refused:false}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:RetainedCloneRef<'_,StateZone>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.refused{return Err(zone_invalid())}if self.checked{return self.inner.preflight_demand(d,s,p,body,max)}Ok(RetirementDemand{copy_bytes:2*size_of::<EntityId>()+size_of::<Option<f64>>()+size_of::<Option<u32>>()+size_of::<u8>(),depth:1,..Default::default()})}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:RetainedCloneRef<'_,StateZone>,body:usize,max:u64)->Result<u64,ValueError>{if self.refused{return Err(zone_invalid())}if self.checked{return self.inner.preflight_work_demand(d,s,p,body,max)}Ok(1)}
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:RetainedCloneRef<'_,StateZone>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
  if self.refused{return Err(zone_invalid())}
  if self.checked{return self.inner.preflight_reply(d,s,p,body,max,effect,receiving,reply,recover)}
  let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedDiffPreflightAdvance,RetirementPollExit>=(||{
   if recover{return Err(zone_invalid().into())}
   let demand=self.preflight_demand(d,s,p,body,max)?;
   if max==0||effect.remaining_work_units()?<1||!retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}
   if !effect.performed(1)?{return Err(zone_invalid().into())}
   work=1;self.checked=true;
   let admitted=p.get().id==s.get().id&&d.get().volume_m3.is_none_or(|v|v.is_finite()&&v>0.0)&&d.get().multiplier.is_none_or(|v|v>0);
   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   if admitted{Ok(RetainedDiffPreflightAdvance::Progress(progress))}else{self.refused=true;Ok(RetainedDiffPreflightAdvance::Ready(progress,false))}
  })();
  conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:&StateZone,body:usize)->Result<RetirementDemand,ValueError>{self.inner.commit_demand(d,s,p,body)}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:&StateZone,body:usize)->Result<u64,ValueError>{self.inner.commit_work_demand(d,s,p,body)}
 fn commit(&mut self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,p:&mut StateZone,effect:&mut RetainedAllocationEffect<'_>,c:ApplyCapability)->OriginalRetirementOutcome{self.inner.commit(d,s,p,effect,c)}
 fn begin_close(&mut self){self.inner.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.inner.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.inner.close_work_demand(body)}
 fn requires_original_birth(&self)->bool{self.inner.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.inner.requires_original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.inner.close_original_birth_step(original,grant)}
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.inner.close_reply(original,body,effect,reply,recover)}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}
fn zone_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Energy Zone sparse apply lost its validated phase")}
