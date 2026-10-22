//! 🪢️ Whole original lifecycle refusals retain their semantic source and receipt refusal separately.
use semio_framework::Fault;
use semio_framework_job::StepContext;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use super::PluginLifecycleStep;

/// 🧳️ The original diagnostic or value refusal moves without cloning or prose conversion.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum PluginLifecyclePrimary{Fault(Fault),Value(ValueError),Domain(semio_framework_job::OriginalDomainPrimary),Receiving(semio_framework_job::OriginalReceivingPrimary)}

/// 🧯️ The complete original cause and an independent refusal to debit its physical effects.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct PluginLifecycleFailure{pub effects:RetainedCloneProgress,pub primary:PluginLifecyclePrimary,pub receipt_refusal:Option<ValueError>,pub context_refusal:Option<ValueError>}
impl PluginLifecycleFailure{
 /// 🧾️ Borrows the actual failed producer effects carried by its complete original cause.
 pub fn retained_progress(&self)->RetainedCloneProgress{self.effects}
 /// 🪙️ Debits one raw producer refusal through the same initiating context before publication.
 pub fn debit_original(mut self,cx:&mut StepContext<'_>)->Self{if self.receipt_refusal.is_none(){self.receipt_refusal=cx.consume_retained(self.retained_progress()).err()}self}
}
impl From<Fault> for PluginLifecycleFailure{fn from(primary:Fault)->Self{Self{effects:primary.retained_progress(),primary:PluginLifecyclePrimary::Fault(primary),receipt_refusal:None,context_refusal:None}}}
impl From<ValueError> for PluginLifecycleFailure{fn from(primary:ValueError)->Self{Self{effects:primary.retained_progress(),primary:PluginLifecyclePrimary::Value(primary),receipt_refusal:None,context_refusal:None}}}
impl From<semio_framework_job::OriginalTurnFailure> for PluginLifecycleFailure{
 fn from(failure:semio_framework_job::OriginalTurnFailure)->Self{Self{effects:failure.effects,primary:match failure.primary{semio_framework_job::OriginalTurnPrimary::Value(error)=>PluginLifecyclePrimary::Value(error),semio_framework_job::OriginalTurnPrimary::Fault(fault)=>PluginLifecyclePrimary::Fault(fault),semio_framework_job::OriginalTurnPrimary::Domain(original)=>PluginLifecyclePrimary::Domain(original),semio_framework_job::OriginalTurnPrimary::Receiving(original)=>PluginLifecyclePrimary::Receiving(original)},receipt_refusal:failure.receipt_refusal,context_refusal:failure.context_refusal}}
}
impl From<PluginLifecycleFailure> for semio_framework_job::OriginalTurnFailure{
 fn from(failure:PluginLifecycleFailure)->Self{Self{effects:failure.effects,primary:match failure.primary{PluginLifecyclePrimary::Value(error)=>semio_framework_job::OriginalTurnPrimary::Value(error),PluginLifecyclePrimary::Fault(fault)=>semio_framework_job::OriginalTurnPrimary::Fault(fault),PluginLifecyclePrimary::Domain(original)=>semio_framework_job::OriginalTurnPrimary::Domain(original),PluginLifecyclePrimary::Receiving(original)=>semio_framework_job::OriginalTurnPrimary::Receiving(original)},receipt_refusal:failure.receipt_refusal,context_refusal:failure.context_refusal}}
}

/// 🎟️ A phase loan narrows the independently remaining grant of its actual initiating context.
pub fn original_lifecycle_grant(cx:&StepContext<'_>,loan:RetainedCloneGrant)->RetainedCloneGrant{
 let remaining=cx.retained_grant();RetainedCloneGrant{maximum_items:remaining.maximum_items.min(loan.maximum_items).min(1),maximum_copy_bytes:remaining.maximum_copy_bytes.min(loan.maximum_copy_bytes),maximum_capacity_bytes:remaining.maximum_capacity_bytes.min(loan.maximum_capacity_bytes),maximum_release_bytes:remaining.maximum_release_bytes.min(loan.maximum_release_bytes),maximum_depth:remaining.maximum_depth.min(loan.maximum_depth)}
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
 let grant=original_lifecycle_grant(cx,loan);if cx.should_yield()||!super::plugin_grant_funds(demand,grant){return Ok(PluginLifecycleStep::Progress(Default::default()))}let before=cx.retained_progress();let fuel=cx.fuel_remaining();let result=advance(cx,grant);if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}let(step,terminal)=result.map_err(|failure|debit_unposted(failure,before,cx))?;let progress=step.progress().unwrap_or_default();
 if !progress.fits(grant){return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle receipt exceeds its exact phase loan").with_retained_progress(progress)),before,cx))}
 if matches!(step,PluginLifecycleStep::Complete(_))&&!terminal{return Err(debit_unposted(PluginLifecycleFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original lifecycle completed without its terminal-empty witness").with_retained_progress(progress)),before,cx))}
 let remaining=original_lifecycle_unposted(progress,before,cx.retained_progress())?;cx.consume_retained(remaining).map_err(|error|PluginLifecycleFailure::from(error.with_retained_progress(progress)))?;Ok(step)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl From<PluginLifecyclePrimary> for PluginLifecycleFailure{fn from(primary:PluginLifecyclePrimary)->Self{let effects=match &primary{PluginLifecyclePrimary::Fault(original)=>original.retained_progress(),PluginLifecyclePrimary::Value(original)=>original.retained_progress(),PluginLifecyclePrimary::Domain(original)=>original.retained_progress(),PluginLifecyclePrimary::Receiving(original)=>original.retained_progress()};Self{effects,primary,receipt_refusal:None,context_refusal:None}}}
