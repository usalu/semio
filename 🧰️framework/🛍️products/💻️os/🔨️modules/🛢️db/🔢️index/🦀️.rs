//! 🗄️ `db_index` — the `db` family's secondary-index engine: immutable sorted runs merged
//! LSM-lite (append a new sorted+checksummed run per write batch, fold old runs together as they
//! accumulate) underneath typed per-kind index builders for all ten kinds (command, actor-seq,
//! frontier, touched-region, inverse, commit, conflict, projection, full-text, preview — see
//! `IndexKind`'s doc). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`) and Part 2 of the approved plan.
//!
//! 🎯️ Design choice: this crate has no opinion on what a key/value byte string *means* — that's
//! `db_artifact`'s job (it decides what to index and when). This crate only guarantees the LSM-lite
//! law: for a fixed `(document, kind)`, `get`/`scan_prefix` always resolve to the value written by
//! the most recent `put`/`delete`, regardless of how many runs that history is currently spread
//! across, and `compact`/the automatic merge policy never change what a reader observes — only how
//! many runs it's spread across (checksums via `pack::crc32c` catch on-disk corruption either
//! way). `db_storage::IndexStorage` stores opaque per-`(document, run_id)` byte blobs; this crate
//! owns everything about what's inside a run and how `run_id`s are namespaced per `IndexKind`.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::Frontier;
use db_ids::{ActorId, ArtifactId, DbError, check_len};
use db_io_core::IndexStorage;
#[cfg(test)]
use pack::crc32c;
use pack::ByteWriter;

#[path = "📏️index-limits-kind-and-run-ids/🦀️.rs"]
mod index_limits_kind_and_run_ids;
use index_limits_kind_and_run_ids::{
    entries_of_run_id, level_capacity, level_of_run_id, make_run_id, namespace_of_run_id, run_namespace, sequence_of_run_id, LEVEL_FOLD_BYTES_MAX, LEVEL_FOLD_RUNS,
    LEVEL_RUN_ENTRIES_MAX, MAX_KEY_LEN, MAX_RUN_ENTRIES, MAX_VALUE_LEN, RUN_LEVEL_MAX,
};
pub use index_limits_kind_and_run_ids::{IndexKind, RUN_ENTRIES_MAX};
#[cfg(test)]
use index_limits_kind_and_run_ids::SEQUENCE_MASK;

#[path = "🧭️index-cursor-control-and-bytes/🦀️.rs"]
mod index_cursor_control_and_bytes;
pub use index_cursor_control_and_bytes::{IndexBytes, IndexBytesRejected, IndexCursorControl};
use index_cursor_control_and_bytes::{admit_generated_index_bytes, close_index_bytes, index_bytes_cmp};

#[path = "📼️run-entries-and-codec/🦀️.rs"]
mod run_entries_and_codec;
pub use run_entries_and_codec::{RunEntries, RunEntry, RunValue};
use run_entries_and_codec::{
    encode_run_pages, encode_sorted_run_pages, encode_varint, index_bytes_from_reader, index_bytes_from_slice_for_operation, read_run_header, run_write, run_write_trailer,
    varint_len, RunPageReader, RUN_MAGIC, RUN_VERSION,
};
#[cfg(test)]
use run_entries_and_codec::decode_run_pages;

#[path = "🔭️run-view-and-merge/🦀️.rs"]
mod run_view_and_merge;
use run_view_and_merge::{close_run_entry, close_run_pages, encode_run_from_views, merge_run_views, view_run_pages, RunPick, RunRange, RunView};
#[cfg(test)]
use run_view_and_merge::merge_run_entries;
pub use run_view_and_merge::{IndexStats, MergePolicy};

#[path = "🗂️index-handle/🦀️.rs"]
mod index_handle;
pub use index_handle::{IndexHandle, OwnedRuns};

#[path = "🧷️location-command-and-inverse-indexes/🦀️.rs"]
mod location_command_and_inverse_indexes;
pub use location_command_and_inverse_indexes::{CommandIndex, InverseIndex, RecordLocation};
#[cfg(test)]
use location_command_and_inverse_indexes::decode_location;
use location_command_and_inverse_indexes::decode_index_bytes;

#[path = "👤️actor-frontier-touched-and-commit-indexes/🦀️.rs"]
mod actor_frontier_touched_and_commit_indexes;
pub use actor_frontier_touched_and_commit_indexes::{ActorSeqIndex, CommitIndex, FrontierIndex, TouchedRegionIndex};
use actor_frontier_touched_and_commit_indexes::{decode_postings, encode_postings, validate_actor_key_safe};

#[path = "🔎️text-blob-conflict-projection-and-preview-indexes/🦀️.rs"]
mod text_blob_conflict_projection_and_preview_indexes;
pub use text_blob_conflict_projection_and_preview_indexes::{ConflictIndex, FullTextIndex, IndexBlobList, PreviewIndex, ProjectionIndex};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🧪️RetainedTests
#[cfg(test)]
#[path = "🧪️tests/🔬️retained/🦀️.rs"]
mod retained_tests;
//#endregion 🧪️RetainedTests
