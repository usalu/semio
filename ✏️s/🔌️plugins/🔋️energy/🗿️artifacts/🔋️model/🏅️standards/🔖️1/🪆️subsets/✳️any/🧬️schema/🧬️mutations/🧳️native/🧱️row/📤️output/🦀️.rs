//! 📤️ A native branch output adopts the unique prepared branch before publishing its original sparse diff.
use super::*;
/// 🧩️ Branch paths author direct or enclosing retained output without synchronous nested mutation.
pub trait BranchOutput<B:Send+Sync+'static>:Cursor+Send+'static{
 fn quote(&self,value:&Option<SealedShared<B>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,value:&mut Option<SealedShared<B>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn ready(&self)->bool;
 fn take_quote(&self)->Result<Quote,ValueError>;
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
}
/// 📦️ Direct root branches move only their paid sealed header into the authored diff field.
pub struct DirectBranchOutput<B:Send+Sync+'static>{convert:fn(SealedShared<B>)->EnergyModelDiff,decision:Held<EnergyModelDiff>,phase:u8,closing:bool,flight:Flight}
impl<B:Send+Sync+'static> DirectBranchOutput<B>{
 pub fn new(convert:fn(SealedShared<B>)->EnergyModelDiff)->Self{Self{convert,decision:Default::default(),phase:0,closing:false,flight:Default::default()}}
 custody_impl!(decision);
}
impl<B:Send+Sync+'static> BranchOutput<B> for DirectBranchOutput<B>{
 fn quote(&self,value:&Option<SealedShared<B>>,_:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=0||value.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<SealedShared<B>>>()+size_of::<EnergyModelDiff>()+2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn advance_reply<'r>(&mut self,value:&mut Option<SealedShared<B>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if recover{return Err(invalid().into())}
   let(d,u)=self.quote(value,o,b,m)?;
   if u>m||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,effect.grant()){return Ok(RetainedCloneStep::Progress(Default::default()))}
   self.decision.set((self.convert)(value.take().ok_or_else(invalid)?));self.phase=1;work=u128::from(u);Ok(RetainedCloneStep::Complete(receipt(d)))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.phase==1}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=2;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
}
crate::custody::std_cursor!([B:Send+Sync+'static] DirectBranchOutput<B>,[],|s|s.begin_close());

#[path="🪆️nested/🦀️.rs"]mod nested_output;pub use nested_output::{NestedBranchPath,NestedBranchOutput};

#[path="📅️schedule/🦀️.rs"]mod schedule;pub use schedule::ScheduleBranchOutput;

#[path="🪪️identity/🦀️.rs"]mod identity;pub use identity::{SiteBranchOutput,GroundBranchOutput,RunPeriodBranchOutput};
