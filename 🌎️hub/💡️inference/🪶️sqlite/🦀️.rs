//! 🪶️ Bounded SQLite private-job ledger with durable idempotency and first-terminal-wins.

use semio_hub_inference::command::COMMAND_MAX_BYTES;
use semio_hub_inference::{sha256, InferenceErrorV1, InferencePrivateBytesV1};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use std::path::Path;
use std::sync::Mutex;
use semio_hub_inference::schema::{CLAIM_LEASE_MAX_MS, EVENT_PAGE_MAX_ITEMS, IDENTITY_JSON_MAX_BYTES, INPUT_MAX_BYTES, InferenceApprovalReceiptV1, InferenceEventV1, InferenceIdentityV1, InferenceJobReceiptV1, InferenceJobReconcileApprovalStateV1, InferenceJobReconcileApprovalV1, InferenceJobReconcileJobV1, InferenceJobReconcilePageV1, InferenceJobStateV1, InferenceProgressV1, InferenceProposalStateV1, JOB_CAPACITY, PROGRESS_MAX_CURSOR, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES, SAFE_INTEGER_MAX, hex};
use semio_hub_gis::inference::{GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1};

#[path = "🧱️ledger-storage/🦀️.rs"]
mod ledger_storage;
use ledger_storage::*;

#[path = "🧾️ledger-types/🦀️.rs"]
mod ledger_types;
pub use ledger_types::*;

#[path = "🗺️gis-map-undo/🦀️.rs"]
mod gis_map_undo;
pub use gis_map_undo::*;

#[path = "✅️approvals/🦀️.rs"]
mod approvals;
pub(crate) use approvals::*;

#[path = "🎬️job-lifecycle/🦀️.rs"]
mod job_lifecycle;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
