//! 🔎️ State scan, clone and encode: the scan, candidate reservation, clone, snapshot, encode and seal steps of the state. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
impl DatabaseCreateCatalogState {
    fn add_encoded(cursor: &mut DatabaseCreateCatalogCursor, bytes: usize) -> Result<(), DbError> {
        let next = cursor.encoded_len.checked_add(bytes).ok_or(DbError::LimitExceeded("database create-catalog encoded bytes"))?;
        if next > DATABASE_CREATE_CATALOG_MAX_PAGES * db_io_core::DB_IO_PAGE_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog page capacity"));
        }
        cursor.encoded_len = next;
        Ok(())
    }

    pub(super) fn scan_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Scanning);
        let document_guard = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = document_guard.as_ref() else {
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog document owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base) = cursor.base.as_ref().map(Arc::clone) else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog base owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let entry_count = base.len();
        let (source, source_capacity, created_at_ms, duplicate) = if cursor.scan_entry < entry_count {
            let entry = &base[cursor.scan_entry];
            (entry.document.0.as_str(), entry.document.0.capacity(), entry.created_at_ms, !cursor.scan_started && entry.document == *document)
        } else {
            (document.0.as_str(), document.0.capacity(), cursor.created_at_ms, false)
        };
        if source_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES || source.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog base document bytes"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        if !cursor.scan_started {
            if cursor.scan_entry < entry_count {
                if let Err(error) = cursor.backing.observe(1, source_capacity) {
                    drop(cursor);
                    drop(document_guard);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            if duplicate {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::AlreadyExists(String::from("document already exists")), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            let mut decimal = [0u8; 20];
            let fixed = usize::from(cursor.scan_entry != 0) + b"{\"document\":\"".len() + b"\",\"created_at_ms\":".len() + decimal_u64(created_at_ms, &mut decimal).len() + 1;
            if let Err(error) = Self::add_encoded(&mut cursor, fixed) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            cursor.scan_started = true;
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.scan_byte < source.len() {
            let byte = source.as_bytes()[cursor.scan_byte];
            cursor.scan_byte += 1;
            let bytes = match byte {
                b'"' | b'\\' | 0x08 | 0x0c | b'\n' | b'\r' | b'\t' => 2,
                0x00..=0x1f => 6,
                _ => 1,
            };
            if let Err(error) = Self::add_encoded(&mut cursor, bytes) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        cursor.scan_entry += 1;
        cursor.scan_byte = 0;
        cursor.scan_started = false;
        if cursor.scan_entry > entry_count {
            self.set_phase(DatabaseCreateCatalogPhase::Reserve);
        } else {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        }
    }

    pub(super) fn reserve_candidate_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base_len) = cursor.base.as_ref().map(|base| base.len()) else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog reserve base"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let Some(capacity) = base_len.checked_add(1) else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog entry count"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut candidate = Vec::new();
        if candidate.try_reserve_exact(capacity).is_err() {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog entry backing"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        let observed_capacity = self.observed_capacity(candidate.capacity());
        cursor.candidate = Some(candidate);
        let backing_bytes = match observed_capacity.checked_mul(size_of::<CatalogEntry>()) {
            Some(bytes) => bytes,
            None => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog candidate observed bytes"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let observed_items = match u64::try_from(observed_capacity) {
            Ok(items) => items,
            Err(_) => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog candidate observed items"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        if let Err(error) = cursor.backing.observe(observed_items, backing_bytes) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.clone_entry = 0;
        cursor.clone_byte = 0;
        self.set_phase(DatabaseCreateCatalogPhase::Clone);
    }

    fn clone_boundary(source: &str, start: usize) -> usize {
        let mut end = start.saturating_add(DATABASE_CREATE_CATALOG_COPY_BYTES).min(source.len());
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        end
    }

    pub(super) fn clone_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Copying);
        let document_guard = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = document_guard.as_ref() else {
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog clone document"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base) = cursor.base.as_ref().map(Arc::clone) else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog clone base"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let (source, created_at_ms) = if cursor.clone_entry < base.len() {
            let entry = &base[cursor.clone_entry];
            (entry.document.0.as_str(), entry.created_at_ms)
        } else {
            (document.0.as_str(), cursor.created_at_ms)
        };
        if cursor.clone_text.is_none() {
            let mut text = String::new();
            if text.try_reserve_exact(source.len()).is_err() {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::LimitExceeded("database create-catalog string backing"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            let observed_capacity = self.observed_capacity(text.capacity());
            cursor.clone_text = Some(text);
            if observed_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::LimitExceeded("database create-catalog cloned string capacity"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            if let Err(error) = cursor.backing.observe(1, observed_capacity) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.clone_byte < source.len() {
            let end = Self::clone_boundary(source, cursor.clone_byte);
            let start = cursor.clone_byte;
            cursor.clone_text.as_mut().unwrap_or_else(|| unreachable!()).push_str(&source[start..end]);
            cursor.clone_byte = end;
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        let text = cursor.clone_text.take().unwrap_or_default();
        let Some(candidate) = cursor.candidate.as_mut() else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog candidate owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        candidate.push(CatalogEntry { document: protocol::ArtifactId(text), created_at_ms });
        cursor.clone_entry += 1;
        cursor.clone_byte = 0;
        if cursor.clone_entry > base.len() {
            self.set_phase(DatabaseCreateCatalogPhase::Snapshot);
        } else {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        }
    }

    pub(super) fn snapshot_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(candidate) = cursor.candidate.take() else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog candidate snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let pages = cursor.encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        if pages == 0 || pages > DATABASE_CREATE_CATALOG_MAX_PAGES {
            cursor.candidate = Some(candidate);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog page count"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.snapshot = Some(Arc::new(candidate));
        if let Err(error) = cursor.backing.observe(1, DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.writer = match db_io_core::DbIoPageWriter::try_reserve(pages) {
            Ok(writer) => Some(writer),
            Err(rejected) => {
                let error = rejected.into_error();
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let page_bytes = match pages.checked_mul(db_io_core::DB_IO_PAGE_BYTES) {
            Some(bytes) => bytes,
            None => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog observed page bytes"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let page_items = match u64::try_from(pages) {
            Ok(items) => items,
            Err(_) => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog observed page items"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        if let Err(error) = cursor.backing.observe(page_items, page_bytes) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.encode = DatabaseCreateCatalogEncodeCursor::new();
        self.set_phase(DatabaseCreateCatalogPhase::Encode);
    }

    pub(super) fn encode_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Encoding);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let snapshot = cursor.snapshot.as_ref().map(Arc::clone);
        let Some(snapshot) = snapshot else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog encode snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut encode = std::mem::replace(&mut cursor.encode, DatabaseCreateCatalogEncodeCursor::new());
        let step = cursor.writer.as_mut().ok_or(DbError::LimitExceeded("database create-catalog writer owner")).and_then(|writer| encode.step(snapshot.as_slice(), writer));
        cursor.encode = encode;
        match step {
            Ok(true) => self.set_phase(DatabaseCreateCatalogPhase::Seal),
            Ok(false) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => {
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub(super) fn seal_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let step = cursor.writer.as_mut().ok_or(DbError::LimitExceeded("database create-catalog seal writer")).and_then(db_io_core::DbIoPageWriter::seal_retained_step);
        match step {
            Ok(Some(pages)) => {
                cursor.writer.take();
                cursor.pages = Some(pages);
                self.set_phase(DatabaseCreateCatalogPhase::Claim);
            }
            Ok(None) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => {
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

}
//#endregion 🔖️CreateDocumentCatalogCas
