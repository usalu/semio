//! 👤️ Users, auth sessions, credentials and agent delegations.

use super::*;

pub(super) async fn insert_auth_audit(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &AuthAuditRecord) -> DirectoryResult<()> {
    sqlx_core::query::query(
        "INSERT INTO hub_auth_audit (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(&event.id)
    .bind(event.occurred_at)
    .bind(&event.event_kind)
    .bind(&event.auth_session_id)
    .bind(&event.target_user_id)
    .bind(&event.actor_user_id)
    .bind(&event.provider)
    .bind(&event.outcome_code)
    .bind(&event.reason_code)
    .bind(&event.correlation_id)
    .bind(&event.peer_class)
    .execute(&mut **tx)
    .await
    .map_err(backend)?;
    Ok(())
}

impl PostgresDirectory {
    async fn revoke_auth_sessions_matching(
        &self,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        admin_effect: Option<&NewAdminOperationEffectReceiptV1>,
    ) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let rows: Vec<(String, String, i64, String)> = if let Some(subject_digest) = subject_digest {
            sqlx_core::query_as::query_as(
                "UPDATE hub_auth_session SET revoked_at = $3, revoked_reason = $4, authorization_generation = authorization_generation + 1 WHERE identity_provider = $1 AND identity_subject_digest = $2 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
            )
            .bind(key)
            .bind(subject_digest.as_slice())
            .bind(revoked_at)
            .bind(reason)
            .fetch_all(&mut *tx)
            .await
            .map_err(backend)?
        } else {
            sqlx_core::query_as::query_as(
                "UPDATE hub_auth_session SET revoked_at = $2, revoked_reason = $3, authorization_generation = authorization_generation + 1 WHERE user_id = $1 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
            )
            .bind(key)
            .bind(revoked_at)
            .bind(reason)
            .fetch_all(&mut *tx)
            .await
            .map_err(backend)?
        };
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&mut tx, &audit).await?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at });
        }
        if let Some(effect) = admin_effect {
            insert_admin_operation_effect_receipt(&mut tx, &admin_operation_effect_receipt_v1(effect, &[])?).await?;
        }
        tx.commit().await.map_err(backend)?;
        Ok(revoked)
    }

    async fn revoke_auth_sessions_matching_with_admin_effect(&self, key: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let rows: Vec<(String, String, i64, String)> =
            admin_effect_try!(tx, sqlx_core::query_as::query_as(
            "UPDATE hub_auth_session SET revoked_at = $2, revoked_reason = $3, authorization_generation = authorization_generation + 1 WHERE user_id = $1 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
        )
        .bind(key)
        .bind(revoked_at)
        .bind(reason)
        .fetch_all(&mut *tx)
        .await
        .map_err(backend));
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let audit = admin_effect_try!(tx, auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server"));
            admin_effect_try!(tx, insert_auth_audit(&mut tx, &audit).await);
            let Ok(authorization_generation) = u64::try_from(generation) else { return admin_effect_rollback(tx).await };
            revoked.push(RevokedAuthSession { id, authorization_generation, revoked_at });
        }
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&mut tx, &receipt).await);
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(revoked),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }
}

//#region Users
impl UserDirectory for PostgresDirectory {
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(email)
            .bind(display_name)
            .bind(password_hash)
            .bind(sso_subject)
            .bind(sso_provider)
            .bind(created_at)
            .execute(&self.pool)
            .await
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
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = $1").bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = $1").bind(email).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = $1 AND sso_subject = $2")
                .bind(provider)
                .bind(subject)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(user_from_row).collect())
    }
}
//#endregion

//#region AuthSessions
impl AuthSessionDirectory for PostgresDirectory {
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(issued.record.issued_at, "session-issued", Some(&issued.record.id), Some(&issued.record.user_id), None, Some(&issued.record.identity_provider), "success", None, &issue.correlation_id, &issue.peer_class)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NULL, NULL, $9, $10, $11)")
            .bind(&issued.record.id)
            .bind(&issued.record.selector)
            .bind(issued.record.secret_digest.as_slice())
            .bind(&issued.record.user_id)
            .bind(&issued.record.identity_provider)
            .bind(issued.record.identity_subject_digest.as_slice())
            .bind(issued.record.issued_at)
            .bind(issued.record.expires_at)
            .bind(i64::try_from(issued.record.authorization_generation).map_err(backend)?)
            .bind(&issued.record.device_instance_id)
            .bind(issued.record.session_kind.as_str())
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let row: Option<AuthSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE selector = $1",
        )
        .bind(capability.selector())
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        let record = row.map(auth_session_from_row).transpose()?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        let row: Option<AuthSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        let Some(record) = row.map(auth_session_from_row).transpose()? else { return Ok(SocketSessionBindingStatus::Unavailable) };
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
                let row: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
                row.and_then(|(role,)| SpaceRole::parse(&role))
            }
            None => None,
        };
        semio_hub_directory::capped_socket_session_binding(self, &record, space_id, role).await
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let row: Option<(String, String, i64, String)> = sqlx_core::query_as::query_as(
            "UPDATE hub_auth_session SET revoked_at = $2, revoked_reason = $3, authorization_generation = authorization_generation + 1 WHERE id = $1 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
        )
        .bind(id)
        .bind(revoked_at)
        .bind(reason)
        .fetch_optional(&mut *tx)
        .await
        .map_err(backend)?;
        let Some((id, user_id, generation, provider)) = row else { return Ok(None) };
        let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(Some(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at }))
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching(user_id, None, reason, actor_user_id, correlation_id, None).await
    }

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching(provider, Some(subject_digest), reason, actor_user_id, correlation_id, None).await
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let rows: Vec<AuthAuditRow> =
            sqlx_core::query_as::query_as("SELECT id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class FROM hub_auth_audit ORDER BY sequence LIMIT $1 OFFSET $2")
                .bind(i64::try_from(limit).map_err(backend)?)
                .bind(i64::try_from(offset).map_err(backend)?)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(auth_audit_from_row).collect())
    }
}

impl CredentialDirectory for PostgresDirectory {
    async fn append_credential_audit(&self, fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord> {
        let record = auth_audit(now_ms(), &fact.event_kind, None, fact.target_user_id.as_deref(), fact.actor_user_id.as_deref(), None, &fact.outcome_code, fact.reason_code.as_deref(), &fact.correlation_id, &fact.peer_class)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        insert_auth_audit(&mut tx, &record).await?;
        tx.commit().await.map_err(backend)?;
        Ok(record)
    }

    async fn set_password_credential(&self, user_id: &str, encoded_credential: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(encoded_credential, "password credential", AUTH_TEXT_MAX_BYTES)?;
        let record = auth_audit(now_ms(), "credential-changed", None, Some(user_id), actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let updated = sqlx_core::query::query("UPDATE hub_user SET password_hash = $2 WHERE id = $1").bind(user_id).bind(encoded_credential).execute(&mut *tx).await.map_err(backend)?;
        if updated.rows_affected() == 0 {
            return Err(DirectoryError::Conflict("password credential target user does not exist".into()));
        }
        insert_auth_audit(&mut tx, &record).await?;
        tx.commit().await.map_err(backend)?;
        Ok(())
    }
}

/// 🚫️ This backend stores no agent delegations: the trait defaults answer `unavailable`.
impl AgentDelegationDirectory for PostgresDirectory {}
//#endregion
