//! 🔗️ File structure (ISO 32000-1 §7.5): classic cross-reference tables, cross-reference streams,
//! hybrid files, `/Prev` chains, object streams, and the brute-force `N G obj` scan every shipping
//! reader falls back to. The [`Resolver`] materializes indirect objects on demand (decrypting
//! them when the document carries a standard security handler) and can flatten the whole graph
//! into the snapshot's retained `objects` lane.

use super::encryption::Decryptor;
use super::filters::decode_stream;
use super::lexer::{brute_force_scan, dict_i64, find_last_subslice, malformed, parse_indirect_at, Lexer, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};
use std::collections::{HashMap, HashSet};

//#region 🔖️Entries
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefEntry {
    Normal { offset: usize, gen: u16 },
    Compressed { stream_num: u32, index: u32 },
}

/// 🧾 The merged cross-reference state of a file: every live entry plus the first (newest)
/// trailer dictionary.
pub struct XrefState {
    pub entries: HashMap<u32, XrefEntry>,
    pub trailer: Vec<PdfDictEntry>,
    pub brute_forced: bool,
}
//#endregion 🔖️Entries

//#region 🔖️Parsing
/// 📐️ Decodes one row of an xref stream given `/W = [w0,w1,w2]` (field widths in bytes; `w0==0`
/// defaults field 1/type to `1` per §7.5.8.2).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_xref_row(row: &[u8], w: [usize; 3]) -> (u8, u64, u64) {
    let mut pos = 0usize;
    let mut read = |width: usize, default: u64| -> u64 {
        if width == 0 {
            return default;
        }
        let mut v: u64 = 0;
        for _ in 0..width {
            v = (v << 8) | *row.get(pos).unwrap_or(&0) as u64;
            pos += 1;
        }
        v
    };
    let f0 = read(w[0], 1);
    let f1 = read(w[1], 0);
    let f2 = read(w[2], 0);
    (f0 as u8, f1, f2)
}

/// 🌊 Parses a classic `xref` table + its `trailer` dict starting at `offset`. Handles multiple
/// subsections; lenient about the fixed-width-20-byte convention (splits on whitespace instead).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_classic_xref(data: &[u8], offset: usize) -> PResult<(HashMap<u32, XrefEntry>, Vec<PdfDictEntry>)> {
    let mut lex = Lexer::new(data).at(offset);
    lex.skip_ws();
    if !lex.consume_keyword(b"xref") {
        return malformed("expected 'xref' keyword");
    }
    let mut entries = HashMap::new();
    loop {
        lex.skip_ws();
        if lex.starts_with(b"trailer") {
            break;
        }
        if !matches!(lex.peek(), Some(b'0'..=b'9')) {
            break;
        }
        let start = match lex.parse_number()? {
            PdfObject::Int(i) => i as u32,
            _ => return malformed("bad xref subsection start"),
        };
        lex.skip_ws();
        let count = match lex.parse_number()? {
            PdfObject::Int(i) => i as u32,
            _ => return malformed("bad xref subsection count"),
        };
        for i in 0..count {
            lex.skip_ws();
            if lex.starts_with(b"trailer") {
                break;
            }
            let off_tok = lex.read_regular_run();
            lex.skip_ws();
            let gen_tok = lex.read_regular_run();
            lex.skip_ws();
            let flag_tok = lex.read_regular_run();
            let off: usize = std::str::from_utf8(off_tok).ok().and_then(|s| s.parse().ok()).unwrap_or(0);
            let gen: u16 = std::str::from_utf8(gen_tok).ok().and_then(|s| s.parse().ok()).unwrap_or(0);
            let in_use = flag_tok.first() == Some(&b'n');
            if in_use {
                entries.entry(start + i).or_insert(XrefEntry::Normal { offset: off, gen });
            }
        }
    }
    lex.skip_ws();
    if !lex.consume_keyword(b"trailer") {
        return malformed("expected 'trailer' keyword");
    }
    lex.skip_ws();
    let trailer = match lex.parse_object()? {
        PdfObject::Dict(d) => d,
        _ => return malformed("trailer is not a dict"),
    };
    Ok((entries, trailer))
}

/// 🌊 Parses an xref STREAM (`/Type /XRef`) at `offset` (§7.5.8).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_xref_stream(data: &[u8], offset: usize) -> PResult<(HashMap<u32, XrefEntry>, Vec<PdfDictEntry>)> {
    let (_id, obj) = parse_indirect_at(data, offset)?;
    let (dict, raw) = match &obj {
        PdfObject::Stream { dict, data, .. } => (dict.clone(), data.clone()),
        _ => return malformed("xref stream object is not a stream"),
    };
    let (decoded, _) = decode_stream(&dict, &raw)?;
    let w = match dict.iter().find(|e| e.key == "W").map(|e| &e.value) {
        Some(PdfObject::Array(a)) if a.len() >= 3 => [a[0].as_i64().unwrap_or(0).max(0) as usize, a[1].as_i64().unwrap_or(0).max(0) as usize, a[2].as_i64().unwrap_or(0).max(0) as usize],
        _ => return malformed("xref stream missing /W"),
    };
    let size = dict_i64(&dict, "Size").unwrap_or(0);
    let index: Vec<i64> = match dict.iter().find(|e| e.key == "Index").map(|e| &e.value) {
        Some(PdfObject::Array(a)) => a.iter().filter_map(|o| o.as_i64()).collect(),
        _ => vec![0, size],
    };
    let row_bytes = w[0] + w[1] + w[2];
    if row_bytes == 0 {
        return malformed("xref stream /W is all zero");
    }
    let mut entries = HashMap::new();
    let mut pos = 0usize;
    for chunk in index.chunks(2) {
        if chunk.len() < 2 {
            break;
        }
        let (start, count) = (chunk[0].max(0) as u32, chunk[1].max(0) as u32);
        for i in 0..count {
            if pos + row_bytes > decoded.len() {
                break;
            }
            let (ty, f1, f2) = decode_xref_row(&decoded[pos..pos + row_bytes], w);
            pos += row_bytes;
            let num = start + i;
            match ty {
                1 => {
                    entries.entry(num).or_insert(XrefEntry::Normal { offset: f1 as usize, gen: f2 as u16 });
                }
                2 => {
                    entries.entry(num).or_insert(XrefEntry::Compressed { stream_num: f1 as u32, index: f2 as u32 });
                }
                _ => {}
            }
        }
    }
    Ok((entries, dict))
}

/// 🧵 Follows `/Prev` (and hybrid `/XRefStm`) chains from `start_offset`, merging older sections
/// without overwriting newer entries. Falls back to a brute-force `N G obj` scan if the structured
/// chain can't even be started, and to the scan for any offset that does not point at its object.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn build_xref(data: &[u8], start_offset: Option<usize>) -> XrefState {
    let mut entries: HashMap<u32, XrefEntry> = HashMap::new();
    let mut trailer: Vec<PdfDictEntry> = Vec::new();
    let mut visited = HashSet::new();
    let mut cursor = start_offset;
    let mut any_structured = false;
    while let Some(off) = cursor {
        if !visited.insert(off) || off >= data.len() {
            break;
        }
        let parsed = {
            let mut l = Lexer::new(data).at(off);
            l.skip_ws();
            if l.starts_with(b"xref") {
                parse_classic_xref(data, off)
            } else {
                parse_xref_stream(data, off)
            }
        };
        let Ok((sect_entries, sect_trailer)) = parsed else { break };
        any_structured = true;
        if let Some(stm_off) = dict_i64(&sect_trailer, "XRefStm") {
            if let Ok((stm_entries, _)) = parse_xref_stream(data, stm_off as usize) {
                for (k, v) in stm_entries {
                    entries.entry(k).or_insert(v);
                }
            }
        }
        for (k, v) in sect_entries {
            entries.entry(k).or_insert(v);
        }
        if trailer.is_empty() {
            trailer = sect_trailer.clone();
        } else {
            for entry in sect_trailer.iter() {
                if !trailer.iter().any(|existing| existing.key == entry.key) && matches!(entry.key.as_str(), "Root" | "Info" | "ID" | "Encrypt") {
                    trailer.push(entry.clone());
                }
            }
        }
        cursor = dict_i64(&sect_trailer, "Prev").map(|p| p as usize);
    }
    let offsets_agree = entries.iter().all(|(num, entry)| match entry {
        XrefEntry::Normal { offset, .. } => parse_indirect_at(data, *offset).is_ok_and(|(id, _)| id.num == *num),
        XrefEntry::Compressed { .. } => true,
    });
    let root_resolvable = trailer.iter().find(|e| e.key == "Root").and_then(|e| e.value.as_ref()).is_some_and(|root| entries.contains_key(&root.num));
    let mut brute_forced = false;
    if !any_structured || entries.is_empty() || !offsets_agree || !root_resolvable {
        brute_forced = true;
        let scanned = brute_force_scan(data);
        for (num, (id, off)) in &scanned {
            entries.insert(*num, XrefEntry::Normal { offset: *off, gen: id.gen });
        }
        // 🧾 Objects living inside object streams are only reachable through those streams.
        let stream_nums: Vec<u32> = scanned.iter().filter(|(_, (_, off))| parse_indirect_at(data, *off).is_ok_and(|(_, object)| object.dict_get("Type").and_then(PdfObject::as_name) == Some("ObjStm"))).map(|(num, _)| *num).collect();
        for stream_num in stream_nums {
            if let Ok((_, PdfObject::Stream { dict, data: raw, .. })) = parse_indirect_at(data, scanned[&stream_num].1) {
                if let Ok((decoded, _)) = decode_stream(&dict, &raw) {
                    let n = dict_i64(&dict, "N").unwrap_or(0).max(0) as usize;
                    let mut lex = Lexer::new(&decoded);
                    for index in 0..n {
                        lex.skip_ws();
                        let Ok(PdfObject::Int(num)) = lex.parse_number() else { break };
                        lex.skip_ws();
                        let Ok(PdfObject::Int(_)) = lex.parse_number() else { break };
                        entries.entry(num as u32).or_insert(XrefEntry::Compressed { stream_num, index: index as u32 });
                    }
                }
            }
        }
        if !root_resolvable || trailer.is_empty() {
            let mut found_trailer: Vec<PdfDictEntry> = trailer.iter().filter(|e| e.key != "Root").cloned().collect();
            for (_, (id, off)) in &scanned {
                if let Ok((_, obj)) = parse_indirect_at(data, *off) {
                    if obj.dict_get("Type").and_then(|v| v.as_name()) == Some("Catalog") {
                        found_trailer.insert(0, PdfDictEntry::new("Root", PdfObject::Ref(*id)));
                        break;
                    }
                    if obj.dict_get("Type").and_then(|v| v.as_name()) == Some("XRef") && !found_trailer.iter().any(|e| e.key == "Root") {
                        if let Some(root) = obj.dict_get("Root") {
                            found_trailer.insert(0, PdfDictEntry::new("Root", root.clone()));
                        }
                    }
                }
            }
            if !found_trailer.iter().any(|e| e.key == "Root") {
                if let Some(trailer_pos) = find_last_subslice(data, b"trailer") {
                    let mut lex = Lexer::new(data).at(trailer_pos + 7);
                    if let Ok(PdfObject::Dict(d)) = lex.parse_object() {
                        for entry in d {
                            if !found_trailer.iter().any(|e| e.key == entry.key) {
                                found_trailer.push(entry);
                            }
                        }
                    }
                }
            }
            trailer = found_trailer;
        }
    }
    XrefState { entries, trailer, brute_forced }
}

/// 🔍 Locates `startxref` and returns the offset it points at, if any.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn startxref_offset(data: &[u8]) -> Option<usize> {
    let pos = find_last_subslice(data, b"startxref")?;
    let mut lex = Lexer::new(data).at(pos + b"startxref".len());
    lex.skip_ws();
    match lex.parse_number() {
        Ok(PdfObject::Int(off)) if off >= 0 && (off as usize) < data.len() => Some(off as usize),
        _ => None,
    }
}
//#endregion 🔖️Parsing

//#region 🔖️Resolver
/// 🧭 Resolves indirect objects into a table, decoding object streams (`/Type /ObjStm`) and
/// decrypting strings/streams transparently.
pub struct Resolver<'a> {
    data: &'a [u8],
    pub xref: HashMap<u32, XrefEntry>,
    cache: HashMap<u32, PdfObject>,
    objstm_cache: HashMap<u32, Vec<(u32, usize)>>,
    objstm_bytes: HashMap<u32, Vec<u8>>,
    decryptor: Option<Decryptor>,
    resolving: HashSet<u32>,
}

impl<'a> Resolver<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(data: &'a [u8], xref: HashMap<u32, XrefEntry>) -> Self {
        Self { data, xref, cache: HashMap::new(), objstm_cache: HashMap::new(), objstm_bytes: HashMap::new(), decryptor: None, resolving: HashSet::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_decryptor(&mut self, decryptor: Option<Decryptor>) {
        self.decryptor = decryptor;
        self.cache.clear();
    }

    /// 🎯 The object `num` refers to, undecoded (stream data still filtered) but decrypted.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve(&mut self, num: u32) -> Option<PdfObject> {
        if let Some(v) = self.cache.get(&num) {
            return Some(v.clone());
        }
        if !self.resolving.insert(num) {
            return None;
        }
        let entry = self.xref.get(&num).copied();
        let value = match entry {
            Some(XrefEntry::Normal { offset, gen }) => match parse_indirect_at(self.data, offset) {
                Ok((id, value)) if id.num == num => {
                    let gen = if id.gen != gen { id.gen } else { gen };
                    match (&self.decryptor, &value) {
                        (Some(decryptor), _) if !is_xref_stream(&value) => Some(decryptor.decrypt_object(value, num, gen)),
                        _ => Some(value),
                    }
                }
                _ => None,
            },
            Some(XrefEntry::Compressed { stream_num, index }) => self.resolve_compressed(stream_num, index),
            None => None,
        };
        self.resolving.remove(&num);
        let value = value?;
        self.cache.insert(num, value.clone());
        Some(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve_compressed(&mut self, stream_num: u32, index: u32) -> Option<PdfObject> {
        if !self.objstm_bytes.contains_key(&stream_num) {
            let obj = self.resolve(stream_num)?;
            let PdfObject::Stream { dict, data, .. } = &obj else { return None };
            let (decoded, _) = decode_stream(dict, data).ok()?;
            let n = dict_i64(dict, "N").unwrap_or(0).max(0) as usize;
            let first = dict_i64(dict, "First").unwrap_or(0).max(0) as usize;
            let mut lex = Lexer::new(&decoded);
            let mut pairs = Vec::with_capacity(n);
            for _ in 0..n {
                lex.skip_ws();
                let on = match lex.parse_number() {
                    Ok(PdfObject::Int(i)) => i as u32,
                    _ => break,
                };
                lex.skip_ws();
                let oo = match lex.parse_number() {
                    Ok(PdfObject::Int(i)) => i as usize,
                    _ => break,
                };
                pairs.push((on, first + oo));
            }
            self.objstm_cache.insert(stream_num, pairs);
            self.objstm_bytes.insert(stream_num, decoded);
        }
        let pairs = self.objstm_cache.get(&stream_num)?;
        let (_, local_off) = *pairs.get(index as usize)?;
        let bytes = self.objstm_bytes.get(&stream_num)?;
        if local_off >= bytes.len() {
            return None;
        }
        let mut lex = Lexer::new(bytes).at(local_off);
        lex.parse_object().ok()
    }

    /// 📚️ Materializes every entry of the xref table into `PdfIndirectObject`s with decoded
    /// stream data (the retained-graph lane), in file order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve_all(&mut self) -> PResult<Vec<PdfIndirectObject>> {
        let mut nums: Vec<u32> = self.xref.keys().copied().collect();
        nums.sort_by_key(|num| match self.xref.get(num) {
            Some(XrefEntry::Normal { offset, .. }) => (*offset, 0),
            Some(XrefEntry::Compressed { stream_num, index }) => {
                let offset = match self.xref.get(stream_num) {
                    Some(XrefEntry::Normal { offset, .. }) => *offset,
                    _ => usize::MAX,
                };
                (offset, index.saturating_add(1) as usize)
            }
            None => (usize::MAX, usize::MAX),
        });
        let mut out = Vec::with_capacity(nums.len());
        for num in nums {
            if let Some(value) = self.resolve(num) {
                if is_xref_stream(&value) || value.dict_get("Type").and_then(PdfObject::as_name) == Some("ObjStm") {
                    continue;
                }
                let gen = match self.xref.get(&num) {
                    Some(XrefEntry::Normal { gen, .. }) => *gen,
                    _ => 0,
                };
                out.push(PdfIndirectObject { id: ObjRef { num, gen }, value: normalize_pdf_object(value)? });
            }
        }
        Ok(out)
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_xref_stream(value: &PdfObject) -> bool {
    matches!(value, PdfObject::Stream { .. }) && value.dict_get("Type").and_then(PdfObject::as_name) == Some("XRef")
}

/// 🧹 Converts parsed COS into snapshot form: stream filters with logical decoders are applied,
/// their declarations dropped from the dictionary and kept as the typed pipeline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normalize_pdf_object(value: PdfObject) -> PResult<PdfObject> {
    match value {
        PdfObject::Array(items) => Ok(PdfObject::Array(items.into_iter().map(normalize_pdf_object).collect::<PResult<_>>()?)),
        PdfObject::Dict(entries) => Ok(PdfObject::Dict(entries.into_iter().map(|entry| Ok(PdfDictEntry { key: entry.key, value: normalize_pdf_object(entry.value)? })).collect::<PResult<_>>()?)),
        PdfObject::Stream { dict, data, .. } => {
            let (decoded, filters) = match decode_stream(&dict, &data) {
                Ok(decoded) => decoded,
                Err(PdfEngineError::Unsupported(_)) | Err(PdfEngineError::Malformed(_)) => (data, Vec::new()),
                Err(error) => return Err(error),
            };
            let dict = dict.into_iter().filter(|entry| !matches!(entry.key.as_str(), "Filter" | "F" | "DecodeParms" | "DP" | "Length") || (entry.key == "F" && !matches!(entry.value, PdfObject::Name(_) | PdfObject::Array(_)))).map(|entry| Ok(PdfDictEntry { key: entry.key, value: normalize_pdf_object(entry.value)? })).collect::<PResult<_>>()?;
            Ok(PdfObject::Stream { dict, data: decoded, filters })
        }
        value => Ok(value),
    }
}
//#endregion 🔖️Resolver

//#region 🔖️Sources
/// 🧭 Anything that can follow an indirect reference: a [`Resolver`] over file bytes, or the
/// snapshot's retained `objects` lane.
pub trait ObjectSource {
    /// 🎯 The object a reference points at (`None` when unresolvable).
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject>;
    /// 🎯 Follows `value` if it is a reference, else returns it as is.
    fn deref(&mut self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Ref(reference) => self.get(*reference).unwrap_or(PdfObject::Null),
            other => other.clone(),
        }
    }
    /// 🎯 Follows a reference chain to a dictionary/stream entry.
    fn deref_key(&mut self, value: &PdfObject, key: &str) -> Option<PdfObject> {
        let owner = self.deref(value);
        let entry = owner.dict_get(key)?.clone();
        Some(self.deref(&entry))
    }
}

impl ObjectSource for Resolver<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.resolve(reference.num).map(|value| normalize_pdf_object(value).unwrap_or(PdfObject::Null))
    }
}

/// 🧭 A source over the retained `objects` lane (already normalized).
pub struct GraphSource<'a> {
    by_number: HashMap<u32, &'a PdfObject>,
}

impl<'a> GraphSource<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(objects: &'a [PdfIndirectObject]) -> Self {
        Self { by_number: objects.iter().map(|object| (object.id.num, &object.value)).collect() }
    }
}

impl ObjectSource for GraphSource<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.by_number.get(&reference.num).map(|value| (*value).clone())
    }
}

/// 🧱 Anything that can allocate indirect objects while a typed lane is lowered.
pub trait ObjectSink {
    /// ➕ Stores `value` as a new indirect object and returns its reference.
    fn add(&mut self, value: PdfObject) -> ObjRef;
}
//#endregion 🔖️Sources

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
