//! 🧪️ Original worker closure borrows the canonical caller without a reverse core edge.
use super::*;
pub trait OriginalFixtureSession{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>;
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<RetainedCloneStep>,WorkerJobDemandError>;
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>;
}
macro_rules! original_session{($owner:ident)=>{impl<J:InteractiveJob+'static> OriginalFixtureSession for $owner<J>{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{self.has_original_cancel_alias_witness(witness)}
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<RetainedCloneStep>,WorkerJobDemandError>{self.return_original_cancel_alias_step(witness,grant)}
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{self.close_step(cx)}
}}}
original_session!(WorkerJobSession);original_session!(MountedWorkerJobSession);original_session!(BatchJobSession);
pub trait OriginalWorkerFixtureCaller{fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>;}
impl OriginalWorkerFixtureCaller for OriginalJobFixtureCaller{
    fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let step=self.turn(grant,|cx|{let remaining=cx.retained_grant();if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            let witnessed=match owner.witnessed_alias(cx.original_cancel_token()){Ok(value)=>value,Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            if !witnessed{return owner.close_original(cx)}
            let turn=owner.return_alias(cx.original_cancel_token(),remaining);let returned=turn.result.as_ref().is_ok_and(Option::is_some);let result=match turn.result{Ok(Some(step))=>Ok(step),Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(WorkerJobDemandError::Refused(error))=>Err(error)};let step=receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;Ok(if returned{WorkerJobCloseStep::Pending{progress:step.progress()}}else{WorkerJobCloseStep::Blocked})
        })?;assert_eq!(self.receipt(),step.progress());Ok(step)
    }
}
