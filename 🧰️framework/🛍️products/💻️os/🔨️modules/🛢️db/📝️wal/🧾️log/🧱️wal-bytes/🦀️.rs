//! 🧱️ WAL bytes: the owned bytes, their cursor and rejection and the payload reference. See [`super`].

use super::*;

//#region 🔖️Records
#[derive(Debug)]
pub struct WalBytes {
    pub pages: db_io_core::DbIoPages,
}

pub struct WalBytesCursor<'bytes> {
    pub bytes: &'bytes WalBytes,
    pub offset: usize,
}

#[derive(Debug)]
pub struct WalBytesRejected {
    pub source: Option<Vec<u8>>,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub error: DbError,
}

impl WalBytes {
    pub async fn try_admit(source: Vec<u8>, maximum: u64, control: &mut WalCursorControl) -> Result<Self, WalBytesRejected> {
        if source.capacity() as u64 > maximum {
            return Err(WalBytesRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("wal source backing capacity") });
        }
        let mut writer = match db_io_core::DbIoPageWriter::try_reserve(source.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES)) {
            Ok(writer) => writer,
            Err(rejected) => return Err(WalBytesRejected { source: Some(source), writer: rejected.into_writer(), error: DbError::Unavailable("wal page admission rejected".to_string()) }),
        };
        let mut reservation = match db_io_core::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(WalBytesRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_io_core::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        let mut turn = WalTurn::start();
        while offset < source.as_slice().map_err(|error| WalBytesRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            turn.step().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            turn.step().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(WalBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            WalBytesRejected { source: None, writer, error }
        })
    }

    /// 🧺️ Stages `source` under an operation that already holds DB I/O credit, so every record
    /// of one transaction shares one ledger slot instead of each claiming its own.
    pub async fn copy_for_operation(operation: u64, source: &[u8], control: &mut WalCursorControl) -> Result<Self, DbError> {
        let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(operation, source.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut offset = 0;
        let mut turn = WalTurn::start();
        while offset < source.len() {
            control.grant()?;
            offset += writer.write_fragment(&source[offset..])?;
            turn.step().await;
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub fn operation(&self) -> u64 {
        self.pages.operation()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn fragments(&self) -> db_io_core::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    pub fn cursor(&self) -> WalBytesCursor<'_> {
        WalBytesCursor { bytes: self, offset: 0 }
    }

    #[cfg(test)]
    pub(super) async fn prepare_platform(&self) -> Result<db_io_registry::DbIoPlatformBuffer, DbError> {
        db_io_registry::db_io_prepare_platform(&self.pages)?.await
    }

    /// 🔏️ The record's content hash in one bounded step (a record spans at most one operation's pages).
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = semio_framework_hash::Hasher::new();
        for fragment in self.fragments() {
            hasher.update(fragment);
        }
        *hasher.finalize().as_bytes()
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

impl<'bytes> WalBytesCursor<'bytes> {
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    pub fn byte(&mut self, control: &mut WalCursorControl) -> Result<u8, DbError> {
        control.grant()?;
        let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
        let byte = self.bytes.pages.page(page).and_then(|fragment| fragment.get(page_offset)).copied().ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
        self.offset += 1;
        Ok(byte)
    }

    pub fn varint(&mut self, control: &mut WalCursorControl) -> Result<u64, DbError> {
        let start = self.offset;
        let result = wal_read_canonical_varint(|| self.byte(control));
        if result.is_err() {
            self.offset = start;
        }
        result
    }

    pub fn begin_field(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<usize, DbError> {
        let len = self.varint(control)?;
        check_len(len, maximum, "wal retained field")?;
        let len = len as usize;
        if len > self.remaining() {
            return Err(DbError::Corrupt("wal retained field length exceeds record".to_string()));
        }
        Ok(len)
    }

    pub fn read_field_fragment(&mut self, remaining: &mut usize, output: &mut [u8], control: &mut WalCursorControl) -> Result<usize, DbError> {
        if *remaining == 0 || output.is_empty() {
            return Ok(0);
        }
        control.grant()?;
        let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
        let fragment = self.bytes.pages.page(page).ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
        let copied = (*remaining).min(output.len()).min(fragment.len().saturating_sub(page_offset));
        if copied == 0 {
            return Err(DbError::Corrupt("wal retained field cursor stalled".to_string()));
        }
        output[..copied].copy_from_slice(&fragment[page_offset..page_offset + copied]);
        self.offset += copied;
        *remaining -= copied;
        Ok(copied)
    }

    /// 📦️ One length-prefixed field copied out page by page; its length is checked against
    /// `maximum` and against what the record still holds before anything is allocated.
    pub fn field(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<Vec<u8>, DbError> {
        let mut remaining = self.begin_field(maximum, control)?;
        let mut output = Vec::with_capacity(remaining);
        while remaining != 0 {
            control.grant()?;
            let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
            let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
            let fragment = self.bytes.pages.page(page).ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
            let copied = remaining.min(fragment.len().saturating_sub(page_offset));
            if copied == 0 {
                return Err(DbError::Corrupt("wal retained field cursor stalled".to_string()));
            }
            output.extend_from_slice(&fragment[page_offset..page_offset + copied]);
            self.offset += copied;
            remaining -= copied;
        }
        Ok(output)
    }

    pub fn text(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<String, DbError> {
        String::from_utf8(self.field(maximum, control)?).map_err(|_| DbError::Corrupt("wal retained text is not valid utf-8".to_string()))
    }
}

impl WalBytesRejected {
    pub fn source(&self) -> Option<&Vec<u8>> {
        self.source.as_ref()
    }

    pub fn into_source(mut self) -> Option<Vec<u8>> {
        self.source.take()
    }

    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn into_error(mut self) -> DbError {
        std::mem::replace(&mut self.error, DbError::Closed)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if let Some(writer) = self.writer.as_mut() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.writer = None;
        Ok(false)
    }
}

impl Drop for WalBytesRejected {
    fn drop(&mut self) {
        if let Some(source) = self.source.take() {
            drop(db_io_core::DbIoExternalBytes::new(source));
        }
    }
}

/// 🫙️ `WAL_PAYLOAD`'s two shapes: small payloads inline, large ones by CAS reference into
/// `db_storage::PayloadStorage` — mirrors that trait's own blake3-CAS design.
pub enum WalPayloadRef {
    Inline(WalBytes),
    CasRef(ContentHash),
}
//#endregion 🔖️Records
