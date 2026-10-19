//! 🪪️ Canonical, bounded interaction addresses over ordered UTF-8 identity segments.

pub const SCOPED_IDENTITY_MAX_BYTES: usize = 65_536;
pub const SCOPED_IDENTITY_MAX_SEGMENTS: usize = 32;
const PREFIX: &str = "scope:1:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopedIdentityError { InvalidSegments, LimitExceeded, Malformed }

impl std::fmt::Display for ScopedIdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "scoped identity {self:?}") }
}

impl std::error::Error for ScopedIdentityError {}

/// 🧩️ Encodes ordered nonempty segments without delimiter escaping or normalization.
pub fn validate_scoped_identity_segments(segments: &[&str]) -> Result<usize, ScopedIdentityError> {
    if segments.is_empty() || segments.len() > SCOPED_IDENTITY_MAX_SEGMENTS || segments.iter().any(|segment| segment.is_empty()) { return Err(ScopedIdentityError::InvalidSegments); }
    let bytes = segments.iter().try_fold(PREFIX.len(), |bytes, segment| bytes.checked_add(segment.len()).and_then(|bytes| bytes.checked_add(segment.len().ilog10() as usize + 2))).ok_or(ScopedIdentityError::LimitExceeded)?;
    if bytes > SCOPED_IDENTITY_MAX_BYTES { return Err(ScopedIdentityError::LimitExceeded); }
    Ok(bytes)
}

/// 🧩️ Encodes admitted segments with one exact output allocation.
pub fn encode_scoped_identity(segments: &[&str]) -> Result<String, ScopedIdentityError> {
    let bytes = validate_scoped_identity_segments(segments)?;
    let mut encoded = String::with_capacity(bytes);
    encoded.push_str(PREFIX);
    for segment in segments { std::fmt::Write::write_fmt(&mut encoded, format_args!("{}:", segment.len())).map_err(|_| ScopedIdentityError::Malformed)?; encoded.push_str(segment); }
    Ok(encoded)
}

/// 🔎️ Decodes an exact canonical address, borrowing each segment from the original identity.
pub fn decode_scoped_identity(identity: &str) -> Result<Vec<&str>, ScopedIdentityError> {
    let (segments, count) = scoped_identity_segments(identity)?;
    Ok(segments[..count].to_vec())
}

/// 👁️ Visits admitted original segments without heap allocation or callbacks on malformed input.
pub fn visit_scoped_identity_segments<'a>(identity: &'a str, mut visitor: impl FnMut(&'a str)) -> Result<usize, ScopedIdentityError> {
    let (segments, count) = scoped_identity_segments(identity)?;
    for segment in &segments[..count] { visitor(*segment); }
    Ok(count)
}

fn scoped_identity_segments(identity: &str) -> Result<([&str; SCOPED_IDENTITY_MAX_SEGMENTS], usize), ScopedIdentityError> {
    if identity.len() > SCOPED_IDENTITY_MAX_BYTES { return Err(ScopedIdentityError::LimitExceeded); }
    let mut tail = identity.strip_prefix(PREFIX).ok_or(ScopedIdentityError::Malformed)?;
    let mut segments = [""; SCOPED_IDENTITY_MAX_SEGMENTS];
    let mut count = 0;
    while !tail.is_empty() {
        if count == SCOPED_IDENTITY_MAX_SEGMENTS { return Err(ScopedIdentityError::LimitExceeded); }
        let (length, body) = tail.split_once(':').ok_or(ScopedIdentityError::Malformed)?;
        if length.is_empty() || length.starts_with('0') || !length.bytes().all(|byte| byte.is_ascii_digit()) { return Err(ScopedIdentityError::Malformed); }
        let length = length.parse::<usize>().map_err(|_| ScopedIdentityError::Malformed)?;
        segments[count] = body.get(..length).ok_or(ScopedIdentityError::Malformed)?;
        tail = body.get(length..).ok_or(ScopedIdentityError::Malformed)?;
        count += 1;
    }
    if count == 0 { return Err(ScopedIdentityError::Malformed); }
    Ok((segments, count))
}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
