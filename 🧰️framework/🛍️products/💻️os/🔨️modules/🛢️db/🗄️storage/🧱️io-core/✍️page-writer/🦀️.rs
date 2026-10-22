//! ✍️ Page writer: the page writer with its seal, the rejected writer and the writer drop and debug. See [`super`].

use super::*;

//#region 🔖️Limits
/// ✍️ Retained writer backed only by exact fixed arena page leases.
pub struct DbIoPageWriter {
    pub operation: u64,
    pub pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    pub reserved: u8,
    pub cursor: u8,
    pub total_len: usize,
    pub(super) shell_returned: bool,
    seal_phase: u8,
    seal_page: u8,
    seal_visible: u8,
    seal_current: Option<DbIoPagePhase>,
}

#[derive(Debug)]
pub struct DbIoPageWriterRejected {
    pub error: DbError,
    pub writer: Option<DbIoPageWriter>,
}

/// 🧵 One retained unused-page retirement opportunity per poll before writer publication.
pub struct DbIoPageWriterSeal {
    pub writer: Option<DbIoPageWriter>,
}

impl DbIoPageWriter {
    pub fn try_reserve(reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let credit = db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()).ok_or_else(|| DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None })?;
        let operation = db_io_operation_reserve(credit).map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        Self::checkout(operation, reserved_pages)
    }

    pub fn try_reserve_for_operation(operation: u64, reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let credit = db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()).ok_or_else(|| DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None })?;
        db_io_operation_add(operation, credit).map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        Self::checkout(operation, reserved_pages)
    }

    fn checkout(operation: u64, reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let pages = match db_io_checkout_pages(operation, reserved_pages, DbIoPagePhase::CheckedOutWriter) {
            Ok(pages) => pages,
            Err(error) => {
                let credit = match db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()) {
                    Some(credit) => credit,
                    None => return Err(DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None }),
                };
                let _ = db_io_operation_return(operation, credit);
                return Err(DbIoPageWriterRejected { error, writer: None });
            }
        };
        Ok(Self { operation, pages, reserved: reserved_pages as u8, cursor: 0, total_len: 0, shell_returned: false, seal_phase: 0, seal_page: 0, seal_visible: 0, seal_current: None })
    }

    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn write_fragment(&mut self, source: &[u8]) -> Result<usize, DbError> {
        let Some(page) = self.pages.get_mut(self.cursor as usize).and_then(Option::as_mut) else {
            return Err(DbError::LimitExceeded("db_io writer page reservation"));
        };
        let available = DB_IO_PAGE_BYTES - usize::from(page.used);
        let written = available.min(source.len());
        page.write(usize::from(page.used), &source[..written])?;
        self.total_len = self.total_len.checked_add(written).ok_or(DbError::LimitExceeded("db_io writer total"))?;
        if usize::from(page.used) == DB_IO_PAGE_BYTES {
            self.cursor += 1;
        }
        Ok(written)
    }

    pub fn patch_fragment(&mut self, offset: usize, source: &[u8]) -> Result<usize, DbError> {
        if offset >= self.total_len || source.is_empty() {
            return Ok(0);
        }
        let page_index = offset / DB_IO_PAGE_BYTES;
        let page_offset = offset % DB_IO_PAGE_BYTES;
        let page = self.pages.get_mut(page_index).and_then(Option::as_mut).ok_or(DbError::LimitExceeded("db_io writer patch page"))?;
        let patchable = usize::from(page.used).checked_sub(page_offset).ok_or_else(|| DbError::Internal("db_io writer patch offset exceeded its admitted page".to_string()))?;
        let written = source.len().min(patchable);
        page.write(page_offset, &source[..written])?;
        Ok(written)
    }

    pub fn read_fragment(&self, offset: usize, output: &mut [u8]) -> Result<usize, DbError> {
        if offset >= self.total_len || output.is_empty() {
            return Ok(0);
        }
        let page_index = offset / DB_IO_PAGE_BYTES;
        let page_offset = offset % DB_IO_PAGE_BYTES;
        let page = self.pages.get(page_index).and_then(Option::as_ref).ok_or(DbError::LimitExceeded("db_io writer read page"))?;
        let source = page.bytes();
        let readable = source.len().checked_sub(page_offset).ok_or_else(|| DbError::Internal("db_io writer read offset exceeded its admitted page".to_string()))?;
        let read = output.len().min(readable);
        output[..read].copy_from_slice(&source[page_offset..page_offset + read]);
        Ok(read)
    }

    pub fn seal_retained(self) -> DbIoPageWriterSeal {
        DbIoPageWriterSeal { writer: Some(self) }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn seal(mut self) -> Result<DbIoPages, DbIoPageWriterRejected> {
        match self.finish() {
            Ok(pages) => Ok(pages),
            Err(error) => Err(DbIoPageWriterRejected { error, writer: Some(self) }),
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn finish(&mut self) -> Result<DbIoPages, DbError> {
        loop {
            if let Some(owner) = self.seal_retained_step()? {
                return Ok(owner);
            }
        }
    }

    /// 🪡 One retained validation, unused-page retirement, or atomic transition-and-publication opportunity.
    pub fn seal_retained_step(&mut self) -> Result<Option<DbIoPages>, DbError> {
        match self.seal_phase {
            0 if self.reserved > self.seal_visible => {
                self.seal_visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) as u8 };
                if self.reserved <= self.seal_visible {
                    return Ok(None);
                }
                let index = usize::from(self.reserved - 1);
                let page = self.pages[index].as_ref().ok_or_else(|| DbError::Internal("DB I/O retained writer lost an unused page".to_string()))?;
                let phase = page.phase()?;
                if !matches!(phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
                    return Err(DbError::Internal(format!("DB I/O unused writer page phase mismatch: expected CheckedOutWriter or Executing, got {phase:?} for operation {} generation {}", page.operation, page.generation)));
                }
                page.transition(phase, DbIoPagePhase::Closing)?;
                let page = self.pages[index].take().ok_or_else(|| DbError::Internal("DB I/O retained writer lost an unused page".to_string()))?;
                page.return_to_arena()?;
                self.reserved -= 1;
                Ok(None)
            }
            0 => {
                self.seal_visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) as u8 };
                let current = match self.pages.iter().take(self.reserved as usize).flatten().next() {
                    Some(page) => page.phase()?,
                    None => DbIoPagePhase::CheckedOutWriter,
                };
                if !matches!(current, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
                    return Err(DbError::Internal("DB I/O retained writer finished outside an owned phase".to_string()));
                }
                self.seal_current = Some(current);
                self.seal_phase = 1;
                Ok(None)
            }
            1 if self.seal_page < self.reserved => {
                let page = self.pages[self.seal_page as usize].as_ref().ok_or_else(|| DbError::Internal("DB I/O retained writer validation lost a page".to_string()))?;
                let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let slot = state.slots[page.slot as usize];
                let current = self.seal_current.ok_or_else(|| DbError::Internal("DB I/O retained writer lost its source phase".to_string()))?;
                page.validate_phase(slot, current)?;
                self.seal_page += 1;
                Ok(None)
            }
            1 => {
                self.seal_phase = 2;
                self.seal_page = 0;
                Ok(None)
            }
            2 => {
                let current = self.seal_current.ok_or_else(|| DbError::Internal("DB I/O retained writer lost its transition phase".to_string()))?;
                let next = if current == DbIoPagePhase::Executing { DbIoPagePhase::TerminalResult } else { DbIoPagePhase::CheckedOutInput };
                self.transition(current, next)?;
                self.seal_phase = 3;
                let pages = std::mem::replace(&mut self.pages, std::array::from_fn(|_| None));
                let owner = DbIoPages { operation: self.operation, pages, retained: self.reserved, visible: self.seal_visible, first_offset: 0, total_len: self.total_len, shell_returned: false, result_handback: None, custody: None };
                self.shell_returned = true;
                self.reserved = 0;
                self.cursor = 0;
                self.total_len = 0;
                self.seal_phase = 4;
                Ok(Some(owner))
            }
            _ => Err(DbError::Closed),
        }
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else {
            if !self.shell_returned {
                db_io_operation_return(self.operation, db_io_page_shell_credit())?;
                self.shell_returned = true;
                return Ok(Some(0));
            }
            return Ok(None);
        };
        let phase = page.phase()?;
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none) && self.shell_returned
    }

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_phase(slot, expected)?;
        }
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }
}

impl Future for DbIoPageWriterSeal {
    type Output = Result<DbIoPages, DbIoPageWriterRejected>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbIoPageWriterRejected { error: DbError::Internal("DB I/O retained writer lost its exact owner".to_string()), writer: None }));
        };
        match writer.seal_retained_step() {
            Ok(Some(published)) => {
                owner.writer.take();
                std::task::Poll::Ready(Ok(published))
            }
            Ok(None) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(error) => std::task::Poll::Ready(Err(DbIoPageWriterRejected { error, writer: owner.writer.take() })),
        }
    }
}

impl Drop for DbIoPageWriter {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self {
            operation: self.operation,
            pages: std::mem::replace(&mut self.pages, std::array::from_fn(|_| None)),
            reserved: self.reserved,
            cursor: self.cursor,
            total_len: self.total_len,
            shell_returned: self.shell_returned,
            seal_phase: self.seal_phase,
            seal_page: self.seal_page,
            seal_visible: self.seal_visible,
            seal_current: self.seal_current,
        };
        self.reserved = 0;
        self.cursor = 0;
        self.total_len = 0;
        self.shell_returned = true;
        if let Err(DbIoLostOwner::PageWriter(owner)) = db_io_park_lost_owner(DbIoLostOwner::PageWriter(owner)) {
            *self = owner;
        }
    }
}

impl std::fmt::Debug for DbIoPageWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoPageWriter").field("operation", &self.operation).field("reserved", &self.reserved).field("cursor", &self.cursor).field("total_len", &self.total_len).finish()
    }
}

impl DbIoPageWriterRejected {
    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn into_writer(self) -> Option<DbIoPageWriter> {
        self.writer
    }

    pub fn into_error(self) -> DbError {
        self.error
    }

    pub fn into_parts(self) -> (DbError, Option<DbIoPageWriter>) {
        (self.error, self.writer)
    }
}
//#endregion 🔖️Limits
