//! 🌱️ Original transient registration keeps its actual typed factory bundle through insertion and cancellation.
use super::*;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand};
use std::{marker::PhantomData,mem::{ManuallyDrop,size_of,size_of_val}};
use semio_framework_value::RetainedCloneStep;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn admission(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn built<T>(cx:&mut StepContext<'_>,output:&mut Option<T>,d:RetirementDemand,build:impl FnOnce()->Result<(T,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{semio_framework::action_bus::admit_original_construction_settled(cx,&mut Default::default(),output,admission(d),1,|_,actual|{let result=build();if let Ok((_,progress))=&result{if progress.retained_capacity_bytes!=0&&actual.is_none(){*actual=Some(progress.retained_capacity_bytes)}}semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})}
fn retired(cx:&mut StepContext<'_>,d:RetirementDemand,close:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{semio_framework_job::close_original_retained_effect_step(cx,d,|effect|{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original transient registration step exceeded its work authority"))}close(effect)})}
/// 🎟️ The original caller retains the exact owner and private registry insertion until donation or closure.
pub struct OriginalWindowTransientOwnerRegistration<O:WindowTransientOwner>{binding:OriginalSourceBinding,lifetime:u64,key:Option<&'static str>,owner:ManuallyDrop<Option<Option<Box<dyn ErasedWindowTransientStoreOwner>>>>,inserting:bool,adopted:bool,closing:bool,marker:PhantomData<fn()->O>}
impl WindowTransientOwnerRegistry{
 /// 🌱️ An empty original frontier is admitted before its concrete factory bundle or registry backing is allocated.
 pub fn admit_original_owner<O:WindowTransientOwner>(&self,cx:&mut StepContext<'_>,output:&mut Option<Box<OriginalWindowTransientOwnerRegistration<O>>>)->Result<bool,OriginalTurnFailure>{
  if output.is_some()||O::WINDOW_KIND_ID.is_empty()||self.owners.contains_key(O::WINDOW_KIND_ID){return Err(invalid("original transient registration requires a unique kind and empty destination").into())}
  let d=RetirementDemand{copy_bytes:2*size_of::<OriginalWindowTransientOwnerRegistration<O>>(),capacity_bytes:size_of::<OriginalWindowTransientOwnerRegistration<O>>(),depth:1,..Default::default()};let binding=OriginalSourceBinding::borrow(cx);
  built(cx,output,d,||Ok((Box::new(OriginalWindowTransientOwnerRegistration{binding,lifetime:self.lifetime,key:Some(O::WINDOW_KIND_ID),owner:ManuallyDrop::new(None),inserting:false,adopted:false,closing:false,marker:PhantomData}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;Ok(output.is_some())
 }
}
impl<O:WindowTransientOwner> OriginalWindowTransientOwnerRegistration<O>{
 pub fn owner_birth_demands()->RetirementDemand{let mut d=O::owners_birth_demands();d.copy_bytes+=2*size_of::<TypedWindowTransientStoreOwner<O>>()+2*size_of::<Option<Box<dyn ErasedWindowTransientStoreOwner>>>();d.capacity_bytes+=size_of::<TypedWindowTransientStoreOwner<O>>();d.depth=d.depth.max(2);d}
 fn validate(&self,registry:&WindowTransientOwnerRegistry,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.matches(cx)||self.lifetime!=registry.lifetime{return Err(invalid("original transient registration changed caller or per-instance registry"))}Ok(())}
 /// 📬️ Publication moves the same original bundle into its privately prepared registry slot.
 pub fn step(&mut self,registry:&mut WindowTransientOwnerRegistry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(registry,cx)?;if self.adopted{return Ok(true)}if self.closing{return Err(invalid("original transient registration is closing").into())}
  if self.owner.is_none(){let d=Self::owner_birth_demands();built(cx,&mut self.owner,d,||Ok((Some(Box::new(TypedWindowTransientStoreOwner::<O>{partitions:WindowRegistry::new(),owners:Some(O::build_owners()),factory_close:[None,None,None],partition_close_cursor:0,partition_open:0,partition_address:None})as Box<dyn ErasedWindowTransientStoreOwner>),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;return Ok(false)}
  let header=3*size_of::<bool>()+size_of::<usize>();let grant=cx.retained_grant();
  let Some(((turn,ready),_))=cx.with_original_effect(grant,|_,effect|{let result=registry.owners.insert_original(&mut self.key,&mut self.owner,&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(header),..grant}));let work=effect.performed_work();let ready=result.as_ref().ok().map(|(ready,_)|*ready);(semio_framework_value::RetainedWorkOutcome{result:result.map(|(_,mut progress)|{progress.copied_bytes+=header;RetainedCloneStep::Progress(progress)}),work},ready)})?else{return Ok(false)};
  semio_framework_job::receive_original_retained_work(cx,turn)?;
  if let Some(ready)=ready{self.inserting=!ready;self.adopted=ready;self.closing=ready;if ready{registry.owner_open+=1}}Ok(self.adopted)
 }
 /// ♻️ Cancellation restores any insertion hole before retiring the same concrete factories and physical owner.
 pub fn close_step(&mut self,registry:&mut WindowTransientOwnerRegistry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(registry,cx)?;cx.with_original_retirement(|cx|{
   if !self.closing{let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};retired(cx,d,|_|{self.closing=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}
   if self.inserting{let mut d=registry.owners.cancel_insert_original_demands()?;d.copy_bytes+=size_of::<bool>();retired(cx,d,|effect|{let g=effect.grant();let(ready,mut p)=registry.owners.cancel_insert_original(&mut self.key,&mut self.owner,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<bool>(),..g})?;self.inserting=!ready;p.copied_bytes+=size_of::<bool>();Ok(RetainedCloneStep::Progress(p))})?;return Ok(false)}
   if let Some(Some(owner))=self.owner.as_ref(){
    if owner.terminal_is_empty(){let d=RetirementDemand{copy_bytes:size_of::<Option<Option<Box<dyn ErasedWindowTransientStoreOwner>>>>(),release_bytes:size_of_val(&**owner),depth:1,..Default::default()};retired(cx,d,|_|{self.owner.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;return Ok(false)}
    let d=owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_effect_step(cx,d,|effect|{let result=self.owner.as_mut().unwrap().as_mut().unwrap().close_step(effect).map(|step|match step{PluginLifecycleStep::Complete(p)=>RetainedCloneStep::Complete(p),other=>RetainedCloneStep::Progress(other.progress().unwrap_or_default())}).map_err(semio_framework_job::OriginalTurnPrimary::from);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()})})?;return Ok(false)
   }
   Ok(self.terminal_is_empty())
  })
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&!self.inserting&&self.owner.is_none()}
 pub fn boxed_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<O:WindowTransientOwner> Drop for OriginalWindowTransientOwnerRegistration<O>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original transient registration retains its actual owner until original closure")}}
