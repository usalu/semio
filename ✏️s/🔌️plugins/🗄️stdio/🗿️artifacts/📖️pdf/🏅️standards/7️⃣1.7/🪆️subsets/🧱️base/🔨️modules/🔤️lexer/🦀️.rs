//! 🔤️ COS syntax (ISO 32000-1 §7.2–7.3): a cursor lexer/parser over the object grammar, the
//! indirect-object and brute-force scanners the cross-reference layer builds on, and the
//! canonical object writer every emitted byte goes through. Shared by both standards of the
//! artifact (PDF 1.4 and 1.7 have the same lexical grammar).

#[path = "🔤️strings/🦀️.rs"]
mod strings;
pub use strings::{PdfByteStringCursor, StringReceipt, StringStep};

#[path = "🧱️containers/🦀️.rs"]
mod containers;
pub use containers::{CosGrant, CosLimits, CosReceipt, CosStatus, PdfCosCursor, grow_table, table_growth};

#[path = "🖼️inline-image-span/🦀️.rs"]
mod inline_image_span;
pub use inline_image_span::{InlineImageBoundary, InlineImageSpanCursor, InlineImageSpanStep};

#[path = "📏️stream-span/🦀️.rs"]
mod stream_span;
pub use stream_span::{StreamSpan, StreamSpanCursor, StreamSpanStep};

#[path = "🎫️content-token/🦀️.rs"]
mod content_token;
pub use content_token::{ContentToken, ContentTokenCursor, ContentTokenEvent, ContentTokenReceipt};

#[path = "🧹️stream-dictionary/🦀️.rs"]
mod stream_dictionary;
pub use stream_dictionary::{retain_stream_entry, PdfStreamDictionaryCursor};

#[path = "🪪️pdf-header/🦀️.rs"]
mod pdf_header;
pub use pdf_header::{pdf_header, PdfHeader, PdfHeaderCursor, PdfHeaderStep};

#[path = "📐️indirect-length/🦀️.rs"]
mod indirect_length;
pub use indirect_length::PdfIndirectLengthCursor;

#[path = "🪪️indirect-header/🦀️.rs"]
mod indirect_header;
pub use indirect_header::{HeaderStep, IndirectHeaderCursor};

#[path = "📦️stream-prefix/🦀️.rs"]
mod stream_prefix;
pub use stream_prefix::{StreamPrefixCursor, StreamPrefixStep};

#[path = "📦️object-frame/🦀️.rs"]
mod object_frame;
pub use object_frame::{IndirectFrame, IndirectFrameCursor, IndirectFrameReceipt, IndirectFrameStatus};

#[path = "🧭️normal-object/🦀️.rs"]
mod normal_object;
pub use normal_object::{NormalObject, NormalObjectReceipt, NormalObjectStatus, PdfNormalObjectCursor};

#[path = "🧭️object-scan/🦀️.rs"]
mod object_scan;
pub use object_scan::{ObjectScanCursor, ObjectScanStep};

#[path = "🧭️xref-section/🦀️.rs"]
mod xref_section;
pub use xref_section::{XrefSectionKind, XrefSectionProbe};

#[path = "🪢️xref-links/🦀️.rs"]
mod xref_links;
pub use xref_links::{XrefLinkCursor, XrefLinks, XrefPrimaryCycle, XrefRevisionSchedule, XrefTarget};

use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error::{malformed, PResult, PdfEngineError};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{stream_bytes, ObjRef, PdfDecimal, PdfDictEntry, PdfObject};
use std::collections::HashMap;


//#region 🔖️Characters
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 0x0C | 0x00)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_delim(b: u8) -> bool {
    matches!(b, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%')
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_subslice(data: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    if from > data.len() || needle.is_empty() {
        return None;
    }
    data[from..].windows(needle.len()).position(|w| w == needle).map(|p| p + from)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_last_subslice(data: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > data.len() {
        return None;
    }
    (0..=data.len() - needle.len()).rev().find(|&i| &data[i..i + needle.len()] == needle)
}
//#endregion 🔖️Characters

//#region 🔖️Lexer
/// 🎫 One token of a content stream or object body: a value, a bare keyword (an operator or
/// `obj`/`stream`/…), or the end of input.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Object(PdfObject),
    Keyword(String),
    End,
}

/// 🔍 Cursor-based recursive-descent lexer/parser over the PDF COS object grammar. Used for
/// top-level `N G obj ... endobj` parsing, values nested inside arrays/dicts, and content streams.
pub struct Lexer<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    pub fn at(&self, offset: usize) -> Self {
        Self { data: self.data, pos: offset }
    }
    pub fn peek(&self) -> Option<u8> {
        self.data.get(self.pos).copied()
    }
    fn peek_at(&self, n: usize) -> Option<u8> {
        self.data.get(self.pos + n).copied()
    }

    pub fn skip_ws(&mut self) {
        loop {
            match self.peek() {
                Some(b) if is_ws(b) => {
                    self.pos += 1;
                }
                Some(b'%') => {
                    while let Some(c) = self.peek() {
                        self.pos += 1;
                        if c == b'\n' || c == b'\r' {
                            break;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    pub fn read_regular_run(&mut self) -> &'a [u8] {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if is_ws(b) || is_delim(b) {
                break;
            }
            self.pos += 1;
        }
        &self.data[start..self.pos]
    }

    pub fn starts_with(&self, kw: &[u8]) -> bool {
        self.data.get(self.pos..self.pos + kw.len()) == Some(kw)
    }

    pub fn consume_keyword(&mut self, kw: &[u8]) -> bool {
        if self.starts_with(kw) {
            self.pos += kw.len();
            true
        } else {
            false
        }
    }

    pub fn parse_number(&mut self) -> PResult<PdfObject> {
        let start = self.pos;
        if matches!(self.peek(), Some(b'+') | Some(b'-')) {
            self.pos += 1;
        }
        let mut is_real = false;
        let mut saw_digit = false;
        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' => {
                    saw_digit = true;
                    self.pos += 1;
                }
                b'.' => {
                    is_real = true;
                    self.pos += 1;
                }
                b'+' | b'-' => {
                    self.pos += 1;
                }
                _ => break,
            }
        }
        if !saw_digit {
            return malformed("expected number");
        }
        let raw = std::str::from_utf8(&self.data[start..self.pos]).unwrap_or("0");
        let mut text = String::with_capacity(raw.len());
        let mut seen_dot = false;
        for (index, character) in raw.chars().enumerate() {
            match character {
                '-' | '+' if index == 0 => text.push(character),
                '-' | '+' => {}
                '.' if !seen_dot => {
                    seen_dot = true;
                    text.push('.');
                }
                '.' => {}
                digit => text.push(digit),
            }
        }
        if is_real {
            PdfDecimal::parse(&text).map(PdfObject::Real).map_err(PdfEngineError::Malformed)
        } else {
            match text.parse::<i64>() {
                Ok(value) => Ok(PdfObject::Int(value)),
                Err(_) => PdfDecimal::parse(&format!("{text}.")).map(PdfObject::Real).map_err(PdfEngineError::Malformed),
            }
        }
    }

    fn parse_name(&mut self) -> PdfObject {
        self.pos += 1;
        let mut out = Vec::new();
        while let Some(b) = self.peek() {
            if is_ws(b) || is_delim(b) {
                break;
            }
            if b == b'#' && self.peek_at(1).is_some_and(|c| c.is_ascii_hexdigit()) && self.peek_at(2).is_some_and(|c| c.is_ascii_hexdigit()) {
                out.push((hex_val(self.data[self.pos + 1]) << 4) | hex_val(self.data[self.pos + 2]));
                self.pos += 3;
                continue;
            }
            out.push(b);
            self.pos += 1;
        }
        PdfObject::Name(String::from_utf8(out).unwrap_or_else(|error| error.into_bytes().into_iter().map(char::from).collect()))
    }

    fn parse_byte_string(&mut self) -> PResult<PdfObject> {
        let mut cursor = PdfByteStringCursor::new(self.pos);
        let mut bytes = Vec::new();
        let mut scratch = [0u8; 256];
        loop {
            let receipt = cursor.advance(self.data, &mut scratch, 1024);
            self.pos = cursor.position();
            bytes.extend_from_slice(&scratch[..receipt.written]);
            match receipt.status {
                Ok(StringStep::Complete) => return Ok(PdfObject::Str(bytes)),
                Ok(StringStep::More | StringStep::NeedsOutput) => {}
                Err(reason) => return malformed(reason),
            }
        }
    }

    fn parse_container(&mut self) -> PResult<PdfObject> {
        let mut cursor = PdfCosCursor::new(self.pos, CosLimits::default());
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        loop {
            match cursor.advance(self.data, grant, false).status {
                Ok(CosStatus::Complete { end }) => {
                    self.pos = end;
                    return cursor.take_value().ok_or_else(|| PdfEngineError::Malformed("completed COS container has no value".into()));
                }
                Ok(CosStatus::More) => {}
                Ok(_) => return malformed("unexpected bounded COS parser status"),
                Err(reason) => {
                    self.pos = cursor.inspected_position();
                    cursor.begin_close();
                    while !cursor.terminal_is_empty() { cursor.close_step(grant); }
                    return malformed(reason);
                }
            }
        }
    }

    fn parse_dict_or_stream(&mut self, resolve_length: &mut dyn FnMut(ObjRef) -> Option<usize>) -> PResult<PdfObject> {
        let PdfObject::Dict(entries) = self.parse_container()? else { return malformed("expected PDF COS dictionary"); };
        let mut prefix = StreamPrefixCursor::new(self.pos);
        let body = loop {
            if let (_, StreamPrefixStep::Complete(body)) = prefix.advance(self.data, 4096) { break body; }
        };
        if let Some(data_start) = body {
            let declared_len = entries.iter().find(|e| e.key == "Length").and_then(|e| match &e.value {
                PdfObject::Int(i) => usize::try_from(*i).ok(),
                PdfObject::Ref(reference) => resolve_length(*reference),
                _ => None,
            });
            let mut span = StreamSpanCursor::new(data_start, declared_len, self.data.len());
            let span = loop {
                if let Some(span) = span.advance(self.data, 4096).span { break span; }
            };
            let raw = self.data[span.start..span.end].to_vec();
            self.pos = span.resume;
            return Ok(PdfObject::Stream { dict: entries, data: stream_bytes(&raw), filters: Vec::new() });
        }
        Ok(PdfObject::Dict(entries))
    }

    /// 🎯 Parses one value: number, `N G R` reference, name, string, array, dict/stream,
    /// `true`/`false`/`null`. A bare keyword yields `Null` (use [`Lexer::next_token`] where
    /// keywords matter).
    pub fn parse_object(&mut self) -> PResult<PdfObject> {
        match self.next_token()? {
            Token::Object(value) => Ok(value),
            Token::Keyword(_) => Ok(PdfObject::Null),
            Token::End => malformed("unexpected end of input"),
        }
    }

    /// 🎫 Reads the next token: a value (with `N G R` references folded), a keyword, or the end.
    pub fn next_token(&mut self) -> PResult<Token> {
        loop {
            self.skip_ws();
            return match self.peek() {
                None => Ok(Token::End),
                Some(b'/') => Ok(Token::Object(self.parse_name())),
                Some(b'(') => self.parse_byte_string().map(Token::Object),
                Some(b'<') if self.peek_at(1) == Some(b'<') => self.parse_dict_or_stream(&mut |_| None).map(Token::Object),
                Some(b'<') => self.parse_byte_string().map(Token::Object),
                Some(b'[') => self.parse_container().map(Token::Object),
                Some(b']' | b'>' | b')' | b'{' | b'}') => {
                    self.pos += 1;
                    continue;
                }
                Some(b'-' | b'+' | b'.' | b'0'..=b'9') => {
                    let first = match self.parse_number() {
                        Ok(value) => value,
                        Err(_) => { self.pos += 1; continue; }
                    };
                    if let PdfObject::Int(num) = first {
                        if let Ok(num) = u32::try_from(num) {
                            let save = self.pos;
                            self.skip_ws();
                            if matches!(self.peek(), Some(b'0'..=b'9')) {
                                if let Ok(PdfObject::Int(gen)) = self.parse_number() {
                                    if let Ok(gen) = u16::try_from(gen) {
                                        self.skip_ws();
                                        if self.consume_keyword(b"R") && self.peek().is_none_or(|b| is_ws(b) || is_delim(b)) {
                                            return Ok(Token::Object(PdfObject::Ref(ObjRef { num, gen })));
                                        }
                                    }
                                }
                            }
                            self.pos = save;
                        }
                    }
                    Ok(Token::Object(first))
                }
                Some(_) => {
                    let run = self.read_regular_run();
                    if run.is_empty() { self.pos += 1; continue; }
                    Ok(match run {
                        b"true" => Token::Object(PdfObject::Bool(true)),
                        b"false" => Token::Object(PdfObject::Bool(false)),
                        b"null" => Token::Object(PdfObject::Null),
                        other => Token::Keyword(String::from_utf8_lossy(other).into_owned()),
                    })
                }
            };
        }
    }
}
//#endregion 🔖️Lexer

//#region 🔖️IndirectObjects
/// 📦️ Parses one `N G obj ... endobj` at `offset`. Returns the parsed value and the id it
/// actually declared (used by the brute-force scanner, which doesn't trust its own guessed id).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_indirect_at(data: &[u8], offset: usize) -> PResult<(ObjRef, PdfObject)> {
    parse_indirect_with_length(data, offset, &mut |_| None)
}

/// 🪪️ Inspect an xref target without decoding or copying its stream body.
pub fn indirect_header_at(data: &[u8], offset: usize) -> PResult<(ObjRef, Lexer<'_>)> {
    let mut cursor = IndirectHeaderCursor::new(offset);
    loop {
        let (_, step) = cursor.advance(|position| data.get(position).copied(), 4096).map_err(|reason| PdfEngineError::Malformed(reason.into()))?;
        if let HeaderStep::Complete { id, body_offset } = step {
            return Ok((id, Lexer::new(data).at(body_offset)));
        }
    }
}

/// 📏️ Resolves an outer stream’s indirect byte length against the current cross-reference revision.
pub fn parse_indirect_with_length(data: &[u8], offset: usize, resolve_length: &mut dyn FnMut(ObjRef) -> Option<usize>) -> PResult<(ObjRef, PdfObject)> {
    let NormalObject { id, value, stream, .. } = frame_indirect_with_length(data, offset, resolve_length)?;
    let value = match (value, stream) {
        (PdfObject::Dict(dict), Some(span)) => PdfObject::Stream {
            dict, data: stream_bytes(&data[span.start..span.end]), filters: Vec::new(),
        },
        (value, _) => value,
    };
    Ok((id, value))
}

/// 🪟️ Frames one indirect object and retains any stream as a span of the original source.
pub fn frame_indirect_with_length(data: &[u8], offset: usize, resolve_length: &mut dyn FnMut(ObjRef) -> Option<usize>) -> PResult<NormalObject> {
    let mut cursor = PdfNormalObjectCursor::new(offset, CosLimits::default());
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let result = loop {
        match cursor.advance(data, grant, false).status {
            Ok(NormalObjectStatus::More) => {}
            Ok(NormalObjectStatus::NeedsLength(reference)) => {
                if let Err(reason) = cursor.provide_length(reference, resolve_length(reference)) { break malformed(reason); }
            }
            Ok(NormalObjectStatus::Complete) => break cursor.take_object().ok_or_else(|| PdfEngineError::Malformed("normal object is absent".into())),
            Err("unexpected PDF COS keyword") => {
                break indirect_header_at(data, offset).and_then(|(id, mut lex)| {
                    lex.skip_ws();
                    if lex.starts_with(b"endobj") { Ok(NormalObject { id, value: PdfObject::Null, value_end: lex.pos, stream: None }) }
                    else { malformed("unexpected PDF COS keyword") }
                });
            }
            Err(reason) => break malformed(reason),
            Ok(_) => break malformed("unexpected normal object parser status"),
        }
    };
    cursor.begin_close();
    while matches!(cursor.close_step(grant).status, Ok(NormalObjectStatus::More)) {}
    result
}

/// 🩹 Brute-force fallback: scans the whole buffer for `N G obj` patterns — used when structured
/// xref parsing fails outright (damaged/`%%EOF`-free files). Real readers all do this; the last
/// occurrence of a given object number wins (later generation/incremental update).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn brute_force_scan(data: &[u8]) -> HashMap<u32, (ObjRef, usize)> {
    let mut found = HashMap::new();
    let mut cursor = ObjectScanCursor::new();
    loop {
        match cursor.advance(data, 4096).1 {
            ObjectScanStep::Found { id, offset } => { found.insert(id.num, (id, offset)); }
            ObjectScanStep::More => {}
            ObjectScanStep::Complete => return found,
        }
    }
}
//#endregion 🔖️IndirectObjects

//#region 🔖️Writer
/// 🔢️ Canonical number text: integers bare, reals exact and exponent-free (PDF has no exponent
/// syntax), `-0` folded to `0`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn number_text(value: f64) -> String {
    if !value.is_finite() {
        return "0".into();
    }
    if value.fract() == 0.0 && value.abs() < 1e15 {
        return format!("{}", value as i64);
    }
    let decimal = PdfDecimal::from_f64(value);
    let mut text = decimal.to_string();
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        text = "0".into();
    }
    text
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_name(out: &mut Vec<u8>, name: &str) {
    out.push(b'/');
    for byte in name.bytes() {
        if (33..=126).contains(&byte) && !is_delim(byte) && byte != b'#' {
            out.push(byte);
        } else {
            out.extend_from_slice(format!("#{byte:02X}").as_bytes());
        }
    }
}

/// 🔤️ Writes a string operand: literal when printable ASCII (escaping the §7.3.4.2 specials and
/// balanced-parenthesis-free), hex otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_string(out: &mut Vec<u8>, bytes: &[u8]) {
    if bytes.iter().all(|byte| matches!(byte, 0x20..=0x7e)) {
        out.push(b'(');
        for byte in bytes {
            if matches!(byte, b'(' | b')' | b'\\') {
                out.push(b'\\');
            }
            out.push(*byte);
        }
        out.push(b')');
    } else {
        out.push(b'<');
        for byte in bytes {
            out.extend_from_slice(format!("{byte:02X}").as_bytes());
        }
        out.push(b'>');
    }
}

/// ✍️ Writes one direct object in canonical form. Streams are written with their `data` AS IS
/// and `/Length` set from it — the caller is responsible for having applied the stream's
/// filters (@see `filters::encode_stream`) so the dictionary and the bytes agree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_object(out: &mut Vec<u8>, object: &PdfObject) {
    match object {
        PdfObject::Null => out.extend_from_slice(b"null"),
        PdfObject::Bool(value) => out.extend_from_slice(if *value { b"true" } else { b"false" }),
        PdfObject::Int(value) => out.extend_from_slice(value.to_string().as_bytes()),
        PdfObject::Real(value) => out.extend_from_slice(number_text(value.to_f64().unwrap_or(0.0)).as_bytes()),
        PdfObject::Str(bytes) => write_string(out, bytes),
        PdfObject::Text(text) => write_string(out,&semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string::encode_text_string(text)),
        PdfObject::Date(date) => write_string(out,semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date(date).as_bytes()),
        PdfObject::Name(name) => write_name(out, name),
        PdfObject::Array(items) => {
            out.push(b'[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(b' ');
                }
                write_object(out, item);
            }
            out.push(b']');
        }
        PdfObject::Dict(entries) => write_dict(out, entries),
        PdfObject::Ref(reference) => out.extend_from_slice(format!("{} {} R", reference.num, reference.gen).as_bytes()),
        PdfObject::Stream { dict, data, .. } => {
            let mut entries: Vec<PdfDictEntry> = dict.iter().filter(|entry| entry.key != "Length").cloned().collect();
            entries.push(PdfDictEntry::new("Length", PdfObject::Int(data.len() as i64)));
            write_dict(out, &entries);
            out.extend_from_slice(b"\nstream\n");
            out.extend(data.iter());
            out.extend_from_slice(b"\nendstream");
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_dict(out: &mut Vec<u8>, entries: &[PdfDictEntry]) {
    out.extend_from_slice(b"<<");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            out.push(b' ');
        }
        write_name(out, &entry.key);
        if !matches!(entry.value, PdfObject::Name(_) | PdfObject::Array(_) | PdfObject::Dict(_) | PdfObject::Str(_) | PdfObject::Text(_) | PdfObject::Date(_) | PdfObject::Stream { .. }) {
            out.push(b' ');
        }
        if let ("URI",PdfObject::Text(text))=(entry.key.as_str(),&entry.value) {write_string(out,text.as_bytes());}else{write_object(out, &entry.value);}
    }
    out.extend_from_slice(b">>");
}

/// 🧾 Serializes one direct object to bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn object_bytes(object: &PdfObject) -> Vec<u8> {
    let mut out = Vec::new();
    write_object(&mut out, object);
    out
}
//#endregion 🔖️Writer

//#region 🔖️DictHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_get<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> {
    entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_put(entries: &mut Vec<PdfDictEntry>, key: &str, value: PdfObject) {
    match entries.iter_mut().find(|entry| entry.key == key) {
        Some(entry) => entry.value = value,
        None => entries.push(PdfDictEntry::new(key, value)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_drop(entries: &mut Vec<PdfDictEntry>, key: &str) {
    entries.retain(|entry| entry.key != key);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_i64(entries: &[PdfDictEntry], key: &str) -> Option<i64> {
    dict_get(entries, key).and_then(PdfObject::as_i64)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_f64(entries: &[PdfDictEntry], key: &str) -> Option<f64> {
    dict_get(entries, key).and_then(PdfObject::as_f64)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_name<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
    dict_get(entries, key).and_then(PdfObject::as_name)
}
//#endregion 🔖️DictHelpers

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_lexer_resumes_after_long_unmatched_delimiter_sequences() {
        let mut input = vec![b']'; 200_000];
        input.extend_from_slice(b" q 2 70000 R 42 65535 R");
        let mut lexer = Lexer::new(&input);
        assert_eq!(lexer.next_token().unwrap(), Token::Keyword("q".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Object(PdfObject::Int(2)));
        assert_eq!(lexer.next_token().unwrap(), Token::Object(PdfObject::Int(70000)));
        assert_eq!(lexer.next_token().unwrap(), Token::Keyword("R".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Object(PdfObject::Ref(ObjRef { num: 42, gen: 65535 })));
        assert_eq!(lexer.next_token().unwrap(), Token::End);
    }
}

//#endregion 🧪️Tests
