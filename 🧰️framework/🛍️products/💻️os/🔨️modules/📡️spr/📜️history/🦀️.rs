//! 🗂️ History stream verification, whole-file encoding and incremental hydration.
pub use semio_framework_os_history_schema::*;
pub use semio_framework_os_history_codec::*;
pub use semio_framework_os_history_record_decode::*;
#[path="🛬️decode/🗂️file/📚️frames/🦀️.rs"]
mod retained_history_source_frames;
pub use retained_history_source_frames::{RetainedHistorySourceFrames,RetainedHistorySourceFrame,RetainedHistorySourceFramesStep};
#[path="🛬️decode/🗂️file/🦀️.rs"]
mod retained_history_source;
pub use retained_history_source::{RetainedHistorySourceDecode,RetainedHistorySourcePartial,RetainedHistorySourceStep};
#[path="🛬️decode/🗂️file/🗜️deflate/🦀️.rs"]
mod retained_history_deflate;
pub use retained_history_deflate::{RetainedHistoryDeflateDecode,RetainedHistoryDeflatePayload,RetainedHistoryDeflateStep};

use semio_framework_os_pack::{ByteReader, ByteWriter, CodecId, PackSink};
use protocol::format::{Blake3Hasher, FrameCursor, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions, HEADER_SIZE};
use protocol::wire::{DictBuilder, DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use std::collections::{HashMap, HashSet};


// Whole-file compile: HistoryLog <-> .spr bytes, using protocol::format::SprWriter/FrameCursor.
//
// 🎯️ Design choice: dictionaries are flushed INCREMENTALLY (a REC_STR_DICT delta record right
// before whichever record first needed the new entries), not in a separate pre-pass — this keeps
// encode_history a single forward pass while still satisfying "dict records come before the
// records that reference them"; determinism (and therefore canonical-stability) falls out of the
// fact that DictBuilder's first-use interning order is itself deterministic.

#[derive(Clone, Debug)]
pub struct EncodeOptions {
    pub canonical: bool,
    pub write_backwards_section: bool,
    pub limits: ProtocolLimits,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { canonical: true, write_backwards_section: false, limits: ProtocolLimits::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DecodeOptions {
    pub verification: VerificationLevel,
    pub limits: ProtocolLimits,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedHistoryDecodeStep {
    Pending { completed_bytes: u64, total_bytes: u64, decoded_records: u64 },
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedHistoryDecodePhase {
    Verify,
    Decode,
    Validate,
    Ready,
    Fault,
}

pub struct RetainedHistoryDecode {
    verifier: Option<protocol::format::retained::RetainedSprVerification>,
    log: Option<HistoryLog>,
    dict: DictReader,
    edit_ids: Vec<String>,
    trusted_end: u64,
    offset: u64,
    decoded_records: u64,
    validation_stage: u8,
    validation_index: usize,
    saw_conflicts: bool,
    limits: protocol::format::retained::RetainedSprLimits,
    require_persisted_document: bool,
    phase: RetainedHistoryDecodePhase,
}

impl RetainedHistoryDecode {
    pub fn new(total_bytes: usize, limits: protocol::format::retained::RetainedSprLimits) -> Result<Self, String> {
        let verifier = protocol::format::retained::RetainedSprVerification::new(total_bytes as u64, limits).map_err(|error| format!("SPR history admission failed: {error:?}"))?;
        Ok(Self {
            verifier: Some(verifier),
            log: Some(HistoryLog::default()),
            dict: DictReader::new(),
            edit_ids: Vec::new(),
            trusted_end: 0,
            offset: HEADER_SIZE as u64,
            decoded_records: 0,
            validation_stage: 0,
            validation_index: 0,
            saw_conflicts: false,
            limits,
            require_persisted_document: false,
            phase: RetainedHistoryDecodePhase::Verify,
        })
    }

    pub fn new_persisted_document(total_bytes: usize, limits: protocol::format::retained::RetainedSprLimits) -> Result<Self, String> {
        let mut decoder = Self::new(total_bytes, limits)?;
        decoder.require_persisted_document = true;
        Ok(decoder)
    }

    pub fn step(&mut self, bytes: &[u8], maximum_bytes: usize, maximum_records: usize) -> Result<RetainedHistoryDecodeStep, String> {
        if self.phase == RetainedHistoryDecodePhase::Fault {
            return Err("SPR history decoder is faulted".into());
        }
        if self.phase == RetainedHistoryDecodePhase::Ready {
            return Ok(RetainedHistoryDecodeStep::Ready);
        }
        if bytes.len() as u64 > self.limits.file_bytes {
            self.phase = RetainedHistoryDecodePhase::Fault;
            return Err("SPR history exceeds retained file byte authority".into());
        }
        if self.phase == RetainedHistoryDecodePhase::Verify {
            let verifier = self.verifier.as_mut().ok_or_else(|| "SPR history verifier owner is absent".to_string())?;
            if verifier.consumed() > bytes.len() as u64 {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history input changed during retained verification".into());
            }
            let start = verifier.consumed() as usize;
            let end = start.saturating_add(maximum_bytes).min(bytes.len());
            let mut fuel = end - start;
            if fuel != 0 {
                if let Err(error) = verifier.push(&bytes[start..end], &mut fuel) {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    return Err(format!("SPR history verification failed: {error:?}"));
                }
            }
            if verifier.consumed() != bytes.len() as u64 {
                return Ok(self.progress(bytes.len()));
            }
            let span = match verifier.finish() {
                Ok(span) => span,
                Err(error) => {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    return Err(format!("SPR history verification failed: {error:?}"));
                }
            };
            if span.sequence() == 0 || span.end() != bytes.len() as u64 || span.tail() != 0 {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history requires one exact committed span without a torn or uncommitted tail".into());
            }
            self.trusted_end = span.end();
            self.verifier = None;
            self.phase = RetainedHistoryDecodePhase::Decode;
            if maximum_records == 0 {
                return Ok(self.progress(bytes.len()));
            }
        }

        if self.phase == RetainedHistoryDecodePhase::Validate {
            for _ in 0..maximum_records {
                match self.validate_one() {
                    Ok(true) => {
                        self.phase = RetainedHistoryDecodePhase::Ready;
                        return Ok(RetainedHistoryDecodeStep::Ready);
                    }
                    Ok(false) => {}
                    Err(error) => {
                        self.phase = RetainedHistoryDecodePhase::Fault;
                        return Err(error);
                    }
                }
            }
            return Ok(self.progress(bytes.len()));
        }

        let trusted_end = usize::try_from(self.trusted_end).map_err(|_| "SPR history trusted span exceeds this platform".to_string())?;
        if trusted_end > bytes.len() {
            self.phase = RetainedHistoryDecodePhase::Fault;
            return Err("SPR history input shortened after retained verification".into());
        }
        for _ in 0..maximum_records {
            if self.offset == self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Validate;
                return Ok(self.progress(bytes.len()));
            }
            if self.offset < HEADER_SIZE as u64 || self.offset > self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history semantic cursor escaped its verified span".into());
            }
            let mut cursor = ::semio_framework_async::poll::resolve_ready(FrameCursor::new(&bytes[..trusted_end], self.offset));
            let frame = ::semio_framework_async::poll::resolve_ready(cursor.next_frame())
                .map_err(|error| {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    format!("SPR history record framing failed: {error}")
                })?
                .ok_or_else(|| {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    "SPR history ended before its verified span".to_string()
                })?;
            let frame_len = ::semio_framework_async::poll::resolve_ready(frame.frame_len());
            if frame_len > self.limits.frame_body_bytes.saturating_add(18) {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history record exceeds retained frame byte authority".into());
            }
            let next = frame.offset.checked_add(frame_len).ok_or_else(|| "SPR history record offset overflowed".to_string())?;
            if next > self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history record crossed its verified span".into());
            }
            self.decode_frame(frame).map_err(|error| {
                self.phase = RetainedHistoryDecodePhase::Fault;
                error
            })?;
            self.offset = next;
            self.decoded_records = self.decoded_records.checked_add(1).ok_or_else(|| "SPR history decoded record count overflowed".to_string())?;
            if self.decoded_records > self.limits.records {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history exceeds retained record authority".into());
            }
        }
        Ok(self.progress(bytes.len()))
    }

    fn validate_one(&mut self) -> Result<bool, String> {
        let log = self.log.as_ref().ok_or_else(|| "SPR history log owner is absent".to_string())?;
        match self.validation_stage {
            0 if self.validation_index < log.edits.len() => {
                let index = self.validation_index;
                let edit = &log.edits[index];
                if edit.id.trim().is_empty() || log.edits.iter().take(index).any(|prior| prior.id == edit.id) {
                    return Err(format!("SPR history repeats or omits authoritative edit {}", edit.id));
                }
                if self.require_persisted_document && edit.meta.is_none() {
                    return Err(format!("SPR history edit {} has no authoritative operation metadata", edit.id));
                }
                if edit.meta.as_ref().is_some_and(|meta| meta.len() != edit.ops.len()) || edit.ops.iter().chain(&edit.inverse).any(|payload| payload.binary.is_none() && payload.text.is_none()) {
                    return Err(format!("SPR history edit {} has malformed operations or metadata", edit.id));
                }
                self.validation_index += 1;
                Ok(false)
            }
            0 => {
                self.validation_stage = 1;
                self.validation_index = 0;
                Ok(false)
            }
            1 if self.validation_index < log.transitions.len() => {
                let index = self.validation_index;
                let transition = &log.transitions[index];
                if transition.id.trim().is_empty() || transition.actor.trim().is_empty() || log.transitions[..index].iter().any(|prior| prior.id == transition.id) {
                    return Err(format!("SPR history repeats or omits authoritative transition {}", transition.id));
                }
                if protocol::decode_history_transition(&transition.payload).is_err() {
                    return Err(format!("SPR history transition {} has a malformed payload", transition.id));
                }
                self.validation_index += 1;
                Ok(false)
            }
            1 => {
                self.validation_stage = 2;
                self.validation_index = 0;
                Ok(false)
            }
            _ => Ok(true),
        }
    }

    fn decode_frame(&mut self, frame: protocol::RecordFrame<'_>) -> Result<(), String> {
        let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
        let log = self.log.as_mut().ok_or_else(|| "SPR history log owner is absent".to_string())?;
        match frame.kind {
            protocol::REC_STR_DICT => ::semio_framework_async::poll::resolve_ready(apply_dict_record(&mut self.dict, payload)).map_err(|error| error.to_string())?,
            protocol::REC_ACTOR_DICT => {}
            protocol::REC_DOC => {
                let (doc_id, schema) = ::semio_framework_async::poll::resolve_ready(decode_doc(payload, &self.dict)).map_err(|error| error.to_string())?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            protocol::REC_EDIT => {
                let edit_ids = &self.edit_ids;
                let edit = ::semio_framework_async::poll::resolve_ready(decode_edit(payload, &self.dict, |ordinal| edit_ids.get(ordinal as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ordinal as u32))))
                    .map_err(|error| error.to_string())?;
                self.edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            REC_TRANSITION => log.transitions.push(::semio_framework_async::poll::resolve_ready(decode_transition(payload, &self.dict)).map_err(|error| error.to_string())?),
            REC_COMPOSITION => log.composition = Some(::semio_framework_async::poll::resolve_ready(decode_composition(payload, &self.dict)).map_err(|error| error.to_string())?),
            REC_CONFLICT => {
                if self.saw_conflicts {
                    return Err("SPR history repeats its conflict record".into());
                }
                let edit_ids = &self.edit_ids;
                log.conflicts = ::semio_framework_async::poll::resolve_ready(decode_conflicts(payload, &self.dict, |ordinal| edit_ids.get(ordinal as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ordinal as u32))))
                    .map_err(|error| error.to_string())?;
                self.saw_conflicts = true;
            }
            REC_VIEWER => {
                let (viewer_line, viewer_checkpoint) = ::semio_framework_async::poll::resolve_ready(decode_viewer(payload, &self.dict)).map_err(|error| error.to_string())?;
                log.viewer_line = viewer_line;
                log.viewer_checkpoint = viewer_checkpoint;
            }
            _ => {}
        }
        Ok(())
    }

    fn progress(&self, total_bytes: usize) -> RetainedHistoryDecodeStep {
        let completed_bytes = self.verifier.as_ref().map_or(self.offset, protocol::format::retained::RetainedSprVerification::consumed);
        RetainedHistoryDecodeStep::Pending { completed_bytes, total_bytes: total_bytes as u64, decoded_records: self.decoded_records }
    }

    pub fn take_ready(&mut self) -> Option<HistoryLog> {
        if self.phase != RetainedHistoryDecodePhase::Ready {
            return None;
        }
        self.log.take()
    }

    pub fn take_partial(&mut self) -> Option<HistoryLog> {
        self.phase = RetainedHistoryDecodePhase::Fault;
        self.verifier = None;
        self.log.take()
    }

    pub fn take_auxiliary_owners(&mut self) -> (Vec<String>, Vec<String>) {
        (self.dict.take_entries(), std::mem::take(&mut self.edit_ids))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.verifier.is_none() && self.log.is_none() && self.dict.is_empty() && self.edit_ids.is_empty()
    }
}

impl Drop for RetainedHistoryDecode {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained history decoder reached Drop before every parsed and auxiliary owner was transferred");
    }
}

async fn flush_dict_delta<S: PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> where ProtocolError: From<S::Error> {
    let len = dict.len();
    if len > *base {
        let entries = dict.entries_since(*base);
        let mut payload = ByteWriter::new();
        payload.write_u8(1);
        payload.write_varint_u64(*base as u64);
        payload.write_varint_u64(entries.len() as u64);
        for entry in entries {
            payload.write_varint_u64(entry.len() as u64);
            payload.write_bytes(entry.as_bytes());
        }
        writer.write_record(protocol::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0)).await?;
        *base = len;
    }
    Ok(())
}

/// 🎞️ `(commit_seq, chain_hash)` out of a `REC_COMMIT` frame's payload — thin wrapper over
/// `protocol::format::parse_commit_payload` (public since that crate's own follow-up review pass)
/// so `VerificationLevel::Full`'s chain recompute doesn't need this crate's own byte-offset copy.
async fn parse_commit_fields(payload: &[u8]) -> Result<(u64, [u8; 32]), ProtocolError> {
    let commit = protocol::format::parse_commit_payload(payload)?;
    Ok((commit.commit_seq, commit.chain_hash))
}

pub async fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    if log.edits.len() as u64 > options.limits.max_record_count {
        return Err(ProtocolError::LimitExceeded("edit count exceeds ProtocolLimits::max_record_count"));
    }
    let write_options = WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: if options.canonical { protocol::OPTIONAL_CANONICAL } else { 0 } };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await?;
    let mut dict = DictBuilder::new();
    let mut dict_base = 0u32;

    let doc_payload = encode_doc(&log.doc_id, &log.schema, &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
    writer.write_record(protocol::REC_DOC, true, &doc_payload, CodecId(0)).await?;

    // 🎯️ Built incrementally (an edit's own id is inserted only AFTER it is encoded), matching
    // `HistoryAppender::append_edit`'s streaming semantics and the decoder's causal resolution
    // (`EditIter`/`prescan_full` only ever know edits already decoded). A one-shot, whole-list
    // `ordinals` map would let an edit's own `mutation_meta[i].mutation_id` — legitimately equal
    // to `edit.id` for a single-op edit — resolve to a self-referencing ordinal at encode time,
    // which the decoder can never resolve (it hasn't registered the current edit's id yet).
    let mut ordinals: HashMap<&str, u64> = HashMap::new();
    for (index, edit) in log.edits.iter().enumerate() {
        // 🎯️ `write_backwards_section` is a batch-level policy switch: even when `edit.inverse`
        // is populated (e.g. by a live store that always computes it), a caller can opt out of
        // persisting it here. `HistoryAppender::append_edit` has no such switch — its streaming,
        // one-edit-at-a-time API gives the caller direct per-edit control via the data itself.
        let payload = if options.write_backwards_section {
            encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).await?
        } else {
            let stripped = HistoryEdit { inverse: Vec::new(), ..edit.clone() };
            encode_edit(&stripped, &mut dict, |id| ordinals.get(id).copied()).await?
        };
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(protocol::REC_EDIT, true, &payload, CodecId(0)).await?;
        ordinals.insert(edit.id.as_str(), index as u64);
    }
    for transition in &log.transitions {
        let payload = encode_transition(transition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await?;
    }

    if let Some(composition) = &log.composition {
        let composition_payload = encode_composition(composition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_COMPOSITION, false, &composition_payload, CodecId(0)).await?;
    }

    if !log.conflicts.is_empty() {
        let conflicts_payload = encode_conflicts(&log.conflicts, &mut dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_CONFLICT, false, &conflicts_payload, CodecId(0)).await?;
    }
    if log.viewer_line.is_some() || log.viewer_checkpoint.is_some() {
        let viewer_payload = encode_viewer(log.viewer_line.as_deref(), log.viewer_checkpoint.as_deref(), &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_VIEWER, false, &viewer_payload, CodecId(0)).await?;
    }

    writer.commit().await?;
    Ok(writer.into_sink().await)
}

async fn decode_history_from(trusted: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids: Vec<String> = Vec::new();
    let mut log = HistoryLog::default();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    let hasher = Blake3Hasher;
    let full = options.verification == VerificationLevel::Full;
    let mut running_chain = if full { hasher.hash(&trusted[..HEADER_SIZE]) } else { [0u8; 32] };
    let mut pending_digests: Vec<[u8; 32]> = Vec::new();
    let mut saw_conflicts = false;

    while let Some(frame) = cursor.next_frame().await? {
        if full && frame.kind != protocol::REC_COMMIT {
            let frame_bytes = &trusted[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
            pending_digests.push(hasher.hash(frame_bytes));
        }
        match frame.kind {
            protocol::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            protocol::REC_ACTOR_DICT => {} // v1 never splits an actor dictionary — see 🔖️Payloads note
            protocol::REC_DOC => {
                let (doc_id, schema) = decode_doc(frame.payload().await, &dict).await?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            protocol::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let edit = decode_edit(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            REC_TRANSITION => log.transitions.push(decode_transition(frame.payload().await, &dict).await?),
            REC_COMPOSITION => log.composition = Some(decode_composition(frame.payload().await, &dict).await?),
            REC_CONFLICT => {
                if saw_conflicts {
                    return Err(ProtocolError::Malformed { what: "history", offset: frame.offset, detail: "duplicate conflict record".to_string() });
                }
                let edit_ids_ref = &edit_ids;
                log.conflicts = decode_conflicts(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                saw_conflicts = true;
            }
            REC_VIEWER => {
                let (viewer_line, viewer_checkpoint) = decode_viewer(frame.payload().await, &dict).await?;
                log.viewer_line = viewer_line;
                log.viewer_checkpoint = viewer_checkpoint;
            }
            protocol::REC_COMMIT if full => {
                let (commit_seq, chain_hash) = parse_commit_fields(frame.payload().await).await?;
                let mut concat = running_chain.to_vec();
                for digest in &pending_digests {
                    concat.extend_from_slice(digest);
                }
                let recomputed = hasher.hash(&concat);
                if recomputed != chain_hash {
                    return Err(ProtocolError::ChainMismatch { commit_seq });
                }
                running_chain = recomputed;
                pending_digests.clear();
            }
            // Every other kind (REC_PROJECTION, REC_INDEX, REC_FRONTIER, extension range, ...) is
            // foreign to this crate's semantic layer — skip regardless of the frame's critical bit;
            // this reader only enforces criticality for its own known kind set handled above.
            _ => {}
        }
    }
    Ok(log)
}

pub async fn decode_history(bytes: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    HistoryReader::open(bytes, options).await?.log().await
}
//#endregion 🔖️Codec

//#region 🔖️Append
// Streaming append API over protocol::format::SprWriter — the hot path. One edit -> one REC_EDIT
// frame, O(new edit) allocation.
pub struct HistoryAppender<S: PackSink> {
    writer: SprWriter<S>,
    dict: DictBuilder,
    dict_base: u32,
    edit_ordinals: HashMap<String, u64>,
    next_edit_ordinal: u64,
}

impl<S: PackSink> HistoryAppender<S> where ProtocolError: From<S::Error> {
    pub async fn begin(sink: S, doc_id: &str, schema: &str, options: &WriteOptions) -> Result<Self, ProtocolError> {
        let mut writer = SprWriter::begin(sink, options).await?;
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;
        let payload = encode_doc(doc_id, schema, &mut dict).await;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(protocol::REC_DOC, true, &payload, CodecId(0)).await?;
        Ok(Self { writer, dict, dict_base, edit_ordinals: HashMap::new(), next_edit_ordinal: 0 })
    }

    pub async fn append_edit(&mut self, edit: &HistoryEdit) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_edit(edit, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        let offset = self.writer.write_record(protocol::REC_EDIT, true, &payload, CodecId(0)).await?;
        self.edit_ordinals.insert(edit.id.clone(), self.next_edit_ordinal);
        self.next_edit_ordinal += 1;
        Ok(offset)
    }

    /// 🔀️ Appends one `REC_TRANSITION` frame (critical); returns its offset.
    pub async fn append_transition(&mut self, transition: &HistoryTransitionRecord) -> Result<u64, ProtocolError> {
        let payload = encode_transition(transition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await
    }

    /// 🧩️ Appends the composition overlay record (skippable extension); returns its offset.
    pub async fn append_composition(&mut self, composition: &HistoryComposition) -> Result<u64, ProtocolError> {
        let payload = encode_composition(composition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_COMPOSITION, false, &payload, CodecId(0)).await
    }

    /// ⚔️ Appends the log's one conflict record, resolving `Degraded` edit references against
    /// the edits appended so far; a file carries at most one such record.
    pub async fn append_conflicts(&mut self, conflicts: &[HistoryConflict]) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_conflicts(conflicts, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_CONFLICT, false, &payload, CodecId(0)).await
    }

    /// 👁 Appends this replica's head. Both fields absent means the canonical trunk tip, and the record is omitted.
    pub async fn append_viewer(&mut self, line: Option<&str>, checkpoint: Option<&str>) -> Result<u64, ProtocolError> {
        let payload = encode_viewer(line, checkpoint, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_VIEWER, false, &payload, CodecId(0)).await
    }

    pub async fn commit(&mut self) -> Result<u64, ProtocolError> {
        self.writer.commit().await
    }

    pub async fn into_sink(self) -> S {
        self.writer.into_sink().await
    }
}
//#endregion 🔖️Append

//#region 🔖️Scan
// Read-side over a byte buffer, via protocol_format cursors. `open` establishes a trusted byte
// range via `protocol::format::recover` (RecoveryMode::LastCommit) once; every subsequent
// operation stays within that range, so a torn tail can never surface a partially-written record.
pub struct HistoryReader<'a> {
    trusted: &'a [u8],
    options: DecodeOptions,
}

impl<'a> HistoryReader<'a> {
    pub async fn open(bytes: &'a [u8], options: &DecodeOptions) -> Result<Self, ProtocolError> {
        let recovery = protocol::format::recover(&bytes, &options.limits, RecoveryMode::LastCommit).await?;
        let trusted = &bytes[..recovery.bytes_recovered as usize];
        Ok(Self { trusted, options: options.clone() })
    }

    pub async fn log(&self) -> Result<HistoryLog, ProtocolError> {
        decode_history_from(self.trusted, &self.options).await
    }

    pub async fn edits(&self) -> EditIter<'a> {
        EditIter { cursor: FrameCursor::new(self.trusted, HEADER_SIZE as u64).await, dict: DictReader::new(), edit_ids: Vec::new() }
    }

    pub async fn edits_rev(&self, limit: usize) -> RevEditIter<'a> {
        match prescan_full(self.trusted).await {
            Ok((dict, edit_ids)) => RevEditIter { state: Ok(RevEditIterReady { cursor: ReverseFrameCursor::at_end(&self.trusted[HEADER_SIZE..]).await, dict, edit_ids, remaining: limit }) },
            Err(e) => RevEditIter { state: Err(Some(e)) },
        }
    }
}

pub struct EditIter<'a> {
    cursor: FrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
}

impl<'a> Iterator for EditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — `Iterator::next` is an externally-declared trait method, so it must stay a
    // plain sync `fn`. Its callees (`next_frame`/`payload`/`apply_dict_record`/`decode_edit`) are
    // pure in-memory byte-buffer parsing with no real suspension point, so they are resolved via
    // the crate's one sanctioned E5 bridge (`io::resolve_ready`) rather than awaited.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match ::semio_framework_async::poll::resolve_ready(self.cursor.next_frame()) {
                Ok(Some(frame)) => match frame.kind {
                    protocol::REC_STR_DICT => {
                        let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                        if let Err(e) = ::semio_framework_async::poll::resolve_ready(apply_dict_record(&mut self.dict, payload)) {
                            return Some(Err(e));
                        }
                    }
                    protocol::REC_EDIT => {
                        let edit_ids_ref = &self.edit_ids;
                        let dict_ref = &self.dict;
                        let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                        let result = ::semio_framework_async::poll::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                        match result {
                            Ok(edit) => {
                                self.edit_ids.push(edit.id.clone());
                                return Some(Ok(edit));
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    _ => continue,
                },
                Ok(None) => return None,
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

struct RevEditIterReady<'a> {
    cursor: ReverseFrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
    remaining: usize,
}

/// 🚧️ `edits_rev` cannot return `Result` per the frozen contract signature, so a prescan
/// failure (needed to build the full dict + edit-id table up front — see `prescan_full`) is
/// deferred: the first `next()` call yields it, every call after that yields `None`.
pub struct RevEditIter<'a> {
    state: Result<RevEditIterReady<'a>, Option<ProtocolError>>,
}

impl<'a> Iterator for RevEditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — same rationale as `EditIter::next` above: `Iterator::next` must stay sync, its
    // callees are pure in-memory parsing, resolved via the crate's one `io::resolve_ready` bridge.
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Err(pending) => pending.take().map(Err),
            Ok(ready) => {
                if ready.remaining == 0 {
                    return None;
                }
                loop {
                    match ::semio_framework_async::poll::resolve_ready(ready.cursor.prev_frame()) {
                        Ok(Some(frame)) => {
                            if frame.kind == protocol::REC_EDIT {
                                let edit_ids_ref = &ready.edit_ids;
                                let dict_ref = &ready.dict;
                                let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                                let result = ::semio_framework_async::poll::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                                ready.remaining -= 1;
                                return Some(result);
                            }
                        }
                        Ok(None) => return None,
                        Err(e) => {
                            ready.remaining = 0;
                            return Some(Err(e));
                        }
                    }
                }
            }
        }
    }
}

/// 🔎️ Builds the FULL dictionary and the FULL forward-ordered edit-id table in one forward
/// pass. Safe to reuse for decoding any earlier record: dict indices and edit ordinals are both
/// append-only and stable once assigned, so the final state is a superset valid at every offset.
async fn prescan_full(trusted: &[u8]) -> Result<(DictReader, Vec<String>), ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids = Vec::new();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await? {
        match frame.kind {
            protocol::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            protocol::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = decode_edit(frame.payload().await, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id);
            }
            _ => {}
        }
    }
    Ok((dict, edit_ids))
}
//#endregion 🔖️Scan

//#region 🔖️Frontier
#[derive(Clone, Debug, PartialEq)]
pub struct AlternativeHead {
    pub alternative_id: String,
    pub checkpoint_id: String,
    pub head_edit_ordinal: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrontierSummary {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub alternatives: Vec<AlternativeHead>,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// 🧭️ Compares two frontiers by head edit ordinal, then head edit id/chain hash for the
/// equal-ordinal case. `Diverged::common_edit_count` is a conservative estimate (the shared
/// ordinal itself) — `FrontierSummary` alone carries no shared-ancestry data to verify a true
/// common ancestor; documented "your choice" per the contract.
pub async fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
    if local.head_edit_ordinal == remote.head_edit_ordinal && local.head_edit_id == remote.head_edit_id && local.chain_hash == remote.chain_hash {
        return FrontierComparison::Equal;
    }
    if local.head_edit_ordinal > remote.head_edit_ordinal {
        return FrontierComparison::Ahead;
    }
    if local.head_edit_ordinal < remote.head_edit_ordinal {
        return FrontierComparison::Behind;
    }
    FrontierComparison::Diverged { common_edit_count: local.head_edit_ordinal }
}
//#endregion 🔖️Frontier

//#region 🔖️Index
// Advisory REC_INDEX payload sections. Rebuildable from scan; never authoritative. Byte layout is
// this crate's own choice (the contract only fixes the section-kind bytes): format u8, then zero
// or more `(section_kind: u8, count: varint, count x entry)` sections back-to-back.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[cfg(test)]
#[path="🧪️testing/♻️retirement/🦀️.rs"]
mod test_retirement;
