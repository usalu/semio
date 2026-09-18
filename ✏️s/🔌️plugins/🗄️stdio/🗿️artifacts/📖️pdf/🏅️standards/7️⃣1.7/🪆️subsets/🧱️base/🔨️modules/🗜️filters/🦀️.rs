//! 🗜️ Stream filters (ISO 32000-1 §7.4): Flate (through the sibling `🗜️deflate` artifact), LZW,
//! ASCIIHex, ASCII85, RunLength with PNG/TIFF predictors, plus the image codecs (DCT, JPX,
//! CCITT, JBIG2) whose bitstreams are retained as they stand. `decode_stream` turns a stream
//! dictionary + raw bytes into logical data + the typed filter pipeline; `encode_stream` is its
//! inverse and yields the bytes and `/Filter`/`/DecodeParms` entries to write.

use super::lexer::{dict_get, is_ws, hex_val, malformed, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfCcittParameters, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter};

//#region 🔖️Ascii
/// 🔤️ `/ASCIIHexDecode`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ascii_hex_decode(s: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::new();
    for &b in s {
        if b == b'>' {
            break;
        }
        if is_ws(b) {
            continue;
        }
        if b.is_ascii_hexdigit() {
            nibbles.push(hex_val(b));
        }
    }
    if nibbles.len() % 2 == 1 {
        nibbles.push(0);
    }
    nibbles.chunks(2).map(|c| (c[0] << 4) | c[1]).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ascii_hex_encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 2 + 1);
    for byte in data {
        out.extend_from_slice(format!("{byte:02X}").as_bytes());
    }
    out.push(b'>');
    out
}

/// 🔡️ `/ASCII85Decode`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ascii85_decode(s: &[u8]) -> PResult<Vec<u8>> {
    let mut out = Vec::new();
    let mut group = [0u8; 5];
    let mut glen = 0usize;
    let s = if s.starts_with(b"<~") { &s[2..] } else { s };
    let mut i = 0usize;
    while i < s.len() {
        let b = s[i];
        i += 1;
        if is_ws(b) {
            continue;
        }
        if b == b'~' {
            break;
        }
        if b == b'z' && glen == 0 {
            out.extend_from_slice(&[0, 0, 0, 0]);
            continue;
        }
        if !(b'!'..=b'u').contains(&b) {
            return malformed("bad ascii85 byte");
        }
        group[glen] = b - b'!';
        glen += 1;
        if glen == 5 {
            let mut v: u32 = 0;
            for g in group {
                v = v.wrapping_mul(85).wrapping_add(g as u32);
            }
            out.extend_from_slice(&v.to_be_bytes());
            glen = 0;
        }
    }
    if glen > 1 {
        let n = glen;
        group[glen..5].fill(84);
        let mut v: u32 = 0;
        for g in group {
            v = v.wrapping_mul(85).wrapping_add(g as u32);
        }
        out.extend_from_slice(&v.to_be_bytes()[..n - 1]);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ascii85_encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for chunk in data.chunks(4) {
        let mut word = 0u32;
        for index in 0..4 {
            word = (word << 8) | chunk.get(index).copied().unwrap_or(0) as u32;
        }
        if chunk.len() == 4 && word == 0 {
            out.push(b'z');
            continue;
        }
        let mut encoded = [0u8; 5];
        for index in (0..5).rev() {
            encoded[index] = (word % 85) as u8 + b'!';
            word /= 85;
        }
        out.extend_from_slice(&encoded[..chunk.len() + 1]);
    }
    out.extend_from_slice(b"~>");
    out
}
//#endregion 🔖️Ascii

//#region 🔖️RunLength
/// 🏃️ `/RunLengthDecode`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn run_length_decode(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let len = s[i];
        i += 1;
        if len == 128 {
            break;
        }
        if len < 128 {
            let n = len as usize + 1;
            if i + n > s.len() {
                out.extend_from_slice(&s[i..]);
                break;
            }
            out.extend_from_slice(&s[i..i + n]);
            i += n;
        } else {
            if i >= s.len() {
                break;
            }
            let b = s[i];
            i += 1;
            out.extend(std::iter::repeat_n(b, 257 - len as usize));
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn run_length_encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < data.len() {
        let mut repeated = 1usize;
        while index + repeated < data.len() && data[index + repeated] == data[index] && repeated < 128 {
            repeated += 1;
        }
        if repeated >= 3 {
            out.push((257 - repeated) as u8);
            out.push(data[index]);
            index += repeated;
            continue;
        }
        let literal_start = index;
        index += repeated;
        while index < data.len() && index - literal_start < 128 {
            let mut next_repeat = 1usize;
            while index + next_repeat < data.len() && data[index + next_repeat] == data[index] && next_repeat < 3 {
                next_repeat += 1;
            }
            if next_repeat >= 3 {
                break;
            }
            index += next_repeat;
        }
        let length = (index - literal_start).min(128);
        out.push((length - 1) as u8);
        out.extend_from_slice(&data[literal_start..literal_start + length]);
        index = literal_start + length;
    }
    out.push(128);
    out
}
//#endregion 🔖️RunLength

//#region 🔖️Lzw
/// 🧬 `/LZWDecode` (§7.4.4.2): variable code width 9–12 bits, MSB-first, `early_change` per
/// `/EarlyChange` (default 1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lzw_decode(data: &[u8], early_change: bool) -> PResult<Vec<u8>> {
    let mut out = Vec::new();
    let mut table: Vec<Vec<u8>> = (0..256).map(|b| vec![b as u8]).collect();
    table.push(Vec::new());
    table.push(Vec::new());
    let mut code_width = 9usize;
    let mut previous: Option<Vec<u8>> = None;
    let mut bit_pos = 0usize;
    let total_bits = data.len() * 8;
    let early = if early_change { 1 } else { 0 };
    while bit_pos + code_width <= total_bits {
        let mut code = 0usize;
        for _ in 0..code_width {
            let byte = data[bit_pos / 8];
            let bit = (byte >> (7 - (bit_pos % 8))) & 1;
            code = (code << 1) | bit as usize;
            bit_pos += 1;
        }
        match code {
            256 => {
                table.truncate(258);
                code_width = 9;
                previous = None;
            }
            257 => break,
            _ => {
                let entry = if code < table.len() {
                    table[code].clone()
                } else if let Some(prev) = &previous {
                    let mut entry = prev.clone();
                    entry.push(prev[0]);
                    entry
                } else {
                    return malformed("LZW code before any entry");
                };
                out.extend_from_slice(&entry);
                if let Some(prev) = previous {
                    let mut new_entry = prev;
                    new_entry.push(entry[0]);
                    table.push(new_entry);
                }
                previous = Some(entry);
                if table.len() + early >= (1 << code_width) && code_width < 12 {
                    code_width += 1;
                }
            }
        }
    }
    Ok(out)
}

/// 🧬 LZW encode, the exact inverse of [`lzw_decode`] for the same `early_change`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lzw_encode(data: &[u8], early_change: bool) -> Vec<u8> {
    use std::collections::HashMap;
    let mut out = Vec::new();
    let mut bit_buffer: u64 = 0;
    let mut bit_count = 0usize;
    let early = if early_change { 1 } else { 0 };
    let mut emit = |code: usize, width: usize, out: &mut Vec<u8>, bit_buffer: &mut u64, bit_count: &mut usize| {
        *bit_buffer = (*bit_buffer << width) | code as u64;
        *bit_count += width;
        while *bit_count >= 8 {
            out.push((*bit_buffer >> (*bit_count - 8)) as u8);
            *bit_count -= 8;
        }
    };
    let mut table: HashMap<Vec<u8>, usize> = (0..256).map(|b| (vec![b as u8], b)).collect();
    let mut next_code = 258usize;
    let mut code_width = 9usize;
    emit(256, code_width, &mut out, &mut bit_buffer, &mut bit_count);
    let mut current: Vec<u8> = Vec::new();
    for &byte in data {
        let mut candidate = current.clone();
        candidate.push(byte);
        if table.contains_key(&candidate) {
            current = candidate;
            continue;
        }
        emit(table[&current], code_width, &mut out, &mut bit_buffer, &mut bit_count);
        table.insert(candidate, next_code);
        next_code += 1;
        if next_code + early > (1 << code_width) && code_width < 12 {
            code_width += 1;
        }
        if next_code >= 4096 - early {
            emit(256, code_width, &mut out, &mut bit_buffer, &mut bit_count);
            table = (0..256).map(|b| (vec![b as u8], b)).collect();
            next_code = 258;
            code_width = 9;
        }
        current = vec![byte];
    }
    if !current.is_empty() {
        emit(table[&current], code_width, &mut out, &mut bit_buffer, &mut bit_count);
        next_code += 1;
        if next_code + early > (1 << code_width) && code_width < 12 {
            code_width += 1;
        }
    }
    emit(257, code_width, &mut out, &mut bit_buffer, &mut bit_count);
    if bit_count > 0 {
        out.push((bit_buffer << (8 - bit_count)) as u8);
    }
    out
}
//#endregion 🔖️Lzw

//#region 🔖️Predictors
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

/// 🧮 PNG predictor decode (Predictor >= 10, §7.4.4.4 / PNG §6): each row is prefixed by a
/// filter-type byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn png_predictor_decode(raw: &[u8], columns: usize, colors: usize, bpc: usize) -> PResult<Vec<u8>> {
    let bpp = (colors * bpc).div_ceil(8).max(1);
    let row_bytes = (columns * colors * bpc).div_ceil(8);
    if row_bytes == 0 {
        return malformed("predictor: zero row width");
    }
    let mut out = Vec::with_capacity(raw.len());
    let mut prev = vec![0u8; row_bytes];
    let mut pos = 0;
    while pos + 1 <= raw.len() {
        let ft = raw[pos];
        pos += 1;
        let available = (raw.len() - pos).min(row_bytes);
        if available == 0 {
            break;
        }
        let mut filt = raw[pos..pos + available].to_vec();
        filt.resize(row_bytes, 0);
        pos += available;
        let mut cur = vec![0u8; row_bytes];
        for x in 0..row_bytes {
            let a = if x >= bpp { cur[x - bpp] } else { 0 };
            let b = prev[x];
            let c = if x >= bpp { prev[x - bpp] } else { 0 };
            cur[x] = match ft {
                0 => filt[x],
                1 => filt[x].wrapping_add(a),
                2 => filt[x].wrapping_add(b),
                3 => filt[x].wrapping_add(((a as u16 + b as u16) / 2) as u8),
                4 => filt[x].wrapping_add(paeth(a, b, c)),
                other => return malformed(format!("unsupported PNG predictor filter type {other}")),
            };
        }
        out.extend_from_slice(&cur[..available]);
        prev = cur;
    }
    Ok(out)
}

/// 🧮 TIFF predictor 2 decode (horizontal differencing) for 8-bit components.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_predictor2_decode(raw: &[u8], columns: usize, colors: usize) -> Vec<u8> {
    let mut out = raw.to_vec();
    let row_bytes = columns * colors;
    if row_bytes == 0 {
        return out;
    }
    for row in out.chunks_mut(row_bytes) {
        for x in colors..row.len() {
            row[x] = row[x].wrapping_add(row[x - colors]);
        }
    }
    out
}

/// 🧮 Applies a predictor before compression: PNG predictors write every row with the `Up`
/// filter (type 2); TIFF predictor 2 differences horizontally.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn predictor_encode(data: &[u8], predictor: &PdfPredictor) -> Vec<u8> {
    let row_bytes = (predictor.columns as usize * predictor.colors as usize * predictor.bits_per_component as usize).div_ceil(8).max(1);
    if predictor.predictor >= 10 {
        let mut out = Vec::with_capacity(data.len() + data.len().div_ceil(row_bytes));
        let mut prev = vec![0u8; row_bytes];
        for row in data.chunks(row_bytes) {
            out.push(2);
            for (index, byte) in row.iter().enumerate() {
                out.push(byte.wrapping_sub(prev[index]));
            }
            prev = row.to_vec();
            prev.resize(row_bytes, 0);
        }
        return out;
    }
    if predictor.predictor == 2 && predictor.bits_per_component == 8 {
        let colors = predictor.colors.max(1) as usize;
        let mut out = data.to_vec();
        for row in out.chunks_mut(row_bytes) {
            for index in (colors..row.len()).rev() {
                row[index] = row[index].wrapping_sub(row[index - colors]);
            }
        }
        return out;
    }
    data.to_vec()
}
//#endregion 🔖️Predictors

//#region 🔖️Flate
/// 🗜️ zlib inflate, lenient about a damaged Adler-32 or a truncated tail the way shipping readers
/// are: the raw DEFLATE body is decoded on its own when the strict path refuses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn flate_decode(data: &[u8]) -> PResult<Vec<u8>> {
    use semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::{inflate_raw, zlib_decompress};
    let start = data.iter().position(|byte| !is_ws(*byte)).unwrap_or(0);
    let data = &data[start..];
    if data.is_empty() {
        return Ok(Vec::new());
    }
    match zlib_decompress(data) {
        Ok(out) => Ok(out),
        Err(strict) => {
            let body = if data.len() > 2 && (data[0] & 0x0F) == 8 && ((data[0] as u16) * 256 + data[1] as u16).is_multiple_of(31) { &data[2..] } else { data };
            inflate_raw(body).or_else(|_| inflate_raw(&body[..body.len().saturating_sub(4)])).map_err(|_| PdfEngineError::Malformed(format!("FlateDecode: {strict}")))
        }
    }
}

/// 🗜️ Deterministic zlib deflate (the sibling artifact's level-nine encoder).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn flate_encode(data: &[u8]) -> Vec<u8> {
    semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress_deterministic(data).expect("deflate never fails on in-memory input")
}
//#endregion 🔖️Flate

//#region 🔖️Pipeline
/// 🎛️ Reads one `/DecodeParms` dictionary's predictor fields with spec defaults.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn predictor_of(parms: Option<&PdfObject>) -> Option<PdfPredictor> {
    let get = |key: &str, default: i64| -> i64 { parms.and_then(|p| p.dict_get(key)).and_then(|v| v.as_i64()).unwrap_or(default) };
    let predictor = get("Predictor", 1);
    (predictor != 1).then(|| PdfPredictor { predictor: predictor as u32, colors: get("Colors", 1).max(1) as u32, bits_per_component: get("BitsPerComponent", 8).max(1) as u32, columns: get("Columns", 1).max(1) as u32 })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_predictor(data: Vec<u8>, predictor: &Option<PdfPredictor>) -> PResult<Vec<u8>> {
    match predictor {
        Some(p) if p.predictor >= 10 => png_predictor_decode(&data, p.columns as usize, p.colors as usize, p.bits_per_component as usize),
        Some(p) if p.predictor == 2 => Ok(tiff_predictor2_decode(&data, p.columns as usize, p.colors as usize)),
        _ => Ok(data),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ccitt_parameters(parms: Option<&PdfObject>) -> PdfCcittParameters {
    let int = |key: &str, default: i64| parms.and_then(|p| p.dict_get(key)).and_then(|v| v.as_i64()).unwrap_or(default);
    let flag = |key: &str, default: bool| parms.and_then(|p| p.dict_get(key)).and_then(|v| v.as_bool()).unwrap_or(default);
    PdfCcittParameters { k: int("K", 0) as i32, columns: int("Columns", 1728).max(1) as u32, rows: int("Rows", 0).max(0) as u32, black_is_1: flag("BlackIs1", false), encoded_byte_align: flag("EncodedByteAlign", false), end_of_line: flag("EndOfLine", false), end_of_block: flag("EndOfBlock", true), damaged_rows_before_error: int("DamagedRowsBeforeError", 0).max(0) as u32 }
}

/// 🧾 The `/Filter` names and per-filter `/DecodeParms` of a stream dictionary (abbreviations
/// included, §7.4 Table 6 and §8.9.7 Table 94).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declared_filters(dict: &[PdfDictEntry]) -> Vec<(String, Option<PdfObject>)> {
    let names: Vec<String> = match dict_get(dict, "Filter").or_else(|| dict_get(dict, "F")).filter(|value| matches!(value, PdfObject::Name(_) | PdfObject::Array(_))) {
        Some(PdfObject::Name(n)) => vec![n.clone()],
        Some(PdfObject::Array(a)) => a.iter().filter_map(|o| o.as_name().map(|s| s.to_string())).collect(),
        _ => Vec::new(),
    };
    let parms: Vec<Option<PdfObject>> = match dict_get(dict, "DecodeParms").or_else(|| dict_get(dict, "DP")) {
        Some(PdfObject::Array(a)) => a.iter().map(|o| matches!(o, PdfObject::Dict(_)).then(|| o.clone())).collect(),
        Some(d @ PdfObject::Dict(_)) => vec![Some(d.clone())],
        _ => Vec::new(),
    };
    names.into_iter().enumerate().map(|(index, name)| (name, parms.get(index).cloned().flatten())).collect()
}

/// 🗜️ Decodes a stream's bytes per its `/Filter` chain into logical data + the typed pipeline.
/// Image codecs stop the chain: their payload is retained encoded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_stream(dict: &[PdfDictEntry], raw: &[u8]) -> PResult<(Vec<u8>, Vec<PdfStreamFilter>)> {
    let mut data = raw.to_vec();
    let mut pipeline = Vec::new();
    for (name, parms) in declared_filters(dict) {
        let filter = match name.as_str() {
            "FlateDecode" | "Fl" => {
                let predictor = predictor_of(parms.as_ref());
                data = apply_predictor(flate_decode(&data)?, &predictor)?;
                PdfStreamFilter::Flate { predictor }
            }
            "LZWDecode" | "LZW" => {
                let predictor = predictor_of(parms.as_ref());
                let early_change = parms.as_ref().and_then(|p| p.dict_get("EarlyChange")).and_then(|v| v.as_i64()).unwrap_or(1) != 0;
                data = apply_predictor(lzw_decode(&data, early_change)?, &predictor)?;
                PdfStreamFilter::Lzw { predictor, early_change }
            }
            "ASCIIHexDecode" | "AHx" => {
                data = ascii_hex_decode(&data);
                PdfStreamFilter::AsciiHex
            }
            "ASCII85Decode" | "A85" => {
                data = ascii85_decode(&data)?;
                PdfStreamFilter::Ascii85
            }
            "RunLengthDecode" | "RL" => {
                data = run_length_decode(&data);
                PdfStreamFilter::RunLength
            }
            "DCTDecode" | "DCT" => PdfStreamFilter::Dct { color_transform: parms.as_ref().and_then(|p| p.dict_get("ColorTransform")).and_then(|v| v.as_i64()).map(|v| v as u32) },
            "JPXDecode" => PdfStreamFilter::Jpx,
            "CCITTFaxDecode" | "CCF" => PdfStreamFilter::Ccitt { parameters: ccitt_parameters(parms.as_ref()) },
            "JBIG2Decode" => PdfStreamFilter::Jbig2 { globals: parms.as_ref().and_then(|p| p.dict_get("JBIG2Globals")).and_then(|v| match v {
                PdfObject::Stream { data, .. } => Some(data.clone()),
                _ => None,
            }) },
            "Crypt" => PdfStreamFilter::Crypt { name: parms.as_ref().and_then(|p| p.dict_get("Name")).and_then(|v| v.as_name()).map(str::to_string) },
            other => return Err(PdfEngineError::Unsupported(format!("stream filter /{other} is not a PDF filter"))),
        };
        let stops = filter.is_image_codec();
        pipeline.push(filter);
        if stops {
            break;
        }
    }
    Ok((data, pipeline))
}

/// 📤️ Applies a typed pipeline to logical data, innermost filter first, and returns the bytes to
/// write plus the `/Filter` and `/DecodeParms` entries describing them. Image codecs contribute
/// their parameters only — their payload is already encoded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_stream(data: &[u8], filters: &[PdfStreamFilter]) -> (Vec<u8>, Vec<PdfDictEntry>) {
    let mut encoded = data.to_vec();
    for filter in filters.iter().rev() {
        encoded = match filter {
            PdfStreamFilter::Flate { predictor } => flate_encode(&predictor.as_ref().map_or_else(|| encoded.clone(), |value| predictor_encode(&encoded, value))),
            PdfStreamFilter::Lzw { predictor, early_change } => lzw_encode(&predictor.as_ref().map_or_else(|| encoded.clone(), |value| predictor_encode(&encoded, value)), *early_change),
            PdfStreamFilter::AsciiHex => ascii_hex_encode(&encoded),
            PdfStreamFilter::Ascii85 => ascii85_encode(&encoded),
            PdfStreamFilter::RunLength => run_length_encode(&encoded),
            PdfStreamFilter::Dct { .. } | PdfStreamFilter::Jpx | PdfStreamFilter::Ccitt { .. } | PdfStreamFilter::Jbig2 { .. } | PdfStreamFilter::Crypt { .. } => encoded,
        };
    }
    let mut entries = Vec::new();
    if filters.is_empty() {
        return (encoded, entries);
    }
    let names: Vec<PdfObject> = filters.iter().map(|filter| PdfObject::name(filter.name())).collect();
    entries.push(PdfDictEntry::new("Filter", if names.len() == 1 { names[0].clone() } else { PdfObject::Array(names) }));
    let predictor_dict = |value: &PdfPredictor| -> Vec<PdfDictEntry> {
        vec![
            PdfDictEntry::new("Predictor", PdfObject::Int(value.predictor as i64)),
            PdfDictEntry::new("Colors", PdfObject::Int(value.colors as i64)),
            PdfDictEntry::new("BitsPerComponent", PdfObject::Int(value.bits_per_component as i64)),
            PdfDictEntry::new("Columns", PdfObject::Int(value.columns as i64)),
        ]
    };
    let parameters: Vec<PdfObject> = filters
        .iter()
        .map(|filter| match filter {
            PdfStreamFilter::Flate { predictor: Some(value) } => PdfObject::Dict(predictor_dict(value)),
            PdfStreamFilter::Lzw { predictor, early_change } => {
                let mut dict = predictor.as_ref().map(predictor_dict).unwrap_or_default();
                if !early_change {
                    dict.push(PdfDictEntry::new("EarlyChange", PdfObject::Int(0)));
                }
                if dict.is_empty() {
                    PdfObject::Null
                } else {
                    PdfObject::Dict(dict)
                }
            }
            PdfStreamFilter::Dct { color_transform: Some(value) } => PdfObject::Dict(vec![PdfDictEntry::new("ColorTransform", PdfObject::Int(*value as i64))]),
            PdfStreamFilter::Ccitt { parameters } => {
                let mut dict = Vec::new();
                if parameters.k != 0 {
                    dict.push(PdfDictEntry::new("K", PdfObject::Int(parameters.k as i64)));
                }
                if parameters.columns != 1728 {
                    dict.push(PdfDictEntry::new("Columns", PdfObject::Int(parameters.columns as i64)));
                }
                if parameters.rows != 0 {
                    dict.push(PdfDictEntry::new("Rows", PdfObject::Int(parameters.rows as i64)));
                }
                if parameters.black_is_1 {
                    dict.push(PdfDictEntry::new("BlackIs1", PdfObject::Bool(true)));
                }
                if parameters.encoded_byte_align {
                    dict.push(PdfDictEntry::new("EncodedByteAlign", PdfObject::Bool(true)));
                }
                if parameters.end_of_line {
                    dict.push(PdfDictEntry::new("EndOfLine", PdfObject::Bool(true)));
                }
                if !parameters.end_of_block {
                    dict.push(PdfDictEntry::new("EndOfBlock", PdfObject::Bool(false)));
                }
                if parameters.damaged_rows_before_error != 0 {
                    dict.push(PdfDictEntry::new("DamagedRowsBeforeError", PdfObject::Int(parameters.damaged_rows_before_error as i64)));
                }
                if dict.is_empty() {
                    PdfObject::Null
                } else {
                    PdfObject::Dict(dict)
                }
            }
            PdfStreamFilter::Jbig2 { globals: Some(globals) } => PdfObject::Dict(vec![PdfDictEntry::new("JBIG2Globals", PdfObject::Stream { dict: Vec::new(), data: globals.clone(), filters: Vec::new() })]),
            PdfStreamFilter::Crypt { name: Some(name) } => PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("CryptFilterDecodeParms")), PdfDictEntry::new("Name", PdfObject::name(name))]),
            _ => PdfObject::Null,
        })
        .collect();
    if parameters.iter().any(|value| !matches!(value, PdfObject::Null)) {
        entries.push(PdfDictEntry::new("DecodeParms", if parameters.len() == 1 { parameters[0].clone() } else { PdfObject::Array(parameters) }));
    }
    (encoded, entries)
}
//#endregion 🔖️Pipeline

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
