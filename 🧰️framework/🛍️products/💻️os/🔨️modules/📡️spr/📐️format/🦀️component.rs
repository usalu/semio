//! 🎞️ Protocol `.spr` binary container: an append-only, live record-stream format distinct from
//! `pack`'s write-once footer-rooted `.spk` files — a `.spr` file's EOF can be torn at any moment,
//! so trust is rooted in a hash-chained sequence of commit frames rather than a trailing footer.
//! Owns record framing (uniform frame grammar for every record kind, including commits), the
//! commit-chain hash algorithm, zero-copy forward/reverse frame cursors, the streaming writer, and
//! crash-recovery (fast commit-chain probe, falling back to a bounded forward scan). Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_format`).
//!
//! Design note (kind-agnostic cursors): this crate never interprets what a record `kind` byte
//! *means* beyond `REC_COMMIT` (which it must parse to maintain the chain) — deciding whether an
//! unrecognized kind should be silently skipped or treated as a fatal `UnknownCriticalRecord` is a
//! semantic-layer concern (`protocol_history` knows which kinds it decodes; this crate does not).
//! `FrameCursor`/`ReverseFrameCursor` therefore surface every structurally valid frame uniformly,
//! regardless of `kind`, giving callers `RecordFrame::kind` and `flags & FRAME_FLAG_CRITICAL` to
//! implement that policy themselves.

use crate::os_pack::{CompressionCodec, PackError, PackSink, PackSource};
use crate::os_spr::wire::{frame_flags, ProtocolError, ProtocolLimits, RecordHasher, FRAME_FLAG_COMPRESSED, FRAME_FLAG_CRITICAL};

//#region 🔖️Header
/// @emoji 🧲️ The 8-byte magic every `.spr` file begins with — distinct from pack's `.spk` magic
/// (own root-of-trust model; see the module doc).
pub const MAGIC: [u8; 8] = [0x89, b'S', b'P', b'R', 0x0D, 0x0A, 0x1A, 0x0A];
/// @emoji 📏️ Fixed wire size of the header, in bytes.
pub const HEADER_SIZE: usize = 32;
/// @emoji 🔢️ The container format version this crate writes and reads.
pub const FORMAT_VERSION_MAJOR: u16 = 1;
/// @emoji 🔢️ The container format minor version this crate writes.
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// @emoji 🎭️ The union of `crate::os_spr::REQUIRED_*` bits this crate understands; any bit outside
/// this mask makes a header (and therefore the whole file) unreadable.
const REQUIRED_KNOWN_MASK: u32 = crate::os_spr::REQUIRED_HASH_CHAIN | crate::os_spr::REQUIRED_SIGNED | crate::os_spr::REQUIRED_ENCRYPTED;

/// @emoji ✍️ Serializes the 32-byte header: magic, version, flags, `header_crc32` over bytes
/// `0..20` (CRC-32C, `crate::os_pack::crc32c`), 8 reserved zero bytes.
fn build_header_bytes(required_flags: u32, optional_flags: u32) -> [u8; HEADER_SIZE] {
    let mut buf = [0u8; HEADER_SIZE];
    buf[0..8].copy_from_slice(&MAGIC);
    buf[8..10].copy_from_slice(&FORMAT_VERSION_MAJOR.to_le_bytes());
    buf[10..12].copy_from_slice(&FORMAT_VERSION_MINOR.to_le_bytes());
    buf[12..16].copy_from_slice(&required_flags.to_le_bytes());
    buf[16..20].copy_from_slice(&optional_flags.to_le_bytes());
    let crc = crate::os_pack::crc32c(&buf[0..20]);
    buf[20..24].copy_from_slice(&crc.to_le_bytes());
    buf
}

/// @emoji 📖️ Validates a source's 32-byte header in place: magic, self-CRC, `required_flags`
/// restricted to `REQUIRED_KNOWN_MASK` (0..=2), `version_major == 1`. Every failure mode reuses a
/// `crate::os_pack::PackError` variant wrapped in `ProtocolError::Pack` — all are directly constructible
/// (no protocol_core amendment needed, unlike the contract's fallback-deviation clause anticipated).
fn validate_header<S: PackSource>(source: &S) -> Result<(), ProtocolError> {
    let total_len = source.len();
    if total_len < HEADER_SIZE as u64 {
        return Err(ProtocolError::Pack(PackError::Truncated(total_len)));
    }
    let mut buf = [0u8; HEADER_SIZE];
    source.read_exact_at(0, &mut buf)?;
    if buf[0..8] != MAGIC {
        return Err(ProtocolError::Pack(PackError::BadMagic));
    }
    let version_major = u16::from_le_bytes([buf[8], buf[9]]);
    let version_minor = u16::from_le_bytes([buf[10], buf[11]]);
    let required_flags = u32::from_le_bytes(buf[12..16].try_into().unwrap());
    let stored_crc = u32::from_le_bytes(buf[20..24].try_into().unwrap());
    let computed_crc = crate::os_pack::crc32c(&buf[0..20]);
    if stored_crc != computed_crc {
        return Err(ProtocolError::Pack(PackError::ChecksumMismatch { segment: "header", offset: 20 }));
    }
    let unknown = required_flags & !REQUIRED_KNOWN_MASK;
    if unknown != 0 {
        return Err(ProtocolError::Pack(PackError::UnknownRequiredFlags(unknown)));
    }
    if version_major != FORMAT_VERSION_MAJOR {
        return Err(ProtocolError::Pack(PackError::UnsupportedVersion { major: version_major, minor: version_minor }));
    }
    Ok(())
}

/// @emoji 🪪️ A header's decoded fields — exposed for downstream crates (`protocol_io`'s
/// `HistoryFile`, `protocol_cli`'s `inspect`/`verify` subcommands) that need to inspect a `.spr`
/// file's format version/flags without re-deriving this crate's private byte layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub optional_flags: u32,
}

/// @emoji 📖️ Validates (see `validate_header`) and returns a source's 32-byte header fields.
pub fn read_header<S: PackSource>(source: &S) -> Result<Header, ProtocolError> {
    validate_header(source)?;
    let mut buf = [0u8; HEADER_SIZE];
    source.read_exact_at(0, &mut buf)?;
    Ok(Header {
        version_major: u16::from_le_bytes([buf[8], buf[9]]),
        version_minor: u16::from_le_bytes([buf[10], buf[11]]),
        required_flags: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
        optional_flags: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
    })
}
//#endregion 🔖️Header

//#region 🔖️Frame
/// @emoji 🔢️ The wire width, in bytes, of `value` encoded as an unsigned LEB128 varint.
fn varint_width(value: u64) -> u64 {
    let mut buf = Vec::with_capacity(10);
    crate::os_pack::write_varint_u64(&mut buf, value);
    buf.len() as u64
}

/// @emoji 🚨️ Builds a `crate::os_spr::ProtocolError::Malformed` for this crate's own structural
/// checks (distinct from `crate::os_pack::PackError`-wrapped errors, which cover pack-primitive-level
/// failures like truncation/varint overflow).
fn malformed(what: &'static str, offset: u64, detail: impl Into<String>) -> ProtocolError {
    ProtocolError::Malformed { what, offset, detail: detail.into() }
}

/// @emoji 🖇️ Frames one record into `scratch` (cleared first, capacity reused across calls — the
/// writer's hot-path scratch buffer): `body_len varint, kind, flags, [raw_len varint], payload,
/// crc32c, back_len`. `crc32c` covers `kind..payload` inclusive; `back_len` covers the WHOLE frame
/// including the leading `body_len` varint and its own 4 bytes.
///
/// 🎯️ Design choice: builds one contiguous buffer and issues a single `PackSink::write_all` at the
/// call site, mirroring `crate::os_pack::format::encode_segment`'s proven pattern (the contract's own cited
/// living example) — `crate::os_pack::crc32c` only accepts one contiguous slice, and this crate has no
/// incremental CRC-32C primitive to stream a crc across a caller-owned payload slice without a
/// buffer of its own. The scratch buffer is still reused call-to-call, avoiding the reallocation
/// pack_format's segment encoder pays every time.
fn encode_frame_into(scratch: &mut Vec<u8>, kind: u8, flags: u8, raw_len: Option<u64>, stored_payload: &[u8]) -> Result<(), ProtocolError> {
    scratch.clear();
    let raw_len_width = raw_len.map_or(0, varint_width);
    let body_len = 2 + raw_len_width + stored_payload.len() as u64;
    crate::os_pack::write_varint_u64(scratch, body_len);
    let body_start = scratch.len();
    scratch.push(kind);
    scratch.push(flags);
    if let Some(rl) = raw_len {
        crate::os_pack::write_varint_u64(scratch, rl);
    }
    scratch.extend_from_slice(stored_payload);
    let crc = crate::os_pack::crc32c(&scratch[body_start..]);
    scratch.extend_from_slice(&crc.to_le_bytes());
    let back_len_u64 = scratch.len() as u64 + 4;
    let back_len: u32 = back_len_u64.try_into().map_err(|_| ProtocolError::LimitExceeded("frame exceeds u32 back_len (4 GiB cap)"))?;
    scratch.extend_from_slice(&back_len.to_le_bytes());
    Ok(())
}

/// @emoji 📦️ One parsed, CRC-verified record frame borrowed zero-copy from the buffer it was read
/// from. `stored` is exactly the on-disk payload bytes — compressed iff `flags & FRAME_FLAG_COMPRESSED`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordFrame<'a> {
    pub kind: u8,
    pub flags: u8,
    pub offset: u64,
    pub stored: &'a [u8],
    pub raw_len: Option<u64>,
}

impl<'a> RecordFrame<'a> {
    /// @emoji 📤️ The on-disk payload bytes: identical to the caller's original data iff
    /// `!compressed`; otherwise the caller must decompress via a `crate::os_pack::CompressionCodec`
    /// keyed by `crate::os_spr::frame_codec_id(self.flags)`, targeting `self.raw_len` bytes.
    pub fn payload(&self) -> &'a [u8] {
        self.stored
    }

    /// @emoji 📏️ Total on-disk bytes this frame occupies (the value its trailing `back_len` field
    /// carries), recomputed from the visible fields rather than cached — every input to the
    /// computation is already public state, so there is nothing to desync.
    pub fn frame_len(&self) -> u64 {
        let raw_len_width = self.raw_len.map_or(0, varint_width);
        let body_len = 2 + raw_len_width + self.stored.len() as u64;
        varint_width(body_len) + body_len + 8
    }
}

/// @emoji 👓️ Zero-copy forward parse of one frame starting at `bytes[pos..]`. Validates `body_len`
/// bounds, `crc32c` (over `kind..payload`), and the `back_len` self-echo before returning. Returns
/// `(frame, next_pos)` so callers (both cursors below) can advance without recomputing `frame_len`.
fn decode_frame_in_slice(bytes: &[u8], pos: usize) -> Result<(RecordFrame<'_>, usize), ProtocolError> {
    let mut cursor = pos;
    let body_len = crate::os_pack::read_varint_u64(bytes, &mut cursor)?;
    if body_len < 2 {
        return Err(malformed("frame body_len", pos as u64, "body_len smaller than kind+flags (2 bytes)"));
    }
    let body_start = cursor;
    let body_end = body_start.checked_add(body_len as usize).ok_or(ProtocolError::LimitExceeded("frame body_len overflows usize"))?;
    let trailer_end = body_end.checked_add(8).ok_or(ProtocolError::LimitExceeded("frame trailer offset overflows usize"))?;
    if trailer_end > bytes.len() {
        return Err(ProtocolError::Pack(PackError::Truncated(body_start as u64)));
    }
    let body = &bytes[body_start..body_end];
    let kind = body[0];
    let flags = body[1];
    let compressed = flags & FRAME_FLAG_COMPRESSED != 0;
    let mut payload_start = 2usize;
    let raw_len = if compressed {
        let mut p = payload_start;
        let v = crate::os_pack::read_varint_u64(body, &mut p)?;
        payload_start = p;
        Some(v)
    } else {
        None
    };
    let stored = &body[payload_start..];
    let stored_crc = u32::from_le_bytes(bytes[body_end..body_end + 4].try_into().unwrap());
    let computed_crc = crate::os_pack::crc32c(body);
    if stored_crc != computed_crc {
        return Err(ProtocolError::Pack(PackError::ChecksumMismatch { segment: "frame", offset: body_end as u64 }));
    }
    let back_len = u32::from_le_bytes(bytes[body_end + 4..trailer_end].try_into().unwrap()) as usize;
    let frame_len = trailer_end - pos;
    if back_len != frame_len {
        return Err(ProtocolError::FrameFraming(pos as u64));
    }
    Ok((RecordFrame { kind, flags, offset: pos as u64, stored, raw_len }, trailer_end))
}

/// @emoji ➡️ Forward, zero-copy iteration over a contiguous record stream (a whole `.spr` buffer,
/// or any already-decompressed span of ordinary frames — e.g. an inflated `REC_SEALED` batch).
pub struct FrameCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> FrameCursor<'a> {
    /// @emoji 🚀️ Positions a cursor over `bytes`, ready to read the frame starting at `start_offset`
    /// (typically `HEADER_SIZE` for a whole file).
    pub fn new(bytes: &'a [u8], start_offset: u64) -> Self {
        Self { bytes, pos: start_offset as usize }
    }

    /// @emoji ⏭️ Parses and returns the next frame, or `None` at exact end-of-buffer. A short
    /// leftover (fewer bytes than a minimal frame needs) is a real parse error, not `None` — only
    /// `recover`'s forward scan is expected to encounter a torn tail and must stop before it.
    pub fn next_frame(&mut self) -> Result<Option<RecordFrame<'a>>, ProtocolError> {
        if self.pos >= self.bytes.len() {
            return Ok(None);
        }
        let (frame, next_pos) = decode_frame_in_slice(self.bytes, self.pos)?;
        self.pos = next_pos;
        Ok(Some(frame))
    }
}

/// @emoji ⬅️ Backward, zero-copy, O(1)-per-step iteration over a contiguous record stream, walking
/// via each frame's trailing `back_len` rather than re-scanning from the start.
///
/// Unlike `FrameCursor::new`, `at_end` takes no `start_offset` — it has no way to know where a
/// leading file header ends, so `bytes` must be a pure record-stream span (e.g. `&file[HEADER_SIZE..]`,
/// or an inflated `REC_SEALED` batch) with nothing but ordinary frames in it; `prev_frame` returns
/// `None` exactly at `pos == 0`. A caller walking the whole file backward slices the header off
/// first (or stops once a frame's `offset` reaches `HEADER_SIZE`, since it always knows that bound).
pub struct ReverseFrameCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

/// @emoji 📏️ The smallest possible on-disk frame: a 1-byte `body_len` varint (value 2, the minimum
/// legal body_len), 1 kind byte, 1 flags byte, 4 crc32c bytes, 4 back_len bytes.
const MIN_FRAME_LEN: usize = 11;

impl<'a> ReverseFrameCursor<'a> {
    /// @emoji 🏁️ Positions a cursor just past the last byte of `bytes`, ready to read the final frame.
    pub fn at_end(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: bytes.len() }
    }

    /// @emoji ⏮️ Reads the trailing `back_len`, jumps back that many bytes, forward-parses from
    /// there, and requires the reproduced end offset to equal the cursor's current position —
    /// exactly the contract's reverse-scan algorithm. `None` at exact start-of-stream.
    pub fn prev_frame(&mut self) -> Result<Option<RecordFrame<'a>>, ProtocolError> {
        if self.pos == 0 {
            return Ok(None);
        }
        if self.pos < MIN_FRAME_LEN {
            return Err(ProtocolError::FrameFraming(self.pos as u64));
        }
        let back_len = u32::from_le_bytes(self.bytes[self.pos - 4..self.pos].try_into().unwrap()) as usize;
        if back_len < MIN_FRAME_LEN || back_len > self.pos {
            return Err(ProtocolError::FrameFraming(self.pos as u64));
        }
        let frame_start = self.pos - back_len;
        let (frame, next_pos) = decode_frame_in_slice(self.bytes, frame_start)?;
        if next_pos != self.pos {
            return Err(ProtocolError::FrameFraming(self.pos as u64));
        }
        self.pos = frame_start;
        Ok(Some(frame))
    }
}
//#endregion 🔖️Frame

//#region 🔖️Commit
/// @emoji 📏️ Every commit frame's fixed total on-disk size: `1(body_len, single byte since 65 <
/// 128) + 1(kind) + 1(flags) + 64(payload) + 4(crc32c) + 4(back_len)`.
pub const COMMIT_FRAME_LEN: u64 = 75;
/// @emoji 📏️ Fixed size of a `REC_COMMIT` frame's payload, in bytes.
pub const COMMIT_PAYLOAD_LEN: usize = 64;

/// @emoji ⛓️ The decoded fields of a `REC_COMMIT` frame's fixed 64-byte payload. Public — this is
/// the only way for a downstream crate (`protocol_history`'s `FrontierSummary.last_commit_seq`/
/// `.chain_hash`, `protocol_materialize`, `protocol_cli`'s `hash`/`inspect` subcommands) to read a
/// commit's chain state without re-deriving this crate's private byte offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommitPayload {
    pub commit_seq: u64,
    pub prev_commit_offset: u64,
    pub records_len: u64,
    pub record_count: u32,
    pub chain_hash: [u8; 32],
}

/// @emoji ✍️ Serializes a commit's fixed 64-byte payload per the contract's exact field offsets.
fn write_commit_payload(commit_seq: u64, prev_commit_offset: u64, records_len: u64, record_count: u32, chain_hash: &[u8; 32]) -> [u8; COMMIT_PAYLOAD_LEN] {
    let mut buf = [0u8; COMMIT_PAYLOAD_LEN];
    buf[0..8].copy_from_slice(&commit_seq.to_le_bytes());
    buf[8..16].copy_from_slice(&prev_commit_offset.to_le_bytes());
    buf[16..24].copy_from_slice(&records_len.to_le_bytes());
    buf[24..28].copy_from_slice(&record_count.to_le_bytes());
    buf[32..64].copy_from_slice(chain_hash);
    buf
}

/// @emoji 📖️ Parses a commit frame's payload; the only structural requirement is the exact 64-byte
/// length (already implied by `COMMIT_FRAME_LEN`, but checked directly since callers may hand this
/// any `stored` slice, e.g. during `recover`'s source-backed reads). Public per the `CommitPayload`
/// doc comment above — the sole intended entry point for decoding a `RecordFrame`'s `stored` bytes
/// once a caller has confirmed `kind == crate::os_spr::REC_COMMIT` via `FrameCursor`/`ReverseFrameCursor`.
pub fn parse_commit_payload(payload: &[u8]) -> Result<CommitPayload, ProtocolError> {
    if payload.len() != COMMIT_PAYLOAD_LEN {
        return Err(malformed("commit payload", 0, format!("expected {COMMIT_PAYLOAD_LEN} bytes, got {}", payload.len())));
    }
    let commit_seq = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let prev_commit_offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let records_len = u64::from_le_bytes(payload[16..24].try_into().unwrap());
    let record_count = u32::from_le_bytes(payload[24..28].try_into().unwrap());
    let mut chain_hash = [0u8; 32];
    chain_hash.copy_from_slice(&payload[32..64]);
    Ok(CommitPayload { commit_seq, prev_commit_offset, records_len, record_count, chain_hash })
}
//#endregion 🔖️Commit

//#region 🔖️Writer
/// @emoji ⚙️ Header flags to open a `.spr` file with.
#[derive(Clone, Copy, Debug, Default)]
pub struct WriteOptions {
    pub required_flags: u32,
    pub optional_flags: u32,
}

/// @emoji 🗜️ `(compressed, raw_len, stored_bytes)` — `Cow::Borrowed` for the identity codec avoids
/// copying an already caller-owned payload slice.
type PreparedPayload<'a> = (bool, Option<u64>, std::borrow::Cow<'a, [u8]>);

/// @emoji 🗜️ Resolves `codec` for one `write_record` call.
fn prepare_payload(codec: crate::os_pack::CodecId, payload: &[u8]) -> Result<PreparedPayload<'_>, ProtocolError> {
    match codec.0 {
        0 => Ok((false, None, std::borrow::Cow::Borrowed(payload))),
        1 => {
            #[cfg(feature = "deflate")]
            {
                let compressed = crate::os_pack::format::DeflateCodec.compress(payload)?;
                Ok((true, Some(payload.len() as u64), std::borrow::Cow::Owned(compressed)))
            }
            #[cfg(not(feature = "deflate"))]
            {
                let _ = payload;
                Err(ProtocolError::Pack(PackError::UnsupportedCodec(1)))
            }
        }
        other => Err(ProtocolError::Pack(PackError::UnsupportedCodec(other))),
    }
}

/// @emoji ✒️ Streaming `.spr` builder: writes the header, then any number of records via
/// `write_record`, periodically closed off with `commit` — which hash-chains everything written
/// since the previous commit (or the header, for the first). The hot path (`write_record`) is
/// allocation-light: one reusable scratch buffer per writer, no payload re-copy for identity codec.
pub struct SprWriter<S: PackSink> {
    sink: S,
    running_chain_hash: [u8; 32],
    pending_digests: Vec<[u8; 32]>,
    pending_records_len: u64,
    pending_record_count: u32,
    next_commit_seq: u64,
    last_commit_offset: Option<u64>,
    scratch: Vec<u8>,
}

impl<S: PackSink> SprWriter<S> {
    /// @emoji 🚀️ Writes the 32-byte header and seeds `chain_0 = blake3(header bytes)`.
    pub fn begin(mut sink: S, options: &WriteOptions) -> Result<Self, ProtocolError> {
        let unknown = options.required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err(ProtocolError::Pack(PackError::UnknownRequiredFlags(unknown)));
        }
        let header = build_header_bytes(options.required_flags, options.optional_flags);
        sink.write_all(&header)?;
        let chain_0 = *blake3::hash(&header).as_bytes();
        Ok(Self { sink, running_chain_hash: chain_0, pending_digests: Vec::new(), pending_records_len: 0, pending_record_count: 0, next_commit_seq: 1, last_commit_offset: None, scratch: Vec::new() })
    }

    /// @emoji 📍️ Current absolute write position — the offset the next record/commit will start at.
    pub fn position(&self) -> u64 {
        self.sink.position()
    }

    /// @emoji 🖇️ Frames (compressing per `codec`), CRCs, and writes one record. Returns its start
    /// offset. Folds the frame's `blake3` digest into the pending commit-chain accumulator — the
    /// digest covers the WHOLE on-disk frame (length-prefix through `back_len`), matching the
    /// contract's `digest_i = blake3(full frame bytes of record i)`.
    pub fn write_record(&mut self, kind: u8, critical: bool, payload: &[u8], codec: crate::os_pack::CodecId) -> Result<u64, ProtocolError> {
        let start_offset = self.sink.position();
        let (compressed, raw_len, stored) = prepare_payload(codec, payload)?;
        let flags = frame_flags(compressed, critical, codec.0);
        encode_frame_into(&mut self.scratch, kind, flags, raw_len, stored.as_ref())?;
        self.sink.write_all(&self.scratch)?;
        let digest = *blake3::hash(&self.scratch).as_bytes();
        self.pending_digests.push(digest);
        self.pending_records_len += self.scratch.len() as u64;
        self.pending_record_count = self.pending_record_count.checked_add(1).ok_or(ProtocolError::LimitExceeded("record_count exceeds u32 per commit generation"))?;
        Ok(start_offset)
    }

    /// @emoji ⛓️ Writes a `REC_COMMIT` frame covering everything since the last commit (or the
    /// header, for the first): `chain_n = blake3(chain_{n-1} || digest_1 || .. || digest_k)`.
    /// Returns the commit frame's start offset.
    pub fn commit(&mut self) -> Result<u64, ProtocolError> {
        let offset = self.sink.position();
        let mut concat = Vec::with_capacity(32 + self.pending_digests.len() * 32);
        concat.extend_from_slice(&self.running_chain_hash);
        for digest in &self.pending_digests {
            concat.extend_from_slice(digest);
        }
        let chain_hash = *blake3::hash(&concat).as_bytes();

        let commit_seq = self.next_commit_seq;
        let prev_commit_offset = self.last_commit_offset.unwrap_or(0);
        let payload = write_commit_payload(commit_seq, prev_commit_offset, self.pending_records_len, self.pending_record_count, &chain_hash);
        let flags = frame_flags(false, true, 0);
        encode_frame_into(&mut self.scratch, crate::os_spr::REC_COMMIT, flags, None, &payload)?;
        self.sink.write_all(&self.scratch)?;

        self.running_chain_hash = chain_hash;
        self.pending_digests.clear();
        self.pending_records_len = 0;
        self.pending_record_count = 0;
        self.next_commit_seq += 1;
        self.last_commit_offset = Some(offset);
        Ok(offset)
    }

    /// @emoji 📤️ Unwraps the underlying sink (e.g. to hand a `Vec<u8>` or file handle onward).
    pub fn into_sink(self) -> S {
        self.sink
    }
}
//#endregion 🔖️Writer

//#region 🔖️Recovery
/// @emoji 📋️ What a `recover` call found: how much of the file is trustworthy and where.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoveryReport {
    pub records_recovered: u64,
    /// @emoji 📏️ Design choice: the absolute count of trusted bytes from the start of the file —
    /// i.e. the offset callers should truncate/slice to (`&bytes[..bytes_recovered]`), not merely
    /// "bytes of record payload." Always `>= HEADER_SIZE` (the header itself always counts).
    pub bytes_recovered: u64,
    pub last_commit_seq: u64,
    pub last_commit_offset: u64,
    pub torn_tail_bytes: u64,
}

/// @emoji 🔀️ Which recovered boundary a caller trusts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RecoveryMode {
    #[default]
    LastCommit,
    LastValidRecord,
}

/// @emoji 🔍️ How much a reader verifies before trusting decoded content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VerificationLevel {
    Trusted,
    #[default]
    Standard,
    Full,
}

/// @emoji 🔢️ Reads one LEB128 varint at an absolute source offset, one byte at a time so it never
/// over-reads a legitimately short remaining source. Mirrors `crate::os_pack::format::read_varint_u64_at`.
fn read_varint_via_source<S: PackSource>(source: &S, offset: u64, total_len: u64) -> Result<(u64, u64), ProtocolError> {
    let mut tmp: Vec<u8> = Vec::with_capacity(10);
    let mut i = 0u64;
    loop {
        if offset + i >= total_len {
            return Err(ProtocolError::Pack(PackError::Truncated(offset + i)));
        }
        let mut byte = [0u8; 1];
        source.read_exact_at(offset + i, &mut byte)?;
        tmp.push(byte[0]);
        i += 1;
        if byte[0] & 0x80 == 0 {
            break;
        }
        if i >= 10 {
            return Err(malformed("varint", offset, "overlong varint (exceeds 10 bytes)"));
        }
    }
    let mut pos = 0usize;
    let value = crate::os_pack::read_varint_u64(&tmp, &mut pos)?;
    Ok((value, i))
}

/// @emoji 👓️ `(kind, flags, raw_len, owned payload, frame_len)` — the owning twin of `RecordFrame`
/// for `PackSource`-backed reads, which cannot borrow zero-copy from a random-access source.
type SourceFrame = (u8, u8, Option<u64>, Vec<u8>, u64);

/// @emoji 👓️ Source-backed (owning, non-zero-copy) frame read at an absolute offset — the
/// `PackSource` twin of `decode_frame_in_slice`, used by `recover`'s forward scan and fast-path
/// commit walk, both of which operate over `PackSource` rather than an in-memory slice. Validates
/// `body_len` against `limits.max_frame_len` BEFORE allocating the body buffer.
fn read_frame_via_source<S: PackSource>(source: &S, offset: u64, limits: &ProtocolLimits) -> Result<SourceFrame, ProtocolError> {
    let total_len = source.len();
    let (body_len, body_len_width) = read_varint_via_source(source, offset, total_len)?;
    if body_len < 2 {
        return Err(malformed("frame body_len", offset, "body_len smaller than kind+flags (2 bytes)"));
    }
    if body_len > limits.max_frame_len {
        return Err(ProtocolError::LimitExceeded("frame body_len exceeds ProtocolLimits::max_frame_len"));
    }
    let body_start = offset + body_len_width;
    let body_end = body_start.checked_add(body_len).ok_or(ProtocolError::LimitExceeded("frame body offset overflow"))?;
    let trailer_end = body_end.checked_add(8).ok_or(ProtocolError::LimitExceeded("frame trailer offset overflow"))?;
    if trailer_end > total_len {
        return Err(ProtocolError::Pack(PackError::Truncated(body_start)));
    }
    let mut body = vec![0u8; body_len as usize];
    source.read_exact_at(body_start, &mut body)?;
    let kind = body[0];
    let flags = body[1];
    let compressed = flags & FRAME_FLAG_COMPRESSED != 0;
    let mut payload_start = 2usize;
    let raw_len = if compressed {
        let mut p = payload_start;
        let v = crate::os_pack::read_varint_u64(&body, &mut p)?;
        payload_start = p;
        Some(v)
    } else {
        None
    };
    let mut trailer = [0u8; 8];
    source.read_exact_at(body_end, &mut trailer)?;
    let stored_crc = u32::from_le_bytes(trailer[0..4].try_into().unwrap());
    let computed_crc = crate::os_pack::crc32c(&body);
    if stored_crc != computed_crc {
        return Err(ProtocolError::Pack(PackError::ChecksumMismatch { segment: "frame", offset: body_end }));
    }
    let back_len = u32::from_le_bytes(trailer[4..8].try_into().unwrap()) as u64;
    let frame_len = trailer_end - offset;
    if back_len != frame_len {
        return Err(ProtocolError::FrameFraming(offset));
    }
    let payload = body.split_off(payload_start);
    Ok((kind, flags, raw_len, payload, frame_len))
}

/// @emoji ⚡️ Fast path: probes the last `COMMIT_FRAME_LEN` bytes for a valid `REC_COMMIT` frame
/// reaching exactly EOF, then walks `prev_commit_offset` all the way back to `commit_seq == 1`,
/// re-validating (CRC + critical bit + sequence linkage) every commit frame along the way — O(commit
/// count), never touching an intervening non-commit record. `None` on any failure, signalling the
/// caller to fall back to the forward scan; a fast-path success always means `torn_tail_bytes == 0`
/// (the tail commit frame ends exactly at EOF), so it is valid regardless of `RecoveryMode`.
fn try_fast_path<S: PackSource>(source: &S, total_len: u64, limits: &ProtocolLimits) -> Option<RecoveryReport> {
    if total_len < HEADER_SIZE as u64 + COMMIT_FRAME_LEN {
        return None;
    }
    let tail_offset = total_len - COMMIT_FRAME_LEN;
    let (kind, flags, _raw_len, payload, frame_len) = read_frame_via_source(source, tail_offset, limits).ok()?;
    if kind != crate::os_spr::REC_COMMIT || frame_len != COMMIT_FRAME_LEN || flags & FRAME_FLAG_CRITICAL == 0 {
        return None;
    }
    let mut current = parse_commit_payload(&payload).ok()?;
    let last_commit_seq = current.commit_seq;
    let last_commit_offset = tail_offset;
    let mut num_commits = 1u64;
    let mut records_sum = current.record_count as u64;
    let mut cursor_offset = tail_offset;

    loop {
        if current.commit_seq == 1 {
            if current.prev_commit_offset != 0 {
                return None;
            }
            break;
        }
        let prev_offset = current.prev_commit_offset;
        if prev_offset == 0 || prev_offset >= cursor_offset {
            return None;
        }
        let (prev_kind, prev_flags, _prev_raw_len, prev_payload, _prev_frame_len) = read_frame_via_source(source, prev_offset, limits).ok()?;
        if prev_kind != crate::os_spr::REC_COMMIT || prev_flags & FRAME_FLAG_CRITICAL == 0 {
            return None;
        }
        let prev_commit = parse_commit_payload(&prev_payload).ok()?;
        if prev_commit.commit_seq != current.commit_seq - 1 {
            return None;
        }
        num_commits += 1;
        records_sum += prev_commit.record_count as u64;
        cursor_offset = prev_offset;
        current = prev_commit;
    }

    Some(RecoveryReport { records_recovered: records_sum + num_commits, bytes_recovered: total_len, last_commit_seq, last_commit_offset, torn_tail_bytes: 0 })
}

/// @emoji 🚑️ Recovers a `.spr` source: validates the header, then tries the O(commits) fast path
/// before falling back to a bounded forward scan from `HEADER_SIZE`, stopping at the first invalid
/// or truncated frame. See the contract's four-step algorithm (reproduced in the inline comments).
pub fn recover<S: PackSource>(source: &S, limits: &ProtocolLimits, mode: RecoveryMode) -> Result<RecoveryReport, ProtocolError> {
    let total_len = source.len();
    if total_len > limits.max_file_len {
        return Err(ProtocolError::LimitExceeded("file exceeds ProtocolLimits::max_file_len"));
    }
    // (1) validate header
    validate_header(source)?;

    if total_len == HEADER_SIZE as u64 {
        return Ok(RecoveryReport { records_recovered: 0, bytes_recovered: HEADER_SIZE as u64, last_commit_seq: 0, last_commit_offset: 0, torn_tail_bytes: 0 });
    }

    // (2) fast path
    if let Some(report) = try_fast_path(source, total_len, limits) {
        return Ok(report);
    }

    // (3) forward scan from offset 32, tracking last_valid_end and last_commit_end
    let mut pos = HEADER_SIZE as u64;
    let mut last_valid_end = HEADER_SIZE as u64;
    let mut last_commit_end = HEADER_SIZE as u64;
    let mut last_commit_seq = 0u64;
    let mut last_commit_offset = 0u64;
    let mut records_valid = 0u64;
    let mut records_at_commit = 0u64;

    while pos < total_len {
        let frame_start = pos;
        match read_frame_via_source(source, pos, limits) {
            Ok((kind, _flags, _raw_len, payload, frame_len)) => {
                records_valid += 1;
                if records_valid > limits.max_record_count {
                    return Err(ProtocolError::LimitExceeded("record count exceeds ProtocolLimits::max_record_count"));
                }
                pos += frame_len;
                last_valid_end = pos;
                if kind == crate::os_spr::REC_COMMIT {
                    if let Ok(commit) = parse_commit_payload(&payload) {
                        last_commit_seq = commit.commit_seq;
                        last_commit_offset = frame_start;
                        last_commit_end = pos;
                        records_at_commit = records_valid;
                    }
                }
            }
            Err(_) => break,
        }
    }

    // (4) LastCommit trusts only [0, last_commit_end); LastValidRecord trusts through last_valid_end
    let (trusted_end, records_recovered) = match mode {
        RecoveryMode::LastCommit => (last_commit_end, records_at_commit),
        RecoveryMode::LastValidRecord => (last_valid_end, records_valid),
    };
    Ok(RecoveryReport { records_recovered, bytes_recovered: trusted_end, last_commit_seq, last_commit_offset, torn_tail_bytes: total_len - trusted_end })
}
//#endregion 🔖️Recovery

//#region 🔖️Crypto
/// @emoji 🔗️ The commit chain's hash primitive: `blake3`, owned here so `protocol_core` stays
/// dependency-free (per the family's crypto-trait-only rule).
pub struct Blake3Hasher;

impl RecordHasher for Blake3Hasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32] {
        *blake3::hash(bytes).as_bytes()
    }
}
//#endregion 🔖️Crypto

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Header
    #[test]
    fn header_round_trips_via_begin_and_validate() {
        let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
        let writer = SprWriter::begin(Vec::new(), &options).unwrap();
        let bytes = writer.into_sink();
        assert_eq!(bytes.len(), HEADER_SIZE);
        assert_eq!(&bytes[0..8], &MAGIC);
        assert!(validate_header(&bytes).is_ok());
    }

    #[test]
    fn header_rejects_bad_magic() {
        let mut bytes = build_header_bytes(0, 0).to_vec();
        bytes[0] = 0x00;
        assert!(matches!(validate_header(&bytes), Err(ProtocolError::Pack(PackError::BadMagic))));
    }

    #[test]
    fn header_rejects_unknown_required_flags() {
        let bytes = build_header_bytes(1 << 5, 0).to_vec();
        let err = validate_header(&bytes).unwrap_err();
        assert!(matches!(err, ProtocolError::Pack(PackError::UnknownRequiredFlags(bits)) if bits == 1 << 5));
    }

    #[test]
    fn header_rejects_corrupted_crc() {
        let mut bytes = build_header_bytes(0, 0).to_vec();
        bytes[15] ^= 0xFF;
        assert!(matches!(validate_header(&bytes), Err(ProtocolError::Pack(PackError::ChecksumMismatch { .. }))));
    }

    #[test]
    fn read_header_exposes_decoded_fields_to_downstream_crates() {
        let bytes = build_header_bytes(crate::os_spr::REQUIRED_HASH_CHAIN, crate::os_spr::OPTIONAL_CANONICAL).to_vec();
        let header = read_header(&bytes).unwrap();
        assert_eq!(header, Header { version_major: FORMAT_VERSION_MAJOR, version_minor: FORMAT_VERSION_MINOR, required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL });
    }

    #[test]
    fn read_header_propagates_validation_failures() {
        let mut bytes = build_header_bytes(0, 0).to_vec();
        bytes[0] = 0x00;
        assert!(matches!(read_header(&bytes), Err(ProtocolError::Pack(PackError::BadMagic))));
    }
    //#endregion 🔖️Header

    //#region 🔖️Frame
    fn build_small_file() -> Vec<u8> {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        writer.write_record(crate::os_spr::REC_DOC, true, b"doc-payload", crate::os_pack::CodecId(0)).unwrap();
        writer.write_record(crate::os_spr::REC_EDIT, true, b"edit-payload", crate::os_pack::CodecId(0)).unwrap();
        writer.commit().unwrap();
        writer.into_sink()
    }

    #[test]
    fn frame_round_trips_kind_flags_and_payload() {
        let bytes = build_small_file();
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let doc = cursor.next_frame().unwrap().unwrap();
        assert_eq!(doc.kind, crate::os_spr::REC_DOC);
        assert_eq!(doc.payload(), b"doc-payload");
        assert_eq!(doc.flags & FRAME_FLAG_CRITICAL, FRAME_FLAG_CRITICAL);
        assert_eq!(doc.offset, HEADER_SIZE as u64);

        let edit = cursor.next_frame().unwrap().unwrap();
        assert_eq!(edit.kind, crate::os_spr::REC_EDIT);
        assert_eq!(edit.payload(), b"edit-payload");

        let commit = cursor.next_frame().unwrap().unwrap();
        assert_eq!(commit.kind, crate::os_spr::REC_COMMIT);
        assert_eq!(commit.stored.len(), 64);

        assert!(cursor.next_frame().unwrap().is_none());
    }

    #[test]
    fn frame_len_matches_actual_bytes_consumed() {
        let bytes = build_small_file();
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let mut pos = HEADER_SIZE as u64;
        while let Some(frame) = cursor.next_frame().unwrap() {
            assert_eq!(frame.offset, pos);
            pos += frame.frame_len();
        }
        assert_eq!(pos, bytes.len() as u64);
    }

    #[test]
    fn reverse_cursor_yields_frames_in_reverse_order() {
        let bytes = build_small_file();
        let mut forward = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let mut forward_kinds = Vec::new();
        while let Some(frame) = forward.next_frame().unwrap() {
            forward_kinds.push(frame.kind);
        }

        // ReverseFrameCursor has no header-boundary awareness (see its doc comment) — a whole-file
        // walk slices the header off first so `prev_frame` naturally bottoms out at pos == 0.
        let mut reverse = ReverseFrameCursor::at_end(&bytes[HEADER_SIZE..]);
        let mut reverse_kinds = Vec::new();
        while let Some(frame) = reverse.prev_frame().unwrap() {
            reverse_kinds.push(frame.kind);
        }
        reverse_kinds.reverse();
        assert_eq!(forward_kinds, reverse_kinds);
    }

    #[test]
    fn reverse_cursor_at_start_of_stream_returns_none() {
        let mut cursor = ReverseFrameCursor::at_end(&[]);
        assert!(cursor.prev_frame().unwrap().is_none());
    }

    #[test]
    fn reverse_cursor_stops_exactly_at_record_stream_start() {
        let bytes = build_small_file();
        let mut cursor = ReverseFrameCursor::at_end(&bytes[HEADER_SIZE..]);
        let mut count = 0;
        while cursor.prev_frame().unwrap().is_some() {
            count += 1;
        }
        assert_eq!(count, 3); // 2 records + 1 commit
        assert!(cursor.prev_frame().unwrap().is_none(), "must keep returning None, not error, once exhausted");
    }

    #[test]
    fn frame_detects_crc_corruption() {
        let mut bytes = build_small_file();
        let corrupt_at = HEADER_SIZE + 4;
        bytes[corrupt_at] ^= 0xFF;
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let err = cursor.next_frame().unwrap_err();
        assert!(matches!(err, ProtocolError::Pack(PackError::ChecksumMismatch { .. })));
    }

    #[test]
    fn frame_detects_back_len_tamper_via_reverse_cursor() {
        let mut bytes = build_small_file();
        let last = bytes.len();
        bytes[last - 1] ^= 0xFF;
        let mut cursor = ReverseFrameCursor::at_end(&bytes);
        let err = cursor.prev_frame().unwrap_err();
        assert!(matches!(err, ProtocolError::Pack(PackError::ChecksumMismatch { .. }) | ProtocolError::FrameFraming(_)));
    }

    #[test]
    fn cursor_surfaces_unrecognized_extension_kind_transparently() {
        // 🎯️ "Skip-unknown" for this crate means: an unrecognized kind (extension range, critical
        // bit clear) is structurally indistinguishable from a known one — the cursor advances past
        // it and returns it like any other frame; interpreting/skipping it is a caller decision
        // (see the module doc). This proves the cursor never special-cases `kind`.
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        writer.write_record(0x50, false, b"extension-payload", crate::os_pack::CodecId(0)).unwrap();
        writer.write_record(crate::os_spr::REC_DOC, true, b"doc-payload", crate::os_pack::CodecId(0)).unwrap();
        let bytes = writer.into_sink();

        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let ext = cursor.next_frame().unwrap().unwrap();
        assert_eq!(ext.kind, 0x50);
        assert_eq!(ext.flags & FRAME_FLAG_CRITICAL, 0);
        assert_eq!(ext.payload(), b"extension-payload");

        let doc = cursor.next_frame().unwrap().unwrap();
        assert_eq!(doc.kind, crate::os_spr::REC_DOC);
        assert!(cursor.next_frame().unwrap().is_none());
    }

    #[cfg(feature = "deflate")]
    #[test]
    fn compressed_frame_round_trips_stored_and_raw_len() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        let payload = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        writer.write_record(crate::os_spr::REC_DOC, true, payload, crate::os_pack::CodecId(1)).unwrap();
        let bytes = writer.into_sink();

        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let frame = cursor.next_frame().unwrap().unwrap();
        assert_eq!(frame.flags & FRAME_FLAG_COMPRESSED, FRAME_FLAG_COMPRESSED);
        assert_eq!(frame.raw_len, Some(payload.len() as u64));
        assert!(frame.stored.len() < payload.len());

        let decompressed = crate::os_pack::format::DeflateCodec.decompress(frame.stored, frame.raw_len.unwrap(), 1024).unwrap();
        assert_eq!(decompressed, payload);
    }
    //#endregion 🔖️Frame

    //#region 🔖️Commit
    #[test]
    fn commit_chain_verifies_by_recomputation_on_a_hand_built_file() {
        let bytes = build_small_file();

        // chain_0 = blake3(header)
        let chain_0 = *blake3::hash(&bytes[..HEADER_SIZE]).as_bytes();

        // Recompute digest_i for every non-commit frame since the header, and the resulting chain_1.
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let mut concat = chain_0.to_vec();
        let mut commit_frame: Option<RecordFrame<'_>> = None;
        while let Some(frame) = cursor.next_frame().unwrap() {
            if frame.kind == crate::os_spr::REC_COMMIT {
                commit_frame = Some(frame);
                break;
            }
            let full_frame_bytes = &bytes[frame.offset as usize..(frame.offset + frame.frame_len()) as usize];
            concat.extend_from_slice(blake3::hash(full_frame_bytes).as_bytes());
        }
        let expected_chain_1 = *blake3::hash(&concat).as_bytes();

        let commit = parse_commit_payload(commit_frame.unwrap().stored).unwrap();
        assert_eq!(commit.commit_seq, 1);
        assert_eq!(commit.prev_commit_offset, 0);
        assert_eq!(commit.record_count, 2);
        assert_eq!(commit.chain_hash, expected_chain_1);
    }

    #[test]
    fn commit_chain_links_prev_commit_offset_across_multiple_commits() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        writer.write_record(crate::os_spr::REC_DOC, true, b"a", crate::os_pack::CodecId(0)).unwrap();
        let commit_1_offset = writer.commit().unwrap();
        writer.write_record(crate::os_spr::REC_EDIT, true, b"b", crate::os_pack::CodecId(0)).unwrap();
        let commit_2_offset = writer.commit().unwrap();
        let bytes = writer.into_sink();

        let mut reverse = ReverseFrameCursor::at_end(&bytes);
        let last = reverse.prev_frame().unwrap().unwrap();
        assert_eq!(last.offset, commit_2_offset);
        let commit_2 = parse_commit_payload(last.stored).unwrap();
        assert_eq!(commit_2.commit_seq, 2);
        assert_eq!(commit_2.prev_commit_offset, commit_1_offset);
    }

    #[test]
    fn parse_commit_payload_is_public_for_downstream_frontier_summaries() {
        // Mirrors how `crate::os_spr::history::FrontierSummary`/`protocol_materialize` are expected to
        // pull `last_commit_seq`/`chain_hash` out of a `REC_COMMIT` frame found via `FrameCursor`.
        let bytes = build_small_file();
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64);
        let commit_frame = std::iter::from_fn(|| cursor.next_frame().transpose()).map(Result::unwrap).find(|frame| frame.kind == crate::os_spr::REC_COMMIT).unwrap();
        let payload: CommitPayload = parse_commit_payload(commit_frame.payload()).unwrap();
        assert_eq!(payload.commit_seq, 1);
        assert_eq!(payload.record_count, 2);
    }
    //#endregion 🔖️Commit

    //#region 🔖️Recovery
    #[test]
    fn recover_fast_path_trusts_a_cleanly_committed_file() {
        let bytes = build_small_file();
        let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
        assert_eq!(report.bytes_recovered, bytes.len() as u64);
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(report.last_commit_seq, 1);
        assert_eq!(report.records_recovered, 3); // 2 records + 1 commit frame
    }

    #[test]
    fn recover_fast_path_walks_multi_commit_chain_to_genesis() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        for i in 0..5u8 {
            writer.write_record(crate::os_spr::REC_EDIT, true, &[i], crate::os_pack::CodecId(0)).unwrap();
            writer.commit().unwrap();
        }
        let bytes = writer.into_sink();
        let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
        assert_eq!(report.last_commit_seq, 5);
        assert_eq!(report.bytes_recovered, bytes.len() as u64);
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(report.records_recovered, 10); // 5 records + 5 commits
    }

    #[test]
    fn recover_forward_scan_truncates_to_last_commit_on_torn_tail() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        writer.write_record(crate::os_spr::REC_DOC, true, b"a", crate::os_pack::CodecId(0)).unwrap();
        let commit_end = writer.commit().unwrap() + COMMIT_FRAME_LEN;
        writer.write_record(crate::os_spr::REC_EDIT, true, b"uncommitted", crate::os_pack::CodecId(0)).unwrap();
        let mut bytes = writer.into_sink();
        bytes.truncate(bytes.len() - 3); // tear the tail mid-record, after the commit

        let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
        assert_eq!(report.bytes_recovered, commit_end);
        assert_eq!(report.torn_tail_bytes, bytes.len() as u64 - commit_end);
        assert_eq!(report.last_commit_seq, 1);
    }

    #[test]
    fn recover_last_valid_record_mode_trusts_past_the_last_commit() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        writer.write_record(crate::os_spr::REC_DOC, true, b"a", crate::os_pack::CodecId(0)).unwrap();
        writer.commit().unwrap();
        writer.write_record(crate::os_spr::REC_EDIT, true, b"uncommitted-but-well-formed", crate::os_pack::CodecId(0)).unwrap();
        let bytes = writer.into_sink();

        let last_commit_report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
        let last_valid_report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastValidRecord).unwrap();
        assert!(last_valid_report.bytes_recovered > last_commit_report.bytes_recovered);
        assert_eq!(last_valid_report.bytes_recovered, bytes.len() as u64);
    }

    #[test]
    fn recover_header_only_file_reports_zero_records() {
        let writer = SprWriter::begin(Vec::new(), &WriteOptions::default()).unwrap();
        let bytes = writer.into_sink();
        let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
        assert_eq!(report.records_recovered, 0);
        assert_eq!(report.bytes_recovered, HEADER_SIZE as u64);
        assert_eq!(report.last_commit_seq, 0);
    }

    #[test]
    fn recover_rejects_files_shorter_than_the_header() {
        let bytes = vec![0u8; 10];
        assert!(recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).is_err());
    }

    #[test]
    fn recover_never_panics_on_truncation_at_every_byte() {
        let options = WriteOptions::default();
        let mut writer = SprWriter::begin(Vec::new(), &options).unwrap();
        for i in 0..4u8 {
            writer.write_record(crate::os_spr::REC_EDIT, true, &[i; 5], crate::os_pack::CodecId(0)).unwrap();
        }
        writer.commit().unwrap();
        writer.write_record(crate::os_spr::REC_EDIT, true, b"tail-record-after-commit", crate::os_pack::CodecId(0)).unwrap();
        let bytes = writer.into_sink();

        for len in 0..=bytes.len() {
            let truncated: Vec<u8> = bytes[..len].to_vec();
            let result = recover(&truncated, &ProtocolLimits::default(), RecoveryMode::LastCommit);
            if let Ok(report) = result {
                assert!(report.bytes_recovered <= len as u64);
                assert!(report.torn_tail_bytes <= len as u64);
            }
        }
    }
    //#endregion 🔖️Recovery

    //#region 🔖️Crypto
    #[test]
    fn blake3_hasher_matches_direct_blake3_call() {
        let hasher = Blake3Hasher;
        assert_eq!(hasher.hash(b"hello"), *blake3::hash(b"hello").as_bytes());
    }
    //#endregion 🔖️Crypto

    //#region 🔖️Writer
    #[test]
    fn begin_rejects_unknown_required_flags() {
        let options = WriteOptions { required_flags: 1 << 10, optional_flags: 0 };
        let result = SprWriter::begin(Vec::new(), &options);
        match result {
            Ok(_) => panic!("expected UnknownRequiredFlags error, got Ok"),
            Err(err) => assert!(matches!(err, ProtocolError::Pack(PackError::UnknownRequiredFlags(bits)) if bits == 1 << 10)),
        }
    }

    #[test]
    fn position_tracks_sink_length_across_writes() {
        let mut writer = SprWriter::begin(Vec::new(), &WriteOptions::default()).unwrap();
        assert_eq!(writer.position(), HEADER_SIZE as u64);
        writer.write_record(crate::os_spr::REC_DOC, true, b"x", crate::os_pack::CodecId(0)).unwrap();
        assert_eq!(writer.position(), writer.into_sink().len() as u64);
    }
    //#endregion 🔖️Writer
}
//#endregion 🧪️Tests
