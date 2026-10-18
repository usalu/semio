//! 🎫️ Db engine artifact handle: submit, history and query operations.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use db_engine_base::{CheckpointPublicationSnapshot, CommandReceipt, Consistency, DatabaseDocumentMountRegistry, DatabaseDocumentMountSlot, Frontier, LiveQuery, LiveQuerySpec, PreviewHandle, Query, QueryResultEntry, QueryStream, SnapshotFuture, SnapshotKind, engine_query_maintenance_step, now_ms, reserve_engine_query_retirement, to_engine_frontier, to_engine_receipt};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_ids::ActorId;

//#region 🔖️History
pub use db_artifact_history::ArtifactHistoryEntry as HistoryEntry;

#[path = "🎟️history-view-lease-and-journal-sink/🦀️.rs"]
mod history_view_lease_and_journal_sink;
pub use history_view_lease_and_journal_sink::{
    ArtifactCompactionFuture, ArtifactHandleLease, HistoryView,
};
use history_view_lease_and_journal_sink::ArtifactHandleJournalSinkV1;

#[path = "📨️submit-admission-and-state/🦀️.rs"]
mod submit_admission_and_state;
#[cfg(test)]
use submit_admission_and_state::{
    ARTIFACT_SUBMIT_BATCH_ITEMS, ARTIFACT_SUBMIT_OPERATION_BYTES, ARTIFACT_SUBMIT_PAGE_BYTES,
};
use submit_admission_and_state::{
    artifact_submit_credit, ArtifactSubmitAdmission, ArtifactSubmitState, ArtifactSubmitWorkOwner,
    ARTIFACT_SUBMIT_RETRY_LIMIT, ARTIFACT_SUBMIT_RETRY_MS,
};
pub use submit_admission_and_state::{
    ArtifactSubmitOutcome, ArtifactSubmitTerminalJob, ArtifactSubmitTerminalWork, SubmitFuture,
    SubmitProgress,
};

#[path = "📤️submit-future-and-terminal/🦀️.rs"]
mod submit_future_and_terminal;

#[path = "🚪️history-admission-and-wake/🦀️.rs"]
mod history_admission_and_wake;
pub use history_admission_and_wake::{
    ArtifactHistoryAdmission, ArtifactHistoryAdmissionError, ArtifactHistoryOutcome,
    ArtifactHistoryState, ArtifactHistoryTerminalConstructionFault, ArtifactHistoryTerminalHandle,
    ArtifactHistoryTerminalJob, ArtifactHistoryTerminalReservation, ArtifactHistoryTerminalWork,
    HistoryFuture, HistoryProgress, ARTIFACT_HISTORY_ADMISSION, ARTIFACT_HISTORY_OPERATION_SLOTS,
};
use history_admission_and_wake::{
    artifact_history_registry, register_artifact_history, unregister_artifact_history,
    ArtifactHistoryWake, ArtifactHistoryWorkOwner,
};

#[path = "📚️history-state/🦀️.rs"]
mod history_state;

#[path = "🔭️history-future-and-terminals/🦀️.rs"]
mod history_future_and_terminals;

#[path = "🤝️artifact-handle/🦀️.rs"]
mod artifact_handle;
pub use artifact_handle::ArtifactHandle;

//#endregion 🔖️History

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
