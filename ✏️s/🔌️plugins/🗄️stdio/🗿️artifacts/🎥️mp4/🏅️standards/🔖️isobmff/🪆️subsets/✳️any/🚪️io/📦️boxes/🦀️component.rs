//! 📦️ ISO-BMFF box primitives — `ByteReader`, `FourCc`, the box iterator, and the box-framing
//! writer. Moved from remodel's video engine (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs`
//! lines 12-236, 3561-3567) per the master plan's extraction map — box header framing, size
//! resolution (32/64-bit, "extends to end") and sibling iteration are byte-identical in spirit
//! to that source, adapted to this artifact's own error type (`BoxError`) instead of remodel's
//! `VideoError`. <https://www.iso.org/standard/74428.html> (ISO/IEC 14496-12)

//#region 🔖️Bytes
/// 🧭️ Four-character box code, compared/hashed by raw bytes (moved from remodel's `FourCc`).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCc(pub [u8; 4]);

impl FourCc {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(bytes: &[u8; 4]) -> Self {
        Self(*bytes)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.0)
    }
}

impl std::fmt::Debug for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FourCc({:?})", self.as_str())
    }
}

/// ⚠️ Box-walk error — malformed/truncated ISO-BMFF input only, never a panic on untrusted bytes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoxError {
    Truncated,
    Bad(&'static str),
}

impl std::fmt::Display for BoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "mp4: truncated box stream"),
            Self::Bad(msg) => write!(f, "mp4: malformed box: {msg}"),
        }
    }
}
impl std::error::Error for BoxError {}

/// 📖️ Bounds-checked big-endian cursor (moved from remodel's `ByteReader`, ISO-BMFF-only fields).
pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pos(&self) -> usize {
        self.pos
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], BoxError> {
        let end = self.pos.checked_add(n).ok_or(BoxError::Truncated)?;
        let slice = self.data.get(self.pos..end).ok_or(BoxError::Truncated)?;
        self.pos = end;
        Ok(slice)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn skip(&mut self, n: usize) -> Result<(), BoxError> {
        self.take(n).map(|_| ())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn u8(&mut self) -> Result<u8, BoxError> {
        Ok(self.take(1)?[0])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn u16_be(&mut self) -> Result<u16, BoxError> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn u32_be(&mut self) -> Result<u32, BoxError> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn u64_be(&mut self) -> Result<u64, BoxError> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap()))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn i32_be(&mut self) -> Result<i32, BoxError> {
        Ok(i32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fourcc(&mut self) -> Result<FourCc, BoxError> {
        Ok(FourCc(self.take(4)?.try_into().unwrap()))
    }
}
//#endregion 🔖️Bytes

//#region 🔖️Iter
/// 📦️ One parsed ISO-BMFF box: type + payload (bytes after the size/type header).
pub struct Mp4BoxRef<'a> {
    pub kind: FourCc,
    pub payload: &'a [u8],
}

/// 🚶️ Iterates sibling boxes at one level (moved from remodel's `Mp4BoxIter`/`iter_boxes`):
/// honors the 32-bit inline size, the 64-bit `largesize` extension (`size == 1`), and the
/// "extends to end of data" convention (`size == 0`).
pub struct Mp4BoxIter<'a> {
    data: &'a [u8],
    pos: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn iter_boxes(data: &[u8]) -> Mp4BoxIter<'_> {
    Mp4BoxIter { data, pos: 0 }
}

impl<'a> Iterator for Mp4BoxIter<'a> {
    type Item = Result<Mp4BoxRef<'a>, BoxError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let mut r = ByteReader::new(&self.data[self.pos..]);
        let size32 = match r.u32_be() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let kind = match r.fourcc() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let (box_len, header_len) = if size32 == 1 {
            match r.u64_be() {
                Ok(v) => (v as usize, 16usize),
                Err(e) => return Some(Err(e)),
            }
        } else if size32 == 0 {
            (self.data.len() - self.pos, 8usize)
        } else {
            (size32 as usize, 8usize)
        };
        if box_len < header_len {
            return Some(Err(BoxError::Bad("box size smaller than its own header")));
        }
        let box_end = match self.pos.checked_add(box_len) {
            Some(v) if v <= self.data.len() => v,
            _ => return Some(Err(BoxError::Truncated)),
        };
        let payload = &self.data[self.pos + header_len..box_end];
        self.pos = box_end;
        Some(Ok(Mp4BoxRef { kind, payload }))
    }
}

/// 🔍️ First direct-child box of the given type (moved from remodel's `find_box`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_box<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Option<&'a [u8]>, BoxError> {
    for item in iter_boxes(data) {
        let b = item?;
        if b.kind.0 == *want {
            return Ok(Some(b.payload));
        }
    }
    Ok(None)
}

/// 🔍️ Every direct-child box of the given type, in document order (moved from `find_boxes`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_boxes<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Vec<&'a [u8]>, BoxError> {
    let mut out = Vec::new();
    for item in iter_boxes(data) {
        let b = item?;
        if b.kind.0 == *want {
            out.push(b.payload);
        }
    }
    Ok(out)
}

/// 🔍️ Like [`find_box`] but a missing box is itself the error (moved from `require_box`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn require_box<'a>(data: &'a [u8], want: &[u8; 4], ctx: &'static str) -> Result<&'a [u8], BoxError> {
    find_box(data, want)?.ok_or(BoxError::Bad(ctx))
}
//#endregion 🔖️Iter

//#region 🔖️Write
/// ✍️ Frames `payload` under `fourcc` with a standard 32-bit box header (moved from remodel's
/// `mp4_box`). Every box this engine writes fits in a 32-bit size (real fixtures are far under
/// 4GiB); 64-bit `largesize` framing is a read-side-only concern here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_box(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len());
    out.extend_from_slice(&((payload.len() + 8) as u32).to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(payload);
    out
}
//#endregion 🔖️Write

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn iterates_two_sibling_boxes_and_resolves_sizes() {
        let ftyp = write_box(b"ftyp", b"isom");
        let free = write_box(b"free", &[0, 0]);
        let mut bytes = ftyp.clone();
        bytes.extend_from_slice(&free);
        let boxes: Vec<_> = iter_boxes(&bytes).collect::<Result<Vec<_>, _>>().expect("iterate");
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0].kind.0, *b"ftyp");
        assert_eq!(boxes[0].payload, b"isom");
        assert_eq!(boxes[1].kind.0, *b"free");
        assert_eq!(find_box(&bytes, b"free").unwrap(), Some(&[0u8, 0][..]));
        assert_eq!(find_box(&bytes, b"nope").unwrap(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn require_box_errors_when_absent() {
        let bytes = write_box(b"ftyp", b"isom");
        assert!(require_box(&bytes, b"moov", "missing moov").is_err());
    }
}
