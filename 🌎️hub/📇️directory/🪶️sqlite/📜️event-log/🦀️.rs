//! 📜️ The append-only directory event log and projection rebuild.

use super::*;

//#region EventLog
impl EventLog for SqliteDirectory {
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return Err(DirectoryError::Conflict("event requires its verified authority append seam".into()));
        }
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            if let Some(reason) = self.projection_rejection(&tx, &event.body)? {
                tx.rollback().map_err(backend)?;
                return Ok(DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason));
            }
            let full = self.persist_event(&tx, event)?;
            self.project(&tx, &full)?;
            self.append_cas_release(&tx, &full)?;
            persisted.push(full);
        }
        tx.commit().map_err(backend)?;
        #[cfg(test)]
        if self.append_commit_ack_failure.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Backend("injected uncertain append commit acknowledgement".into()));
        }
        Ok(DirectoryAppendOutcomeV1::Appended(persisted))
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        if events.is_empty() || events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return AdminEffectCommitV1::RejectedBeforeCommit;
        }
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate) {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            if admin_effect_try!(tx, self.projection_rejection(&tx, &event.body)).is_some() {
                return admin_effect_rollback(tx);
            }
            let full = admin_effect_try!(tx, self.persist_event(&tx, event));
            admin_effect_try!(tx, self.project(&tx, &full));
            admin_effect_try!(tx, self.append_cas_release(&tx, &full));
            persisted.push(full);
        }
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &persisted));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        if tx.commit().is_err() {
            return AdminEffectCommitV1::Indeterminate;
        }
        #[cfg(test)]
        if self.append_commit_ack_failure.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return AdminEffectCommitV1::Indeterminate;
        }
        AdminEffectCommitV1::Applied(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        #[cfg(test)]
        if self.event_read_failure.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Backend("injected directory event read failure".into()));
        }
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > ?1 ORDER BY seq LIMIT ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![since_seq, limit], event_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let seq: Option<i64> = self.lock()?.query_row("SELECT MAX(seq) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        u64::try_from(seq.unwrap_or(0)).map_err(backend)
    }
}
//#endregion
