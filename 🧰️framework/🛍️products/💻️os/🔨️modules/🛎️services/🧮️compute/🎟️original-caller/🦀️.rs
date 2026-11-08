//! 🎟️ The compute pool's one original caller: it owns the paid cancellation root, payload authority and source parent that every job callback borrows, and debits each turn to the supplied recipient.
use crate::ComputeRetainedRecipient;
use semio_framework_async::{CancelToken,CancelTokenRetirement};
use semio_framework_job::{ClockStride,Generation,OperationId,OriginalCallerSourceParent,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,StepBudget,StepContext,default_now_us};
use semio_framework_job_payload::JobPayloadAuthority;
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicU64,Ordering};

/// ⛽️ Total source fuel one compute caller issues to every callback it lends.
const COMPUTE_CALLER_FUEL: u64 = 1 << 40;
/// 🛟️ Items withheld from job steps so a funded caller can always pay its own retirement.
const COMPUTE_RETIREMENT_RESERVE_ITEMS: usize = 256;
/// 🛟️ Copy bytes withheld from job steps for the same retirement.
const COMPUTE_RETIREMENT_RESERVE_COPY_BYTES: usize = 65_536;
static NEXT_COMPUTE_CLOCK_BINDING: AtomicU64 = AtomicU64::new(1);

pub struct ComputeOriginalCaller{parent:ManuallyDrop<Option<OriginalCallerSourceParent>>,sequence:u64,clock_binding:u64,deadline:u64,fuel:u64,stride:ClockStride,stock:RetainedCloneGrant,spent:RetainedCloneProgress,last:RetainedCloneProgress}

fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:stock.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:stock.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:stock.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:stock.maximum_release_bytes.saturating_sub(spent.released_bytes),..stock}}
fn narrow(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
fn delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)}}
fn retire_roots(mut authority:Option<JobPayloadAuthority>,cancel:Option<CancelToken>,grant:RetainedCloneGrant){
    let mut cancel=cancel.map(CancelTokenRetirement::from_token);
    for _ in 0..64{
        if let Some(owner)=authority.as_mut().filter(|owner|!owner.terminal_is_empty()){let _=owner.close_step(grant);continue}
        if let Some(owner)=cancel.as_mut().filter(|owner|!owner.terminal_is_empty()){let _=owner.close_step(grant);continue}
        break
    }
}

impl ComputeOriginalCaller{
    /// 🌱️ Births the cancellation root, payload authority and source parent from `stock`, or reports `None` while the stock cannot fund all three.
    pub fn admit(operation:OperationId,generation:Generation,stock:RetainedCloneGrant,deadline:u64)->Result<Option<Self>,ValueError>{
        let capacity=CancelToken::root_admission_demands().capacity_bytes.checked_add(JobPayloadAuthority::admission_demand().capacity_bytes).and_then(|bytes|bytes.checked_add(OriginalCallerSourceParent::admission_demand().capacity_bytes)).ok_or_else(||refused("compute caller birth layout overflow"))?;
        if stock.maximum_items<3||stock.maximum_capacity_bytes<capacity||stock.maximum_depth==0{return Ok(None)}
        let Some((cancel,receipt))=CancelToken::admit_root(stock).result?else{return Ok(None)};
        let mut spent=RetainedCloneProgress::default().checked_add(receipt)?;
        let mut payload_actual=None;let mut parent_actual=None;
        let admitted=JobPayloadAuthority::admit(operation,generation,remaining(stock,spent),&mut payload_actual).result;
        let (authority,receipt)=match admitted{Ok(Some(born))=>born,Ok(None)=>{retire_roots(None,Some(cancel),stock);return Ok(None)},Err(error)=>{retire_roots(None,Some(cancel),stock);return Err(error)}};
        spent=spent.checked_add(receipt)?;
        match OriginalCallerSourceParent::admit(authority,cancel,remaining(stock,spent),&mut parent_actual).result{
            Ok((parent,receipt))=>{
                spent=spent.checked_add(receipt)?;
                Ok(Some(Self{parent:ManuallyDrop::new(Some(parent)),sequence:0,clock_binding:NEXT_COMPUTE_CLOCK_BINDING.fetch_add(1,Ordering::Relaxed),deadline,fuel:COMPUTE_CALLER_FUEL,stride:ClockStride::new(),stock,spent,last:receipt}))
            },
            Err((error,authority,cancel))=>{retire_roots(Some(authority),Some(cancel),stock);Err(error)},
        }
    }

    /// 🧾️ The cumulative physical receipt of the births and every turn so far.
    pub fn cumulative_receipt(&self)->RetainedCloneProgress{self.spent}

    /// 🧾️ The physical receipt of the latest turn only.
    pub fn receipt(&self)->RetainedCloneProgress{self.last}

    pub fn remaining_grant(&self)->RetainedCloneGrant{remaining(self.stock,self.spent)}

    pub fn terminal_is_empty(&self)->bool{self.parent.is_none()}

    pub fn cancel(&self)->Result<&CancelToken,ValueError>{self.parent.as_ref().ok_or_else(||refused("compute caller parent already closed"))?.cancellation()}

    /// 🎟️ Lends one genuine context under `grant` and returns the callback's value; the receipt of exactly this turn stays readable through [`Self::receipt`].
    pub fn turn<R>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.last=Default::default();
        let before=self.spent;let issued=self.fuel;let allowed=narrow(grant,self.remaining_grant());let mut value=None;self.stride.begin(default_now_us());
        let parent=self.parent.as_ref().ok_or_else(||refused("compute caller parent already closed"))?;
        let mut cx=parent.borrow_context(StepBudget::new(self.fuel,self.deadline,self.stock),default_now_us,&mut self.sequence,&mut self.spent)?;
        cx.bind_original_clock_binding(self.clock_binding)?;cx.restore_clock_stride(self.stride);
        let result=cx.with_retained_loan(allowed,|cx|{value=Some(run(cx)?);Ok(())});
        self.fuel=cx.original_source_fuel_remaining().min(issued);self.stride=cx.clock_stride();drop(cx);
        self.last=delta(self.spent,before);
        result?;value.ok_or_else(||refused("compute caller callback returned no value").into())
    }

    /// 💰️ Runs one turn under the recipient's remaining grant and debits the recipient with its actual receipt even when the callback fails.
    pub fn turn_paid<R>(&mut self,recipient:&mut impl ComputeRetainedRecipient,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let grant=recipient.remaining_grant()?;
        let result=self.turn(grant,run);
        let paid=recipient.record_progress(self.last);
        let value=result?;paid?;Ok(value)
    }

    fn spendable(&self,recipient:&impl ComputeRetainedRecipient)->Result<RetainedCloneGrant,ValueError>{
        let grant=narrow(recipient.remaining_grant()?,self.remaining_grant());
        Ok(RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(COMPUTE_RETIREMENT_RESERVE_ITEMS),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(COMPUTE_RETIREMENT_RESERVE_COPY_BYTES),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(self.spent.retained_capacity_bytes.saturating_sub(self.spent.released_bytes)),..grant})
    }

    /// 🛟️ Whether the recipient can no longer fund a job step without eating the retirement reserve.
    pub fn is_exhausted(&self,recipient:&impl ComputeRetainedRecipient)->bool{self.spendable(recipient).map_or(true,|grant|grant.maximum_items==0||grant.maximum_depth==0)}

    /// 💰️ Runs one job turn like [`Self::turn_paid`] but never spends the retirement reserve.
    pub fn turn_funded<R>(&mut self,recipient:&mut impl ComputeRetainedRecipient,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let grant=self.spendable(recipient)?;
        let result=self.turn(grant,run);
        let paid=recipient.record_progress(self.last);
        let value=result?;paid?;Ok(value)
    }

    /// ♻️ Runs one closing turn as original retirement so an expired job deadline never starves paid closure.
    pub fn turn_retiring<R>(&mut self,recipient:&mut impl ComputeRetainedRecipient,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.turn_paid(recipient,|cx|cx.with_original_retirement(run))
    }

    /// 🪢️ Admits one alias of the owned cancellation root for a job's parameters.
    pub fn admit_alias(&mut self,recipient:&mut impl ComputeRetainedRecipient)->Result<Option<CancelToken>,OriginalTurnFailure>{
        let mut alias=None;
        self.turn_paid(recipient,|cx|cx.admit_original_cancel_alias(&mut alias))?;
        Ok(alias)
    }

    /// ♻️ Closes the owned roots one paid step at a time; the recipient is debited with the actual receipt.
    pub fn retire_paid(&mut self,recipient:&mut impl ComputeRetainedRecipient)->Result<RetainedCloneStep,ValueError>{
        self.last=Default::default();
        if self.parent.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let grant=narrow(recipient.remaining_grant()?,self.remaining_grant());
        if grant.maximum_items==0||grant.maximum_depth==0||self.fuel==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=if self.parent.as_ref().is_some_and(OriginalCallerSourceParent::terminal_is_empty){drop(self.parent.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}else{match self.parent.as_mut(){Some(parent)=>parent.close_step(grant),None=>return Err(refused("compute caller parent already closed"))}};
        self.fuel=self.fuel.saturating_sub(u64::try_from(turn.work).unwrap_or(u64::MAX));
        let step=turn.result?;let receipt=step.progress();
        self.spent=self.spent.checked_add(receipt)?;self.last=receipt;
        recipient.record_progress(receipt)?;
        Ok(if self.parent.is_none(){RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})
    }
}

impl Drop for ComputeOriginalCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.parent.is_none(),"compute original caller reached Drop before its paid roots closed")}}
