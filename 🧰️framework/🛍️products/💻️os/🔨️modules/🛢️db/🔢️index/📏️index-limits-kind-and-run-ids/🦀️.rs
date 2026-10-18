//! 📏️ Index limits, kind and run ids: the key, value and run limits, the index kind and the run id layout with its capacity and field accessors. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🛡️ Ceiling on one entry's key, validated via `check_len` before the key's bytes
/// are read off storage (decode side) or written into a run (encode side).
pub(super) const MAX_KEY_LEN: u64 = 64 * 1024;

/// 🛡️ Ceiling on one entry's value — generous enough for a serialized `Frontier`/postings
/// list/location pointer, small enough to refuse an obviously-corrupt on-disk length before
/// allocating it.
pub(super) const MAX_VALUE_LEN: u64 = 16 * 1024 * 1024;

/// 🛡️ Ceiling on the number of entries a single run may hold, checked against the header's
/// `entry_count` field before admitting decoded fixed entry slots.
pub(super) const MAX_RUN_ENTRIES: u64 = 64;

/// 🧺️ The entries one run holds at most — what an owner batching its own entries fills.
pub const RUN_ENTRIES_MAX: usize = MAX_RUN_ENTRIES as usize;

/// 🪜️ The highest level an append-only owner's runs fold to ([`IndexHandle::append_owned_sorted_run`]): the newest
/// [`LEVEL_FOLD_RUNS`] full runs of level `L` fold into one run of level `L + 1`, so a level-`L` run holds up to
/// `MAX_RUN_ENTRIES · 4^L` entries and a kind of `N` entries spans about `3 · RUN_LEVEL_MAX + N / (MAX_RUN_ENTRIES · 4^RUN_LEVEL_MAX)` runs.
pub(super) const RUN_LEVEL_MAX: u32 = 3;

/// 🪜️ How many full runs of one level fold into one run of the next.
pub(super) const LEVEL_FOLD_RUNS: usize = 4;

/// 🛡️ Ceiling on the entries of any run a reader admits: a level-[`RUN_LEVEL_MAX`] run's.
pub(super) const LEVEL_RUN_ENTRIES_MAX: u64 = MAX_RUN_ENTRIES << (2 * RUN_LEVEL_MAX);

/// 🛡️ Ceiling on the bytes a fold reads (its input runs together): one read operation's credit, so the folded run —
/// never larger than its inputs — is always read in one operation; a fold that would exceed it is skipped and its runs
/// stay as they are.
pub(super) const LEVEL_FOLD_BYTES_MAX: usize = db_io_core::DB_IO_MAX_READ_BYTES as usize;
//#endregion 🔖️Limits

//#region 🔖️IndexKind
/// 🗂️ The ten index namespaces `db_artifact`/`db_conflict`/`db_projection`/`db_query` build
/// on top of this crate's sorted-run engine (per the contract's per-crate responsibility line for
/// `db_index`). Every kind shares the same generic `IndexHandle` mechanism (`put`/`get`/`delete`/
/// `scan_prefix`/`compact`/`stats` all work identically for any kind); the typed wrappers below
/// (`CommandIndex`, `ActorSeqIndex`, `FrontierIndex`, `TouchedRegionIndex`, `InverseIndex`,
/// `CommitIndex`, `ConflictIndex`, `ProjectionIndex`, `FullTextIndex`, `PreviewIndex`) give every
/// kind a key/value codec on top. Each typed wrapper's value shape is deliberately opaque bytes
/// (`db_conflict::ConflictRecord`, projection state, a preview payload, …) supplied by the caller —
/// this crate never depends on the crates that own those shapes (see the module doc's design note).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IndexKind {
    Command,
    ActorSeq,
    Frontier,
    TouchedRegion,
    Inverse,
    Commit,
    Conflict,
    Projection,
    FullText,
    Preview,
}

impl IndexKind {
    /// 📋️ Every kind, for tests and for callers that want to enumerate/verify a document's
    /// whole index (e.g. `db_cli verify`).
    pub const ALL: [IndexKind; 10] = [IndexKind::Command, IndexKind::ActorSeq, IndexKind::Frontier, IndexKind::TouchedRegion, IndexKind::Inverse, IndexKind::Commit, IndexKind::Conflict, IndexKind::Projection, IndexKind::FullText, IndexKind::Preview];

    /// 🏷️ The one-byte tag stamped in every run's header and packed into the high byte of
    /// its `run_id`s (see `make_run_id`) — this crate's own on-disk representation, not part of the
    /// frozen contract.
    pub(super) fn tag(self) -> u8 {
        match self {
            IndexKind::Command => 1,
            IndexKind::ActorSeq => 2,
            IndexKind::Frontier => 3,
            IndexKind::TouchedRegion => 4,
            IndexKind::Inverse => 5,
            IndexKind::Commit => 6,
            IndexKind::Conflict => 7,
            IndexKind::Projection => 8,
            IndexKind::FullText => 9,
            IndexKind::Preview => 10,
        }
    }
}

/// 🔢️ A `run_id`'s layout, high to low: `[format:4][kind:4][sequence:48][level:2][entries-1:6]`.
/// `db_storage::IndexStorage` addresses runs by a single flat `u64` per document; the high byte
/// namespaces the runs of one kind (and this crate's run-id format) so ten kinds share one document's
/// storage without colliding, and the low bits carry the run's fold level and, for a level-0 run, its
/// entry count, so a listing alone tells the merge policy every run's size — no run is read just to
/// learn how many entries it holds. A folded run keeps its oldest input's sequence and sorts right after
/// it, so ascending ids stay oldest-first.
const RUN_ID_FORMAT: u64 = 2;

const RUN_NAMESPACE_SHIFT: u32 = 56;

const RUN_ENTRY_BITS: u32 = 6;

const RUN_ENTRY_MASK: u64 = (1u64 << RUN_ENTRY_BITS) - 1;

const RUN_LEVEL_BITS: u32 = 2;

const RUN_LEVEL_MASK: u64 = (1u64 << RUN_LEVEL_BITS) - 1;

const RUN_SEQUENCE_SHIFT: u32 = RUN_ENTRY_BITS + RUN_LEVEL_BITS;

const SEQUENCE_BITS: u32 = RUN_NAMESPACE_SHIFT - RUN_SEQUENCE_SHIFT;

pub(super) const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;

const _: () = assert!(MAX_RUN_ENTRIES == 1 << RUN_ENTRY_BITS);

const _: () = assert!(RUN_LEVEL_MAX as u64 <= RUN_LEVEL_MASK);

/// 🏷️ The high byte every run of `kind` carries in its id.
pub(super) fn run_namespace(kind: IndexKind) -> u8 {
    ((RUN_ID_FORMAT << 4) | u64::from(kind.tag())) as u8
}

/// 🧮️ Packs `kind`, `sequence`, the run's fold `level` and its `entries` count into one `run_id`. Errors
/// `LimitExceeded` if `sequence` doesn't fit its 48 bits (2^48 runs of one kind for one document), `level` exceeds
/// [`RUN_LEVEL_MAX`], or `entries` is outside `1..=` the level's capacity (an empty run is never written).
pub(super) fn make_run_id(kind: IndexKind, sequence: u64, level: u32, entries: usize) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 48-bit per-kind namespace"));
    }
    if level > RUN_LEVEL_MAX {
        return Err(DbError::LimitExceeded("db_index run level"));
    }
    if entries == 0 || entries as u64 > level_capacity(level) {
        return Err(DbError::LimitExceeded("db_index run entry count"));
    }
    let count = if level == 0 { entries as u64 - 1 } else { RUN_ENTRY_MASK };
    Ok((u64::from(run_namespace(kind)) << RUN_NAMESPACE_SHIFT) | (sequence << RUN_SEQUENCE_SHIFT) | (u64::from(level) << RUN_ENTRY_BITS) | count)
}

/// 📐️ The entries a run of `level` holds at most.
pub(super) fn level_capacity(level: u32) -> u64 {
    MAX_RUN_ENTRIES << (2 * level)
}

pub(super) fn namespace_of_run_id(run_id: u64) -> u8 {
    (run_id >> RUN_NAMESPACE_SHIFT) as u8
}

pub(super) fn sequence_of_run_id(run_id: u64) -> u64 {
    (run_id >> RUN_SEQUENCE_SHIFT) & SEQUENCE_MASK
}

pub(super) fn level_of_run_id(run_id: u64) -> u32 {
    ((run_id >> RUN_ENTRY_BITS) & RUN_LEVEL_MASK) as u32
}

/// 📏️ A level-0 run's exact entry count; a folded run's capacity (its level's).
pub(super) fn entries_of_run_id(run_id: u64) -> u64 {
    match level_of_run_id(run_id) {
        0 => (run_id & RUN_ENTRY_MASK) + 1,
        level => level_capacity(level),
    }
}
//#endregion 🔖️IndexKind
