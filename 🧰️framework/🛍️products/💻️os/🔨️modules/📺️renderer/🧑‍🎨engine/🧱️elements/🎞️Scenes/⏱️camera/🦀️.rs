//! ⏱️ Original camera identities have one exact text allocation and separately paid directory custody.
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetainedCloneStep, ValueError, ValueRefusalKind};
use std::{alloc::{alloc, Layout}, mem::{size_of, ManuallyDrop}};

pub const CAPACITY: usize = 256;
const IDENTIFIER_BYTES: usize = 256;
fn error(kind: ValueRefusalKind, reason: &'static str) -> ValueError { ValueError::literal(kind, reason) }
fn demand(grant: RetainedCloneGrant, copy: usize, birth: usize, release: usize) -> Result<(), ValueError> {
    if grant.maximum_items == 0 { return Err(error(ValueRefusalKind::WorkLimit, "camera item authority is absent")); }
    if grant.maximum_depth == 0 { return Err(error(ValueRefusalKind::DepthLimit, "camera depth authority is absent")); }
    if copy > grant.maximum_copy_bytes || birth > grant.maximum_capacity_bytes || release > grant.maximum_release_bytes { return Err(error(ValueRefusalKind::OwnershipLimit, "camera original extent exceeds caller authority")); }
    Ok(())
}
fn progress(copy: usize, birth: usize, release: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: birth, released_bytes: release } }
fn close_body(body: &mut Option<Box<[u8]>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let Some(original) = body.as_ref() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
    let release = Layout::array::<u8>(original.len()).map_err(|_| error(ValueRefusalKind::OwnershipLimit, "camera body layout overflow"))?.size();
    let copy = size_of::<Option<Box<[u8]>>>();
    if demand(grant, copy, 0, release).is_err() { return Ok(RetainedCloneStep::Progress(Default::default())); }
    drop(body.take());
    Ok(RetainedCloneStep::Complete(progress(copy, 0, release)))
}

pub struct Record<M: Copy> {
    body: Option<Box<[u8]>>,
    spans: [(usize, usize); 6],
    pub metadata: M,
    pub at_ms: f64,
    pub processed: bool,
}
impl<M: Copy> Record<M> {
    fn admit(text: [&str; 6], metadata: M, at_ms: f64, grant: RetainedCloneGrant, headers: usize) -> Result<(Self, RetainedCloneProgress), ValueError> {
        if text[0].len() > IDENTIFIER_BYTES || text[1].len() > IDENTIFIER_BYTES { return Err(error(ValueRefusalKind::WorkLimit, "camera identifier exceeds its original byte policy")); }
        let bytes = text.iter().try_fold(0usize, |sum, value| sum.checked_add(value.len())).ok_or_else(|| error(ValueRefusalKind::OwnershipLimit, "camera packed identity extent overflow"))?;
        let copy = bytes.checked_add(headers).ok_or_else(|| error(ValueRefusalKind::OwnershipLimit, "camera packed metadata extent overflow"))?;
        let layout = Layout::array::<u8>(bytes).map_err(|_| error(ValueRefusalKind::OwnershipLimit, "camera packed identity layout overflow"))?;
        demand(grant, copy, layout.size(), 0)?;
        let mut spans = [(0, 0); 6];
        let body = if bytes == 0 { Vec::new().into_boxed_slice() } else {
            let pointer = unsafe { alloc(layout) };
            if pointer.is_null() { return Err(error(ValueRefusalKind::AllocationFailed, "camera packed identity allocation refused")); }
            let mut offset = 0;
            for (index, value) in text.iter().enumerate() {
                spans[index] = (offset, value.len());
                unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), pointer.add(offset), value.len()); }
                offset += value.len();
            }
            unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(pointer, bytes)) }
        };
        Ok((Self { body: Some(body), spans, metadata, at_ms, processed: false }, progress(copy, layout.size(), 0)))
    }
    pub fn text(&self, index: usize) -> &str {
        let (start, len) = self.spans[index];
        std::str::from_utf8(&self.body.as_ref().expect("original camera body remains owned")[start..start + len]).expect("camera spans retain original UTF8 boundaries")
    }
    pub fn original_body_ptr(&self) -> Option<*const u8> { self.body.as_ref().map(|body| body.as_ptr()) }
    pub fn original_bytes(&self) -> &[u8] { self.body.as_deref().unwrap_or(&[]) }
    pub fn terminal_is_empty(&self) -> bool { self.body.is_none() }
    fn same_text(&self, text: [&str; 6]) -> bool { text.iter().enumerate().all(|(index, value)| self.text(index) == *value) }
}

struct DirectoryPayload<M: Copy> {
    entries: [Option<Record<M>>; CAPACITY],
    retiring: [Option<Box<[u8]>>; CAPACITY],
    len: usize,
}
impl<M: Copy> DirectoryPayload<M> {
    fn empty() -> Self { Self { entries: std::array::from_fn(|_| None), retiring: std::array::from_fn(|_| None), len: 0 } }
    fn empty_witness(&self) -> bool { self.entries.iter().all(Option::is_none) && self.retiring.iter().all(Option::is_none) && self.len == 0 }
    fn find(&self, host: &str) -> Option<usize> { self.entries.iter().position(|entry| entry.as_ref().is_some_and(|entry| !entry.terminal_is_empty() && entry.text(0) == host)) }
}

pub enum RestoreStep { Returned { progress: RetainedCloneProgress }, Retained { progress: RetainedCloneProgress } }

pub struct DirectoryOwner<M: Copy> {
    original: ManuallyDrop<DirectoryPayload<M>>,
    authority: Option<RetainedCloneGrant>,
    observed: RetainedCloneProgress,
}
impl<M: Copy> Default for DirectoryOwner<M> { fn default() -> Self { Self { original: ManuallyDrop::new(DirectoryPayload::empty()), authority: None, observed: RetainedCloneProgress::default() } } }
impl<M: Copy + PartialEq> DirectoryOwner<M> {
    /// 🪙️ Installs the explicit incoming bootstrap authority without creating a policy default.
    pub fn install(&mut self, grant: RetainedCloneGrant) -> Result<(), ValueError> {
        if self.authority.is_some_and(|original| original != grant) { return Err(error(ValueRefusalKind::InvariantViolated, "camera bootstrap authority changed while installed")); }
        self.authority = Some(grant);
        Ok(())
    }
    pub fn received_grant(&self) -> Result<RetainedCloneGrant, ValueError> { self.authority.ok_or_else(|| error(ValueRefusalKind::UnsupportedOwner, "camera requires original bootstrap authority")) }
    fn admit_grant(&self, grant: RetainedCloneGrant) -> Result<(), ValueError> {
        let original = self.received_grant()?;
        if grant.maximum_items > original.maximum_items || grant.maximum_copy_bytes > original.maximum_copy_bytes || grant.maximum_capacity_bytes > original.maximum_capacity_bytes || grant.maximum_release_bytes > original.maximum_release_bytes || grant.maximum_depth > original.maximum_depth { return Err(error(ValueRefusalKind::InvariantViolated, "camera receiver increased original authority")); }
        Ok(())
    }
    fn observe(&mut self, receipt: RetainedCloneProgress) {
        self.observed.copied_items = self.observed.copied_items.checked_add(receipt.copied_items).expect("camera item receipt overflow");
        self.observed.copied_bytes = self.observed.copied_bytes.checked_add(receipt.copied_bytes).expect("camera copy receipt overflow");
        self.observed.retained_capacity_bytes = self.observed.retained_capacity_bytes.checked_add(receipt.retained_capacity_bytes).expect("camera birth receipt overflow");
        self.observed.released_bytes = self.observed.released_bytes.checked_add(receipt.released_bytes).expect("camera release receipt overflow");
    }
    pub fn observed_progress(&self) -> RetainedCloneProgress { self.observed }
    pub fn len(&self) -> usize { self.original.len }
    pub fn get(&self, host: &str) -> Option<&Record<M>> { self.original.find(host).and_then(|index| self.original.entries[index].as_ref()) }
    /// 📦️ Creates one exact packed original or updates an unchanged original's scalar deadline.
    pub fn schedule(&mut self, text: [&str; 6], metadata: M, at_ms: f64, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.admit_grant(grant)?;
        let existing = self.original.find(text[0]);
        if let Some(index) = existing {
            let record = self.original.entries[index].as_mut().unwrap();
            if record.metadata == metadata && record.same_text(text) {
                demand(grant, size_of::<f64>(), 0, 0)?;
                record.at_ms = at_ms;
                let receipt = progress(size_of::<f64>(), 0, 0);
                self.observe(receipt);
                return Ok(receipt);
            }
            if self.original.retiring[index].is_some() { return Err(error(ValueRefusalKind::OwnershipLimit, "camera replacement retains an earlier original body")); }
        }
        let index = existing.or_else(|| self.original.entries.iter().zip(self.original.retiring.iter()).position(|(entry, retired)| entry.is_none() && retired.is_none())).ok_or_else(|| error(ValueRefusalKind::WorkLimit, "camera original entry capacity exceeded"))?;
        let headers = size_of::<Record<M>>() + size_of::<Option<Record<M>>>() + size_of::<usize>();
        let (record, receipt) = Record::admit(text, metadata, at_ms, grant, headers)?;
        if let Some(mut previous) = self.original.entries[index].take() { self.original.retiring[index] = previous.body.take(); } else { self.original.len += 1; }
        self.original.entries[index] = Some(record);
        self.observe(receipt);
        Ok(receipt)
    }
    /// 🧳️ Returns the same original record under a separate paid metadata move.
    pub fn restore(&mut self, cursor: &mut DirectoryCursor<M>, grant: RetainedCloneGrant) -> Result<RestoreStep, ValueError> {
        self.admit_grant(grant)?;
        let Some((source, record)) = cursor.peek() else { return Ok(RestoreStep::Returned { progress: Default::default() }); };
        if let Some(target) = self.original.find(record.text(0)) {
            let active = self.original.entries[target].as_ref().unwrap();
            let same_owner = active.metadata == record.metadata && (2..6).all(|index| active.text(index) == record.text(index));
            if same_owner && active.at_ms < record.at_ms {
                if self.original.retiring[target].is_some() { return Err(error(ValueRefusalKind::OwnershipLimit, "camera restore retains an earlier target body")); }
                let copy = 2 * size_of::<Option<Record<M>>>() + size_of::<Option<Box<[u8]>>>() + size_of::<usize>();
                demand(grant, copy, 0, 0)?;
                let source_owner = cursor.original.as_mut().unwrap();
                self.original.retiring[target] = self.original.entries[target].take().unwrap().body.take();
                self.original.entries[target] = source_owner.entries[source].take();
                source_owner.len -= 1;
                cursor.scan = source + 1;
                let receipt = progress(copy, 0, 0);
                self.observe(receipt);
                return Ok(RestoreStep::Returned { progress: receipt });
            }
            return Ok(RestoreStep::Retained { progress: Default::default() });
        }
        let Some(target) = self.original.entries.iter().zip(self.original.retiring.iter()).position(|(entry, retired)| entry.is_none() && retired.is_none()) else { return Ok(RestoreStep::Retained { progress: Default::default() }); };
        let copy = 2 * size_of::<Option<Record<M>>>() + 2 * size_of::<usize>();
        demand(grant, copy, 0, 0)?;
        let original = cursor.original.as_mut().unwrap();
        self.original.entries[target] = original.entries[source].take();
        self.original.len += 1;
        original.len -= 1;
        cursor.scan = source + 1;
        Ok(RestoreStep::Returned { progress: progress(copy, 0, 0) })
    }
    /// 🚪️ Invalidates a logical deadline while retaining its original body for paid release.
    pub fn remove(&mut self, host: &str, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.admit_grant(grant)?;
        let Some(index) = self.original.find(host) else { return Ok(Default::default()); };
        if self.original.retiring[index].is_some() { return Err(error(ValueRefusalKind::OwnershipLimit, "camera remove retains an earlier original body")); }
        let copy = size_of::<Option<Record<M>>>() + size_of::<Option<Box<[u8]>>>() + size_of::<usize>();
        demand(grant, copy, 0, 0)?;
        self.original.retiring[index] = self.original.entries[index].take().unwrap().body.take();
        self.original.len -= 1;
        let receipt = progress(copy, 0, 0);
        self.observe(receipt);
        Ok(receipt)
    }
    pub fn transfer_demands() -> (usize, usize) { let bytes = Layout::new::<DirectoryPayload<M>>().size(); (bytes + size_of::<Box<DirectoryPayload<M>>>(), bytes) }
    /// 🪢️ Transfers the existing native bodies without cloning any identity or guessing backing extent.
    pub fn begin(&mut self, grant: RetainedCloneGrant) -> Result<(DirectoryCursor<M>, RetainedCloneProgress), ValueError> {
        self.admit_grant(grant)?;
        let (copy, birth) = Self::transfer_demands();
        demand(grant, copy, birth, 0)?;
        let pointer = unsafe { alloc(Layout::new::<DirectoryPayload<M>>()) }.cast::<DirectoryPayload<M>>();
        if pointer.is_null() { return Err(error(ValueRefusalKind::AllocationFailed, "camera original directory allocation refused")); }
        let payload = std::mem::replace(&mut *self.original, DirectoryPayload::empty());
        unsafe { pointer.write(payload); }
        let original = unsafe { Box::from_raw(pointer) };
        Ok((DirectoryCursor { original: ManuallyDrop::new(Some(original)), scan: 0, authority: self.received_grant()? }, progress(copy, birth, 0)))
    }
    /// 🧾️ Retires one original global body or header without moving authority to a new allocation.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.admit_grant(grant)?;
        if let Some(index) = self.original.retiring.iter().position(Option::is_some) {
            let step = close_body(&mut self.original.retiring[index], grant)?;
            self.observe(step.progress());
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(index) = self.original.entries.iter().position(Option::is_some) {
            if !self.original.entries[index].as_ref().unwrap().terminal_is_empty() {
                let step = close_body(&mut self.original.entries[index].as_mut().unwrap().body, grant)?;
                self.observe(step.progress());
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            let copy = size_of::<Option<Record<M>>>() + size_of::<usize>();
            if demand(grant, copy, 0, 0).is_err() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.original.entries[index] = None;
            self.original.len -= 1;
            let receipt = progress(copy, 0, 0);
            self.observe(receipt);
            return Ok(RetainedCloneStep::Progress(receipt));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn terminal_is_empty(&self) -> bool { self.original.empty_witness() }
}
impl<M: Copy> Drop for DirectoryOwner<M> {
    fn drop(&mut self) {
        assert!(self.original.empty_witness() || std::thread::panicking(), "camera directory abandoned original native body custody");
        if self.original.empty_witness() { unsafe { ManuallyDrop::drop(&mut self.original); } }
    }
}

pub struct DirectoryCursor<M: Copy> { original: ManuallyDrop<Option<Box<DirectoryPayload<M>>>>, scan: usize, authority: RetainedCloneGrant }
impl<M: Copy> DirectoryCursor<M> {
    fn admit_grant(&self, grant: RetainedCloneGrant) -> Result<(), ValueError> {
        let original = self.authority;
        if grant.maximum_items > original.maximum_items || grant.maximum_copy_bytes > original.maximum_copy_bytes || grant.maximum_capacity_bytes > original.maximum_capacity_bytes || grant.maximum_release_bytes > original.maximum_release_bytes || grant.maximum_depth > original.maximum_depth { return Err(error(ValueRefusalKind::InvariantViolated, "camera cursor increased original authority")); }
        Ok(())
    }
    pub fn len(&self) -> usize { self.original.as_ref().map_or(0, |original| original.len) }
    pub fn original_directory_ptr(&self) -> Option<*const ()> { self.original.as_ref().map(|original| &**original as *const DirectoryPayload<M> as *const ()) }
    pub fn peek(&self) -> Option<(usize, &Record<M>)> { self.original.as_ref().and_then(|original| (self.scan..CAPACITY).find_map(|index| original.entries[index].as_ref().map(|record| (index, record)))) }
    pub fn has_retiring(&self) -> bool { self.original.as_ref().is_some_and(|original| original.retiring.iter().any(Option::is_some)) }
    /// 🧹️ Frees one exact previous body while retaining the original directory allocation.
    pub fn close_retiring(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.admit_grant(grant)?;
        let Some(original) = self.original.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        let Some(index) = original.retiring.iter().position(Option::is_some) else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        close_body(&mut original.retiring[index], grant)
    }
    pub fn mark_processed(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.admit_grant(grant)?;
        let Some((index, _)) = self.peek() else { return Ok(Default::default()); };
        demand(grant, size_of::<bool>(), 0, 0)?;
        self.original.as_mut().unwrap().entries[index].as_mut().unwrap().processed = true;
        Ok(progress(size_of::<bool>(), 0, 0))
    }
    /// ♻️ Releases the actual body before a separately funded original entry-header transition.
    pub fn close_current(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.admit_grant(grant)?;
        let Some((index, record)) = self.peek() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !record.terminal_is_empty() { return close_body(&mut self.original.as_mut().unwrap().entries[index].as_mut().unwrap().body, grant); }
        let copy = size_of::<Option<Record<M>>>() + size_of::<usize>();
        if demand(grant, copy, 0, 0).is_err() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let original = self.original.as_mut().unwrap();
        original.entries[index] = None;
        original.len -= 1;
        self.scan = index + 1;
        Ok(RetainedCloneStep::Progress(progress(copy, 0, 0)))
    }
    /// 🏁️ Quotes and returns the original directory allocation only after every body is absent.
    pub fn close_backing(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.admit_grant(grant)?;
        let Some(original) = self.original.as_ref() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !original.empty_witness() { return Err(error(ValueRefusalKind::InvariantViolated, "camera backing still owns original bodies")); }
        let copy = size_of::<Option<Box<DirectoryPayload<M>>>>();
        let release = Layout::new::<DirectoryPayload<M>>().size();
        if demand(grant, copy, 0, release).is_err() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        drop(self.original.take());
        Ok(RetainedCloneStep::Complete(progress(copy, 0, release)))
    }
    pub fn terminal_is_empty(&self) -> bool { self.original.is_none() }
}
impl<M: Copy> Drop for DirectoryCursor<M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "camera cursor abandoned original directory custody");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.original); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
