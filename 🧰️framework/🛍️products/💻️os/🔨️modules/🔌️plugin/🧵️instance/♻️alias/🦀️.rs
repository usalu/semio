/// 🔗️ Retains the original instance alias and every failed dynamic close source.
pub struct ArtifactInstanceAliasRetirement {
 original:ArtifactInstanceOperationOwnerAliasRetirement,
 failure:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Fault>>>,
}
impl ArtifactInstanceAliasRetirement {
 pub fn new(source:ArtifactInstanceOperationOwnerHandle)->Self{Self{original:ArtifactInstanceOperationOwnerAliasRetirement::new(source),failure:std::mem::ManuallyDrop::new(None)}}
 pub fn failure(&self)->Option<&Fault>{self.failure.as_ref().and_then(semio_framework_value::retirement::controlled::ControlledRetirement::original)}
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.failure.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original instance failure depth overflow"))?})}self.original.demands(copy)}
}
impl semio_framework_value::retirement::RetirementCursor for ArtifactInstanceAliasRetirement {
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;if self.terminal_is_empty(){return RetirementStep::Complete}let demand=match self.demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error)};if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return RetirementStep::BudgetExhausted}
  if let Some(owner)=self.failure.as_mut(){if owner.terminal_is_empty(){self.failure.take();return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}return match owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
  match self.original.step(grant){Ok(RetainedCloneStep::Complete(progress))if progress==Default::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),Err(fault)=>{let progress=fault.retained_progress();*self.failure=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(fault).unwrap_or_else(|_|unreachable!("original diagnostic fault defines whole closure")));RetirementStep::Failure(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance close retained its complete diagnostic fault").with_retained_progress(progress))}}
 }
 fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()&&self.failure.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl semio_framework_value::retirement::RetireOwned for ArtifactInstanceOperationOwnerHandle {
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(ArtifactInstanceAliasRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<ArtifactInstanceAliasRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for ArtifactInstanceAliasRetirement {fn drop(&mut self){assert!(std::thread::panicking()||semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self),"original instance alias abandoned its dynamic owner or diagnostic source");if self.failure.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.failure);}}}}
