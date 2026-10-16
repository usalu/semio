//! 👤️ Users, auth sessions, credentials and agent delegations.

use super::*;

pub(super) fn insert_auth_audit(conn: &Connection, event: &AuthAuditRecord) -> DirectoryResult<()> {
    conn.execute(
        "INSERT INTO hub_auth_audit (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![event.id, event.occurred_at, event.event_kind, event.auth_session_id, event.target_user_id, event.actor_user_id, event.provider, event.outcome_code, event.reason_code, event.correlation_id, event.peer_class],
    )
    .map_err(backend)?;
    Ok(())
}

impl SqliteDirectory {
    fn revoke_auth_sessions_matching(
        &self,
        predicate: &str,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        admin_effect: Option<&NewAdminOperationEffectReceiptV1>,
    ) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let rows: Vec<(String, String, i64, String)> = {
            let sql = format!("SELECT id, user_id, authorization_generation, identity_provider FROM hub_auth_session WHERE {predicate} AND revoked_at IS NULL ORDER BY id");
            let mut statement = tx.prepare(&sql).map_err(backend)?;
            if let Some(digest) = subject_digest {
                statement.query_map(rusqlite::params![key, digest.as_slice()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(backend)?.collect::<Result<Vec<_>, _>>().map_err(backend)?
            } else {
                statement.query_map([key], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(backend)?.collect::<Result<Vec<_>, _>>().map_err(backend)?
            }
        };
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let next_generation = generation.checked_add(1).ok_or_else(|| DirectoryError::Conflict("authorization generation overflow".into()))?;
            tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![id, revoked_at, reason, next_generation]).map_err(backend)?;
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&tx, &audit)?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(next_generation).map_err(backend)?, revoked_at });
        }
        if let Some(effect) = admin_effect {
            insert_admin_operation_effect_receipt(&tx, &admin_operation_effect_receipt_v1(effect, &[])?)?;
        }
        tx.commit().map_err(backend)?;
        Ok(revoked)
    }

    fn revoke_auth_sessions_matching_with_admin_effect(&self, predicate: &str, key: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let mut conn = match self.lock() {
            Ok(conn) => conn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let rows_result: DirectoryResult<Vec<(String, String, i64, String)>> = (|| {
            let sql = format!("SELECT id, user_id, authorization_generation, identity_provider FROM hub_auth_session WHERE {predicate} AND revoked_at IS NULL ORDER BY id");
            let mut statement = tx.prepare(&sql).map_err(backend)?;
            let rows = statement.query_map([key], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })();
        let rows = admin_effect_try!(tx, rows_result);
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let Some(next_generation) = generation.checked_add(1) else { return admin_effect_rollback(tx) };
            admin_effect_try!(
                tx,
                tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![id, revoked_at, reason, next_generation]).map_err(backend)
            );
            let audit = admin_effect_try!(tx, auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server"));
            admin_effect_try!(tx, insert_auth_audit(&tx, &audit));
            let Ok(authorization_generation) = u64::try_from(next_generation) else { return admin_effect_rollback(tx) };
            revoked.push(RevokedAuthSession { id, authorization_generation, revoked_at });
        }
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&tx, &receipt));
        match tx.commit() {
            Ok(()) => AdminEffectCommitV1::Applied(revoked),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }
}

//#region Users
impl UserDirectory for SqliteDirectory {
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        self.lock()?
            .execute(
                "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![id, email, display_name, password_hash, sso_subject, sso_provider, created_at],
            )
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = ?1", [user_id], user_row).optional().map_err(backend)
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1", [email], user_row).optional().map_err(backend)
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?
            .query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2", rusqlite::params![provider, subject], user_row)
            .optional()
            .map_err(backend)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], user_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
}
//#endregion

//#region AuthSessions
impl AuthSessionDirectory for SqliteDirectory {
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(issued.record.issued_at, "session-issued", Some(&issued.record.id), Some(&issued.record.user_id), None, Some(&issued.record.identity_provider), "success", None, &issue.correlation_id, &issue.peer_class)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, NULL, ?9, ?10, ?11)",
            rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), issued.record.user_id, issued.record.identity_provider, issued.record.identity_subject_digest.as_slice(), issued.record.issued_at, issued.record.expires_at, i64::try_from(issued.record.authorization_generation).map_err(backend)?, issued.record.device_instance_id, issued.record.session_kind.as_str()],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let record = self
            .lock()?
            .query_row(
                "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE selector = ?1",
                [capability.selector()],
                auth_session_row,
            )
            .optional()
            .map_err(backend)?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        let (record, role) = {
            let conn = self.lock()?;
            let record = conn
                .query_row(
                    "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE id = ?1",
                    [session_id],
                    auth_session_row,
                )
                .optional()
                .map_err(backend)?;
            let Some(record) = record else { return Ok(SocketSessionBindingStatus::Unavailable) };
            if record.user_id != user_id {
                return Ok(SocketSessionBindingStatus::Unavailable);
            }
            if record.revoked_at.is_some() || record.authorization_generation != authorization_generation {
                return Ok(SocketSessionBindingStatus::Revoked);
            }
            if record.expires_at <= now_ms {
                return Ok(SocketSessionBindingStatus::Expired);
            }
            let role = match space_id {
                Some(space_id) => {
                    conn.query_row("SELECT role FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id], |row| row.get::<_, String>(0)).optional().map_err(backend)?.and_then(|role| SpaceRole::parse(&role))
                }
                None => None,
            };
            (record, role)
        };
        semio_hub_directory::capped_socket_session_binding(self, &record, space_id, role).await
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let row: Option<(String, String, i64)> =
            tx.query_row("SELECT id, user_id, authorization_generation FROM hub_auth_session WHERE id = ?1 AND revoked_at IS NULL", [id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(backend)?;
        let Some((id, user_id, generation)) = row else { return Ok(None) };
        let next_generation = generation.checked_add(1).ok_or_else(|| DirectoryError::Conflict("authorization generation overflow".into()))?;
        tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1", rusqlite::params![id, revoked_at, reason, next_generation]).map_err(backend)?;
        let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(Some(RevokedAuthSession { id, authorization_generation: u64::try_from(next_generation).map_err(backend)?, revoked_at }))
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching("user_id = ?1", user_id, None, reason, actor_user_id, correlation_id, None)
    }

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching_with_admin_effect("user_id = ?1", user_id, reason, actor_user_id, correlation_id, effect)
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching("identity_provider = ?1 AND identity_subject_digest = ?2", provider, Some(subject_digest), reason, actor_user_id, correlation_id, None)
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare("SELECT id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class FROM hub_auth_audit ORDER BY sequence LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = statement.query_map(rusqlite::params![i64::try_from(limit).map_err(backend)?, i64::try_from(offset).map_err(backend)?], auth_audit_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }
}

impl CredentialDirectory for SqliteDirectory {
    async fn append_credential_audit(&self, fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord> {
        let record = auth_audit(now_ms(), &fact.event_kind, None, fact.target_user_id.as_deref(), fact.actor_user_id.as_deref(), None, &fact.outcome_code, fact.reason_code.as_deref(), &fact.correlation_id, &fact.peer_class)?;
        let conn = self.lock()?;
        insert_auth_audit(&conn, &record)?;
        Ok(record)
    }

    async fn set_password_credential(&self, user_id: &str, encoded_credential: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(encoded_credential, "password credential", AUTH_TEXT_MAX_BYTES)?;
        let record = auth_audit(now_ms(), "credential-changed", None, Some(user_id), actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let updated = tx.execute("UPDATE hub_user SET password_hash = ?2 WHERE id = ?1", rusqlite::params![user_id, encoded_credential]).map_err(backend)?;
        if updated == 0 {
            return Err(DirectoryError::Conflict("password credential target user does not exist".into()));
        }
        insert_auth_audit(&tx, &record)?;
        tx.commit().map_err(backend)?;
        Ok(())
    }
}

impl AgentDelegationDirectory for SqliteDirectory {
    async fn create_agent_delegation(&self, issued: &IssuedAgentDelegation, correlation_id: &str, peer_class: &str) -> DirectoryResult<()> {
        let record = &issued.record;
        let audit = auth_audit(record.created_at, AGENT_DELEGATED_EVENT, None, Some(&record.delegating_user_id), Some(&record.delegating_user_id), Some(AGENT_IDENTITY_PROVIDER), "success", Some(record.audience.as_str()), correlation_id, peer_class)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_agent_delegation (id, selector, secret_digest, space_id, delegating_user_id, agent_label, audience, created_at, expires_at, revoked_at, revoked_reason) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, NULL)",
            rusqlite::params![record.id, record.selector, record.secret_digest.as_slice(), record.space_id, record.delegating_user_id, record.agent_label, record.audience.as_str(), record.created_at, record.expires_at],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(())
    }

    async fn list_agent_delegations(&self, space_id: &str, delegating_user_id: &str, limit: usize) -> DirectoryResult<Vec<AgentDelegationRow>> {
        if limit == 0 || limit > AGENT_DELEGATION_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("agent delegation limit must be 1..={AGENT_DELEGATION_PAGE_MAX}")));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT d.id, d.space_id, d.delegating_user_id, d.agent_label, d.audience, d.created_at, d.expires_at, d.revoked_at, \
                 (SELECT MAX(s.issued_at) FROM hub_auth_session s WHERE s.device_instance_id = d.id AND s.session_kind = 'agent') \
                 FROM hub_agent_delegation d WHERE d.space_id = ?1 AND d.delegating_user_id = ?2 ORDER BY d.created_at DESC, d.id DESC LIMIT ?3",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![space_id, delegating_user_id, i64::try_from(limit).map_err(backend)?], |row| {
                let audience: String = row.get(4)?;
                let revoked_at: Option<i64> = row.get(7)?;
                let last_used_at: Option<i64> = row.get(8)?;
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, audience, row.get::<_, i64>(5)?, row.get::<_, i64>(6)?, revoked_at, last_used_at))
            })
            .map_err(backend)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(backend)?;
        rows.into_iter()
            .map(|(delegation_id, space_id, delegating_user_id, agent_label, audience, created_at_ms, expires_at_ms, revoked_at, last_used_at_ms)| {
                Ok(AgentDelegationRow {
                    delegation_id,
                    space_id,
                    delegating_user_id,
                    agent_label,
                    audience: AgentAudience::parse(&audience).ok_or_else(|| DirectoryError::Backend("stored agent delegation audience is invalid".into()))?,
                    created_at_ms,
                    expires_at_ms,
                    revoked: revoked_at.is_some(),
                    last_used_at_ms,
                })
            })
            .collect()
    }

    async fn revoke_agent_delegation(&self, delegation_id: &str, delegating_user_id: &str, reason: &str, correlation_id: &str, now_ms: i64) -> DirectoryResult<Option<Vec<RevokedAuthSession>>> {
        validate_bounded_auth_text(reason, "agent delegation revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let audit = auth_audit(now_ms, AGENT_DELEGATION_REVOKED_EVENT, None, Some(delegating_user_id), Some(delegating_user_id), Some(AGENT_IDENTITY_PROVIDER), "success", Some(reason), correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let owned: i64 = tx
            .query_row("SELECT COUNT(*) FROM hub_agent_delegation WHERE id = ?1 AND delegating_user_id = ?2", rusqlite::params![delegation_id, delegating_user_id], |row| row.get(0))
            .map_err(backend)?;
        if owned == 0 {
            return Ok(None);
        }
        tx.execute("UPDATE hub_agent_delegation SET revoked_at = ?2, revoked_reason = ?3 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![delegation_id, now_ms, reason]).map_err(backend)?;
        let mut statement = tx.prepare("SELECT id, user_id, authorization_generation, identity_provider FROM hub_auth_session WHERE device_instance_id = ?1 AND session_kind = 'agent' AND revoked_at IS NULL ORDER BY id").map_err(backend)?;
        let sessions: Vec<(String, String, i64, String)> =
            statement.query_map([delegation_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(backend)?.collect::<Result<Vec<_>, _>>().map_err(backend)?;
        drop(statement);
        let mut revoked = Vec::with_capacity(sessions.len());
        for (id, user_id, generation, provider) in sessions {
            let next_generation = generation.checked_add(1).ok_or_else(|| DirectoryError::Conflict("authorization generation overflow".into()))?;
            tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![id, now_ms, reason, next_generation]).map_err(backend)?;
            let session_audit = auth_audit(now_ms, "session-revoked", Some(&id), Some(&user_id), Some(delegating_user_id), Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&tx, &session_audit)?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(next_generation).map_err(backend)?, revoked_at: now_ms });
        }
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(Some(revoked))
    }

    async fn load_agent_delegation(&self, selector: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        let conn = self.lock()?;
        conn.query_row("SELECT id, selector, secret_digest, space_id, delegating_user_id, agent_label, audience, created_at, expires_at, revoked_at, revoked_reason FROM hub_agent_delegation WHERE selector = ?1", [selector], agent_delegation_row)
            .optional()
            .map_err(backend)
    }

    async fn agent_delegation(&self, delegation_id: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        let conn = self.lock()?;
        conn.query_row("SELECT id, selector, secret_digest, space_id, delegating_user_id, agent_label, audience, created_at, expires_at, revoked_at, revoked_reason FROM hub_agent_delegation WHERE id = ?1", [delegation_id], agent_delegation_row)
            .optional()
            .map_err(backend)
    }
}
//#endregion
