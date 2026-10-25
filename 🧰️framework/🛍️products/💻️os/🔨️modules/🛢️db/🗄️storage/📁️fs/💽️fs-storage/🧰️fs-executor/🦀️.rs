//! 🧰️ Fs executor: the filesystem executor, its read state and its direct storage operations. See [`super`].

use super::*;

pub(super) struct FsDbIoExecutor {
    pub(super) root: DbIoText,
    pub(super) lifecycle: FsLifecycle,
    pub(super) catalog_lock: Mutex<()>,
    pub(super) lease_lock: Mutex<()>,
    pub(super) writers: Mutex<Option<Box<WalWriterTable<WalFileWriterGuard>>>>,
    pub(super) payload_hashes: [Mutex<Option<(u64, semio_framework_hash::Hasher)>>; super::super::DB_IO_LEDGER_ITEMS],
    pub(super) readers: [Mutex<Option<FsReadState>>; super::super::DB_IO_LEDGER_ITEMS],
    pub(super) backend_close_cursor: std::sync::atomic::AtomicUsize,
    pub(super) backend_terminal: std::sync::atomic::AtomicBool,
}

pub(super) struct FsReadState {
    pub(super) operation: u64,
    pub(super) file: std::fs::File,
    pub(super) offset: u64,
    pub(super) total: usize,
    pub(super) fence: Option<EpochFence>,
}

impl FsDbIoExecutor {
    pub(super) fn new(root: DbIoText) -> Self {
        Self {
            root,
            lifecycle: FsLifecycle::default(),
            catalog_lock: Mutex::new(()),
            lease_lock: Mutex::new(()),
            writers: Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
            payload_hashes: [const { Mutex::new(None) }; super::super::DB_IO_LEDGER_ITEMS],
            readers: [const { Mutex::new(None) }; super::super::DB_IO_LEDGER_ITEMS],
            backend_close_cursor: std::sync::atomic::AtomicUsize::new(0),
            backend_terminal: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub(super) fn root(&self) -> &Path {
        Path::new(self.root.as_str())
    }

    pub(super) fn document_dir(&self, family: &str, document: &DbIoText) -> Result<PathBuf, DbError> {
        Ok(self.root().join(family).join(safe_component(document.as_str())?))
    }

    pub(super) fn writer_sidecar(&self, document: &DbIoText) -> Result<PathBuf, DbError> {
        let canonical = std::fs::canonicalize(self.root()).map_err(io_err)?;
        let canonical = canonical.to_str().ok_or_else(|| DbError::InvalidArgument("canonical filesystem storage root is not UTF-8".to_string()))?;
        let directory = Path::new(canonical).join(".semio-wal-writer");
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(canonical.as_bytes());
        hash.update(&[0]);
        hash.update(document.as_str().as_bytes());
        Ok(directory.join(format!("{}.lock", semio_framework_hash::hex_lower(&hash.finalize()))))
    }

    pub(super) fn read_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, catalog: bool, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<(DbIoPages, Option<EpochFence>)>), DbError> {
        let slot = super::super::db_io_operation_owner_slot(operation)?;
        let mut owner = self.readers[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if owner.as_ref().is_some_and(|reader| reader.operation != operation) {
            return Err(DbError::Unavailable("filesystem read cursor capacity exhausted".to_string()));
        }
        if owner.is_none() {
            let mut file = std::fs::File::open(path).map_err(|error| open_err(error, || format!("database I/O object {} not found", path.display())))?;
            let available = file.metadata().map_err(io_err)?.len().checked_sub(offset).ok_or_else(|| DbError::InvalidArgument("DB I/O read offset exceeds object length".to_string()))?;
            let total = exact_len.unwrap_or(available);
            check_len(total, DB_IO_MAX_READ_BYTES, "db_io typed filesystem read")?;
            if total > available {
                return Err(DbError::InvalidArgument("DB I/O read range exceeds object length".to_string()));
            }
            let fence = if catalog {
                let mut header = [0u8; 8];
                file.read_exact(&mut header).map_err(io_err)?;
                Some(EpochFence { epoch: u64::from_le_bytes(header) })
            } else {
                None
            };
            *owner = Some(FsReadState { operation, file, offset, total: total as usize, fence });
        }
        let Some(reader) = owner.as_mut() else {
            return Err(DbError::Internal("filesystem read cursor lost its retained owner".to_string()));
        };
        if output.len() == reader.total {
            let fence = reader.fence;
            return match output.seal_retained_step()? {
                Some(pages) => {
                    owner.take();
                    Ok((DbIoExecutionStep::Complete, Some((pages, fence))))
                }
                None => Ok((DbIoExecutionStep::Yield, None)),
            };
        }
        let mut fragment = [0u8; super::super::DB_IO_PAGE_BYTES];
        let fragment_len = (reader.total - output.len()).min(fragment.len());
        reader.file.seek(SeekFrom::Start(reader.offset + output.len() as u64)).map_err(io_err)?;
        reader.file.read_exact(&mut fragment[..fragment_len]).map_err(io_err)?;
        if output.write_fragment(&fragment[..fragment_len])? != fragment_len {
            return Err(DbError::Internal("DB I/O output writer accepted a partial fixed fragment".to_string()));
        }
        Ok((DbIoExecutionStep::Yield, None))
    }

    pub(super) fn read_pages_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let (step, result) = self.read_step(operation, path, offset, exact_len, false, output)?;
        Ok((step, result.map(|(pages, _)| DbIoResult::Pages(pages))))
    }

    pub(super) fn replace_step(&self, path: &Path, operation: u64, input: &mut DbIoPages, header: &[u8]) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let parent = path.parent().ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path has no parent".to_string()))?;
        durable_directory(parent, &self.lifecycle)?;
        let name = path.file_name().and_then(std::ffi::OsStr::to_str).ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path is not UTF-8".to_string()))?;
        let temporary = parent.join(format!(".{name}.{operation:016x}.dbio"));
        if !temporary.exists() {
            let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(io_err)?;
            if !header.is_empty() {
                file.write_all(header).map_err(io_err)?;
            }
        }
        if let Some(fragment) = input.page(0) {
            let fragment_len = fragment.len();
            let mut file = std::fs::OpenOptions::new().append(true).open(&temporary).map_err(io_err)?;
            file.write_all(fragment).map_err(io_err)?;
            input.advance(fragment_len)?;
            if !input.is_empty() {
                return Ok((DbIoExecutionStep::Yield, None));
            }
        }
        let file = std::fs::OpenOptions::new().write(true).open(&temporary).map_err(io_err)?;
        durable_file_sync(&file, &self.lifecycle)?;
        drop(file);
        std::fs::rename(temporary, path).map_err(io_err)?;
        self.lifecycle.hit(FsLifecyclePhase::ReplacementRenamed)?;
        sync_directory(parent)?;
        self.lifecycle.hit(FsLifecyclePhase::ReplacementParentSynced)?;
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }

    pub(super) fn list_step(&self, dir: &Path, prefix: &str, suffix: &str, list: &mut DbIoU64List) -> Result<bool, DbError> {
        if !dir.exists() {
            return Ok(true);
        }
        let after = list.as_slice().last().copied();
        for entry in std::fs::read_dir(dir).map_err(io_err)? {
            let name = entry.map_err(io_err)?.file_name();
            let name = name.to_string_lossy();
            let Some(number) = name.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix)) else { continue };
            let Ok(value) = number.parse::<u64>() else { continue };
            if after.is_none_or(|after| value > after) {
                list.push(value)?;
            }
        }
        list.sort_ascending();
        Ok(true)
    }

    pub(super) fn payload_hash_step(&self, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
        let slot = super::super::db_io_operation_owner_slot(operation)?;
        let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
            return Err(DbError::Unavailable("DB I/O filesystem hash cursor capacity exhausted".to_string()));
        }
        let (_, hasher) = state.get_or_insert_with(|| (operation, semio_framework_hash::Hasher::new()));
        if let Some(fragment) = input.page(0) {
            let len = fragment.len();
            hasher.update(fragment);
            input.advance(len)?;
        }
        if !input.is_empty() {
            return Ok(None);
        }
        let Some((_, hasher)) = state.take() else {
            return Err(DbError::Internal("DB I/O payload hash cursor lost its retained owner".to_string()));
        };
        Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
    }
}
