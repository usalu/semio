//! 🔡️ Incremental lexical tokens for typed PDF containers.

use super::{buffers::{allocate, Growing}, Container, Turn};
use crate::{hex_val, is_delim, is_ws, PdfByteStringCursor, PdfDecimal, PdfObject, StringStep};

pub(super) const MAX_SCALAR_BYTES: usize = 256;

pub(super) enum Token {
    Value(PdfObject),
    Open(Container),
    Close(Container),
    Reference,
    Unexpected,
    End,
}

enum Scan {
    Idle,
    Comment,
    Angle(bool),
    Atom { start: usize, name: bool },
    Bytes { parser: PdfByteStringCursor, data: Growing<u8>, pending: Option<u8> },
}

pub(super) struct Reader {
    pub(super) position: usize,
    scan: Scan,
    atom: [u8; MAX_SCALAR_BYTES],
}

impl Reader {
    pub(super) fn new(position: usize) -> Self { Self { position, scan: Scan::Idle, atom: [0; MAX_SCALAR_BYTES] } }

    pub(super) fn step(&mut self, byte_at: &impl Fn(usize) -> Option<u8>, scalar_limit: usize, string_limit: usize, turn: &mut Turn) -> Result<Option<Token>, &'static str> {
        let byte = match self.scan {
            Scan::Angle(_) | Scan::Bytes { .. } => None,
            _ => byte_at(self.position),
        };
        match &mut self.scan {
            Scan::Idle => match byte {
                None => Ok(Some(Token::End)),
                Some(b'%') => { self.position += 1; self.scan = Scan::Comment; Ok(None) }
                Some(b) if is_ws(b) => { self.position += 1; Ok(None) }
                Some(b'[') => { self.position += 1; Ok(Some(Token::Open(Container::Array))) }
                Some(b']') => { self.position += 1; Ok(Some(Token::Close(Container::Array))) }
                Some(b'<') if self.position.checked_add(1).and_then(byte_at) == Some(b'<') => {
                    self.position += 1;
                    self.scan = Scan::Angle(true);
                    Ok(None)
                }
                Some(b'>') if self.position.checked_add(1).and_then(byte_at) == Some(b'>') => {
                    self.position += 1;
                    self.scan = Scan::Angle(false);
                    Ok(None)
                }
                Some(b'(' | b'<') => {
                    self.scan = Scan::Bytes { parser: PdfByteStringCursor::new(self.position), data: Growing::new(), pending: None };
                    Ok(None)
                }
                Some(b'/') => {
                    self.position += 1;
                    self.scan = Scan::Atom { start: self.position, name: true };
                    Ok(None)
                }
                Some(b) if is_delim(b) => Err("unexpected PDF COS delimiter"),
                Some(_) => {
                    self.scan = Scan::Atom { start: self.position, name: false };
                    Ok(None)
                }
            },
            Scan::Angle(open_or_close) => {
                let token = if *open_or_close { Token::Open(Container::Dict) } else { Token::Close(Container::Dict) };
                self.position += 1;
                self.scan = Scan::Idle;
                Ok(Some(token))
            }
            Scan::Comment => {
                if byte.is_some() { self.position += 1; }
                if matches!(byte, None | Some(b'\r' | b'\n')) { self.scan = Scan::Idle; }
                Ok(None)
            }
            Scan::Atom { start, name } => {
                let length = self.position - *start;
                if let Some(byte) = byte.filter(|&byte| !is_ws(byte) && !is_delim(byte)) {
                    if length >= scalar_limit { return Err("PDF COS scalar token limit exceeded"); }
                    self.atom[length] = byte;
                    self.position += 1;
                    return Ok(None);
                }
                let raw = &self.atom[..length];
                let token = if *name { parse_name(raw, turn)? } else { parse_regular(raw, turn)? };
                if token.is_some() { self.scan = Scan::Idle; }
                Ok(token)
            }
            Scan::Bytes { parser, data, pending } => {
                if let Some(byte) = *pending {
                    if data.prepare_push(string_limit, turn)? {
                        data.push(byte);
                        *pending = None;
                    }
                    return Ok(None);
                }
                let mut scratch = [0];
                let decoded = parser.advance_bytes(byte_at, &mut scratch, 1);
                self.position = parser.position();
                if decoded.written != 0 { *pending = Some(scratch[0]); }
                match decoded.status {
                    Err(reason) => Err(reason),
                    Ok(StringStep::Complete) if pending.is_none() => {
                        let Scan::Bytes { data, .. } = std::mem::replace(&mut self.scan, Scan::Idle) else { unreachable!() };
                        Ok(Some(Token::Value(PdfObject::Str(data.into_vec()))))
                    }
                    _ => Ok(None),
                }
            }
        }
    }

    pub(super) fn ends_reference_lookahead(&self, byte_at: &impl Fn(usize) -> Option<u8>) -> bool {
        matches!(self.scan, Scan::Idle) && byte_at(self.position).is_none_or(|byte| {
            !is_ws(byte) && !matches!(byte, b'%' | b'R' | b'+' | b'-' | b'0'..=b'9')
        })
    }

    pub(super) fn retired(&mut self, turn: &mut Turn) -> bool {
        if let Scan::Bytes { data, .. } = &mut self.scan {
            if !data.retire_empty(turn) { return false; }
        }
        self.scan = Scan::Idle;
        true
    }
}

fn parse_name(raw: &[u8], turn: &mut Turn) -> Result<Option<Token>, &'static str> {
    let mut decoded = [0; MAX_SCALAR_BYTES];
    let mut length = 0;
    let mut index = 0;
    while index < raw.len() {
        let mut byte = raw[index];
        if byte == b'#' && raw.get(index + 1..index + 3).is_some_and(|hex| hex.iter().all(u8::is_ascii_hexdigit)) {
            byte = (hex_val(raw[index + 1]) << 4) | hex_val(raw[index + 2]);
            index += 2;
        }
        decoded[length] = byte;
        length += 1;
        index += 1;
    }
    let decoded = &decoded[..length];
    let mut latin = [0; MAX_SCALAR_BYTES * 2];
    let checked = if let Ok(utf8) = std::str::from_utf8(decoded) {
        utf8
    } else {
        let mut length = 0;
        for &byte in decoded {
            length += char::from(byte).encode_utf8(&mut latin[length..]).len();
        }
        std::str::from_utf8(&latin[..length]).map_err(|_| "invalid PDF COS name encoding")?
    };
    Ok(own_string(checked.as_bytes(), turn)?.map(|name| Token::Value(PdfObject::Name(name))))
}

fn parse_regular(raw: &[u8], turn: &mut Turn) -> Result<Option<Token>, &'static str> {
    let literal = match raw {
        b"true" => Token::Value(PdfObject::Bool(true)),
        b"false" => Token::Value(PdfObject::Bool(false)),
        b"null" => Token::Value(PdfObject::Null),
        b"R" => Token::Reference,
        [b'+' | b'-' | b'.' | b'0'..=b'9', ..] => return parse_number(raw, turn),
        _ => Token::Unexpected,
    };
    Ok(Some(literal))
}

fn parse_number(raw: &[u8], turn: &mut Turn) -> Result<Option<Token>, &'static str> {
    let (negative, unsigned) = match raw {
        [b'-', rest @ ..] => (true, rest),
        [b'+', rest @ ..] => (false, rest),
        _ => (false, raw),
    };
    let dot = unsigned.iter().position(|&b| b == b'.');
    let (integer, fraction) = dot.map_or((unsigned, &[][..]), |index| (&unsigned[..index], &unsigned[index + 1..]));
    if (integer.is_empty() && fraction.is_empty()) || !integer.iter().chain(fraction).all(u8::is_ascii_digit) {
        return Err("invalid PDF COS number");
    }
    if dot.is_none() {
        if let Ok(value) = std::str::from_utf8(raw).map_err(|_| "invalid PDF COS integer")?.parse::<i64>() {
            return Ok(Some(Token::Value(PdfObject::Int(value))));
        }
    }
    let integer = if integer.is_empty() { b"0" } else { integer };
    let mut coefficient = [0; MAX_SCALAR_BYTES];
    coefficient[..integer.len()].copy_from_slice(integer);
    let length = integer.len() + fraction.len();
    coefficient[integer.len()..length].copy_from_slice(fraction);
    let Some(coefficient) = own_string(&coefficient[..length], turn)? else { return Ok(None); };
    Ok(Some(Token::Value(PdfObject::Real(PdfDecimal { negative, coefficient, scale: fraction.len() as u32 }))))
}

fn own_string(bytes: &[u8], turn: &mut Turn) -> Result<Option<String>, &'static str> {
    let Some(mut owned) = allocate(bytes.len(), turn)? else { return Ok(None); };
    owned.extend_from_slice(bytes);
    String::from_utf8(owned).map(Some).map_err(|_| "invalid PDF COS token encoding")
}
