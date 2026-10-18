//! 🧭️ Index cursor control and bytes: the cursor control, the retained index bytes with their rejection, the admission and the byte comparisons. See [`super`].

use super::*;

//#region 🔖️SortedRun
/// 📇️ One entry's value in a sorted run: either a live payload or a tombstone recording that
/// a key was deleted (and must keep shadowing that key in any older, not-yet-merged run beneath).
pub struct IndexCursorControl {
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    pub fuel: usize,
    cooperative: Option<(std::time::Duration, usize)>,
}

impl IndexCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("index cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel, cooperative: None })
    }

    pub fn retained(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        let duration = deadline.saturating_duration_since(std::time::Instant::now());
        if fuel == 0 || duration.is_zero() {
            return Err(DbError::LimitExceeded("index cursor retained budget"));
        }
        Ok(Self { cancelled, deadline, fuel, cooperative: Some((duration, fuel)) })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("index cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("index cursor cancelled".to_string()));
        }
        let now = std::time::Instant::now();
        if (now >= self.deadline || self.fuel == 0) && self.cooperative.is_some() {
            let (duration, fuel) = self.cooperative.expect("checked retained index budget");
            std::thread::yield_now();
            self.deadline = std::time::Instant::now() + duration;
            self.fuel = fuel;
        } else if now >= self.deadline {
            return Err(DbError::Unavailable("index cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("index cursor fuel"))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct IndexBytes {
    pub pages: db_io_core::DbIoPages,
}

impl PartialEq for IndexBytes {
    fn eq(&self, expected: &Self) -> bool {
        self.pages == expected.pages
    }
}

impl Eq for IndexBytes {}

#[derive(Debug)]
pub struct IndexBytesRejected {
    pub(super) source: Option<Vec<u8>>,
    pub(super) writer: Option<db_io_core::DbIoPageWriter>,
    pub(super) error: DbError,
}

impl IndexBytes {
    pub async fn try_admit(source: Vec<u8>, maximum: u64, control: &mut IndexCursorControl) -> Result<Self, IndexBytesRejected> {
        if source.capacity() as u64 > maximum {
            return Err(IndexBytesRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("index source backing capacity") });
        }
        let pages = source.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        let mut writer = match db_io_core::DbIoPageWriter::try_reserve(pages) {
            Ok(writer) => writer,
            Err(error) => return Err(IndexBytesRejected { source: Some(source), writer: error.into_writer(), error: DbError::Unavailable("index page admission rejected".to_string()) }),
        };
        let mut reservation = match db_io_core::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(IndexBytesRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_io_core::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        while offset < source.as_slice().map_err(|error| IndexBytesRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            semio_framework_async::yield_once().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(IndexBytesRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            semio_framework_async::yield_once().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(IndexBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            IndexBytesRejected { source: None, writer, error }
        })
    }

    pub fn operation(&self) -> u64 {
        self.pages.operation()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn fragments(&self) -> db_io_core::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    #[cfg(test)]
    pub(super) async fn prepare_platform(&self) -> Result<db_io_registry::DbIoPlatformBuffer, DbError> {
        db_io_registry::db_io_prepare_platform(&self.pages)?.await
    }

    pub async fn copy_for_operation(operation: u64, bytes: &[u8], control: &mut IndexCursorControl) -> Result<Self, DbError> {
        index_bytes_from_slice_for_operation(operation, bytes, control).await
    }

    pub fn read_fragment(&self, offset: usize, output: &mut [u8]) -> usize {
        if offset >= self.len() || output.is_empty() {
            return 0;
        }
        let page = (offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
        let page_offset = offset % db_io_core::DB_IO_PAGE_BYTES;
        let Some(fragment) = self.pages.page(page) else { return 0 };
        let read = output.len().min(fragment.len().saturating_sub(page_offset));
        output[..read].copy_from_slice(&fragment[page_offset..page_offset + read]);
        read
    }

    pub fn starts_with(&self, prefix: &IndexBytes) -> bool {
        index_bytes_compare_prefix(self, prefix)
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

impl IndexBytesRejected {
    pub fn source(&self) -> Option<&Vec<u8>> {
        self.source.as_ref()
    }

    pub fn into_source(mut self) -> Option<Vec<u8>> {
        self.source.take()
    }

    pub fn error(&self) -> &DbError {
        &self.error
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

impl Drop for IndexBytesRejected {
    fn drop(&mut self) {
        if let Some(source) = self.source.take() {
            drop(db_io_core::DbIoExternalBytes::new(source));
        }
    }
}

pub(super) async fn admit_generated_index_bytes(source: Vec<u8>, maximum: u64, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    match IndexBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            let error = std::mem::replace(&mut rejected.error, DbError::Closed);
            loop {
                control.grant()?;
                if !rejected.close_step()? {
                    break;
                }
                semio_framework_async::yield_once().await;
            }
            if let Some(source) = rejected.source.take() {
                let mut source = db_io_core::DbIoExternalBytes::new(source);
                while !source.terminal_is_empty() {
                    control.grant()?;
                    let _ = source.close_step();
                    semio_framework_async::yield_once().await;
                }
            }
            Err(error)
        }
    }
}

pub(super) async fn close_index_bytes(mut bytes: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
    control.grant()?;
    let _ = bytes.close_step()?;
    drop(bytes);
    Ok(())
}

pub(super) fn index_bytes_cmp(left: &IndexBytes, right: &IndexBytes) -> std::cmp::Ordering {
    let mut left_fragments = left.fragments();
    let mut right_fragments = right.fragments();
    let mut left_fragment = left_fragments.next().unwrap_or_default();
    let mut right_fragment = right_fragments.next().unwrap_or_default();
    let (mut left_offset, mut right_offset) = (0, 0);
    loop {
        let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
        let order = left_fragment[left_offset..left_offset + compared].cmp(&right_fragment[right_offset..right_offset + compared]);
        if order != std::cmp::Ordering::Equal {
            return order;
        }
        left_offset += compared;
        right_offset += compared;
        if left_offset == left_fragment.len() {
            match left_fragments.next() {
                Some(next) => {
                    left_fragment = next;
                    left_offset = 0;
                }
                None => return if right_offset == right_fragment.len() && right_fragments.next().is_none() { std::cmp::Ordering::Equal } else { std::cmp::Ordering::Less },
            }
        }
        if right_offset == right_fragment.len() {
            match right_fragments.next() {
                Some(next) => {
                    right_fragment = next;
                    right_offset = 0;
                }
                None => return std::cmp::Ordering::Greater,
            }
        }
    }
}

fn index_bytes_compare_prefix(value: &IndexBytes, prefix: &IndexBytes) -> bool {
    if prefix.len() > value.len() {
        return false;
    }
    let mut compared = 0;
    for fragment in value.fragments() {
        for byte in fragment {
            let Some(prefix_byte) = prefix.pages.page((compared / db_io_core::DB_IO_PAGE_BYTES) as u8).and_then(|page| page.get(compared % db_io_core::DB_IO_PAGE_BYTES)) else { return true };
            if byte != prefix_byte {
                return false;
            }
            compared += 1;
            if compared == prefix.len() {
                return true;
            }
        }
    }
    prefix.is_empty()
}
//#endregion 🔖️SortedRun
