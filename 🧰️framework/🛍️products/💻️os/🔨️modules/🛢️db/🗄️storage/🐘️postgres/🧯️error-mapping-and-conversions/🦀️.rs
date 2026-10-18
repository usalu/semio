//! 🧯️ Postgres error mapping and conversions: the sqlx and create error mapping, the integer and ascending id conversions and the read range and truncate validation. See [`super`].

use super::*;

//#region 🔖️ErrorMapping
/// 🚨️ Maps a `sqlx::Error` to this family's `DbError` — the only place `sqlx::Error` is
/// allowed to appear, mirroring `db_storage::FsStorage`'s single `io_err` chokepoint for
/// `std::io::Error`. A `Database`-flavored error further classifies via
/// `DatabaseError::is_unique_violation` (driver-agnostic, no hand-parsed SQLSTATE string).
#[allow(clippy::needless_pass_by_value)] // used directly as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
pub(super) fn map_sqlx_error(err: sqlx::Error) -> DbError {
    match &err {
        sqlx::Error::RowNotFound => return DbError::NotFound("row not found".to_string()),
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut | sqlx::Error::WorkerCrashed => {
            return DbError::Unavailable(err.to_string());
        }
        sqlx::Error::Configuration(_) | sqlx::Error::Protocol(_) | sqlx::Error::InvalidArgument(_) => {
            return DbError::InvalidArgument(err.to_string());
        }
        _ => {}
    }
    if let Some(db_err) = err.as_database_error() {
        if db_err.is_unique_violation() {
            return DbError::AlreadyExists(db_err.message().to_string());
        }
        return DbError::Io(db_err.message().to_string());
    }
    DbError::Io(err.to_string())
}

/// 🆕️ Like `map_session_error`, but a unique-violation becomes `DbError::AlreadyExists(what())`
/// — used by the writer session's `create_segment`.
// 🚫️async: E1 pure accessor called from sync `.map_err(|err| map_create_error(...))` closures — see R9
pub(super) fn map_create_error(err: sqlx::Error, what: impl FnOnce() -> String) -> DbError {
    if let Some(db_err) = err.as_database_error() {
        if db_err.is_unique_violation() {
            return DbError::AlreadyExists(what());
        }
    }
    map_session_error(err)
}
//#endregion 🔖️ErrorMapping

//#region 🔖️Conversions
/// 🔢️ Every dense index/generation/run/epoch/timestamp this crate stores is `u64` at the
/// trait boundary but `BIGINT` (`i64`) in Postgres — this is the one narrowing conversion point,
/// erroring rather than silently wrapping on the (astronomically unlikely) values above
/// `i64::MAX`.
pub(super) fn to_i64(value: u64) -> Result<i64, DbError> {
    i64::try_from(value).map_err(|_| DbError::InvalidArgument(format!("value {value} exceeds i64::MAX")))
}

/// 📋️ One document's ascending id column in ONE round trip, bounded by the DB I/O list capacity (`sql` binds
/// the document as `$1` and the row bound as `$2`; a list one row over the capacity is refused exactly as pushing it
/// would be). A list used to cost one round trip per row (`… > $2 ORDER BY … LIMIT 1` in a loop): the index lists its
/// runs several times per edit, so every edit on a remote server paid hundreds of round trips and outgrew the hub's
/// 30 s frame deadline under load (ticket 26/09/23 H9 session 12, two-client e2e `tc19`–`tc22`).
pub(super) async fn postgres_ascending_ids(pool: &PgPool, sql: &'static str, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
    let bound = to_i64(DB_IO_LIST_ITEMS as u64 + 1)?;
    let rows: Vec<(i64,)> = sqlx::query_as(sql).bind(document.0.as_str()).bind(bound).fetch_all(pool).await.map_err(map_sqlx_error)?;
    let mut result = DbIoU64List::new();
    for (id,) in rows {
        result.push(u64::try_from(id).map_err(|_| DbError::InvalidArgument(format!("stored id {id} is negative")))?)?;
    }
    Ok(result)
}

/// ✂️ Validates a `WalStorage::read` range against the segment's actual current length
/// (already fetched via `octet_length`, so this never touches the segment bytes themselves) and
/// converts to the 1-indexed `(offset, len)` pair Postgres's `substring(bytea, int, int)` expects.
pub(super) fn validate_read_range(current_len: u64, range: ByteRange) -> Result<(i64, i64), DbError> {
    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("read range overflows u64".to_string()))?;
    if end > current_len {
        return Err(DbError::InvalidArgument(format!("read range {}..{end} out of bounds (len {current_len})", range.offset)));
    }
    Ok((to_i64(range.offset)?, to_i64(range.len)?))
}

/// ✂️ Validates a `WalStorage::truncate_tail` request against the segment's sealed flag and
/// current length, matching `db_storage::{MemoryStorage, FsStorage}`'s identical checks.
pub(super) fn validate_truncate(sealed: bool, current_len: u64, new_len: u64) -> Result<(), DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
    }
    if new_len > current_len {
        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
    }
    Ok(())
}
//#endregion 🔖️Conversions
