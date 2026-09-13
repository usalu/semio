//! 📥 Replaces the document with imported fixture JSON, reassembled from the host's chunked inbound lane.

use crate::editor::puzzle3d::{Puzzle3dActionCtx, Puzzle3dFixture, PUZZLE3D_FIXTURE_SCHEMA};
use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use dsl::json;
use dsl::os_pack::json::{array, object, parse, Value};
use dsl::FromValue;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use std::sync::{Mutex, OnceLock};

/// 📏️ Bytes ONE inbound import chunk may carry.
///
/// 🧊️ Derived, never a literal: a chunk crosses as one `payload` STRING inside one command, and every hop
/// that carries it asks the guest for one contiguous block of it — the renderer's pack encode, the
/// channel's `read_bounded_bytes` (`📡️spr/🧵️channel/🦀️.rs`) and the retained job's own wire owner. Half
/// [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`] leaves the whole second half of that one guest page for the
/// JSON-escaped op envelope the chunk travels in (measured 1.10× on the Nakagin export, worst case 6× per
/// escaped control byte), so a chunk plus its envelope never asks for a block the fixed guest heap must
/// grow for. A 145 924-byte document is five chunks.
pub const PUZZLE3D_IMPORT_CHUNK_BYTES: usize = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;

/// 📏️ Largest document ONE import may reassemble — the SAME budget one export may stream
/// ([`PUZZLE_COMMAND_OUTPUT_BYTES`], `puzzle3d_export_segmented_budget_bytes`), so a file this app wrote
/// is always a file this app can read back.
pub const PUZZLE3D_IMPORT_TOTAL_BYTES: usize = PUZZLE_COMMAND_OUTPUT_BYTES;

/// 📦️ Chunks one import may stage — the whole budget at the chunk extent, never a second literal.
pub const PUZZLE3D_IMPORT_MAXIMUM_CHUNKS: usize = PUZZLE3D_IMPORT_TOTAL_BYTES.div_ceil(PUZZLE3D_IMPORT_CHUNK_BYTES);

/// 📦️ Import runs staged at once. One open run per document instance a user is importing into; an
/// abandoned run costs one slot until the next sweep, never unbounded memory.
const PUZZLE3D_IMPORT_SLOTS: usize = 4;

/// 🚫️ Why one import chunk was refused. Every arm becomes a localized shell notice in the command arm —
/// never a silent no-op, which is exactly what a 145 924-byte import used to be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dImportFault {
    /// 🏷️ `chunk`/`chunkCount` are out of order, zero, or beyond [`PUZZLE3D_IMPORT_MAXIMUM_CHUNKS`].
    Envelope,
    /// 📏️ One chunk is larger than [`PUZZLE3D_IMPORT_CHUNK_BYTES`] — the host sliced by the wrong constant,
    /// or a whole document was sent unchunked.
    Chunk,
    /// 🕳️ The chunk does not continue the staged run. The broken run is dropped so the client re-opens at
    /// chunk 0 instead of resuming into bytes nobody can account for.
    Gap,
    /// 📦️ The run would exceed [`PUZZLE3D_IMPORT_TOTAL_BYTES`].
    Capacity,
    /// 🧊️ One JSON member of the reassembled document is itself larger than the guest's per-request
    /// contiguous ceiling, so it cannot be parsed without growing the fixed heap.
    Element,
    /// 🔤️ The reassembled bytes are not one JSON object, or not a puzzle 3D document.
    Payload,
}

impl Puzzle3dImportFault {
    /// 🏷️ Stable wire code, mirrored by the import notice and the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "puzzle3d-import-envelope",
            Self::Chunk => "puzzle3d-import-chunk",
            Self::Gap => "puzzle3d-import-gap",
            Self::Capacity => "puzzle3d-import-capacity",
            Self::Element => "puzzle3d-import-element",
            Self::Payload => "puzzle3d-import-payload",
        }
    }
}

/// 🧱️ What one accepted chunk did to its run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Puzzle3dImportStep {
    /// 🧱️ The chunk landed and the run is still open — no document edit, no history row.
    Staged { next_chunk: usize, chunk_count: usize },
    /// ✅️ The chunk closed the run; the pages are the whole document, each one page of the paged owner.
    Complete(Vec<String>),
}

/// 🧵️ One import's open chunk run. `pages` IS the paged owner: one page per admitted chunk, each bounded
/// by [`PUZZLE3D_IMPORT_CHUNK_BYTES`], so the reassembled document never exists as one contiguous block.
struct Puzzle3dImportSlot {
    name: String,
    chunk_count: usize,
    next_chunk: usize,
    bytes: usize,
    pages: Vec<String>,
    touched: u64,
}

struct Puzzle3dImportStaging {
    slots: Vec<Puzzle3dImportSlot>,
    sequence: u64,
    swept: u64,
}

fn import_staging() -> &'static Mutex<Puzzle3dImportStaging> {
    static STAGING: OnceLock<Mutex<Puzzle3dImportStaging>> = OnceLock::new();
    STAGING.get_or_init(|| Mutex::new(Puzzle3dImportStaging { slots: Vec::new(), sequence: 0, swept: 0 }))
}

/// 📥️ The chunk envelope one `importFixture` invocation carries. An invocation that declares none is one
/// whole-document chunk, which is the only shape that fits [`PUZZLE3D_IMPORT_CHUNK_BYTES`] on its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle3dImportEnvelope {
    pub name: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

/// 📥️ Reads the envelope off the invocation args, defaulting an undeclared one to the single-chunk shape.
pub fn puzzle3d_import_envelope(args: &Value) -> Puzzle3dImportEnvelope {
    let number = |key: &str| args.get(key).and_then(Value::as_f64).filter(|value| *value >= 0.0).map(|value| value as usize);
    Puzzle3dImportEnvelope {
        name: args.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
        chunk: number("chunk").unwrap_or(0),
        chunk_count: number("chunkCount").unwrap_or(1),
    }
}

/// 📥️ Slices one document's JSON into the chunks the host sends, so the guest's own laws exercise the wire
/// the renderer builds rather than a shape only tests use. Mirrored by `importPayloadChunks`
/// (`🛠️ShellHelpers/🟦️.tsx`) and held equal to it by the engine contract's fixture law.
pub fn puzzle3d_import_chunks(payload: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut page = String::new();
    for character in payload.chars() {
        if page.len().saturating_add(character.len_utf8()) > PUZZLE3D_IMPORT_CHUNK_BYTES {
            chunks.push(std::mem::take(&mut page));
        }
        page.push(character);
    }
    if !page.is_empty() || chunks.is_empty() {
        chunks.push(page);
    }
    chunks
}

/// 🧩️ Admits one chunk of an import run into the staging area, and hands back every page the moment the
/// run closes. A run is keyed by `(name, chunkCount)`: a client that restarts an import opens a different
/// run instead of corrupting the staged one, and a chunk the run already admitted is a RETRANSMISSION
/// acknowledged at the cursor it stands on rather than a gap that costs the whole upload again.
pub fn stage_import_chunk(envelope: &Puzzle3dImportEnvelope, text: &str) -> Result<Puzzle3dImportStep, Puzzle3dImportFault> {
    if envelope.chunk_count == 0 || envelope.chunk_count > PUZZLE3D_IMPORT_MAXIMUM_CHUNKS || envelope.chunk >= envelope.chunk_count {
        return Err(Puzzle3dImportFault::Envelope);
    }
    if text.len() > PUZZLE3D_IMPORT_CHUNK_BYTES {
        return Err(Puzzle3dImportFault::Chunk);
    }
    if envelope.chunk_count == 1 {
        return Ok(Puzzle3dImportStep::Complete(vec![text.to_string()]));
    }
    let mut staging = import_staging().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    staging.sequence = staging.sequence.saturating_add(1);
    let sequence = staging.sequence;
    let held = staging.slots.iter().position(|slot| slot.name == envelope.name && slot.chunk_count == envelope.chunk_count);
    let index = match held {
        Some(index) if staging.slots[index].next_chunk == envelope.chunk => index,
        Some(index) if envelope.chunk != 0 && envelope.chunk < staging.slots[index].next_chunk => {
            let slot = &mut staging.slots[index];
            slot.touched = sequence;
            return Ok(Puzzle3dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
        }
        Some(index) if envelope.chunk != 0 => {
            staging.slots.swap_remove(index);
            return Err(Puzzle3dImportFault::Gap);
        }
        Some(index) => {
            staging.slots[index] = Puzzle3dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence };
            index
        }
        None if envelope.chunk != 0 => return Err(Puzzle3dImportFault::Gap),
        None => {
            if staging.slots.len() >= PUZZLE3D_IMPORT_SLOTS {
                let stale = staging.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Puzzle3dImportFault::Capacity)?;
                staging.slots.swap_remove(stale);
            }
            staging.slots.push(Puzzle3dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence });
            staging.slots.len() - 1
        }
    };
    let slot = &mut staging.slots[index];
    if slot.bytes.saturating_add(text.len()) > PUZZLE3D_IMPORT_TOTAL_BYTES {
        staging.slots.swap_remove(index);
        return Err(Puzzle3dImportFault::Capacity);
    }
    slot.bytes = slot.bytes.saturating_add(text.len());
    slot.pages.push(text.to_string());
    slot.next_chunk = envelope.chunk.saturating_add(1);
    slot.touched = sequence;
    if slot.next_chunk < slot.chunk_count {
        return Ok(Puzzle3dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
    }
    let closed = staging.slots.swap_remove(index);
    Ok(Puzzle3dImportStep::Complete(closed.pages))
}

/// 🧹️ Drops every run that did not advance across a whole session-retirement cycle.
pub fn retire_abandoned_import_runs() {
    let mut staging = import_staging().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let watermark = staging.swept;
    staging.slots.retain(|slot| slot.touched > watermark);
    staging.swept = staging.sequence;
}

/// 🔎️ Open chunk runs, for the unit laws and the staging census.
pub fn staged_import_runs() -> Vec<(String, usize, usize, usize)> {
    let staging = import_staging().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    staging.slots.iter().map(|slot| (slot.name.clone(), slot.next_chunk, slot.chunk_count, slot.bytes)).collect()
}

/// 📄️ A byte cursor over the paged owner — the pages are read in order and never joined.
struct Puzzle3dImportPages<'a> {
    pages: &'a [String],
    page: usize,
    offset: usize,
}

impl<'a> Puzzle3dImportPages<'a> {
    fn new(pages: &'a [String]) -> Self {
        Self { pages, page: 0, offset: 0 }
    }

    fn peek(&self) -> Option<u8> {
        let mut page = self.page;
        let mut offset = self.offset;
        while let Some(bytes) = self.pages.get(page).map(String::as_bytes) {
            if let Some(byte) = bytes.get(offset) {
                return Some(*byte);
            }
            page += 1;
            offset = 0;
        }
        None
    }

    fn take(&mut self) -> Option<u8> {
        while let Some(bytes) = self.pages.get(self.page).map(String::as_bytes) {
            if let Some(byte) = bytes.get(self.offset) {
                self.offset += 1;
                return Some(*byte);
            }
            self.page += 1;
            self.offset = 0;
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.take();
        }
    }

    /// 📐️ Copies exactly ONE complete JSON value's text into `scratch`, refusing a value above the guest's
    /// per-request contiguous ceiling. String state and nesting depth are tracked so a brace inside a
    /// label can never close a value.
    fn capture(&mut self, scratch: &mut Vec<u8>) -> Result<(), Puzzle3dImportFault> {
        scratch.clear();
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        loop {
            let byte = self.take().ok_or(Puzzle3dImportFault::Payload)?;
            if scratch.len() >= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES {
                return Err(Puzzle3dImportFault::Element);
            }
            scratch.push(byte);
            if in_string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    in_string = false;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                continue;
            }
            match byte {
                b'"' => in_string = true,
                b'{' | b'[' => depth += 1,
                b'}' | b']' => {
                    depth = depth.checked_sub(1).ok_or(Puzzle3dImportFault::Payload)?;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                _ => {}
            }
            if depth == 0 && !matches!(self.peek(), Some(byte) if !byte.is_ascii_whitespace() && byte != b',' && byte != b'}' && byte != b']') {
                return Ok(());
            }
        }
    }
}

fn parse_captured(scratch: &[u8]) -> Result<Value, Puzzle3dImportFault> {
    let text = std::str::from_utf8(scratch).map_err(|_| Puzzle3dImportFault::Payload)?;
    parse(text).map_err(|_| Puzzle3dImportFault::Payload)
}

/// 📥️ Rebuilds the document's root JSON object from the paged owner without ever holding it as one
/// contiguous string: each root member is captured on its own, and an ARRAY member's elements are parsed
/// ONE AT A TIME into a growing `Vec<Value>`. The largest contiguous request the whole import makes is one
/// member — refused above [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`] with a named fault, so a document
/// with one enormous member is an answer the user reads rather than a heap the guest cannot grow.
pub fn puzzle3d_import_root_value(pages: &[String]) -> Result<Value, Puzzle3dImportFault> {
    let mut cursor = Puzzle3dImportPages::new(pages);
    let mut scratch: Vec<u8> = Vec::new();
    cursor.skip_whitespace();
    if cursor.take() != Some(b'{') {
        return Err(Puzzle3dImportFault::Payload);
    }
    let mut members: Vec<(String, Value)> = Vec::new();
    loop {
        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b'}') => {
                cursor.take();
                return Ok(object(members));
            }
            Some(b',') => {
                cursor.take();
                continue;
            }
            Some(b'"') => {}
            _ => return Err(Puzzle3dImportFault::Payload),
        }
        cursor.capture(&mut scratch)?;
        let key = parse_captured(&scratch)?.as_str().ok_or(Puzzle3dImportFault::Payload)?.to_string();
        cursor.skip_whitespace();
        if cursor.take() != Some(b':') {
            return Err(Puzzle3dImportFault::Payload);
        }
        cursor.skip_whitespace();
        if cursor.peek() == Some(b'[') {
            cursor.take();
            let mut items: Vec<Value> = Vec::new();
            loop {
                cursor.skip_whitespace();
                match cursor.peek() {
                    Some(b']') => {
                        cursor.take();
                        break;
                    }
                    Some(b',') => {
                        cursor.take();
                        continue;
                    }
                    None => return Err(Puzzle3dImportFault::Payload),
                    _ => {}
                }
                cursor.capture(&mut scratch)?;
                items.push(parse_captured(&scratch)?);
            }
            members.push((key, array(items)));
            continue;
        }
        cursor.capture(&mut scratch)?;
        members.push((key, parse_captured(&scratch)?));
    }
}

/// 📥️ The localized notice one refused import publishes. Every fault answers, none is silent.
fn notice_import_fault(ctx: &mut Puzzle3dActionCtx<'_>, fault: Puzzle3dImportFault) {
    match fault {
        Puzzle3dImportFault::Chunk | Puzzle3dImportFault::Capacity | Puzzle3dImportFault::Element => ctx.notice(|labels| labels.import_too_large.as_str()),
        Puzzle3dImportFault::Envelope | Puzzle3dImportFault::Gap => ctx.notice(|labels| labels.import_incomplete.as_str()),
        Puzzle3dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
    }
    ctx.abort = true;
}

/// 📥 Replaces the live fixture with the reassembled document as one document edit.
///
/// 🧩️ A chunk that does not close its run stages and ABORTS — no document edit and no history row for a
/// partial import — and every refusal publishes a named notice. Before ticket 26/09/02 wave B59 a
/// 145 924-byte import produced none of the three: the retained job's wire ladder spent one host step per
/// BYTE with an O(n) checkpoint on each, so the command never reached its decode and the import was a
/// silent no-op (`paneObjects=180→180`).
pub fn import_fixture(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(args) = args else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    let value = match args.get("payload").and_then(Value::as_str) {
        Some(text) => {
            let envelope = puzzle3d_import_envelope(args);
            match stage_import_chunk(&envelope, text) {
                Err(fault) => return notice_import_fault(ctx, fault),
                Ok(Puzzle3dImportStep::Staged { .. }) => {
                    ctx.abort = true;
                    return;
                }
                Ok(Puzzle3dImportStep::Complete(pages)) => match puzzle3d_import_root_value(&pages) {
                    Ok(value) => Some(value),
                    Err(fault) => return notice_import_fault(ctx, fault),
                },
            }
        }
        None => args
            .get("json")
            .cloned()
            .filter(|value| value.as_object().is_some())
            .or_else(|| args.get("fixture").cloned().filter(|value| value.as_object().is_some()))
            .or_else(|| args.get("payload").cloned().filter(|value| value.as_object().is_some())),
    };
    let Some(value) = value else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    let Ok(mut fixture) = Puzzle3dFixture::from_value(json::to_dsl_value(&value)) else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    if fixture.schema.is_empty() {
        fixture.schema = PUZZLE3D_FIXTURE_SCHEMA.into();
    }
    ctx.scene.fixture = fixture;
}
