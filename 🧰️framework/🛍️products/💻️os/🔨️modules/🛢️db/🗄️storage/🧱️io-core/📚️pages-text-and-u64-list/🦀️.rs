//! 📚️ Pages, text and u64 list: the retained pages with their comparisons, the page reader, the bounded text and the bounded u64 list. See [`super`].

use super::*;

//#region 🔖️Limits
/// 📄 Exact ordered fixed-page leases with a zero-copy movable range cursor.
#[derive(Debug)]
pub struct DbIoPages {
    pub operation: u64,
    pub pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    pub retained: u8,
    pub(super) visible: u8,
    pub(super) first_offset: usize,
    pub total_len: usize,
    pub(super) shell_returned: bool,
    pub result_handback: Option<DbIoResultHandback>,
    pub custody: Option<DbIoFaultCustody>,
}

impl DbIoPages {
    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn take_for_async_driver(&mut self) -> Self {
        let operation = self.operation;
        std::mem::replace(self, Self { operation, pages: std::array::from_fn(|_| None), retained: 0, visible: 0, first_offset: 0, total_len: 0, shell_returned: true, result_handback: None })
    }

    pub fn transfer_to_backend(&mut self, backend_operation: u64) -> Result<(), DbError> {
        if self.result_handback.is_some() {
            return Err(DbError::Internal("DB I/O result lease cannot transfer into backend backing".to_string()));
        }
        let page_count = self.pages.iter().flatten().count();
        let mut credit = db_io_page_credit(page_count);
        if !self.shell_returned {
            credit = credit.checked_add(db_io_page_shell_credit()).ok_or(DbError::LimitExceeded("DB I/O backend page-shell transfer"))?;
        }
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().flatten() {
            let owner = arena.slots[page.slot as usize];
            if owner.generation != page.generation || owner.operation != self.operation || owner.phase == DbIoPagePhase::Free {
                return Err(DbError::StaleGeneration { expected: GenerationId(page.generation), actual: GenerationId(owner.generation) });
            }
        }
        db_io_operation_transfer_to_backend(self.operation, backend_operation, credit)?;
        for page in self.pages.iter_mut().flatten() {
            arena.slots[page.slot as usize].operation = backend_operation;
            page.operation = backend_operation;
        }
        self.operation = backend_operation;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn page_count(&self) -> u8 {
        self.visible
    }

    pub fn page(&self, index: u8) -> Option<&[u8]> {
        if index >= self.visible {
            return None;
        }
        let physical = usize::from(index) + self.first_offset / DB_IO_PAGE_BYTES;
        let page = self.pages.get(physical)?.as_ref()?.bytes();
        let start = if index == 0 { self.first_offset % DB_IO_PAGE_BYTES } else { 0 };
        let preceding = usize::from(index).checked_mul(DB_IO_PAGE_BYTES)?.checked_sub(if index == 0 { 0 } else { self.first_offset % DB_IO_PAGE_BYTES })?;
        let remaining = self.total_len.checked_sub(preceding)?;
        let available = page.len().checked_sub(start)?;
        let end = start.checked_add(remaining.min(available))?;
        page.get(start..end)
    }

    pub fn fragments(&self) -> DbIoPageReader<'_> {
        DbIoPageReader { pages: self, cursor: 0 }
    }

    pub fn try_range(mut self, start: usize) -> Result<Self, Self> {
        if start > self.total_len {
            return Err(self);
        }
        self.first_offset += start;
        self.total_len -= start;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn try_prefix(mut self, len: usize) -> Result<Self, Self> {
        if len > self.total_len {
            return Err(self);
        }
        self.total_len = len;
        self.visible = if len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn advance(&mut self, len: usize) -> Result<(), DbError> {
        if len > self.total_len {
            return Err(DbError::InvalidArgument("DB I/O page cursor advanced past its retained range".to_string()));
        }
        self.first_offset += len;
        self.total_len -= len;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(())
    }

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_phase(slot, expected)?;
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }

    pub fn admit(&self) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_identity(slot)?;
            if !matches!(slot.phase, DbIoPagePhase::CheckedOutInput | DbIoPagePhase::TerminalResult) {
                return Err(DbError::Internal(format!("DB I/O input page phase mismatch: expected CheckedOutInput or TerminalResult, got {:?} for operation {} generation {}", slot.phase, page.operation, page.generation)));
            }
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = DbIoPagePhase::Queued;
        }
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else {
            if !self.shell_returned {
                db_io_operation_return(self.operation, db_io_page_shell_credit())?;
                self.shell_returned = true;
                return Ok(Some(0));
            }
            if let Some(handback) = self.result_handback {
                db_io_result_handback(handback)?;
                self.result_handback = None;
                return Ok(Some(0));
            }
            return Ok(None);
        };
        let phase = page.phase()?;
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none) && self.shell_returned && self.result_handback.is_none()
    }
}

impl Drop for DbIoPages {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self {
            operation: self.operation,
            pages: std::mem::replace(&mut self.pages, std::array::from_fn(|_| None)),
            retained: self.retained,
            visible: self.visible,
            first_offset: self.first_offset,
            total_len: self.total_len,
            shell_returned: self.shell_returned,
            result_handback: self.result_handback.take(),
        };
        self.retained = 0;
        self.visible = 0;
        self.first_offset = 0;
        self.total_len = 0;
        self.shell_returned = true;
        if let Err(DbIoLostOwner::Pages(owner)) = db_io_park_lost_owner(DbIoLostOwner::Pages(owner)) {
            *self = owner;
        }
    }
}

impl PartialEq<[u8]> for DbIoPages {
    fn eq(&self, expected: &[u8]) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut offset = 0;
        for fragment in self.fragments() {
            let end = offset + fragment.len();
            if fragment != &expected[offset..end] {
                return false;
            }
            offset = end;
        }
        true
    }
}

impl PartialEq for DbIoPages {
    fn eq(&self, expected: &Self) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut left = self.fragments();
        let mut right = expected.fragments();
        let (mut left_fragment, mut right_fragment) = (left.next().unwrap_or_default(), right.next().unwrap_or_default());
        let (mut left_offset, mut right_offset) = (0, 0);
        while left_offset < left_fragment.len() || right_offset < right_fragment.len() || self.len() == 0 {
            if self.len() == 0 {
                return true;
            }
            let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
            if left_fragment[left_offset..left_offset + compared] != right_fragment[right_offset..right_offset + compared] {
                return false;
            }
            left_offset += compared;
            right_offset += compared;
            if left_offset == left_fragment.len() {
                let Some(next) = left.next() else { return right_offset == right_fragment.len() && right.next().is_none() };
                left_fragment = next;
                left_offset = 0;
            }
            if right_offset == right_fragment.len() {
                let Some(next) = right.next() else { return left_offset == left_fragment.len() && left.next().is_none() };
                right_fragment = next;
                right_offset = 0;
            }
        }
        true
    }
}

impl Eq for DbIoPages {}

impl PartialEq<&[u8]> for DbIoPages {
    fn eq(&self, expected: &&[u8]) -> bool {
        self == *expected
    }
}

impl<const N: usize> PartialEq<[u8; N]> for DbIoPages {
    fn eq(&self, expected: &[u8; N]) -> bool {
        self == expected.as_slice()
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for DbIoPages {
    fn eq(&self, expected: &&[u8; N]) -> bool {
        self == expected.as_slice()
    }
}

impl PartialEq<Vec<u8>> for DbIoPages {
    fn eq(&self, expected: &Vec<u8>) -> bool {
        self == expected.as_slice()
    }
}

pub struct DbIoPageReader<'a> {
    pub pages: &'a DbIoPages,
    pub cursor: u8,
}

impl<'a> Iterator for DbIoPageReader<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self.pages.page(self.cursor)?;
        self.cursor += 1;
        Some(fragment)
    }
}

pub const DB_IO_TEXT_BYTES: usize = 1024;

/// 📋️ The exact capacity of one [`DbIoU64List`]: a backend list longer than this is refused.
pub const DB_IO_LIST_ITEMS: usize = 4096;

/// 🔤 Fixed repository-owned path, document, key or fault text.
#[derive(Clone, PartialEq, Eq)]
pub struct DbIoText {
    pub bytes: [u8; DB_IO_TEXT_BYTES],
    pub len: u16,
}

impl DbIoText {
    pub fn new() -> Self {
        Self { bytes: [0; DB_IO_TEXT_BYTES], len: 0 }
    }

    pub fn try_from_str(value: &str) -> Result<Self, DbError> {
        if value.len() > DB_IO_TEXT_BYTES {
            return Err(DbError::LimitExceeded("db_io text authority"));
        }
        let mut bytes = [0; DB_IO_TEXT_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    pub const fn maximum_capacity() -> usize {
        DB_IO_TEXT_BYTES
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len = 0;
        true
    }
}

impl std::fmt::Write for DbIoText {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(std::fmt::Error)?;
        let target = self.bytes.get_mut(start..end).ok_or(std::fmt::Error)?;
        target.copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }
}

impl std::fmt::Debug for DbIoText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("DbIoText").field(&self.as_str()).finish()
    }
}

/// 🔢 Bounded heap-backed typed list result with incremental close ownership.
pub struct DbIoU64List {
    pub values: Option<Box<[u64]>>,
    pub len: u16,
    pub result_handback: Option<DbIoResultHandback>,
}

impl DbIoU64List {
    pub fn retained_bytes(&self) -> u64 {
        (size_of::<Self>() + self.values.as_ref().map_or(0, |values| size_of_val(values.as_ref()))) as u64
    }

    pub fn new() -> Self {
        Self { values: None, len: 0, result_handback: None }
    }

    pub fn take_for_result(&mut self) -> Self {
        std::mem::replace(self, Self::new())
    }

    pub(super) fn ensure_backing(&mut self) -> Result<(), DbError> {
        if self.values.is_some() {
            return Ok(());
        }
        let mut values = Vec::new();
        values.try_reserve_exact(DB_IO_LIST_ITEMS).map_err(|_| DbError::Unavailable("DB I/O list backing allocation failed".to_string()))?;
        values.resize(DB_IO_LIST_ITEMS, 0);
        self.values = Some(values.into_boxed_slice());
        Ok(())
    }

    pub fn push(&mut self, value: u64) -> Result<(), DbError> {
        let index = usize::from(self.len);
        self.ensure_backing()?;
        let Some(slot) = self.values.as_deref_mut().and_then(|values| values.get_mut(index)) else {
            return Err(DbError::LimitExceeded("db_io list authority"));
        };
        *slot = value;
        self.len += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[u64] {
        self.values.as_deref().map_or(&[], |values| &values[..usize::from(self.len)])
    }

    /// 🔢️ Orders the listed values ascending in place, for a backend whose enumeration has no order.
    pub fn sort_ascending(&mut self) {
        let len = usize::from(self.len);
        if let Some(values) = self.values.as_deref_mut() {
            values[..len].sort_unstable();
        }
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            if self.values.take().is_some() {
                return true;
            }
            if let Some(handback) = self.result_handback {
                if db_io_result_handback(handback).is_ok() {
                    self.result_handback = None;
                    return true;
                }
            }
            return false;
        }
        self.len -= 1;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.values.is_none() && self.result_handback.is_none()
    }
}

impl Default for DbIoU64List {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for DbIoU64List {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::ops::DerefMut for DbIoU64List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = usize::from(self.len);
        self.values.as_deref_mut().map_or(&mut [], |values| &mut values[..len])
    }
}

impl std::fmt::Debug for DbIoU64List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.as_slice()).finish()
    }
}

impl PartialEq<Vec<u64>> for DbIoU64List {
    fn eq(&self, expected: &Vec<u64>) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl PartialEq for DbIoU64List {
    fn eq(&self, expected: &Self) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl<'a> IntoIterator for &'a DbIoU64List {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<const N: usize> PartialEq<[u64; N]> for DbIoU64List {
    fn eq(&self, expected: &[u64; N]) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl IntoIterator for DbIoU64List {
    type Item = u64;
    type IntoIter = std::iter::Take<std::vec::IntoIter<u64>>;

    fn into_iter(mut self) -> Self::IntoIter {
        let len = usize::from(self.len);
        self.values.take().map_or_else(Vec::new, Vec::from).into_iter().take(len)
    }
}

impl Drop for DbIoU64List {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { values: self.values.take(), len: self.len, result_handback: self.result_handback.take() };
        self.len = 0;
        if let Err(DbIoLostOwner::List(owner)) = db_io_park_lost_owner(DbIoLostOwner::List(owner)) {
            *self = owner;
        }
    }
}
//#endregion 🔖️Limits
