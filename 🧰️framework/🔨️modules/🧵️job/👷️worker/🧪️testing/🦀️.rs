//! 🧪️ Original worker closure borrows the canonical caller without a reverse core edge.
use super::*;
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
pub trait OriginalWorkerFixtureCaller{fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>;}
impl OriginalWorkerFixtureCaller for OriginalJobFixtureCaller{
    fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let step=self.turn(grant,|cx|{let remaining=cx.retained_grant();if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            let witnessed=match owner.witnessed_alias(cx.original_cancel_token()){Ok(value)=>value,Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            if !witnessed{return owner.close_original(cx)}
            match owner.return_alias(cx.original_cancel_token(),remaining){Ok(Some(step))=>{cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(WorkerJobCloseStep::Pending{progress:step.progress()})},Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>Err(OriginalTurnFailure::debit_original(error,cx))}
        })?;assert_eq!(self.receipt(),step.progress());Ok(step)
    }
}
