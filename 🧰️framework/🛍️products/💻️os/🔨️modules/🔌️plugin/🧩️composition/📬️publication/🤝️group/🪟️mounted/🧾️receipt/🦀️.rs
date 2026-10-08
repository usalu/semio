use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

pub(crate) const MOUNTED_RECEIPT_MAXIMUM_BYTES: usize = 1_048_576;

pub(crate) struct MountedReceiptBytes {
    bytes: ManuallyDrop<Vec<u8>>,
    candidate: ManuallyDrop<Option<Vec<u8>>>,
    copied: usize,
    required: Option<usize>,
    closing: bool,
    closed: bool,
}

impl MountedReceiptBytes {
    pub(crate) fn new() -> Self { Self::from_original(Vec::new()) }
    pub(crate) fn from_original(bytes: Vec<u8>) -> Self { Self { bytes: ManuallyDrop::new(bytes), candidate: ManuallyDrop::new(None), copied: 0, required: None, closing: false, closed: false } }
    pub(crate) fn as_slice(&self) -> &[u8] { self.bytes.as_slice() }
    pub(crate) fn ready_for(&self, additional: usize) -> bool { !self.closed && !self.closing && self.candidate.is_none() && self.bytes.capacity() - self.bytes.len() >= additional }
    /// 📐️ Queries one whole replacement capacity while retaining the original byte allocation.
    pub(crate) fn next_capacity_byte_demand(&self, additional: usize) -> Result<usize, ValueError> {
        if self.closed || self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted receipt byte owner is closed")); }
        let required = self.bytes.len().checked_add(additional).filter(|bytes| *bytes <= MOUNTED_RECEIPT_MAXIMUM_BYTES).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "mounted receipt exceeds its original byte admission"))?;
        if self.required.is_some_and(|retained| retained != required) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted receipt growth changed its exact required prefix")); }
        Ok(if self.candidate.is_some() || self.bytes.capacity() >= required { 0 } else { required.max(self.bytes.capacity().saturating_mul(2)).max(64).min(MOUNTED_RECEIPT_MAXIMUM_BYTES) })
    }
    /// 🪵️ Exposes the exact old allocation only after its initialized prefix has been copied.
    pub(crate) fn next_growth_release_byte_demand(&self) -> usize { if self.candidate.is_some() && self.copied == self.bytes.len() { self.bytes.capacity() } else { 0 } }
    /// 🌱️ Separates whole birth, bounded prefix copying, and whole original retirement into distinct turns.
    pub(crate) fn reserve_step(&mut self, additional: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let capacity = self.next_capacity_byte_demand(additional)?;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.ready_for(additional) { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.candidate.is_none() {
            if grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(Default::default())); }
            *self.candidate = Some(Vec::with_capacity(capacity));
            self.required = Some(self.bytes.len() + additional);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }));
        }
        if self.copied < self.bytes.len() {
            let count = (self.bytes.len() - self.copied).min(grant.maximum_copy_bytes).min(64);
            if count == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.candidate.as_mut().unwrap().extend_from_slice(&self.bytes[self.copied..self.copied + count]);
            self.copied += count;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() }));
        }
        let bytes = self.bytes.capacity();
        if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let next = self.candidate.take().unwrap();
        drop(std::mem::replace(&mut *self.bytes, next));
        self.required = None;
        self.copied = 0;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
    }
    /// ✍️ Appends one original borrowed prefix into already funded storage.
    pub(crate) fn append_step(&mut self, source: &[u8], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closed || self.closing || self.candidate.is_some() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted receipt append requires its exact settled backing")); }
        let count = source.len().min(grant.maximum_copy_bytes).min(64).min(self.bytes.capacity() - self.bytes.len());
        if grant.maximum_items == 0 || count == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.bytes.extend_from_slice(&source[..count]);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() }))
    }
    /// 🧳️ Transfers completed original backing without copying or a second allocation.
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<Vec<u8>> { if self.closed || self.closing || self.candidate.is_some() || grant.maximum_items == 0 { return None; } self.closed = true; Some(std::mem::take(&mut *self.bytes)) }
    pub(crate) fn next_close_byte_demand(&self) -> usize { self.candidate.as_ref().map_or_else(|| self.bytes.capacity(), Vec::capacity) }
    /// ♻️ Retires each original candidate and output allocation under its own whole physical grant.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        if self.terminal_is_empty() { return RetainedCloneStep::Complete(Default::default()); }
        let bytes = self.next_close_byte_demand();
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return RetainedCloneStep::Progress(Default::default()); }
        self.closing = true;
        if self.candidate.is_some() { drop(self.candidate.take()); self.required = None; self.copied = 0; }
        else { drop(std::mem::take(&mut *self.bytes)); self.closed = true; }
        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() })
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.closed && self.bytes.capacity() == 0 && self.candidate.is_none() }
}

impl Drop for MountedReceiptBytes { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mounted receipt byte owner requires exact transfer or whole funded close"); } }

pub(crate) struct MountedPreparedOperationBytes {
    cursor: store::ArtifactPreparedOperationCursor,
    output: MountedReceiptBytes,
    pending: [u8; 64],
    pending_len: usize,
    pending_offset: usize,
    encoded_complete: bool,
    cursor_closed: bool,
    failed: bool,
    closing: bool,
}

impl MountedPreparedOperationBytes {
    pub(crate) fn new() -> Self { Self { cursor: Default::default(), output: MountedReceiptBytes::new(), pending: [0; 64], pending_len: 0, pending_offset: 0, encoded_complete: false, cursor_closed: false, failed: false, closing: false } }
    pub(crate) fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.pending_offset < self.pending_len { self.output.next_capacity_byte_demand(self.pending_len - self.pending_offset) } else { self.cursor.next_capacity_byte_demand() } }
    pub(crate) fn next_growth_release_byte_demand(&self) -> Result<usize, ValueError> { if self.pending_offset < self.pending_len { Ok(self.output.next_growth_release_byte_demand()) } else if !self.cursor_closed { self.cursor.next_close_byte_demand() } else { Ok(0) } }
    pub(crate) fn is_complete(&self) -> bool { !self.failed && !self.closing && self.encoded_complete && self.cursor_closed && self.pending_len == self.pending_offset }
    /// 🧵️ Retains the exact encoded prefix before copying it into independently admitted output backing.
    pub(crate) fn advance(&mut self, source: store::ArtifactPreparedOperationSource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.failed || self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted prepared operation requires funded close after refusal")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.pending_offset < self.pending_len {
            let additional = self.pending_len - self.pending_offset;
            if !self.output.ready_for(additional) { return self.output.reserve_step(additional, grant); }
            let step = self.output.append_step(&self.pending[self.pending_offset..self.pending_len], grant)?;
            self.pending_offset += step.progress().copied_bytes;
            return Ok(step);
        }
        if self.encoded_complete {
            if self.cursor_closed { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let progress = self.cursor.close(grant).map_err(|error| error.reason)?;
            self.cursor_closed = progress.complete;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.processed_items, copied_bytes: progress.copied_bytes, retained_capacity_bytes: progress.retained_capacity_bytes, released_bytes: progress.released_bytes, ..Default::default() }));
        }
        match self.cursor.advance(source, &mut self.pending, grant) {
            Ok(progress) => {
                self.pending_offset = 0;
                self.pending_len = progress.written_bytes;
                self.encoded_complete = progress.complete;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.processed_items, copied_bytes: progress.copied_bytes, retained_capacity_bytes: progress.retained_capacity_bytes, released_bytes: progress.released_bytes, ..Default::default() }))
            }
            Err(error) => { self.pending_offset = 0; self.pending_len = error.written_bytes; self.failed = true; Err(error.reason) }
        }
    }
    /// 🧳️ Transfers the fully prestaged original operation only after inline codec authority is closed.
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<Vec<u8>> { if !self.is_complete() { return None; } self.output.take(grant) }
    pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> { if self.cursor_closed { Ok(self.output.next_close_byte_demand()) } else { self.cursor.next_close_byte_demand() } }
    /// 🍂️ Closes inline codec authority before retiring either original output allocation.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.closing = true;
        if !self.cursor_closed {
            let progress = self.cursor.close(grant).map_err(|error| error.reason)?;
            self.cursor_closed = progress.complete;
            if self.cursor_closed { self.pending_len = 0; self.pending_offset = 0; }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.processed_items, copied_bytes: progress.copied_bytes, retained_capacity_bytes: progress.retained_capacity_bytes, released_bytes: progress.released_bytes, ..Default::default() }));
        }
        Ok(self.output.close_step(grant))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.cursor_closed && self.output.terminal_is_empty() }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MountedOperationFramingPhase { Count, Encoding, Length, Payload, Release, Complete }

pub(crate) struct MountedPreparedOperationsBytes {
    output: MountedReceiptBytes,
    operation: Option<MountedPreparedOperationBytes>,
    encoded: ManuallyDrop<Option<Vec<u8>>>,
    count: usize,
    index: usize,
    phase: MountedOperationFramingPhase,
    integer: [u8; 10],
    integer_len: usize,
    integer_offset: usize,
    payload_offset: usize,
    closing: bool,
}

impl MountedPreparedOperationsBytes {
    fn integer(value: usize) -> ([u8; 10], usize) {
        let mut output = [0; 10]; let mut value = value; let mut count = 0;
        loop { let byte = (value & 127) as u8; value >>= 7; output[count] = byte | if value == 0 { 0 } else { 128 }; count += 1; if value == 0 { break; } }
        (output, count)
    }
    pub(crate) fn new(count: usize) -> Self { let (integer, integer_len) = Self::integer(count); Self { output: MountedReceiptBytes::new(), operation: (count > 0).then(MountedPreparedOperationBytes::new), encoded: ManuallyDrop::new(None), count, index: 0, phase: MountedOperationFramingPhase::Count, integer, integer_len, integer_offset: 0, payload_offset: 0, closing: false } }
    pub(crate) fn operation_index(&self) -> Option<usize> { (!self.closing && self.phase == MountedOperationFramingPhase::Encoding).then_some(self.index) }
    fn additional(&self) -> usize { match self.phase { MountedOperationFramingPhase::Count | MountedOperationFramingPhase::Length => self.integer_len - self.integer_offset, MountedOperationFramingPhase::Payload => (self.encoded.as_ref().unwrap().len() - self.payload_offset).min(64), _ => 0 } }
    pub(crate) fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.phase == MountedOperationFramingPhase::Encoding { self.operation.as_ref().unwrap().next_capacity_byte_demand() } else { self.output.next_capacity_byte_demand(self.additional()) } }
    pub(crate) fn next_release_byte_demand(&self) -> Result<usize, ValueError> { match self.phase { MountedOperationFramingPhase::Encoding => self.operation.as_ref().unwrap().next_growth_release_byte_demand(), MountedOperationFramingPhase::Release => Ok(self.encoded.as_ref().unwrap().capacity()), _ => Ok(self.output.next_growth_release_byte_demand()) } }
    pub(crate) fn is_complete(&self) -> bool { !self.closing && self.phase == MountedOperationFramingPhase::Complete }
    /// 📦️ Frames every original inverse operation only after its complete length is retained.
    pub(crate) fn advance(&mut self, source: Option<store::ArtifactPreparedOperationSource<'_>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted operation list is closing")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.phase == MountedOperationFramingPhase::Complete { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.phase == MountedOperationFramingPhase::Encoding {
            let operation = self.operation.as_mut().unwrap();
            if !operation.is_complete() { return operation.advance(source.ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "mounted operation list lost its original prepared codec"))?, grant); }
            *self.encoded = operation.take(grant);
            self.operation = None;
            (self.integer, self.integer_len) = Self::integer(self.encoded.as_ref().unwrap().len());
            self.integer_offset = 0; self.payload_offset = 0; self.phase = MountedOperationFramingPhase::Length;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.phase == MountedOperationFramingPhase::Release {
            let bytes = self.encoded.as_ref().unwrap().capacity();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            drop(self.encoded.take()); self.index += 1;
            self.phase = if self.index == self.count { MountedOperationFramingPhase::Complete } else { self.operation = Some(MountedPreparedOperationBytes::new()); MountedOperationFramingPhase::Encoding };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        let additional = self.additional();
        if additional > 0 {
            if !self.output.ready_for(additional) { return self.output.reserve_step(additional, grant); }
            let source = if self.phase == MountedOperationFramingPhase::Payload { &self.encoded.as_ref().unwrap()[self.payload_offset..self.payload_offset + additional] } else { &self.integer[self.integer_offset..self.integer_len] };
            let step = self.output.append_step(source, grant)?;
            if self.phase == MountedOperationFramingPhase::Payload { self.payload_offset += step.progress().copied_bytes; } else { self.integer_offset += step.progress().copied_bytes; }
            return Ok(step);
        }
        self.phase = match self.phase { MountedOperationFramingPhase::Count if self.count == 0 => MountedOperationFramingPhase::Complete, MountedOperationFramingPhase::Count => MountedOperationFramingPhase::Encoding, MountedOperationFramingPhase::Length => MountedOperationFramingPhase::Payload, MountedOperationFramingPhase::Payload => MountedOperationFramingPhase::Release, _ => unreachable!() };
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<Vec<u8>> { if !self.is_complete() { return None; } self.output.take(grant) }
    pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> { if let Some(operation) = self.operation.as_ref() { operation.next_close_byte_demand() } else if let Some(encoded) = self.encoded.as_ref() { Ok(encoded.capacity()) } else { Ok(self.output.next_close_byte_demand()) } }
    /// ♻️ Retains the exact operation, candidate, and framed backing until each original extent is funded.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.closing = true;
        if let Some(operation) = self.operation.as_mut() { let step = operation.close_step(grant)?; if operation.terminal_is_empty() { self.operation = None; } return Ok(step); }
        if let Some(encoded) = self.encoded.as_ref() {
            let bytes = encoded.capacity();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            drop(self.encoded.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        Ok(self.output.close_step(grant))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.operation.is_none() && self.encoded.is_none() && self.output.terminal_is_empty() }
}

impl Drop for MountedPreparedOperationsBytes { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mounted operation framing retains every original owner until funded transfer or close"); } }

pub(crate) struct MountedKernelMutationReceiptSource<'a> {
    pub(crate) document: ArtifactHandle,
    pub(crate) mutation_id: &'a str,
    pub(crate) invocation_id: &'a str,
    pub(crate) schema: &'a str,
    pub(crate) schema_separator: &'a str,
    pub(crate) schema_suffix: &'a str,
    pub(crate) author: &'a str,
    pub(crate) dependencies: &'a [MutationId],
    pub(crate) inverse: &'a [u8],
    pub(crate) base_version: ArtifactVersion,
    pub(crate) timestamp: HybridLogicalTimestamp,
    pub(crate) undo_policy: UndoPolicy,
}

impl MountedKernelMutationReceiptSource<'_> {
    fn identity(&self) -> [(usize, usize); 8] { [(self.mutation_id.as_ptr() as usize, self.mutation_id.len()), (self.invocation_id.as_ptr() as usize, self.invocation_id.len()), (self.schema.as_ptr() as usize, self.schema.len()), (self.schema_separator.as_ptr() as usize, self.schema_separator.len()), (self.schema_suffix.as_ptr() as usize, self.schema_suffix.len()), (self.author.as_ptr() as usize, self.author.len()), (self.dependencies.as_ptr() as usize, self.dependencies.len()), (self.inverse.as_ptr() as usize, self.inverse.len())] }
    fn text_parts(&self, field: usize) -> [&str; 4] { match field { 0 | 3 | 6 | 7 => [self.mutation_id, "", "", ""], 1 => [self.invocation_id, "", "", ""], 2 => [self.schema, self.schema_separator, self.schema_suffix, ""], 4 | 8 => [self.schema, self.schema_separator, self.schema_suffix, ".inverse"], 5 => [self.author, "", "", ""], _ => unreachable!() } }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MountedKernelReceiptPhase { Strings, Inverse, DependencyBacking, Dependencies, Complete }

pub(crate) struct MountedKernelMutationReceipt {
    identity: [(usize, usize); 8],
    document: ArtifactHandle,
    base_version: ArtifactVersion,
    timestamp: HybridLogicalTimestamp,
    undo_policy: UndoPolicy,
    strings: [MountedReceiptBytes; 9],
    forward: MountedReceiptBytes,
    inverse: [MountedReceiptBytes; 2],
    dependencies: [ManuallyDrop<Vec<MutationId>>; 3],
    dependency: Option<MountedReceiptBytes>,
    phase: MountedKernelReceiptPhase,
    field: usize,
    segment: usize,
    offset: usize,
    dependency_index: usize,
    closing: bool,
    transferred: bool,
}

impl MountedKernelMutationReceipt {
    pub(crate) fn frame_birth_bytes() -> usize { std::mem::size_of::<Self>() }
    pub(crate) fn new(source: &MountedKernelMutationReceiptSource<'_>, forward: Vec<u8>) -> Self { Self { identity: source.identity(), document: source.document, base_version: source.base_version, timestamp: source.timestamp, undo_policy: source.undo_policy, strings: std::array::from_fn(|_| MountedReceiptBytes::new()), forward: MountedReceiptBytes::from_original(forward), inverse: std::array::from_fn(|_| MountedReceiptBytes::new()), dependencies: std::array::from_fn(|_| ManuallyDrop::new(Vec::new())), dependency: Some(MountedReceiptBytes::new()), phase: MountedKernelReceiptPhase::Strings, field: 0, segment: 0, offset: 0, dependency_index: 0, closing: false, transferred: false } }
    fn validate(&self, source: &MountedKernelMutationReceiptSource<'_>) -> Result<(), ValueError> { if self.closing || self.transferred || self.identity != source.identity() || self.document != source.document || self.base_version != source.base_version || self.timestamp != source.timestamp || self.undo_policy != source.undo_policy { Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted host receipt lost its exact original causal source")) } else { Ok(()) } }
    fn active_bytes(&self) -> Option<&MountedReceiptBytes> { match self.phase { MountedKernelReceiptPhase::Strings => self.strings.get(self.field), MountedKernelReceiptPhase::Inverse => self.inverse.get(self.field), MountedKernelReceiptPhase::Dependencies => self.dependency.as_ref(), _ => None } }
    fn additional(&self, source: &MountedKernelMutationReceiptSource<'_>) -> usize { match self.phase { MountedKernelReceiptPhase::Strings => (source.text_parts(self.field)[self.segment].len() - self.offset).min(64), MountedKernelReceiptPhase::Inverse => (source.inverse.len() - self.offset).min(64), MountedKernelReceiptPhase::Dependencies if self.dependency_index < source.dependencies.len() => (source.dependencies[self.dependency_index].0.len() - self.offset).min(64), _ => 0 } }
    pub(crate) fn next_capacity_byte_demand(&self, source: &MountedKernelMutationReceiptSource<'_>) -> Result<usize, ValueError> {
        self.validate(source)?;
        if self.phase == MountedKernelReceiptPhase::DependencyBacking { return source.dependencies.len().checked_mul(std::mem::size_of::<MutationId>()).filter(|bytes| *bytes <= MOUNTED_RECEIPT_MAXIMUM_BYTES).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "mounted causal dependency backing exceeds original admission")); }
        self.active_bytes().map_or(Ok(0), |bytes| bytes.next_capacity_byte_demand(self.additional(source)))
    }
    pub(crate) fn next_release_byte_demand(&self) -> usize { self.active_bytes().map_or(0, MountedReceiptBytes::next_growth_release_byte_demand) }
    pub(crate) fn is_complete(&self) -> bool { !self.closing && !self.transferred && self.phase == MountedKernelReceiptPhase::Complete }
    /// 🧬️ Copies every original host-visible causal field and all independent dependency owners before staging.
    pub(crate) fn advance(&mut self, source: &MountedKernelMutationReceiptSource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.validate(source)?;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.phase == MountedKernelReceiptPhase::Complete { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.phase == MountedKernelReceiptPhase::DependencyBacking {
            let capacity = self.next_capacity_byte_demand(source)?;
            if grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(Default::default())); }
            *self.dependencies[self.field] = Vec::with_capacity(source.dependencies.len()); self.field += 1;
            if self.field == 3 { self.field = 0; self.phase = MountedKernelReceiptPhase::Dependencies; }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }));
        }
        if self.phase == MountedKernelReceiptPhase::Dependencies && self.dependency_index == source.dependencies.len() {
            if let Some(dependency) = self.dependency.as_mut() { let step = dependency.close_step(grant); if dependency.terminal_is_empty() { self.dependency = None; } return Ok(step); }
            self.field += 1; self.dependency_index = 0; self.offset = 0;
            if self.field == 3 { self.phase = MountedKernelReceiptPhase::Complete; } else { self.dependency = Some(MountedReceiptBytes::new()); }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let additional = self.additional(source);
        if additional > 0 {
            let borrowed = match self.phase { MountedKernelReceiptPhase::Strings => source.text_parts(self.field)[self.segment].as_bytes(), MountedKernelReceiptPhase::Inverse => source.inverse, MountedKernelReceiptPhase::Dependencies => source.dependencies[self.dependency_index].0.as_bytes(), _ => unreachable!() };
            let owner = match self.phase { MountedKernelReceiptPhase::Strings => &mut self.strings[self.field], MountedKernelReceiptPhase::Inverse => &mut self.inverse[self.field], MountedKernelReceiptPhase::Dependencies => self.dependency.as_mut().unwrap(), _ => unreachable!() };
            if !owner.ready_for(additional) { return owner.reserve_step(additional, grant); }
            let step = owner.append_step(&borrowed[self.offset..self.offset + additional], grant)?; self.offset += step.progress().copied_bytes;
            return Ok(step);
        }
        self.offset = 0;
        match self.phase {
            MountedKernelReceiptPhase::Strings => { self.segment += 1; if self.segment == 4 { self.segment = 0; self.field += 1; if self.field == 9 { self.field = 0; self.phase = MountedKernelReceiptPhase::Inverse; } } }
            MountedKernelReceiptPhase::Inverse => { self.field += 1; if self.field == 2 { self.field = 0; self.phase = MountedKernelReceiptPhase::DependencyBacking; } }
            MountedKernelReceiptPhase::Dependencies => { let bytes = self.dependency.as_mut().unwrap().take(grant).unwrap(); self.dependency = None; self.dependencies[self.field].push(MutationId(unsafe { String::from_utf8_unchecked(bytes) })); self.dependency_index += 1; if self.dependency_index < source.dependencies.len() { self.dependency = Some(MountedReceiptBytes::new()); } }
            _ => unreachable!(),
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }
    /// 🧳️ Assembles the complete mutation, undo ID, and undo inverse through constant original-owner moves.
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<(KernelMutation, MutationId, InverseMutation)> {
        if !self.is_complete() || grant.maximum_items == 0 { return None; }
        let [id, invocation, schema, target, inverse_schema, author, undo_id, undo_target, undo_schema] = std::array::from_fn(|index| unsafe { String::from_utf8_unchecked(self.strings[index].take(grant).unwrap()) });
        let [inverse, undo_inverse] = std::array::from_fn(|index| self.inverse[index].take(grant).unwrap());
        let dependencies = std::mem::take(&mut *self.dependencies[0]); let inverse_dependencies = std::mem::take(&mut *self.dependencies[1]); let undo_dependencies = std::mem::take(&mut *self.dependencies[2]);
        let forward = self.forward.take(grant).unwrap(); self.transferred = true;
        Some((KernelMutation { id: MutationId(id), document: self.document, base_version: self.base_version, invocation_id: InvocationId(invocation), diff: ArtifactDiff { schema: SchemaId(schema), payload: forward }, inverse: InverseMutation { target_mutation: MutationId(target), inverse_diff: ArtifactDiff { schema: SchemaId(inverse_schema), payload: inverse }, base_version: self.base_version, dependencies: inverse_dependencies, undo_policy: self.undo_policy }, dependencies, author: ActorId(author), timestamp: self.timestamp }, MutationId(undo_id), InverseMutation { target_mutation: MutationId(undo_target), inverse_diff: ArtifactDiff { schema: SchemaId(undo_schema), payload: undo_inverse }, base_version: self.base_version, dependencies: undo_dependencies, undo_policy: self.undo_policy }))
    }
    pub(crate) fn next_close_byte_demand(&self) -> usize {
        if !self.forward.terminal_is_empty() { return self.forward.next_close_byte_demand(); }
        if let Some(bytes) = self.strings.iter().chain(self.inverse.iter()).find(|bytes| !bytes.terminal_is_empty()) { return bytes.next_close_byte_demand(); }
        if let Some(bytes) = self.dependency.as_ref() { return bytes.next_close_byte_demand(); }
        for dependencies in &self.dependencies { if let Some(id) = dependencies.last() { return id.0.capacity(); } if dependencies.capacity() > 0 { return dependencies.capacity() * std::mem::size_of::<MutationId>(); } }
        0
    }
    /// 🍂️ Cancels each original payload, field, dependency string, and vector backing under whole physical grants.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        if self.terminal_is_empty() { return RetainedCloneStep::Complete(Default::default()); }
        if grant.maximum_items == 0 { return RetainedCloneStep::Progress(Default::default()); }
        self.closing = true;
        if !self.forward.terminal_is_empty() { return self.forward.close_step(grant); }
        if let Some(bytes) = self.strings.iter_mut().chain(self.inverse.iter_mut()).find(|bytes| !bytes.terminal_is_empty()) { return bytes.close_step(grant); }
        if let Some(bytes) = self.dependency.as_mut() { let step = bytes.close_step(grant); if bytes.terminal_is_empty() { self.dependency = None; } return step; }
        for dependencies in &mut self.dependencies {
            if let Some(id) = dependencies.last_mut() { let bytes = id.0.capacity(); if grant.maximum_release_bytes < bytes { return RetainedCloneStep::Progress(Default::default()); } drop(std::mem::take(&mut id.0)); dependencies.pop(); return RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }); }
            let bytes = dependencies.capacity() * std::mem::size_of::<MutationId>();
            if bytes > 0 { if grant.maximum_release_bytes < bytes { return RetainedCloneStep::Progress(Default::default()); } drop(std::mem::take(&mut **dependencies)); return RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }); }
        }
        RetainedCloneStep::Complete(Default::default())
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.forward.terminal_is_empty() && self.strings.iter().chain(self.inverse.iter()).all(MountedReceiptBytes::terminal_is_empty) && self.dependency.is_none() && self.dependencies.iter().all(|dependencies| dependencies.is_empty() && dependencies.capacity() == 0) }
}

impl Drop for MountedKernelMutationReceipt { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mounted host receipt retains every original causal field until funded transfer or close"); } }

pub(crate) struct MountedArtifactHandleIssuer {
    identity: Option<(usize, usize)>,
    offset: usize,
    first: u64,
    second: u64,
    complete: bool,
    closed: bool,
}

impl MountedArtifactHandleIssuer {
    pub(crate) fn new() -> Self { Self { identity: None, offset: 0, first: 0xcbf2_9ce4_8422_2325, second: 0xcbf2_9ce4_8422_2325 ^ 0x9e37_79b9_7f4a_7c15, complete: false, closed: false } }
    /// 🆔️ Hashes one exact borrowed UTF-8 prefix into the existing two-lane child document handle.
    pub(crate) fn advance(&mut self, source: &str, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.closed || self.identity.is_some_and(|identity| identity != (source.as_ptr() as usize, source.len())) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted child handle lost its exact original artifact identity")); }
        if self.complete { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let count = (source.len() - self.offset).min(grant.maximum_copy_bytes).min(64);
        if count == 0 && !source.is_empty() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.identity = Some((source.as_ptr() as usize, source.len()));
        for byte in &source.as_bytes()[self.offset..self.offset + count] { self.first = (self.first ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3); self.second = (self.second ^ u64::from(*byte).rotate_left(7)).wrapping_mul(0x0000_0100_0000_01b3); }
        self.offset += count; self.complete = self.offset == source.len();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() };
        Ok(if self.complete { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    pub(crate) fn ready(&self) -> bool { self.complete && !self.closed }
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<ArtifactHandle> { if !self.ready() || grant.maximum_items == 0 { return None; } self.closed = true; Some(ArtifactHandle((u128::from(self.first) << 64) | u128::from(self.second))) }
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep { if self.closed { return RetainedCloneStep::Complete(Default::default()); } if grant.maximum_items == 0 { return RetainedCloneStep::Progress(Default::default()); } self.identity = None; self.closed = true; RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }) }
}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");
