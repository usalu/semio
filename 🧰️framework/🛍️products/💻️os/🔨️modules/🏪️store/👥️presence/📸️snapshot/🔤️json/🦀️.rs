//! 🔤️ Every value-projectable presence owns one canonical JSON native codec under the original snapshot owners.
use semio_framework_io_control::{NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonError, JsonGrammarCursor, JsonMemberPolicy};
use semio_framework_value::{retirement::RetireOwned, DslValue, RetainedCloneGrant, RetainedCloneProgress, ToValue, ValueError, ValueRefusalKind};

const TURN_UNITS: usize = 1024;

fn admit_inline<T>(body: &NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<T>(), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 }) }
fn record_inline<T>(body: &mut NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() }) }
fn admit_turn(body: &NativeSnapshotBodyWallet, demand: semio_framework_value::RetirementDemand) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }) }
fn ceiling(owned: usize, body: &NativeSnapshotBodyWallet) -> Result<usize, ValueError> { owned.checked_add(body.remaining_grant().maximum_capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON capacity ceiling overflow")) }
fn typed_receipt<T>(charged: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: charged, released_bytes: 0 } }

/// 🛬️ Parses the borrowed presence pack through the original grammar cursor, then binds it into the original typed owner.
pub fn decode_presence_json<P: semio_framework_value_native_owned::native_decoding::NativeReceive<DslValue>>(bytes:&[u8],owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<P,ValueError>{
    use semio_framework_value::retained_clone::RetainedCloneSource;
    use semio_framework_value_native_owned::native_decoding::owned::NativeReceivingOriginal;
    type Source=RetainedCloneSource<DslValue>;
    type Receiving<P>=(JsonGrammarCursor<DslValue>,Option<DslValue>,Option<JsonError>,Option<NativeReceivingOriginal<DslValue,P>>,(Option<Source>,Option<P>));
    let text=std::str::from_utf8(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"presence pack is not UTF-8 JSON"))?;
    owner.receive::<Receiving<P>,P>(|slot,native,body|{
        admit_inline::<Receiving<P>>(body)?;*slot=Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject),None,None,None,(None,None)));record_inline::<Receiving<P>>(body)?;
        let(cursor,tree,refusal,receiving,(_,typed))=slot.as_mut().ok_or_else(||ValueError::invariant("slot absent"))?;
        loop{
            let demand=match cursor.normal_step_demands(text){Ok(demand)=>demand,Err(JsonError::Native(error))=>return Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);return Err(ValueError::literal(kind,"presence JSON source was refused"))}};
            admit_turn(body,demand)?;let grant=body.remaining_grant();let result=cursor.step(text,grant.maximum_items.min(TURN_UNITS),native,grant);let performed=cursor.normal_step_progress();
            let ready=match result{Ok(Some(value))=>{*tree=Some(value);Ok(true)},Ok(None)=>Ok(false),Err(JsonError::Native(error))=>Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);Err(ValueError::literal(kind,"presence JSON grammar refused the original pack"))}};body.record_progress(performed)?;
            if ready?{break;}if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"presence JSON parser has no funded frontier"));}
        }
        let mut admission=Source::admission_owned(tree.take().ok_or_else(||ValueError::invariant("tree absent"))?,());let mut original=None;
        let admitted=(||->Result<(),ValueError>{
            while admission.phase()!="ready"{admit_turn(body,admission.next_demand()?)?;let step=admission.advance(body.remaining_grant()).result?;body.record_progress(step.progress())?;}
            admit_turn(body,admission.next_demand()?)?;let step=admission.take_output(&mut original,body.remaining_grant()).result?;body.record_progress(step.progress())?;Ok(())
        })();
        if let Err(error)=admitted{
            let mut back=None;
            while !admission.terminal_is_empty(){
                if admit_turn(body,admission.next_return_demand().unwrap_or_default()).is_err(){break}
                let turn=admission.return_inputs(&mut back,body.remaining_grant());
                match turn.result{Ok(step)=>{if body.record_progress(step.progress()).is_err(){break}},Err(_)=>break}
            }
            *tree=back.map(|(value,())|value);
            return Err(error);
        }
        while !admission.terminal_is_empty(){admit_turn(body,admission.next_return_demand()?)?;let step=admission.return_inputs(&mut None,body.remaining_grant()).result?;body.record_progress(step.progress())?;}
        let source=original.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"presence source admission produced no original"))?;
        *receiving=Some(NativeReceivingOriginal::<DslValue,P>::new(source));let frame=receiving.as_mut().ok_or_else(||ValueError::invariant("receiving absent"))?;
        while !frame.output_ready(){let logical_body=body.remaining_grant().maximum_copy_bytes;admit_turn(body,frame.demands(logical_body)?)?;let grant=body.remaining_grant();let limit=ceiling(native.owned_bytes(),body)?;let turn=native.scoped_maximum::<_,ValueError>(limit,|native|Ok(frame.advance(native,logical_body,grant)))?;match turn.result{Ok(step)=>body.record_progress(step.progress())?,Err(error)=>{body.record_progress(error.retained_progress())?;let refusal=error.primary.or(error.quote_refusal).or(error.allocation_refusal).unwrap_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"presence original receiving was refused"));return Err(refusal);}}}
        admit_turn(body,frame.take_demand()?)?;let(value,progress)=frame.take(body.remaining_grant()).result?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"funded presence receiving take refused"))?;*typed=Some(value);body.record_progress(progress)?;
        frame.begin_close();while !frame.terminal_is_empty(){admit_turn(body,frame.close_demands(body.remaining_grant().maximum_copy_bytes)?)?;match frame.close_step(body.remaining_grant()).result{Ok(step)=>body.record_progress(step.progress())?,Err(error)=>{body.record_progress(error.retained_progress())?;return Err(error);}}}
        let copy=2*size_of::<Option<P>>();admit_turn(body,semio_framework_value::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;Ok(typed.take().unwrap())
    })
}

/// 🛫️ Projects the original typed owner into one retained tree and writes its canonical JSON through the original cursor.
pub(super) fn encode_presence_json<P: ToValue + RetireOwned>(presence: &P, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
    type Receiving = (Option<DslValue>, JsonBorrowedWriteCursor, Option<String>);
    owner.receive::<Receiving, String>(|slot, native, body| {
        admit_inline::<Receiving>(body)?;
        *slot = Some((None, JsonBorrowedWriteCursor::new(), None));
        record_inline::<Receiving>(body)?;
        let (tree, cursor, output) = slot.as_mut().unwrap();
        admit_inline::<DslValue>(body)?;
        let before = native.owned_bytes();
        let limit = ceiling(before, body)?;
        let built = native.scoped_maximum(limit, |native| native.scoped_stage(|native| { native.begin_stage(0)?; presence.to_value_controlled(native) }));
        let receipt = typed_receipt::<DslValue>(native.owned_bytes().saturating_sub(before));
        match built { Ok(value) => { *tree = Some(value); body.record_progress(receipt)?; } Err(error) => { body.record_progress(receipt)?; return Err(error) } }
        let tree = tree.as_ref().unwrap();
        loop {
            admit_turn(body, cursor.normal_step_demands(tree)?)?;
            let grant = body.remaining_grant();
            let result = cursor.step(tree, grant.maximum_items.min(TURN_UNITS), native, grant);
            let performed = cursor.normal_step_progress();
            let outcome = match result { Ok(Some(text)) => { *output = Some(text); Ok(true) } Ok(None) => Ok(false), Err(error) => Err(error) };
            body.record_progress(performed)?;
            if outcome? { return Ok(output.take().unwrap()) }
            if performed == RetainedCloneProgress::default() { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON writer has no funded frontier")) }
        }
    }).map(String::into_bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
