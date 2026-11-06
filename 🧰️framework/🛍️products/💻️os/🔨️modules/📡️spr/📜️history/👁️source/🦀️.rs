//! 👁️ One immutable original history source owns the backing until every controlled fold alias closes.
use super::{HistoryLog, RetainedHistoryFold};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::arc_custody::{SharedControlledRetirement, arc_bytes}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeConstructionStepOutcome, native_decoding::NativeConstructionStepResult, native_decoding::NativeDecodeCloseControl, native_decoding::OriginalReceivingPrimary};
use std::{mem::{ManuallyDrop, size_of}, sync::Arc};

pub struct OriginalHistorySource {
    source: ManuallyDrop<Option<Arc<HistoryLog>>>,
    closing: ManuallyDrop<Option<SharedControlledRetirement<HistoryLog>>>,
    binding: (u64, Option<u64>),
}

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }
fn refused<T>(error: ValueError) -> NativeConstructionStepOutcome<T, OriginalReceivingPrimary> { NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() } }

fn source_birth_outcome(turn:semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>,ready:bool)->NativeConstructionStepResult<bool,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>{
 let progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|failure|failure.retained_progress());
 NativeConstructionStepResult{result:turn.result.map(|_|ready),progress,work:turn.work}
}

impl OriginalHistorySource {
    /// 📐️ The defining Arc layout and fixed owner moves are quoted before consuming the original history.
    pub fn birth_demand() -> RetirementDemand { RetirementDemand { copy_bytes: (2 * size_of::<Self>() + size_of::<HistoryLog>()).max(2 * size_of::<HistoryLog>()), capacity_bytes: arc_bytes::<HistoryLog>(), depth: 1, ..Default::default() } }

    /// ⛽️ Bounds input transfer, allocator attempt and output publication or refused input restoration.
    pub fn birth_work_demand()->u64{3}

    /// 🫴️ Keeps allocation and receipt callbacks in the supplied resident frame until original settlement.
    pub fn admit(history:&mut Option<HistoryLog>,output:&mut Option<Self>,frame:&mut semio_framework_value_native_owned::native_decoding::NativeReceivingEffect<ValueError>,native:&mut NativeDecodeControl<'_>)->Result<NativeConstructionStepResult<bool,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>,OriginalReceivingPrimary>{
        use semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase as Phase;
        let demand=Self::birth_demand();
        if frame.phase()==Phase::Empty{
            if history.is_none()||output.is_some(){return Err(invalid("original history source requires an occupied input and empty destination").into())}
            frame.quote(native,demand)?;
        }else if !frame.matches_demand(demand){return Err(invalid("original history source changed its resident birth demand").into())}
        if frame.phase()==Phase::Granted{
            match native.prepare_work(Self::birth_work_demand()){
                Ok(true)=>{},
                Ok(false)=>{frame.cancel_unstarted(native)?;return frame.take(native).map(|turn|source_birth_outcome(turn,false)).map_err(Into::into)},
                Err(error)=>{frame.record_work_refusal(native,error).map_err(|(state,refusal)|{let mut failure=OriginalReceivingPrimary::from(state);failure.work_refusal=Some(refusal);failure})?;frame.cancel_unstarted(native)?;return frame.take(native).map(|turn|source_birth_outcome(turn,false)).map_err(Into::into)}
            }
            let binding=(native.ledger_identity(),native.retirement_recipient_identity());
            frame.effect(native,|actual,grant|{
                let Some(original)=history.take()else{return semio_framework_value::RetainedWorkOutcome{result:Err(invalid("original History source input vanished before its quoted birth")),work:0}};
                let turn=semio_framework_value::retirement::arc_custody::admit_shared_allocation(original,grant,actual);
                let work=turn.work+2;
                let result=match turn.result{
                    Ok((source,mut progress))=>{*output=Some(Self{source:ManuallyDrop::new(Some(source)),closing:ManuallyDrop::new(None),binding});progress.copied_bytes=2*size_of::<Self>()+size_of::<HistoryLog>();Ok(RetainedCloneStep::Progress(progress))},
                    Err((error,original))=>{*history=Some(original);Err(error.with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<HistoryLog>(),..Default::default()}))}
                };
                semio_framework_value::RetainedWorkOutcome{result,work}
            })?;
        }
        if frame.phase()==Phase::EffectStarted{frame.finish_allocation(native)?}
        if frame.phase()==Phase::EffectFinished{
            if frame.work_commit_is_pending(){frame.retry_original_work(native)?}
            if !frame.work_is_settled(){
                if let Some(work)=frame.child_work(){frame.settle_original_work(native,work)?}else{frame.cancel_unperformed_work(native)?}
            }
            frame.receipt(native)?;
        }
        if frame.phase()!=Phase::Settled{return Err(invalid("original History source requires its interrupted original callback verdict").into())}
        frame.take(native).map(|turn|source_birth_outcome(turn,output.is_some())).map_err(Into::into)
    }

    fn validate(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.binding != (native.ledger_identity(), native.retirement_recipient_identity()) { return Err(invalid("history source changed its original receiving authority")); }
        Ok(())
    }

    /// 📖️ Readers borrow the retained immutable root without acquiring an unpriced alias.
    pub fn history(&self) -> Option<&HistoryLog> { self.source.as_deref() }

    /// 🎞️ The sole fold alias is born inside the actual funded coroutine constructor.
    pub fn admit_fold(&self, shape: protocol::HistoryShape, output: &mut Option<protocol::HistoryFoldJob<'static, RetainedHistoryFold>>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        let Some(source) = self.source.as_ref() else { return refused(invalid("history source is already closing")); };
        let mut runner = Some(|control| { let source = Arc::clone(source); async move { source.fold_controlled(shape, &control).await } });
        protocol::HistoryFoldJob::admit_original(&mut runner, output, native)
    }

    /// 📬️ Only the final unique owner can transfer the same history after every fold alias has closed.
    pub fn take_history(&mut self, output: &mut Option<HistoryLog>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        if output.is_some() || self.closing.is_some() || self.source.is_none() { return refused(invalid("original history handoff requires the resident root and an empty destination")); }
        let source = self.source.as_ref().unwrap();
        if Arc::strong_count(source) != 1 || Arc::weak_count(source) != 0 { return NativeConstructionStepOutcome { work: 0, result: None, receipt_refusal: None, progress: Default::default() }; }
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<HistoryLog>() + size_of::<Option<Arc<HistoryLog>>>(), release_bytes: arc_bytes::<HistoryLog>(), depth: 1, ..Default::default() };
        native.receive_construction_step_typed(demand, |_, _| {
            match Arc::try_unwrap(self.source.take().unwrap()) {
                Ok(history) => { *output = Some(history); NativeConstructionStepResult { work: 1, result: Ok(true), progress: RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() } } }
                Err(source) => { *self.source = Some(source); NativeConstructionStepResult { work: 0, result: Ok(false), progress: Default::default() } }
            }
        })
    }

    /// 📏️ Shared-source retirement starts with a heapless, separately quoted fixed owner move.
    pub fn next_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.source.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<HistoryLog>>() + size_of::<SharedControlledRetirement<HistoryLog>>(), depth: 1, ..Default::default() }); }
        self.closing.as_ref().map_or(Ok(Default::default()), |owner| if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SharedControlledRetirement<HistoryLog>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)})
    }

    /// ♻️ Cancellation borrows the original close authority and waits for live fold aliases before releasing history.
    pub fn close_original(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome {
        let mut work=0;
        let result=(||{
        self.validate(native)?;
        let demand = self.next_close_demand(grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(source) = self.source.take() {
            work=1;
            *self.closing = Some(SharedControlledRetirement::new(source));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.closing.as_mut() {
            if owner.terminal_is_empty(){work=1;*self.closing=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            native.charge(demand.capacity_bytes)?;
            let result=owner.step(grant);work=owner.last_turn_work();
            return result.map(|step|RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
        })();semio_framework_value::RetainedWorkOutcome{work,result}
    }

    pub fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.closing.is_none() }
}

impl Drop for OriginalHistorySource {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original history source abandoned its shared allocation");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.source); ManuallyDrop::drop(&mut self.closing); } }
    }
}
