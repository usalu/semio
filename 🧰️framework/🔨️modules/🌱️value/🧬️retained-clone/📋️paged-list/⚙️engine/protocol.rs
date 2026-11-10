//! 📋️ Native element seeds and caller-resident replies preserve actual Source and lifecycle custody.
use semio_framework_value_control_contract::*;
use semio_framework_value_clone_source::RetainedCloneRef;
use semio_framework_value_retirement::{RetireOwned,OriginalRetirementFailure,OriginalRetirementCloseLoan,OriginalRetirementPoll};
pub use semio_framework_value_native_clone_engine::RetainedCursorRelease;
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
/// 🔭️ Quotes and readiness queries are pure, bounded, nonallocating, callback-free and nonunwinding.
pub trait PagedCloneElement<T:RetireOwned+Send+Sync+'static>:Send+Sized{
 /// 🌱️ Constructs fixed native inline metadata without allocation, callbacks or unwind.
 fn new()->Self;
 /// 🎟️ Declares the real terminal native destructor; unknown owners keep their header.
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{None}
 fn output_ready(&self)->bool;
 fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>;
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>;
 fn advance_selection(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<(RetirementDemand,u64),ValueError>{Ok((self.advance_demands(source,body,maximum)?,self.advance_work_demand(source,body,maximum)?))}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn recover_take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn begin_close_work_demand(&self)->Result<u64,ValueError>;
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn close_started(&self)->bool;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn requires_original_birth(&self)->bool{false}
 fn close_original_birth_step(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"paged element has no original birth").into()),work:0})}
 fn requires_original_control(&self)->bool{false}
 fn close_original_into<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if self.requires_original_control(){return Err(ValueError::invariant("paged element lacks its original close producer"))}self.close_into(body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if self.requires_original_control(){return Err(ValueError::invariant("paged element lacks its original close recovery"))}self.recover_close_into(body,effect,reply)}
 fn terminal_is_empty(&self)->bool;
}
