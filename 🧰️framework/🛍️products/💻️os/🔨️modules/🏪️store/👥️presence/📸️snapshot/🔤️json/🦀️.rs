//! 🔤️ Every value-projectable presence owns one canonical JSON native codec under the original snapshot owners.
use crate::os_store::{NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonError, JsonGrammarCursor, JsonMemberPolicy};
use semio_framework_value::{retirement::RetireOwned, DslValue, FromValue, RetainedCloneGrant, RetainedCloneProgress, ToValue, ValueError, ValueRefusalKind};

const TURN_UNITS: usize = 1024;

fn admit_inline<T>(body: &NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<T>(), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 }) }
fn record_inline<T>(body: &mut NativeSnapshotBodyWallet) -> Result<(), ValueError> { body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() }) }
fn admit_turn(body: &NativeSnapshotBodyWallet, demand: semio_framework_value::RetirementDemand) -> Result<(), ValueError> { body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }) }
fn ceiling(owned: usize, body: &NativeSnapshotBodyWallet) -> Result<usize, ValueError> { owned.checked_add(body.remaining_grant().maximum_capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON capacity ceiling overflow")) }
fn typed_receipt<T>(charged: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: charged, released_bytes: 0 } }

/// 🛬️ Parses the borrowed presence pack through the original grammar cursor, then binds it into the original typed owner.
pub(super) fn decode_presence_json<P: FromValue + RetireOwned>(bytes: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<P, ValueError> {
    let text = std::str::from_utf8(bytes).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "presence pack is not UTF-8 JSON"))?;
    type Receiving<P> = (JsonGrammarCursor<DslValue>, Option<DslValue>, Option<JsonError>, Option<P>);
    owner.receive::<Receiving<P>, P>(|slot, native, body| {
        admit_inline::<Receiving<P>>(body)?;
        *slot = Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject), None, None, None));
        record_inline::<Receiving<P>>(body)?;
        let (cursor, tree, refusal, typed) = slot.as_mut().unwrap();
        loop {
            let demand = match cursor.normal_step_demands(text) { Ok(demand) => demand, Err(JsonError::Native(error)) => return Err(error), Err(error) => { let kind = error.kind(); *refusal = Some(error); return Err(ValueError::literal(kind, "presence JSON source was refused")) } };
            admit_turn(body, demand)?;
            let grant = body.remaining_grant();
            let result = cursor.step(text, grant.maximum_items.min(TURN_UNITS), native, grant);
            let performed = cursor.normal_step_progress();
            let outcome = match result { Ok(Some(value)) => { *tree = Some(value); Ok(true) } Ok(None) => Ok(false), Err(JsonError::Native(error)) => Err(error), Err(error) => { let kind = error.kind(); *refusal = Some(error); Err(ValueError::literal(kind, "presence JSON grammar refused the original pack")) } };
            body.record_progress(performed)?;
            if outcome? {
                admit_inline::<P>(body)?;
                let before = native.owned_bytes();
                let limit = ceiling(before, body)?;
                let built = native.scoped_maximum(limit, |native| native.scoped_stage(|native| { native.begin_stage(0)?; P::from_value_controlled(tree.as_ref().unwrap(), native) }));
                let receipt = typed_receipt::<P>(native.owned_bytes().saturating_sub(before));
                return match built {
                    Ok(value) => { *typed = Some(value); body.record_progress(receipt)?; Ok(typed.take().unwrap()) }
                    Err(error) => { body.record_progress(receipt)?; Err(error) }
                };
            }
            if performed == RetainedCloneProgress::default() { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "presence JSON parser has no funded frontier")) }
        }
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
