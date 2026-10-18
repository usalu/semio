use super::*;

use db_io_core::WalStorage;
use db_storage_memory::MemoryStorage;
use db_wal_log::{GroupCommitPolicy, MAX_FIELD_BYTES, SharedBuf, WAL_COMMAND, WAL_FRONTIER, WAL_PAYLOAD, WAL_SNAPSHOT_PUB, WAL_TRANSACTION_RECORDS_MAX, WalAuthenticatedSource, WalAuthenticatedStep, WalBytes, WalCommittedRecordStep, WalCommittedStep, WalCursorControl, WalImmutableByteSource, WalPayloadRef, WalPriorChainTip, WalRecord, WalRecordBatch, WalRecoveryReport, WalSegmentChain, WalTransactionGate, WalVerifiedFrameStep, replay_committed_document, replay_document, wal_crc_range, wal_next_verified_page_frame};

include!("🧰️wal-test-fixtures/🦀️.rs");
include!("🛑️recovery-abort-laws/🦀️.rs");
include!("📏️neutral-committed-span-laws/🦀️.rs");
include!("🧮️capacity-preflight-and-neutral-recovery-laws/🦀️.rs");
include!("🧩️segment-and-artifact-wal-laws/🦀️.rs");
