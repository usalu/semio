use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};

/// ♻️ Original child emission sources retain their native close authority.
struct OwnedChildEmitRetirement{original:ManuallyDrop<Option<OwnedChildEmit>>}
impl RetirementCursor for OwnedChildEmitRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  let Some(source)=self.original.as_mut()else{return RetirementStep::Complete};
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if source.terminal_is_empty(){drop(self.original.take());return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
  match source.close_granted(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|source.retirement_demands(0).map(|d|d.copy_bytes))}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.retirement_demands(copy).ok().map(|d|d.capacity_bytes))}
 fn next_close_byte_demand(&self)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.retirement_demands(0).ok().map(|d|d.release_bytes))}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|if source.terminal_is_empty(){Ok(1)}else{source.retirement_demands(0).map(|d|d.depth)})}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for OwnedChildEmitRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.original.is_none(),"original owned child emission remains retained until full close");}}
impl RetireOwned for OwnedChildEmit{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OwnedChildEmitRetirement{original:ManuallyDrop::new(Some(self))})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<OwnedChildEmitRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

struct ChildEmitPreparationRetirement{original:ManuallyDrop<Option<ChildEmitPreparation>>,fault:Option<ControlledRetirement<Fault>>}
impl ChildEmitPreparationRetirement{
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(fault)=self.fault.as_ref(){return Ok(RetirementDemand{copy_bytes:fault.next_copy_byte_demand()?,capacity_bytes:fault.next_capacity_byte_demand(copy)?,release_bytes:fault.next_release_byte_demand()?,depth:fault.next_depth_demand()?})}
  self.original.as_ref().map_or(Ok(Default::default()),|source|if source.terminal_is_empty(){Ok(RetirementDemand{release_bytes:source.owner_cell_bytes(),depth:1,..Default::default()})}else{source.retirement_demands(copy)})
 }
}
impl RetirementCursor for ChildEmitPreparationRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if let Some(fault)=self.fault.as_mut(){return match fault.step(grant){Ok(step)=>{if fault.terminal_is_empty(){drop(self.fault.take());}RetirementStep::Progress(step.progress())},Err(error)=>RetirementStep::Failure(error)}}
  let Some(source)=self.original.as_mut()else{return RetirementStep::Complete};
  if source.terminal_is_empty(){let bytes=source.owner_cell_bytes();if grant.maximum_depth==0||grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted}drop(self.original.take());return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})}
  source.begin_close();
  match source.close_step(grant){
   Ok(PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress))=>RetirementStep::Progress(progress),
   Ok(PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..})=>RetirementStep::BudgetExhausted,
   Err(original)=>{self.fault=Some(ControlledRetirement::new(original).expect("full original diagnostic close authority"));RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"original child preparation refusal remains retained"))}
  }
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.fault.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for ChildEmitPreparationRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child preparation and full refusal remain retained until granted close");}}
impl RetireOwned for ChildEmitPreparation{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ChildEmitPreparationRetirement{original:ManuallyDrop::new(Some(self)),fault:None})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<ChildEmitPreparationRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
