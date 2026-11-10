//! 👁️ One immutable original history source owns the backing until every controlled fold alias closes.
use super::{HistoryLog, RetainedHistoryFold};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::arc_custody::{SharedControlledRetirement, original::{SharedOriginalRelease, SharedOriginalStorage}}};
use semio_framework_value_native_owned::{native_decoding::OriginalEffectReply, native_decoding::NativeDecodeControl, native_decoding::NativeConstructionStepOutcome, native_decoding::NativeConstructionStepResult, native_decoding::NativeDecodeCloseControl, native_decoding::OriginalReceivingPrimary};
use std::mem::{ManuallyDrop, size_of};

pub struct OriginalHistorySource {
    source: ManuallyDrop<Option<SharedOriginalStorage<HistoryLog>>>,
    closing: ManuallyDrop<Option<SharedControlledRetirement<HistoryLog>>>,
    binding: (semio_framework_value::retained_clone::NativeAllocationIdentity, Option<u64>),
}

/// 🫂️ One counted fold alias of the issued history; it retires its count when the fold future ends, and the source waits until every alias has retired.
struct HistoryAlias(Option<SharedOriginalStorage<HistoryLog>>);
impl HistoryAlias { fn get(&self) -> &HistoryLog { self.0.as_ref().expect("live history alias").get() } }
impl Drop for HistoryAlias { fn drop(&mut self) { if let Some(alias) = self.0.take() { if let SharedOriginalRelease::Original(last) = alias.retire() { last.abandon() } } } }

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }
fn refused<T>(error: ValueError) -> NativeConstructionStepOutcome<T, OriginalReceivingPrimary> { NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() } }

fn source_birth_outcome(turn:semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>,ready:bool)->NativeConstructionStepResult<bool,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>{
 let progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|failure|failure.retained_progress());
 NativeConstructionStepResult{result:turn.result.map(|_|ready),progress,work:turn.work}
}

impl OriginalHistorySource {
    /// 📐️ The issued allocation, its typed placement and the fixed owner moves are quoted before consuming the original history.
    pub fn birth_demand() -> RetirementDemand {
        let mut demand = SharedOriginalStorage::<HistoryLog>::source_admission_demand().expect("history source admission extent");
        demand.copy_bytes += 2 * size_of::<Self>();
        demand
    }

    /// ⛽️ Bounds input transfer, allocator attempt and output publication.
    pub fn birth_work_demand() -> u64 { SharedOriginalStorage::<HistoryLog>::source_admission_work_demand() + 1 }

    /// 🫴️ Keeps allocation and receipt callbacks in the supplied resident frame until original settlement.
    pub fn admit(history:&mut Option<HistoryLog>,output:&mut Option<Self>,frame:&mut semio_framework_value_native_owned::native_decoding::NativeReceivingEffect<ValueError>,native:&mut NativeDecodeControl<'_>)->Result<NativeConstructionStepResult<bool,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>,OriginalReceivingPrimary>{
        use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffectPhase as Phase,NativeReceivingEffectPoll};
        if frame.phase()==Phase::Empty&&(history.is_none()||output.is_some()){return Err(invalid("original history source requires an occupied input and empty destination").into())}
        let binding=(native.ledger_identity(),native.retirement_recipient_identity());
        let poll=native.receive_effect(Self::birth_demand(),Self::birth_work_demand(),frame,|_,effect|{
            let outcome=|result,effect:&semio_framework_value::RetainedAllocationEffect<'_>|semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()};
            if history.is_none(){return outcome(Err(invalid("original History source input vanished before its quoted birth")),effect)}
            if effect.remaining_work_units().map_or(true,|remaining|remaining<Self::birth_work_demand()){return outcome(Ok(RetainedCloneStep::Progress(Default::default())),effect)}
            let mut issued=None;
            let turn=SharedOriginalStorage::admit_from(history,&mut issued,effect);
            let result=match(turn.result,issued){
                (Ok(step),Some(source))=>{
                    let mut progress=step.progress();progress.copied_bytes+=2*size_of::<Self>();
                    *output=Some(Self{source:ManuallyDrop::new(Some(source)),closing:ManuallyDrop::new(None),binding});
                    match effect.performed(1){Ok(true)=>Ok(RetainedCloneStep::Progress(progress)),Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original History source birth exceeded its work grant").with_retained_progress(progress)),Err(error)=>Err(error.with_retained_progress(progress))}
                },
                (Ok(step),None)=>Ok(RetainedCloneStep::Progress(step.progress())),
                (Err(error),_)=>Err(error),
            };
            outcome(result,effect)
        })?;
        Ok(match poll{
            NativeReceivingEffectPoll::Complete(turn)=>source_birth_outcome(turn,output.is_some()),
            NativeReceivingEffectPoll::Pending{work}=>NativeConstructionStepResult{result:Ok(false),progress:Default::default(),work},
        })
    }

    fn validate(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.binding != (native.ledger_identity(), native.retirement_recipient_identity()) { return Err(invalid("history source changed its original receiving authority")); }
        Ok(())
    }

    /// 📖️ Readers borrow the retained immutable root without acquiring an unpriced alias.
    pub fn history(&self) -> Option<&HistoryLog> { self.source.as_ref().map(SharedOriginalStorage::get) }

    /// 🎞️ The sole fold alias is born inside the actual funded coroutine constructor.
    pub fn admit_fold(&self, shape: protocol::HistoryShape, output: &mut Option<protocol::HistoryFoldJob<'static, RetainedHistoryFold>>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        let Some(source) = self.source.as_ref() else { return refused(invalid("history source is already closing")); };
        let mut runner = Some(|control| { let alias = HistoryAlias(Some(source.alias())); async move { alias.get().fold_controlled(shape, &control).await } });
        protocol::HistoryFoldJob::admit_original(&mut runner, output, native)
    }

    /// 📬️ Only the final unique owner can transfer the same history after every fold alias has closed.
    pub fn take_history(&mut self, output: &mut Option<HistoryLog>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        if output.is_some() || self.closing.is_some() || self.source.is_none() { return refused(invalid("original history handoff requires the resident root and an empty destination")); }
        let source = self.source.as_ref().unwrap();
        if !source.physically_unique() { return NativeConstructionStepOutcome { work: 0, result: None, receipt_refusal: None, progress: Default::default() }; }
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<HistoryLog>() + size_of::<Option<SharedOriginalStorage<HistoryLog>>>(), release_bytes: SharedOriginalStorage::<HistoryLog>::allocation_bytes(), depth: 1, ..Default::default() };
        native.receive_construction_step_typed(demand, |_, _| {
            match self.source.take().unwrap().try_owner() {
                Ok(mut owner) => { *output = owner.take_value(); NativeConstructionStepResult { work: 1, result: Ok(output.is_some()), progress: RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() } } }
                Err(source) => { *self.source = Some(source); NativeConstructionStepResult { work: 0, result: Ok(false), progress: Default::default() } }
            }
        })
    }

    /// 📏️ Shared-source retirement starts with a heapless, separately quoted fixed owner move.
    pub fn next_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.source.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<SharedOriginalStorage<HistoryLog>>() + size_of::<SharedControlledRetirement<HistoryLog>>(), depth: 1, ..Default::default() }); }
        self.closing.as_ref().map_or(Ok(Default::default()), |owner| if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SharedControlledRetirement<HistoryLog>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)})
    }

    /// ♻️ Cancellation borrows the original close authority and waits for live fold aliases before releasing history.
    pub fn close_original(&mut self, native: &mut dyn NativeDecodeCloseControl, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::RetainedWorkOutcome {
        let grant = effect.grant();
        let mut work = 0;
        let result = (|| {
            self.validate(native)?;
            let demand = self.next_close_demand(grant.maximum_copy_bytes)?;
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if self.source.as_ref().is_some() {
                if !self.source.as_ref().unwrap().physically_unique() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                work = 1;
                *self.closing = Some(SharedControlledRetirement::new(self.source.take().unwrap()));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            if let Some(owner) = self.closing.as_mut() {
                if owner.terminal_is_empty() { work = 1; *self.closing = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
                let before = effect.performed_work();
                let mut answer = None;
                drop(owner.step_into(grant.maximum_copy_bytes, &mut effect.nested(grant), OriginalEffectReply::borrow(&mut answer)?)?);
                work = effect.performed_work() - before;
                return answer.ok_or_else(|| invalid("history source retirement suspended inside a native action"))?.map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|failure| { let (primary, work, receipt, ..) = failure.into_parts(); primary.or(work).or(receipt).unwrap_or_else(|| invalid("history source retirement failed without a cause")) });
            }
            Ok(RetainedCloneStep::Complete(Default::default()))
        })();
        semio_framework_value::RetainedWorkOutcome { work, result }
    }

    pub fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.closing.is_none() }
}

impl Drop for OriginalHistorySource {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original history source abandoned its shared allocation");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.source); ManuallyDrop::drop(&mut self.closing); } }
    }
}
