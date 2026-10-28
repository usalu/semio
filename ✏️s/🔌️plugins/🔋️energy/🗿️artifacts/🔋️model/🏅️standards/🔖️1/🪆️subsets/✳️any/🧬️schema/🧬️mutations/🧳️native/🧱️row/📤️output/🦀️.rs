//! 📤️ A native branch output adopts the unique prepared branch before publishing its original sparse diff.
use super::*;
/// 🧩️ Branch paths author direct or enclosing retained output without synchronous nested mutation.
pub trait BranchOutput<B:Send+Sync+'static>:CursorClose+Send+'static{
 fn demand(&self,value:&Option<SealedShared<B>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,value:&mut Option<SealedShared<B>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);

}
/// 📦️ Direct root branches move only their paid sealed header into the authored diff field.
pub struct DirectBranchOutput<B:Send+Sync+'static>{convert:fn(SealedShared<B>)->EnergyModelDiff,decision:ManuallyDrop<Option<EnergyModelDiff>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<B:Send+Sync+'static> DirectBranchOutput<B>{pub fn new(convert:fn(SealedShared<B>)->EnergyModelDiff)->Self{Self{convert,decision:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}}
impl<B:Send+Sync+'static> BranchOutput<B> for DirectBranchOutput<B>{
 fn demand(&self,value:&Option<SealedShared<B>>,_:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=0||value.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<SealedShared<B>>>()+size_of::<EnergyModelDiff>()+2*size_of::<Option<EnergyModelDiff>>()+1))}
 fn advance(&mut self,value:&mut Option<SealedShared<B>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(value,o,b)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}*self.decision=Some((self.convert)(value.take().ok_or_else(invalid)?));self.phase=1;Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn ready(&self)->bool{!self.closing&&self.phase==1}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<EnergyModelDiff>>()+1))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=2;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;}

}

impl<B:Send+Sync+'static> CursorClose for DirectBranchOutput<B>{
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}self.retirement.next_owner_demands::<EnergyModelDiff>(self.decision.is_some(),&None,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}Err(invalid())}
 fn terminal_is_empty(&self)->bool{self.closing&&self.decision.is_none()&&self.retirement.is_empty()}
}
impl<B:Send+Sync+'static> Drop for DirectBranchOutput<B>{fn drop(&mut self){assert!(std::thread::panicking()||<Self as CursorClose>::terminal_is_empty(self),"original direct branch output retains candidate custody");}}

#[path="🪆️nested/🦀️.rs"]mod nested_output;pub use nested_output::{NestedBranchPath,NestedBranchOutput};

#[path="📅️schedule/🦀️.rs"]mod schedule;pub use schedule::ScheduleBranchOutput;

#[path="🪪️identity/🦀️.rs"]mod identity;pub use identity::{SiteBranchOutput,GroundBranchOutput,RunPeriodBranchOutput};
