//! 🎟️ An original receiving continuation retains its initiating parent, recipient and physical work lifetime.
use std::{cell::Cell,mem::ManuallyDrop};
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
#[doc(hidden)]
pub struct ReceivingSource{pub identity:(semio_framework_job::OperationId,semio_framework_job::Generation),pub parent:usize,pub recipient:usize,pub epoch:Cell<Option<u64>>,pub clock:fn()->Option<u64>,observed:Cell<Option<u64>>,deadline:u64,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning_loan:bool}
impl ReceivingSource{
 pub fn source_is_returned(&self)->bool{self.loan.is_none()}
 pub fn new(cx:&StepContext<'_>)->Result<Self,OriginalTurnFailure>{Ok(Self{identity:(cx.operation(),cx.generation()),parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),epoch:Cell::new(cx.original_source_epoch()),clock:cx.clock_source(),observed:Cell::new(cx.latest_us()),deadline:cx.deadline_us(),loan:ManuallyDrop::new(None),returning_loan:false})}
 pub fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let epoch=cx.original_source_epoch();let now=cx.now_us();
  if self.identity!=(cx.operation(),cx.generation())||self.parent!=cx.original_source_parent()?.source_identity()?||self.recipient!=cx.original_recipient_identity()||!std::ptr::fn_addr_eq(self.clock,cx.clock_source())||self.deadline!=cx.deadline_us()||self.epoch.get().is_some()!=epoch.is_some()||matches!((self.epoch.get(),epoch),(Some(before),Some(after))if after<before)||now.is_none()||matches!((self.observed.get(),now),(Some(before),Some(after))if after<before){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving continuation requires its initiating source recipient clock and work lifetime").into())}
  self.epoch.set(epoch);self.observed.set(now);
  if !self.returning_loan{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())
 }
 pub fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.loan.is_none()&&cx.is_cancelled(){return Ok(true)}if self.returning_loan||self.loan.is_some(){return Ok(true)}cx.original_source_parent()?.admit_loan(&mut self.loan,cx)?;Ok(false)}
 pub fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;let Some(loan)=self.loan.as_mut()else{return Ok(true)};self.returning_loan=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}Ok(false)}
}

pub use semio_plugin_job_receiving::original_steps::{begun,forgotten};
pub fn retired_poll(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_effect_step(cx,demand,advance)}
