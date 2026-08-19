//! @emoji 🎞️ Protocol materialization: resolving *which* checkpoint/tail combination to replay
//! from a `.spr` byte stream (`resolve_plan`), and a closure-generic driver that actually replays
//! it into a caller-owned snapshot type `P` (`materialize_with`). This crate never knows what
//! `P` is or how an op applies to it — `crate::os_spr::history::HistoryEdit`'s `ops: Vec<OpPayload>` stay
//! opaque text/binary blobs all the way through; the `apply_edit` closure is where a downstream
//! technology (its own `dsl`-generated `Mutation` impls) turns them into real mutations. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_materialize`).
//!
//! @emoji 🧭️ `REC_PROJECTION` bodies (a complete `.spk` or dsl-text snapshot) are just as opaque to
//! this crate as op payloads are — it stores/hashes/frames them, never decodes them; only
//! `decode_base`'s caller knows how to turn embedded bytes into `P`.

use crate::os_pack::{ByteReader, ByteWriter};
use crate::os_spr::wire::{DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use crate::os_spr::format::{Blake3Hasher, FrameCursor, RecoveryMode, ReverseFrameCursor, VerificationLevel, HEADER_SIZE};
use std::collections::HashMap;

//#region 🔖️Snapshot
/// @emoji 🗂️ How a `REC_PROJECTION` frame's body bytes are stored relative to the frame itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotBodyKind {
    EmbeddedPack,
    SidecarPack,
    EmbeddedDsl,
}

/// @emoji 📸️ One decoded `REC_PROJECTION` record: an anchor (checkpoint id or bare edit ordinal),
/// the edit ordinal it was taken at, and its opaque body (present iff embedded — `None` for
/// `SidecarPack`, whose bytes live in a separate `.sprc` file this crate never opens).
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotRecord {
    pub anchor_checkpoint_id: Option<String>,
    pub edit_ordinal: u64,
    pub body_kind: SnapshotBodyKind,
    pub body_hash: [u8; 32],
    pub body: Option<Vec<u8>>,
}

/// @emoji 🚨️ This crate's own structural-decode error, `offset` left at 0 since snapshot payloads
/// are decoded standalone (no absolute file position in scope) — callers threading through
/// `resolve_plan` see the frame's real offset via the propagated `ProtocolError` from
/// `protocol_format` itself when the surrounding frame is malformed.
// 🚫️async: R9 pure accessor — most call sites are inside `.ok_or_else`/`.map_err`'s sync
// closure; no suspension point exists in the body either.
fn malformed(what: &'static str, detail: impl Into<String>) -> ProtocolError {
    ProtocolError::Malformed { what, offset: 0, detail: detail.into() }
}

async fn body_kind_to_byte(kind: SnapshotBodyKind) -> u8 {
    match kind {
        SnapshotBodyKind::EmbeddedPack => 0,
        SnapshotBodyKind::SidecarPack => 1,
        SnapshotBodyKind::EmbeddedDsl => 2,
    }
}

async fn body_kind_from_byte(byte: u8) -> Result<SnapshotBodyKind, ProtocolError> {
    match byte {
        0 => Ok(SnapshotBodyKind::EmbeddedPack),
        1 => Ok(SnapshotBodyKind::SidecarPack),
        2 => Ok(SnapshotBodyKind::EmbeddedDsl),
        other => Err(malformed("snapshot body_kind", format!("unknown body_kind {other:#x}"))),
    }
}

/// @emoji 🪪️ The header fields of a `REC_PROJECTION` payload, decoded without copying the body —
/// `resolve_plan` uses this directly against a zero-copy `RecordFrame::payload()` slice so the
/// eventual `BaseBytes::Borrowed` span never allocates; `decode_snapshot` (the public, owning API)
/// layers a `body.to_vec()` on top for callers that don't care about zero-copy (e.g. `protocol_cli`).
struct SnapshotHeader {
    anchor_checkpoint_id: Option<String>,
    edit_ordinal: u64,
    body_kind: SnapshotBodyKind,
    body_hash: [u8; 32],
}

/// @emoji 👓️ Parses a `REC_PROJECTION` payload's header, returning the `(start, len)` span of the
/// embedded body *within `payload`* (so a caller already holding a `&'a [u8]` payload can slice it
/// zero-copy) — `None` iff `body_kind == SidecarPack`, which never embeds a body.
async fn parse_snapshot(payload: &[u8]) -> Result<(SnapshotHeader, Option<(usize, usize)>), ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format != 1 {
        return Err(malformed("snapshot format", format!("unsupported format {format}")));
    }
    let anchor_tag = input.read_u8().await?;
    let anchor_checkpoint_id = match anchor_tag {
        0 => {
            let len = input.read_varint_u64().await? as usize;
            let bytes = input.read_bytes(len).await?;
            Some(std::str::from_utf8(bytes).map_err(|_| malformed("snapshot checkpoint_id utf8", "invalid utf-8"))?.to_string())
        }
        1 => None,
        other => return Err(malformed("snapshot anchor_tag", format!("unknown anchor tag {other:#x}"))),
    };
    let edit_ordinal = input.read_varint_u64().await?;
    let body_kind = body_kind_from_byte(input.read_u8().await?).await?;
    let body_hash = input.read_array32().await?;
    let body_span = if body_kind != SnapshotBodyKind::SidecarPack {
        let len = input.read_varint_u64().await? as usize;
        let start = input.position().await;
        input.read_bytes(len).await?; // bounds-checked; establishes the span is actually present
        Some((start, len))
    } else {
        None
    };
    Ok((SnapshotHeader { anchor_checkpoint_id, edit_ordinal, body_kind, body_hash }, body_span))
}

/// @emoji ✍️ `format(1), anchor_tag(0=checkpoint+id / 1=ordinal-only), [checkpoint_id], edit_ordinal
/// varint, body_kind, body_hash[32], [body_len varint + body iff embedded]` — no `DictBuilder`
/// parameter (unlike `protocol_history`'s per-kind codecs) since a snapshot anchor is written at
/// most once per snapshot and gains nothing from dictionary interning; `checkpoint_id` is always
/// tag-0 raw text.
pub async fn encode_snapshot(record: &SnapshotRecord) -> Vec<u8> {
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    match &record.anchor_checkpoint_id {
        Some(id) => {
            out.write_u8(0).await;
            out.write_varint_u64(id.len() as u64).await;
            out.write_bytes(id.as_bytes()).await;
        }
        None => out.write_u8(1).await,
    }
    out.write_varint_u64(record.edit_ordinal).await;
    out.write_u8(body_kind_to_byte(record.body_kind).await).await;
    out.write_bytes(&record.body_hash).await;
    if record.body_kind != SnapshotBodyKind::SidecarPack {
        let body = record.body.as_deref().unwrap_or(&[]);
        out.write_varint_u64(body.len() as u64).await;
        out.write_bytes(body).await;
    }
    out.into_bytes().await
}

/// @emoji 👓️ The owning twin of `parse_snapshot`, for callers (e.g. `protocol_cli inspect`) that
/// want a self-contained `SnapshotRecord` rather than a zero-copy span.
pub async fn decode_snapshot(payload: &[u8]) -> Result<SnapshotRecord, ProtocolError> {
    let (header, body_span) = parse_snapshot(payload).await?;
    let body = body_span.map(|(start, len)| payload[start..start + len].to_vec());
    Ok(SnapshotRecord { anchor_checkpoint_id: header.anchor_checkpoint_id, edit_ordinal: header.edit_ordinal, body_kind: header.body_kind, body_hash: header.body_hash, body })
}

/// @emoji 🔎️ Reads a dict-record payload written by `protocol_history`'s (private) flush routine —
/// `format(1), base_count varint, count varint, count x (len varint + utf8)`. Duplicated here rather
/// than imported because it is a private implementation detail of `protocol_history`; the wire shape
/// is fully pinned by that crate's own `//#region 🔖️Codec` doc comment, so this stays in lockstep by
/// construction, not by convention.
async fn apply_dict_record(dict: &mut DictReader, payload: &[u8]) -> Result<(), ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed("dict record format", format!("unsupported format {format}")));
    }
    let base_count = input.read_varint_u64().await? as u32;
    let count = input.read_varint_u64().await?;
    // 🎯️ Never `Vec::with_capacity(count)`: `count` is untrusted input read before any bound check
    // against the (already frame-limited) payload — an adversarial huge varint must not itself
    // trigger a huge allocation before the loop's own bounds-checked reads would fail it anyway.
    let mut entries: Vec<String> = Vec::new();
    for _ in 0..count {
        let len = input.read_varint_u64().await? as usize;
        let bytes = input.read_bytes(len).await?;
        entries.push(std::str::from_utf8(bytes).map_err(|_| malformed("dict entry utf8", "invalid utf-8"))?.to_string());
    }
    dict.extend(base_count, entries).await
}
//#endregion 🔖️Snapshot

//#region 🔖️Policy
/// @emoji 🗓️ Advisory triggers for when a technology's writer should ask this crate's caller to
/// take a fresh `REC_PROJECTION` snapshot; this crate never triggers a checkpoint itself (it has no
/// write path), it only carries the policy so `protocol_io`/a `db` server can consult one shared
/// definition rather than each inventing its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckpointPolicy {
    pub every_edits: u64,
    pub every_bytes: u64,
    pub on_checkpoint_commit: bool,
    pub embed_below: u64,
}

impl Default for CheckpointPolicy {
    fn default() -> Self {
        Self { every_edits: 512, every_bytes: 4 * 1024 * 1024, on_checkpoint_commit: true, embed_below: 1024 * 1024 }
    }
}

impl CheckpointPolicy {
    /// @emoji ✅️ Whether a snapshot is due given how much has accumulated since the last one.
    pub async fn should_checkpoint(&self, edits_since_last: u64, bytes_since_last: u64, is_checkpoint_commit: bool) -> bool {
        (self.on_checkpoint_commit && is_checkpoint_commit) || edits_since_last >= self.every_edits || bytes_since_last >= self.every_bytes
    }

    /// @emoji 📦️ Whether a body of `body_len` bytes should be embedded inline vs. written as a
    /// `.sprc` sidecar (`SnapshotBodyKind::SidecarPack`).
    pub async fn should_embed(&self, body_len: u64) -> bool {
        body_len < self.embed_below
    }
}
//#endregion 🔖️Policy

//#region 🔖️Plan
/// @emoji 🧺️ Where the base of a materialization plan's bytes actually live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseBytes<'a> {
    Borrowed(&'a [u8]),
    Sidecar { expected_hash: [u8; 32] },
}

/// @emoji 🧱️ A resolved base to decode (`P::default()`-equivalent for the caller's snapshot type)
/// plus how many leading edits (0-based ordinals `0..applied_edits`) it already reflects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseSnapshot<'a> {
    pub bytes: BaseBytes<'a>,
    pub applied_edits: u64,
}

/// @emoji 🗺️ A fully resolved plan: decode `base.bytes` into `P`, then replay every `REC_EDIT` frame
/// starting at `tail_start_offset` whose 0-based ordinal is `<= target_edit_ordinal` (`None` means
/// "no cap — replay through the trusted tail").
///
/// 🎯️ Design choice: this struct carries one extra `pub(crate)` field (`skipped_corrupt`) beyond the
/// three the contract lists — `MaterializeReport::snapshots_skipped_corrupt` needs a value from
/// `resolve_plan`'s own corrupt-snapshot retries, and the contract's frozen `pub` field list has
/// nowhere else to carry it forward to `materialize_with`. Never `pub`, never named by another
/// crate (the facade re-exports `MaterializePlan` but not a way to construct one outside
/// `resolve_plan`), so this is additive, not a violation of the frozen public shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterializePlan<'a> {
    pub base: BaseSnapshot<'a>,
    pub tail_start_offset: u64,
    pub target_edit_ordinal: Option<u64>,
    pub(crate) skipped_corrupt: u32,
}

/// @emoji 🎯️ What edit ordinal a caller wants materialized through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterializeTarget {
    LatestOnActive,
    AtCheckpoint(String),
    AtEditOrdinal(u64),
}

/// @emoji 📸️ One candidate `REC_PROJECTION` frame found during plan resolution, still borrowing
/// zero-copy from the trusted byte range so a `Borrowed` base never allocates.
struct Candidate<'a> {
    offset: u64,
    frame_len: u64,
    header: SnapshotHeader,
    body_span: Option<(usize, usize)>,
    payload: &'a [u8],
}

async fn verify_candidate(hasher: &Blake3Hasher, candidate: &Candidate<'_>) -> bool {
    match candidate.body_span {
        Some((start, len)) => hasher.hash(&candidate.payload[start..start + len]).await == candidate.header.body_hash,
        // A sidecar body lives outside `trusted` entirely; nothing here to hash against.
        None => true,
    }
}

/// @emoji 👓️ Reads and header-parses the `REC_PROJECTION` frame expected at an absolute offset
/// (as recorded by a `crate::os_spr::history::IndexReader`'s `SEC_SNAPSHOT_OFFSETS` section).
async fn read_snapshot_at(trusted: &[u8], offset: u64) -> Result<Candidate<'_>, ProtocolError> {
    let mut cursor = FrameCursor::new(trusted, offset).await;
    let frame = cursor.next_frame().await?.ok_or_else(|| malformed("snapshot frame", "missing frame at indexed offset"))?;
    if frame.kind != crate::os_spr::REC_PROJECTION {
        return Err(malformed("snapshot frame", "kind mismatch at indexed offset"));
    }
    let (header, body_span) = parse_snapshot(frame.payload().await).await?;
    Ok(Candidate { offset, frame_len: frame.frame_len().await, header, body_span, payload: frame.payload().await })
}

/// @emoji 🔎️ Reverse-scans the whole trusted record stream for the LATEST `REC_INDEX` frame — the
/// only kind this crate needs from the advisory index, since `SEC_SNAPSHOT_OFFSETS`/
/// `SEC_CHECKPOINT_OFFSETS` are exactly what `resolve_plan` consults. `None` if no valid `REC_INDEX`
/// frame exists (a file that has never been compacted/indexed), signalling the reverse-frame-scan
/// fallback below.
async fn locate_index(trusted: &[u8]) -> Option<crate::os_spr::history::IndexReader<'_>> {
    let record_stream = &trusted[HEADER_SIZE..];
    let mut cursor = ReverseFrameCursor::at_end(record_stream).await;
    while let Ok(Some(frame)) = cursor.prev_frame().await {
        if frame.kind == crate::os_spr::REC_INDEX {
            if let Ok(reader) = crate::os_spr::history::IndexReader::open(frame.payload().await).await {
                return Some(reader);
            }
        }
    }
    None
}

/// @emoji 🔁️ Index-backed search: looks up the newest snapshot `<= cap`, verifies it, and on
/// corruption retries at a strictly lower ordinal cap (jumping straight past the corrupt entry when
/// its own ordinal is known, else stepping down by one) until a valid candidate is found or the
/// index is exhausted.
async fn find_snapshot_via_index<'a>(trusted: &'a [u8], index: &crate::os_spr::history::IndexReader<'_>, cap: u64, skipped: &mut u32) -> Option<Candidate<'a>> {
    let hasher = Blake3Hasher;
    let mut remaining_cap = cap;
    loop {
        let offset = index.latest_snapshot_offset_at_or_before(remaining_cap).await?;
        match read_snapshot_at(trusted, offset).await {
            Ok(candidate) if verify_candidate(&hasher, &candidate).await => return Some(candidate),
            Ok(candidate) => {
                *skipped += 1;
                remaining_cap = candidate.header.edit_ordinal.checked_sub(1)?;
            }
            Err(_) => {
                *skipped += 1;
                remaining_cap = remaining_cap.checked_sub(1)?;
            }
        }
    }
}

/// @emoji 🔁️ Reverse-frame-scan fallback for when no usable `REC_INDEX` exists (or it doesn't cover
/// the snapshot actually needed): walks every frame back from the trusted end, returning the
/// first valid `REC_PROJECTION` at or before `cap`.
async fn find_snapshot_by_scan<'a>(trusted: &'a [u8], cap: u64, skipped: &mut u32) -> Option<Candidate<'a>> {
    let hasher = Blake3Hasher;
    let record_stream = &trusted[HEADER_SIZE..];
    let mut cursor = ReverseFrameCursor::at_end(record_stream).await;
    while let Ok(Some(frame)) = cursor.prev_frame().await {
        if frame.kind != crate::os_spr::REC_PROJECTION {
            continue;
        }
        match parse_snapshot(frame.payload().await).await {
            Ok((header, body_span)) if header.edit_ordinal <= cap => {
                let candidate = Candidate { offset: frame.offset + HEADER_SIZE as u64, frame_len: frame.frame_len().await, header, body_span, payload: frame.payload().await };
                if verify_candidate(&hasher, &candidate).await {
                    return Some(candidate);
                }
                *skipped += 1;
            }
            Ok(_) => {} // a snapshot past `cap` — keep walking backward for an older one
            Err(_) => *skipped += 1,
        }
    }
    None
}

async fn find_best_snapshot<'a>(trusted: &'a [u8], cap: u64, skipped: &mut u32) -> Option<Candidate<'a>> {
    if let Some(index) = locate_index(trusted).await {
        if let Some(candidate) = find_snapshot_via_index(trusted, &index, cap, skipped).await {
            return Some(candidate);
        }
    }
    find_snapshot_by_scan(trusted, cap, skipped).await
}

/// @emoji 🧮️ Resolves a checkpoint id to the 0-based edit ordinal of the last edit it covers, via
/// the advisory index when available, else a full decode-and-walk fallback (checkpoint ->
/// change_ids -> each change's edit_ids -> max ordinal by position in `log.edits`, matching
/// `crate::os_spr::history::encode_history`'s own ordinal assignment).
async fn resolve_checkpoint_edit_ordinal(trusted: &[u8], checkpoint_id: &str, limits: &ProtocolLimits) -> Result<Option<u64>, ProtocolError> {
    if let Some(index) = locate_index(trusted).await {
        if let Some((_, ordinal)) = index.checkpoint_offset(checkpoint_id).await {
            return Ok(Some(ordinal));
        }
    }
    let options = crate::os_spr::history::DecodeOptions { verification: VerificationLevel::Standard, limits: limits.clone() };
    let log = crate::os_spr::history::decode_history(trusted, &options).await?;
    let checkpoint = log.checkpoints.iter().find(|c| c.id == checkpoint_id).ok_or_else(|| malformed("checkpoint", format!("checkpoint '{checkpoint_id}' not found")))?;
    let ordinals: HashMap<&str, u64> = log.edits.iter().enumerate().map(|(i, e)| (e.id.as_str(), i as u64)).collect();
    let mut max_ordinal: Option<u64> = None;
    for change_id in &checkpoint.change_ids {
        let Some(change) = log.changes.iter().find(|c| &c.id == change_id) else { continue };
        for edit_id in &change.edit_ids {
            if let Some(&ordinal) = ordinals.get(edit_id.as_str()) {
                max_ordinal = Some(max_ordinal.map_or(ordinal, |m| m.max(ordinal)));
            }
        }
    }
    Ok(max_ordinal)
}

async fn resolve_target_edit_ordinal(trusted: &[u8], target: &MaterializeTarget, limits: &ProtocolLimits) -> Result<Option<u64>, ProtocolError> {
    match target {
        // 🎯️ Design choice: "latest" has no ordinal cap — alternatives in this data model name sets
        // of checkpoints, not forked edit sequences (`HistoryLog::edits` is one flat, shared log), so
        // "latest on the active alternative" and "latest, full stop" resolve identically here.
        MaterializeTarget::LatestOnActive => Ok(None),
        MaterializeTarget::AtEditOrdinal(ordinal) => Ok(Some(*ordinal)),
        MaterializeTarget::AtCheckpoint(id) => resolve_checkpoint_edit_ordinal(trusted, id, limits).await,
    }
}

/// @emoji 🗺️ Resolves which base (an embedded/sidecar snapshot, or `initial_pack` at ordinal 0)
/// and tail range to replay to reach `target`. Steps (per the frozen contract): recover the trusted
/// byte range; resolve `target` to a concrete edit ordinal cap; find the newest valid snapshot at
/// or before that cap (index first, reverse-scan fallback, skipping and retrying older candidates on
/// corruption); fall back to `initial_pack` at ordinal 0 if none verify.
// 🔒️ `target` is by-value per the frozen contract signature (downstream callers pass an owned
// enum they're done with); it happens to be read-only in this implementation.
#[allow(clippy::needless_pass_by_value)]
pub async fn resolve_plan<'a>(protocol_bytes: &'a [u8], initial_pack: &'a [u8], target: MaterializeTarget, limits: &ProtocolLimits) -> Result<MaterializePlan<'a>, ProtocolError> {
    let recovery = crate::os_spr::format::recover(&protocol_bytes, limits, RecoveryMode::LastCommit).await?;
    let trusted: &'a [u8] = &protocol_bytes[..recovery.bytes_recovered as usize];

    let target_edit_ordinal = resolve_target_edit_ordinal(trusted, &target, limits).await?;
    let cap = target_edit_ordinal.unwrap_or(u64::MAX);

    let mut skipped_corrupt = 0u32;
    let candidate = find_best_snapshot(trusted, cap, &mut skipped_corrupt);

    let (base, tail_start_offset) = match candidate.await {
        Some(candidate) => {
            let bytes = match candidate.header.body_kind {
                SnapshotBodyKind::SidecarPack => BaseBytes::Sidecar { expected_hash: candidate.header.body_hash },
                _ => {
                    let (start, len) = candidate.body_span.ok_or_else(|| malformed("snapshot body", "embedded kind missing a body span"))?;
                    BaseBytes::Borrowed(&candidate.payload[start..start + len])
                }
            };
            let applied_edits = candidate.header.edit_ordinal + 1;
            (BaseSnapshot { bytes, applied_edits }, candidate.offset + candidate.frame_len)
        }
        None => (BaseSnapshot { bytes: BaseBytes::Borrowed(initial_pack), applied_edits: 0 }, HEADER_SIZE as u64),
    };

    Ok(MaterializePlan { base, tail_start_offset, target_edit_ordinal, skipped_corrupt })
}
//#endregion 🔖️Plan

//#region 🔖️Drive
/// @emoji 📋️ What a `materialize_with` call actually did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterializeReport {
    /// @emoji 🧾️ `Some((anchor_checkpoint_id, applied_edits))` iff a snapshot (not `initial_pack`)
    /// was used as the base. 🎯️ Design choice: `MaterializePlan` (frozen shape) never threads the
    /// anchor checkpoint id this far — only `resolve_plan` ever saw it — so the inner `Option<String>`
    /// is always `None` here; the field shape is kept exactly as specified for a future revision that
    /// does thread it through.
    pub snapshot_used: Option<(Option<String>, u64)>,
    pub snapshots_skipped_corrupt: u32,
    pub edits_replayed: u64,
    pub bytes_read: u64,
    pub genesis_replay: bool,
}

/// @emoji 🔎️ Builds the `DictReader` + forward-ordered edit-id table covering every `REC_STR_DICT`/
/// `REC_EDIT` frame strictly before `up_to_offset` — needed before decoding any tail `REC_EDIT`
/// frame, since its `id`/dependency fields may reference dictionary entries or edit ordinals
/// introduced anywhere earlier in the file, including inside the base snapshot's own coverage.
async fn prescan_dict_and_edits(trusted: &[u8], up_to_offset: u64) -> Result<(DictReader, Vec<String>), ProtocolError> {
    let mut dict = DictReader::new().await;
    let mut edit_ids = Vec::new();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await? {
        if frame.offset >= up_to_offset {
            break;
        }
        match frame.kind {
            crate::os_spr::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            crate::os_spr::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = crate::os_spr::history::decode_edit(frame.payload().await, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id);
            }
            _ => {}
        }
    }
    Ok((dict, edit_ids))
}

/// @emoji ▶️ Decodes `plan.base.bytes` into `P` (never for `BaseBytes::Sidecar` — a caller that
/// resolved a sidecar snapshot must substitute `BaseBytes::Borrowed` with the fetched `.sprc`
/// bytes, e.g. via `crate::os_spr::io::read_sidecar`, before calling this), then replays every `REC_EDIT`
/// frame from `plan.tail_start_offset` onward whose ordinal is `<= plan.target_edit_ordinal` (no cap
/// iff `None`) through `apply_edit`. Re-derives its own trusted byte range from `protocol_bytes`
/// (default limits — no `limits` parameter on this frozen signature) so a torn live tail is silently
/// excluded rather than surfaced as an error, matching how every other reader in this crate family
/// treats the boundary `crate::os_spr::format::recover` establishes.
pub async fn materialize_with<P, E>(plan: MaterializePlan<'_>, protocol_bytes: &[u8], decode_base: impl FnOnce(&[u8]) -> Result<P, E>, mut apply_edit: impl FnMut(&mut P, &crate::os_spr::history::HistoryEdit) -> Result<(), E>) -> Result<(P, MaterializeReport), E>
where
    E: From<ProtocolError>,
{
    let base_bytes = match plan.base.bytes {
        BaseBytes::Borrowed(bytes) => bytes,
        BaseBytes::Sidecar { .. } => return Err(E::from(malformed("materialize base", "BaseBytes::Sidecar has no inline bytes; resolve the sidecar and substitute BaseBytes::Borrowed before calling materialize_with"))),
    };
    let mut snapshot = decode_base(base_bytes)?;

    let limits = ProtocolLimits::default();
    let recovery = crate::os_spr::format::recover(&protocol_bytes, &limits, RecoveryMode::LastCommit).await.map_err(E::from)?;
    let trusted = &protocol_bytes[..recovery.bytes_recovered as usize];

    let (mut dict, mut edit_ids) = prescan_dict_and_edits(trusted, plan.tail_start_offset).await.map_err(E::from)?;
    let mut edit_ordinal = edit_ids.len() as u64;

    let mut cursor = FrameCursor::new(trusted, plan.tail_start_offset).await;
    let mut edits_replayed = 0u64;
    let mut bytes_read = 0u64;

    while let Some(frame) = cursor.next_frame().await.map_err(E::from)? {
        bytes_read += frame.frame_len().await;
        match frame.kind {
            crate::os_spr::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await.map_err(E::from)?,
            crate::os_spr::REC_EDIT => {
                if let Some(target) = plan.target_edit_ordinal {
                    if edit_ordinal > target {
                        break;
                    }
                }
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = crate::os_spr::history::decode_edit(frame.payload().await, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.map_err(E::from)?;
                apply_edit(&mut snapshot, &edit)?;
                edit_ids.push(edit.id);
                edit_ordinal += 1;
                edits_replayed += 1;
            }
            _ => {}
        }
    }

    let genesis_replay = plan.base.applied_edits == 0;
    let snapshot_used = if genesis_replay { None } else { Some((None, plan.base.applied_edits)) };

    Ok((snapshot, MaterializeReport { snapshot_used, snapshots_skipped_corrupt: plan.skipped_corrupt, edits_replayed, bytes_read, genesis_replay }))
}
//#endregion 🔖️Drive

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_pack::CodecId;
    use crate::os_spr::wire::{DictBuilder, REC_EDIT, REQUIRED_HASH_CHAIN};
    use crate::os_spr::format::{SprWriter, WriteOptions};
    use crate::os_spr::history::{HistoryChange, HistoryCheckpoint, HistoryEdit, HistoryLog, OpPayload};

    //#region 🔖️Snapshot
    async fn sample_record(anchor: Option<&str>, ordinal: u64, kind: SnapshotBodyKind, body: Option<Vec<u8>>) -> SnapshotRecord {
        let body_hash = body.as_deref().map_or([0u8; 32], |b| Blake3Hasher.hash(b));
        SnapshotRecord { anchor_checkpoint_id: anchor.map(str::to_string), edit_ordinal: ordinal, body_kind: kind, body_hash, body }
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_round_trips_embedded_with_checkpoint_anchor() {
        let record = sample_record(Some("cp-1"), 7, SnapshotBodyKind::EmbeddedPack, Some(vec![1, 2, 3, 4]));
        let bytes = encode_snapshot(&record);
        let decoded = decode_snapshot(&bytes).await.unwrap();
        assert_eq!(decoded, record.await);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_round_trips_embedded_dsl_with_ordinal_only_anchor() {
        let record = sample_record(None, 0, SnapshotBodyKind::EmbeddedDsl, Some(b"(doc)".to_vec()));
        let bytes = encode_snapshot(&record);
        let decoded = decode_snapshot(&bytes).await.unwrap();
        assert_eq!(decoded, record.await);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_round_trips_sidecar_without_body() {
        let record = sample_record(Some("cp-9"), 42, SnapshotBodyKind::SidecarPack, None);
        let bytes = encode_snapshot(&record);
        let decoded = decode_snapshot(&bytes).await.unwrap();
        assert_eq!(decoded.body, None);
        assert_eq!(decoded, record.await);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_rejects_unknown_format() {
        let mut bytes = encode_snapshot(&sample_record(None, 0, SnapshotBodyKind::EmbeddedPack, Some(vec![9])));
        bytes[0] = 7;
        assert!(matches!(decode_snapshot(&bytes).await, Err(ProtocolError::Malformed { .. })));
    }
    //#endregion 🔖️Snapshot

    //#region 🔖️Plan
    async fn sample_edit(id: &str, op_text: &str) -> HistoryEdit {
        HistoryEdit {
            id: id.to_string(),
            actor: None,
            started_at: format!("t-{id}"),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some(op_text.to_string()), binary: None }],
            inverse: Vec::new(),
            meta: None,
        }
    }

    async fn flush_dict_delta<S: crate::os_pack::PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) {
        let len = dict.len();
        if len > *base {
            let entries = dict.entries_since(*base);
            let mut payload = ByteWriter::new().await;
            payload.write_u8(1).await;
            payload.write_varint_u64(*base as u64).await;
            payload.write_varint_u64(entries.await.len() as u64).await;
            for entry in entries {
                payload.write_varint_u64(entry.len() as u64).await;
                payload.write_bytes(entry.as_bytes()).await;
            }
            writer.write_record(crate::os_spr::REC_STR_DICT, true, &payload.into_bytes().await, CodecId(0)).await.unwrap();
            *base = len.await;
        }
    }

    /// @emoji 🏗️ Hand-assembles a `.spr` stream with 4 edits and one embedded-pack `REC_PROJECTION`
    /// taken right after edit ordinal 1 (i.e. covering edits 0 and 1) — the shape `resolve_plan`'s
    /// index-free reverse-scan fallback and `materialize_with`'s tail replay are exercised against.
    async fn build_stream_with_snapshot(snapshot_body: &[u8]) -> Vec<u8> {
        let write_options = WriteOptions { required_flags: REQUIRED_HASH_CHAIN, optional_flags: 0 };
        let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await.unwrap();
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;

        let doc_payload = crate::os_spr::history::encode_doc("doc-1", "schema-1", &mut dict);
        flush_dict_delta(&mut writer, &dict, &mut dict_base);
        writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0)).await.unwrap();

        for (i, edit) in [sample_edit("edit-0", "op-0"), sample_edit("edit-1", "op-1")].iter().enumerate() {
            let _ = i;
            let payload = crate::os_spr::history::encode_edit(edit, &mut dict, |_| None).await.unwrap();
            flush_dict_delta(&mut writer, &dict, &mut dict_base);
            writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
        }

        let snapshot = sample_record(None, 1, SnapshotBodyKind::EmbeddedPack, Some(snapshot_body.to_vec()));
        writer.write_record(crate::os_spr::REC_PROJECTION, false, &encode_snapshot(&snapshot), CodecId(0)).await.unwrap();

        for edit in [sample_edit("edit-2", "op-2"), sample_edit("edit-3", "op-3")] {
            let payload = crate::os_spr::history::encode_edit(&edit, &mut dict, |_| None).await.unwrap();
            flush_dict_delta(&mut writer, &dict, &mut dict_base);
            writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
        }

        writer.commit().await.unwrap();
        writer.into_sink().await
    }

    type Collected = (Vec<u8>, Vec<String>);

    // 🔒️ Must return `Result` to satisfy `materialize_with`'s `apply_edit` closure bound even
    // though this particular collector never fails.
    #[allow(clippy::unnecessary_wraps)]
    async fn collect_ids(p: &mut Collected, edit: &HistoryEdit) -> Result<(), ProtocolError> {
        p.1.push(edit.id.clone());
        Ok(())
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_plan_picks_snapshot_and_replays_only_the_tail() {
        let body = vec![0xAA, 0xBB, 0xCC];
        let bytes = build_stream_with_snapshot(&body);

        let plan = resolve_plan(&bytes, &[], MaterializeTarget::LatestOnActive, &ProtocolLimits::default()).await.unwrap();
        assert_eq!(plan.base.applied_edits, 2);
        assert_eq!(plan.target_edit_ordinal, None);
        assert_eq!(plan.skipped_corrupt, 0);
        match plan.base.bytes {
            BaseBytes::Borrowed(b) => assert_eq!(b, body.as_slice()),
            BaseBytes::Sidecar { .. } => panic!("expected an embedded base"),
        }

        let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
        assert_eq!(result.0, body);
        assert_eq!(result.1, vec!["edit-2".to_string(), "edit-3".to_string()]);
        assert!(!report.genesis_replay);
        assert_eq!(report.snapshot_used, Some((None, 2)));
        assert_eq!(report.edits_replayed, 2);
        assert_eq!(report.snapshots_skipped_corrupt, 0);
        assert!(report.bytes_read > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_plan_falls_back_to_initial_pack_when_target_precedes_every_snapshot() {
        let bytes = build_stream_with_snapshot(&[0xAA, 0xBB, 0xCC]);
        let initial_pack = b"INIT";

        let plan = resolve_plan(&bytes, initial_pack, MaterializeTarget::AtEditOrdinal(0), &ProtocolLimits::default()).await.unwrap();
        assert_eq!(plan.base.applied_edits, 0);
        match plan.base.bytes {
            BaseBytes::Borrowed(b) => assert_eq!(b, initial_pack),
            BaseBytes::Sidecar { .. } => panic!("expected the initial pack"),
        }

        let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
        assert_eq!(result.0, initial_pack);
        assert_eq!(result.1, vec!["edit-0".to_string()]);
        assert!(report.genesis_replay);
        assert_eq!(report.snapshot_used, None);
        assert_eq!(report.edits_replayed, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_plan_at_edit_ordinal_beyond_snapshot_replays_full_tail() {
        let body = vec![0xAA, 0xBB, 0xCC];
        let bytes = build_stream_with_snapshot(&body);

        let plan = resolve_plan(&bytes, &[], MaterializeTarget::AtEditOrdinal(3), &ProtocolLimits::default()).await.unwrap();
        let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
        assert_eq!(result.1, vec!["edit-2".to_string(), "edit-3".to_string()]);
        assert_eq!(report.edits_replayed, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_plan_at_checkpoint_falls_back_to_full_decode_without_an_index() {
        let mut log = HistoryLog { doc_id: "doc-2".to_string(), schema: "schema-2".to_string(), ..Default::default() };
        log.edits.push(sample_edit("edit-0", "op-0").await);
        log.edits.push(sample_edit("edit-1", "op-1").await);
        log.changes.push(HistoryChange { id: "change-1".to_string(), saved_at: "t-change-1".to_string(), edit_ids: vec!["edit-0".to_string(), "edit-1".to_string()], description: None });
        log.checkpoints.push(HistoryCheckpoint { id: "cp-1".to_string(), timestamp: "t-cp-1".to_string(), change_ids: vec!["change-1".to_string()], parent_id: None, authors: Vec::new(), message: None });

        let bytes = crate::os_spr::history::encode_history(&log, &crate::os_spr::history::EncodeOptions::default()).await.unwrap();
        let initial_pack = b"BASE";

        let plan = resolve_plan(&bytes, initial_pack, MaterializeTarget::AtCheckpoint("cp-1".to_string()), &ProtocolLimits::default()).await.unwrap();
        assert_eq!(plan.target_edit_ordinal, Some(1));
        assert_eq!(plan.base.applied_edits, 0); // no REC_PROJECTION in this stream at all

        let (result, _report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
        assert_eq!(result.0, initial_pack);
        assert_eq!(result.1, vec!["edit-0".to_string(), "edit-1".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_plan_skips_a_corrupt_snapshot_and_falls_back_to_initial_pack() {
        let write_options = WriteOptions { required_flags: REQUIRED_HASH_CHAIN, optional_flags: 0 };
        let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await.unwrap();
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;

        let doc_payload = crate::os_spr::history::encode_doc("doc-3", "schema-3", &mut dict);
        flush_dict_delta(&mut writer, &dict, &mut dict_base);
        writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0)).await.unwrap();

        // A snapshot whose stored body_hash does not match its body — must be treated as corrupt.
        let mut bad_record = sample_record(None, 0, SnapshotBodyKind::EmbeddedPack, Some(vec![1, 2, 3]));
        bad_record.await.body_hash = [0xFFu8; 32];
        writer.write_record(crate::os_spr::REC_PROJECTION, false, &encode_snapshot(&bad_record), CodecId(0)).await.unwrap();

        let payload = crate::os_spr::history::encode_edit(&sample_edit("edit-0", "op-0"), &mut dict, |_| None).await.unwrap();
        flush_dict_delta(&mut writer, &dict, &mut dict_base);
        writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
        writer.commit().await.unwrap();
        let bytes = writer.into_sink();

        let plan = resolve_plan(&bytes, b"INIT", MaterializeTarget::LatestOnActive, &ProtocolLimits::default()).await.unwrap();
        assert_eq!(plan.base.applied_edits, 0);
        assert_eq!(plan.skipped_corrupt, 1);
        match plan.base.bytes {
            BaseBytes::Borrowed(b) => assert_eq!(b, b"INIT"),
            BaseBytes::Sidecar { .. } => panic!("expected the initial pack fallback"),
        }
    }
    //#endregion 🔖️Plan

    //#region 🔖️Policy
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_policy_default_matches_documented_values() {
        let policy = CheckpointPolicy::default();
        assert_eq!(policy.every_edits, 512);
        assert_eq!(policy.every_bytes, 4 * 1024 * 1024);
        assert!(policy.on_checkpoint_commit);
        assert_eq!(policy.embed_below, 1024 * 1024);
    }

    #[semio_framework_async_macros::async_test]
    async fn checkpoint_policy_triggers_on_any_threshold() {
        let policy = CheckpointPolicy::default();
        assert!(policy.should_checkpoint(512, 0, false));
        assert!(policy.should_checkpoint(0, 4 * 1024 * 1024, false));
        assert!(policy.should_checkpoint(0, 0, true));
        assert!(!policy.should_checkpoint(1, 1, false));
        assert!(policy.should_embed(1024));
        assert!(!policy.should_embed(2 * 1024 * 1024));
    }
    //#endregion 🔖️Policy
}
//#endregion 🧪️Tests
