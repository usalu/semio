//! 🧮️ The protocol byte walker: deep-walks bytes against a `ProtocolFile` with walk-wide state and exact mismatch offsets.

use super::{Block, Cond, CondOp, Count, DispatchOrder, Field, Framing, NestedDispatch, Prim, ProtocolFile, ProtocolMismatch, ProtocolTrace, RepeatDispatch, parse_protocol, walk_literal_protocol_controlled};

//#region 📡️ProtocolWalk
/// 🧮️ P2-M2 items 3+6: walk-wide state threaded through the ENTIRE `walk_protocol` pass
/// (every block, in order) — not reset per block/call as the pre-M2 `walk_fields`-local `HashMap`
/// was. `env` makes a LATER block's `Count::Field`/`Cond` resolve against a value decoded by any
/// EARLIER block (las's VLR/point-record repeat counts from the header; gif89a's GCE state carried
/// to the next block; bmp's palette/row counts). `big_endian` is the runtime endian mode a
/// `Prim::Endian` field can flip for the remainder of the walk (TIFF's `II`/`MM`); plain `U16`/
/// `U32`/`U64`/`I32`/`I64`/`F32`/`F64` obey it, `*Be` variants (item 2) always ignore it.
#[derive(Default)]
struct WalkState {
    pub(super) env: std::collections::HashMap<String, u64>,
    big_endian: bool,
}

pub(super) fn prim_fixed_width(prim: &Prim) -> Option<usize> {
    match prim {
        Prim::U8 => Some(1),
        Prim::U16 | Prim::U16Be => Some(2),
        Prim::U32 | Prim::I32 | Prim::F32 | Prim::U32Be | Prim::I32Be | Prim::F32Be => Some(4),
        Prim::U64 | Prim::I64 | Prim::F64 | Prim::U64Be | Prim::I64Be | Prim::F64Be => Some(8),
        Prim::Fixed(n) => Some(*n),
        Prim::MarkerScan(_) => Some(1),
        Prim::Endian(arms) => Some(arms.first().map_or(0, |(k, _)| k.len())),
        Prim::Varint | Prim::Zigzag | Prim::Tag | Prim::Bytes | Prim::Utf8 | Prim::Array(_, _) | Prim::Ref(_) => None,
    }
}

fn decode_u16(slice: &[u8], big_endian: bool) -> u16 {
    let bytes = [slice[0], slice[1]];
    if big_endian {
        u16::from_be_bytes(bytes)
    } else {
        u16::from_le_bytes(bytes)
    }
}

fn decode_u32(slice: &[u8], big_endian: bool) -> u32 {
    let bytes: [u8; 4] = slice.try_into().unwrap();
    if big_endian {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    }
}

fn decode_u64(slice: &[u8], big_endian: bool) -> u64 {
    let bytes: [u8; 8] = slice.try_into().unwrap();
    if big_endian {
        u64::from_be_bytes(bytes)
    } else {
        u64::from_le_bytes(bytes)
    }
}

// 🚫️async: E1 pure error constructor, consumed by `Option::ok_or_else` sync closures at several
// call sites across this file's protocol-walk decoder — see R9
pub(super) fn mismatch(offset: usize, message: impl Into<String>) -> ProtocolMismatch {
    ProtocolMismatch { offset, message: message.into() }
}

fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolMismatch> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() {
            return Err(mismatch(*pos, "truncated varint"));
        }
        let byte = bytes[*pos];
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(mismatch(*pos, "varint overflow"));
        }
    }
}

fn need<'a>(bytes: &'a [u8], pos: usize, n: usize, what: &str) -> Result<&'a [u8], ProtocolMismatch> {
    if pos + n > bytes.len() {
        return Err(mismatch(pos, format!("truncated {what}: need {n} bytes, have {}", bytes.len().saturating_sub(pos))));
    }
    Ok(&bytes[pos..pos + n])
}

fn trailing_reserved(blocks: &[Block], from: usize) -> usize {
    let mut reserved = 0usize;
    for block in &blocks[from..] {
        match block {
            Block::Footer(n) => reserved += *n,
            Block::Chain(prim) => reserved += prim_fixed_width(prim).unwrap_or(0),
            Block::Struct { .. } | Block::Enum { .. } => {}
            Block::Header(_) | Block::Segment { .. } | Block::Record { .. } | Block::Repeat { .. } | Block::BackwardScan { .. } | Block::JumpTo { .. } => break,
        }
    }
    reserved
}

fn resolve_count(count: &Count, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<usize, ProtocolMismatch> {
    match count {
        Count::Fixed(n) => Ok(*n),
        Count::Varint => Err(mismatch(offset, "Count::Varint must be read from the byte stream, not resolved from env")),
        Count::Field(name) => env.get(name).map(|v| *v as usize).ok_or_else(|| mismatch(offset, format!("unknown count field `{name}`"))),
    }
}

/// 🔀️ P2-M2 item 4: evaluate a field/segment presence guard against the walk-wide env.
fn eval_cond(cond: &Cond, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<bool, ProtocolMismatch> {
    let actual = *env.get(&cond.field).ok_or_else(|| mismatch(offset, format!("condition references unknown field `{}`", cond.field)))?;
    Ok(match cond.op {
        CondOp::Eq => actual == cond.value,
        CondOp::Ne => actual != cond.value,
        CondOp::Lt => actual < cond.value,
        CondOp::Le => actual <= cond.value,
        CondOp::Gt => actual > cond.value,
        CondOp::Ge => actual >= cond.value,
        CondOp::BitsSet => actual & cond.value == cond.value,
    })
}

fn walk_prim(prim: &Prim, bytes: &[u8], pos: &mut usize, state: &mut WalkState, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            need(bytes, *pos, 1, "u8")?;
            *pos += 1;
        }
        Prim::U16 | Prim::U16Be => {
            need(bytes, *pos, 2, "u16")?;
            *pos += 2;
        }
        Prim::U32 | Prim::I32 | Prim::F32 | Prim::U32Be | Prim::I32Be | Prim::F32Be => {
            need(bytes, *pos, 4, "u32/i32/f32")?;
            *pos += 4;
        }
        Prim::U64 | Prim::I64 | Prim::F64 | Prim::U64Be | Prim::I64Be | Prim::F64Be => {
            need(bytes, *pos, 8, "u64/i64/f64")?;
            *pos += 8;
        }
        Prim::Fixed(n) => {
            need(bytes, *pos, *n, "fixed")?;
            *pos += *n;
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => {
            let _ = read_varint_u64(bytes, pos)?;
        }
        Prim::Bytes | Prim::Utf8 => {
            let end = bytes.len().saturating_sub(reserved_tail);
            if *pos > end {
                return Err(mismatch(*pos, "bytes field overlaps trailing reserved region"));
            }
            *pos = end;
        }
        Prim::Array(inner, count) => {
            let n = match count {
                Count::Varint => read_varint_u64(bytes, pos)? as usize,
                other => resolve_count(other, &state.env, *pos)?,
            };
            if matches!(inner.as_ref(), Prim::U8) {
                need(bytes, *pos, n, "byte array")?;
                *pos += n;
            } else {
                for _ in 0..n {
                    walk_prim(inner, bytes, pos, state, reserved_tail)?;
                }
            }
        }
        Prim::Ref(name) => return Err(mismatch(*pos, format!("unresolved protocol Ref({name}) during walk"))),
        // P2-M2 item 1c.
        Prim::MarkerScan(prefix) => {
            while *pos < bytes.len() && bytes[*pos] == *prefix {
                *pos += 1;
            }
            need(bytes, *pos, 1, "marker byte")?;
            *pos += 1;
        }
        // P2-M2 item 6.
        Prim::Endian(arms) => {
            let width = arms.first().map_or(0, |(k, _)| k.len());
            let slice = need(bytes, *pos, width, "endian marker")?;
            match arms.iter().find(|(k, _)| k.as_bytes() == slice) {
                Some((_, be)) => state.big_endian = *be,
                None => return Err(mismatch(*pos, format!("unrecognized endianness marker bytes {slice:?}"))),
            }
            *pos += width;
        }
    }
    Ok(())
}

/// 🏷️ P2-M2 item 1: reads a discriminator's exact raw bytes (no numeric decode — arm tags
/// are compared byte-for-byte, see [`parse_tag_value`]) and advances `pos` past them.
fn read_raw_prim_bytes(prim: &Prim, bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, ProtocolMismatch> {
    match prim {
        Prim::MarkerScan(prefix) => {
            while *pos < bytes.len() && bytes[*pos] == *prefix {
                *pos += 1;
            }
            let slice = need(bytes, *pos, 1, "marker byte")?.to_vec();
            *pos += 1;
            Ok(slice)
        }
        Prim::Fixed(n) => {
            let slice = need(bytes, *pos, *n, "fixed discriminator")?.to_vec();
            *pos += n;
            Ok(slice)
        }
        other => {
            let width = prim_fixed_width(other).ok_or_else(|| mismatch(*pos, format!("{other:?} has no fixed width, cannot be used as a discriminator")))?;
            let slice = need(bytes, *pos, width, "discriminator")?.to_vec();
            *pos += width;
            Ok(slice)
        }
    }
}

/// 🔢️ Reads a numeric scalar (length/count field) honoring [`WalkState::big_endian`] for
/// the plain (non-`Be`) variants — used by `repeat`'s `length` directive and, via [`walk_fields`],
/// for ordinary `Count::Field`-producing fields.
fn read_scalar_prim(prim: &Prim, bytes: &[u8], pos: &mut usize, state: &WalkState) -> Result<u64, ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            let slice = need(bytes, *pos, 1, "u8")?;
            let v = u64::from(slice[0]);
            *pos += 1;
            Ok(v)
        }
        Prim::U16 => {
            let slice = need(bytes, *pos, 2, "u16")?;
            let v = u64::from(decode_u16(slice, state.big_endian));
            *pos += 2;
            Ok(v)
        }
        Prim::U16Be => {
            let slice = need(bytes, *pos, 2, "u16be")?;
            let v = u64::from(decode_u16(slice, true));
            *pos += 2;
            Ok(v)
        }
        Prim::U32 => {
            let slice = need(bytes, *pos, 4, "u32")?;
            let v = u64::from(decode_u32(slice, state.big_endian));
            *pos += 4;
            Ok(v)
        }
        Prim::U32Be => {
            let slice = need(bytes, *pos, 4, "u32be")?;
            let v = u64::from(decode_u32(slice, true));
            *pos += 4;
            Ok(v)
        }
        Prim::U64 => {
            let slice = need(bytes, *pos, 8, "u64")?;
            let v = decode_u64(slice, state.big_endian);
            *pos += 8;
            Ok(v)
        }
        Prim::U64Be => {
            let slice = need(bytes, *pos, 8, "u64be")?;
            let v = decode_u64(slice, true);
            *pos += 8;
            Ok(v)
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => read_varint_u64(bytes, pos),
        other => Err(mismatch(*pos, format!("{other:?} cannot be read as a scalar length/count"))),
    }
}

/// 🔎️ P2-M2 item 5a: the rightmost occurrence of `pattern` in `bytes` — ZIP's EOCD is
/// located by scanning BACKWARD from EOF because its preceding comment field is 0-65535 bytes, so
/// its start is unknowable except by finding the EOCD's own magic first.
fn find_last_occurrence(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > bytes.len() {
        return None;
    }
    (0..=bytes.len() - pattern.len()).rev().find(|&i| &bytes[i..i + pattern.len()] == pattern)
}

fn walk_fields(fields: &[Field], bytes: &[u8], pos: &mut usize, state: &mut WalkState, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    for (index, field) in fields.iter().enumerate() {
        // P2-M2 item 4: a field absent under its guard is simply skipped — not read at all.
        if let Some(cond) = &field.cond {
            if !eval_cond(cond, &state.env, *pos)? {
                continue;
            }
        }
        let field_reserved = if index + 1 == fields.len() {
            reserved_tail
        } else {
            let mut tail_width = 0;
            for f in &fields[index + 1..] {
                tail_width += prim_fixed_width(&f.ty).unwrap_or(0);
            }
            reserved_tail + tail_width
        };
        match &field.ty {
            Prim::U8 => {
                let slice = need(bytes, *pos, 1, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(slice[0]));
                *pos += 1;
            }
            // P2-M2 item 6: plain U16/U32/U64 honor the walker's current runtime endian mode.
            Prim::U16 => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                let value = u64::from(decode_u16(slice, state.big_endian));
                state.env.insert(field.name.clone(), value);
                *pos += 2;
            }
            Prim::U32 => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                let value = u64::from(decode_u32(slice, state.big_endian));
                state.env.insert(field.name.clone(), value);
                *pos += 4;
            }
            Prim::U64 => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                let value = decode_u64(slice, state.big_endian);
                state.env.insert(field.name.clone(), value);
                *pos += 8;
            }
            // P2-M2 item 2: *Be variants are ALWAYS big-endian, regardless of the runtime mode.
            Prim::U16Be => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(decode_u16(slice, true)));
                *pos += 2;
            }
            Prim::U32Be => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(decode_u32(slice, true)));
                *pos += 4;
            }
            Prim::U64Be => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                state.env.insert(field.name.clone(), decode_u64(slice, true));
                *pos += 8;
            }
            Prim::Varint | Prim::Tag | Prim::Zigzag => {
                let value = read_varint_u64(bytes, pos)?;
                state.env.insert(field.name.clone(), value);
            }
            // P2-M2 item 6: the endian-marker field itself — mutates walker state, binds no value.
            Prim::Endian(arms) => {
                let width = arms.first().map_or(0, |(k, _)| k.len());
                let slice = need(bytes, *pos, width, "endian marker")?;
                match arms.iter().find(|(k, _)| k.as_bytes() == slice) {
                    Some((_, be)) => state.big_endian = *be,
                    None => return Err(mismatch(*pos, format!("unrecognized endianness marker bytes {slice:?}"))),
                }
                *pos += width;
            }
            other => walk_prim(other, bytes, pos, state, field_reserved)?,
        }
    }
    Ok(())
}

/// 🌿️ P2-M2 item 1b: single-shot (non-repeating) second-level dispatch — GIF 89a's
/// extension-introducer arm dispatches again on the label byte.
fn walk_nested_dispatch(nested: &NestedDispatch, bytes: &[u8], pos: &mut usize, state: &mut WalkState) -> Result<(), ProtocolMismatch> {
    let tag = read_raw_prim_bytes(&nested.discriminator, bytes, pos)?;
    match nested.arms.iter().find(|arm| arm.tag == tag) {
        Some(arm) => {
            walk_fields(&arm.fields, bytes, pos, state, 0)?;
            if let Some(deeper) = &arm.nested {
                walk_nested_dispatch(deeper, bytes, pos, state)?;
            }
            Ok(())
        }
        None => Err(mismatch(*pos, format!("unrecognized nested discriminator {tag:?} in `{}`", nested.name))),
    }
}

/// 🔁️ P2-M2 item 1: read discriminator (+ optional length, per `order`), dispatch into a
/// known arm's fields or skip an unrecognized discriminator's declared `length` as opaque bytes,
/// repeat until EOF or `until`'s sentinel discriminator value is seen.
fn walk_repeat(dispatch: &RepeatDispatch, bytes: &[u8], pos: &mut usize, state: &mut WalkState) -> Result<(), ProtocolMismatch> {
    loop {
        if *pos >= bytes.len() {
            break;
        }
        let iter_start = *pos;
        let (tag, length_value) = match dispatch.order {
            DispatchOrder::LengthFirst => {
                let len = match &dispatch.length {
                    Some(p) => Some(read_scalar_prim(p, bytes, pos, state)?),
                    None => None,
                };
                let tag = read_raw_prim_bytes(&dispatch.discriminator, bytes, pos)?;
                (tag, len)
            }
            DispatchOrder::TagFirst => {
                let tag = read_raw_prim_bytes(&dispatch.discriminator, bytes, pos)?;
                let len = match &dispatch.length {
                    Some(p) => Some(read_scalar_prim(p, bytes, pos, state)?),
                    None => None,
                };
                (tag, len)
            }
        };
        let is_sentinel = dispatch.until.as_deref() == Some(tag.as_slice());
        let body_start = *pos;
        match dispatch.arms.iter().find(|arm| arm.tag == tag) {
            Some(arm) => {
                walk_fields(&arm.fields, bytes, pos, state, 0)?;
                if let Some(nested) = &arm.nested {
                    walk_nested_dispatch(nested, bytes, pos, state)?;
                }
                if let Some(len) = length_value {
                    let expected_end = body_start + len as usize;
                    if expected_end > bytes.len() {
                        return Err(mismatch(*pos, "repeat arm declared length exceeds buffer"));
                    }
                    if *pos > expected_end {
                        return Err(mismatch(*pos, format!("repeat arm fields overran declared length ({} > {})", *pos, expected_end)));
                    }
                    *pos = expected_end;
                }
            }
            // P2-M2 item 1a: unrecognized discriminator — skip its declared length as opaque bytes.
            None => match length_value {
                Some(len) => {
                    need(bytes, *pos, len as usize, "repeat skip")?;
                    *pos += len as usize;
                }
                None => return Err(mismatch(*pos, format!("unrecognized discriminator {tag:?} with no declared `length` to skip"))),
            },
        }
        if let Some(trailer_prim) = &dispatch.trailer {
            walk_prim(trailer_prim, bytes, pos, state, 0)?;
        }
        if is_sentinel {
            break;
        }
        if *pos == iter_start {
            return Err(mismatch(*pos, "repeat block made no forward progress"));
        }
    }
    Ok(())
}

fn definitions_only(block: &Block) -> bool {
    matches!(block, Block::Struct { .. } | Block::Enum { .. })
}

/// 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly `bytes.len()`, else returns [`ProtocolMismatch`] with the failing offset. **Exception**
/// (P2-M2 item 5, documented precisely per the plan's own instruction): once ANY block has
/// explicitly JUMPED `pos` (`Block::BackwardScan`/`Block::JumpTo`), the walk is no longer a pure
/// linear forward accounting of every byte in the buffer — that is the whole point of a
/// backward-scan/offset-jump (ZIP's EOCD sits at a position determined only by scanning backward
/// from EOF, and its `cd_offset` field points at a position determined only by decoding the EOCD
/// first; the bytes physically between a jump target and EOF, or before it, are validly described
/// by OTHER blocks the walk already visited, not by "whatever's left after this one"). The final
/// `pos == bytes.len()` law is therefore skipped for any walk that performed at least one jump; it
/// holds EXACTLY as before for every protocol that declares neither block (the overwhelming
/// majority). The walker still only ever reads FORWARD from a jump's landing point — jumps move
/// `pos` directly, they never make the walker itself search or backtrack mid-block.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    if matches!(spec.framing,Framing::Literal){return walk_literal_protocol_controlled(spec,bytes,&mut crate::NativeDecodeControl::new(256*1024*1024,&mut |_|true));}
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked | Framing::Literal => {}
    }

    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
    let record_body_as_rest = matches!(spec.framing, Framing::Record);
    let mut consumed_record_body = false;
    // P2-M2 items 3+6: ONE state, shared across every block in this walk (see `WalkState` doc).
    let mut state = WalkState::default();
    // P2-M2 item 5: set once a BackwardScan/JumpTo block executes — relaxes the final EOF check.
    let mut jumped = false;

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, &mut state, reserved)?,
            Block::Segment { fields, cond, .. } => {
                let present = match cond {
                    Some(c) => eval_cond(c, &state.env, pos)?,
                    None => true,
                };
                if present {
                    walk_fields(fields, bytes, &mut pos, &mut state, reserved)?;
                }
            }
            Block::Record { name, fields, .. } => {
                if skip_named_records && !name.is_empty() {
                    continue;
                }
                if record_body_as_rest && !name.is_empty() {
                    if !consumed_record_body {
                        pos = bytes.len();
                        consumed_record_body = true;
                    }
                    continue;
                }
                walk_fields(fields, bytes, &mut pos, &mut state, reserved)?;
            }
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => walk_prim(prim, bytes, &mut pos, &mut state, 0)?,
            Block::Repeat { dispatch, .. } => walk_repeat(dispatch, bytes, &mut pos, &mut state)?,
            Block::BackwardScan { magic, fields, .. } => {
                let found = find_last_occurrence(bytes, magic).ok_or_else(|| mismatch(bytes.len(), "backward-scan magic not found in buffer"))?;
                // `fields` describe what comes AFTER the magic pattern itself, not the magic bytes.
                pos = found + magic.len();
                jumped = true;
                walk_fields(fields, bytes, &mut pos, &mut state, 0)?;
            }
            Block::JumpTo { offset_field, fields, .. } => {
                let target = *state.env.get(offset_field).ok_or_else(|| mismatch(pos, format!("jump target field `{offset_field}` was not decoded by an earlier block")))?;
                let target = target as usize;
                if target > bytes.len() {
                    return Err(mismatch(target, "jump target offset exceeds buffer length"));
                }
                pos = target;
                jumped = true;
                walk_fields(fields, bytes, &mut pos, &mut state, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if !jumped && pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// 📡️ Parses handcrafted `.protocol.semio` source then deep-walks bytes via [`walk_protocol`].
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    walk_protocol(&spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}

//#endregion 📡️ProtocolWalk
