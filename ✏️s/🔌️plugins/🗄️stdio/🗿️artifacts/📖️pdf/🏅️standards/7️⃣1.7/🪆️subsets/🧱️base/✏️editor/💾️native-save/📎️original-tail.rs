//! 📎️ Reads the cross-reference facts an incremental update continues from, straight out of the retained original bytes.

use semio_framework_value::ByteReadSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

const WINDOW: usize = 4096;
const MINIMUM: usize = 24;

/// 🚧️ Why the retained original cannot be continued with an appended section.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TailRefusal { Short, StartXref, Section, Trailer, Size, Root, Encrypted }

/// 📎️ Where the original's last cross-reference section starts, how large its table is and which kind it is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalTail {
    pub length: usize,
    pub startxref: usize,
    pub size: u32,
    pub root: ObjRef,
    pub stream: bool,
    pub ends_with_eol: bool,
}

impl OriginalTail {
    /// 🔭️ Reads the last 4 KiB for `startxref` and the trailer, and the first 4 KiB of the last section for its kind.
    pub fn read(source: &(impl ByteReadSource + ?Sized)) -> Result<Self, TailRefusal> {
        let length = source.byte_len();
        if length < MINIMUM { return Err(TailRefusal::Short); }
        let start = length.saturating_sub(WINDOW);
        let mut window = [0u8; WINDOW];
        let count = source.read_bytes(start, &mut window);
        let window = &window[..count];
        let marker = rfind(window, b"startxref").ok_or(TailRefusal::StartXref)?;
        let startxref = integer(&window[marker + 9..]).and_then(|value| usize::try_from(value).ok())
            .filter(|offset| *offset < length).ok_or(TailRefusal::StartXref)?;
        let ends_with_eol = matches!(window.last(), Some(b'\n' | b'\r'));
        let mut head = [0u8; WINDOW];
        let read = source.read_bytes(startxref, &mut head);
        let head = &head[..read];
        let lead = head.iter().take_while(|byte| byte.is_ascii_whitespace()).count();
        let (dictionary, stream) = if head[lead..].starts_with(b"xref") {
            let trailer = rfind(&window[..marker], b"trailer").ok_or(TailRefusal::Trailer)?;
            (&window[trailer + 7..marker], false)
        } else {
            let object = find(&head[lead..], b"obj").ok_or(TailRefusal::Section)?;
            let body = &head[lead + object + 3..];
            let end = find(body, b"stream").ok_or(TailRefusal::Section)?;
            if find(&body[..end], b"/XRef").is_none() { return Err(TailRefusal::Section); }
            (&body[..end], true)
        };
        if find(dictionary, b"/Encrypt").is_some() { return Err(TailRefusal::Encrypted); }
        let size = keyed_integer(dictionary, b"/Size").and_then(|value| u32::try_from(value).ok())
            .filter(|size| *size > 0).ok_or(TailRefusal::Size)?;
        let root = keyed_reference(dictionary, b"/Root").ok_or(TailRefusal::Root)?;
        Ok(Self { length, startxref, size, root, stream, ends_with_eol })
    }
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).rposition(|window| window == needle)
}

fn integer(bytes: &[u8]) -> Option<u64> {
    let lead = bytes.iter().take_while(|byte| byte.is_ascii_whitespace()).count();
    let digits = bytes[lead..].iter().take_while(|byte| byte.is_ascii_digit()).count();
    if digits == 0 || digits > 18 { return None; }
    std::str::from_utf8(&bytes[lead..lead + digits]).ok()?.parse().ok()
}

fn keyed(dictionary: &[u8], key: &[u8]) -> Option<usize> {
    dictionary.windows(key.len()).enumerate()
        .find(|(index, window)| *window == key && dictionary.get(index + key.len()).is_none_or(|next| !next.is_ascii_alphanumeric()))
        .map(|(index, _)| index + key.len())
}

fn keyed_integer(dictionary: &[u8], key: &[u8]) -> Option<u64> {
    integer(&dictionary[keyed(dictionary, key)?..])
}

fn keyed_reference(dictionary: &[u8], key: &[u8]) -> Option<ObjRef> {
    let mut rest = &dictionary[keyed(dictionary, key)?..];
    let mut take = |rest: &mut &[u8]| -> Option<u64> {
        let value = integer(rest)?;
        let lead = rest.iter().take_while(|byte| byte.is_ascii_whitespace()).count();
        *rest = &rest[lead + rest[lead..].iter().take_while(|byte| byte.is_ascii_digit()).count()..];
        Some(value)
    };
    let number = take(&mut rest)?;
    let generation = take(&mut rest)?;
    let marker = rest.iter().take_while(|byte| byte.is_ascii_whitespace()).count();
    (rest.get(marker) == Some(&b'R')).then_some(ObjRef { num: u32::try_from(number).ok()?, gen: u16::try_from(generation).ok()? })
}
