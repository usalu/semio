//! 📬️ A cold crossing retains its whole physical reply until the real Source receives both receipts.
use semio_framework_actor::{OriginalGuestExecutionReceipt,OriginalJobStartReply,OriginalJobStepReply,OriginalJobOutcome,OriginalJobOutputReply,OriginalJobOutputAckReply,OriginalJobCancelReply,OriginalJobTurnLoan};
use semio_plugin_host_guest_runtime::OriginalGuestJobCall;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind};

pub(super) enum GuestRelayNativeResponse{
 Start(OriginalGuestJobCall<OriginalJobStartReply>),
 Step(OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>),
 Read(OriginalGuestJobCall<OriginalJobOutputReply>),
 Ack(OriginalGuestJobCall<OriginalJobOutputAckReply>),
 Cancel(OriginalGuestJobCall<OriginalJobCancelReply>),
}
impl GuestRelayNativeResponse{
 pub(super) fn execution(&self)->OriginalGuestExecutionReceipt{
  match self{Self::Start(call)=>call.execution,Self::Step(call)=>call.execution,Self::Read(call)=>call.execution,Self::Ack(call)=>call.execution,Self::Cancel(call)=>call.execution}
 }
 pub(super) fn receive(&self,loan:&mut OriginalJobTurnLoan,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let receipt=match self{
   Self::Start(call)=>call.original.as_ref().ok().map(|reply|(reply.input,reply.spent,reply.work_units_spent)),
   Self::Step(call)=>call.original.as_ref().ok().map(|reply|(reply.input,reply.spent,reply.work_units_spent)),
   Self::Read(call)=>call.original.as_ref().ok().map(|reply|(reply.input,reply.spent,reply.work_units_spent)),
   Self::Ack(call)=>call.original.as_ref().ok().map(|reply|(reply.input,reply.spent,reply.work_units_spent)),
   Self::Cancel(call)=>call.original.as_ref().ok().map(|reply|(reply.input,reply.spent,reply.work_units_spent)),
  };
  if let Some((input,spent,work))=receipt{return loan.receive(input,spent,work,cx).map_err(Into::into)}
  let execution=self.execution();
  if !execution.submitted{return loan.receive(execution.budget.input,Default::default(),0,cx).map_err(Into::into)}
  Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"submitted guest crossing did not return its original CPU and retained receipt").into())
 }
}
