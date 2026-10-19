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
pub fn decode_presence_json<P: semio_framework_value::native_decoding::NativeReceive<DslValue>>(bytes:&[u8],owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<P,ValueError>{
    use semio_framework_value::{native_decoding::owned::NativeReceivingOriginal,retained_clone::RetainedCloneSource};
    type Source=RetainedCloneSource<DslValue>;
    type Receiving<P>=(JsonGrammarCursor<DslValue>,Option<DslValue>,Option<JsonError>,Option<NativeReceivingOriginal<DslValue,P>>,(Option<Source>,Option<P>));
    let text=std::str::from_utf8(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"presence pack is not UTF-8 JSON"))?;
    owner.receive::<Receiving<P>,P>(|slot,native,body|{
        admit_inline::<Receiving<P>>(body)?;*slot=Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject),None,None,None,(None,None)));record_inline::<Receiving<P>>(body)?;
        let(cursor,tree,refusal,receiving,(source,typed))=slot.as_mut().unwrap();
        admit_turn(body,NativeReceivingOriginal::<DslValue,P>::shell_demand())?;
        let(frame,progress)=NativeReceivingOriginal::<DslValue,P>::begin_shell(body.remaining_grant())?;*receiving=Some(frame);body.record_progress(progress)?;
        loop{
            let demand=match cursor.normal_step_demands(text){Ok(demand)=>demand,Err(JsonError::Native(error))=>return Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);return Err(ValueError::literal(kind,"presence JSON source was refused"))}};
            admit_turn(body,demand)?;let grant=body.remaining_grant();let result=cursor.step(text,grant.maximum_items.min(TURN_UNITS),native,grant);let performed=cursor.normal_step_progress();
            let ready=match result{Ok(Some(value))=>{*tree=Some(value);Ok(true)},Ok(None)=>Ok(false),Err(JsonError::Native(error))=>Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);Err(ValueError::literal(kind,"presence JSON grammar refused the original pack"))}};body.record_progress(performed)?;
            if ready?{break;}if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"presence JSON parser has no funded frontier"));}
        }
        let birth=Source::owned_constructor_demand::<()>();let movement=2*size_of::<Option<DslValue>>()+size_of::<Option<Source>>();let copy=Source::constructor_copy_bytes().checked_add(movement).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"presence source transfer overflow"))?;
        admit_turn(body,semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})?;
        let grant=RetainedCloneGrant{maximum_copy_bytes:body.remaining_grant().maximum_copy_bytes-movement,..body.remaining_grant()};
        let(original,mut progress)=match Source::admit_owned(tree.take().unwrap(),(),grant){Ok(pair)=>pair,Err((error,original,()))=>{*tree=Some(original);return Err(error);}};*source=Some(original);progress.copied_bytes=progress.copied_bytes.checked_add(movement).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"presence source receipt overflow"))?;body.record_progress(progress)?;
        let frame=receiving.as_mut().unwrap();admit_turn(body,frame.original_capture_demand()?)?;let progress=frame.admit_original(source,body.remaining_grant())?;body.record_progress(progress)?;
        admit_turn(body,frame.receiver_birth_demand()?)?;let progress=frame.begin_receiver(body.remaining_grant())?;body.record_progress(progress)?;
        while !frame.output_ready(){let logical_body=body.remaining_grant().maximum_copy_bytes;admit_turn(body,frame.demands(logical_body)?)?;let grant=body.remaining_grant();let limit=ceiling(native.owned_bytes(),body)?;let result=native.scoped_maximum(limit,|native|frame.advance(native,logical_body,grant));match result{Ok(step)=>body.record_progress(step.progress())?,Err(error)=>{body.record_progress(error.retained_progress())?;return Err(error);}}}
        admit_turn(body,frame.take_demand()?)?;let(value,progress)=frame.take(body.remaining_grant())?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"funded presence receiving take refused"))?;*typed=Some(value);body.record_progress(progress)?;
        frame.begin_close();while !frame.terminal_is_empty(){admit_turn(body,frame.close_demands(body.remaining_grant().maximum_copy_bytes)?)?;let result=frame.close_step(body.remaining_grant());match result{Ok(step)=>body.record_progress(step.progress())?,Err(error)=>{body.record_progress(error.retained_progress())?;return Err(error);}}}
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
