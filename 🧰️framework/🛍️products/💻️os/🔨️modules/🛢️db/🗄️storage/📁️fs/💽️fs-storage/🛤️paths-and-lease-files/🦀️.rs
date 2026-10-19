//! 🛤️ Fs paths and lease files: the open error, the safe component, the wal, snapshot, index, payload, catalog, lease, segment, marker, generation and run paths and the lease file reader and writer. See [`super`].

use super::*;

/// 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
/// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
/// caller-addressed blob that might legitimately not exist yet.
// 🚫️async: E1 pure accessor called from sync `.map_err(|err| open_err(...))` closures — see R9
pub(super) fn open_err(err: std::io::Error, missing: impl FnOnce() -> String) -> DbError {
    if err.kind() == std::io::ErrorKind::NotFound {
        DbError::NotFound(missing())
    } else {
        io_err(err)
    }
}

/// 🛡️ Rejects a path component that could escape `root` (empty, `.`, `..`, or
/// containing a path separator/NUL) — every document id, resource name, etc. that becomes a
/// filesystem path component is validated through this before use.
pub(super) fn safe_component(raw: &str) -> Result<&str, DbError> {
    if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
        return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
    }
    Ok(raw)
}

fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
    Ok(root.join("wal").join(safe_component(&document.0)?))
}

fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
    Ok(root.join("snapshot").join(safe_component(&document.0)?))
}

fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
    Ok(root.join("index").join(safe_component(&document.0)?))
}

pub(super) fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
    let hex = hash.to_string();
    root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
}

pub(super) fn catalog_path(root: &Path) -> PathBuf {
    root.join("catalog").join("root.bin")
}

pub(super) fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
    Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
}

pub(super) fn segment_path(dir: &Path, index: u64) -> PathBuf {
    dir.join(format!("segment-{index:020}.bin"))
}

pub(super) fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
    dir.join(format!("segment-{index:020}.sealed"))
}

pub(super) fn generation_path(dir: &Path, generation: u64) -> PathBuf {
    dir.join(format!("gen-{generation:020}.pack"))
}

pub(super) fn run_path(dir: &Path, run_id: u64) -> PathBuf {
    dir.join(format!("run-{run_id:020}.bin"))
}

pub(super) fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, DbIoText)>, DbError> {
    if !path.exists() {
        return Ok(None);
    }
    let mut file = std::fs::File::open(path).map_err(io_err)?;
    let mut header = [0u8; 18];
    file.read_exact(&mut header).map_err(io_err)?;
    let mut epoch_bytes = [0u8; 8];
    epoch_bytes.copy_from_slice(&header[..8]);
    let mut expiry_bytes = [0u8; 8];
    expiry_bytes.copy_from_slice(&header[8..16]);
    let mut holder_len_bytes = [0u8; 2];
    holder_len_bytes.copy_from_slice(&header[16..18]);
    let epoch = u64::from_le_bytes(epoch_bytes);
    let expires_at_ms = u64::from_le_bytes(expiry_bytes);
    let holder_len = usize::from(u16::from_le_bytes(holder_len_bytes));
    if holder_len > super::super::DB_IO_TEXT_BYTES || file.metadata().map_err(io_err)?.len() != (18 + holder_len) as u64 {
        return Err(DbError::Corrupt("lease record exceeds its fixed authority".to_string()));
    }
    let mut holder = [0u8; super::super::DB_IO_TEXT_BYTES];
    file.read_exact(&mut holder[..holder_len]).map_err(io_err)?;
    let holder = std::str::from_utf8(&holder[..holder_len]).map_err(|_| DbError::Corrupt("lease holder is not valid UTF-8".to_string()))?;
    Ok(Some((EpochFence { epoch }, expires_at_ms, DbIoText::try_from_str(holder)?)))
}

pub(super) struct FsLeaseTransportPolicy { handle: super::DbIoTaskHandle }

    impl pack::PackTransportPolicy for FsLeaseTransportPolicy {
        fn checkpoint(&self, _: pack::PackTransportProgress) -> Result<(), semio_framework_value::ValueError> {
            if super::db_io_cancel_requested(self.handle) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled, "filesystem lease operation canceled")); }
            Ok(())
        }
        fn progress(&self, _: pack::PackTransportProgress) {
            let waker = {
                let owner = super::DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !super::db_io_slot_matches(&owner, self.handle) { return; }
                owner.waker.clone()
            };
            if let Some(waker) = waker { waker.wake(); }
        }
    }

    fn write_lease_file(path: &Path, operation: u64, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {
    let holder = DbIoText::try_from_str(holder)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(io_err)?;
    }
    let mut record = [0u8; 18 + super::super::DB_IO_TEXT_BYTES];
    record[..8].copy_from_slice(&fence.epoch.to_le_bytes());
    record[8..16].copy_from_slice(&expires_at_ms.to_le_bytes());
    record[16..18].copy_from_slice(&(holder.as_str().len() as u16).to_le_bytes());
    record[18..18 + holder.as_str().len()].copy_from_slice(holder.as_str().as_bytes());
    let handle = super::DB_IO_TASK_SLOTS.iter().enumerate().find_map(|(slot, owner)| {
            let owner = owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            (owner.operation == operation && owner.counted).then_some(super::DbIoTaskHandle { slot: slot as u16, generation: owner.generation, operation })
        }).ok_or(DbError::Closed)?;
        let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(super::DB_IO_PAGE_BYTES, 0), FsLeaseTransportPolicy { handle }).map_err(|refusal| {
            let error = match refusal.cause {
                pack::TransportContextRefusalCause::Admission(refusal) => pack::PackRefusal::TransportAdmission { category: pack::PackTransportCategory::NativeIo, refusal },
                pack::TransportContextRefusalCause::Value(error) => pack::PackRefusal::ValueRefusal(error),
            };
            DbError::from(error)
        })?;
        pack::write_atomic(path, &record[..18 + holder.as_str().len()], &context)?;
    Ok(())
}
