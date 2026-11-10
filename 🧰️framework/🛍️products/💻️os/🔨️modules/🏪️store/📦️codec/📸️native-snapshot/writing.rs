//! 📨️ Borrowed canonical Pack documents write directly into their sole final declared envelope buffer.
use pack::record::BorrowedPackDocumentCursor;
use semio_framework_dsl_record::{BorrowedRecordSpec, native_encoding::FieldProjectionSource};
use semio_framework_value::{
    NativeEncodeControl, RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::retained_clone_grant_admits,
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
};
use std::mem::{ManuallyDrop, size_of};
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "original Pack writer requires its borrowed canonical document and funded sole final buffer")
}
fn unit(effect: &mut RetainedAllocationEffect<'_>) -> Result<(), ValueError> {
    if effect.performed(1)? { Ok(()) } else { Err(ValueError::literal(ValueRefusalKind::WorkLimit, "original Pack writer exceeds its work ceiling")) }
}
/// 📦️ Codec scratch, initialized bytes and successful scratch draining retain independent original demands.
pub struct OriginalPackWriting {
    cursor: ManuallyDrop<Option<BorrowedPackDocumentCursor>>,
    wire: ManuallyDrop<Vec<u8>>,
    envelope: &'static str,
    prefix: usize,
    initialized: usize,
    prefix_written: usize,
    written: usize,
    draining: bool,
    closing: bool,
}
impl OriginalPackWriting {
    /// 🎟️ Constructs only the inline writer header; its caller must first admit that header under its original grant.
    pub fn new(envelope: &'static str, maximum: usize) -> Result<Self, ValueError> {
        let prefix = semio_os_semio::declared_envelope_prefix_len(envelope, semio_os_semio::Component::Pack, 1)?;
        let body = maximum.checked_sub(prefix).ok_or_else(invalid)?;
        Ok(Self {
            cursor: ManuallyDrop::new(Some(BorrowedPackDocumentCursor::with_maximum_file_len(body).with_maximum_body_len(body))),
            wire: ManuallyDrop::new(Vec::new()),
            envelope,
            prefix,
            initialized: 0,
            prefix_written: 0,
            written: 0,
            draining: false,
            closing: false,
        })
    }
    pub fn ready(&self) -> bool {
        !self.closing && self.cursor.is_none() && self.wire.capacity() > 0
    }
    pub fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing {
            return self.close_demand(body);
        }
        let cursor = self.cursor.as_ref().ok_or_else(invalid)?;
        if self.draining || cursor.is_complete() {
            return cursor.retirement_demands();
        }
        if let Some(length) = cursor.output_len() {
            let total = self.prefix.checked_add(length).ok_or_else(invalid)?;
            if self.wire.capacity() == 0 {
                return Ok(RetirementDemand { copy_bytes: size_of::<Vec<u8>>(), capacity_bytes: total, depth: 1, ..Default::default() });
            }
            if self.initialized < total {
                return Ok(RetirementDemand { copy_bytes: (total - self.initialized).min(body).min(65536), depth: 1, ..Default::default() });
            }
            if self.prefix_written < self.prefix {
                return Ok(RetirementDemand { copy_bytes: (self.prefix - self.prefix_written).min(body), depth: 1, ..Default::default() });
            }
        }
        Ok(RetirementDemand { copy_bytes: cursor.next_copy_byte_demand()?, capacity_bytes: cursor.next_capacity_byte_demand()?, depth: cursor.next_depth_demand()?, ..Default::default() })
    }
    /// ✍️ Denied turns preserve every owner and callback count; accepted spans use the supplied original encoder.
    pub fn advance<T: FieldProjectionSource>(&mut self, source: &T, schema: BorrowedRecordSpec, native: &mut NativeEncodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand(grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(Default::default());
        }
        native.checkpoint()?;
        let progress = self.advance_owned(source, schema, native, effect, demand)?;
        if progress != RetainedCloneProgress::default() {
            native.step().map_err(|e| e.with_retained_progress(progress))?;
        }
        Ok(progress)
    }
    fn advance_owned<T: FieldProjectionSource>(&mut self, source: &T, schema: BorrowedRecordSpec, native: &mut NativeEncodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>, demand: RetirementDemand) -> Result<RetainedCloneProgress, ValueError> {
        let grant = effect.grant();
        let cursor = self.cursor.as_mut().ok_or_else(invalid)?;
        if self.draining || cursor.is_complete() {
            self.draining = true;
            native.charge(demand.capacity_bytes)?;
            let step = cursor.close(effect).map_err(|e| e.reason)?;
            if cursor.terminal_is_empty() {
                self.cursor.take();
            }
            return Ok(step.progress);
        }
        if let Some(length) = cursor.output_len() {
            let total = self.prefix.checked_add(length).ok_or_else(invalid)?;
            if self.wire.capacity() == 0 {
                unit(effect)?;
                *self.wire = native.allocate_vec(total)?;
                return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u8>>(), retained_capacity_bytes: self.wire.capacity(), ..Default::default() });
            }
            if self.initialized < total {
                let count = (total - self.initialized).min(grant.maximum_copy_bytes).min(65536);
                if count == 0 {
                    return Ok(Default::default());
                }
                unit(effect)?;
                self.initialized += count;
                self.wire.resize(self.initialized, 0);
                return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() });
            }
            if self.prefix_written < self.prefix {
                let count = (self.prefix - self.prefix_written).min(grant.maximum_copy_bytes);
                if count == 0 {
                    return Ok(Default::default());
                }
                unit(effect)?;
                for i in self.prefix_written..self.prefix_written + count {
                    self.wire[i] = semio_os_semio::declared_binary_prefix_byte(self.envelope, semio_os_semio::Component::Pack, 1, i)?.ok_or_else(invalid)?;
                }
                self.prefix_written += count;
                return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() });
            }
        }
        native.charge(demand.capacity_bytes)?;
        let out = if self.wire.capacity() == 0 {
            &mut [][..]
        } else {
            let start = self.prefix.checked_add(self.written).ok_or_else(invalid)?;
            &mut self.wire[start..]
        };
        match cursor.advance(source, schema, out, effect) {
            Ok(step) => {
                self.written = self.written.checked_add(step.written_bytes).ok_or_else(invalid)?;
                Ok(step.progress)
            }
            Err(error) => {
                self.written = self.written.saturating_add(error.written_bytes);
                Err(error.reason)
            }
        }
    }
    /// 📬️ Transfers the actual sole final Pack after all canonical codec scratch has reached terminal close.
    pub fn take(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<(Vec<u8>, RetainedCloneProgress)>, ValueError> {
        let grant = effect.grant();
        if !self.ready() {
            return Err(invalid());
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(None);
        }
        if self.wire.len() != self.prefix.checked_add(self.written).ok_or_else(invalid)? {
            return Err(invalid());
        }
        unit(effect)?;
        Ok(Some((std::mem::take(&mut *self.wire), RetainedCloneProgress { copied_items: 1, ..Default::default() })))
    }
    pub fn close_demand(&self, _body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(cursor) = self.cursor.as_ref() {
            cursor.retirement_demands()
        } else {
            Ok(RetirementDemand { copy_bytes: usize::from(self.wire.capacity() > 0) * size_of::<Vec<u8>>(), release_bytes: self.wire.capacity(), depth: usize::from(self.wire.capacity() > 0), ..Default::default() })
        }
    }
    pub fn close_step(&mut self, _body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        match self.close(effect) {
            Ok(step) => RetirementStep::Progress(step.progress(), effect.performed_work()),
            Err(e) => RetirementStep::Failure(e.into(), effect.performed_work()),
        }
    }
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant = effect.grant();
        let demand = self.close_demand(grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.closing = true;
        if let Some(cursor) = self.cursor.as_mut() {
            let step = cursor.close(effect).map_err(|e| e.reason)?;
            if cursor.terminal_is_empty() {
                self.cursor.take();
            }
            return Ok(RetainedCloneStep::Progress(step.progress));
        }
        if self.wire.capacity() > 0 {
            unit(effect)?;
            let released_bytes = self.wire.capacity();
            drop(std::mem::take(&mut *self.wire));
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn empty(&self) -> bool {
        self.cursor.is_none() && self.wire.capacity() == 0
    }
}
impl RetireOwned for OriginalPackWriting {
    type RetirementCursor = Self;
    fn retirement_seed(mut self) -> Self {
        self.closing = true;
        self
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl RetirementCursor for OriginalPackWriting {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demand(body)
    }
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self) -> bool {
        self.empty()
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
impl Drop for OriginalPackWriting {
    fn drop(&mut self) {
        assert!(self.empty() || std::thread::panicking(), "original Pack writer requires funded scratch and final output closure");
    }
}
#[cfg(test)]
#[path="writing-tests.rs"]
mod tests;
