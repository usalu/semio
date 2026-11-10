//! 🌱️ Adapts a source-free default cursor to the receiver contract so the shared receiver driver can cut, deny and close it.
use semio_framework_value_native_owned::*;
use semio_framework_value_native_owned::owned::{NativeAdvanceSelection,NativeReceiver};
use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary};
use semio_framework_value_native_owned::retained_clone::RetainedCloneRef;
use semio_framework_value_native_owned::retirement::RetireOwned;
/// 🌱️ Wraps one default cursor.
pub struct DefaultReceiver<C>(pub C);
type Reply<'r,E>=OriginalEffectReply<'r,RetainedCloneStep,E>;
type Done<'r,E>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>;
impl<T:RetireOwned,C:semio_framework_value_native_owned::owned::NativeDefaultCursor<T>> NativeReceiver<DslValue,T> for DefaultReceiver<C>{
 fn demands(&self,_:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<RetirementDemand,ValueError>{self.0.demands(b,m)}
 fn select_advance(&self,_:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<NativeAdvanceSelection,ValueError>{self.0.select_advance(b,m)}
 fn advance_nested(&mut self,_:RetainedCloneRef<'_,DslValue>,c:&mut NativeDecodeControl<'_>,w:OriginalSourceTurnWitness<'_>,b:usize,m:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,semio_framework_value_native_owned::native_decoding::NativeReceivingFailure>{self.0.advance_nested(c,w,b,m,g)}
 fn advance_local<'r>(&mut self,_:RetainedCloneRef<'_,DslValue>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.0.advance_local(b,e,r)}
 fn recover_advance_local<'r>(&mut self,_:RetainedCloneRef<'_,DslValue>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.0.recover_advance_local(b,e,r)}
 fn advance_pending(&self)->bool{self.0.advance_pending()}
 fn output_ready(&self)->bool{self.0.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.0.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.0.take_work_demand()}
 fn take<'r>(&mut self,o:&mut Option<T>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.0.take(o,e,r)}
 fn recover_take<'r>(&mut self,o:&mut Option<T>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.0.recover_take(o,e,r)}
 fn is_closing(&self)->bool{self.0.is_closing()}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.0.begin_close_demand()}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.0.begin_close_work_demand()}
 fn begin_close_into<'r>(&mut self,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.begin_close_into(e,r)}
 fn recover_begin_close_into<'r>(&mut self,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.recover_begin_close_into(e,r)}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{self.0.close_work_demand(b)}
 fn close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.close_step(b,e,r)}
 fn recover_close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.recover_close_step(b,e,r)}
 fn close_original_step<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.close_original_step(o,b,e,r)}
 fn recover_close_original_step<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.0.recover_close_original_step(o,b,e,r)}
 fn close_original_birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{self.0.close_original_birth_step(o,g)}
 fn requires_original_control(&self)->bool{self.0.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.0.requires_original_birth()}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
