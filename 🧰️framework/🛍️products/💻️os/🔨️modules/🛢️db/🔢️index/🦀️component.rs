//! 🗄️ `db_index` — the `db` family's secondary-index engine: immutable sorted runs merged
//! LSM-lite (append a new sorted+checksummed run per write batch, fold old runs together as they
//! accumulate) underneath typed per-kind index builders for all ten kinds (command, actor-seq,
//! frontier, touched-region, inverse, commit, conflict, projection, full-text, preview — see
//! `IndexKind`'s doc). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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

use crate::db_durability::Frontier;
use crate::db_ids::{check_len, ActorId, ArtifactId, DbError};
use crate::*;
use db_storage::IndexStorage;
use pack::{crc32c, ByteReader, ByteWriter};

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on one entry's key, validated via `check_len` before the key's bytes
/// are read off storage (decode side) or written into a run (encode side).
const MAX_KEY_LEN: u64 = 64 * 1024;

/// @emoji 🛡️ Ceiling on one entry's value — generous enough for a serialized `Frontier`/postings
/// list/location pointer, small enough to refuse an obviously-corrupt on-disk length before
/// allocating it.
const MAX_VALUE_LEN: u64 = 16 * 1024 * 1024;

/// @emoji 🛡️ Ceiling on the number of entries a single run may hold, checked against the header's
/// `entry_count` field before allocating the decoded `Vec<RunEntry>`.
const MAX_RUN_ENTRIES: u64 = 1_000_000;
//#endregion 🔖️Limits

//#region 🔖️IndexKind
/// @emoji 🗂️ The ten index namespaces `db_artifact`/`db_conflict`/`db_projection`/`db_query` build
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
    /// @emoji 📋️ Every kind, for tests and for callers that want to enumerate/verify a document's
    /// whole index (e.g. `db_cli verify`).
    pub const ALL: [IndexKind; 10] = [IndexKind::Command, IndexKind::ActorSeq, IndexKind::Frontier, IndexKind::TouchedRegion, IndexKind::Inverse, IndexKind::Commit, IndexKind::Conflict, IndexKind::Projection, IndexKind::FullText, IndexKind::Preview];

    /// @emoji 🏷️ The one-byte tag stamped in every run's header and packed into the high byte of
    /// its `run_id`s (see `make_run_id`) — this crate's own on-disk representation, not part of the
    /// frozen contract.
    fn tag(self) -> u8 {
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

/// @emoji 🔢️ How many low bits of a `run_id` are the within-kind sequence — the remaining high
/// bits are `IndexKind::tag()`. `db_storage::IndexStorage` addresses runs by a single flat `u64`
/// per document; this crate carves that space into one namespace per kind so ten kinds can share
/// one document's `IndexStorage` without colliding.
const SEQUENCE_BITS: u32 = 56;
const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;

/// @emoji 🧮️ Packs `kind` and `sequence` into one `run_id`. Errors `LimitExceeded` if `sequence`
/// doesn't fit the 56-bit namespace (never happens in practice — that's 2^56 runs of one kind for
/// one document before overflow, and the merge policy keeps live run counts tiny).
fn make_run_id(kind: IndexKind, sequence: u64) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 56-bit per-kind namespace"));
    }
    Ok(((kind.tag() as u64) << SEQUENCE_BITS) | sequence)
}

fn kind_tag_of_run_id(run_id: u64) -> u8 {
    (run_id >> SEQUENCE_BITS) as u8
}

fn sequence_of_run_id(run_id: u64) -> u64 {
    run_id & SEQUENCE_MASK
}
//#endregion 🔖️IndexKind

//#region 🔖️SortedRun
/// @emoji 📇️ One entry's value in a sorted run: either a live payload or a tombstone recording that
/// a key was deleted (and must keep shadowing that key in any older, not-yet-merged run beneath).
#[derive(Clone, PartialEq, Debug)]
pub enum RunValue {
    Put(Vec<u8>),
    Tombstone,
}

/// @emoji 📌️ One `(key, value)` pair inside a sorted run. A well-formed run's entries are strictly
/// ascending and unique by `key` — both `encode_run` (on the way in) and `decode_run` (on the way
/// back out, defending against on-disk corruption) enforce this.
#[derive(Clone, PartialEq, Debug)]
pub struct RunEntry {
    pub key: Vec<u8>,
    pub value: RunValue,
}

/// @emoji 🧱️ Sorts an unordered batch of writes into a well-formed run's entry list: ascending by
/// key, with same-key duplicates collapsed to the LAST one in `entries`' original order (so a
/// caller can hand `put_batch` a batch containing both an old and a newer write for the same key
/// and get the newer one, matching ordinary write-then-overwrite semantics within one batch).
pub fn build_run(entries: Vec<(Vec<u8>, RunValue)>) -> Vec<RunEntry> {
    let mut indexed: Vec<(usize, Vec<u8>, RunValue)> = entries.into_iter().enumerate().map(|(index, (key, value))| (index, key, value)).collect();
    indexed.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut out: Vec<RunEntry> = Vec::with_capacity(indexed.len());
    for (_, key, value) in indexed {
        if let Some(last) = out.last_mut() {
            if last.key == key {
                last.value = value;
                continue;
            }
        }
        out.push(RunEntry { key, value });
    }
    out
}

/// @emoji 🪧️ A run's 6-byte header: 4-byte magic, 1-byte format version, 1-byte `IndexKind` tag —
/// see `read_run_header`.
const RUN_MAGIC: [u8; 4] = *b"DBIR";
const RUN_VERSION: u8 = 1;

/// @emoji 📐️ A run header's parsed fields plus how many bytes of `body` it occupied, so the caller
/// knows where the entry stream starts.
struct RunHeader {
    entry_count: u64,
    header_len: usize,
}

/// @emoji 📖️ Parses `body`'s header (magic/version/kind tag/entry count) WITHOUT touching the entry
/// stream — `decode_run` uses this before allocating the full entry vector; `peek_entry_count`
/// (used by `stats`, which wants counts without paying for a full decode) uses only this.
async fn read_run_header(body: &[u8], expected_kind: IndexKind) -> Result<RunHeader, DbError> {
    if body.len() < RUN_MAGIC.len() + 2 {
        return Err(DbError::Corrupt("index run is shorter than its header".to_string()));
    }
    if body[..RUN_MAGIC.len()] != RUN_MAGIC {
        return Err(DbError::Corrupt("index run has a bad magic".to_string()));
    }
    let version = body[RUN_MAGIC.len()];
    if version != RUN_VERSION {
        return Err(DbError::Corrupt(format!("unsupported index run version {version}")));
    }
    let kind_tag = body[RUN_MAGIC.len() + 1];
    if kind_tag != expected_kind.tag() {
        return Err(DbError::Corrupt(format!("index run kind mismatch: expected {expected_kind:?} (tag {}), found tag {kind_tag}", expected_kind.tag())));
    }
    let mut reader = ByteReader::new(&body[RUN_MAGIC.len() + 2..]).await;
    let entry_count = reader.read_varint_u64().await?;
    check_len(entry_count, MAX_RUN_ENTRIES, "db_index::entries")?;
    Ok(RunHeader { entry_count, header_len: RUN_MAGIC.len() + 2 + reader.position().await })
}

/// @emoji 👀️ Reads just a run's `entry_count` (no checksum verification, no entry decode) — the
/// cheap path `IndexHandle::stats` uses; `IndexHandle::verify`/`get`/`scan_prefix` go through the
/// full, checksum-verifying `decode_run` instead.
async fn peek_entry_count(bytes: &[u8], expected_kind: IndexKind) -> Result<u64, DbError> {
    if bytes.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let body = &bytes[..bytes.len() - 4];
    Ok(read_run_header(body, expected_kind).await?.entry_count)
}

/// @emoji ✍️ Encodes a well-formed (strictly ascending, unique-by-key) entry list into one run's
/// bytes: `MAGIC(4) VERSION(1) KIND(1) entry_count(varint) entries... crc32c(4, LE)`. Each entry is
/// `key_len(varint) key value_tag(1: 0=tombstone,1=put) [value_len(varint) value]`. Errors
/// `InvalidArgument` if `entries` isn't strictly ascending — this fn never silently re-sorts, since
/// a caller with unsorted/duplicate entries should go through `build_run` first.
async fn encode_run(kind: IndexKind, entries: &[RunEntry]) -> Result<Vec<u8>, DbError> {
    check_len(entries.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut writer = ByteWriter::new().await;
    writer.write_bytes(&RUN_MAGIC).await;
    writer.write_u8(RUN_VERSION).await;
    writer.write_u8(kind.tag()).await;
    writer.write_varint_u64(entries.len() as u64).await;
    let mut previous_key: Option<&[u8]> = None;
    for entry in entries {
        if let Some(previous) = previous_key {
            if entry.key.as_slice() <= previous {
                return Err(DbError::InvalidArgument("db_index run entries must be strictly ascending and unique by key".to_string()));
            }
        }
        previous_key = Some(entry.key.as_slice());
        check_len(entry.key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
        writer.write_varint_u64(entry.key.len() as u64).await;
        writer.write_bytes(&entry.key).await;
        match &entry.value {
            RunValue::Tombstone => writer.write_u8(0).await,
            RunValue::Put(value) => {
                check_len(value.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
                writer.write_u8(1).await;
                writer.write_varint_u64(value.len() as u64).await;
                writer.write_bytes(value).await;
            }
        }
    }
    let mut bytes = writer.into_bytes().await;
    let checksum = crc32c(&bytes).await;
    bytes.extend_from_slice(&checksum.to_le_bytes());
    Ok(bytes)
}

/// @emoji 📖️ Inverse of `encode_run`: verifies the trailing crc32c over everything before it FIRST
/// (so a torn/corrupt run is caught before any entry is trusted), then the header, then decodes
/// entries — re-validating strict ascending-and-unique order defensively (corruption could produce
/// an out-of-order file even with a matching checksum on a byte flip that happens to preserve it,
/// vanishingly unlikely but the check is nearly free). `expected_kind` guards against a `run_id`
/// namespace bug silently handing one kind's bytes to another kind's decoder.
async fn decode_run(bytes: &[u8], expected_kind: IndexKind) -> Result<Vec<RunEntry>, DbError> {
    if bytes.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let (body, checksum_bytes) = bytes.split_at(bytes.len() - 4);
    let mut checksum_array = [0u8; 4];
    checksum_array.copy_from_slice(checksum_bytes);
    if crc32c(body).await != u32::from_le_bytes(checksum_array) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let header = read_run_header(body, expected_kind).await?;
    let mut reader = ByteReader::new(&body[header.header_len..]).await;
    let mut entries = Vec::with_capacity(usize::try_from(header.entry_count).unwrap_or(0));
    let mut previous_key: Option<Vec<u8>> = None;
    for _ in 0..header.entry_count {
        let key_len = reader.read_varint_u64().await?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = reader.read_bytes(key_len as usize).await?.to_vec();
        if let Some(previous) = &previous_key {
            if key.as_slice() <= previous.as_slice() {
                return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
            }
        }
        let tag = reader.read_u8().await?;
        let value = match tag {
            0 => RunValue::Tombstone,
            1 => {
                let value_len = reader.read_varint_u64().await?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                RunValue::Put(reader.read_bytes(value_len as usize).await?.to_vec())
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        previous_key = Some(key.clone());
        entries.push(RunEntry { key, value });
    }
    Ok(entries)
}
//#endregion 🔖️SortedRun

//#region 🔖️Merge
/// @emoji 🌀️ The LSM-lite k-way merge: `runs` MUST be ordered oldest-first/newest-last (index 0 is
/// the oldest run, `runs.len() - 1` the newest); on a key collision the newest run's value wins,
/// matching ordinary overwrite semantics. `drop_tombstones` is the caller's choice: `false` for a
/// *partial* merge (some older run might still exist beneath the merged result — a tombstone must
/// keep shadowing it), `true` for a *complete* merge across every run for a kind (nothing older
/// remains, so a tombstone has served its purpose and can be dropped).
///
/// 🎯️ Design choice: a straightforward multi-pointer scan (`O(total_entries * runs.len())`) rather
/// than a binary-heap k-way merge — simple and easy to audit, and appropriate because
/// `MergePolicy`'s automatic compaction keeps the live run count for any one kind small (a binary
/// heap would only pay off with dozens+ of concurrently live runs, which this crate's merge policy
/// never lets happen).
pub fn merge_runs(runs: &[Vec<RunEntry>], drop_tombstones: bool) -> Vec<RunEntry> {
    let mut pointers = vec![0usize; runs.len()];
    let mut out = Vec::new();
    loop {
        let mut best_key: Option<Vec<u8>> = None;
        for (i, run) in runs.iter().enumerate() {
            if let Some(entry) = run.get(pointers[i]) {
                if best_key.as_deref().is_none_or(|current| entry.key.as_slice() < current) {
                    best_key = Some(entry.key.clone());
                }
            }
        }
        let Some(best_key) = best_key else { break };
        let mut chosen = RunValue::Tombstone;
        for (i, run) in runs.iter().enumerate() {
            if run.get(pointers[i]).is_some_and(|entry| entry.key == best_key) {
                chosen = run[pointers[i]].value.clone();
                pointers[i] += 1;
            }
        }
        if !(drop_tombstones && matches!(chosen, RunValue::Tombstone)) {
            out.push(RunEntry { key: best_key, value: chosen });
        }
    }
    out
}
//#endregion 🔖️Merge

//#region 🔖️MergePolicy
/// @emoji ⚖️ When `IndexHandle::put_batch` should automatically fold old runs together. This
/// crate's own choice (the contract fixes the LSM-lite shape, not the trigger threshold): after
/// every write, while a kind's live run count exceeds `max_runs_before_merge`, the two OLDEST runs
/// are merged into one (see `IndexHandle::maybe_auto_merge`) — a bounded, incremental amount of
/// merge work per write rather than a large stop-the-world compaction.
#[derive(Clone, Copy, Debug)]
pub struct MergePolicy {
    pub max_runs_before_merge: usize,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self { max_runs_before_merge: 4 }
    }
}
//#endregion 🔖️MergePolicy

//#region 🔖️Stats
/// @emoji 📊️ A kind's current shape: how many runs it's spread across, how many live entries (each
/// counted once even if shadowed copies exist in older runs — `entry_count` sums each run's raw
/// header count, so a key overwritten `N` times across `N` runs is NOT deduplicated here; `compact`
/// first is the way to get an exact live-key count) and how many bytes on `IndexStorage`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexStats {
    pub run_count: usize,
    pub entry_count: u64,
    pub total_bytes: u64,
}
//#endregion 🔖️Stats

//#region 🔖️Aliases
/// @emoji 🏷️ `scan_prefix`'s result shape, named so its signature reads as one concept rather than
/// tripping `clippy::type_complexity`.
pub type KeyValuePairs = Vec<(Vec<u8>, Vec<u8>)>;
//#endregion 🔖️Aliases

//#region 🔖️IndexHandle
/// @emoji 🔍️ One `(document, kind)`'s view onto its sorted runs — every typed wrapper below
/// (`CommandIndex`, `FrontierIndex`, ...) is a thin codec layered on top of one of these. Never
/// interprets key/value bytes itself; that's the typed layer's job.
pub struct IndexHandle<'a, S: IndexStorage> {
    storage: &'a S,
    document: ArtifactId,
    kind: IndexKind,
    policy: MergePolicy,
}

impl<'a, S: IndexStorage> IndexHandle<'a, S> {
    /// @emoji 🚀️ Opens a handle with the default `MergePolicy`.
    pub async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self::with_policy(storage, document, kind, MergePolicy::default()).await
    }

    /// @emoji 🚀️ Opens a handle with an explicit `MergePolicy` (e.g. a tighter threshold for a
    /// hot, frequently-scanned kind, or a looser one for a write-heavy, rarely-read kind).
    pub async fn with_policy(storage: &'a S, document: ArtifactId, kind: IndexKind, policy: MergePolicy) -> Self {
        Self { storage, document, kind, policy }
    }

    /// @emoji 📋️ This handle's live run ids, ascending by sequence (oldest first) — every other id
    /// belonging to a different kind for the same document is filtered out.
    async fn kind_run_ids(&self) -> Result<Vec<u64>, DbError> {
        Ok(self.storage.list_runs(&self.document).await?.into_iter().filter(|id| kind_tag_of_run_id(*id) == self.kind.tag()).collect())
    }

    /// @emoji ⏭️ The sequence the next `put_batch` should claim: one past the newest live run's
    /// sequence, or `0` if this kind has no runs yet.
    async fn next_sequence(&self) -> Result<u64, DbError> {
        Ok(self.kind_run_ids().await?.last().map_or(0, |id| sequence_of_run_id(*id) + 1))
    }

    async fn load_run(&self, run_id: u64) -> Result<Vec<RunEntry>, DbError> {
        decode_run(&self.storage.read_run(&self.document, run_id).await?, self.kind).await
    }

    /// @emoji ✍️ Durably appends `entries` as one new, newest run (via `build_run` + `encode_run`),
    /// then applies `MergePolicy`. A no-op (no run written) if `entries` is empty.
    pub async fn put_batch(&self, entries: Vec<(Vec<u8>, RunValue)>) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        for (key, value) in &entries {
            check_len(key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
            if let RunValue::Put(value_bytes) = value {
                check_len(value_bytes.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
            }
        }
        let built = build_run(entries);
        let encoded = encode_run(self.kind, &built).await?;
        let run_id = make_run_id(self.kind, self.next_sequence().await?)?;
        self.storage.write_run(&self.document, run_id, &encoded).await?;
        self.maybe_auto_merge().await
    }

    /// @emoji ➕️ Convenience over `put_batch` for a single key.
    pub async fn put(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), DbError> {
        self.put_batch(vec![(key, RunValue::Put(value))]).await
    }

    /// @emoji 🪦️ Convenience over `put_batch` for a single tombstone.
    pub async fn delete(&self, key: &[u8]) -> Result<(), DbError> {
        self.put_batch(vec![(key.to_vec(), RunValue::Tombstone)]).await
    }

    /// @emoji 🔎️ Resolves `key` by scanning runs newest-to-oldest and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DbError> {
        for run_id in self.kind_run_ids().await?.into_iter().rev() {
            let entries = self.load_run(run_id).await?;
            if let Ok(position) = entries.binary_search_by(|entry| entry.key.as_slice().cmp(key)) {
                return Ok(match &entries[position].value {
                    RunValue::Put(value) => Some(value.clone()),
                    RunValue::Tombstone => None,
                });
            }
        }
        Ok(None)
    }

    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`,
    /// ascending by key — merges every run (newest wins on collision, tombstones dropped since this
    /// is a complete view across the whole kind) then filters.
    pub async fn scan_prefix(&self, prefix: &[u8]) -> Result<KeyValuePairs, DbError> {
        let run_ids = self.kind_run_ids().await?;
        let mut runs = Vec::with_capacity(run_ids.len());
        for run_id in run_ids {
            runs.push(self.load_run(run_id).await?);
        }
        let merged = merge_runs(&runs, true);
        let mut out = Vec::new();
        for entry in merged {
            if !entry.key.starts_with(prefix) {
                continue;
            }
            if let RunValue::Put(value) = entry.value {
                out.push((entry.key, value));
            }
        }
        Ok(out)
    }

    /// @emoji 🌀️ `MergePolicy`'s enforcement: while this kind has more live runs than
    /// `policy.max_runs_before_merge`, merges the two oldest into one (written back under the
    /// older's `run_id`, preserving the oldest-first ordering invariant `kind_run_ids` relies on;
    /// the younger's `run_id` is then deleted). Tombstones are preserved (`drop_tombstones: false`)
    /// since runs even older than these two may still exist.
    async fn maybe_auto_merge(&self) -> Result<(), DbError> {
        loop {
            let run_ids = self.kind_run_ids().await?;
            if run_ids.len() <= self.policy.max_runs_before_merge {
                return Ok(());
            }
            let (oldest, second_oldest) = (run_ids[0], run_ids[1]);
            let merged = merge_runs(&[self.load_run(oldest).await?, self.load_run(second_oldest).await?], false);
            self.storage.write_run(&self.document, oldest, &encode_run(self.kind, &merged).await?).await?;
            self.storage.delete_run(&self.document, second_oldest).await?;
        }
    }

    /// @emoji 🧹️ Merges EVERY live run for this kind into exactly one (dropping tombstones, since
    /// nothing older remains beneath a complete merge), written back under the oldest run's
    /// `run_id`. A no-op if already at zero or one runs. Returns the post-compaction `stats()`.
    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let run_ids = self.kind_run_ids().await?;
        if run_ids.len() > 1 {
            let mut runs = Vec::with_capacity(run_ids.len());
            for &run_id in &run_ids {
                runs.push(self.load_run(run_id).await?);
            }
            let merged = merge_runs(&runs, true);
            self.storage.write_run(&self.document, run_ids[0], &encode_run(self.kind, &merged).await?).await?;
            for &run_id in &run_ids[1..] {
                self.storage.delete_run(&self.document, run_id).await?;
            }
        }
        self.stats().await
    }

    /// @emoji 📊️ Current shape of this kind's runs — see `IndexStats`'s doc for what `entry_count`
    /// does and doesn't count. Cheap: reads every run's bytes but only parses each one's header.
    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let run_ids = self.kind_run_ids().await?;
        let mut entry_count = 0u64;
        let mut total_bytes = 0u64;
        for &run_id in &run_ids {
            let bytes = self.storage.read_run(&self.document, run_id).await?;
            total_bytes += bytes.len() as u64;
            entry_count += peek_entry_count(&bytes, self.kind).await?;
        }
        Ok(IndexStats { run_count: run_ids.len(), entry_count, total_bytes })
    }

    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    pub async fn verify(&self) -> Result<(), DbError> {
        for run_id in self.kind_run_ids().await? {
            self.load_run(run_id).await?;
        }
        Ok(())
    }
}
//#endregion 🔖️IndexHandle

//#region 🔖️RecordLocation
/// @emoji 📍️ A pointer into a document's WAL: which segment, what byte offset, how many bytes.
/// `CommandIndex`/`InverseIndex`'s value shape — deliberately NOT the WAL record itself (this crate
/// never depends on `db_wal`/`protocol`; a location is exactly enough for a caller who DOES depend
/// on those to seek and re-read the actual record).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RecordLocation {
    pub segment: u64,
    pub offset: u64,
    pub len: u64,
}

async fn encode_location(location: RecordLocation) -> Vec<u8> {
    let mut writer = ByteWriter::new().await;
    writer.write_varint_u64(location.segment).await;
    writer.write_varint_u64(location.offset).await;
    writer.write_varint_u64(location.len).await;
    writer.into_bytes().await
}

async fn decode_location(bytes: &[u8]) -> Result<RecordLocation, DbError> {
    let mut reader = ByteReader::new(bytes).await;
    let segment = reader.read_varint_u64().await?;
    let offset = reader.read_varint_u64().await?;
    let len = reader.read_varint_u64().await?;
    Ok(RecordLocation { segment, offset, len })
}

/// @emoji 🔢️ `u64 -> RecordLocation`, keyed big-endian so byte order matches numeric order — the
/// shared shape behind both `CommandIndex` (keyed by command seq) and `InverseIndex` (keyed by the
/// same command seq, pointing at its inverse's location instead).
struct SeqLocationIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> SeqLocationIndex<'a, S> {
    async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self { handle: IndexHandle::new(storage, document, kind).await }
    }

    async fn record(&self, seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.handle.put(seq.to_be_bytes().to_vec(), encode_location(location).await).await
    }

    async fn lookup(&self, seq: u64) -> Result<Option<RecordLocation>, DbError> {
        match self.handle.get(&seq.to_be_bytes()).await? {
            Some(bytes) => Ok(Some(decode_location(&bytes).await?)),
            None => Ok(None),
        }
    }

    async fn remove(&self, seq: u64) -> Result<(), DbError> {
        self.handle.delete(&seq.to_be_bytes()).await
    }
}
//#endregion 🔖️RecordLocation

//#region 🔖️CommandIndex
/// @emoji 🗃️ `command_seq -> RecordLocation` — `db_artifact`'s primary lookup for "where in the
/// WAL is command N", the backbone of replay-from-a-point and `Consistency::Exact`/`AtLeast` query
/// resolution.
pub struct CommandIndex<'a, S: IndexStorage>(SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> CommandIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Command).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        self.0.handle.stats().await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        self.0.handle.compact().await
    }
}
//#endregion 🔖️CommandIndex

//#region 🔖️InverseIndex
/// @emoji ↩️ `command_seq -> RecordLocation` of that command's inverse operation payload —
/// `db_artifact`'s undo machinery's lookup.
pub struct InverseIndex<'a, S: IndexStorage>(SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> InverseIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Inverse).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        self.0.handle.stats().await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        self.0.handle.compact().await
    }
}
//#endregion 🔖️InverseIndex

//#region 🔖️ActorSeqIndex
/// @emoji 👤️ `(actor, actor_seq) -> command_seq` — resolves an actor's own local operation sequence
/// number (idempotency / causal-order checks at admission) to the document's global command
/// sequence. Keys are `actor_bytes || 0x00 || actor_seq(8, BE)`; `actor`'s id must not itself
/// contain a NUL byte (validated) so the `0x00` separator stays unambiguous and prefix scans by
/// actor (`latest_for_actor`) can't spill into a neighboring actor's entries.
pub struct ActorSeqIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn validate_actor_key_safe(actor: &ActorId) -> Result<(), DbError> {
    if actor.0.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("actor id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn actor_seq_key(actor: &ActorId, actor_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + 8);
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&actor_seq.to_be_bytes());
    Ok(key)
}

// 🚫️async: E1 pure accessor consumed by sync Option::map/closures — see R9
fn decode_u64_le(bytes: &[u8]) -> Result<u64, DbError> {
    let array: [u8; 8] = bytes.try_into().map_err(|_| DbError::Corrupt("expected an 8-byte little-endian u64 index value".to_string()))?;
    Ok(u64::from_le_bytes(array))
}

impl<'a, S: IndexStorage> ActorSeqIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::ActorSeq).await }
    }

    pub async fn record(&self, actor: &ActorId, actor_seq: u64, command_seq: u64) -> Result<(), DbError> {
        self.handle.put(actor_seq_key(actor, actor_seq).await?, command_seq.to_le_bytes().to_vec()).await
    }

    pub async fn lookup(&self, actor: &ActorId, actor_seq: u64) -> Result<Option<u64>, DbError> {
        self.handle.get(&actor_seq_key(actor, actor_seq).await?).await?.map(|bytes| decode_u64_le(&bytes)).transpose()
    }

    /// @emoji 🥇️ The highest `(actor_seq, command_seq)` pair recorded for `actor`, or `None` if
    /// `actor` has never been recorded.
    pub async fn latest_for_actor(&self, actor: &ActorId) -> Result<Option<(u64, u64)>, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let entries = self.handle.scan_prefix(&prefix).await?;
        entries
            .into_iter()
            .last()
            .map(|(key, value)| {
                let actor_seq_bytes: [u8; 8] = key[prefix.len()..].try_into().map_err(|_| DbError::Corrupt("actor-seq index key has a malformed suffix".to_string()))?;
                Ok((u64::from_be_bytes(actor_seq_bytes), decode_u64_le(&value)?))
            })
            .transpose()
    }
}
//#endregion 🔖️ActorSeqIndex

//#region 🔖️FrontierIndex
/// @emoji 🧭️ `commit_seq -> Frontier` — a per-commit snapshot of `Frontier`, letting
/// `Consistency::Historical`/replica resume resolve "what did the frontier look like at commit N"
/// without replaying.
pub struct FrontierIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_frontier(frontier: &Frontier) -> Vec<u8> {
    let mut writer = ByteWriter::new().await;
    let document_bytes = frontier.document.0.as_bytes();
    writer.write_varint_u64(document_bytes.len() as u64).await;
    writer.write_bytes(document_bytes).await;
    writer.write_varint_u64(frontier.head_seq).await;
    writer.write_varint_u64(frontier.commit_seq).await;
    writer.write_bytes(&frontier.chain_hash).await;
    writer.write_varint_u64(frontier.epoch).await;
    writer.into_bytes().await
}

async fn decode_frontier(bytes: &[u8]) -> Result<Frontier, DbError> {
    let mut reader = ByteReader::new(bytes).await;
    let document_len = reader.read_varint_u64().await?;
    check_len(document_len, MAX_KEY_LEN, "db_index::frontier_document")?;
    let document_bytes = reader.read_bytes(document_len as usize).await?.to_vec();
    let document = ArtifactId(String::from_utf8(document_bytes).map_err(|_| DbError::Corrupt("frontier document id is not valid utf-8".to_string()))?);
    let head_seq = reader.read_varint_u64().await?;
    let commit_seq = reader.read_varint_u64().await?;
    let chain_hash = reader.read_array32().await?;
    let epoch = reader.read_varint_u64().await?;
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

impl<'a, S: IndexStorage> FrontierIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Frontier).await }
    }

    pub async fn record(&self, frontier: &Frontier) -> Result<(), DbError> {
        self.handle.put(frontier.commit_seq.to_be_bytes().to_vec(), encode_frontier(frontier).await).await
    }

    pub async fn lookup(&self, commit_seq: u64) -> Result<Option<Frontier>, DbError> {
        match self.handle.get(&commit_seq.to_be_bytes()).await? {
            Some(bytes) => Ok(Some(decode_frontier(&bytes).await?)),
            None => Ok(None),
        }
    }

    /// @emoji 🥇️ The frontier recorded under the highest `commit_seq`, or `None` if none recorded.
    pub async fn latest(&self) -> Result<Option<Frontier>, DbError> {
        match self.handle.scan_prefix(&[]).await?.into_iter().last() {
            Some((_, value)) => Ok(Some(decode_frontier(&value).await?)),
            None => Ok(None),
        }
    }
}
//#endregion 🔖️FrontierIndex

//#region 🔖️TouchedRegionIndex
/// @emoji 🎯️ `region -> [command_seq]` (ascending, deduplicated) — `db_conflict`'s reverse index:
/// given a region a new command is about to touch, which prior commands also touched it (the
/// candidate set for touched-region-intersection conflict checks).
pub struct TouchedRegionIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_postings(postings: &[u64]) -> Vec<u8> {
    let mut writer = ByteWriter::new().await;
    writer.write_varint_u64(postings.len() as u64).await;
    for posting in postings {
        writer.write_varint_u64(*posting).await;
    }
    writer.into_bytes().await
}

async fn decode_postings(bytes: &[u8]) -> Result<Vec<u64>, DbError> {
    let mut reader = ByteReader::new(bytes).await;
    let count = reader.read_varint_u64().await?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::postings")?;
    let mut postings = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        postings.push(reader.read_varint_u64().await?);
    }
    Ok(postings)
}

impl<'a, S: IndexStorage> TouchedRegionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::TouchedRegion).await }
    }

    /// @emoji ➕️ Records that `command_seq` touched `region` — read-modify-write over the region's
    /// current posting list, kept sorted and deduplicated.
    pub async fn record_touch(&self, region: &[u8], command_seq: u64) -> Result<(), DbError> {
        let mut postings = self.touching(region).await?;
        if let Err(position) = postings.binary_search(&command_seq) {
            postings.insert(position, command_seq);
        }
        self.handle.put(region.to_vec(), encode_postings(&postings).await).await
    }

    pub async fn touching(&self, region: &[u8]) -> Result<Vec<u64>, DbError> {
        match self.handle.get(region).await? {
            Some(bytes) => decode_postings(&bytes).await,
            None => Ok(Vec::new()),
        }
    }
}
//#endregion 🔖️TouchedRegionIndex

//#region 🔖️CommitIndex
/// @emoji 🏁️ `commit_id -> command_seq` — resolves a VCS-facing commit id (`vcs::Checkpoint.id`,
/// per the contract's content-addressed `ck-<hex16>` scheme) to the command sequence it was cut at,
/// for `Consistency::Historical(commit_id)` query resolution.
pub struct CommitIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> CommitIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Commit).await }
    }

    pub async fn record(&self, commit_id: &str, command_seq: u64) -> Result<(), DbError> {
        self.handle.put(commit_id.as_bytes().to_vec(), command_seq.to_le_bytes().to_vec()).await
    }

    pub async fn lookup(&self, commit_id: &str) -> Result<Option<u64>, DbError> {
        self.handle.get(commit_id.as_bytes()).await?.map(|bytes| decode_u64_le(&bytes)).transpose()
    }
}
//#endregion 🔖️CommitIndex

//#region 🔖️FullTextIndex
/// @emoji 🔤️ `term -> [doc_ref]` — a minimal inverted index: `index_document` tokenizes text into
/// lowercase alphanumeric-run terms and records `doc_ref` (an opaque caller-chosen id, typically a
/// field/command location) against each; `search` resolves one term to its posting list. No
/// ranking/stemming/stopwords — `db_query`'s full-text query planner is expected to layer that on
/// top of this crate's exact-term postings.
pub struct FullTextIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn tokenize(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric()).filter(|term| !term.is_empty()).map(str::to_lowercase).collect()
}

impl<'a, S: IndexStorage> FullTextIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::FullText).await }
    }

    async fn postings(&self, term_key: &[u8]) -> Result<Vec<u64>, DbError> {
        match self.handle.get(term_key).await? {
            Some(bytes) => decode_postings(&bytes).await,
            None => Ok(Vec::new()),
        }
    }

    /// @emoji ➕️ Tokenizes `text` and records `doc_ref` against every distinct term it contains.
    pub async fn index_document(&self, doc_ref: u64, text: &str) -> Result<(), DbError> {
        let mut terms = tokenize(text).await;
        terms.sort();
        terms.dedup();
        for term in terms {
            let mut postings = self.postings(term.as_bytes()).await?;
            if let Err(position) = postings.binary_search(&doc_ref) {
                postings.insert(position, doc_ref);
            }
            self.handle.put(term.into_bytes(), encode_postings(&postings).await).await?;
        }
        Ok(())
    }

    /// @emoji 🔎️ The posting list for `term` (case-folded to match `index_document`'s tokenizer),
    /// or an empty list if the term has never been indexed.
    pub async fn search(&self, term: &str) -> Result<Vec<u64>, DbError> {
        self.postings(term.to_lowercase().as_bytes()).await
    }
}
//#endregion 🔖️FullTextIndex

//#region 🔖️BlobList
/// @emoji 📦️ Encodes a list of opaque byte blobs (`ConflictIndex`'s per-command conflict records)
/// as `count(varint) [len(varint) bytes]...` — the same read-modify-write accumulation shape
/// `TouchedRegionIndex`/`FullTextIndex` use for their posting lists, generalized to arbitrary-size
/// values instead of `u64` postings.
async fn encode_blob_list(blobs: &[Vec<u8>]) -> Vec<u8> {
    let mut writer = ByteWriter::new().await;
    writer.write_varint_u64(blobs.len() as u64).await;
    for blob in blobs {
        writer.write_varint_u64(blob.len() as u64).await;
        writer.write_bytes(blob).await;
    }
    writer.into_bytes().await
}

async fn decode_blob_list(bytes: &[u8]) -> Result<Vec<Vec<u8>>, DbError> {
    let mut reader = ByteReader::new(bytes).await;
    let count = reader.read_varint_u64().await?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::blob_list")?;
    let mut blobs = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        let len = reader.read_varint_u64().await?;
        check_len(len, MAX_VALUE_LEN, "db_index::blob_list_entry")?;
        blobs.push(reader.read_bytes(len as usize).await?.to_vec());
    }
    Ok(blobs)
}
//#endregion 🔖️BlobList

//#region 🔖️ConflictIndex
/// @emoji ⚔️ `command_seq -> [ConflictRecord bytes]` — a command may surface more than one
/// conflict (touched-region collision, constraint violation, …), so this accumulates a list per
/// `command_seq` the same way `TouchedRegionIndex` accumulates a posting list: read the current
/// list, append, write back. Record shapes are `db_conflict`'s concern; this index only stores and
/// returns the opaque bytes it's handed.
pub struct ConflictIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> ConflictIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Conflict).await }
    }

    /// @emoji ➕️ Appends `record` to `command_seq`'s conflict list.
    pub async fn record_conflict(&self, command_seq: u64, record: Vec<u8>) -> Result<(), DbError> {
        check_len(record.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
        let mut records = self.conflicts_for(command_seq).await?;
        records.push(record);
        self.handle.put(command_seq.to_be_bytes().to_vec(), encode_blob_list(&records).await).await
    }

    /// @emoji 📋️ Every conflict record recorded for `command_seq`, in the order they were
    /// recorded, or empty if none.
    pub async fn conflicts_for(&self, command_seq: u64) -> Result<Vec<Vec<u8>>, DbError> {
        match self.handle.get(&command_seq.to_be_bytes()).await? {
            Some(bytes) => decode_blob_list(&bytes).await,
            None => Ok(Vec::new()),
        }
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        self.handle.stats().await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        self.handle.compact().await
    }
}
//#endregion 🔖️ConflictIndex

//#region 🔖️ProjectionIndex
/// @emoji 📽️ `(projection_id, frontier_seq) -> opaque projection state bytes`, floor-queryable per
/// projection id — `db_projection`'s "this projection's state as of at or before frontier X"
/// lookup. Keys are `projection_id_bytes || 0x00 || frontier_seq(8, BE)`, the same NUL-separated
/// composite shape `ActorSeqIndex` uses (`projection_id` must not itself contain a NUL byte,
/// validated) so a prefix scan by projection id can't spill into a lexicographically-neighboring
/// projection's entries.
pub struct ProjectionIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn validate_projection_id_key_safe(projection_id: &str) -> Result<(), DbError> {
    if projection_id.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("projection id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn projection_key(projection_id: &str, frontier_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_projection_id_key_safe(projection_id).await?;
    let mut key = Vec::with_capacity(projection_id.len() + 1 + 8);
    key.extend_from_slice(projection_id.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&frontier_seq.to_be_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> ProjectionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Projection).await }
    }

    pub async fn record(&self, projection_id: &str, frontier_seq: u64, state: Vec<u8>) -> Result<(), DbError> {
        self.handle.put(projection_key(projection_id, frontier_seq).await?, state).await
    }

    /// @emoji 🎯️ The exact state recorded for `projection_id` at `frontier_seq`, or `None` if
    /// nothing was recorded at that exact sequence.
    pub async fn at(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<Vec<u8>>, DbError> {
        self.handle.get(&projection_key(projection_id, frontier_seq).await?).await
    }

    /// @emoji 🏔️ The state recorded at the greatest `frontier_seq' <= frontier_seq` for
    /// `projection_id` specifically — scoped to `projection_id`'s own key range (via the NUL
    /// separator) before scanning, so a projection with no entry at or before `frontier_seq` never
    /// wrongly surfaces a different, lexicographically-earlier projection's entry.
    pub async fn latest_at_or_before(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<(u64, Vec<u8>)>, DbError> {
        validate_projection_id_key_safe(projection_id).await?;
        let mut prefix = projection_id.as_bytes().to_vec();
        prefix.push(0u8);
        let entries = self.handle.scan_prefix(&prefix).await?;
        let mut result = None;
        for (key, value) in entries {
            let seq_bytes: [u8; 8] = key[prefix.len()..].try_into().map_err(|_| DbError::Corrupt("projection index key has a malformed suffix".to_string()))?;
            let seq = u64::from_be_bytes(seq_bytes);
            if seq > frontier_seq {
                break; // entries are ascending by key, i.e. ascending by seq within this prefix
            }
            result = Some((seq, value));
        }
        Ok(result)
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        self.handle.stats().await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        self.handle.compact().await
    }
}
//#endregion 🔖️ProjectionIndex

//#region 🔖️PreviewIndex
/// @emoji 🌫️ `(actor, preview_key) -> opaque latest preview bytes` — `publish`/`withdraw` are
/// plain `put`/`delete`, so `latest` naturally coalesces to the most recently published-or-
/// withdrawn value per `(actor, preview_key)`, matching the contract's "coalescing
/// latest-per-(actor,key)" preview law. Keys are `actor_bytes || 0x00 || preview_key_bytes`
/// (`actor`'s id must not contain a NUL byte, validated; `preview_key` is the final component so
/// it needs no such restriction). Never durable per that same law — `db_preview` is responsible for
/// never routing this index's writes through a durable `DurabilityClass`.
pub struct PreviewIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_preview_key(actor: &ActorId, preview_key: &str) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + preview_key.len());
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(preview_key.as_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> PreviewIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Preview).await }
    }

    pub async fn publish(&self, actor: &ActorId, preview_key: &str, value: Vec<u8>) -> Result<(), DbError> {
        self.handle.put(encode_preview_key(actor, preview_key).await?, value).await
    }

    pub async fn withdraw(&self, actor: &ActorId, preview_key: &str) -> Result<(), DbError> {
        self.handle.delete(&encode_preview_key(actor, preview_key).await?).await
    }

    pub async fn latest(&self, actor: &ActorId, preview_key: &str) -> Result<Option<Vec<u8>>, DbError> {
        self.handle.get(&encode_preview_key(actor, preview_key).await?).await
    }

    /// @emoji 📋️ Every currently-live `(preview_key, value)` published by `actor`.
    pub async fn for_actor(&self, actor: &ActorId) -> Result<Vec<(String, Vec<u8>)>, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        self.handle
            .scan_prefix(&prefix)
            .await?
            .into_iter()
            .map(|(key, value)| {
                let preview_key = String::from_utf8(key[prefix.len()..].to_vec()).map_err(|_| DbError::Corrupt("preview index key suffix is not valid utf-8".to_string()))?;
                Ok((preview_key, value))
            })
            .collect()
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        self.handle.stats().await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        self.handle.compact().await
    }
}
//#endregion 🔖️PreviewIndex

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::MemoryStorage;

    async fn entry(key: &[u8], value: &[u8]) -> RunEntry {
        RunEntry { key: key.to_vec(), value: RunValue::Put(value.to_vec()) }
    }

    async fn tombstone(key: &[u8]) -> RunEntry {
        RunEntry { key: key.to_vec(), value: RunValue::Tombstone }
    }

    //#region 🔖️SortedRun
    #[semio_framework_async_macros::async_test]
    async fn run_round_trips_through_encode_and_decode() {
        let entries = vec![entry(b"a", b"1").await, entry(b"b", b"2").await, tombstone(b"c").await];
        let encoded = encode_run(IndexKind::Command, &entries).await.expect("encode");
        let decoded = decode_run(&encoded, IndexKind::Command).await.expect("decode");
        assert_eq!(decoded, entries);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_detects_corruption_via_checksum() {
        let entries = vec![entry(b"a", b"1").await];
        let mut encoded = encode_run(IndexKind::Command, &entries).await.expect("encode");
        let last = encoded.len() - 1;
        encoded[last] ^= 0xFF;
        assert!(matches!(decode_run(&encoded, IndexKind::Command).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_rejects_kind_mismatch() {
        let entries = vec![entry(b"a", b"1").await];
        let encoded = encode_run(IndexKind::Command, &entries).await.expect("encode");
        assert!(matches!(decode_run(&encoded, IndexKind::Commit).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_rejects_non_ascending_entries() {
        // Hand-build a malformed body bypassing `encode_run`'s own ordering check, to exercise
        // `decode_run`'s independent defensive re-validation.
        let mut writer = ByteWriter::new().await;
        writer.write_bytes(&RUN_MAGIC).await;
        writer.write_u8(RUN_VERSION).await;
        writer.write_u8(IndexKind::Command.tag()).await;
        writer.write_varint_u64(2).await;
        for key in [b"b".as_slice(), b"a".as_slice()] {
            writer.write_varint_u64(key.len() as u64).await;
            writer.write_bytes(key).await;
            writer.write_u8(1).await;
            writer.write_varint_u64(1).await;
            writer.write_bytes(b"x").await;
        }
        let mut bytes = writer.into_bytes().await;
        let checksum = crc32c(&bytes).await;
        bytes.extend_from_slice(&checksum.to_le_bytes());
        assert!(matches!(decode_run(&bytes, IndexKind::Command).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_run_sorts_and_last_write_wins_on_duplicate_keys() {
        let built = build_run(vec![(b"b".to_vec(), RunValue::Put(b"1".to_vec())), (b"a".to_vec(), RunValue::Put(b"2".to_vec())), (b"b".to_vec(), RunValue::Put(b"3".to_vec()))]);
        assert_eq!(built, vec![entry(b"a", b"2").await, entry(b"b", b"3").await]);
    }
    //#endregion 🔖️SortedRun

    //#region 🔖️Merge
    #[semio_framework_async_macros::async_test]
    async fn merge_runs_prefers_newest_and_respects_drop_tombstones() {
        let older = vec![entry(b"a", b"old-a").await, entry(b"b", b"old-b").await];
        let newer = vec![tombstone(b"b").await, entry(b"c", b"new-c").await];

        let keep_tombstones = merge_runs(&[older.clone(), newer.clone()], false);
        assert_eq!(keep_tombstones, vec![entry(b"a", b"old-a").await, tombstone(b"b").await, entry(b"c", b"new-c").await]);

        let dropped = merge_runs(&[older, newer], true);
        assert_eq!(dropped, vec![entry(b"a", b"old-a").await, entry(b"c", b"new-c").await]);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_runs_of_zero_runs_is_empty() {
        assert!(merge_runs(&[], true).is_empty());
    }
    //#endregion 🔖️Merge

    //#region 🔖️IndexKind
    #[semio_framework_async_macros::async_test]
    async fn run_id_round_trips_kind_and_sequence_for_every_kind() {
        for kind in IndexKind::ALL {
            for sequence in [0u64, 1, SEQUENCE_MASK] {
                let run_id = make_run_id(kind, sequence).expect("make_run_id");
                assert_eq!(kind_tag_of_run_id(run_id), kind.tag());
                assert_eq!(sequence_of_run_id(run_id), sequence);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn run_id_rejects_sequence_overflowing_the_namespace() {
        assert!(matches!(make_run_id(IndexKind::Command, SEQUENCE_MASK + 1), Err(DbError::LimitExceeded(_))));
    }
    //#endregion 🔖️IndexKind

    //#region 🔖️IndexHandle
    #[semio_framework_async_macros::async_test]
    async fn index_handle_put_get_delete_round_trips() {
        let storage = MemoryStorage::new().await;
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        db_actor::block_on(handle.put(b"k1".to_vec(), b"v1".to_vec())).expect("put");
        db_actor::block_on(handle.put(b"k2".to_vec(), b"v2".to_vec())).expect("put");
        assert_eq!(db_actor::block_on(handle.get(b"k1")).expect("get"), Some(b"v1".to_vec()));
        assert_eq!(db_actor::block_on(handle.get(b"k2")).expect("get"), Some(b"v2".to_vec()));
        assert_eq!(db_actor::block_on(handle.get(b"missing")).expect("get"), None);

        db_actor::block_on(handle.delete(b"k1")).expect("delete");
        assert_eq!(db_actor::block_on(handle.get(b"k1")).expect("get"), None);
        assert_eq!(db_actor::block_on(handle.get(b"k2")).expect("get"), Some(b"v2".to_vec()));
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_put_overwrites_earlier_value_for_same_key() {
        let storage = MemoryStorage::new().await;
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        db_actor::block_on(handle.put(b"k".to_vec(), b"first".to_vec())).expect("put");
        db_actor::block_on(handle.put(b"k".to_vec(), b"second".to_vec())).expect("put");
        assert_eq!(db_actor::block_on(handle.get(b"k")).expect("get"), Some(b"second".to_vec()));
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_scan_prefix_returns_sorted_live_entries_only() {
        let storage = MemoryStorage::new().await;
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        db_actor::block_on(handle.put(b"a/1".to_vec(), b"1".to_vec())).expect("put");
        db_actor::block_on(handle.put(b"a/2".to_vec(), b"2".to_vec())).expect("put");
        db_actor::block_on(handle.put(b"b/1".to_vec(), b"3".to_vec())).expect("put");
        db_actor::block_on(handle.delete(b"a/2")).expect("delete");

        let scanned = db_actor::block_on(handle.scan_prefix(b"a/")).expect("scan_prefix");
        assert_eq!(scanned, vec![(b"a/1".to_vec(), b"1".to_vec())]);
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_auto_merges_to_stay_within_policy() {
        let storage = MemoryStorage::new().await;
        let policy = MergePolicy { max_runs_before_merge: 2 };
        let handle = IndexHandle::with_policy(&storage, ArtifactId::from("doc-1"), IndexKind::Command, policy).await;
        for i in 0..6u64 {
            db_actor::block_on(handle.put(format!("k{i:03}").into_bytes(), i.to_le_bytes().to_vec())).expect("put");
        }
        let stats = db_actor::block_on(handle.stats()).expect("stats");
        assert!(stats.run_count <= 2, "run_count {} should respect the merge policy", stats.run_count);
        for i in 0..6u64 {
            let value = db_actor::block_on(handle.get(format!("k{i:03}").as_bytes())).expect("get").expect("present");
            assert_eq!(u64::from_le_bytes(value.try_into().expect("8 bytes")), i);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_compact_collapses_to_one_run_and_drops_tombstones() {
        let storage = MemoryStorage::new().await;
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        db_actor::block_on(handle.put(b"a".to_vec(), b"1".to_vec())).expect("put");
        db_actor::block_on(handle.put(b"b".to_vec(), b"2".to_vec())).expect("put");
        db_actor::block_on(handle.delete(b"a")).expect("delete");

        let stats = db_actor::block_on(handle.compact()).expect("compact");
        assert_eq!(stats.run_count, 1);
        assert_eq!(stats.entry_count, 1);
        assert_eq!(db_actor::block_on(handle.get(b"a")).expect("get"), None);
        assert_eq!(db_actor::block_on(handle.get(b"b")).expect("get"), Some(b"2".to_vec()));
        db_actor::block_on(handle.verify()).expect("verify");
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_compact_of_one_run_is_a_no_op() {
        let storage = MemoryStorage::new().await;
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        db_actor::block_on(handle.put(b"a".to_vec(), b"1".to_vec())).expect("put");
        let before = db_actor::block_on(handle.stats()).expect("stats");
        let after = db_actor::block_on(handle.compact()).expect("compact");
        assert_eq!(before, after);
    }

    #[semio_framework_async_macros::async_test]
    async fn different_kinds_do_not_collide_for_the_same_document() {
        let storage = MemoryStorage::new().await;
        let document = ArtifactId::from("doc-1");
        let commands = IndexHandle::new(&storage, document.clone(), IndexKind::Command).await;
        let regions = IndexHandle::new(&storage, document, IndexKind::TouchedRegion).await;

        db_actor::block_on(commands.put(b"shared-key".to_vec(), b"command-value".to_vec())).expect("put");
        db_actor::block_on(regions.put(b"shared-key".to_vec(), b"region-value".to_vec())).expect("put");

        assert_eq!(db_actor::block_on(commands.get(b"shared-key")).expect("get"), Some(b"command-value".to_vec()));
        assert_eq!(db_actor::block_on(regions.get(b"shared-key")).expect("get"), Some(b"region-value".to_vec()));
        assert_eq!(db_actor::block_on(commands.stats()).expect("stats").run_count, 1);
        assert_eq!(db_actor::block_on(regions.stats()).expect("stats").run_count, 1);
    }
    //#endregion 🔖️IndexHandle

    //#region 🔖️TypedIndexes
    #[semio_framework_async_macros::async_test]
    async fn command_index_records_and_looks_up_locations() {
        let storage = MemoryStorage::new().await;
        let index = CommandIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let location = RecordLocation { segment: 3, offset: 128, len: 64 };
        db_actor::block_on(index.record(42, location)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), Some(location));
        assert_eq!(db_actor::block_on(index.lookup(43)).expect("lookup"), None);
        db_actor::block_on(index.remove(42)).expect("remove");
        assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_index_records_and_looks_up_locations() {
        let storage = MemoryStorage::new().await;
        let index = InverseIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let location = RecordLocation { segment: 1, offset: 0, len: 16 };
        db_actor::block_on(index.record(7, location)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup(7)).expect("lookup"), Some(location));
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_seq_index_resolves_and_tracks_latest_per_actor() {
        let storage = MemoryStorage::new().await;
        let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let alice = ActorId::from("alice");
        let bob = ActorId::from("bob");
        db_actor::block_on(index.record(&alice, 1, 100)).expect("record");
        db_actor::block_on(index.record(&alice, 2, 101)).expect("record");
        db_actor::block_on(index.record(&bob, 1, 200)).expect("record");

        assert_eq!(db_actor::block_on(index.lookup(&alice, 1)).expect("lookup"), Some(100));
        assert_eq!(db_actor::block_on(index.lookup(&alice, 2)).expect("lookup"), Some(101));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&alice)).expect("latest"), Some((2, 101)));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&bob)).expect("latest"), Some((1, 200)));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&ActorId::from("carol"))).expect("latest"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_seq_index_rejects_actor_id_with_embedded_nul() {
        let storage = MemoryStorage::new().await;
        let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let unsafe_actor = ActorId::from("bad\u{0}actor");
        assert!(matches!(db_actor::block_on(index.record(&unsafe_actor, 1, 1)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn frontier_index_round_trips_and_tracks_latest() {
        let storage = MemoryStorage::new().await;
        let index = FrontierIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let first = Frontier { document: ArtifactId::from("doc-1"), head_seq: 1, commit_seq: 1, chain_hash: [1u8; 32], epoch: 0 };
        let second = Frontier { document: ArtifactId::from("doc-1"), head_seq: 5, commit_seq: 2, chain_hash: [2u8; 32], epoch: 1 };
        db_actor::block_on(index.record(&first)).expect("record");
        db_actor::block_on(index.record(&second)).expect("record");

        assert_eq!(db_actor::block_on(index.lookup(1)).expect("lookup"), Some(first));
        assert_eq!(db_actor::block_on(index.latest()).expect("latest"), Some(second));
    }

    #[semio_framework_async_macros::async_test]
    async fn touched_region_index_accumulates_sorted_unique_seqs() {
        let storage = MemoryStorage::new().await;
        let index = TouchedRegionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
        db_actor::block_on(index.record_touch(b"region-a", 2)).expect("record_touch");
        db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
        assert_eq!(db_actor::block_on(index.touching(b"region-a")).expect("touching"), vec![2, 5]);
        assert_eq!(db_actor::block_on(index.touching(b"region-b")).expect("touching"), Vec::<u64>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn commit_index_round_trips() {
        let storage = MemoryStorage::new().await;
        let index = CommitIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record("ck-abc123", 9)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup("ck-abc123")).expect("lookup"), Some(9));
        assert_eq!(db_actor::block_on(index.lookup("ck-missing")).expect("lookup"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn full_text_index_search_finds_indexed_documents() {
        let storage = MemoryStorage::new().await;
        let index = FullTextIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.index_document(1, "The Quick Brown Fox")).expect("index");
        db_actor::block_on(index.index_document(2, "quick jumps")).expect("index");

        assert_eq!(db_actor::block_on(index.search("quick")).expect("search"), vec![1, 2]);
        assert_eq!(db_actor::block_on(index.search("QUICK")).expect("search"), vec![1, 2]);
        assert_eq!(db_actor::block_on(index.search("fox")).expect("search"), vec![1]);
        assert_eq!(db_actor::block_on(index.search("absent")).expect("search"), Vec::<u64>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn conflict_index_accumulates_multiple_records_per_command() {
        let storage = MemoryStorage::new().await;
        let index = ConflictIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record_conflict(5, b"region-collision".to_vec())).expect("record_conflict");
        db_actor::block_on(index.record_conflict(5, b"constraint-violation".to_vec())).expect("record_conflict");
        db_actor::block_on(index.record_conflict(6, b"other".to_vec())).expect("record_conflict");

        assert_eq!(db_actor::block_on(index.conflicts_for(5)).expect("conflicts_for"), vec![b"region-collision".to_vec(), b"constraint-violation".to_vec()]);
        assert_eq!(db_actor::block_on(index.conflicts_for(6)).expect("conflicts_for"), vec![b"other".to_vec()]);
        assert_eq!(db_actor::block_on(index.conflicts_for(7)).expect("conflicts_for"), Vec::<Vec<u8>>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_index_resolves_exact_and_floor_lookups_scoped_to_projection_id() {
        let storage = MemoryStorage::new().await;
        let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record("by-author", 10, b"state-10".to_vec())).expect("record");
        db_actor::block_on(index.record("by-author", 20, b"state-20".to_vec())).expect("record");

        assert_eq!(db_actor::block_on(index.at("by-author", 10)).expect("at"), Some(b"state-10".to_vec()));
        assert_eq!(db_actor::block_on(index.at("by-author", 15)).expect("at"), None);
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-author", 15)).expect("latest_at_or_before"), Some((10, b"state-10".to_vec())));
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-author", 20)).expect("latest_at_or_before"), Some((20, b"state-20".to_vec())));
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-author", 5)).expect("latest_at_or_before"), None);
        // 🎯️ "by-color" sorts after "by-author" but has no entries at all — must not fall back to
        // a lexicographically-earlier projection's entry.
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-color", 100)).expect("latest_at_or_before"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_index_rejects_projection_id_with_embedded_nul() {
        let storage = MemoryStorage::new().await;
        let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        assert!(matches!(db_actor::block_on(index.record("bad\u{0}id", 1, vec![1])), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_index_coalesces_latest_publish_or_withdraw_per_actor_and_key() {
        let storage = MemoryStorage::new().await;
        let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let alice = ActorId::from("alice");

        db_actor::block_on(index.publish(&alice, "drag-ghost", vec![1])).expect("publish");
        assert_eq!(db_actor::block_on(index.latest(&alice, "drag-ghost")).expect("latest"), Some(vec![1]));

        db_actor::block_on(index.publish(&alice, "drag-ghost", vec![2])).expect("publish");
        assert_eq!(db_actor::block_on(index.latest(&alice, "drag-ghost")).expect("latest"), Some(vec![2]));

        db_actor::block_on(index.publish(&alice, "cursor", vec![9])).expect("publish");
        let mut for_alice = db_actor::block_on(index.for_actor(&alice)).expect("for_actor");
        for_alice.sort();
        assert_eq!(for_alice, vec![("cursor".to_string(), vec![9]), ("drag-ghost".to_string(), vec![2])]);

        db_actor::block_on(index.withdraw(&alice, "drag-ghost")).expect("withdraw");
        assert_eq!(db_actor::block_on(index.latest(&alice, "drag-ghost")).expect("latest"), None);
        assert_eq!(db_actor::block_on(index.for_actor(&alice)).expect("for_actor"), vec![("cursor".to_string(), vec![9])]);
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_index_rejects_actor_id_with_embedded_nul() {
        let storage = MemoryStorage::new().await;
        let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let unsafe_actor = ActorId::from("bad\u{0}actor");
        assert!(matches!(db_actor::block_on(index.publish(&unsafe_actor, "k", vec![1])), Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖️TypedIndexes
}
//#endregion 🧪️Tests
