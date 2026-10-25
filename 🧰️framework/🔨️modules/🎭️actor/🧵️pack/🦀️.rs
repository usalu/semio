//! 🧵️ Actor binary primitives own the exact shared wire tags, lengths and error vocabulary.

/// 🚨️ The one error type every `pack_decode` returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackError {
    Truncated(usize, &'static str),
    InvalidTag { what: &'static str, tag: u8, offset: usize },
    InvalidUtf8(&'static str, usize),
    OverlongVarint(usize),
    InvalidLifecycle(&'static str),
    InvalidUiPatchReceipt(&'static str),
    InvalidColdPair(&'static str),
    InvalidRetainedTurn(&'static str),
}

impl std::fmt::Display for PackError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated(offset, what) => write!(formatter, "pack: truncated at offset {offset} reading {what}"),
            Self::InvalidTag { what, tag, offset } => write!(formatter, "pack: invalid tag {tag} for {what} at offset {offset}"),
            Self::InvalidUtf8(what, offset) => write!(formatter, "pack: invalid utf8 in {what} at offset {offset}"),
            Self::OverlongVarint(offset) => write!(formatter, "pack: overlong varint at offset {offset}"),
            Self::InvalidRetainedTurn(reason) => write!(formatter, "pack: invalid original retained turn: {reason}"),
            Self::InvalidLifecycle(reason) => write!(formatter, "pack: invalid instance lifecycle: {reason}"),
            Self::InvalidUiPatchReceipt(reason) => write!(formatter, "pack: invalid issued UI patch receipt: {reason}"),
            Self::InvalidColdPair(reason) => write!(formatter, "pack: invalid cold artifact pair status: {reason}"),
        }
    }
}

impl std::error::Error for PackError {}

pub async fn write_u8(out: &mut Vec<u8>, v: u8) {
    out.push(v);
}
pub async fn read_u8(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u8, PackError> {
    let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
    *pos += 1;
    Ok(byte)
}

pub async fn write_bool(out: &mut Vec<u8>, v: bool) {
    out.push(v as u8);
}
pub async fn read_bool(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<bool, PackError> {
    Ok(read_u8(bytes, pos, what).await? != 0)
}

pub async fn write_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub async fn read_u16(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u16, PackError> {
    let end = *pos + 2;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(u16::from_le_bytes(slice.try_into().expect("2 bytes")))
}

pub async fn write_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub async fn read_u32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u32, PackError> {
    let end = *pos + 4;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(u32::from_le_bytes(slice.try_into().expect("4 bytes")))
}

pub async fn write_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub async fn read_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
    let end = *pos + 8;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(u64::from_le_bytes(slice.try_into().expect("8 bytes")))
}

pub async fn write_f32(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub async fn read_f32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<f32, PackError> {
    let end = *pos + 4;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(f32::from_le_bytes(slice.try_into().expect("4 bytes")))
}

pub async fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    let mut remaining = value;
    loop {
        let byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}
pub async fn read_varint_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
    let start = *pos;
    let mut result: u64 = 0;
    for i in 0..10usize {
        let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
        *pos += 1;
        let more = byte & 0x80 != 0;
        let payload = (byte & 0x7F) as u64;
        if i == 9 && (more || payload > 1) {
            return Err(PackError::OverlongVarint(start));
        }
        result |= payload << (i as u32 * 7);
        if !more {
            return Ok(result);
        }
    }
    Err(PackError::OverlongVarint(start))
}

pub async fn write_bytes(out: &mut Vec<u8>, v: &[u8]) {
    write_varint_u64(out, v.len() as u64).await;
    out.extend_from_slice(v);
}
pub async fn read_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Vec<u8>, PackError> {
    let len = read_varint_u64(bytes, pos, what).await? as usize;
    let end = *pos + len;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(slice.to_vec())
}

pub async fn write_str(out: &mut Vec<u8>, v: &str) {
    write_bytes(out, v.as_bytes()).await;
}
pub async fn read_str(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<String, PackError> {
    let start = *pos;
    let raw = read_bytes(bytes, pos, what).await?;
    String::from_utf8(raw).map_err(|_| PackError::InvalidUtf8(what, start))
}

pub async fn write_hash32(out: &mut Vec<u8>, v: &[u8; 32]) {
    out.extend_from_slice(v);
}
pub async fn read_hash32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<[u8; 32], PackError> {
    let end = *pos + 32;
    let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
    *pos = end;
    Ok(slice.try_into().expect("32 bytes"))
}

/// 🪡 Concrete (non-generic) `Option<Vec<u8>>` combinators — a closure-based generic
/// `write_opt`/`read_opt` hits rustc's "implementation of `AsyncFnOnce` is not general enough"
/// (the returned per-call future can't be proven to outlive a HRTB-quantified borrow of `out`/
/// `bytes`, a known async-closures rough edge). Concrete types sidestep it entirely; every
/// `Option<Vec<u8>>` field in this file's wire format goes through these two.
pub async fn write_opt_bytes(out: &mut Vec<u8>, v: &Option<Vec<u8>>) {
    write_bool(out, v.is_some()).await;
    if let Some(x) = v {
        write_bytes(out, x).await;
    }
}
pub async fn read_opt_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<Vec<u8>>, PackError> {
    if read_bool(bytes, pos, what).await? { Ok(Some(read_bytes(bytes, pos, what).await?)) } else { Ok(None) }
}

/// ⏱️ Same rationale as `write_opt_bytes`/`read_opt_bytes`, for `Option<u64>` (deadlines/wake times).
pub async fn write_opt_u64(out: &mut Vec<u8>, v: &Option<u64>) {
    write_bool(out, v.is_some()).await;
    if let Some(x) = v {
        write_u64(out, *x).await;
    }
}
pub async fn read_opt_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<u64>, PackError> {
    if read_bool(bytes, pos, what).await? { Ok(Some(read_u64(bytes, pos, what).await?)) } else { Ok(None) }
}

/// 📦️ `f` is a bare async fn item (`Type::pack_encode`), never a closure — fn items are
/// automatically higher-ranked over their argument lifetimes, so they don't hit the
/// `write_opt_bytes` doc's HRTB rough edge. `(item, out)` order (not `(out, item)`) is what
/// lets `T::pack_encode`'s own `(&self, out)` signature line up without an adapter closure.
pub async fn write_vec<T>(out: &mut Vec<u8>, v: &[T], mut f: impl AsyncFnMut(&T, &mut Vec<u8>)) {
    write_varint_u64(out, v.len() as u64).await;
    for item in v {
        f(item, out).await;
    }
}
pub async fn read_vec<T>(bytes: &[u8], pos: &mut usize, what: &'static str, mut f: impl AsyncFnMut(&[u8], &mut usize) -> Result<T, PackError>) -> Result<Vec<T>, PackError> {
    let len = read_varint_u64(bytes, pos, what).await? as usize;
    let mut out = Vec::with_capacity(len.min(1 << 20));
    for _ in 0..len {
        out.push(f(bytes, pos).await?);
    }
    Ok(out)
}
