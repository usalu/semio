//! 🗄️ 📸️ `db_snapshot` — pack-file-based document snapshots for the `db` crate family:
//! immutable state pages are written into `KIND_CHUNK` segments, a `SnapshotDescriptor`
//! (frontier, protocol version, base pack hash, root list) is written into the
//! reserved `KIND_SNAPSHOT` (`0x07`) pack segment kind — this crate is the first real consumer
//! of that segment kind — and incremental generations chain via `pack::Footer.prev_footer_offset`
//! plus the `REQUIRED_FOOTER_CHAIN` flag, also previously reserved-unused. Built strictly against
//! the frozen contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_snapshot` row).
//!
//! 🎯️ Design choice — how `Footer.prev_footer_offset` is actually used: `db_storage::SnapshotStorage`
//! stores each generation as an *independent*, self-contained `.spk` blob (own header at local
//! offset 0, own footer, individually `delete_generation`-able without corrupting siblings — see
//! that trait's own doc). So `prev_footer_offset`/`REQUIRED_FOOTER_CHAIN` cannot literally address
//! "the other stored blob" — instead this crate defines their meaning precisely: if every
//! generation from the chain's root up to and including this one were concatenated, in
//! generation order, into one retained page authority (`SnapshotChainCursor::materialize_pages`), then
//! `prev_footer_offset` is the absolute offset *within that concatenation* of the parent
//! generation's own footer (`parent_base + parent_len - FOOTER_SIZE`), and `REQUIRED_FOOTER_CHAIN`
//! marks that the field is meaningful at all (distinguishing a legitimate `prev_footer_offset == 0`
//! — a parent that starts at the very first byte, i.e. the chain's root — from "no parent").
//! Reading only ever needs `pack`'s public API (`read_footer_only`, `open_manifest`, `PackSource`)
//! plus a small local length-bounded `PackSource` view (`SubSource`, below) — no `pack_format`
//! private internals are reimplemented.
//!
//! 🎯️ Design choice — locating the `KIND_SNAPSHOT` descriptor segment: `pack::Manifest` has no
//! dedicated span field for it (unsurprising — it's a newly-real segment kind as of this crate).
//! This crate always writes it as the very first segment right after the 32-byte header (before
//! any `write_chunk` call), so it lives at a fixed, predictable local offset
//! (`pack::HEADER_SIZE`) in every generation's own coordinate space, and `decode_snapshot_segment`
//! reads it directly from there using only `pack_core`-level primitives re-exported by the `pack`
//! facade (`crc32c`, `read_varint_u64`, `PackSource`) — the segment is always written with the
//! identity codec, so this reader only needs to understand the uncompressed framing.
//!
//! 🎯️ Scope boundary: retention (`SnapshotManager::retain_from`) only allows pruning generations
//! *below* a full-baseline floor (a generation with no parent) — rolling an incremental chain up
//! into a fresh full baseline first is `db_compact`'s "snapshot consolidation" responsibility
//! (see the contract's `db_compact` row), not this crate's. Similarly, coordinating retention with
//! `db_storage::LeaseStorage` so an in-flight replica read isn't pruned out from under it belongs
//! to `db_compact`'s "online compaction with manifest CAS + fencing" — `retain_from` here is the
//! safe, mechanical pruning primitive that caller is expected to call after fencing itself.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::EpochFence;
use db_durability::Frontier;
use db_ids::{ArtifactId, DbError, check_len};
use db_ids::GenerationId;
use pack::ContentHash;
use db_state::Page;
use db_io_core::{LeaseInfo, LeaseStorage, SnapshotStorage};

#[path = "📇️snapshot-descriptor/🦀️.rs"]
mod snapshot_descriptor;
use snapshot_descriptor::{
    snapshot_field_len, snapshot_segment_write, snapshot_segment_write_field, snapshot_segment_write_varint, snapshot_varint_len, DESCRIPTOR_FORMAT_VERSION, MAX_HASH_LIST_LEN,
    MAX_STRING_BYTES,
};
pub use snapshot_descriptor::SnapshotDescriptor;

#[path = "🧵️snapshot-segment-io/🦀️.rs"]
mod snapshot_segment_io;
#[cfg(any(test,feature="testkit"))]
pub use snapshot_segment_io::SubSource;
use snapshot_segment_io::{decode_snapshot_descriptor, PageSubSource};

#[path = "📚️snapshot-generation-pages/🦀️.rs"]
mod snapshot_generation_pages;
use snapshot_generation_pages::{
    build_generation_page_source, build_generation_retained_expected, open_latest_pages, OptionalSnapshotPages, SnapshotPageSink, SnapshotPageSource, SnapshotPublicationClaim,
};
pub use snapshot_generation_pages::{build_generation_pages, GenerationHandle};
#[cfg(test)]
use snapshot_generation_pages::build_generation;
#[cfg(any(test,feature="testkit"))]
pub use snapshot_generation_pages::{open_generation_at, open_latest};
#[cfg(test)]
pub use snapshot_generation_pages::{open_ancestor, read_page};

#[path = "🗂️snapshot-manager-and-lease/🦀️.rs"]
mod snapshot_manager_and_lease;
pub use snapshot_manager_and_lease::{
    SnapshotBody, SnapshotChainCursor, SnapshotCursorControl, SnapshotLease, SnapshotManager, SnapshotOrigin, SnapshotPolicy, SnapshotRetainedPublication,
    SnapshotRetainedPublicationRejected,
};

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
