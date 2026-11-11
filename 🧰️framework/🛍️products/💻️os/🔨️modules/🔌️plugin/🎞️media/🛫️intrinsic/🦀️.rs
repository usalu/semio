use semio_framework_value::{ValueError as MediaValueError, ValueRefusalKind as MediaRefusal, NativeEncodeControl as MediaNative, RetainedAllocationEffect as MediaEffect, RetainedWorkOutcome as MediaOutcome};
use semio_framework_value::retained_clone::{RetainedCloneGrant as MediaGrant, RetainedCloneProgress as MediaProgress};
use semio_framework_value::retirement::{RetireOwned as MediaRetireOwned, controlled::ControlledRetirement as MediaControlled};
use semio_os_kernel::io::control::{NativeSnapshotBodyWallet as MediaWallet, NativeSnapshotEffectProgress as MediaMarker, NativeSnapshotFailure as MediaFailure, NativeSnapshotWork as MediaWork, NativeSnapshotEncodeOwner as MediaOwner, native_snapshot_turn as media_turn};
use store::os_pack::record::BorrowedProjectedPackCursor as MediaPackCursor;

type IntrinsicMediaParts = (String, String, Vec<u8>);

/// 🧳️ Owns the Pack cursor and the partially built parts of one intrinsic media encoding until the original frame retires it.
#[derive(semio_framework_value::RetireOwned)]
struct IntrinsicMediaEncoding { pub cursor: Option<Box<MediaPackCursor>>, pub parts: Option<IntrinsicMediaParts> }

struct IntrinsicMediaSource<'a>(&'a DslValue);
impl IntrinsicMediaSource<'_> {
    pub fn at(&self, path: &[usize]) -> Result<&DslValue, MediaValueError> {
        let mut value = self.0;
        for index in path { value = match value { DslValue::Array(items) => items.get(*index), DslValue::Object(items) => items.get(*index).map(|(_, value)| value), _ => None }.ok_or_else(media_projection_error)?; }
        Ok(value)
    }
}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for IntrinsicMediaSource<'_> {
    fn projection_identity(&self) -> usize { self.0 as *const DslValue as usize }
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>, MediaValueError> {
        use semio_framework_dsl_record::native_encoding::FieldProjectionView as View;
        Ok(match self.at(path)? { DslValue::Null => View::IntrinsicNull, DslValue::Bool(value) => View::IntrinsicBool(*value), DslValue::Number(value) => View::IntrinsicNumber(*value), DslValue::String(text) => View::IntrinsicText(text), DslValue::Bytes(bytes) => View::IntrinsicBytes(bytes), DslValue::Array(items) => View::IntrinsicArray(items.len()), DslValue::Object(items) => View::IntrinsicObject(items.len()) })
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, MediaValueError> { match self.at(path)? { DslValue::Object(items) => items.get(index).map(|(key, _)| key.as_str()).ok_or_else(media_projection_error), _ => Err(media_projection_error()) } }
}
fn media_frontier_error() -> MediaValueError { MediaValueError::literal(MediaRefusal::OwnershipLimit, "intrinsic media frontier exceeds original receiving grant") }

/// 🧾️ Projects an original snapshot failure onto the plain media value error without losing kind, message or retained progress.
fn media_snapshot_error(failure: MediaFailure) -> MediaValueError { MediaValueError::new(failure.kind(), failure.message().to_owned()).with_retained_progress(failure.retained_progress()) }

fn media_performed(effect: &mut MediaEffect<'_>) -> Result<(), MediaValueError> {
    if effect.performed(1)? { Ok(()) } else { Err(MediaValueError::literal(MediaRefusal::WorkLimit, "media encoder effect exceeded its original work")) }
}

/// 🎟️ Opens exactly one quoted original effect per call, charges its receipt to the turn wallet and keeps the marker and failure slots caller-retained.
struct MediaEffectTurn<'a> { pub wallet: &'a mut MediaWallet, pub marker: &'a mut MediaMarker, pub unwind: &'a mut Option<MediaFailure>, pub work: &'a mut MediaWork }
impl MediaEffectTurn<'_> {
    pub fn run(&mut self, native: &mut MediaNative<'_>, demand: MediaGrant, bytes: Option<usize>, operation: &mut impl FnMut(&mut MediaProgress, &mut MediaEffect<'_>) -> Result<(), MediaValueError>) -> Result<(), MediaFailure> {
        if self.unwind.is_some() { return Err(MediaValueError::literal(MediaRefusal::InvariantViolated, "media encoder effect requires empty retained failure slot").into()); }
        self.wallet.admit_frontier(demand).map_err(|_| media_frontier_error())?;
        native.checkpoint()?;
        let grant = self.wallet.remaining_grant();
        let mut local = MediaProgress::default();
        let attempt = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| native.effect_allocated_with_work(grant, &mut local, demand, bytes, 1, &mut self.marker.state.view(), &mut self.marker.failure, operation)));
        let recorded = self.wallet.record_progress(local);
        let progress = self.wallet.progress();
        match attempt {
            Ok(turn) => {
                let done = self.work.take(MediaOutcome { work: turn.work, result: turn.result.map_err(|error| MediaFailure::from_encode_effect(error, progress)) })?;
                recorded?;
                if done.is_none() { return Err(MediaValueError::literal(MediaRefusal::InvariantViolated, "media encoder effect awaits an unfinished original reply").into()); }
                self.marker.reset_completed()?;
                Ok(())
            }
            Err(payload) => {
                if let Some(error) = self.marker.failure.take() { *self.unwind = Some(MediaFailure::from_encode_effect(error, progress)); }
                std::panic::resume_unwind(payload)
            }
        }
    }
}

/// 🌱️ Quotes the next Pack advance from the cursor and runs it as one effect whose receipt is the step's exact progress.
fn advance_media_cursor(cursor: &mut MediaPackCursor, source: &IntrinsicMediaSource<'_>, scratch: &mut [u8; 64], native: &mut MediaNative<'_>, turn: &mut MediaEffectTurn<'_>) -> Result<store::os_pack::record::BorrowedProjectedPackProgress, MediaFailure> {
    let remaining = turn.wallet.remaining_grant();
    let capacity = cursor.next_capacity_byte_demand()?;
    let demand = MediaGrant { maximum_items: 1, maximum_copy_bytes: remaining.maximum_copy_bytes.min(scratch.len()).max(cursor.next_minimum_copy_bytes()), maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: cursor.next_advance_depth_demand()? };
    let mut step = None;
    turn.run(native, demand, (capacity != 0).then_some(capacity), &mut |physical, effect| match cursor.advance_intrinsic(source, 1, scratch, effect) {
        Ok(done) => { *physical = done.progress; step = Some(done); Ok(()) }
        Err(failure) => { *physical = failure.reason.retained_progress(); Err(failure.reason) }
    })?;
    let step = step.ok_or_else(media_projection_error)?;
    if step.progress == MediaProgress::default() && !step.complete { return Err(media_frontier_error().into()); }
    Ok(step)
}

/// ♻️ Retires the cursor symbols through one quoted effect per physical close until its terminal-empty witness holds.
fn close_media_cursor(cursor: &mut MediaPackCursor, native: &mut MediaNative<'_>, turn: &mut MediaEffectTurn<'_>) -> Result<(), MediaFailure> {
    while !cursor.terminal_is_empty() {
        let quote = cursor.retirement_demands()?;
        let demand = MediaGrant { maximum_items: 1, maximum_copy_bytes: quote.copy_bytes, maximum_capacity_bytes: quote.capacity_bytes, maximum_release_bytes: quote.release_bytes, maximum_depth: quote.depth };
        let mut progress = MediaProgress::default();
        turn.run(native, demand, (quote.capacity_bytes != 0).then_some(quote.capacity_bytes), &mut |physical, effect| match cursor.close(effect) {
            Ok(done) => { *physical = done.progress; progress = done.progress; Ok(()) }
            Err(failure) => { *physical = failure.reason.retained_progress(); Err(failure.reason) }
        })?;
        if progress == MediaProgress::default() && !cursor.terminal_is_empty() { return Err(media_frontier_error().into()); }
    }
    Ok(())
}

/// 🛫️ Encodes one intrinsic value through measured and emitting Pack passes, each step an effect on the original encoder.
fn encode_intrinsic_into(port: &str, schema: &str, value: &DslValue, state: &mut IntrinsicMediaEncoding, native: &mut MediaNative<'_>, turn: &mut MediaEffectTurn<'_>) -> Result<(), MediaFailure> {
    let parts = state.parts.insert((String::new(), String::new(), Vec::new()));
    turn.wallet.copy_encode_text_into(native, port, &mut parts.0, 2)?;
    turn.wallet.copy_encode_text_into(native, schema, &mut parts.1, 2)?;
    let cursor_bytes = size_of::<MediaPackCursor>(); let header_bytes = size_of::<Option<Box<MediaPackCursor>>>();
    let birth = MediaGrant { maximum_items: 1, maximum_copy_bytes: header_bytes, maximum_capacity_bytes: cursor_bytes, maximum_release_bytes: 0, maximum_depth: 2 };
    turn.run(native, birth, Some(cursor_bytes), &mut |physical, effect| {
        media_performed(effect)?;
        *effect.actual() = Some(0);
        state.cursor = Some(Box::new(MediaPackCursor::default()));
        *effect.actual() = Some(cursor_bytes);
        *physical = MediaProgress { copied_items: 1, copied_bytes: header_bytes, retained_capacity_bytes: cursor_bytes, released_bytes: 0 };
        Ok(())
    })?;
    let cursor = state.cursor.as_mut().ok_or_else(media_projection_error)?;
    let source = IntrinsicMediaSource(value); let mut length = 0usize; let mut scratch = [0; 64];
    native.scoped_stage(|native| -> Result<(), MediaFailure> {
        native.begin_stage(0)?;
        while !cursor.is_complete() {
            let step = advance_media_cursor(cursor, &source, &mut scratch, native, turn)?;
            length = length.checked_add(step.written_bytes).ok_or_else(media_frontier_error)?;
            native.step()?;
        }
        close_media_cursor(cursor, native, turn)
    })?;
    **cursor = MediaPackCursor::default();
    let data = &mut parts.2;
    turn.wallet.allocate_encode_vec_into(native, length, data, 2)?;
    native.scoped_stage(|native| -> Result<(), MediaFailure> {
        native.begin_stage(length)?;
        while !cursor.is_complete() {
            let step = advance_media_cursor(cursor, &source, &mut scratch, native, turn)?;
            let count = step.written_bytes;
            if count > length.saturating_sub(data.len()) { return Err(media_projection_error().into()); }
            if count != 0 {
                let demand = MediaGrant { maximum_items: 1, maximum_copy_bytes: count, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
                turn.run(native, demand, None, &mut |physical, effect| {
                    media_performed(effect)?;
                    data.extend_from_slice(&scratch[..count]);
                    *physical = MediaProgress { copied_items: 1, copied_bytes: count, ..Default::default() };
                    Ok(())
                })?;
                native.advance(count)?;
            }
        }
        if data.len() != length { return Err(media_projection_error().into()); }
        close_media_cursor(cursor, native, turn)
    })?;
    Ok(())
}

fn encode_intrinsic_parts(port: &str, schema: &str, value: &DslValue, owner: &mut MediaOwner<'_, '_>) -> Result<IntrinsicMediaParts, MediaValueError> {
    let grant = owner.remaining_grant();
    if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 || grant.maximum_capacity_bytes == 0 || grant.maximum_release_bytes == 0 || grant.maximum_depth < 4 { return Err(media_frontier_error()); }
    let mut output = None;
    owner.receive::<IntrinsicMediaEncoding, IntrinsicMediaParts, MediaFailure>(&mut output, &mut |slot, result, marker, unwind, native, wallet| media_turn(|work| {
        work.performed();
        let state = slot.insert(IntrinsicMediaEncoding { cursor: None, parts: None });
        let mut turn = MediaEffectTurn { wallet: &mut *wallet, marker: &mut *marker, unwind: &mut *unwind, work: &mut *work };
        encode_intrinsic_into(port, schema, value, state, native, &mut turn)?;
        work.take(wallet.snapshot_encode(native).move_existing(&mut state.parts, result, marker, unwind))
    })).result.map_err(media_snapshot_error)?;
    output.ok_or_else(media_projection_error)
}

#[doc(hidden)]
pub fn copy_media_text_parts(port: &str, schema: &str, text: &str, owner: &mut MediaOwner<'_, '_>) -> Result<(String, String, String), MediaValueError> {
    let mut output = None;
    owner.receive::<(String, String, String), (String, String, String), MediaFailure>(&mut output, &mut |slot, result, marker, unwind, native, wallet| media_turn(|work| {
        work.performed();
        let parts = slot.insert((String::new(), String::new(), String::new()));
        wallet.copy_encode_text_into(native, port, &mut parts.0, 2)?; wallet.copy_encode_text_into(native, schema, &mut parts.1, 2)?; wallet.copy_encode_text_into(native, text, &mut parts.2, 2)?;
        work.take(wallet.snapshot_encode(native).move_existing(slot, result, marker, unwind))
    })).result.map_err(media_snapshot_error)?;
    output.ok_or_else(media_projection_error)
}

pub use semio_plugin_decl_app::media_intrinsic::*;
