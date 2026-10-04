//! 📦️ Retained cloning and retirement for schema-preserving paged carriers.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement, admit_retained_clone_scaffold_retirement};
use crate::{
    SnapshotRetirementStep, ValueError, ValueRefusalKind,
    list::PagedList,
    paged::{PAGED_BYTES_CHUNK_BYTES, PagedBytes, PagedMap, PagedUtf8},
    retirement::{RetireOwned, RetirementCursor},
};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneProgress {
    match step {
        SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 },
        SnapshotRetirementStep::Blocked | SnapshotRetirementStep::Complete => RetainedCloneProgress::default(),
    }
}

pub struct PagedBytesCursor<const N: usize> {
    state: ManuallyDrop<PagedBytesCursorState<N>>,
}

#[doc(hidden)]
pub struct PagedBytesCursorState<const N: usize> {
    bytes: PagedList<u8, N>,
    output: Option<PagedBytes<N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl<const N: usize> Deref for PagedBytesCursor<N> {
    type Target = PagedBytesCursorState<N>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<const N: usize> DerefMut for PagedBytesCursor<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<const N: usize> Default for PagedBytesCursor<N> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(PagedBytesCursorState { bytes: PagedList::default(), output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default() }) }
    }
}

impl<const N: usize> Drop for PagedBytesCursor<N> {
    fn drop(&mut self) {
        let empty = self.bytes.terminal_is_empty() && self.output.is_none() && self.source.is_none() && self.close.is_empty();
        assert!(std::thread::panicking() || empty, "paged octet clone cursor abandoned before bounded terminal close");
        if empty {
            unsafe { ManuallyDrop::drop(&mut self.state) };
        }
    }
}

impl<const N: usize> RetainedCloneCursor<PagedBytes<N>> for PagedBytesCursor<N> {
    fn advance(&mut self, source: RetainedCloneRef<'_, PagedBytes<N>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is closing"));
        }
        source.bind(&mut self.source)?;
        match self.phase {
            0 => {
                let source = source.get().retained_bytes();
                if self.bytes.capacity() < source.len() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let required = self.bytes.next_capacity_allocation_bytes(source.len()).map_err(ValueError::from)?.ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone capacity state is inconsistent"))?;
                    if required > grant.maximum_capacity_bytes {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let progress = self.bytes.reserve_capacity_one(source.len(), grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    if !progress.progressed {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }));
                }
                if self.bytes.len() < source.len() {
                    if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let count = (source.len() - self.bytes.len()).min(grant.maximum_copy_bytes).min(PAGED_BYTES_CHUNK_BYTES);
                    let start = self.bytes.len();
                    for index in start..start + count {
                        self.bytes.push_reserved(*source.get(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone source changed"))?).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone lost its reserved capacity"))?;
                    }
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0 }));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                self.output = Some(PagedBytes::from_retained_bytes(std::mem::take(&mut self.bytes)));
                self.phase = 1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            1 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is spent")),
        }
    }

    fn take(&mut self) -> Option<PagedBytes<N>> {
        if self.phase != 1 {
            return None;
        }
        self.phase = 2;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if !self.bytes.terminal_is_empty() {
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let bytes = std::mem::take(&mut self.bytes);
            self.close.begin(bytes)?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.output.is_some() {
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let output = self.output.take().expect("checked paged octet output owner");
            self.close.begin(output)?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.terminal_is_empty() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<const N: usize> RetireOwned for PagedBytes<N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_bytes().retirement()
    }
}

impl<const N: usize> RetainedClone for PagedBytes<N> {
    type Cursor = PagedBytesCursor<N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedBytesCursor::default()
    }
}

pub struct PagedUtf8Cursor<const N: usize> {
    inner: <PagedList<String, N> as RetainedClone>::Cursor,
    chunks: Option<PagedList<String, N>>,
    output: Option<PagedUtf8<N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl<const N: usize> Default for PagedUtf8Cursor<N> {
    fn default() -> Self {
        Self { inner: PagedList::retained_clone_cursor(), chunks: None, output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<const N: usize> RetainedCloneCursor<PagedUtf8<N>> for PagedUtf8Cursor<N> {
    fn advance(&mut self, source: RetainedCloneRef<'_, PagedUtf8<N>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is closing"));
        }
        source.bind(&mut self.source)?;
        match self.phase {
            0 => match self.inner.advance(source.project(1, PagedUtf8::retained_chunks), grant)? {
                RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "paged UTF-8 chunks")?)),
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(grant, progress, "paged UTF-8 chunks")?;
                    self.chunks = Some(self.inner.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 chunk cursor completed without its owner"))?);
                    let _ = self.inner.begin_close();
                    self.phase = 1;
                    Ok(RetainedCloneStep::Progress(progress))
                }
            },
            1 => {
                let step = admit_retained_clone_scaffold_retirement(self.inner.close_step(grant.maximum_items, grant.maximum_copy_bytes)?, grant.maximum_items, grant.maximum_copy_bytes, "paged UTF-8 chunk cursor close")?;
                if step != SnapshotRetirementStep::Complete {
                    return Ok(RetainedCloneStep::Progress(close_progress(step)));
                }
                if !self.inner.terminal_is_empty() {
                    return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 chunk cursor completed close with a live owner"));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            2 => {
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < size_of::<usize>() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(PagedUtf8::from_retained_chunks_cloned(self.chunks.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone lost its chunks"))?, source.get().len()));
                self.phase = 3;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0 }))
            }
            3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is spent")),
        }
    }

    fn take(&mut self) -> Option<PagedUtf8<N>> {
        if self.phase != 3 {
            return None;
        }
        self.phase = 4;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        let _ = self.inner.begin_close();
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.inner.terminal_is_empty() {
            let step = admit_retained_clone_retirement(self.inner.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "paged UTF-8 inner close")?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !self.inner.terminal_is_empty() {
                return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 inner close completed with a live owner"));
            }
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.chunks, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.inner.terminal_is_empty() && self.chunks.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<const N: usize> RetireOwned for PagedUtf8<N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_chunks().retirement()
    }
}

impl<const N: usize> RetainedClone for PagedUtf8<N> {
    type Cursor = PagedUtf8Cursor<N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedUtf8Cursor::default()
    }
}

pub struct PagedMapCursor<V: RetainedClone, const N: usize> {
    inner: <PagedList<(String, V), N> as RetainedClone>::Cursor,
    entries: Option<PagedList<(String, V), N>>,
    output: Option<PagedMap<V, N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl<V: RetainedClone, const N: usize> Default for PagedMapCursor<V, N> {
    fn default() -> Self {
        Self { inner: PagedList::retained_clone_cursor(), entries: None, output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<V: RetainedClone, const N: usize> RetainedCloneCursor<PagedMap<V, N>> for PagedMapCursor<V, N> {
    fn advance(&mut self, source: RetainedCloneRef<'_, PagedMap<V, N>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is closing"));
        }
        source.bind(&mut self.source)?;
        match self.phase {
            0 => match self.inner.advance(source.project(1, PagedMap::retained_entries), grant)? {
                RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "paged map entries")?)),
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(grant, progress, "paged map entries")?;
                    self.entries = Some(self.inner.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged map entry cursor completed without its owner"))?);
                    let _ = self.inner.begin_close();
                    self.phase = 1;
                    Ok(RetainedCloneStep::Progress(progress))
                }
            },
            1 => {
                let step = admit_retained_clone_scaffold_retirement(self.inner.close_step(grant.maximum_items, grant.maximum_copy_bytes)?, grant.maximum_items, grant.maximum_copy_bytes, "paged map entry cursor close")?;
                if step != SnapshotRetirementStep::Complete {
                    return Ok(RetainedCloneStep::Progress(close_progress(step)));
                }
                if !self.inner.terminal_is_empty() {
                    return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map entry cursor completed close with a live owner"));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            2 => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(PagedMap::from_retained_entries_cloned(self.entries.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged map retained clone lost its entries"))?));
                self.phase = 3;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is spent")),
        }
    }

    fn take(&mut self) -> Option<PagedMap<V, N>> {
        if self.phase != 3 {
            return None;
        }
        self.phase = 4;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        let _ = self.inner.begin_close();
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.inner.terminal_is_empty() {
            let step = admit_retained_clone_retirement(self.inner.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "paged map inner close")?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !self.inner.terminal_is_empty() {
                return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map inner close completed with a live owner"));
            }
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.entries, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.inner.terminal_is_empty() && self.entries.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<V: RetireOwned + Send + Sync + 'static, const N: usize> RetireOwned for PagedMap<V, N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_entries().retirement()
    }
}

impl<V: RetainedClone, const N: usize> RetainedClone for PagedMap<V, N> {
    type Cursor = PagedMapCursor<V, N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedMapCursor::default()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
