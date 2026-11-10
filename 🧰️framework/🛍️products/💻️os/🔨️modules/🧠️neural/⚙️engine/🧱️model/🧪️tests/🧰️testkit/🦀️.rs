//! 🧰️ Test kit of the neural engine producers: the ownership observation scope over the one shared value test allocator.

/// 🔭️ Observes the requested and released heap bytes of one operation; a scope inside an open scope measures by difference.
pub fn observe_ownership<T>(operation:impl FnOnce()->T)->(T,usize,usize) {
    use semio_framework_value_testing_allocator::{observe_retirement_allocations,retirement_allocations_so_far};
    let Some((allocated,released))=retirement_allocations_so_far() else {
        let (value,(allocated,released))=observe_retirement_allocations(operation);
        return (value,allocated,released);
    };
    let value=operation();
    let (after_allocated,after_released)=retirement_allocations_so_far().expect("the enclosing observation scope stays open");
    (value,after_allocated-allocated,after_released-released)
}

use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,StepContext};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,ValueError,ValueRefusalKind};

const CALLER_STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:64};

/// 🏦️ Funds raw effect-shaped owner steps through genuine original caller turns, so turn-less suites never author a wallet.
pub struct EffectCaller{inner:Option<OriginalJobFixtureCaller>}
impl Default for EffectCaller{fn default()->Self{Self::new()}}
impl EffectCaller{
    pub fn new()->Self{Self{inner:Some(OriginalJobFixtureCaller::admit(OperationId(707),Generation(9),CALLER_STOCK,||Some(1),u64::MAX,1<<40))}}
    /// 🧾️ Reports the physical receipt of the last turn this caller funded.
    pub fn last(&self)->RetainedCloneProgress{self.inner.as_ref().expect("live caller").receipt()}
    /// 🪙️ Runs one original step under a loan of exactly `work` fuel and takes its closure result.
    pub fn turn<T>(&mut self,grant:RetainedCloneGrant,work:u64,step:impl FnOnce(&mut StepContext<'_>)->T)->T {
        let caller=self.inner.as_mut().expect("live caller");let mut out=None;
        caller.turn(grant,|cx|cx.with_original_work_loan(work,u64::MAX,|cx|{out=Some(step(cx));Ok(())})).unwrap_or_else(|_|panic!("original fixture turn"));
        out.expect("original fixture step ran")
    }
    /// 🪙️ Runs one effect-shaped close or advance as the single original effect of a funded turn and charges its receipt to that turn.
    pub fn poll(&mut self,grant:RetainedCloneGrant,work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError> {
        self.turn(grant,work,|cx|{
            let Some((poll,_))=cx.with_original_effect(grant,|_,effect|operation(effect)).expect("original fixture effect")else{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"fixture work quote was not admitted"))};
            match poll {
                RetainedWorkPoll::Pending{..}=>panic!("synchronous fixture suspended"),
                RetainedWorkPoll::Complete(turn)=>match turn.result {
                    Ok(step)=>{cx.consume_retained(step.progress())?;Ok(step)},
                    Err(failure)=>{let progress=failure.retained_progress();cx.consume_retained(progress)?;Err(failure.into_value_error())},
                },
            }
        })
    }
}
impl EffectCaller {
    /// 🪙️ Runs one effect-shaped result step as the single original effect of a funded turn and charges its receipt to that turn.
    pub fn result(&mut self,grant:RetainedCloneGrant,work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneStep,ValueError> {
        self.turn(grant,work,|cx|{
            let Some((result,_))=cx.with_original_effect(grant,|_,effect|operation(effect)).expect("original fixture effect")else{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"fixture work quote was not admitted"))};
            cx.consume_retained(match &result {Ok(step)=>step.progress(),Err(error)=>error.retained_progress()})?;result
        })
    }
}
impl Drop for EffectCaller{fn drop(&mut self){if let Some(caller)=self.inner.take(){caller.retire(CALLER_STOCK);}}}
