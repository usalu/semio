//! 🎟️ Every fixture supplies its original issuer, source admission, clock and work before a once-paid caller is born.
use super::{CancelToken,JobPayloadAuthority,OperationId,Generation,RetainedCloneGrant,RetainedCloneProgress,StepContext,StepBudget,OriginalTurnFailure};
use std::mem::ManuallyDrop;
use super::{InteractiveJob,WorkerJobSession,MountedWorkerJobSession,BatchJobSession,WorkerJobCloseStep,WorkerJobDemandError,RetainedCloneStep,ValueError,ValueRefusalKind};
pub trait OriginalFixtureSession{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>;
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>;
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>;
}
macro_rules! original_session{($owner:ident)=>{impl<J:InteractiveJob+'static> OriginalFixtureSession for $owner<J>{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{self.has_original_cancel_alias_witness(witness)}
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{self.return_original_cancel_alias_step(witness,grant)}
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{self.close_step(cx)}
}}}
original_session!(WorkerJobSession);original_session!(MountedWorkerJobSession);original_session!(BatchJobSession);
pub struct OriginalJobFixtureCaller{operation:OperationId,generation:Generation,cancel:ManuallyDrop<Option<CancelToken>>,authority:JobPayloadAuthority,sequence:u64,clock:fn()->Option<u64>,deadline:u64,fuel:u64,born:usize,last:RetainedCloneProgress}
impl OriginalJobFixtureCaller{
    pub fn admit(operation:OperationId,generation:Generation,source:RetainedCloneGrant,clock:fn()->Option<u64>,deadline:u64,fuel:u64)->Self{
        let((cancel,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||CancelToken::admit_root(source).unwrap().unwrap());assert!(receipt.fits(source));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));let mut born=heap.requested_bytes;
        let((authority,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobPayloadAuthority::admit(operation,generation,source).unwrap().unwrap());assert!(receipt.fits(source));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));born+=heap.requested_bytes;
        Self{operation,generation,cancel:ManuallyDrop::new(Some(cancel)),authority,sequence:0,clock,deadline,fuel,born,last:RetainedCloneProgress::default()}
    }
    pub fn born_bytes(&self)->usize{self.born}
    pub fn cancel(&self)->&CancelToken{self.cancel.as_ref().expect("original fixture caller retains its root")}
    pub fn admit_alias(&mut self,source:RetainedCloneGrant)->CancelToken{let alias=self.turn(source,|cx|cx.admit_original_cancel_alias().map_err(OriginalTurnFailure::from)).expect("original fixture alias turn").expect("original fixture alias admitted");assert!(alias.is_same_node(self.cancel()));assert_eq!(self.last,RetainedCloneProgress{copied_items:1,..Default::default()});alias}
    pub fn turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let mut receipt=RetainedCloneProgress::default();let mut cx=StepContext::with_payload_authority(self.operation,self.generation,StepBudget::new(self.fuel,self.deadline,grant),self.cancel.as_ref().unwrap(),self.clock,&mut self.sequence,&mut receipt,&self.authority)?;let result=advance(&mut cx);drop(cx);self.last=receipt;if result.is_ok(){assert!(receipt.fits(grant));}result
    }
    pub fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let step=self.turn(grant,|cx|{let remaining=cx.retained_grant();if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            let witnessed=match owner.witnessed_alias(cx.original_cancel_token()){Ok(value)=>value,Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            if !witnessed{return owner.close_original(cx)}
            match owner.return_alias(cx.original_cancel_token(),remaining){Ok(Some(step))=>{cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(WorkerJobCloseStep::Pending{progress:step.progress()})},Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>Err(OriginalTurnFailure::debit_original(error,cx))}
        })?;assert_eq!(self.last,step.progress());Ok(step)
    }
    pub fn receipt(&self)->RetainedCloneProgress{self.last}
    pub fn return_alias(&mut self,token:CancelToken,grant:RetainedCloneGrant){
        assert!(token.is_same_node(self.cancel()));let mut owner=semio_framework_async::CancelTokenRetirement::from_token(token);self.turn(grant,|cx|{let step=owner.return_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>OriginalTurnFailure::from(error),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>OriginalTurnFailure::from(super::ValueError::literal(super::ValueRefusalKind::UnsupportedOwner,"original fixture alias remains blocked"))})?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(step)}).unwrap();assert!(owner.terminal_is_empty());
    }


    pub fn retire(mut self,source:RetainedCloneGrant)->usize{
        let mut released=0;while !self.authority.terminal_is_empty(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.authority.close_step(source).unwrap());assert!(step.progress().fits(source));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));released+=heap.released_bytes;}
        let mut cancel=semio_framework_async::CancelTokenRetirement::from_token(self.cancel.take().unwrap());while !cancel.terminal_is_empty(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cancel.close_step(source).unwrap());assert!(step.progress().fits(source));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));released+=heap.released_bytes;}assert_eq!(released,self.born);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cancel));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));released
    }
}
impl Drop for OriginalJobFixtureCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.cancel.is_none()&&self.authority.terminal_is_empty(),"original fixed fixture caller requires paid terminal closure")}}
