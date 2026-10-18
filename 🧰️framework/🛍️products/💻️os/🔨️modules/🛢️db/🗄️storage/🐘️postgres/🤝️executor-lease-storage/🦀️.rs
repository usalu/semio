//! 🤝️ Postgres executor lease storage: the existing lease row, the acquire, renew and release decisions and the lease storage operations of the executor. See [`super`].

use super::*;

//#region 🔖️LeaseStorage
/// ⏳️ The row currently held on a resource, as read from `db_lease` — the DB-shaped input to
/// the pure `lease_*_decision`/`lease_*_check` functions below, so the hand-off/renew/release state
/// machine is unit-testable without a live Postgres connection.
pub(super) struct ExistingLease {
    pub(super) holder: DbIoText,
    pub(super) fence: EpochFence,
    pub(super) expires_at_ms: u64,
}

/// 🤝️ Pure decision for `LeaseStorage::acquire` — identical state machine to
/// `db_storage::{MemoryStorage, FsStorage}::acquire`: re-acquire by the same still-live holder keeps
/// the fence, a genuine hand-off (absent or expired) bumps it, a live foreign holder conflicts.
pub(super) fn lease_acquire_decision(existing: Option<&ExistingLease>, holder: &str, now_ms: u64) -> Result<EpochFence, DbError> {
    match existing {
        Some(info) if now_ms < info.expires_at_ms => {
            if info.holder.as_str() != holder {
                return Err(DbError::Conflict(format!("resource is leased by another holder ({})", info.holder.as_str())));
            }
            Ok(info.fence)
        }
        Some(info) => Ok(info.fence.next()),
        None => Ok(EpochFence::INITIAL),
    }
}

/// ♻️ Pure decision for `LeaseStorage::renew` — errors precisely as documented on the trait:
/// `NotFound` absent, `Unavailable` expired, `Unauthorized` wrong holder, `Fenced` wrong epoch.
pub(super) fn lease_renew_check(existing: Option<&ExistingLease>, holder: &str, fence: EpochFence, now_ms: u64) -> Result<(), DbError> {
    let info = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
    if now_ms >= info.expires_at_ms {
        return Err(DbError::Unavailable("lease already expired".to_string()));
    }
    if info.holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// 🕊️ Pure decision for `LeaseStorage::release` — same holder/fence checks as `renew`, minus
/// the expiry check (a holder may release its own already-expired-but-not-yet-reclaimed lease).
pub(super) fn lease_release_check(existing: Option<&ExistingLease>, holder: &str, fence: EpochFence) -> Result<(), DbError> {
    let info = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
    if info.holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// 🔒️ Reads `resource`'s current lease row through `executor`, taking a `FOR UPDATE` row
/// lock when `executor` is a transaction (the row-lock variant every `acquire`/`renew`/`release`
/// call uses so the read-decide-write sequence is atomic across concurrent connections) —
/// `LeaseStorage::current` instead calls this with the bare pool for a non-locking snapshot read.
async fn read_existing_lease_for_update<'e, E>(operation: u64, executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1 FOR UPDATE").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    let Some((holder, epoch, expires_at_ms)) = row else {
        reservation.close_step()?;
        return Ok(None);
    };
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder).await?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

async fn read_existing_lease<'e, E>(operation: u64, executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    let Some((holder, epoch, expires_at_ms)) = row else {
        reservation.close_step()?;
        return Ok(None);
    };
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder).await?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

impl LeaseStorage for PostgresDbIoExecutor {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        let fence = lease_acquire_decision(existing.as_ref(), holder, now_ms)?;
        let expires_at = to_i64(now_ms.checked_add(ttl_ms).ok_or(DbError::LimitExceeded("PostgreSQL lease expiry"))?)?;
        sqlx::query(
            "INSERT INTO db_lease (resource, holder, epoch, expires_at_ms) VALUES ($1, $2, $3, $4)
                 ON CONFLICT (resource) DO UPDATE SET holder = EXCLUDED.holder, epoch = EXCLUDED.epoch, expires_at_ms = EXCLUDED.expires_at_ms",
        )
        .bind(resource)
        .bind(holder)
        .bind(to_i64(fence.epoch)?)
        .bind(expires_at)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(fence)
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        lease_renew_check(existing.as_ref(), holder, fence, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or(DbError::LimitExceeded("PostgreSQL lease expiry"))?;
        sqlx::query("UPDATE db_lease SET expires_at_ms = $1 WHERE resource = $2").bind(to_i64(expires_at_ms)?).bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        lease_release_check(existing.as_ref(), holder, fence)?;
        sqlx::query("DELETE FROM db_lease WHERE resource = $1").bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let existing = read_existing_lease(self.active_operation, &self.pool, resource).await?;
        existing.filter(|info| now_ms < info.expires_at_ms).map(|info| Ok(DbIoLeaseResult::new(DbIoText::try_from_str(resource)?, info.holder, info.fence, info.expires_at_ms))).transpose()
    }
}
//#endregion 🔖️LeaseStorage
