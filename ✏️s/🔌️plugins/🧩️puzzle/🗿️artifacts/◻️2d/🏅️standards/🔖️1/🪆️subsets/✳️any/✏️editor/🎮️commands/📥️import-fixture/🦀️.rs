//! 📥 Replaces the document with imported fixture JSON, reassembled from the host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importFixture {payload, name, chunk, chunkCount}` per
//! `IMPORT_CHUNK_BYTES` page). The staging law is the 2d twin of `🧊️3d/…/📥️import-fixture`.

use crate::editor::puzzle2d::{Puzzle2dActionCtx, PUZZLE2D_FIXTURE_SCHEMA};
use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use serde_json::{Map, Value};
use std::sync::{Mutex, OnceLock};

//#region 🔖️Limits
/// 📏️ Bytes ONE inbound import chunk may carry — half the guest's contiguous ceiling, leaving the other
/// half for the escaped op envelope the chunk rides in (`semio_framework::kernel::IMPORT_CHUNK_BYTES`).
pub const PUZZLE2D_IMPORT_CHUNK_BYTES: usize = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;
/// 📏️ Largest document ONE import may reassemble — the same budget one export may stream.
pub const PUZZLE2D_IMPORT_TOTAL_BYTES: usize = PUZZLE_COMMAND_OUTPUT_BYTES;
pub const PUZZLE2D_IMPORT_MAXIMUM_CHUNKS: usize = PUZZLE2D_IMPORT_TOTAL_BYTES.div_ceil(PUZZLE2D_IMPORT_CHUNK_BYTES);
const PUZZLE2D_IMPORT_SLOTS: usize = 4;
//#endregion 🔖️Limits

//#region 🔖️Vocabulary
/// 🚫️ Why one import chunk was refused — every arm becomes a shell notice, never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dImportFault {
    Envelope,
    Chunk,
    Gap,
    Capacity,
    Element,
    Payload,
}

impl Puzzle2dImportFault {
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "puzzle2d-import-envelope",
            Self::Chunk => "puzzle2d-import-chunk",
            Self::Gap => "puzzle2d-import-gap",
            Self::Capacity => "puzzle2d-import-capacity",
            Self::Element => "puzzle2d-import-element",
            Self::Payload => "puzzle2d-import-payload",
        }
    }

    fn notice(self) -> &'static str {
        match self {
            Self::Chunk | Self::Capacity | Self::Element => "The fixture is too large to import",
            Self::Envelope | Self::Gap => "The import arrived incomplete — open the file again",
            Self::Payload => "The file is not a puzzle 2d fixture",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Puzzle2dImportStep {
    Staged { next_chunk: usize, chunk_count: usize },
    Complete(Vec<String>),
}

struct Puzzle2dImportSlot {
    name: String,
    chunk_count: usize,
    next_chunk: usize,
    bytes: usize,
    pages: Vec<String>,
    touched: u64,
}

struct Puzzle2dImportStaging {
    slots: Vec<Puzzle2dImportSlot>,
    sequence: u64,
}

fn import_staging() -> &'static Mutex<Puzzle2dImportStaging> {
    static STAGING: OnceLock<Mutex<Puzzle2dImportStaging>> = OnceLock::new();
    STAGING.get_or_init(|| Mutex::new(Puzzle2dImportStaging { slots: Vec::new(), sequence: 0 }))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle2dImportEnvelope {
    pub name: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

/// 📥️ Reads the envelope off the invocation args, defaulting an undeclared one to the single-chunk shape.
pub fn puzzle2d_import_envelope(args: &Value) -> Puzzle2dImportEnvelope {
    let number = |key: &str| args.get(key).and_then(Value::as_f64).filter(|value| *value >= 0.0).map(|value| value as usize);
    Puzzle2dImportEnvelope { name: args.get("name").and_then(Value::as_str).unwrap_or_default().to_string(), chunk: number("chunk").unwrap_or(0), chunk_count: number("chunkCount").unwrap_or(1) }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Staging
/// 🧩️ Admits one chunk of an import run, and hands back every page the moment the run closes. A run is
/// keyed by `(name, chunkCount)`; a chunk the run already admitted is a retransmission acknowledged at
/// the cursor, a chunk past the cursor is a gap that drops the run.
pub fn stage_import_chunk(envelope: &Puzzle2dImportEnvelope, text: &str) -> Result<Puzzle2dImportStep, Puzzle2dImportFault> {
    if envelope.chunk_count == 0 || envelope.chunk_count > PUZZLE2D_IMPORT_MAXIMUM_CHUNKS || envelope.chunk >= envelope.chunk_count {
        return Err(Puzzle2dImportFault::Envelope);
    }
    if text.len() > PUZZLE2D_IMPORT_CHUNK_BYTES {
        return Err(Puzzle2dImportFault::Chunk);
    }
    if envelope.chunk_count == 1 {
        return Ok(Puzzle2dImportStep::Complete(vec![text.to_string()]));
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
            return Ok(Puzzle2dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
        }
        Some(index) if envelope.chunk != 0 => {
            staging.slots.swap_remove(index);
            return Err(Puzzle2dImportFault::Gap);
        }
        Some(index) => {
            staging.slots[index] = Puzzle2dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence };
            index
        }
        None if envelope.chunk != 0 => return Err(Puzzle2dImportFault::Gap),
        None => {
            if staging.slots.len() >= PUZZLE2D_IMPORT_SLOTS {
                let stale = staging.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Puzzle2dImportFault::Capacity)?;
                staging.slots.swap_remove(stale);
            }
            staging.slots.push(Puzzle2dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence });
            staging.slots.len() - 1
        }
    };
    let slot = &mut staging.slots[index];
    if slot.bytes.saturating_add(text.len()) > PUZZLE2D_IMPORT_TOTAL_BYTES {
        staging.slots.swap_remove(index);
        return Err(Puzzle2dImportFault::Capacity);
    }
    slot.bytes = slot.bytes.saturating_add(text.len());
    slot.pages.push(text.to_string());
    slot.next_chunk = envelope.chunk.saturating_add(1);
    slot.touched = sequence;
    if slot.next_chunk < slot.chunk_count {
        return Ok(Puzzle2dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
    }
    let closed = staging.slots.swap_remove(index);
    Ok(Puzzle2dImportStep::Complete(closed.pages))
}

/// 🔎️ Open chunk runs, for the unit laws.
pub fn staged_import_runs() -> Vec<(String, usize, usize, usize)> {
    let staging = import_staging().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    staging.slots.iter().map(|slot| (slot.name.clone(), slot.next_chunk, slot.chunk_count, slot.bytes)).collect()
}
//#endregion 🔖️Staging

//#region 🔖️PagedParse
/// 📄️ A byte cursor over the paged owner — the pages are read in order and never joined.
struct Puzzle2dImportPages<'a> {
    pages: &'a [String],
    page: usize,
    offset: usize,
}

impl<'a> Puzzle2dImportPages<'a> {
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

    /// 📐️ Copies exactly ONE complete JSON value's text into `scratch`, refusing a value above the guest's
    /// per-request contiguous ceiling.
    fn capture(&mut self, scratch: &mut Vec<u8>) -> Result<(), Puzzle2dImportFault> {
        scratch.clear();
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        loop {
            let byte = self.take().ok_or(Puzzle2dImportFault::Payload)?;
            if scratch.len() >= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES {
                return Err(Puzzle2dImportFault::Element);
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
                    depth = depth.checked_sub(1).ok_or(Puzzle2dImportFault::Payload)?;
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

fn parse_captured(scratch: &[u8]) -> Result<Value, Puzzle2dImportFault> {
    serde_json::from_slice(scratch).map_err(|_| Puzzle2dImportFault::Payload)
}

/// 📥️ Rebuilds the document's root JSON object from the paged owner without ever holding it as one
/// contiguous string: each root member is captured on its own, and an ARRAY member's elements are parsed
/// one at a time.
pub fn puzzle2d_import_root_value(pages: &[String]) -> Result<Value, Puzzle2dImportFault> {
    let mut cursor = Puzzle2dImportPages::new(pages);
    let mut scratch: Vec<u8> = Vec::new();
    cursor.skip_whitespace();
    if cursor.take() != Some(b'{') {
        return Err(Puzzle2dImportFault::Payload);
    }
    let mut members = Map::new();
    loop {
        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b'}') => {
                cursor.take();
                return Ok(Value::Object(members));
            }
            Some(b',') => {
                cursor.take();
                continue;
            }
            Some(b'"') => {}
            _ => return Err(Puzzle2dImportFault::Payload),
        }
        cursor.capture(&mut scratch)?;
        let key = parse_captured(&scratch)?.as_str().ok_or(Puzzle2dImportFault::Payload)?.to_string();
        cursor.skip_whitespace();
        if cursor.take() != Some(b':') {
            return Err(Puzzle2dImportFault::Payload);
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
                    None => return Err(Puzzle2dImportFault::Payload),
                    _ => {}
                }
                cursor.capture(&mut scratch)?;
                items.push(parse_captured(&scratch)?);
            }
            members.insert(key, Value::Array(items));
            continue;
        }
        cursor.capture(&mut scratch)?;
        members.insert(key, parse_captured(&scratch)?);
    }
}
//#endregion 🔖️PagedParse

//#region 🔖️Command
/// 📥 The fixture an import payload carries: a staged/complete chunk run, or an inline `json`/`fixture`
/// object — accepted only when it is a puzzle 2d fixture (its `schema` or a `nodes` array).
fn puzzle2d_import_value(args: &Value) -> Result<Option<Value>, Puzzle2dImportFault> {
    let value = match args.get("payload").and_then(Value::as_str) {
        Some(text) => match stage_import_chunk(&puzzle2d_import_envelope(args), text)? {
            Puzzle2dImportStep::Staged { .. } => return Ok(None),
            Puzzle2dImportStep::Complete(pages) => puzzle2d_import_root_value(&pages)?,
        },
        None => args.get("json").or_else(|| args.get("fixture")).or_else(|| args.get("payload")).cloned().filter(Value::is_object).ok_or(Puzzle2dImportFault::Payload)?,
    };
    let is_fixture = value.get("schema").and_then(Value::as_str) == Some(PUZZLE2D_FIXTURE_SCHEMA) || value.get("nodes").is_some_and(Value::is_array);
    if !is_fixture {
        return Err(Puzzle2dImportFault::Payload);
    }
    Ok(Some(value))
}

/// 📥 Replaces the document with the reassembled fixture as one document edit. A chunk that does not close
/// its run stages and changes nothing; every refusal publishes a named notice, never a fault.
pub fn import_fixture(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let outcome = args.ok_or(Puzzle2dImportFault::Payload).and_then(puzzle2d_import_value);
    let mut fixture = match outcome {
        Ok(Some(fixture)) => fixture,
        Ok(None) => {
            *ctx.ui_scope = UiDirtyScope::None;
            return;
        }
        Err(fault) => {
            ctx.effects.push(Effect::Notify { message: format!("{} ({})", fault.notice(), fault.code()) });
            *ctx.ui_scope = UiDirtyScope::None;
            return;
        }
    };
    if let Some(object) = fixture.as_object_mut() {
        object.entry("schema").or_insert_with(|| Value::String(PUZZLE2D_FIXTURE_SCHEMA.into()));
        object.entry("edges").or_insert_with(|| Value::Array(Vec::new()));
    }
    ctx.scene.fixture = fixture;
}
//#endregion 🔖️Command
