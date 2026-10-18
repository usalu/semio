//! 🗄️ `db_wal` — the `db` crate family's write-ahead log: a per-document, per-segment `.spr`
//! container (reusing `protocol::{SprWriter, FrameCursor, ReverseFrameCursor}` and
//! `protocol::format::recover` directly — no new framing invented) built on top of
//! `db_storage::WalStorage`'s opaque byte segments. Owns the WAL's own record kinds (SPR
//! extension range `0x40..=0x4F`), real group-commit batching (bounded by delay/bytes/count),
//! segment rotation with a cross-segment hash chain, and crash recovery (torn-tail truncation,
//! multi-segment replay). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice (dependency): the `protocol` facade re-exports `SprWriter`/`FrameCursor`/
//! `ReverseFrameCursor`/`RecoveryMode`/`RecoveryReport`/`ProtocolError`/`ProtocolLimits`, but not
//! the free functions `protocol::format::recover`/`parse_commit_payload`, the constants
//! `COMMIT_FRAME_LEN`/`HEADER_SIZE`, or `protocol::wire::REC_COMMIT`/`REQUIRED_HASH_CHAIN` — this
//! crate needs all of those (recovery; reading back a just-sealed segment's tip `chain_hash` to
//! seed the next segment's cross-segment chain; recognizing a `REC_COMMIT` frame explicitly while
//! decoding rather than assuming "not one of ours" means "must be a commit"; opening every segment
//! with hash-chaining required), so it takes direct `protocol_format`/`protocol_core` path
//! dependencies alongside `protocol` rather than duplicating their logic. Every other
//! protocol-family type this crate touches comes from the `protocol` facade, per the contract.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::Frontier;
use db_ids::{ArtifactId, DbError, check_len};
use pack::ContentHash;

#[path = "🏷️record-kinds-and-field-codec/🦀️.rs"]
mod record_kinds_and_field_codec;
pub use record_kinds_and_field_codec::{
    is_wal_record_kind, wal_field_len, wal_read_canonical_varint, wal_record_write, wal_varint_len,
    WalCursorControl, MAX_FIELD_BYTES, WAL_COMMAND, WAL_DIFF, WAL_EVENT, WAL_FRONTIER,
    WAL_INDEX_CKPT, WAL_INVERSE, WAL_LEASE, WAL_MIGRATION, WAL_OUTBOX, WAL_PAYLOAD,
    WAL_SEGMENT_HEADER, WAL_SNAPSHOT_PUB, WAL_TX_ABORT, WAL_TX_BEGIN, WAL_TX_COMMIT,
};
#[cfg(test)]
use record_kinds_and_field_codec::{decode_frontier, read_field_string, MAX_RUN_IDS};
#[cfg(any(test,feature="testkit"))]
use record_kinds_and_field_codec::{encode_frontier, write_field};
use record_kinds_and_field_codec::{
    wal_frontier_len, wal_record_write_field, wal_record_write_frontier, wal_varint,
};

#[path = "🧱️wal-bytes/🦀️.rs"]
mod wal_bytes;
pub use wal_bytes::{WalBytes, WalBytesCursor, WalBytesRejected, WalPayloadRef};

#[path = "🧾️wal-record/🦀️.rs"]
mod wal_record;
pub use wal_record::{WalRecord, WalRecordBatch};

#[path = "📤️payload-transform-group-commit-and-sink/🦀️.rs"]
mod payload_transform_group_commit_and_sink;
pub use payload_transform_group_commit_and_sink::{
    segment_write_options, GroupCommitPolicy, IdentityPayloadTransform, PayloadTransform, SharedBuf,
};

#[path = "📖️page-source-reader-and-frames/🦀️.rs"]
mod page_source_reader_and_frames;
pub use page_source_reader_and_frames::{
    protocol_err, wal_next_verified_page_frame, WalImmutableByteSource, WalPageReader,
    WalVerifiedFrameStep,
};
use page_source_reader_and_frames::{WalPageSource, WalRetainedRecordDecoder};
#[cfg(any(test,feature="testkit"))]
pub use page_source_reader_and_frames::wal_crc_range;
pub(crate) use page_source_reader_and_frames::WalRecordFrame;

#[path = "⛓️transaction-gate-and-segment-chain/🦀️.rs"]
mod transaction_gate_and_segment_chain;
pub use transaction_gate_and_segment_chain::{
    WalPriorChainTip, WalRecoveryReport, WalSegmentChain, WalTransactionGate,
    WAL_TRANSACTION_RECORDS_MAX,
};

#[path = "🔐️authenticated-source-and-replay-cursor/🦀️.rs"]
mod authenticated_source_and_replay_cursor;
pub use authenticated_source_and_replay_cursor::{
    WalAuthenticatedSource, WalAuthenticatedStep, WalReplayCursor, WalReplayStep,
};

#[path = "✅️committed-replay-and-validation/🦀️.rs"]
mod committed_replay_and_validation;
pub use committed_replay_and_validation::{
    exact_prior_tip, replay_committed_document, replay_committed_segment, replay_document,
    scan_retained_pages, validate_wal_prefix, WalCommittedCursor, WalCommittedRecordStep,
    WalCommittedStep, WalCommittedTransaction, WalTurn, WalValidatedPrefix, WAL_REPLAY_STEP_FUEL,
    WAL_REPLAY_STEP_STALL_BOUND, WAL_TURN,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️retained/🦀️.rs"]
mod retained_tests;
//#endregion 🧪️Tests
