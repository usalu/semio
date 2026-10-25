//! 📜️ Db artifact history replay.
extern crate semio_framework_os_kernel as pack;

use db_durability::Frontier;
use db_ids::{*, ArtifactId, DbError};
use protocol::MutationDiff as _;
use std::{future::Future, pin::Pin, sync::Arc};
use db_io_core::WalStorage;

#[path = "🧭️replay-limits-view-and-reservation/🦀️.rs"]
mod replay_limits_view_and_reservation;
use replay_limits_view_and_reservation::{
    HistoryTextRange, HISTORY_REPLAY_CONSTRUCTION_SLOTS, HISTORY_REPLAY_MAX_ENTRIES,
    HISTORY_REPLAY_MAX_FIELD_BYTES, HISTORY_REPLAY_MAX_FRAME_BYTES,
    HISTORY_REPLAY_MAX_OPERATION_IDS, HISTORY_REPLAY_OPERATION_BYTES, HISTORY_REPLAY_PAGE_BYTES,
    HISTORY_REPLAY_SEGMENT_PAGES,
};
#[cfg(test)]
use replay_limits_view_and_reservation::{HISTORY_REPLAY_RESULT_BYTES, HISTORY_REPLAY_RESULT_PAGES};
pub use replay_limits_view_and_reservation::{
    ArtifactHistoryEntry, ArtifactHistoryView, HistoryReplayReservation,
    HistoryReplayReservationCloseCursor,
};
#[cfg(any(test,feature="testkit"))]
pub use replay_limits_view_and_reservation::history_capacity_test_lock;

#[path = "🛠️reservation-construction/🦀️.rs"]
mod reservation_construction;
#[cfg(test)]
use reservation_construction::{
    claim_history_replay_reservation_construction, handback_history_replay_reservation_construction,
    history_replay_reservation_construction_registry, HistoryReplayReservationConstructionToken,
};
#[cfg(test)]
pub(crate) use reservation_construction::{
    take_history_replay_reservation_construction_fault,
    HistoryReplayReservationConstructionHandbackRejection,
};
use reservation_construction::HistoryReplayReservationConstructionBuilder;
pub use reservation_construction::HistoryReplayReservationConstructionFault;

#[path = "🔖️page-set-and-cursors/🦀️.rs"]
mod page_set_and_cursors;
use page_set_and_cursors::HistoryPageSet;
pub use page_set_and_cursors::{
    HistoryEnvelopeCursor, HistoryEnvelopeField, HistoryFrontierCursor, HistoryFrontierField,
};

#[path = "⏳️replay-future/🦀️.rs"]
mod replay_future;
pub use replay_future::{HistoryReplayFuture, HistoryReplayPhase, HistoryReplayTransition};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
