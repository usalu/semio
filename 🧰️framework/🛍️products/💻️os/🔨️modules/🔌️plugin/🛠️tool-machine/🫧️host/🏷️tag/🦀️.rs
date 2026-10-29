//! 🪦️ Refused native operation tags retain their original receiving authority until physical closure.
use super::{ToolTag,ToolMachineRuntime};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"refused original tool tag changed its receiving source")}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 🪦️ A refused operation returns its actual registry tag through the same financed source.
pub struct OriginalToolTagRetirement{binding:OriginalSourceBinding,decoder:(u64,u64),operation:u64,owner:ControlledRetirement<Option<ToolTag>>,closed:bool}
impl OriginalToolTagRetirement{
 fn receive(operation:u64,extra:usize,take:impl FnOnce()->ToolTag,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if output.is_some(){return Err(invalid().into())}let d=RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<ToolTag>()+extra,capacity_bytes:size_of::<Self>(),depth:1,..Default::default()};let binding=OriginalSourceBinding::borrow(cx);admit_original_construction(cx,output,original(d),|native|{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,operation,owner:ControlledRetirement::retain(Some(take())),closed:false}),progress(d)))})}
 pub fn admit_owned(operation:u64,tag:&mut Option<ToolTag>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if tag.is_none(){return Err(invalid().into())}Self::receive(operation,0,||tag.take().expect("original tag remains in its actual source receiver"),cx,output)}
 pub fn admit<P,M:semio_framework_value::retirement::RetireOwned+Send+'static,C,CM>(machines:&mut ToolMachineRuntime<P,M,C,CM>,operation:u64,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if machines.operation_tag(operation).is_none(){return Err(invalid().into())}let shift=machines.operations.len().checked_mul(size_of::<(u64,ToolTag)>()).ok_or_else(invalid)?;Self::receive(operation,shift,||machines.take_operation(operation).expect("quoted original operation tag remains in its registry"),cx,output)}
 pub fn operation(&self)->u64{self.operation}
 pub fn matches_context(&self,cx:&StepContext<'_>)->bool{self.binding.matches(cx)&&Some(self.decoder)==cx.native_receiving_identity()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_demand(body)}else{Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(())}if !self.matches_context(cx){return Err(invalid().into())}let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;if !self.owner.terminal_is_empty(){close_original_retained_step(cx,d,|g|self.owner.step(g))?;}else{close_original_retained_step(cx,d,|_|{self.closed=true;Ok(RetainedCloneStep::Complete(progress(d)))})?;}Ok(())}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.owner.terminal_is_empty()}
 pub fn shell_close_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalToolTagRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"refused operation tag requires its original paid close")}}
