//! 💽️ Filesystem storage module: the lifecycle, durable file operations, paths, executor and storage backend of the filesystem storage, with the shared imports of the former inline module. See [`super`].

use super::check_len;
use db_io_driver::{WalFileWriterGuard, WalWriterTable};
use super::{
    close_db_io_backend, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted, ArtifactId, ByteRange, ContentHash, DbError, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoBackendRollbackReservation,
    DbIoExecutionStep, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbStorageOpenRejected, DurabilityClass, EpochFence, LeaseInfo, DB_IO_MAX_READ_BYTES,
};
use super::{CatalogStorage, DbIoWriterReleaseStep, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit};
use semio_framework_async::WorkerPool;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[path = "🚦️lifecycle-state/🦀️.rs"]
mod lifecycle_state;
use lifecycle_state::{io_err, FsLifecycle, FsLifecyclePhase, FS_DURABLE_DIRECTORIES};
#[cfg(test)]
use lifecycle_state::FsLifecycleState;

#[path = "💾️durable-file-operations/🦀️.rs"]
mod durable_file_operations;
use durable_file_operations::{durable_directory, durable_file_sync, durable_wal_create, durable_wal_delete, durable_wal_seal, sync_directory};

#[path = "🛤️paths-and-lease-files/🦀️.rs"]
mod paths_and_lease_files;
use paths_and_lease_files::{
    catalog_path, generation_path, lease_path, open_err, payload_path, read_lease_file, run_path, safe_component, sealed_marker_path, segment_path, write_lease_file,
};

#[path = "🧰️fs-executor/🦀️.rs"]
mod fs_executor;
use fs_executor::FsDbIoExecutor;

#[path = "📨️fs-task-executor/🦀️.rs"]
mod fs_task_executor;

#[path = "📁️fs-storage-backend/🦀️.rs"]
mod fs_storage_backend;
use fs_storage_backend::read_root_fence;
pub use fs_storage_backend::FsStorage;

#[cfg(test)]
include!("../../🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs");
