//! 📥 Replaces the document with imported JSON, reassembled from the host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importFixture {payload, name, chunk, chunkCount}` per
//! [`IMPORT_CHUNK_BYTES`] page). The staging law is the 5d twin of `🧊️3d/…/📥️import-fixture` and
//! `◻️2d/…/📥️import-fixture`, with the chunk extent taken from the framework's OWN shell-side
//! constant rather than re-derived, so the guest measures exactly what the shell slices.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, Puzzle5dDocument, PUZZLE5D_SCHEMA};
use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use dsl::os_pack::json::Value;
use semio_framework::kernel::{import_payload_chunks, IMPORT_CHUNK_BYTES, IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_NAME, IMPORT_ARGUMENT_PAYLOAD};
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use serde_json::{Map, Value as JsonValue};
use std::sync::{Mutex, OnceLock};

//#region 🔖️Limits
/// 📏️ Bytes ONE inbound import chunk may carry — the framework's own shell-side slice extent
/// (`semio_framework::kernel::IMPORT_CHUNK_BYTES`, half the guest's contiguous ceiling, leaving the
/// other half for the escaped op envelope the chunk rides in), never a second literal.
pub const PUZZLE5D_IMPORT_CHUNK_BYTES: usize = IMPORT_CHUNK_BYTES;
/// 📏️ Largest document ONE import may reassemble — the SAME budget one export may stream
/// (`puzzle5d_export_segmented_budget_bytes`), so a file this app wrote is always a file this app
/// can read back. Capsule Dream is above it and is refused with a notice, in both directions.
pub const PUZZLE5D_IMPORT_TOTAL_BYTES: usize = PUZZLE_COMMAND_OUTPUT_BYTES;
/// 📦️ Chunks one import may stage — the whole budget at the chunk extent, never a second literal.
pub const PUZZLE5D_IMPORT_MAXIMUM_CHUNKS: usize = PUZZLE5D_IMPORT_TOTAL_BYTES.div_ceil(PUZZLE5D_IMPORT_CHUNK_BYTES);
/// 📦️ Import runs staged at once. One open run per document a user is importing into; an abandoned
/// run costs one slot until the oldest is evicted, never unbounded memory.
const PUZZLE5D_IMPORT_SLOTS: usize = 4;
//#endregion 🔖️Limits

//#region 🔖️Vocabulary
/// 🚫️ Why one import chunk was refused. Every arm becomes a localized shell notice in the command
/// arm — never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle5dImportFault {
    /// 🏷️ `chunk`/`chunkCount` are out of order, zero, or beyond [`PUZZLE5D_IMPORT_MAXIMUM_CHUNKS`].
    Envelope,
    /// 📏️ One chunk is larger than [`PUZZLE5D_IMPORT_CHUNK_BYTES`] — the host sliced by the wrong
    /// constant, or a whole document was sent unchunked.
    Chunk,
    /// 🕳️ The chunk does not continue the staged run; the broken run is dropped so the client
    /// re-opens at chunk 0 instead of resuming into bytes nobody can account for.
    Gap,
    /// 📦️ The run would exceed [`PUZZLE5D_IMPORT_TOTAL_BYTES`].
    Capacity,
    /// 🧊️ One JSON member of the reassembled document is itself larger than the guest's per-request
    /// contiguous ceiling, so it cannot be parsed without growing the fixed heap.
    Element,
    /// 🔤️ The reassembled bytes are not one JSON object, or not a puzzle 5d document.
    Payload,
}

impl Puzzle5dImportFault {
    /// 🏷️ Stable wire code, mirrored by the import notice and the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "puzzle5d-import-envelope",
            Self::Chunk => "puzzle5d-import-chunk",
            Self::Gap => "puzzle5d-import-gap",
            Self::Capacity => "puzzle5d-import-capacity",
            Self::Element => "puzzle5d-import-element",
            Self::Payload => "puzzle5d-import-payload",
        }
    }
}

/// 🧱️ What one accepted chunk did to its run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Puzzle5dImportStep {
    /// 🧱️ The chunk landed and the run is still open — no document edit. The command logs one unapplied row.
    Staged { next_chunk: usize, chunk_count: usize },
    /// ✅️ The chunk closed the run; the pages are the whole document, each one page of the paged owner.
    Complete(Vec<String>),
}

/// 🧵️ One import's open chunk run. `pages` IS the paged owner: one page per admitted chunk, each
/// bounded by [`PUZZLE5D_IMPORT_CHUNK_BYTES`], so the reassembled document never exists as one
/// contiguous block.
struct Puzzle5dImportSlot {
    name: String,
    chunk_count: usize,
    next_chunk: usize,
    bytes: usize,
    pages: Vec<String>,
    touched: u64,
}

struct Puzzle5dImportStaging {
    slots: Vec<Puzzle5dImportSlot>,
    sequence: u64,
}

fn import_staging() -> &'static Mutex<Puzzle5dImportStaging> {
    static STAGING: OnceLock<Mutex<Puzzle5dImportStaging>> = OnceLock::new();
    STAGING.get_or_init(|| Mutex::new(Puzzle5dImportStaging { slots: Vec::new(), sequence: 0 }))
}

/// 📥️ The chunk envelope one `importFixture` invocation carries. An invocation that declares none is
/// one whole-document chunk, which is the only shape that fits [`PUZZLE5D_IMPORT_CHUNK_BYTES`] alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle5dImportEnvelope {
    pub name: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

/// 📥️ Reads the envelope off the invocation args, defaulting an undeclared one to the single-chunk
/// shape. The argument spellings are the framework's own (`IMPORT_ARGUMENT_*`), so no shell can send
/// a second spelling of the same envelope.
pub fn puzzle5d_import_envelope(args: &Value) -> Puzzle5dImportEnvelope {
    let number = |key: &str| args.get(key).and_then(Value::as_f64).filter(|value| *value >= 0.0).map(|value| value as usize);
    Puzzle5dImportEnvelope {
        name: args.get(IMPORT_ARGUMENT_NAME).and_then(Value::as_str).unwrap_or_default().to_string(),
        chunk: number(IMPORT_ARGUMENT_CHUNK).unwrap_or(0),
        chunk_count: number(IMPORT_ARGUMENT_CHUNK_COUNT).unwrap_or(1),
    }
}

/// 📥️ Slices one document's JSON into the chunks the host sends, through the framework's own
/// `import_payload_chunks` — the guest's laws therefore exercise the exact wire the shell builds.
pub fn puzzle5d_import_chunks(payload: &str) -> Vec<String> {
    import_payload_chunks(payload).into_iter().map(|chunk| chunk.payload).collect()
}
//#endregion 🔖️Vocabulary

//#region 🔖️Staging
/// 🧩️ Admits one chunk of an import run into the staging area, and hands back every page the moment
/// the run closes. A run is keyed by `(name, chunkCount)`: a client that restarts an import opens a
/// different run instead of corrupting the staged one, and a chunk the run already admitted is a
/// RETRANSMISSION acknowledged at the cursor it stands on rather than a gap that costs the whole
/// upload again.
pub fn stage_import_chunk(envelope: &Puzzle5dImportEnvelope, text: &str) -> Result<Puzzle5dImportStep, Puzzle5dImportFault> {
    if envelope.chunk_count == 0 || envelope.chunk_count > PUZZLE5D_IMPORT_MAXIMUM_CHUNKS || envelope.chunk >= envelope.chunk_count {
        return Err(Puzzle5dImportFault::Envelope);
    }
    if text.len() > PUZZLE5D_IMPORT_CHUNK_BYTES {
        return Err(Puzzle5dImportFault::Chunk);
    }
    if envelope.chunk_count == 1 {
        return Ok(Puzzle5dImportStep::Complete(vec![text.to_string()]));
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
            return Ok(Puzzle5dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
        }
        Some(index) if envelope.chunk != 0 => {
            staging.slots.swap_remove(index);
            return Err(Puzzle5dImportFault::Gap);
        }
        Some(index) => {
            staging.slots[index] = Puzzle5dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence };
            index
        }
        None if envelope.chunk != 0 => return Err(Puzzle5dImportFault::Gap),
        None => {
            if staging.slots.len() >= PUZZLE5D_IMPORT_SLOTS {
                let stale = staging.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Puzzle5dImportFault::Capacity)?;
                staging.slots.swap_remove(stale);
            }
            staging.slots.push(Puzzle5dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence });
            staging.slots.len() - 1
        }
    };
    let slot = &mut staging.slots[index];
    if slot.bytes.saturating_add(text.len()) > PUZZLE5D_IMPORT_TOTAL_BYTES {
        staging.slots.swap_remove(index);
        return Err(Puzzle5dImportFault::Capacity);
    }
    slot.bytes = slot.bytes.saturating_add(text.len());
    slot.pages.push(text.to_string());
    slot.next_chunk = envelope.chunk.saturating_add(1);
    slot.touched = sequence;
    if slot.next_chunk < slot.chunk_count {
        return Ok(Puzzle5dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
    }
    let closed = staging.slots.swap_remove(index);
    Ok(Puzzle5dImportStep::Complete(closed.pages))
}

/// 🔎️ Open chunk runs, for the unit laws and the staging census.
pub fn staged_import_runs() -> Vec<(String, usize, usize, usize)> {
    let staging = import_staging().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    staging.slots.iter().map(|slot| (slot.name.clone(), slot.next_chunk, slot.chunk_count, slot.bytes)).collect()
}
//#endregion 🔖️Staging

//#region 🔖️PagedParse
/// 📄️ A byte cursor over the paged owner — the pages are read in order and never joined.
struct Puzzle5dImportPages<'a> {
    pages: &'a [String],
    page: usize,
    offset: usize,
}

impl<'a> Puzzle5dImportPages<'a> {
    fn new(pages: &'a [String]) -> Self {
        Self { pages, page: 0, offset: 0 }
    }

    fn peek(&self) -> Option<u8> {
        let (mut page, mut offset) = (self.page, self.offset);
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

    /// 📐️ Copies exactly ONE complete JSON value's text into `scratch`, refusing a value above the
    /// guest's per-request contiguous ceiling. String state and nesting depth are tracked so a brace
    /// inside a label can never close a value.
    fn capture(&mut self, scratch: &mut Vec<u8>) -> Result<(), Puzzle5dImportFault> {
        scratch.clear();
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        loop {
            let byte = self.take().ok_or(Puzzle5dImportFault::Payload)?;
            if scratch.len() >= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES {
                return Err(Puzzle5dImportFault::Element);
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
                    depth = depth.checked_sub(1).ok_or(Puzzle5dImportFault::Payload)?;
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

fn parse_captured(scratch: &[u8]) -> Result<JsonValue, Puzzle5dImportFault> {
    serde_json::from_slice(scratch).map_err(|_| Puzzle5dImportFault::Payload)
}

/// 📥️ Rebuilds the document's root JSON object from the paged owner without ever holding it as one
/// contiguous string: each root member is captured on its own, and an ARRAY member's elements are
/// parsed ONE AT A TIME into a growing `Vec`. The largest contiguous request the whole import makes
/// is one member — refused above [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`] with a named fault, so a
/// document with one enormous member is an answer the user reads rather than a heap the guest cannot
/// grow.
pub fn puzzle5d_import_root_value(pages: &[String]) -> Result<JsonValue, Puzzle5dImportFault> {
    let mut cursor = Puzzle5dImportPages::new(pages);
    let mut scratch: Vec<u8> = Vec::new();
    cursor.skip_whitespace();
    if cursor.take() != Some(b'{') {
        return Err(Puzzle5dImportFault::Payload);
    }
    let mut members = Map::new();
    loop {
        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b'}') => {
                cursor.take();
                return Ok(JsonValue::Object(members));
            }
            Some(b',') => {
                cursor.take();
                continue;
            }
            Some(b'"') => {}
            _ => return Err(Puzzle5dImportFault::Payload),
        }
        cursor.capture(&mut scratch)?;
        let key = parse_captured(&scratch)?.as_str().ok_or(Puzzle5dImportFault::Payload)?.to_string();
        cursor.skip_whitespace();
        if cursor.take() != Some(b':') {
            return Err(Puzzle5dImportFault::Payload);
        }
        cursor.skip_whitespace();
        if cursor.peek() == Some(b'[') {
            cursor.take();
            let mut items: Vec<JsonValue> = Vec::new();
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
                    None => return Err(Puzzle5dImportFault::Payload),
                    _ => {}
                }
                cursor.capture(&mut scratch)?;
                items.push(parse_captured(&scratch)?);
            }
            members.insert(key, JsonValue::Array(items));
            continue;
        }
        cursor.capture(&mut scratch)?;
        members.insert(key, parse_captured(&scratch)?);
    }
}
//#endregion 🔖️PagedParse

//#region 🔖️Command
/// 📥 The document an import payload carries: a staged/complete chunk run, or an inline `json`/
/// `fixture` object — admitted only when it really is a puzzle 5d document (its `schema`, or a
/// `parts` array).
fn puzzle5d_import_value(args: &Value) -> Result<Option<JsonValue>, Puzzle5dImportFault> {
    let value = match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) => match stage_import_chunk(&puzzle5d_import_envelope(args), text)? {
            Puzzle5dImportStep::Staged { .. } => return Ok(None),
            Puzzle5dImportStep::Complete(pages) => puzzle5d_import_root_value(&pages)?,
        },
        None => {
            let inline = args.get("json").or_else(|| args.get("fixture")).or_else(|| args.get(IMPORT_ARGUMENT_PAYLOAD)).ok_or(Puzzle5dImportFault::Payload)?;
            let inline = serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(inline));
            if !inline.is_object() {
                return Err(Puzzle5dImportFault::Payload);
            }
            inline
        }
    };
    let is_document = value.get("schema").and_then(JsonValue::as_str) == Some(PUZZLE5D_SCHEMA) || value.get("parts").is_some_and(JsonValue::is_array);
    if !is_document {
        return Err(Puzzle5dImportFault::Payload);
    }
    Ok(Some(value))
}

/// 📥 Replaces the live document with the reassembled one as ONE document edit.
///
/// 🧩️ A chunk that does not close its run stages and ABORTS — no document edit for a partial import.
/// The command still logs one unapplied history row, the same empty-emit row every lane-less
/// Mutation owes, and every refusal publishes a named, localized notice: silence is the one
/// answer an import must never give.
pub fn import_fixture(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let outcome = args.ok_or(Puzzle5dImportFault::Payload).and_then(puzzle5d_import_value);
    let mut document: Puzzle5dDocument = match outcome {
        Ok(Some(value)) => match serde_json::from_value(value) {
            Ok(document) => document,
            Err(_) => return refuse(ctx, Puzzle5dImportFault::Payload),
        },
        Ok(None) => {
            ctx.abort = true;
            return;
        }
        Err(fault) => return refuse(ctx, fault),
    };
    if document.schema.is_empty() {
        document.schema = PUZZLE5D_SCHEMA.into();
    }
    ctx.scene.document = document;
}

/// 📥️ The one localized notice a refused import publishes, then the abort that keeps the document
/// and the history untouched.
fn refuse(ctx: &mut Puzzle5dActionCtx<'_>, fault: Puzzle5dImportFault) {
    match fault {
        Puzzle5dImportFault::Chunk | Puzzle5dImportFault::Capacity | Puzzle5dImportFault::Element => ctx.notice(|labels| labels.import_too_large.as_str()),
        Puzzle5dImportFault::Envelope | Puzzle5dImportFault::Gap => ctx.notice(|labels| labels.import_incomplete.as_str()),
        Puzzle5dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
    }
    ctx.abort = true;
}
//#endregion 🔖️Command
