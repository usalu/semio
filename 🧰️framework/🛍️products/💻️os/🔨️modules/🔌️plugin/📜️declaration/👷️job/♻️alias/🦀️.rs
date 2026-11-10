/// 🫴️ The original shared alias remains parked until its final dynamic payload is already empty.
pub struct ArtifactInstanceAliasRetirement{original:ArtifactInstanceOperationOwnerAliasRetirement}
impl ArtifactInstanceAliasRetirement{
 pub fn new(source:ArtifactInstanceOperationOwnerHandle)->Self{Self{original:ArtifactInstanceOperationOwnerAliasRetirement::new(source)}}
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.original.alias_return_demands(copy)}
}
impl semio_framework_value::retirement::RetirementCursor for ArtifactInstanceAliasRetirement{
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 semio_framework_value::native_retirement_reply!();
 fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl ArtifactInstanceAliasRetirement{
 pub fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{
  use semio_framework_value::retirement::{RetirementStep,RetirementCursor as _};
  let grant=effect.grant();
  if self.terminal_is_empty(){return RetirementStep::Complete(0)}
  let demand=match self.demands(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return RetirementStep::BudgetExhausted}
  let work=match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
  match self.original.alias_return_step(body,effect){Ok(RetainedCloneStep::Complete(progress))if progress==Default::default()=>RetirementStep::Complete(work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,work),Err(error)=>RetirementStep::Failure(error.into(),work)}
 }
}
impl semio_framework_value::retirement::RetireOwned for ArtifactInstanceOperationOwnerHandle{
 type RetirementCursor=ArtifactInstanceAliasRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ArtifactInstanceAliasRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ArtifactInstanceAliasRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for ArtifactInstanceAliasRetirement{fn drop(&mut self){assert!(std::thread::panicking()||semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self),"original instance alias abandoned dynamic custody")}}
