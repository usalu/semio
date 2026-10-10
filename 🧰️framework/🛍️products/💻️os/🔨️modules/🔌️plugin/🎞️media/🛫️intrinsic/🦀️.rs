use semio_framework_value::{ValueError as MediaValueError, ValueRefusalKind as MediaRefusal};
use semio_framework_value::retirement::{RetireOwned as MediaRetireOwned, RetirementCursor as MediaRetirementCursor, RetirementStep as MediaRetirementStep, controlled::ControlledRetirement as MediaControlled};
use semio_framework_value::retained_clone::{RetainedCloneGrant as MediaGrant, RetainedCloneProgress as MediaProgress, RetainedCloneStep as MediaStep};
use store::os_pack::record::BorrowedProjectedPackCursor as MediaPackCursor;

type IntrinsicMediaParts = (String, String, Vec<u8>);
struct IntrinsicMediaEncoding { cursor: Option<Box<MediaPackCursor>>, parts: MediaControlled<IntrinsicMediaParts> }
impl IntrinsicMediaEncoding {
    fn new() -> Self { Self { cursor: None, parts: MediaControlled::new((String::new(), String::new(), Vec::new())).ok().unwrap() } }
    fn demand(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, MediaValueError> {
        if let Some(cursor) = &self.cursor {
            if !cursor.terminal_is_empty() { return cursor.retirement_demands(); }
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Option<Box<MediaPackCursor>>>(), capacity_bytes: 0, release_bytes: std::mem::size_of::<MediaPackCursor>(), depth: 1 });
        }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: self.parts.next_copy_byte_demand()?, capacity_bytes: self.parts.next_capacity_byte_demand(copy)?, release_bytes: self.parts.next_release_byte_demand()?, depth: self.parts.next_depth_demand()? })
    }
    fn close(&mut self, grant: MediaGrant) -> Result<MediaStep, MediaValueError> {
        if let Some(cursor) = &mut self.cursor {
            if !cursor.terminal_is_empty() { let result = cursor.close(grant).map_err(|error| error.reason)?; return Ok(MediaStep::Progress(result.progress)); }
            let demand = self.demand(0)?;
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Err(media_frontier_error()); }
            drop(self.cursor.take());
            return Ok(MediaStep::Progress(MediaProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: 0, released_bytes: demand.release_bytes }));
        }
        self.parts.step(grant)
    }
    fn empty(&self) -> bool { self.cursor.is_none() && self.parts.terminal_is_empty() }
}
impl MediaRetireOwned for IntrinsicMediaEncoding {
    fn retirement(self) -> Box<dyn MediaRetirementCursor> { Box::new(self) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }
    fn controlled_retirement_supported() -> bool { true }
    fn retirement_element_copy_bytes() -> usize { std::mem::size_of::<Self>() }
}
impl MediaRetirementCursor for IntrinsicMediaEncoding {
    fn close_step(&mut self, grant: MediaGrant) -> MediaRetirementStep {
        match self.close(grant) { Ok(step) => if self.empty() && step.progress() == MediaProgress::default() { MediaRetirementStep::Complete } else { MediaRetirementStep::Progress(step.progress()) }, Err(error) => MediaRetirementStep::Failure(error) }
    }
    fn terminal_is_empty(&self) -> bool { self.empty() }
    fn next_depth_demand(&self) -> Result<usize, MediaValueError> { Ok(self.demand(0)?.depth) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.demand(0).ok().map(|demand| demand.release_bytes) }
    fn next_work_byte_demand(&self) -> Result<usize, MediaValueError> { Ok(self.demand(0)?.copy_bytes) }
    fn next_birth_bytes(&self, copy: usize) -> Option<usize> { self.demand(copy).ok().map(|demand| demand.capacity_bytes) }
}

struct IntrinsicMediaSource<'a>(&'a DslValue);
impl IntrinsicMediaSource<'_> {
    fn at(&self, path: &[usize]) -> Result<&DslValue, MediaValueError> {
        let Some((0, path)) = path.split_first() else { return Err(media_projection_error()); };
        let mut value = self.0;
        for index in path { value = match value { DslValue::Array(items) => items.get(*index), DslValue::Object(items) => items.get(*index).map(|(_, value)| value), _ => None }.ok_or_else(media_projection_error)?; }
        Ok(value)
    }
}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for IntrinsicMediaSource<'_> {
    fn projection_identity(&self) -> usize { self.0 as *const DslValue as usize }
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>, MediaValueError> {
        use semio_framework_dsl_record::native_encoding::FieldProjectionView as View;
        if path.is_empty() { return Ok(View::Record(&[1])); }
        Ok(match self.at(path)? { DslValue::Null => View::IntrinsicNull, DslValue::Bool(value) => View::IntrinsicBool(*value), DslValue::Number(value) => View::IntrinsicNumber(*value), DslValue::String(text) => View::IntrinsicText(text), DslValue::Bytes(bytes) => View::IntrinsicBytes(bytes), DslValue::Array(items) => View::IntrinsicArray(items.len()), DslValue::Object(items) => View::IntrinsicObject(items.len()) })
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, MediaValueError> { match self.at(path)? { DslValue::Object(items) => items.get(index).map(|(key, _)| key.as_str()).ok_or_else(media_projection_error), _ => Err(media_projection_error()) } }
}
fn media_projection_error() -> MediaValueError { MediaValueError::literal(MediaRefusal::InvariantViolated, "intrinsic media original source path changed") }
fn media_frontier_error() -> MediaValueError { MediaValueError::literal(MediaRefusal::OwnershipLimit, "intrinsic media frontier exceeds original receiving grant") }

fn close_intrinsic_cursor(cursor: &mut MediaPackCursor, native: &mut semio_framework_value::NativeEncodeControl<'_>, wallet: &mut store::NativeSnapshotBodyWallet) -> Result<(), MediaValueError> {
    while !cursor.terminal_is_empty() {
        let grant = wallet.remaining_grant(); let demand = cursor.retirement_demands()?;
        if grant.maximum_items == 0 || demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Err(media_frontier_error()); }
        native.checkpoint()?;
        let result = cursor.close(grant); let progress = match &result { Ok(step) => step.progress, Err(error) => error.reason.retained_progress() };
        wallet.record_progress(progress)?; result.map_err(|error| error.reason)?;
        if progress == MediaProgress::default() && !cursor.terminal_is_empty() { return Err(media_frontier_error()); }
    }
    Ok(())
}

fn encode_intrinsic_parts(port: &str, schema: &str, value: &DslValue, owner: &mut store::NativeSnapshotEncodeOwner<'_, '_>) -> Result<IntrinsicMediaParts, MediaValueError> {
    let grant = owner.remaining_grant();
    if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 || grant.maximum_capacity_bytes == 0 || grant.maximum_release_bytes == 0 || grant.maximum_depth < 4 { return Err(media_frontier_error()); }
    owner.receive::<IntrinsicMediaEncoding, IntrinsicMediaParts>(|slot, native, wallet| {
        *slot = Some(IntrinsicMediaEncoding::new()); let state = slot.as_mut().unwrap();
        wallet.copy_encode_text_into(native, port, &mut state.parts.original_mut().unwrap().0, 2)?;
        wallet.copy_encode_text_into(native, schema, &mut state.parts.original_mut().unwrap().1, 2)?;
        let cursor_bytes = std::mem::size_of::<MediaPackCursor>(); let header_bytes = std::mem::size_of::<Option<Box<MediaPackCursor>>>();
        wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: header_bytes, maximum_capacity_bytes: cursor_bytes, maximum_release_bytes: 0, maximum_depth: 2 })?;
        native.checkpoint()?; native.charge(cursor_bytes)?; state.cursor = Some(Box::new(MediaPackCursor::default()));
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: header_bytes, retained_capacity_bytes: cursor_bytes, released_bytes: 0 })?;
        let source = IntrinsicMediaSource(value); let mut length = 0usize; let mut scratch = [0; 64];
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            while !state.cursor.as_ref().unwrap().is_complete() {
                let remaining = wallet.remaining_grant(); let capacity = state.cursor.as_ref().unwrap().next_capacity_byte_demand()?;
                if remaining.maximum_items == 0 || remaining.maximum_copy_bytes < state.cursor.as_ref().unwrap().next_minimum_copy_bytes() || capacity > remaining.maximum_capacity_bytes || remaining.maximum_depth < state.cursor.as_ref().unwrap().next_advance_depth_demand()? { return Err(media_frontier_error()); }
                native.checkpoint()?; native.charge(capacity)?;
                let result = state.cursor.as_mut().unwrap().advance_intrinsic(&source, 1, &mut scratch, remaining);
                let progress = match &result { Ok(step) => step.progress, Err(error) => error.reason.retained_progress() }; wallet.record_progress(progress)?;
                let step = result.map_err(|error| error.reason)?; length = length.checked_add(step.written_bytes).ok_or_else(media_frontier_error)?;
                if progress == MediaProgress::default() && !step.complete { return Err(media_frontier_error()); }
                native.step()?;
            }
            close_intrinsic_cursor(state.cursor.as_mut().unwrap(), native, wallet)
        })?;
        **state.cursor.as_mut().unwrap() = MediaPackCursor::default();
        wallet.allocate_encode_vec_into(native, length, &mut state.parts.original_mut().unwrap().2, 2)?;
        native.scoped_stage(|native| {
            native.begin_stage(length)?;
            while !state.cursor.as_ref().unwrap().is_complete() {
                let remaining = wallet.remaining_grant(); let capacity = state.cursor.as_ref().unwrap().next_capacity_byte_demand()?;
                if remaining.maximum_items == 0 || capacity > remaining.maximum_capacity_bytes || remaining.maximum_depth < state.cursor.as_ref().unwrap().next_advance_depth_demand()? { return Err(media_frontier_error()); }
                let emission = MediaGrant { maximum_copy_bytes: remaining.maximum_copy_bytes / 2, ..remaining };
                if emission.maximum_copy_bytes < state.cursor.as_ref().unwrap().next_minimum_copy_bytes() { return Err(media_frontier_error()); }
                native.checkpoint()?; native.charge(capacity)?;
                let result = state.cursor.as_mut().unwrap().advance_intrinsic(&source, 1, &mut scratch, emission);
                let progress = match &result { Ok(step) => step.progress, Err(error) => error.reason.retained_progress() }; wallet.record_progress(progress)?;
                let step = result.map_err(|error| error.reason)?;
                let data = &mut state.parts.original_mut().unwrap().2;
                if step.written_bytes > length.saturating_sub(data.len()) { return Err(media_projection_error()); }
                wallet.record_progress(MediaProgress { copied_bytes: step.written_bytes, ..Default::default() })?;
                data.extend_from_slice(&scratch[..step.written_bytes]); native.advance(step.written_bytes)?;
                if progress == MediaProgress::default() && !step.complete { return Err(media_frontier_error()); }
            }
            if state.parts.original().unwrap().2.len() != length { return Err(media_projection_error()); }
            close_intrinsic_cursor(state.cursor.as_mut().unwrap(), native, wallet)
        })?;
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: std::mem::size_of::<IntrinsicMediaParts>(), ..Default::default() })?;
        state.parts.take_original().ok_or_else(media_projection_error)
    })
}

fn copy_media_text_parts(port: &str, schema: &str, text: &str, owner: &mut store::NativeSnapshotEncodeOwner<'_, '_>) -> Result<(String, String, String), MediaValueError> {
    owner.receive::<(String, String, String), (String, String, String)>(|slot, native, wallet| {
        *slot = Some((String::new(), String::new(), String::new())); let parts = slot.as_mut().unwrap();
        wallet.copy_encode_text_into(native, port, &mut parts.0, 2)?; wallet.copy_encode_text_into(native, schema, &mut parts.1, 2)?; wallet.copy_encode_text_into(native, text, &mut parts.2, 2)?;
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: std::mem::size_of::<(String, String, String)>(), ..Default::default() })?;
        Ok(slot.take().unwrap())
    })
}
