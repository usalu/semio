//! 💾️ Fs durable file operations: the directory sync, the durable file sync and directory creation and the durable wal create, seal and delete. See [`super`].

use super::*;

/// 🧱️ A directory-name durability barrier on the admitted filesystem lane; unsupported flushes fail closed.
pub(super) fn sync_directory(path: &Path) -> Result<(), DbError> {
    let path = if path.as_os_str().is_empty() { Path::new(".") } else { path };
    #[cfg(windows)]
    let file = {
        use std::os::windows::fs::OpenOptionsExt as _;
        std::fs::OpenOptions::new().read(true).write(true).custom_flags(0x0200_0000).open(path).map_err(io_err)?
    };
    #[cfg(not(windows))]
    let file = std::fs::File::open(path).map_err(io_err)?;
    #[cfg(target_vendor = "apple")]
    {
        use std::os::fd::AsRawFd as _;
        unsafe extern "C" {
            fn fsync(fd: std::ffi::c_int) -> std::ffi::c_int;
        }
        if unsafe { fsync(file.as_raw_fd()) } != 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        Ok(())
    }
    #[cfg(not(target_vendor = "apple"))]
    file.sync_all().map_err(io_err)
}

/// 💾️ Makes `file`'s bytes durable on the medium (see `FsFullSyncGroup`); elsewhere the
/// platform's own full `fsync`.
pub(super) fn durable_file_sync(file: &std::fs::File, lifecycle: &FsLifecycle) -> Result<(), DbError> {
    #[cfg(target_vendor = "apple")]
    {
        use std::os::fd::AsRawFd as _;
        const F_FULLFSYNC: std::ffi::c_int = 51;
        unsafe extern "C" {
            fn fsync(fd: std::ffi::c_int) -> std::ffi::c_int;
            fn fcntl(fd: std::ffi::c_int, command: std::ffi::c_int, ...) -> std::ffi::c_int;
        }
        if unsafe { fsync(file.as_raw_fd()) } != 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        let group = &lifecycle.full_sync;
        let mut epochs = group.epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        epochs.requested += 1;
        let ticket = epochs.requested;
        loop {
            if epochs.completed >= ticket {
                return Ok(());
            }
            if epochs.flushing {
                epochs = group.flushed.wait(epochs).unwrap_or_else(std::sync::PoisonError::into_inner);
                continue;
            }
            epochs.flushing = true;
            let target = epochs.requested;
            drop(epochs);
            let failure = (unsafe { fcntl(file.as_raw_fd(), F_FULLFSYNC) } == -1).then(std::io::Error::last_os_error);
            epochs = group.epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            epochs.flushing = false;
            if failure.is_none() {
                epochs.completed = epochs.completed.max(target);
            }
            group.flushed.notify_all();
            if let Some(error) = failure {
                return Err(io_err(error));
            }
        }
    }
    #[cfg(not(target_vendor = "apple"))]
    {
        let _ = lifecycle;
        file.sync_all().map_err(io_err)
    }
}

pub(super) fn durable_directory(path: &Path, lifecycle: &FsLifecycle) -> Result<(), DbError> {
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    if lifecycle.durable_directories.lock().unwrap_or_else(std::sync::PoisonError::into_inner).paths.contains(path) && path.is_dir() {
        return Ok(());
    }
    let parent = path.parent();
    if let Some(parent) = parent {
        durable_directory(parent, lifecycle)?;
    }
    match std::fs::create_dir(path) {
        Ok(()) => lifecycle.hit(FsLifecyclePhase::DirectoryCreated)?,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists && path.is_dir() => {}
        Err(error) => return Err(io_err(error)),
    }
    sync_directory(parent.unwrap_or(path))?;
    lifecycle.hit(FsLifecyclePhase::DirectoryParentSynced)?;
    let mut durable = lifecycle.durable_directories.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if durable.paths.len() == FS_DURABLE_DIRECTORIES {
        durable.paths.clear();
    }
    durable.paths.insert(path.to_path_buf());
    Ok(())
}

pub(super) fn durable_wal_create(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
    durable_directory(dir, lifecycle)?;
    let file = std::fs::OpenOptions::new().write(true).create_new(true).open(segment_path(dir, index)).map_err(|error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            DbError::AlreadyExists(format!("wal segment {index} already exists"))
        } else {
            io_err(error)
        }
    })?;
    lifecycle.hit(FsLifecyclePhase::SegmentCreated)?;
    durable_file_sync(&file, lifecycle)?;
    lifecycle.hit(FsLifecyclePhase::SegmentFileSynced)?;
    drop(file);
    sync_directory(dir)?;
    lifecycle.hit(FsLifecyclePhase::SegmentParentSynced)
}

pub(super) fn durable_wal_seal(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
    std::fs::metadata(segment_path(dir, index)).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
    let marker = sealed_marker_path(dir, index);
    let file = match std::fs::OpenOptions::new().write(true).create_new(true).open(&marker) {
        Ok(file) => {
            lifecycle.hit(FsLifecyclePhase::MarkerCreated)?;
            file
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let file = std::fs::OpenOptions::new().write(true).open(&marker).map_err(io_err)?;
            let metadata = file.metadata().map_err(io_err)?;
            if !metadata.is_file() || metadata.len() != 0 {
                return Err(DbError::Corrupt("WAL seal marker must be an empty regular file".to_string()));
            }
            file
        }
        Err(error) => return Err(io_err(error)),
    };
    durable_file_sync(&file, lifecycle)?;
    lifecycle.hit(FsLifecyclePhase::MarkerFileSynced)?;
    drop(file);
    sync_directory(dir)?;
    lifecycle.hit(FsLifecyclePhase::MarkerParentSynced)
}

pub(super) fn durable_wal_delete(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
    match std::fs::symlink_metadata(dir) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => return Err(DbError::Corrupt("WAL directory must be a directory, not a link".to_string())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(io_err(error)),
    }
    for (path, removed, synced) in
        [(segment_path(dir, index), FsLifecyclePhase::SegmentDeleted, FsLifecyclePhase::SegmentDeleteParentSynced), (sealed_marker_path(dir, index), FsLifecyclePhase::MarkerDeleted, FsLifecyclePhase::MarkerDeleteParentSynced)]
    {
        match std::fs::remove_file(path) {
            Ok(()) => lifecycle.hit(removed)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(io_err(error)),
        }
        sync_directory(dir)?;
        lifecycle.hit(synced)?;
    }
    Ok(())
}
