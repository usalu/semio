//! 📖️ State query source and pages: the state query source and the state page codecs. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️QuerySource
/// 🚰️ The `db_query::QuerySource` this crate supplies over its own `DocumentState`: one row
/// per stored path, `{"path": <path>, "value": <text-or-bytes>}`.
pub(super) struct StateQuerySource<'a>(pub(super) &'a db_state::RetainedStateMap);

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map (QuerySource::scan's row builder) — see R9
async fn path_row_value(path: &str, bytes: &db_io_core::DbIoPages, control: &mut db_query::QueryCursorControl) -> Result<db_query::Value, DbError> {
    let mut map = std::collections::BTreeMap::new();
    map.insert("path".to_string(), db_query::Value::Text(path.to_string()));
    map.insert("value".to_string(), db_query::Value::Bytes(db_query::QueryBytes::copy_from_pages(bytes, control).await?));
    Ok(db_query::Value::Map(map))
}

impl<'a> db_query::QuerySource for StateQuerySource<'a> {
    async fn scan(&self, control: &mut db_query::QueryCursorControl) -> Result<db_query::QueryRows, DbError> {
        let mut rows = db_query::QueryRows::new();
        for (index, entry) in self.0.iter().enumerate() {
            control.grant()?;
            rows.preflight_push()?;
            let row = db_query::QueryRow::new(db_query::RowId(index as u64), path_row_value(entry.key(), entry.value(), control).await?);
            rows.push_preflighted(row);
        }
        Ok(rows)
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️Snapshot
const MAX_STATE_PAGE_ENTRIES: u64 = 10_000_000;

const MAX_STATE_PAGE_PATH_BYTES: u64 = db_io_core::DbIoText::maximum_capacity() as u64;

pub(super) const MAX_STATE_PAGE_VALUE_BYTES: u64 = 256 * 1024 * 1024;

fn state_page_varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

fn state_page_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

struct StatePageGrant {
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    fuel: usize,
}

impl StatePageGrant {
    fn consume(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("state page cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("state page cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("state page cursor fuel"))?;
        Ok(())
    }
}

pub(super) struct StatePageEncodeCursor<'state> {
    pub(super) values: &'state db_state::RetainedStateMap,
    pub writer: db_io_core::DbIoPageWriter,
    pub grant: StatePageGrant,
}

impl<'state> StatePageEncodeCursor<'state> {
    pub fn try_new(values: &'state db_state::RetainedStateMap, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        check_len(values.len() as u64, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
        let mut len = state_page_varint_len(values.len() as u64);
        for entry in values.iter() {
            let path = entry.key();
            let value = entry.value();
            check_len(path.len() as u64, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
            check_len(value.len() as u64, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
            len = len.checked_add(state_page_varint_len(path.len() as u64) + path.len() + 1 + state_page_varint_len(value.len() as u64) + value.len()).ok_or(DbError::LimitExceeded("state page encoded bytes"))?;
        }
        let pages = len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        if pages > db_io_core::DB_IO_OPERATION_PAGES {
            return Err(DbError::LimitExceeded("state page reservation"));
        }
        let writer = db_io_core::DbIoPageWriter::try_reserve(pages).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        Ok(Self { values, writer, grant: StatePageGrant { cancelled, deadline, fuel } })
    }

    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), DbError> {
        let mut offset = 0;
        while offset < bytes.len() {
            self.grant.consume()?;
            offset += self.writer.write_fragment(&bytes[offset..])?;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    pub async fn finish(mut self) -> Result<db_io_core::DbIoPages, DbError> {
        let mut varint = [0u8; 10];
        self.write(state_page_varint(self.values.len() as u64, &mut varint)).await?;
        for entry in self.values.iter() {
            let path = entry.key();
            let value = entry.value();
            self.write(state_page_varint(path.len() as u64, &mut varint)).await?;
            self.write(path.as_bytes()).await?;
            self.write(&[1]).await?;
            self.write(state_page_varint(value.len() as u64, &mut varint)).await?;
            for fragment in value.fragments() {
                self.write(fragment).await?;
            }
        }
        self.writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }
}

pub(super) struct StatePageDecodeCursor<'pages> {
    pub pages: &'pages db_io_core::DbIoPages,
    pub page: u8,
    pub offset: usize,
    pub remaining: Option<u64>,
    pub grant: StatePageGrant,
    pub closed: bool,
}

impl<'pages> StatePageDecodeCursor<'pages> {
    pub fn new(pages: &'pages db_io_core::DbIoPages, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if pages.operation() == 0 || fuel == 0 {
            return Err(DbError::InvalidArgument("state page decode authority".to_string()));
        }
        Ok(Self { pages, page: 0, offset: 0, remaining: None, grant: StatePageGrant { cancelled, deadline, fuel }, closed: false })
    }

    pub fn byte(&mut self) -> Result<u8, DbError> {
        self.grant.consume()?;
        loop {
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page ended early".to_string()))?;
            if let Some(byte) = fragment.get(self.offset) {
                self.offset += 1;
                return Ok(*byte);
            }
            self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
            self.offset = 0;
        }
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("state page varint overflow".to_string()))
    }

    fn fixed_field<const N: usize>(&mut self, len: usize) -> Result<[u8; N], DbError> {
        if len > N {
            return Err(DbError::LimitExceeded("state page fixed field"));
        }
        let mut output = [0u8; N];
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page field ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            output[written..written + copied].copy_from_slice(&fragment[self.offset..self.offset + copied]);
            written += copied;
            self.offset += copied;
        }
        Ok(output)
    }

    async fn retained_field(&mut self, len: usize) -> Result<db_io_core::DbIoPages, DbError> {
        let pages = len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), pages).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page value ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            let accepted = writer.write_fragment(&fragment[self.offset..self.offset + copied])?;
            if accepted != copied {
                return Err(DbError::Internal("state page value writer made partial progress".to_string()));
            }
            self.offset += copied;
            written += copied;
            semio_framework_async::yield_once().await;
        }
        writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub async fn next(&mut self) -> Result<Option<db_state::StateEntry>, DbError> {
        let remaining = match self.remaining {
            Some(remaining) => remaining,
            None => {
                let count = self.varint()?;
                check_len(count, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
                self.remaining = Some(count);
                count
            }
        };
        if remaining == 0 {
            return Ok(None);
        }
        let path_len = self.varint()?;
        check_len(path_len, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
        let path = self.fixed_field::<1024>(path_len as usize)?;
        let path = std::str::from_utf8(&path[..path_len as usize]).map_err(|_| DbError::Corrupt("snapshot page path is not valid utf-8".to_string()))?;
        let value = match self.byte()? {
            1 => {
                let len = self.varint()?;
                check_len(len, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
                self.retained_field(len as usize).await?
            }
            _ => return Err(DbError::Corrupt("state page value tag".to_string())),
        };
        let entry = db_state::StateEntry::try_new(path, value).map_err(|(error, mut value)| {
            let _ = value.close_step();
            drop(value);
            error
        })?;
        self.remaining = Some(remaining - 1);
        semio_framework_async::yield_once().await;
        Ok(Some(entry))
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        self.grant.consume()?;
        if self.closed {
            return Ok(false);
        }
        self.remaining = Some(0);
        self.closed = true;
        Ok(true)
    }
}

/// 📋️ What `ArtifactEngine::open` did to materialize state — "initial ⊕ snapshot ⊕ WAL
/// suffix" made observable. New this revision (was `db_wal::WalRecoveryReport` alone before).
#[derive(Clone, Debug, Default)]
pub struct MaterializeReport {
    pub from_snapshot: bool,
    pub snapshot_generation: Option<u64>,
    pub torn_tail_bytes: u64,
    pub commands_replayed: u64,
}
//#endregion 🔖️Snapshot
//#endregion 🔖️Ids
