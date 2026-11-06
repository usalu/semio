//! 🌳️ Original prepared branch headers stage before central unpublished adoption and keep every inverse alias.
use super::{EnergyModelState,EnergyModelStatePatch};
use protocol::{ApplyCapability,MutationMessage,RetainedDiffFragment,RetainedDiffApply,RetainedDiffPreflightAdvance,retained_mutation_demand_fits,RetainedRowPatchApply};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

/// 🌳️ A native sparse path stage supplies sealed branch candidates; only the central capability publishes their headers.
pub struct PreparedEnergyBranchesApply{fields:RetainedRowPatchApply<EnergyModelState,EnergyModelStatePatch>,phase:u8}
impl RetainedDiffFragment<EnergyModelState> for EnergyModelStatePatch{type FragmentApply=PreparedEnergyBranchesApply;}
impl RetainedDiffApply<EnergyModelState,EnergyModelStatePatch> for PreparedEnergyBranchesApply{
 fn new()->Self{Self{fields:Default::default(),phase:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,EnergyModelStatePatch>,s:RetainedCloneRef<'_,EnergyModelState>,_:RetainedCloneRef<'_,EnergyModelState>,body:usize)->Result<RetirementDemand,ValueError>{if self.phase!=0{return Err(invalid());}nested(self.fields.stage_demand(d,s,body.saturating_sub(1))?)}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,EnergyModelStatePatch>,s:RetainedCloneRef<'_,EnergyModelState>,p:RetainedCloneRef<'_,EnergyModelState>,body:usize)->Result<u64,ValueError>{self.preflight_demand(d,s,p,body)?;self.fields.stage_work_demand(d,s,body.saturating_sub(1))}
 fn preflight(&mut self,d:RetainedCloneRef<'_,EnergyModelStatePatch>,s:RetainedCloneRef<'_,EnergyModelState>,p:RetainedCloneRef<'_,EnergyModelState>,body:usize,g:RetainedCloneGrant,_:&mut NativeDecodeControl<'_>)->semio_framework_value::RetainedWorkOutcome<RetainedDiffPreflightAdvance,semio_framework_value::OriginalRetirementFailure>{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{let q=self.preflight_demand(d,s,p,body)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}let turn=self.fields.stage(d,s,body.saturating_sub(1),child(g));work=turn.work;let step=turn.result.map_err(|error|if work==0{error}else{let mut progress=error.retained_progress();progress.copied_bytes=progress.copied_bytes.saturating_add(1);error.with_retained_progress(progress)})?;let mut progress=step.progress();progress.copied_bytes+=1;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1;Ok(RetainedDiffPreflightAdvance::Ready(progress,true))}else{Ok(RetainedDiffPreflightAdvance::Progress(progress))}})();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,EnergyModelStatePatch>,_:RetainedCloneRef<'_,EnergyModelState>,_:&EnergyModelState,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=1{return Err(invalid());}nested(self.fields.exchange_demand()?)}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,EnergyModelStatePatch>,s:RetainedCloneRef<'_,EnergyModelState>,p:&EnergyModelState,body:usize)->Result<u64,ValueError>{self.commit_demand(d,s,p,body)?;self.fields.exchange_work_demand()}
 fn commit(&mut self,d:RetainedCloneRef<'_,EnergyModelStatePatch>,s:RetainedCloneRef<'_,EnergyModelState>,p:&mut EnergyModelState,g:RetainedCloneGrant,c:ApplyCapability)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{let q=self.commit_demand(d,s,p,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let turn=self.fields.exchange(p,c,child(g));work=turn.work;let step=turn.result.map_err(|error|if work==0{error}else{let mut progress=error.retained_progress();progress.copied_bytes=progress.copied_bytes.saturating_add(1);error.with_retained_progress(progress)})?;let mut progress=step.progress();progress.copied_bytes+=1;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;Ok(RetainedCloneStep::Complete(progress))}else{Ok(RetainedCloneStep::Progress(progress))}})();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn begin_close(&mut self){self.fields.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.fields.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.fields.close_work_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{self.fields.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.fields.terminal_is_empty()}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"prepared Energy branch adoption lost its original staged phase")}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(1).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-1,maximum_depth:g.maximum_depth-1,..g}}
