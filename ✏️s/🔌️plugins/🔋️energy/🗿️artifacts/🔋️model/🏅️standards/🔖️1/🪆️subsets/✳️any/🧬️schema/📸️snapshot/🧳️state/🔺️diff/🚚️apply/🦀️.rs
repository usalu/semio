//! 🚚️ Genuine Zone sparse preparation validates original identity and physics before central writes.
use super::{StateZonePatch,StateZone,EntityId};
use protocol::{ApplyCapability,MutationMessage,mutation::retained::{RetainedDiffFragment,RetainedDiffApply,RetainedDiffPreflightAdvance,retained_mutation_demand_fits,patch::RetainedRowPatchApply}};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneRef, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::size_of;
/// 🚦️ Zone authored fields validate before shared sparse staging and central capability exchange.
pub struct StateZoneApply{fields:RetainedRowPatchApply<StateZone,StateZonePatch>,phase:u8}
impl RetainedDiffFragment<StateZone> for StateZonePatch{type FragmentApply=StateZoneApply;}
impl RetainedDiffApply<StateZone,StateZonePatch> for StateZoneApply{
 fn new()->Self{Self{fields:Default::default(),phase:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,_:&StateZone,body:usize)->Result<RetirementDemand,ValueError>{if self.phase==0{return Ok(RetirementDemand{copy_bytes:2*size_of::<EntityId>()+size_of::<Option<f64>>()+size_of::<Option<u32>>()+size_of::<u8>(),depth:1,..Default::default()});}if self.phase==1{return zone_nested(self.fields.stage_demand(d,s,body)?);}Err(zone_invalid())}
 fn preflight(&mut self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,post:&StateZone,body:usize,g:RetainedCloneGrant,_:&mut NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{
  let demand=<Self as RetainedDiffApply<StateZone,StateZonePatch>>::preflight_demand(self,d,s,post,body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}
  if self.phase==0{let admitted=post.id==s.get().id&&d.get().volume_m3.is_none_or(|v|v.is_finite()&&v>0.0)&&d.get().multiplier.is_none_or(|v|v>0);self.phase=if admitted{1}else{4};let p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};return Ok(if admitted{RetainedDiffPreflightAdvance::Progress(p)}else{RetainedDiffPreflightAdvance::Ready(p,false)});}
  let step=self.fields.stage(d,s,body,zone_child(g))?;let p=zone_receipt(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;Ok(RetainedDiffPreflightAdvance::Ready(p,true))}else{Ok(RetainedDiffPreflightAdvance::Progress(p))}
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,StateZonePatch>,_:RetainedCloneRef<'_,StateZone>,_:&StateZone,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=2{return Err(zone_invalid());}zone_nested(self.fields.exchange_demand()?)}
 fn commit(&mut self,d:RetainedCloneRef<'_,StateZonePatch>,s:RetainedCloneRef<'_,StateZone>,post:&mut StateZone,g:RetainedCloneGrant,c:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let demand=<Self as RetainedDiffApply<StateZone,StateZonePatch>>::commit_demand(self,d,s,post,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=self.fields.exchange(post,c,zone_child(g))?;let p=zone_receipt(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3;Ok(RetainedCloneStep::Complete(p))}else{Ok(RetainedCloneStep::Progress(p))}}
 fn begin_close(&mut self){self.fields.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.fields.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.fields.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.fields.terminal_is_empty()}
}
fn zone_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Energy Zone sparse apply lost its validated phase")}
fn zone_nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(1).ok_or_else(zone_invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(zone_invalid)?;Ok(d)}
fn zone_child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-1,maximum_depth:g.maximum_depth-1,..g}}
fn zone_receipt(mut p:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(1).ok_or_else(zone_invalid)?;Ok(p)}