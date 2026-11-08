//! 🔤️ Every value-projectable presence owns one canonical JSON native codec under the original snapshot owners.
use semio_framework_io_control::{native_snapshot_turn, NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure, NativeSnapshotWork};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonError, JsonGrammarCursor, JsonMemberPolicy};
use semio_framework_value::{retirement::RetireOwned, DslValue, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ToValue, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl, NativeReceivingEffect, NativeReceivingEffectPhase, NativeReceivingEffectPoll};

const TURN_UNITS: usize = 1024;

fn unfunded() -> NativeSnapshotFailure { ValueError::literal(ValueRefusalKind::WorkLimit, "presence original effect work quote was not admitted").into() }
fn stalled() -> NativeSnapshotFailure { ValueError::literal(ValueRefusalKind::WorkLimit, "presence admission made no progress under its own demand").into() }
fn pending_refusal() -> NativeSnapshotFailure { pending() }
fn pending() -> NativeSnapshotFailure { ValueError::literal(ValueRefusalKind::WorkLimit, "presence receiving effect stayed pending on a synchronous recipient").into() }
type EffectFrame = NativeReceivingEffect<ValueError>;
fn effect_step<E: Into<NativeSnapshotFailure>>(native: &mut NativeDecodeControl<'_>, frame: &mut EffectFrame, work: &mut NativeSnapshotWork, body: &NativeSnapshotBodyWallet, demand: RetirementDemand, operation: impl FnOnce(&mut NativeDecodeControl<'_>, &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<RetainedCloneStep, E>) -> Result<Option<RetainedCloneStep>, NativeSnapshotFailure> {
    admit_turn(body, demand)?;
    let mut refused = None;
    let polled = native.receive_effect(demand, TURN_UNITS as u64, frame, |control, effect| {
        let turn = operation(control, effect);
        let result = turn.result.map_err(|error| { refused = Some(error.into()); ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence original step refused") });
        RetainedWorkOutcome { result, work: effect.performed_work() }
    })?;
    match polled {
        NativeReceivingEffectPoll::Complete(turn) => work.take(RetainedWorkOutcome { result: turn.result.map_err(|failure| refused.take().unwrap_or_else(|| failure.receiving.into())), work: turn.work }).map(Some),
        NativeReceivingEffectPoll::Pending { work: 0 } if frame.phase() == NativeReceivingEffectPhase::Granted => { frame.cancel_unstarted(native)?; Ok(None) }
        NativeReceivingEffectPoll::Pending { .. } => Err(pending()),
    }
}
fn effect_poll<E: Into<NativeSnapshotFailure>>(native: &mut NativeDecodeControl<'_>, frame: &mut EffectFrame, work: &mut NativeSnapshotWork, body: &NativeSnapshotBodyWallet, demand: RetirementDemand, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<RetainedCloneStep, E>) -> Result<Option<RetainedCloneStep>, NativeSnapshotFailure> {
    effect_step(native, frame, work, body, demand, |_, effect| match operation(effect) {
        RetainedWorkPoll::Complete(turn) => RetainedWorkOutcome { result: turn.result.map_err(Into::<NativeSnapshotFailure>::into), work: turn.work },
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Err(pending()), work },
    })
}

fn admit_inline<T>(body: &NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<T>(), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 }) }
fn record_inline<T>(body: &mut NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() }) }
fn admit_turn(body: &NativeSnapshotBodyWallet, demand: RetirementDemand) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }) }
fn ceiling(owned: usize, body: &NativeSnapshotBodyWallet) -> Result<usize, ValueError> { owned.checked_add(body.remaining_grant().maximum_capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON capacity ceiling overflow")) }
fn typed_receipt<T>(charged: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: charged, released_bytes: 0 } }

/// 🛬️ Parses the borrowed presence pack through the original grammar cursor, then binds it into the original typed owner.
pub fn decode_presence_json<P: semio_framework_value_native_owned::native_decoding::NativeReceive<DslValue>>(bytes:&[u8],owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<P,NativeSnapshotFailure>{
    use semio_framework_value::retained_clone::RetainedCloneSource;
    use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
    type Source=RetainedCloneSource<DslValue>;
    type Receiving<P>=(JsonGrammarCursor<DslValue>,Option<DslValue>,Option<JsonError>,Option<NativeReceivingOriginal<DslValue,P>>,Option<Source>,EffectFrame);
    let Ok(text)=std::str::from_utf8(bytes) else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"presence pack is not UTF-8 JSON").into()),work:0}};
    let mut output=None;
    let turn=owner.receive::<Receiving<P>,P,NativeSnapshotFailure>(&mut output,&mut |slot,result,_marker,_failure,native,body|native_snapshot_turn(|work|{
        admit_inline::<Receiving<P>>(body)?;*slot=Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject),None,None,None,None,EffectFrame::default()));record_inline::<Receiving<P>>(body)?;
        let(cursor,tree,refusal,receiving,_,effects)=slot.as_mut().ok_or_else(||ValueError::invariant("slot absent"))?;
        loop{
            let demand=match cursor.normal_step_demands(text){Ok(demand)=>demand,Err(JsonError::Native(error))=>return Err(error.into()),Err(error)=>{let kind=error.kind();*refusal=Some(error);return Err(ValueError::literal(kind,"presence JSON source was refused").into())}};
            let mut result=Ok(None);effect_step(native,effects,work,body,demand,|control,effect|{result=cursor.step(text,1,control,effect);RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneStep::Progress(Default::default())),work:effect.performed_work()}})?.ok_or_else(unfunded)?;let performed=cursor.normal_step_progress();
            let ready=match result{Ok(Some(value))=>{*tree=Some(value);Ok(true)},Ok(None)=>Ok(false),Err(JsonError::Native(error))=>Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);Err(ValueError::literal(kind,"presence JSON grammar refused the original pack"))}};body.record_progress(performed)?;
            if ready?{break;}if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"presence JSON parser has no funded frontier").into());}
        }
        let mut admission=Source::admission_owned(tree.take().ok_or_else(||ValueError::invariant("tree absent"))?,());let mut original=None;
        let admitted=(||->Result<(),NativeSnapshotFailure>{
            let mut idle=0u8;while admission.phase()!="ready"{let phase=admission.phase();let demand=admission.next_demand()?;let step=effect_step(native,effects,work,body,demand,|_,effect|admission.advance(effect))?.ok_or_else(unfunded)?;body.record_progress(step.progress())?;idle=if step.progress()==RetainedCloneProgress::default()&&admission.phase()==phase{idle+1}else{0};if idle>=2{return Err(stalled())}}
            admit_turn(body,admission.next_demand()?)?;let step=work.take(admission.take_output(&mut original,body.remaining_grant()))?;body.record_progress(step.progress())?;Ok(())
        })();
        if let Err(error)=admitted{
            let mut back=None;
            while !admission.terminal_is_empty(){
                let demand=admission.next_return_demand().unwrap_or_default();
                match effect_step(native,effects,work,body,demand,|_,effect|admission.return_inputs(&mut back,effect)){Ok(Some(step))=>{if body.record_progress(step.progress()).is_err(){break}},_=>break}
            }
            *tree=back.map(|(value,())|value);
            return Err(error);
        }
        while !admission.terminal_is_empty(){let demand=admission.next_return_demand()?;let step=effect_step(native,effects,work,body,demand,|_,effect|admission.return_inputs(&mut None,effect))?.ok_or_else(unfunded)?;body.record_progress(step.progress())?;}
        let source=original.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"presence source admission produced no original"))?;
        *receiving=Some(NativeReceivingOriginal::<DslValue,P>::new(source));let frame=receiving.as_mut().ok_or_else(||ValueError::invariant("receiving absent"))?;
        while !frame.output_ready(){
            let logical_body=body.remaining_grant().maximum_copy_bytes;admit_turn(body,frame.demands(logical_body,TURN_UNITS as u64)?)?;let grant=body.remaining_grant();let limit=ceiling(native.owned_bytes(),body)?;
            let poll=native.scoped_maximum::<_,ValueError>(limit,|native|Ok(frame.advance(native,logical_body,TURN_UNITS as u64,grant)))?;
            match poll{
                RetainedWorkPoll::Pending{work:pending}=>{if pending==0{return Err(pending_refusal())}work.take(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:pending})?;},
                RetainedWorkPoll::Complete(turn)=>match work.take(turn){Ok(step)=>body.record_progress(step.progress())?,Err(failure)=>return Err(failure)},
            }
        }
        let demand=frame.take_demand()?;
        match effect_poll(native,effects,work,body,demand,|effect|frame.take(result,effect))?{Some(step)=>body.record_progress(step.progress())?,None=>return Err(unfunded())}
        frame.begin_close();
        while !frame.terminal_is_empty(){let demand=frame.close_demands(body.remaining_grant().maximum_copy_bytes)?;match effect_poll(native,effects,work,body,demand,|effect|frame.close_step(effect))?{Some(step)=>body.record_progress(step.progress())?,None=>return Err(unfunded())}}
        Ok(())
    }));
    RetainedWorkOutcome{result:turn.result.and_then(|()|output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"presence receiving produced no output").into())),work:turn.work}
}

/// 🛫️ Projects the original typed owner into one retained tree and writes its canonical JSON through the original cursor.
pub(super) fn encode_presence_json<P: ToValue + RetireOwned>(presence: &P, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> RetainedWorkOutcome<Vec<u8>, NativeSnapshotFailure> {
    type Receiving = (Option<DslValue>, JsonBorrowedWriteCursor);
    let mut output = None;
    let turn = owner.receive::<Receiving, String, NativeSnapshotFailure>(&mut output, &mut |slot, result, _marker, _failure, native, body| native_snapshot_turn(|work| {
        admit_inline::<Receiving>(body)?;
        *slot = Some((None, JsonBorrowedWriteCursor::new()));
        record_inline::<Receiving>(body)?;
        let (tree, cursor) = slot.as_mut().ok_or_else(|| ValueError::invariant("slot absent"))?;
        admit_inline::<DslValue>(body)?;
        let before = native.owned_bytes();
        let limit = ceiling(before, body)?;
        let built = native.scoped_maximum(limit, |native| native.scoped_stage(|native| { native.begin_stage(0)?; presence.to_value_controlled(native) }));
        let receipt = typed_receipt::<DslValue>(native.owned_bytes().saturating_sub(before));
        match built { Ok(value) => { *tree = Some(value); body.record_progress(receipt)?; } Err(error) => { body.record_progress(receipt)?; return Err(error.into()) } }
        let tree = tree.as_ref().ok_or_else(|| ValueError::invariant("tree absent"))?;
        loop {
            work.performed();admit_turn(body, cursor.normal_step_demands(tree)?)?;
            let grant = body.remaining_grant();
            let step = cursor.step(tree, grant.maximum_items.min(TURN_UNITS), native, grant);
            let performed = cursor.normal_step_progress();
            let outcome = match step { Ok(Some(text)) => { *result = Some(text); Ok(true) } Ok(None) => Ok(false), Err(error) => Err(error) };
            body.record_progress(performed)?;
            if outcome? { return Ok(()) }
            if performed == RetainedCloneProgress::default() { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON writer has no funded frontier").into()) }
        }
    }));
    RetainedWorkOutcome { result: turn.result.and_then(|()| output.take().map(String::into_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "presence encoding produced no output").into())), work: turn.work }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
