//! 🧩️ The receive slot is one field slot of the record machine.
use super::{NativeReceive,OwnedFrame,SharedFrame,native_close_settle};
use super::tree::{NativeValueSource,invalid};
use crate::native_decoding::NativeReceivingFailure;
use crate::{NativeDecodeControl,OriginalReceivingPrimary,OriginalRetirementCloseLoan,OriginalRetirementPoll,OriginalSourceTurnWitness,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError};
use semio_framework_value_native_record_machine::machine::{RecordSlot,RecordSlotView,Source,Frame,Step,Close,progress,close_plan_of};
use super::tree::NativeReceiveSlot;
fn reset_shared<T:NativeReceive<dyn NativeValueSource>>(slot:&mut NativeReceiveSlot<dyn NativeValueSource,T,SharedFrame>,frame:*mut Frame){if slot.terminal_is_empty(){*slot=Default::default();slot.anchor_frame(frame)}}
fn reset_owned<T:NativeReceive<dyn NativeValueSource>>(_:&mut NativeReceiveSlot<dyn NativeValueSource,T,OwnedFrame>,_:*mut Frame){}
macro_rules! slot_impls{($frame:ty,$reset:path)=>{

impl<T:NativeReceive<dyn NativeValueSource>> RecordSlotView for NativeReceiveSlot<dyn NativeValueSource,T,$frame>{
 fn view_empty(&self)->bool{self.terminal_is_empty()}
 fn view_birth(&self)->bool{self.requires_original_birth()}
 fn view_control(&self)->bool{self.requires_original_control()}
 fn view_pending(&self)->bool{self.advance_pending()}
 fn view_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.close_original_birth_step(original,grant)}
}
impl<T:NativeReceive<dyn NativeValueSource>> RecordSlot<T> for NativeReceiveSlot<dyn NativeValueSource,T,$frame>{
 fn slot_work(&self,source:&Source,body:usize)->Result<u64,ValueError>{self.advance_work_demand(source.as_ref().ok_or_else(invalid)?.borrow()?,body)}
 fn slot_nested(&mut self,source:&Source,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{
  let source=match source.as_ref().ok_or_else(invalid).and_then(|s|s.borrow()){Ok(s)=>s,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  self.advance(source,control,witness,body,maximum,grant).map(|s|Step::Progress(s.progress()))}
 fn slot_retire(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{native_close_settle(effect,|effect,reply|self.close_into(body,effect,reply)).map(progress)}
 fn slot_begin(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Close{native_close_settle(effect,|effect,reply|self.begin_close_into(effect,reply)).map(progress)}
 fn slot_close_plan(&self,body:usize)->Option<Result<(RetirementDemand,u64),ValueError>>{close_plan_of(self.terminal_is_empty(),self.is_closing(),||Ok((self.close_demands(body)?,self.close_work_demand(body)?)),||Ok((self.begin_close_demand()?,self.begin_close_work_demand()?)))}
 fn slot_close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<Close>{
  if self.terminal_is_empty(){return None}
  Some(if self.is_closing(){native_close_settle(effect,|effect,reply|match original.as_deref_mut(){Some(loan)=>self.close_original_into(loan,body,effect,reply),None=>self.close_into(body,effect,reply)})}else{native_close_settle(effect,|effect,reply|self.begin_close_into(effect,reply))}.map(progress))}
 fn slot_take(&mut self,pending:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<Step,OriginalReceivingPrimary>{self.take(pending,effect)}
 fn slot_reset(&mut self,frame:*mut Frame){$reset(self,frame)}
}
}}
slot_impls!(SharedFrame,reset_shared);
slot_impls!(OwnedFrame,reset_owned);
