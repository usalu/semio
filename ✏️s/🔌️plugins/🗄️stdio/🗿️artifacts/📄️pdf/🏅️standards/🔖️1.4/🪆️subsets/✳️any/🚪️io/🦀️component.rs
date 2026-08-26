//! 🚪️ IO stdio.pdf (1.4/✳️any) — a real PDF 1.4 reader and writer over the document's page tree.
//!
//! 📜️ **Scope, stated as this standard's feature set rather than as a limitation.** PDF 1.4 (Adobe
//! PDF Reference 1.4 — the version ISO 19005-1/PDF-A-1 and ISO 15930-1/PDF-X-1a are written
//! against) stores its cross-reference information in a classic `xref` TABLE terminated by a
//! `trailer` dictionary. Cross-reference streams and object streams are PDF **1.5** constructs, so
//! this reader has neither and needs neither; what it does have is the table, its `/Prev` chain,
//! and the brute-force `N G obj` rescan every real reader falls back to on a damaged file. The
//! writer emits `%PDF-1.4` with a classic table for the same reason.
//!
//! 🔀️ **Shared syntax, own model.** The COS object grammar (ISO 32000-1 §7.2–7.3) is version-
//! independent: the same lexer reads a 1.0 file and a 1.7 file. It is therefore REUSED from the
//! 1.7 subtree ([`cos::Lexer`], [`cos::decode_stream`]) instead of being re-typed here — exactly
//! the reuse `ifc` makes of `step`'s Part-21 tokenizer and `gif` 89a makes of 87a's LZW/sub-block
//! codec. `cos::PdfObject` is that lexer's WORKING representation and never enters this standard's
//! persisted snapshot, which is its own `PdfSnapshot { schema, pages: Vec<PageDoc> }`.
//!
//! 📝️ **What `PageDoc::text` is.** The operand bytes of the text-showing operators (`Tj`, `TJ`,
//! `'`, `"` — ISO 32000-1 §9.4.3) in content-stream order, lossily decoded to UTF-8. No
//! `/ToUnicode` reverse mapping is applied: this standard's writer shows the field back through a
//! simple single-byte `/Type1` font, so what is read is what any reader recovers and
//! decode→encode→decode is stable on it. Font-decoded text is 1.7's richer reading, not 1.4's.
//!
//! 🦑 Codec + `register_schema_specs` dissolved out of the former `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); registration otherwise flows through
//! `crate::artifacts::pdf::declaration_1_4()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{
    diff::PdfDiff,
    snapshot::{PageDoc, PdfSnapshot},
};
use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use std::collections::{HashMap, HashSet};

/// 🔀️ The shared COS syntax layer, hosted in the 1.7 subtree (see the module doc comment).
use crate::artifacts::pdf::standards::v1_7::subsets::any::io as cos;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfObject};

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::PdfAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct PdfComposerComposition;

    impl ArtifactComposition for PdfComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PdfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PdfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PdfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Bytes
/// 📜️ ISO 32000-1 §7.2.2 Table 1 — the six white-space bytes of the COS grammar.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_ws(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | 0x0C | 0x00)
}

/// 📜️ ISO 32000-1 §7.2.2 Table 2 — the delimiter bytes. Everything else is a regular character,
/// which is why `'` and `"` (both text-showing operators) read as ordinary keyword runs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_delim(byte: u8) -> bool {
    matches!(byte, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%')
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_last_subslice(data: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > data.len() {
        return None;
    }
    (0..=data.len() - needle.len()).rev().find(|&index| &data[index..index + needle.len()] == needle)
}

/// 🔢️ One unsigned decimal integer at `at`, skipping leading white space. Returns the value and
/// the position just past its last digit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_uint(data: &[u8], at: usize) -> Option<(u64, usize)> {
    let mut index = at;
    while index < data.len() && is_ws(data[index]) {
        index += 1;
    }
    let start = index;
    while index < data.len() && data[index].is_ascii_digit() {
        index += 1;
    }
    if index == start || index - start > 19 {
        return None;
    }
    let text = std::str::from_utf8(&data[start..index]).ok()?;
    Some((text.parse().ok()?, index))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn skip_ws(data: &[u8], at: usize) -> usize {
    let mut index = at;
    while index < data.len() && is_ws(data[index]) {
        index += 1;
    }
    index
}
//#endregion 🔖️Bytes

//#region 🔖️IndirectObjects
/// 📦️ Parses the `N G obj … endobj` that starts at `offset`, returning the object number it
/// actually declared together with its value. The declared number is returned rather than assumed
/// because the brute-force rescan does not trust its own guess.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_indirect_at(data: &[u8], offset: usize) -> Option<(u32, PdfObject)> {
    let (number, after_number) = read_uint(data, offset)?;
    let (_generation, after_generation) = read_uint(data, after_number)?;
    let at_keyword = skip_ws(data, after_generation);
    if data.get(at_keyword..at_keyword + 3) != Some(b"obj") {
        return None;
    }
    let mut lexer = cos::Lexer::new(data);
    lexer.pos = at_keyword + 3;
    let value = lexer.parse_object().ok()?;
    Some((u32::try_from(number).ok()?, value))
}

/// 🩹 Brute-force rescan: every `N G obj` header in the whole buffer, last occurrence winning (an
/// incremental update's later revision shadows the earlier one, which is exactly what a classic
/// `/Prev` chain would have said). This is the fallback every real reader keeps for files whose
/// `startxref`/`xref` is damaged or absent — not a shortcut around parsing the table.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn brute_force_offsets(data: &[u8]) -> HashMap<u32, usize> {
    let mut found: HashMap<u32, usize> = HashMap::new();
    let mut index = 0usize;
    while index < data.len() {
        if data[index].is_ascii_digit() && (index == 0 || is_ws(data[index - 1]) || is_delim(data[index - 1])) {
            if let Some((number, after_number)) = read_uint(data, index) {
                if let Some((_, after_generation)) = read_uint(data, after_number) {
                    let at_keyword = skip_ws(data, after_generation);
                    if data.get(at_keyword..at_keyword + 3) == Some(b"obj") {
                        if let Ok(number) = u32::try_from(number) {
                            found.insert(number, index);
                        }
                        index = at_keyword + 3;
                        continue;
                    }
                }
            }
        }
        index += 1;
    }
    found
}
//#endregion 🔖️IndirectObjects

//#region 🔖️Xref
/// 📇️ The document's object index as PDF 1.4 spells it: byte offsets from a classic `xref` table
/// (plus its `/Prev` chain), and the `trailer` dictionary that terminates it.
struct Xref {
    offsets: HashMap<u32, usize>,
    trailer: Vec<PdfDictEntry>,
}

/// 📖️ Reads one classic `xref` section at `start` and every earlier section its trailer's `/Prev`
/// points at. Later sections win — a `/Prev` chain is read newest-first, and an entry already
/// carried by a newer section is the one in force (ISO 32000-1 §7.5.6).
///
/// Free entries (`f`) are recorded as absent rather than as offset 0: a free object is genuinely
/// not there, and treating its slot as an offset would make the resolver parse whatever byte 0 of
/// the file happens to be.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_classic_xref(data: &[u8], start: usize) -> Option<Xref> {
    let mut offsets: HashMap<u32, usize> = HashMap::new();
    let mut trailer: Vec<PdfDictEntry> = Vec::new();
    let mut next = Some(start);
    let mut visited: HashSet<usize> = HashSet::new();
    while let Some(section_start) = next.take() {
        if !visited.insert(section_start) || section_start >= data.len() {
            break;
        }
        let at_keyword = skip_ws(data, section_start);
        if data.get(at_keyword..at_keyword + 4) != Some(b"xref") {
            // 📄️ A `startxref` that does not point at a table is a 1.5+ cross-reference stream or
            // a damaged file; either way this standard's reader hands over to the rescan rather
            // than inventing a reading of bytes it does not model.
            return None;
        }
        let mut cursor = at_keyword + 4;
        loop {
            cursor = skip_ws(data, cursor);
            if data.get(cursor..cursor + 7) == Some(b"trailer") {
                cursor += 7;
                break;
            }
            let Some((first, after_first)) = read_uint(data, cursor) else { break };
            let Some((count, after_count)) = read_uint(data, after_first) else { break };
            cursor = skip_ws(data, after_count);
            for row in 0..count {
                if cursor + 18 > data.len() {
                    break;
                }
                let entry = &data[cursor..cursor + 18];
                let offset = std::str::from_utf8(&entry[0..10]).ok().and_then(|text| text.trim().parse::<usize>().ok());
                let kind = entry[17];
                if kind == b'n' {
                    if let (Some(offset), Ok(number)) = (offset, u32::try_from(first + row)) {
                        offsets.entry(number).or_insert(offset);
                    }
                }
                cursor += 18;
                while cursor < data.len() && is_ws(data[cursor]) {
                    cursor += 1;
                }
            }
        }
        let mut lexer = cos::Lexer::new(data);
        lexer.pos = cursor;
        let section_trailer = match lexer.parse_object() {
            Ok(PdfObject::Dict(entries)) => entries,
            _ => Vec::new(),
        };
        for entry in &section_trailer {
            if !trailer.iter().any(|existing| existing.key == entry.key) {
                trailer.push(entry.clone());
            }
        }
        next = section_trailer.iter().find(|entry| entry.key == "Prev").and_then(|entry| entry.value.as_i64()).and_then(|value| usize::try_from(value).ok());
    }
    if offsets.is_empty() {
        return None;
    }
    Some(Xref { offsets, trailer })
}

/// 🩹 The trailer of last resort: the dictionary after the file's last `trailer` keyword. Used
/// only when the table itself could not be read, so a rescued object graph still has a `/Root`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rescued_trailer(data: &[u8]) -> Vec<PdfDictEntry> {
    let Some(at) = find_last_subslice(data, b"trailer") else { return Vec::new() };
    let mut lexer = cos::Lexer::new(data);
    lexer.pos = at + b"trailer".len();
    match lexer.parse_object() {
        Ok(PdfObject::Dict(entries)) => entries,
        _ => Vec::new(),
    }
}

/// 📇️ The object index, table first and rescan second.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_index(data: &[u8]) -> Xref {
    let from_table = find_last_subslice(data, b"startxref")
        .and_then(|at| read_uint(data, at + b"startxref".len()))
        .and_then(|(offset, _)| usize::try_from(offset).ok())
        .filter(|offset| *offset < data.len())
        .and_then(|offset| read_classic_xref(data, offset));
    match from_table {
        Some(xref) => xref,
        None => {
            let mut trailer = rescued_trailer(data);
            let offsets = brute_force_offsets(data);
            if !trailer.iter().any(|entry| entry.key == "Root") {
                // 🔍️ No usable trailer either: the catalog is found by its own `/Type /Catalog`.
                let catalog = offsets.iter().find(|(_, offset)| parse_indirect_at(data, **offset).map(|(_, value)| value.dict_get("Type").and_then(|value| value.as_name()) == Some("Catalog")).unwrap_or(false)).map(|(number, _)| *number);
                if let Some(number) = catalog {
                    trailer.push(PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(ObjRef { num: number, gen: 0 }) });
                }
            }
            Xref { offsets, trailer }
        }
    }
}
//#endregion 🔖️Xref

//#region 🔖️Resolver
/// 🔗️ Lazily parses and memoizes indirect objects by number.
struct Resolver<'a> {
    data: &'a [u8],
    offsets: HashMap<u32, usize>,
    cache: HashMap<u32, Option<PdfObject>>,
}

impl<'a> Resolver<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(data: &'a [u8], offsets: HashMap<u32, usize>) -> Self {
        Self { data, offsets, cache: HashMap::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve(&mut self, number: u32) -> Option<PdfObject> {
        if let Some(hit) = self.cache.get(&number) {
            return hit.clone();
        }
        let parsed = self.offsets.get(&number).copied().and_then(|offset| parse_indirect_at(self.data, offset)).and_then(|(declared, value)| (declared == number).then_some(value));
        self.cache.insert(number, parsed.clone());
        parsed
    }
}
//#endregion 🔖️Resolver

//#region 🔖️ContentStream
/// 📝️ The shown text of one already-decoded content stream: every `Tj`/`'`/`"` operand and every
/// string element of every `TJ` array, in order, lossily decoded to UTF-8.
///
/// Deliberately NOT font-decoded (see the module doc comment). `'` and `"` (ISO 32000-1 §9.4.3's
/// next-line-show operators) take the shown string as their LAST operand, which is why the scan
/// keeps the operand list rather than only the first value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shown_text(content: &[u8]) -> String {
    let mut lexer = cos::Lexer::new(content);
    let mut operands: Vec<PdfObject> = Vec::new();
    let mut out: Vec<u8> = Vec::new();
    let push_last_string = |operands: &[PdfObject], out: &mut Vec<u8>| {
        if let Some(PdfObject::Str(bytes)) = operands.iter().rev().find(|operand| matches!(operand, PdfObject::Str(_))) {
            out.extend_from_slice(bytes);
        }
    };
    loop {
        lexer.skip_ws();
        let at = lexer.pos;
        let Some(byte) = content.get(at).copied() else { break };
        if matches!(byte, b'/' | b'(' | b'<' | b'[' | b'-' | b'+' | b'.' | b'0'..=b'9') {
            match lexer.parse_object() {
                Ok(value) => operands.push(value),
                Err(_) => lexer.pos = at + 1,
            }
            if lexer.pos <= at {
                lexer.pos = at + 1;
            }
            continue;
        }
        if is_delim(byte) {
            // 🧹 A stray closing delimiter (`]`, `>`, `)`) from a malformed stream: step over it
            // rather than looping forever on it.
            lexer.pos = at + 1;
            continue;
        }
        let start = at;
        let mut end = at;
        while end < content.len() && !is_ws(content[end]) && !is_delim(content[end]) {
            end += 1;
        }
        lexer.pos = if end > start { end } else { start + 1 };
        match &content[start..lexer.pos] {
            b"Tj" | b"'" | b"\"" => push_last_string(&operands, &mut out),
            b"TJ" => {
                if let Some(PdfObject::Array(items)) = operands.iter().rev().find(|operand| matches!(operand, PdfObject::Array(_))) {
                    for item in items {
                        if let PdfObject::Str(bytes) = item {
                            out.extend_from_slice(bytes);
                        }
                    }
                }
            }
            _ => {}
        }
        operands.clear();
    }
    String::from_utf8_lossy(&out).into_owned()
}
//#endregion 🔖️ContentStream

//#region 🔖️PageTree
/// 📐️ `/MediaBox`'s `[x0 y0 x1 y1]` reduced to this standard's `width`/`height` extent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_extent(value: &PdfObject) -> Option<(f64, f64)> {
    let items = value.as_array()?;
    if items.len() < 4 {
        return None;
    }
    let numbers: Vec<f64> = items.iter().take(4).map(|item| item.as_f64().unwrap_or(0.0)).collect();
    Some((numbers[2] - numbers[0], numbers[3] - numbers[1]))
}

/// 🌳️ Walks `/Root → /Pages → /Kids` down to the `/Page` leaves, carrying `/MediaBox` down the
/// inheritance chain (ISO 32000-1 §7.7.3.4) and extracting each leaf's shown text. Cycle-guarded:
/// real files occasionally carry a self-referential `/Kids`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_page_tree(node: ObjRef, resolver: &mut Resolver<'_>, inherited: Option<(f64, f64)>, visited: &mut HashSet<u32>, out: &mut Vec<PageDoc>) {
    if !visited.insert(node.num) {
        return;
    }
    let Some(value) = resolver.resolve(node.num) else { return };
    let here = value.dict_get("MediaBox").and_then(|media| match media {
        PdfObject::Ref(reference) => resolver.resolve(reference.num).as_ref().and_then(box_extent),
        other => box_extent(other),
    });
    let here = here.or(inherited);

    let kids: Option<Vec<ObjRef>> = value.dict_get("Kids").and_then(|kids| kids.as_array()).map(|kids| kids.iter().filter_map(|kid| kid.as_ref()).collect());
    if let Some(kids) = kids {
        for kid in kids {
            walk_page_tree(kid, resolver, here, visited, out);
        }
        return;
    }
    if value.dict_get("Type").and_then(|value| value.as_name()) == Some("Pages") {
        return;
    }

    let (width, height) = here.unwrap_or((PageDoc::DEFAULT_WIDTH, PageDoc::DEFAULT_HEIGHT));
    let mut combined: Vec<u8> = Vec::new();
    if let Some(contents) = value.dict_get("Contents") {
        let streams: Vec<ObjRef> = match contents {
            PdfObject::Ref(reference) => vec![*reference],
            PdfObject::Array(items) => items.iter().filter_map(|item| item.as_ref()).collect(),
            _ => Vec::new(),
        };
        for stream in streams {
            if let Some(PdfObject::Stream { dict, data, .. }) = resolver.resolve(stream.num) {
                if let Ok((decoded, _)) = cos::decode_stream(&dict, &data) {
                    if !combined.is_empty() {
                        combined.push(b'\n');
                    }
                    combined.extend_from_slice(&decoded);
                }
            }
        }
    }
    out.push(PageDoc { width, height, text: shown_text(&combined) });
}
//#endregion 🔖️PageTree

//#region 🔖️Decode
/// 📥️ Reads a real PDF 1.4 document into this standard's page-tree snapshot.
///
/// An encrypted document is refused rather than guessed at: `/Encrypt` means every string and
/// stream in the file is enciphered, and a reader that shrugged and read the ciphertext would
/// report confident nonsense.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf(data: &[u8]) -> Result<PdfSnapshot, String> {
    if !data.starts_with(b"%PDF-") {
        return Err("pdf 1.4: not a PDF file (missing %PDF- header)".to_string());
    }
    let index = build_index(data);
    if index.trailer.iter().any(|entry| entry.key == "Encrypt") {
        return Err("pdf 1.4: /Encrypt is present — an encrypted document is refused, never guessed at".to_string());
    }
    let root = index.trailer.iter().find(|entry| entry.key == "Root").and_then(|entry| entry.value.as_ref()).ok_or_else(|| "pdf 1.4: the trailer names no /Root catalog".to_string())?;
    let mut resolver = Resolver::new(data, index.offsets.clone());
    let catalog = resolver.resolve(root.num).ok_or_else(|| format!("pdf 1.4: the /Root catalog (object {}) is not in the cross-reference table", root.num))?;
    if catalog.dict_get("Encrypt").is_some() {
        return Err("pdf 1.4: /Encrypt is present on /Root — an encrypted document is refused, never guessed at".to_string());
    }
    let pages_root = catalog.dict_get("Pages").and_then(|value| value.as_ref()).ok_or_else(|| "pdf 1.4: the /Root catalog names no /Pages tree".to_string())?;
    let mut pages = Vec::new();
    let mut visited = HashSet::new();
    walk_page_tree(pages_root, &mut resolver, None, &mut visited, &mut pages);
    if pages.is_empty() {
        return Err("pdf 1.4: the /Pages tree resolved to no page at all".to_string());
    }
    Ok(PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🔢️ A PDF real, written the way ISO 32000-1 §7.3.3 permits: plain decimal, never exponent
/// notation (which the COS grammar has no production for).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pdf_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    let plain = format!("{value}");
    if plain.contains(['e', 'E']) {
        return format!("{value:.6}");
    }
    plain
}

/// 🔤️ One `(…)` literal string operand, written as BYTES. ISO 32000-1 §7.3.4.2 gives `\`, `(`
/// and `)` special meaning; the two EOL bytes are escaped as well so a writer's line structure can
/// never be mistaken for the string's own. Every other byte of `text`'s UTF-8 encoding goes out
/// verbatim — a literal string is a byte string, and the reader recovers exactly what was written.
///
/// ⚠️ This function returns `Vec<u8>` and not `String` for a reason the differential run found:
/// pushing each byte as `byte as char` into a `String` re-encodes every byte ≥ `0x80` as the TWO
/// UTF-8 bytes of the Latin-1 code point of the same value, so the reader gets `Ã©` back where
/// `é` was written and `decode → encode → decode` is not stable. The committed 65-page thesis
/// carries exactly one such byte on page 1 (a glyph code with no Unicode reading, which
/// [`shown_text`]'s lossy decode turns into `U+FFFD`), which was enough to move the projection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn literal_string(text: &str) -> Vec<u8> {
    let mut out: Vec<u8> = vec![b'('];
    for byte in text.bytes() {
        match byte {
            b'\\' => out.extend_from_slice(b"\\\\"),
            b'(' => out.extend_from_slice(b"\\("),
            b')' => out.extend_from_slice(b"\\)"),
            b'\r' => out.extend_from_slice(b"\\r"),
            b'\n' => out.extend_from_slice(b"\\n"),
            other => out.push(other),
        }
    }
    out.push(b')');
    out
}

/// ✏️️ One page's content stream. The text is shown through a simple `/Type1` font as a literal
/// string, so the operand bytes a reader recovers ARE the snapshot's own `text` — the property
/// [`decode_pdf`] relies on for a stable round trip. A page with no text gets a genuinely empty
/// text object rather than a `Tj` of the empty string.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_ops(page: &PageDoc) -> Vec<u8> {
    if page.text.is_empty() {
        return b"BT\nET\n".to_vec();
    }
    let baseline = pdf_number(page.height - 72.0);
    let mut out = format!("BT\n/F1 12 Tf\n72 {baseline} Td\n").into_bytes();
    out.extend_from_slice(&literal_string(&page.text));
    out.extend_from_slice(b" Tj\nET\n");
    out
}

/// 📤️ Writes the snapshot as a real PDF 1.4 file: catalog, one-level page tree with every page,
/// a per-page FlateDecode content stream, a classic cross-reference table and a trailer.
///
/// Byte layout is fully deterministic (no timestamps, no document id), so the same snapshot always
/// encodes to the same bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pdf(snapshot: &PdfSnapshot) -> Result<Vec<u8>, String> {
    let pages: Vec<PageDoc> = if snapshot.pages.is_empty() { vec![PageDoc::default()] } else { snapshot.pages.clone() };
    let needs_font = pages.iter().any(|page| !page.text.is_empty());

    let mut next_number = 1u32;
    let mut allocate = || {
        let number = next_number;
        next_number += 1;
        number
    };
    let catalog_number = allocate();
    let pages_number = allocate();
    let font_number = needs_font.then(&mut allocate);
    let page_numbers: Vec<(u32, u32)> = pages.iter().map(|_| (allocate(), allocate())).collect();

    let mut objects: Vec<(u32, Vec<u8>)> = Vec::new();
    if let Some(font_number) = font_number {
        objects.push((font_number, format!("{font_number} 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>\nendobj\n").into_bytes()));
    }

    let mut kids = String::new();
    for (page, (page_number, content_number)) in pages.iter().zip(&page_numbers) {
        let compressed = crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&content_ops(page))?;
        let mut content = Vec::new();
        content.extend_from_slice(format!("{content_number} 0 obj\n<< /Length {} /Filter /FlateDecode >>\nstream\n", compressed.len()).as_bytes());
        content.extend_from_slice(&compressed);
        content.extend_from_slice(b"\nendstream\nendobj\n");
        objects.push((*content_number, content));

        let resources = match font_number {
            Some(font_number) => format!("<< /Font << /F1 {font_number} 0 R >> >>"),
            None => "<< >>".to_string(),
        };
        let media = format!("[0 0 {} {}]", pdf_number(page.width), pdf_number(page.height));
        objects.push((*page_number, format!("{page_number} 0 obj\n<< /Type /Page /Parent {pages_number} 0 R /MediaBox {media} /Contents {content_number} 0 R /Resources {resources} >>\nendobj\n").into_bytes()));
        kids.push_str(&format!("{page_number} 0 R "));
    }
    objects.push((pages_number, format!("{pages_number} 0 obj\n<< /Type /Pages /Kids [{}] /Count {} >>\nendobj\n", kids.trim_end(), pages.len()).into_bytes()));
    objects.push((catalog_number, format!("{catalog_number} 0 obj\n<< /Type /Catalog /Pages {pages_number} 0 R >>\nendobj\n").into_bytes()));
    objects.sort_by_key(|(number, _)| *number);

    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");
    let mut offsets = vec![0usize; next_number as usize];
    for (number, bytes) in &objects {
        offsets[*number as usize] = body.len();
        body.extend_from_slice(bytes);
    }
    let xref_offset = body.len();
    body.extend_from_slice(format!("xref\n0 {next_number}\n0000000000 65535 f \n").as_bytes());
    for number in 1..next_number {
        body.extend_from_slice(format!("{:010} 00000 n \n", offsets[number as usize]).as_bytes());
    }
    body.extend_from_slice(format!("trailer\n<< /Size {next_number} /Root {catalog_number} 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n").as_bytes());
    Ok(body)
}
//#endregion 🔖️Encode

//#region 🔖️SchemaSpecs
/// 📇️ P2-FG3: `dsl::registry::register_schema_spec` -- genuinely callable for the SNAPSHOT:
/// `PdfSnapshot`/`PageDoc` both derive `dsl::DslRecord`, so `__dsl_spec` genuinely exists.
///
/// 🔺️ The DIFF is deliberately not registered. `PdfDiff` carries the index-keyed `pages` triple
/// (`removed`/`modified`/`added`) a real page tree needs, and that shape is off `#[derive(dsl::
/// DslDiff)]`'s path, so `__dsl_diff_spec` does not exist to register — the diff's wire shape is
/// the handcrafted `DiffCodec` in `🔺️diff/🦀️component.rs`, stated production for production in
/// that facet's own `📝️text/📖️component.grammar.semio` and `💾️binary/📡️component.protocol.semio`.
/// Same position `gif` 87a/89a's identical registration functions hold for the same reason.
/// `PdfMutation`'s own per-variant specs are NOT registered here either -- same scope boundary
/// binary/raw's and txt's own registration functions document: `register_schema_spec` registers
/// one spec under one schema id, and there is no single canonical id for a Mutation enum's
/// per-variant shapes.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    semio_framework_plugin::resolve_ready(dsl::registry::register_schema_spec("stdio.pdf", PdfSnapshot::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}
//#endregion 🔖️SchemaSpecs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::demo_pdf_snapshot;

    /// 🧫️ The real committed document every 1.4 test case runs on — 65 pages, a classic
    /// cross-reference table, page 1 typeset at A4.
    const THESIS: &[u8] = include_bytes!("../📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    /// 🧪️ `codec_retention_law`: decode→encode→decode is stable on everything this standard's
    /// snapshot carries — every page, its extent and its shown text.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law_round_trips_every_page() {
        let original = PdfSnapshot {
            schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
            pages: vec![
                PageDoc { width: 612.0, height: 792.0, text: "Hello Semio".into() },
                PageDoc { width: 595.276, height: 841.89, text: "Zweite Seite (mit Klammern)".into() },
                PageDoc { width: 200.0, height: 300.0, text: String::new() },
                // 🔤 Non-ASCII, and a `U+FFFD` — what `shown_text`'s lossy decode leaves behind for
                // a glyph code with no Unicode reading, which the committed thesis carries on page 1.
                // Writing a literal string through `byte as char` re-encoded every byte ≥ 0x80 as
                // TWO bytes and broke exactly this round trip; the differential run caught it.
                PageDoc { width: 300.0, height: 400.0, text: "Grüße \u{fffd} 中文 — a\\b(c)d".into() },
            ],
        };
        let bytes = encode_pdf(&original).expect("encode");
        let redecoded = decode_pdf(&bytes).expect("decode");
        assert_eq!(redecoded.pages, original.pages);
    }

    /// 🧪️ The defect this standard's real codec exists to close: the committed 65-page thesis used
    /// to come back as a one-page, 607-byte skeleton. Every page must survive decode AND encode.
    #[semio_framework_async_macros::async_test]
    async fn the_real_65_page_thesis_survives_decode_and_re_encode() {
        let decoded = decode_pdf(THESIS).expect("the committed thesis decodes");
        assert_eq!(decoded.pages.len(), 65, "the thesis has 65 pages and every one of them must be read");
        let first = &decoded.pages[0];
        assert!((first.width - 595.276).abs() < 1e-3 && (first.height - 841.89).abs() < 1e-3, "page 1 is typeset at A4, got {}x{}", first.width, first.height);
        assert!(first.text.contains("SemIO"), "page 1's shown text must be recovered, got {:?}", first.text);

        let bytes = encode_pdf(&decoded).expect("re-encode");
        assert_ne!(bytes, THESIS, "a re-encode from the model alone can never be a byte pass-through");
        let again = decode_pdf(&bytes).expect("the re-encoded document decodes");
        assert_eq!(again.pages.len(), 65, "re-encoding must not drop a single page");
        assert_eq!(again.pages[0].text, first.text, "page 1's shown text must survive the re-encode");
        assert!((again.pages[0].width - first.width).abs() < 1e-6);
    }

    /// 🧪️ A degenerate page extent is a real, writable state — `1.4/✳️x`'s `collapse-page-size`
    /// depends on it — so the writer must never clamp it away.
    #[semio_framework_async_macros::async_test]
    async fn a_collapsed_page_extent_is_written_as_it_stands() {
        let snapshot = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 0.0, height: 841.89, text: "x".into() }] };
        let bytes = encode_pdf(&snapshot).expect("encode");
        assert!(String::from_utf8_lossy(&bytes).contains("/MediaBox [0 0 0 841.89]"));
        assert_eq!(decode_pdf(&bytes).expect("decode").pages[0].width, 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_document_that_is_not_a_pdf_is_refused() {
        assert!(decode_pdf(b"not a pdf at all").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_snapshot_round_trip() {
        let snap = demo_pdf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG3: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives
    /// here (the engine's own test region), not any framework file.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation;
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn demo_mutation_cases() -> Vec<PdfMutation> {
            vec![
                PdfMutation::NoMutation,
                PdfMutation::SetSnapshot { snapshot: demo_pdf_snapshot() },
                PdfMutation::SetSnapshot {
                    snapshot: PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "hello world".into() }, PageDoc { width: 200.0, height: 300.0, text: String::new() }] },
                },
            ]
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn demo_diff_cases() -> Vec<PdfDiff> {
            let a = demo_pdf_snapshot();
            let b = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 300.5, height: 400.25, text: "changed text".into() }, PageDoc { width: 100.0, height: 100.0, text: "second".into() }] };
            let c = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: Vec::new() };
            vec![
                <PdfDiff as protocol::command::DiffAlgebra<PdfSnapshot>>::between(&a, &b),
                <PdfDiff as protocol::command::DiffAlgebra<PdfSnapshot>>::between(&b, &c),
                PdfDiff::default(),
            ]
        }

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect.
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_pdf_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every demo `PdfMutation`.
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_pdf_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_pdf_snapshot()`.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_pdf_snapshot();

            let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pdf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pdf_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pdf_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pdf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::pdf::standards::v1_4::subsets::a::schema::PdfAComposer;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::PdfComposer as PdfRawAnyComposer;
    use crate::artifacts::pdf::standards::v1_4::subsets::x::schema::PdfXComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PdfRawAnyComposer>(), composer_entry_of::<PdfAComposer>(), composer_entry_of::<PdfXComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
