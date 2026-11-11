use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};

/// ♻️ Original child emission sources retain their native close authority.
#[doc(hidden)]
pub struct OwnedChildEmitRetirement{original:ManuallyDrop<Option<OwnedChildEmit>>}
/// ⏱️ Records work a still grant-shaped delegate already did on the caller's effect.
fn pay_work(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,work:u64)->Result<(),ValueError>{
 if effect.performed(work)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original child emission delegate exceeds its work grant"))}
}
impl RetirementCursor for OwnedChildEmitRetirement{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.original.as_ref().map_or(Ok(Default::default()),|source|if source.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{source.retirement_demands(maximum_body_bytes)})}

 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|source.retirement_demands(0).map(|d|d.copy_bytes))}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.retirement_demands(copy).ok().map(|d|d.capacity_bytes))}
 fn next_close_byte_demand(&self)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.retirement_demands(0).ok().map(|d|d.release_bytes))}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|if source.terminal_is_empty(){Ok(1)}else{source.retirement_demands(0).map(|d|d.depth)})}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl OwnedChildEmitRetirement{
 pub(crate) fn close_step(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();
  let Some(source)=self.original.as_mut()else{return RetirementStep::Complete(0)};
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if source.terminal_is_empty(){
   if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return RetirementStep::Failure(error.into(),effect.performed_work())}
   drop(self.original.take());return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},effect.performed_work())
  }
  match source.close_step(effect){Ok(step)=>RetirementStep::Progress(step.progress(),effect.performed_work()),Err(error)=>RetirementStep::Failure(error,effect.performed_work())}
 }
}
impl Drop for OwnedChildEmitRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.original.is_none(),"original owned child emission remains retained until full close");}}
impl RetireOwned for OwnedChildEmit{
 type RetirementCursor=OwnedChildEmitRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{OwnedChildEmitRetirement{original:ManuallyDrop::new(Some(self))}}
 fn controlled_retirement_supported()->bool{true}
}

#[doc(hidden)]
pub struct ChildEmitPreparationRetirement{original:ManuallyDrop<Option<ChildEmitPreparation>>,fault:Option<ControlledRetirement<PluginLifecycleFailure>>}
impl ChildEmitPreparationRetirement{
 pub(crate) fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if let Some(fault)=self.fault.as_mut(){return match fault.step(body,effect){
   semio_framework_value::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),
   semio_framework_value::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>{if fault.terminal_is_empty(){drop(self.fault.take());}RetirementStep::Progress(step.progress(),effect.performed_work())},Err(error)=>RetirementStep::Failure(error,effect.performed_work())}
  }}
  let Some(source)=self.original.as_mut()else{return RetirementStep::Complete(0)};
  if source.terminal_is_empty(){let bytes=source.owner_cell_bytes();if grant.maximum_depth==0||grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted}if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return RetirementStep::Failure(error.into(),effect.performed_work())}drop(self.original.take());return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()},effect.performed_work())}
  source.begin_close();
  let result=source.close_step(effect);let paid=pay_work(effect,source.last_turn_work());let work=effect.performed_work();
  if let Err(error)=paid{return RetirementStep::Failure(error.into(),work)}
  match result{
   Ok(PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress))=>RetirementStep::Progress(progress,work),
   Ok(PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..})=>if work==0{RetirementStep::BudgetExhausted}else{RetirementStep::Progress(Default::default(),work)},
   Err(original)=>{
    if original.receipt_refusal.is_none()&&original.context_refusal.is_none()&&original.work_refusal.is_none(){
     match original.primary{
      semio_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Retirement(failure) if failure.retained_progress()==original.effects=>return RetirementStep::Failure(failure,work),
      semio_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Value(failure) if failure.retained_progress()==original.effects=>return RetirementStep::Failure(semio_framework_value::OriginalRetirementFailure::from_primary(failure),work),
      primary=>{let effects=original.effects;self.fault=Some(ControlledRetirement::retain(PluginLifecycleFailure{effects,primary,receipt_refusal:None,context_refusal:None,work_refusal:None}));return RetirementStep::Progress(effects,work)}
     }
    }
    let effects=original.effects;self.fault=Some(ControlledRetirement::retain(original));RetirementStep::Progress(effects,work)
   }
  }
 }

 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(fault)=self.fault.as_ref(){return fault.next_demand(copy)}
  self.original.as_ref().map_or(Ok(Default::default()),|source|if source.terminal_is_empty(){Ok(RetirementDemand{release_bytes:source.owner_cell_bytes(),depth:1,..Default::default()})}else{source.retirement_demands(copy)})
 }
}
impl RetirementCursor for ChildEmitPreparationRetirement{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}

 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.fault.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ChildEmitPreparationRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child preparation and full refusal remain retained until granted close");}}
impl RetireOwned for ChildEmitPreparation{
 type RetirementCursor=ChildEmitPreparationRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ChildEmitPreparationRetirement{original:ManuallyDrop::new(Some(self)),fault:None}}
 fn controlled_retirement_supported()->bool{true}
}
