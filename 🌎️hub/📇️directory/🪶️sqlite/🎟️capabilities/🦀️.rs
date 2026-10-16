//! 🎟️ Share tokens and invites: the capability issuers and their atomic redemption.

use super::*;

//#region ShareTokens
impl ShareTokenDirectory for SqliteDirectory {
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let changed = tx
            .execute(
                "INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason)
             SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL
             FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id
             WHERE s.id = ?4 AND d.space_id = ?4 AND d.document_id = ?5",
                rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), scope.space_id, scope.document_id, issued.record.created_at, issued.record.expires_at],
            )
            .map_err(backend)?;
        if changed != 1 {
            return Err(DirectoryError::NotFound(format!("document descriptor {}/{}", scope.space_id, scope.document_id)));
        }
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn issue_share_token_as_with_admin_effect(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedShareToken> {
        let issued = match admin_effect_preflight(prepare_share_token(scope, ttl_secs, now_ms())) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let audit = match admin_effect_preflight(auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate) {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed = admin_effect_try!(
            tx,
            tx.execute(
                "INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason)
                 SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id
                 WHERE s.id = ?4 AND d.space_id = ?4 AND d.document_id = ?5",
                rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), scope.space_id, scope.document_id, issued.record.created_at, issued.record.expires_at],
            )
            .map_err(backend)
        );
        if changed != 1 {
            return admin_effect_rollback(tx);
        }
        admin_effect_try!(tx, insert_auth_audit(&tx, &audit));
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        match tx.commit() {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let changed = tx
            .execute("UPDATE hub_share_grant SET revoked_at = ?4, revoked_reason = ?5 WHERE id = ?1 AND space_id = ?2 AND document_id = ?3 AND revoked_at IS NULL", rusqlite::params![share_id, scope.space_id, scope.document_id, revoked_at, reason])
            .map_err(backend)?;
        if changed == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&tx, &audit)?;
            tx.commit().map_err(backend)?;
            Ok(())
        }
    }

    async fn revoke_share_token_as_with_admin_effect(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let audit = match admin_effect_preflight(auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        if admin_effect_try!(
            tx,
            tx.execute("UPDATE hub_share_grant SET revoked_at = ?4, revoked_reason = ?5 WHERE id = ?1 AND space_id = ?2 AND document_id = ?3 AND revoked_at IS NULL", rusqlite::params![share_id, scope.space_id, scope.document_id, revoked_at, reason])
                .map_err(backend)
        ) != 1
        {
            return admin_effect_rollback(tx);
        }
        admin_effect_try!(tx, insert_auth_audit(&tx, &audit));
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        match tx.commit() {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let record = self
            .lock()?
            .query_row(
                "SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2 AND selector = ?3",
                rusqlite::params![scope.space_id, scope.document_id, capability.selector()],
                share_token_row,
            )
            .optional()
            .map_err(backend)?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        let record = self
            .lock()?
            .query_row(
                "SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE id = ?1 AND selector = ?2 AND space_id = ?3 AND document_id = ?4",
                rusqlite::params![share_id, selector, scope.space_id, scope.document_id],
                share_token_row,
            )
            .optional()
            .map_err(backend)?;
        Ok(match record {
            None => SocketShareBindingStatus::Unavailable,
            Some(record) if record.revoked_at.is_some() => SocketShareBindingStatus::Revoked,
            Some(record) if record.expires_at <= now_ms => SocketShareBindingStatus::Expired,
            Some(record) => SocketShareBindingStatus::Active { expires_at_ms: record.expires_at },
        })
    }
}
//#endregion

//#region Invites
impl InviteDirectory for SqliteDirectory {
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, NULL, NULL)",
            rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), space_id, role.as_str(), issued.record.created_at, issued.record.expires_at],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn issue_invite_as_with_admin_effect(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedInvite> {
        let issued = match admin_effect_preflight(prepare_invite(space_id, role, ttl_secs, now_ms())) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let audit = match admin_effect_preflight(auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        admin_effect_try!(
            tx,
            tx.execute(
                "INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, NULL, NULL)",
                rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), space_id, role.as_str(), issued.record.created_at, issued.record.expires_at],
            )
            .map_err(backend)
        );
        admin_effect_try!(tx, insert_auth_audit(&tx, &audit));
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        match tx.commit() {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        let conn = self.lock()?;
        let record = conn.query_row(
            "SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE selector = ?1 AND EXISTS(SELECT 1 FROM hub_space WHERE id = hub_space_invite.space_id) AND EXISTS(SELECT 1 FROM hub_user WHERE id = ?2)",
            rusqlite::params![capability.selector(), user_id], invite_row,
        ).optional().map_err(backend)?;
        verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let accepted_at_ms = now_ms();
        let record = tx
            .query_row("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE selector = ?1", [capability.selector()], invite_row)
            .optional()
            .map_err(backend)?;
        let user_exists: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM hub_user WHERE id = ?1)", [user_id], |row| row.get(0)).map_err(backend)?;
        let space_kind: Option<String> = match record.as_ref() {
            Some(invite) => tx.query_row("SELECT kind FROM hub_space WHERE id = ?1", [&invite.space_id], |row| row.get(0)).optional().map_err(backend)?,
            None => None,
        };
        let space_state = InviteRedemptionSpaceStateV1::from_kind(space_kind.as_deref())?;
        match invite_redemption_preflight(record.as_ref(), capability, actor, user_id, user_exists == 1, space_state, accepted_at_ms) {
            InviteRedemptionPreflight::AlreadyCommitted => {
                let invite = record.as_ref().expect("committed preflight requires a record");
                let event = tx
                    .query_row("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE id = ?1", [invite.accepted_event_id.as_deref().unwrap_or_default()], event_row)
                    .optional()
                    .map_err(backend)?;
                let event = verify_invite_redemption_event(invite, event, user_id)?;
                tx.commit().map_err(backend)?;
                return Ok(InviteRedemptionCommit::AlreadyCommitted { event });
            }
            InviteRedemptionPreflight::Revoked => return Err(DirectoryError::Conflict("invite already revoked".into())),
            InviteRedemptionPreflight::Expired => return Err(DirectoryError::Conflict("invite expired".into())),
            InviteRedemptionPreflight::Denied => return Err(DirectoryError::Unauthorized),
            InviteRedemptionPreflight::Corrupt => return Err(DirectoryError::Backend("invite acceptance marker is incomplete".into())),
            InviteRedemptionPreflight::Claim => {}
        }
        let invite = record.expect("claim preflight requires a record");
        let event_id = time_ordered_id();
        let changed = tx
            .execute(
                "UPDATE hub_space_invite SET accepted_at = ?2, accepted_event_id = ?3 WHERE id = ?1 AND accepted_at IS NULL AND accepted_event_id IS NULL AND revoked_at IS NULL AND expires_at > ?2",
                rusqlite::params![invite.id, accepted_at_ms, event_id],
            )
            .map_err(backend)?;
        if changed != 1 {
            return Err(DirectoryError::Backend("SQLite invitation claim lost its immediate transaction fence".into()));
        }
        let event = NewDirectoryEvent {
            hlc,
            actor: actor.clone(),
            space_id: Some(invite.space_id.clone()),
            user_id: Some(user_id.to_string()),
            body: DirectoryEventBody::InviteRedeemed { space_id: invite.space_id, user_id: user_id.to_string(), invite_id: invite.id, role: role_to_wire(invite.role) },
        };
        let persisted = self.persist_event_with_identity(&tx, &event, event_id, accepted_at_ms)?;
        if let Err(error) = self.project(&tx, &persisted) {
            tx.rollback().map_err(backend)?;
            return Err(error);
        }
        tx.commit().map_err(backend)?;
        Ok(InviteRedemptionCommit::NewlyCommitted { event: persisted })
    }

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let changed =
            tx.execute("UPDATE hub_space_invite SET revoked_at = ?2, revoked_reason = ?3 WHERE space_id = ?4 AND id = ?1 AND revoked_at IS NULL AND accepted_at IS NULL", rusqlite::params![invite_id, revoked_at, reason, space_id]).map_err(backend)?;
        if changed == 0 {
            let accepted: Option<i64> = tx.query_row("SELECT accepted_at FROM hub_space_invite WHERE space_id = ?2 AND id = ?1", rusqlite::params![invite_id, space_id], |row| row.get(0)).optional().map_err(backend)?.flatten();
            return if accepted.is_some() { Err(DirectoryError::Conflict("invite already accepted".into())) } else { Err(DirectoryError::NotFound(format!("invite {invite_id}"))) };
        }
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(())
    }

    async fn revoke_invite_as_with_admin_effect(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let audit = match admin_effect_preflight(auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed = admin_effect_try!(
            tx,
            tx.execute("UPDATE hub_space_invite SET revoked_at = ?2, revoked_reason = ?3 WHERE space_id = ?4 AND id = ?1 AND revoked_at IS NULL AND accepted_at IS NULL", rusqlite::params![invite_id, revoked_at, reason, space_id]).map_err(backend)
        );
        if changed == 0 {
            return admin_effect_rollback(tx);
        }
        admin_effect_try!(tx, insert_auth_audit(&tx, &audit));
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        match tx.commit() {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let conn = self.lock()?;
        let mut stmt =
            conn.prepare("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE space_id = ?1 ORDER BY created_at DESC").map_err(backend)?;
        let rows = stmt.query_map([space_id], invite_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT id, role, created_at, expires_at, revoked_at IS NOT NULL, accepted_at IS NOT NULL
                 FROM hub_space_invite
                 WHERE space_id = ?1 AND (?2 IS NULL OR created_at < ?2 OR (created_at = ?2 AND id < ?3))
                 ORDER BY created_at DESC, id DESC LIMIT ?4",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![space_id, after.map(|(created_at, _)| created_at), after.map(|(_, invite_id)| invite_id), i64::try_from(limit).map_err(backend)?], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, i64>(2)?, row.get::<_, i64>(3)?, row.get::<_, bool>(4)?, row.get::<_, bool>(5)?))
            })
            .map_err(backend)?;
        rows.map(|row| {
            let (invite_id, role, created_at_ms, expires_at_ms, revoked, accepted) = row.map_err(backend)?;
            let role = SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored invite role is invalid".into()))?;
            Ok(SpaceAdministrationInviteRow { invite_id, role, created_at_ms, expires_at_ms, revoked, accepted })
        })
        .collect()
    }
}
//#endregion
