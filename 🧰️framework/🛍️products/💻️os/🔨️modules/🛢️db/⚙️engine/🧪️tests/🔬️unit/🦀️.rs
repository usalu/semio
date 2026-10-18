use super::*;

use db_engine_base::{Consistency, DATABASE_DOCUMENT_MOUNT_WAITERS, DatabaseDocumentMountDriver, DatabaseDocumentMountRegistry, DatabaseDocumentMountSlot, DatabaseDocumentMountWork, LiveQuerySpec, Query, QueryResultEntry, QueryStream, SnapshotKind, now_ms, test_worker_pool, to_core_document_id};
use db_engine_handle::{ARTIFACT_HISTORY_ADMISSION, ARTIFACT_HISTORY_OPERATION_SLOTS, ArtifactHandle, ArtifactHistoryAdmission, ArtifactHistoryAdmissionError, ArtifactHistoryState, HistoryFuture};
use db_io_core::{PayloadStorage as _, WalStorage as _};

include!("🧸️fixtures-batch-and-concurrent-ensure/🦀️.rs");
include!("🏊️worker-pool-use-and-mount-owner-laws/🦀️.rs");
include!("📣️mount-fanout-shutdown-and-mount-failure-laws/🦀️.rs");
include!("🔌️mount-cleanup-fault-round-trip-and-deferred-seams/🦀️.rs");
include!("✍️writer-residency-compaction-and-hello/🦀️.rs");
include!("📚️history-wake-and-terminal-close-laws/🦀️.rs");
include!("🧭️long-module-and-source-laws/🦀️.rs");
