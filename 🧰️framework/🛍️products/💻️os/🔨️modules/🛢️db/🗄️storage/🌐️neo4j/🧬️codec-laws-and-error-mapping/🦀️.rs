//! 🧬️ Codec, laws and error mapping: driver byte codecs, the WAL and lease laws and the Neo4j error mapping. See [`super`].

use super::*;

//#region 🔖️Codec
pub(super) async fn write_driver_bytes(reservation: DbIoDriverReservation, bytes: BoltBytes, offset: usize, length: usize, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
    db_io_write_observed_bytes_range(reservation, bytes.value.to_vec(), offset, length, output)?.await
}

/// 🔢️ Neo4j's `Integer` bolt type is a signed 64-bit value; the family's identity/sequence
/// numbers are `u64`. Converts with an explicit range check rather than a silent wrapping `as i64`.
pub(super) fn u64_to_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
    i64::try_from(value).map_err(|_| DbError::InvalidArgument(format!("{what} exceeds neo4j's signed 64-bit integer range: {value}")))
}

/// 🔢️ Inverse of `u64_to_i64` for values read back from a Neo4j `Integer` property. A
/// negative value here means the property was corrupted (hand-edited or written by a bug) since
/// every writer path exclusively writes non-negative values.
pub(super) fn i64_to_u64(value: i64, what: &'static str) -> Result<u64, DbError> {
    u64::try_from(value).map_err(|_| DbError::Corrupt(format!("{what} decoded as a negative neo4j integer: {value}")))
}

/// ✂️ Slices `bytes[range.offset..range.offset+range.len]`, bounds-checked against
/// `bytes`'s actual length — the shared implementation `WalStorage::read` validates against,
/// mirroring `MemoryStorage`/`FsStorage`'s identical bounds-checking law.
pub(super) fn slice_range(bytes: &[u8], range: ByteRange) -> Result<&[u8], DbError> {
    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("read range overflows u64".to_string()))?;
    if end > bytes.len() as u64 {
        return Err(DbError::InvalidArgument(format!("read range {}..{end} out of bounds (len {})", range.offset, bytes.len())));
    }
    Ok(&bytes[range.offset as usize..end as usize])
}
//#endregion 🔖️Codec

//#region 🔖️WalLaws
#[cfg(test)]
pub(super) fn apply_append(current: &[u8], sealed: bool, extra: &[u8]) -> Result<Vec<u8>, DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot append to sealed wal segment".to_string()));
    }
    let mut updated = Vec::with_capacity(current.len() + extra.len());
    updated.extend_from_slice(current);
    updated.extend_from_slice(extra);
    Ok(updated)
}

#[cfg(test)]
pub(super) fn apply_truncate(current: &[u8], sealed: bool, new_len: u64) -> Result<Vec<u8>, DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
    }
    if new_len > current.len() as u64 {
        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
    }
    Ok(current[..new_len as usize].to_vec())
}
//#endregion 🔖️WalLaws

//#region 🔖️LeaseLaws
/// ⏳️ One lease row as read back from Neo4j: `(fence, expires_at_ms, holder)`.
pub(super) type LeaseRow = (EpochFence, u64, DbIoText);

/// 🤝️ The pure decision behind `LeaseStorage::acquire` — see `MemoryStorage::acquire`'s
/// identical law in `db_storage`, factored out here for unit testing without a live connection.
pub(super) fn decide_acquire_fence(resource: &str, existing: Option<LeaseRow>, holder: &str, now_ms: u64) -> Result<EpochFence, DbError> {
    match existing {
        Some((fence, expires_at_ms, existing_holder)) if now_ms < expires_at_ms => {
            if existing_holder.as_str() != holder {
                return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
            }
            Ok(fence)
        }
        Some((fence, _, _)) => Ok(fence.next()),
        None => Ok(EpochFence::INITIAL),
    }
}

/// ♻️ The pure decision behind `LeaseStorage::renew`.
pub(super) fn validate_renew(resource: &str, existing: Option<LeaseRow>, holder: &str, fence: EpochFence, now_ms: u64) -> Result<(), DbError> {
    let (current_fence, expires_at_ms, current_holder) = existing.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
    if now_ms >= expires_at_ms {
        return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
    }
    if current_holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
    }
    fence.check(current_fence)
}

/// 🕊️ The pure decision behind `LeaseStorage::release`.
pub(super) fn validate_release(resource: &str, existing: Option<LeaseRow>, holder: &str, fence: EpochFence) -> Result<(), DbError> {
    let (current_fence, _, current_holder) = existing.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
    if current_holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
    }
    fence.check(current_fence)
}
//#endregion 🔖️LeaseLaws

//#region 🔖️ErrorMapping
/// 🚨️ Maps a `neo4rs::Error` into the family's single `DbError` — never lets a foreign
/// error type leak through a public signature, per the repo's binding convention.
#[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
pub(super) fn map_neo4rs_error(err: neo4rs::Error) -> DbError {
    match &err {
        neo4rs::Error::IOError { .. } | neo4rs::Error::ConnectionError => DbError::Unavailable(err.to_string()),
        neo4rs::Error::AuthenticationError(_) => DbError::Unauthorized(err.to_string()),
        neo4rs::Error::Neo4j(neo4j_err) => map_neo4j_error(neo4j_err),
        _ => DbError::Internal(err.to_string()),
    }
}

/// 🏷️ Classifies a server-reported `Neo4jError` by its `Neo4jErrorKind` (the driver's own
/// status-code classification) into the family's `DbError`.
// 🚫️async: E1 pure accessor called from `map_neo4rs_error`, itself an E4 fn-pointer slot — see R9
fn map_neo4j_error(err: &neo4rs::Neo4jError) -> DbError {
    use neo4rs::{Neo4jClientErrorKind, Neo4jErrorKind};
    match err.kind() {
        Neo4jErrorKind::Client(Neo4jClientErrorKind::Security(_)) => DbError::Unauthorized(err.message().to_string()),
        Neo4jErrorKind::Client(Neo4jClientErrorKind::TransactionTerminated) => DbError::Conflict(err.message().to_string()),
        Neo4jErrorKind::Transient => DbError::Unavailable(err.message().to_string()),
        _ => DbError::Internal(format!("{}: {}", err.code(), err.message())),
    }
}

/// 🚨️ Maps a row-decoding error (a property missing or of the wrong bolt type) into
/// `DbError::Corrupt` — a decode failure always means the stored data doesn't match this crate's
/// own schema, never a caller mistake.
#[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
pub(super) fn map_de_error(err: neo4rs::DeError) -> DbError {
    DbError::Corrupt(format!("neo4j row decode error: {err}"))
}
//#endregion 🔖️ErrorMapping
