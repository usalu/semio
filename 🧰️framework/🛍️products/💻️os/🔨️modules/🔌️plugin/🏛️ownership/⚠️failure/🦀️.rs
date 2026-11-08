//! 🪢️ Whole original lifecycle refusals retain their semantic source and receipt refusal separately.
use semio_framework::Fault;
use semio_framework_job::StepContext;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use super::PluginLifecycleStep;

/// 🧳️ The original diagnostic or value refusal moves without cloning or prose conversion.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum PluginLifecyclePrimary{Fault(Fault),Value(ValueError),Domain(semio_framework_job::OriginalDomainPrimary),Receiving(semio_framework_job::OriginalReceivingPrimary),Retirement(semio_framework_value::OriginalRetirementFailure),RetirementReceiving(semio_framework_job::OriginalRetirementReceivingPrimary)}
impl PluginLifecyclePrimary{
 /// 🧾️ Borrows the complete physical receipt of this unpaired producer cause.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Fault(original)=>original.retained_progress(),Self::Value(original)=>original.retained_progress(),Self::Domain(original)=>original.retained_progress(),Self::Receiving(original)=>original.retained_progress(),Self::Retirement(original)=>original.retained_progress(),Self::RetirementReceiving(original)=>original.retirement.retained_progress()}}
 /// 🧷️ Pairs a raw producer cause with the independently refused original receiving debit.
 pub fn with_receiving_receipt(self,receipt:Result<(),ValueError>)->PluginLifecycleFailure{let mut failure=PluginLifecycleFailure::from(self);failure.receipt_refusal=receipt.err();failure}
}
impl From<Fault> for PluginLifecyclePrimary{fn from(original:Fault)->Self{Self::Fault(original)}}
impl From<ValueError> for PluginLifecyclePrimary{fn from(original:ValueError)->Self{Self::Value(original)}}
impl From<semio_framework_value::OriginalRetirementFailure> for PluginLifecyclePrimary{fn from(original:semio_framework_value::OriginalRetirementFailure)->Self{Self::Retirement(original)}}

/// 🧯️ The complete original cause and an independent refusal to debit its physical effects.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct PluginLifecycleFailure{pub effects:RetainedCloneProgress,pub primary:PluginLifecyclePrimary,pub receipt_refusal:Option<ValueError>,pub context_refusal:Option<ValueError>,pub work_refusal:Option<ValueError>}
impl PluginLifecycleFailure{
 /// 🧾️ Borrows the actual failed producer effects carried by its complete original cause.
 pub fn retained_progress(&self)->RetainedCloneProgress{self.effects}
 /// 🪙️ Debits one raw producer refusal through the same initiating context before publication.
 pub fn debit_original(mut self,cx:&mut StepContext<'_>)->Self{if self.receipt_refusal.is_none(){self.receipt_refusal=cx.consume_retained(self.retained_progress()).err()}self}
}
impl From<Fault> for PluginLifecycleFailure{fn from(primary:Fault)->Self{Self{effects:primary.retained_progress(),primary:PluginLifecyclePrimary::Fault(primary),receipt_refusal:None,context_refusal:None,work_refusal:None}}}
impl From<ValueError> for PluginLifecycleFailure{fn from(primary:ValueError)->Self{Self{effects:primary.retained_progress(),primary:PluginLifecyclePrimary::Value(primary),receipt_refusal:None,context_refusal:None,work_refusal:None}}}
impl From<semio_framework_value::OriginalRetirementFailure> for PluginLifecycleFailure{fn from(original:semio_framework_value::OriginalRetirementFailure)->Self{PluginLifecyclePrimary::Retirement(original).into()}}
impl From<semio_framework_job::OriginalTurnFailure> for PluginLifecycleFailure{
 fn from(failure:semio_framework_job::OriginalTurnFailure)->Self{Self{effects:failure.effects,primary:match failure.primary{semio_framework_job::OriginalTurnPrimary::Value(error)=>PluginLifecyclePrimary::Value(error),semio_framework_job::OriginalTurnPrimary::Fault(fault)=>PluginLifecyclePrimary::Fault(fault),semio_framework_job::OriginalTurnPrimary::Domain(original)=>PluginLifecyclePrimary::Domain(original),semio_framework_job::OriginalTurnPrimary::Receiving(original)=>PluginLifecyclePrimary::Receiving(original),semio_framework_job::OriginalTurnPrimary::Retirement(original)=>PluginLifecyclePrimary::Retirement(original),semio_framework_job::OriginalTurnPrimary::RetirementReceiving(original)=>PluginLifecyclePrimary::RetirementReceiving(original),semio_framework_job::OriginalTurnPrimary::NativeSource(outcome)=>PluginLifecyclePrimary::Value(ValueError::new(ValueRefusalKind::InvariantViolated,outcome.message().to_owned()).with_retained_progress(outcome.retained_progress())),semio_framework_job::OriginalTurnPrimary::NativeRetirementSource(outcome)=>PluginLifecyclePrimary::Value(ValueError::new(ValueRefusalKind::InvariantViolated,outcome.message().to_owned()).with_retained_progress(outcome.retained_progress()))},receipt_refusal:failure.receipt_refusal,context_refusal:failure.context_refusal,work_refusal:failure.work_refusal}}
}
impl From<PluginLifecycleFailure> for semio_framework_job::OriginalTurnFailure{
 fn from(failure:PluginLifecycleFailure)->Self{Self{effects:failure.effects,primary:match failure.primary{PluginLifecyclePrimary::Value(error)=>semio_framework_job::OriginalTurnPrimary::Value(error),PluginLifecyclePrimary::Fault(fault)=>semio_framework_job::OriginalTurnPrimary::Fault(fault),PluginLifecyclePrimary::Domain(original)=>semio_framework_job::OriginalTurnPrimary::Domain(original),PluginLifecyclePrimary::Receiving(original)=>semio_framework_job::OriginalTurnPrimary::Receiving(original),PluginLifecyclePrimary::Retirement(original)=>semio_framework_job::OriginalTurnPrimary::Retirement(original),PluginLifecyclePrimary::RetirementReceiving(original)=>semio_framework_job::OriginalTurnPrimary::RetirementReceiving(original)},receipt_refusal:failure.receipt_refusal,context_refusal:failure.context_refusal,work_refusal:failure.work_refusal}}
}

/// 🎟️ A phase loan narrows the independently remaining grant of its actual initiating context.
pub fn original_lifecycle_grant(cx:&StepContext<'_>,loan:RetainedCloneGrant)->RetainedCloneGrant{
 let remaining=cx.retained_grant();RetainedCloneGrant{maximum_items:remaining.maximum_items.min(loan.maximum_items).min(1),maximum_copy_bytes:remaining.maximum_copy_bytes.min(loan.maximum_copy_bytes),maximum_capacity_bytes:remaining.maximum_capacity_bytes.min(loan.maximum_capacity_bytes),maximum_release_bytes:remaining.maximum_release_bytes.min(loan.maximum_release_bytes),maximum_depth:remaining.maximum_depth.min(loan.maximum_depth)}
}

/// 🧾️ Removes the reserved return effects the same turn posted from its raw recipient total, the way the worker pump qualifies its own receipt.
pub fn original_lifecycle_net_after(after:RetainedCloneProgress,reserved_before:RetainedCloneProgress,reserved_after:RetainedCloneProgress)->RetainedCloneProgress{
 let net=|total:usize,earlier:usize,later:usize|total.saturating_sub(later.saturating_sub(earlier));
 RetainedCloneProgress{copied_items:net(after.copied_items,reserved_before.copied_items,reserved_after.copied_items),copied_bytes:net(after.copied_bytes,reserved_before.copied_bytes,reserved_after.copied_bytes),retained_capacity_bytes:net(after.retained_capacity_bytes,reserved_before.retained_capacity_bytes,reserved_after.retained_capacity_bytes),released_bytes:net(after.released_bytes,reserved_before.released_bytes,reserved_after.released_bytes)}
}

/// 🧮️ Subtracts already posted child effects from the same original producer's full receipt.
pub fn original_lifecycle_unposted(progress:RetainedCloneProgress,before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
 let difference=|whole:usize,earlier:usize,later:usize|later.checked_sub(earlier).and_then(|posted|whole.checked_sub(posted)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle child receipt differs from its whole producer receipt").with_retained_progress(progress));
 Ok(RetainedCloneProgress{copied_items:difference(progress.copied_items,before.copied_items,after.copied_items)?,copied_bytes:difference(progress.copied_bytes,before.copied_bytes,after.copied_bytes)?,retained_capacity_bytes:difference(progress.retained_capacity_bytes,before.retained_capacity_bytes,after.retained_capacity_bytes)?,released_bytes:difference(progress.released_bytes,before.released_bytes,after.released_bytes)?})
}

/// 🧮️ Returns the exact four-axis difference of two receipts from one original context.
pub fn original_lifecycle_delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{original_lifecycle_unposted(after,Default::default(),before)}

fn debit_unposted(mut failure:PluginLifecycleFailure,before:RetainedCloneProgress,cx:&mut StepContext<'_>)->PluginLifecycleFailure{
 if failure.receipt_refusal.is_none(){failure.receipt_refusal=match original_lifecycle_unposted(failure.retained_progress(),before,cx.retained_progress()){Ok(progress)=>cx.consume_retained(progress).err(),Err(error)=>Some(error)}}failure
}

/// ⏱️ One original owner action adopts nested effects and debits only its unposted physical receipt.
pub fn original_lifecycle_step(cx:&mut StepContext<'_>,loan:RetainedCloneGrant,demand:RetirementDemand,advance:impl FnOnce(&mut StepContext<'_>,RetainedCloneGrant)->Result<(PluginLifecycleStep,bool),PluginLifecycleFailure>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
 let grant=original_lifecycle_grant(cx,loan);if cx.retirement_should_yield()||!super::plugin_grant_funds(demand,grant){return Ok(PluginLifecycleStep::Progress(Default::default()))}let before=cx.retained_progress();let reserved_before=cx.reserved_return_progress();let fuel=cx.fuel_remaining();let result=advance(cx,grant);if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}let(step,terminal)=result.map_err(|failure|debit_unposted(failure,before,cx))?;let progress=step.progress().unwrap_or_default();
 if !progress.fits(grant){return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle receipt exceeds its exact phase loan").with_retained_progress(progress)),before,cx))}
 if matches!(step,PluginLifecycleStep::Complete(_))&&!terminal{return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle completed without its terminal-empty witness").with_retained_progress(progress)),before,cx))}
 let remaining=original_lifecycle_unposted(progress,before,original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress()))?;cx.consume_retained(remaining).map_err(|error|PluginLifecycleFailure::from(error.with_retained_progress(progress)))?;Ok(step)
}

/// 🪙️ Like [`original_lifecycle_step`] but the owner runs inside the context's own performed-work effect; the performed work is debited once here, and a refused quote runs nothing and charges nothing.
pub fn original_lifecycle_effect_step(cx:&mut StepContext<'_>,loan:RetainedCloneGrant,demand:RetirementDemand,advance:impl FnOnce(&mut StepContext<'_>,&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(PluginLifecycleStep,bool),PluginLifecycleFailure>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
 let grant=original_lifecycle_grant(cx,loan);if cx.retirement_should_yield()||!super::plugin_grant_funds(demand,grant){return Ok(PluginLifecycleStep::Progress(Default::default()))}
 let before=cx.retained_progress();let reserved_before=cx.reserved_return_progress();let fuel=cx.fuel_remaining();
 let Some((result,performed))=cx.with_original_effect(grant,advance)?else{return Ok(PluginLifecycleStep::Progress(Default::default()))};
 cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
 let(step,terminal)=result.map_err(|failure|debit_unposted(failure,before,cx))?;let progress=step.progress().unwrap_or_default();
 if !progress.fits(grant){return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle receipt exceeds its exact phase loan").with_retained_progress(progress)),before,cx))}
 if matches!(step,PluginLifecycleStep::Complete(_))&&!terminal{return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle completed without its terminal-empty witness").with_retained_progress(progress)),before,cx))}
 let remaining=original_lifecycle_unposted(progress,before,original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress()))?;cx.consume_retained(remaining).map_err(|error|PluginLifecycleFailure::from(error.with_retained_progress(progress)))?;Ok(step)
}

/// 🧾️ Folds one finished retirement poll into the plain step lifecycle closers return; the effect already recorded the work and a pending poll made no physical progress.
pub fn plugin_settle_poll(poll:semio_framework_value::OriginalRetirementPoll)->Result<semio_framework_value::RetainedCloneStep,PluginLifecycleFailure>{
 match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(PluginLifecycleFailure::from)}
}

/// 🧾️ Projects the first original cause of a retirement failure into one value refusal, keeping its physical receipt.
pub fn plugin_failure_error(failure:semio_framework_value::OriginalRetirementFailure)->ValueError{
 let progress=failure.retained_progress();let(primary,work,receipt,allocation,child,_)=failure.into_parts();
 primary.or(work).or(receipt).or_else(||child.map(|child|plugin_failure_error(*child))).unwrap_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,if allocation.is_some(){"original retirement allocation was refused"}else{"original retirement failed without a cause"})).with_retained_progress(progress)
}

/// 🧾️ Folds one finished retirement poll into a plain value step; a pending poll made no physical progress.
pub fn plugin_settle_value(poll:semio_framework_value::OriginalRetirementPoll)->Result<semio_framework_value::RetainedCloneStep,ValueError>{
 match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(plugin_failure_error)}
}

/// ⏱️ Records one pure retirement action on the effect; a refused unit is a typed work limit.
pub fn plugin_pay_unit(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(),ValueError>{
 if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original lifecycle action exceeds its work grant"))}
}

/// 🌉️ Runs one still grant-shaped closer under the effect's grant and records its reported work on the effect exactly once.
pub fn plugin_grant_step<E:From<ValueError>>(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,close:impl FnOnce(RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep,E>)->Result<semio_framework_value::RetainedCloneStep,E>{
 let turn=close(effect.grant());let work=u64::try_from(turn.work).unwrap_or(u64::MAX);
 match effect.performed(work){Ok(true)=>turn.result,Ok(false)=>turn.result.and_then(|_|Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original lifecycle closer exceeds its work grant").into())),Err(error)=>Err(error.into())}
}

/// 🌉️ Runs one pure grant-shaped closer that reports no work and records one unit on the effect.
pub fn plugin_unit_step<T,E:From<ValueError>>(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,close:impl FnOnce(RetainedCloneGrant)->Result<T,E>)->Result<T,E>{
 let result=close(effect.grant());
 match effect.performed(1){Ok(true)=>result,Ok(false)=>result.and_then(|_|Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original lifecycle closer exceeds its work grant").into())),Err(error)=>Err(error.into())}
}

/// 🪙️ Runs one effect-shaped closer inside the context's own performed-work effect and debits its performed work once; a refused quote advances nothing.
pub fn plugin_effect_close<E:From<ValueError>>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,close:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<semio_framework_value::RetainedCloneStep,E>)->Result<semio_framework_value::RetainedCloneStep,E>{
 match cx.with_original_effect(grant,|_,effect|close(effect)).map_err(E::from)?{
  Some((result,performed))=>{cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));result}
  None=>Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))
 }
}

/// 🪙️ Runs one plain-result owner inside the context's own performed-work effect and posts its receipt and performed work through the original receiving debit once; a yielding turn or refused quote advances nothing.
pub fn plugin_effect_turn<E:Into<semio_framework_job::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,close:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<semio_framework_value::RetainedCloneStep,E>)->Result<semio_framework_value::RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
 if cx.retirement_should_yield(){return Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))}
 let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|close(effect))? else{return Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))};
 semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:performed})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl From<PluginLifecyclePrimary> for PluginLifecycleFailure{fn from(primary:PluginLifecyclePrimary)->Self{let effects=primary.retained_progress();Self{effects,primary,receipt_refusal:None,context_refusal:None,work_refusal:None}}}

/// 📏️ The largest inline frame a retained command work may have: its terminal release must fit one close grant, so large owners are boxed or paged and never carried inline.
pub const TERMINAL_FRAME_GUARD_BYTES:usize=64*1024;
/// 📏️ Why a quoted terminal frame was refused.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WorkFrameRefusal{Unadmitted,Oversized}
/// 📏️ The pure guard of the inline frame: an absent quote is unadmitted, a quote above the maximum is oversized.
pub fn checked_work_frame_bytes(quote:Option<usize>)->Result<usize,WorkFrameRefusal>{match quote{None=>Err(WorkFrameRefusal::Unadmitted),Some(bytes)if bytes>TERMINAL_FRAME_GUARD_BYTES=>Err(WorkFrameRefusal::Oversized),Some(bytes)=>Ok(bytes)}}
/// 🧪️ Size probe shared by every plugin's command-work census: panics with the measured size when a work frame would exceed [`TERMINAL_FRAME_GUARD_BYTES`].
pub fn assert_work_frame_fits<W>(){let bytes=std::mem::size_of::<W>();assert!(bytes<=TERMINAL_FRAME_GUARD_BYTES,"{} is {bytes} B inline, above the {TERMINAL_FRAME_GUARD_BYTES} B work frame bound",std::any::type_name::<W>());}
