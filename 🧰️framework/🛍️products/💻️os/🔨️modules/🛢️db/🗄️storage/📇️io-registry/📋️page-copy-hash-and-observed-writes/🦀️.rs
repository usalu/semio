//! 📋️ Page copy, hash and observed writes: the page copy, owner copy, page hash, observed byte writes and list transfer futures. See [`super`].

use super::*;

//#region 🔖️Limits
/// 📥 One-fragment-per-poll copy into already checked-out fixed pages.
pub struct DbIoPageCopy<'a> {
    pub source: &'a [u8],
    pub cursor: usize,
    pub writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageOwnerCopy<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageHash<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub hasher: semio_framework_hash::Hasher,
}

pub fn db_io_copy_pages(source: &[u8]) -> Result<DbIoPageCopy<'_>, DbError> {
    let pages = source.len().div_ceil(DB_IO_PAGE_BYTES);
    let writer = DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageCopy { source, cursor: 0, writer: Some(writer) })
}

/// 🪜 Retained external-driver byte conversion with one observed owner/page/close opportunity per poll.
pub struct DbIoObservedBytesWrite<'a> {
    pub reservation: Option<DbIoDriverReservation>,
    pub source: Option<DbIoExternalBytes>,
    pub output: &'a mut DbIoPageWriter,
    pub cursor: usize,
    limit: usize,
    pub phase: u8,
}

pub fn db_io_write_observed_bytes(reservation: DbIoDriverReservation, source: Vec<u8>, output: &mut DbIoPageWriter) -> DbIoObservedBytesWrite<'_> {
    let limit = source.len();
    DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: 0, limit, phase: 0 }
}

/// 🧷 Retained ranged external-driver transfer with the same exact close authority.
pub fn db_io_write_observed_bytes_range(reservation: DbIoDriverReservation, source: Vec<u8>, offset: usize, length: usize, output: &mut DbIoPageWriter) -> Result<DbIoObservedBytesWrite<'_>, DbError> {
    let limit = offset.checked_add(length).ok_or(DbError::LimitExceeded("DB I/O observed-byte range"))?;
    if limit > source.len() {
        return Err(DbError::InvalidArgument("DB I/O observed-byte range exceeds result".to_string()));
    }
    Ok(DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: offset, limit, phase: 0 })
}

impl Future for DbIoObservedBytesWrite<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        match owner.phase {
            0 => {
                let capacity = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its exact source".to_string()))?.capacity()?;
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation".to_string()))?.observe_capacity(capacity)?;
                owner.phase = 1;
            }
            1 => {
                let source = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during copy".to_string()))?.as_slice()?;
                if owner.cursor < owner.limit {
                    let end = owner.cursor.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O observed-byte cursor"))?.min(owner.limit);
                    let written = owner.output.write_fragment(&source[owner.cursor..end])?;
                    owner.cursor += written;
                } else {
                    owner.phase = 2;
                }
            }
            2 => {
                let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during close".to_string()))?;
                if !source.close_step() {
                    owner.source.take();
                    owner.phase = 3;
                }
            }
            3 => {
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation during close".to_string()))?.close_step()?;
                owner.reservation.take();
                owner.phase = 4;
            }
            _ => match owner.output.seal_retained_step()? {
                Some(published) => return std::task::Poll::Ready(Ok(published)),
                None => {}
            },
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn db_io_copy_page_owner(source: &DbIoPages) -> Result<DbIoPageOwnerCopy<'_>, DbError> {
    let writer = DbIoPageWriter::try_reserve(source.page_count() as usize).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageOwnerCopy { source, cursor: 0, writer: Some(writer) })
}

pub fn db_io_hash_pages(source: &DbIoPages) -> DbIoPageHash<'_> {
    DbIoPageHash { source, cursor: 0, hasher: semio_framework_hash::Hasher::new() }
}

/// 🔢 Persisted one-scalar transfer and one-owner close authority for driver lists.
pub struct DbIoListTransfer<'a> {
    pub source: Option<DbIoU64List>,
    pub output: &'a mut DbIoU64List,
    pub cursor: usize,
    pub phase: u8,
}

pub fn db_io_transfer_list(source: DbIoU64List, output: &mut DbIoU64List) -> DbIoListTransfer<'_> {
    DbIoListTransfer { source: Some(source), output, cursor: 0, phase: 0 }
}

impl Future for DbIoListTransfer<'_> {
    type Output = Result<DbIoU64List, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.phase == 2 {
            return std::task::Poll::Ready(Ok(std::mem::take(owner.output)));
        }
        let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O list transfer lost its exact source".to_string()))?;
        match owner.phase {
            0 if owner.cursor < source.len() => {
                owner.output.push(source.as_slice()[owner.cursor])?;
                owner.cursor += 1;
            }
            0 => owner.phase = 1,
            1 if source.close_step() => {}
            1 => {
                owner.source.take();
                owner.phase = 2;
            }
            _ => return std::task::Poll::Ready(Err(DbError::Internal("DB I/O list transfer lost its exact source".to_string()))),
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.cursor == owner.source.len() {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        }
        let start = owner.cursor;
        let end = start.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O page-copy cursor"))?.min(owner.source.len());
        let source = &owner.source[start..end];
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        owner.cursor += written;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageOwnerCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(source) = owner.source.page(owner.cursor) else {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        };
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        if written != source.len() {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O source fragment exceeded one fixed page".to_string())));
        }
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageHash<'_> {
    type Output = ContentHash;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            let hasher = std::mem::replace(&mut owner.hasher, semio_framework_hash::Hasher::new());
            return std::task::Poll::Ready(ContentHash(*hasher.finalize().as_bytes()));
        };
        owner.hasher.update(fragment);
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}
//#endregion 🔖️Limits
