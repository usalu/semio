//! 🏷️ Retains validated canonical symbol spans in their original immutable wire owner.

use crate::format::{RetainedUtf8Cursor, RetainedVarintCursor, RetainedVarintStep};
use semio_framework_value::list::PagedList;
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlineSymbolRefusal { Incomplete, Noncanonical, Utf8, SymbolCount, SymbolLength, Source, Ownership }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolSpan { pub start: usize, pub end: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Count, Length, Payload, Place, Complete }

/// 🧷️ The original source authority remains immutable and retained until terminal closure.
pub struct RetainedInlineSymbols {
    binding: Option<RetainedCloneBinding>,
    symbols: PagedList<SymbolSpan, {usize::MAX}>,
    pending: Option<SymbolSpan>,
    varint: RetainedVarintCursor,
    utf8: RetainedUtf8Cursor,
    offset: usize,
    count: usize,
    maximum_symbols: usize,
    maximum_symbol_bytes: usize,
    remaining: usize,
    phase: Phase,
    closing: bool,
    fault: Option<InlineSymbolRefusal>,
}

impl RetainedInlineSymbols {
    pub fn new(symbol_offset: usize, maximum_symbols: usize, maximum_symbol_bytes: usize) -> Self {
        Self { binding: None, symbols: PagedList::default(), pending: None, varint: RetainedVarintCursor::default(), utf8: RetainedUtf8Cursor::default(), offset: symbol_offset, count: 0, maximum_symbols, maximum_symbol_bytes, remaining: 0, phase: Phase::Count, closing: false, fault: None }
    }

    pub fn is_finished(&self) -> bool { self.phase == Phase::Complete && self.fault.is_none() && !self.closing }
    pub fn prefix_bytes(&self) -> Option<usize> { self.is_finished().then_some(self.offset) }
    pub fn len(&self) -> usize { self.symbols.len() }
    pub fn is_empty(&self) -> bool { self.symbols.is_empty() }

    pub fn next_capacity_byte_demand(&self) -> Result<usize, InlineSymbolRefusal> {
        if self.closing || self.fault.is_some() || self.phase != Phase::Place || self.symbols.has_reserved_slot() { return Ok(0); }
        self.symbols.next_allocation_bytes().map_err(|_| InlineSymbolRefusal::Ownership)
    }

    pub fn next_copy_byte_demand(&self) -> usize {
        if self.closing || self.fault.is_some() || self.is_finished() { return 0; }
        if self.phase == Phase::Place { if self.symbols.has_reserved_slot() { size_of::<SymbolSpan>() } else { 0 } } else { 1 }
    }

    fn refused(&mut self, fault: InlineSymbolRefusal) -> InlineSymbolRefusal { self.fault = Some(fault); fault }

    pub fn advance(&mut self, source: RetainedCloneRef<'_, Vec<u8>>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, InlineSymbolRefusal> {
        let idle = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || self.closing || self.is_finished() { return Ok(idle); }
        if let Some(fault) = self.fault { return Err(fault); }
        if self.phase == Phase::Place {
            if !self.symbols.has_reserved_slot() {
                if grant.maximum_capacity_bytes == 0 { return Ok(idle); }
                source.bind(&mut self.binding).map_err(|_| self.refused(InlineSymbolRefusal::Source))?;
                let progress = self.symbols.reserve_one(grant.maximum_capacity_bytes).map_err(|_| self.refused(InlineSymbolRefusal::Ownership))?;
                return Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, ..idle });
            }
            if grant.maximum_copy_bytes < size_of::<SymbolSpan>() { return Ok(idle); }
            source.bind(&mut self.binding).map_err(|_| self.refused(InlineSymbolRefusal::Source))?;
            let progress = self.symbols.place_reserved(&mut self.pending, grant.maximum_copy_bytes).map_err(|_| self.refused(InlineSymbolRefusal::Ownership))?;
            self.phase = if self.symbols.len() == self.count { Phase::Complete } else { Phase::Length };
            return Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: progress.placed_bytes, ..idle });
        }
        if grant.maximum_copy_bytes == 0 { return Ok(idle); }
        source.bind(&mut self.binding).map_err(|_| self.refused(InlineSymbolRefusal::Source))?;
        let bytes = source.get();
        if self.phase == Phase::Payload {
            let copied = self.remaining.min(grant.maximum_copy_bytes).min(4096);
            if bytes.len().saturating_sub(self.offset) < copied { return Err(self.refused(InlineSymbolRefusal::Incomplete)); }
            for index in 0..copied {
                if self.utf8.admit(bytes[self.offset + index], (self.offset + index) as u64).is_err() { return Err(self.refused(InlineSymbolRefusal::Utf8)); }
            }
            self.offset += copied;
            self.remaining -= copied;
            if self.remaining == 0 {
                if !self.utf8.complete() { return Err(self.refused(InlineSymbolRefusal::Utf8)); }
                self.phase = Phase::Place;
            }
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..idle });
        }
        let Some(byte) = bytes.get(self.offset).copied() else { return Err(self.refused(InlineSymbolRefusal::Incomplete)); };
        let result = self.varint.admit(byte, self.offset as u64).map_err(|_| self.refused(InlineSymbolRefusal::Noncanonical))?;
        self.offset += 1;
        if let RetainedVarintStep::Complete(value) = result {
            self.varint = RetainedVarintCursor::default();
            if self.phase == Phase::Count {
                self.count = usize::try_from(value).ok().filter(|count| *count <= self.maximum_symbols).ok_or_else(|| self.refused(InlineSymbolRefusal::SymbolCount))?;
                self.phase = if self.count == 0 { Phase::Complete } else { Phase::Length };
            } else {
                self.remaining = usize::try_from(value).ok().filter(|bytes| *bytes <= self.maximum_symbol_bytes).ok_or_else(|| self.refused(InlineSymbolRefusal::SymbolLength))?;
                let end = self.offset.checked_add(self.remaining).ok_or_else(|| self.refused(InlineSymbolRefusal::SymbolLength))?;
                self.pending = Some(SymbolSpan { start: self.offset, end });
                self.utf8 = RetainedUtf8Cursor::default();
                self.phase = if self.remaining == 0 { Phase::Place } else { Phase::Payload };
            }
        }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..idle })
    }

    /// 🔤️ Returns the admitted original UTF8 byte span without reconstruction or rescanning.
    pub fn symbol_bytes<'source>(&mut self, source: RetainedCloneRef<'source, Vec<u8>>, index: usize) -> Result<Option<&'source [u8]>, InlineSymbolRefusal> {
        if !self.is_finished() { return Err(InlineSymbolRefusal::Source); }
        source.bind(&mut self.binding).map_err(|_| InlineSymbolRefusal::Source)?;
        let Some(span) = self.symbols.get(index) else { return Ok(None); };
        let bytes = source.get().get(span.start..span.end).ok_or(InlineSymbolRefusal::Source)?;
        Ok(Some(bytes))
    }

    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn next_close_copy_byte_demand(&self) -> Result<usize, InlineSymbolRefusal> { if self.pending.is_some() || !self.symbols.is_empty() { Ok(size_of::<SymbolSpan>()) } else if !self.symbols.terminal_is_empty() { Ok(0) } else { RetainedCloneBinding::copy_demand(&self.binding).map_err(|_| InlineSymbolRefusal::Ownership) } }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, InlineSymbolRefusal> { if self.pending.is_some() || !self.symbols.terminal_is_empty() { Ok(0) } else { RetainedCloneBinding::capacity_demand(&self.binding, body).map_err(|_| InlineSymbolRefusal::Ownership) } }
    pub fn next_close_depth_demand(&self) -> Result<usize, InlineSymbolRefusal> { if self.pending.is_some() || !self.symbols.terminal_is_empty() { Ok(0) } else { RetainedCloneBinding::depth_demand(&self.binding).map_err(|_| InlineSymbolRefusal::Ownership) } }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, InlineSymbolRefusal> {
        if self.next_close_copy_byte_demand()? != 0 { return Ok(0); }
        if self.symbols.terminal_is_empty() { return RetainedCloneBinding::release_demand(&self.binding).map_err(|_| InlineSymbolRefusal::Ownership); }
        self.symbols.next_release_allocation_bytes().map_err(|_| InlineSymbolRefusal::Ownership)
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, InlineSymbolRefusal> {
        let idle = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 { return Ok(idle); }
        let copied = if self.pending.is_some() || !self.symbols.is_empty() { size_of::<SymbolSpan>() } else { 0 };
        if copied != 0 {
            if grant.maximum_copy_bytes < copied { return Ok(idle); }
            if self.pending.take().is_none() { self.symbols.pop(); }
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..idle });
        }
        if !self.symbols.terminal_is_empty() {
            let progress = self.symbols.release_empty_page(grant.maximum_release_bytes).map_err(|_| InlineSymbolRefusal::Ownership)?;
            return Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes, ..idle });
        }
        RetainedCloneBinding::close_one(&mut self.binding, grant).map(|step|step.progress()).map_err(|_| InlineSymbolRefusal::Source)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.pending.is_none() && self.symbols.terminal_is_empty() && self.binding.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
