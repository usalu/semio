//! 📬️ One delivered fact keeps every metadata, fact and receipt owner until paid closure.
use super::*;
use semio_framework_value::{ValueError,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;
#[derive(Default)]
pub struct OriginalHostEventDelivery{pub metadata:ManuallyDrop<Option<ActionMeta>>,pub event:ManuallyDrop<Option<crate::HostEvent>>,pub outcome:ManuallyDrop<Option<semio_framework::kernel::InvocationResult>>,retirement:RetainedCloneClose}
impl OriginalHostEventDelivery{
 pub fn terminal_is_empty(&self)->bool{self.metadata.is_none()&&self.event.is_none()&&self.outcome.is_none()&&self.retirement.is_empty()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.outcome.is_some(){return self.retirement.next_owner_demands::<semio_framework::kernel::InvocationResult>(true,&None,body)}if self.event.is_some(){return self.retirement.next_owner_demands::<crate::HostEvent>(true,&None,body)}if self.metadata.is_some(){return self.retirement.next_owner_demands::<ActionMeta>(true,&None,body)}Ok(Default::default())}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.with_original_retirement(|cx|{let body=cx.retained_grant().maximum_copy_bytes;let d=self.close_demands(body)?;if !self.retirement.is_empty(){close_pending(cx,&mut self.retirement)?;return Ok(false)}macro_rules! close{($field:ident)=>{if self.$field.is_some(){begin_close(cx,d,&mut self.retirement,&mut self.$field)?;return Ok(false)}}}close!(outcome);close!(event);close!(metadata);Ok(true)})}
}
