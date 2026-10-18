//! 📨️ Retained Actor Job publications own replay pages, progress checkouts and their exact acknowledgement.
use std::mem::{ManuallyDrop,MaybeUninit};
use semio_framework_value_derive::{FromValue,ToValue};
use semio_framework_actor_vocabulary::*;

#[path = "🧭️replay-routes-and-policy/🦀️.rs"]
mod replay_routes_and_policy;
use replay_routes_and_policy::{JOB_REPLAY_SCALAR_TRANSFERS, JobReplayPage};
pub use replay_routes_and_policy::{JOB_REPLAY_PAGE_BYTES, JOB_REPLAY_PROCESS_PAGE_CAPACITY, JOB_REPLAY_RECORD_CAPACITY, JOB_REPLAY_RECORD_PAGE_CAPACITY, JobReplayCloseStep, JobReplayFault, JobReplayPublicationKind, JobReplayPublicationPolicy, JobReplayRoute, JobReplayStep};

#[path = "📼️replay-records-and-capture/🦀️.rs"]
mod replay_records_and_capture;
use replay_records_and_capture::{JobReplayCapture, JobReplayRecord};
pub use replay_records_and_capture::{JobReplayRecordHeader, JobReplayRejected, JobReplaySchedule};

#[path = "📜️replay-log/🦀️.rs"]
mod replay_log;
use replay_log::replay_payload_items;
pub use replay_log::JobReplayLog;

#[path = "🪪️progress-identities-and-faults/🦀️.rs"]
mod progress_identities_and_faults;
pub use progress_identities_and_faults::{JOB_PROGRESS_ACTIVE_CAPACITY, JOB_PROGRESS_PAGE_MAXIMUM_BYTES, JOB_PROGRESS_RETIREMENT_CAPACITY, JOB_PROGRESS_TOTAL_MAXIMUM_BYTES, JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS, JobProgressFault, JobProgressIdentity, JobProgressKind, JobProgressLiveAuthority, JobProgressRejected};

#[path = "🎰️progress-slots-and-receipts/🦀️.rs"]
mod progress_slots_and_receipts;
use progress_slots_and_receipts::{JobProgressPriorState, JobProgressRetirementSlot, JobProgressSlot, JobPublicationOwner, publication_owner_shape};
pub use progress_slots_and_receipts::{JobProgressAdmission, JobProgressCloseStep, JobProgressOverlayStore, JobProgressReceipt};

#[path = "🗃️progress-overlay-store/🦀️.rs"]
mod progress_overlay_store;
pub use progress_overlay_store::JobProgressCheckout;

#[path = "🌉️publication-error-and-bridge/🦀️.rs"]
mod publication_error_and_bridge;
pub use publication_error_and_bridge::JobPublicationError;
#[cfg(test)]
pub use publication_error_and_bridge::JobTurnBridge;

//#endregion 🪪️JobBridge

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
