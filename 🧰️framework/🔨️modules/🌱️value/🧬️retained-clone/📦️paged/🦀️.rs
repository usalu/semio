//! 📦️ Retained cloning and retirement for schema-preserving paged carriers.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress};
use crate::{
    ValueError, ValueRefusalKind,
    list::PagedList,
    paged::{PAGED_BYTES_CHUNK_BYTES, PagedBytes, PagedMap, PagedUtf8},
    retirement::{RetireOwned, RetirementCursor},
};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

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
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
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
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0, released_bytes: 0 }));
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

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged owner must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        let state = &mut *self.state;
        if !state.bytes.terminal_is_empty() { if let Some(step) = state.close.begin_default_granted(&mut state.bytes, grant)? { return Ok(step); } }
        if let Some(step) = state.close.begin_granted(&mut state.output, grant)? { return Ok(step); }
        super::close_retained_binding(&mut state.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(!self.bytes.terminal_is_empty()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if !self.bytes.terminal_is_empty() { return self.close.next_owner_capacity_with_binding::<PagedList<u8,N>>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<PagedBytes<N>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { if !self.closing { Ok(0) } else { self.close.next_release_with_binding(&self.source) } }
    fn terminal_is_empty(&self) -> bool { self.closing && self.bytes.terminal_is_empty() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<const N: usize> RetireOwned for PagedBytes<N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_bytes().retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<super::paged_list::PagedListRetirement<u8, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
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
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        match self.phase {
            0 => match self.inner.advance(source.project(1, PagedUtf8::retained_chunks), grant)? {
                RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "paged UTF-8 chunks")?)),
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(grant, progress, "paged UTF-8 chunks")?;
                    self.phase = 5;
                    Ok(RetainedCloneStep::Progress(progress))
                }
            },
            5 => {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.chunks = Some(self.inner.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 chunk cursor completed without its owner"))?);
                let _ = self.inner.begin_close();
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            1 => {
                if !self.inner.terminal_is_empty() {
                    let step = self.inner.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, self.inner.terminal_is_empty(), "paged UTF-8 chunk cursor close")?.progress()));
                }
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            2 => {
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < size_of::<usize>() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(PagedUtf8::from_retained_chunks_cloned(self.chunks.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone lost its chunks"))?, source.get().len()));
                self.phase = 3;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0, released_bytes: 0 }))
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

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged owner must begin close before granted retirement")); }
        if !self.inner.terminal_is_empty() { let step = self.inner.close_step(grant)?; return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, self.inner.terminal_is_empty(), "retained child close")?.progress())); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.chunks, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        super::close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if !self.inner.terminal_is_empty(){return self.inner.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.chunks.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.chunks.is_some() { return self.close.next_owner_capacity_with_binding::<PagedList<String,N>>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<PagedUtf8<N>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.inner.terminal_is_empty() && self.chunks.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<const N: usize> RetireOwned for PagedUtf8<N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_chunks().retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<super::paged_list::PagedListRetirement<String, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}

impl<const N: usize> RetainedClone for PagedUtf8<N> {
    type Cursor = PagedUtf8Cursor<N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedUtf8Cursor::default()
    }
}

/// 🔎️ Compares original paged UTF-8 identifiers with stable chunk and byte offsets.
#[derive(Default)]
pub struct PagedUtf8BoundedOrdCursor<const N: usize> {
    initialized:bool,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    left_chunk: usize,
    right_chunk: usize,
    left_byte: usize,
    right_byte: usize,
    complete: Option<std::cmp::Ordering>,
    closing: bool,
}

impl<const N: usize> super::ordered_map::BoundedOrdCursor<PagedUtf8<N>> for PagedUtf8BoundedOrdCursor<N> {
fn advance_retirement_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(if self.left.is_none()||self.right.is_none(){crate::RetirementDemand{copy_bytes:super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()}}else{Default::default()})}
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}super::close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant)}
    fn compare(&mut self, left: RetainedCloneRef<'_, PagedUtf8<N>>, right: RetainedCloneRef<'_, PagedUtf8<N>>, grant: super::ordered_map::BoundedOrdGrant, retirement:RetainedCloneGrant) -> Result<super::ordered_map::BoundedOrdStep, ValueError> {
        use super::ordered_map::{BoundedOrdProgress, BoundedOrdStep};
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged identifier comparator is closing")); }
        let first = !self.initialized;
        if first && grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
        if let Some(progress)=left.bind(&mut self.left,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)=right.bind(&mut self.right,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering) = self.complete { return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress::default() }); }
        if first {self.initialized=true; return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 })); }
        let left = left.get().retained_chunks();
        let right = right.get().retained_chunks();
        if left.get(self.left_chunk).is_some_and(|chunk| chunk.len() == self.left_byte) {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            self.left_chunk += 1;
            self.left_byte = 0;
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        if right.get(self.right_chunk).is_some_and(|chunk| chunk.len() == self.right_byte) {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            self.right_chunk += 1;
            self.right_byte = 0;
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        let left_available = left.get(self.left_chunk).is_some();
        let right_available = right.get(self.right_chunk).is_some();
        if !left_available || !right_available {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            let ordering = left_available.cmp(&right_available);
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        let mut progress = BoundedOrdProgress::default();
        while progress.compared_bytes < grant.maximum_bytes {
            let l = left.get(self.left_chunk).and_then(|chunk| chunk.as_bytes().get(self.left_byte)).copied();
            let r = right.get(self.right_chunk).and_then(|chunk| chunk.as_bytes().get(self.right_byte)).copied();
            let ordering = match (l, r) {
                (None, None) | (None, Some(_)) | (Some(_), None) => {
                    if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(progress)); }
                    progress.compared_items = 1;
                    l.cmp(&r)
                }
                (Some(l), Some(r)) => {
                    progress.compared_bytes += 1;
                    self.left_byte += 1;
                    self.right_byte += 1;
                    if self.left_byte == left[self.left_chunk].len() { self.left_chunk += 1; self.left_byte = 0; }
                    if self.right_byte == right[self.right_chunk].len() { self.right_chunk += 1; self.right_byte = 0; }
                    if l == r { continue; }
                    l.cmp(&r)
                }
            };
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress });
        }
        Ok(BoundedOrdStep::Progress(progress))
    }
}

impl<const N: usize> super::ordered_map::BoundedOrd for PagedUtf8<N> {
    type Cursor = PagedUtf8BoundedOrdCursor<N>;
    fn bounded_ord_cursor() -> Self::Cursor { PagedUtf8BoundedOrdCursor::default() }
}

pub struct PagedMapCursor<V: RetainedClone, const N: usize> {
    inner: <PagedList<(PagedUtf8<{usize::MAX}>, V), N> as RetainedClone>::Cursor,
    entries: Option<PagedList<(PagedUtf8<{usize::MAX}>, V), N>>,
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
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        match self.phase {
            0 => match self.inner.advance(source.project(1, PagedMap::retained_entries), grant)? {
                RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "paged map entries")?)),
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(grant, progress, "paged map entries")?;
                    self.phase = 5;
                    Ok(RetainedCloneStep::Progress(progress))
                }
            },
            5 => {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.entries = Some(self.inner.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged map entry cursor completed without its owner"))?);
                let _ = self.inner.begin_close();
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            1 => {
                if !self.inner.terminal_is_empty() {
                    let step = self.inner.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, self.inner.terminal_is_empty(), "paged object entry cursor close")?.progress()));
                }
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
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

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged owner must begin close before granted retirement")); }
        if !self.inner.terminal_is_empty() { let step = self.inner.close_step(grant)?; return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, self.inner.terminal_is_empty(), "retained child close")?.progress())); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.entries, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        super::close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if !self.inner.terminal_is_empty(){return self.inner.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.entries.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.entries.is_some() { return self.close.next_owner_capacity_with_binding::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<PagedMap<V,N>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.inner.terminal_is_empty() { return self.inner.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.inner.terminal_is_empty() && self.entries.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<V: RetireOwned + Send + Sync + 'static, const N: usize> RetireOwned for PagedMap<V, N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_retained_entries().retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<super::paged_list::PagedListRetirement<(PagedUtf8<{usize::MAX}>, V), N>>()) }
    fn controlled_retirement_supported() -> bool { V::controlled_retirement_supported() }
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
