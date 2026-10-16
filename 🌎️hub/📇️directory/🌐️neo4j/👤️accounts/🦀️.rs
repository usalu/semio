//! 👤️ Users, auth sessions, credentials and agent delegations.

use super::*;

impl Neo4jDirectory {
    async fn revoke_auth_sessions_by(
        &self,
        field: &str,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        admin_effect: Option<&NewAdminOperationEffectReceiptV1>,
    ) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = match field {
            "id" => {
                txn.execute(
                    query("MATCH (a:AuthSession {id: $key})-[:BELONGS_TO]->(u:User) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            "user" => {
                txn.execute(
                    query("MATCH (a:AuthSession)-[:BELONGS_TO]->(u:User {id: $key}) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            "identity" => {
                let digest = subject_digest.ok_or_else(|| DirectoryError::Backend("identity revocation requires subject digest".into()))?;
                txn.execute(
                    query("MATCH (a:AuthSession {identityProvider: $key, identitySubjectDigest: $digest})-[:BELONGS_TO]->(u:User) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("digest", encode_capability_bytes(&digest))
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            _ => return Err(DirectoryError::Backend("invalid auth revocation selector".into())),
        };
        let mut rows = Vec::new();
        while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
            let id: String = row.get("id").map_err(backend)?;
            let user_id: String = row.get("userId").map_err(backend)?;
            let provider: String = row.get("provider").map_err(backend)?;
            let generation: i64 = row.get("generation").map_err(backend)?;
            rows.push((id, user_id, provider, generation));
        }
        drop(result);
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, provider, generation) in rows {
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&mut txn, &audit).await?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at });
        }
        if let Some(effect) = admin_effect {
            insert_admin_operation_effect_receipt(&mut txn, &admin_operation_effect_receipt_v1(effect, &[])?).await?;
        }
        txn.commit().await.map_err(backend)?;
        Ok(revoked)
    }

    async fn revoke_auth_sessions_by_user_with_admin_effect(&self, key: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let rows_result: DirectoryResult<Vec<(String, String, String, i64)>> = async {
            let mut result = txn
                .execute(
                    query("MATCH (a:AuthSession)-[:BELONGS_TO]->(u:User {id: $key}) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let mut rows = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                rows.push((row.get("id").map_err(backend)?, row.get("userId").map_err(backend)?, row.get("provider").map_err(backend)?, row.get("generation").map_err(backend)?));
            }
            drop(result);
            Ok(rows)
        }
        .await;
        let rows = admin_effect_try!(txn, rows_result);
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, provider, generation) in rows {
            let audit = admin_effect_try!(txn, auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server"));
            admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
            let Ok(authorization_generation) = u64::try_from(generation) else { return admin_effect_rollback(txn).await };
            revoked.push(RevokedAuthSession { id, authorization_generation, revoked_at });
        }
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(revoked),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }
}

//#region Users
impl UserDirectory for Neo4jDirectory {
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        self.graph
            .run(
                query(
                    "CREATE (u:User {id: $id, email: $email, displayName: $display_name, passwordHash: $password_hash,
                                      ssoSubject: $sso_subject, ssoProvider: $sso_provider, createdAt: $created_at})",
                )
                .param("id", id.clone())
                .param("email", email)
                .param("display_name", display_name)
                .param("password_hash", password_hash.unwrap_or_default())
                .param("sso_subject", sso_subject.unwrap_or_default())
                .param("sso_provider", sso_provider.unwrap_or_default())
                .param("created_at", created_at),
            )
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
        let mut result = self.graph.execute(query("MATCH (u:User {id: $id}) RETURN u AS u").param("id", user_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {email: $email}) RETURN u AS u").param("email", email)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {ssoProvider: $provider, ssoSubject: $subject}) RETURN u AS u").param("provider", provider).param("subject", subject)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User) RETURN u AS u ORDER BY u.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut users = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            users.push(user_from_node(&row)?);
        }
        Ok(users)
    }
}
//#endregion

//#region AuthSessions
impl AuthSessionDirectory for Neo4jDirectory {
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(issued.record.issued_at, "session-issued", Some(&issued.record.id), Some(&issued.record.user_id), None, Some(&issued.record.identity_provider), "success", None, &issue.correlation_id, &issue.peer_class)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(
                query(
                    "MATCH (u:User {id: $user_id})
                     CREATE (a:AuthSession {id: $id, selector: $selector, secretDigest: $secret_digest, identityProvider: $identity_provider, identitySubjectDigest: $identity_subject_digest, issuedAt: $issued_at, expiresAt: $expires_at, authorizationGeneration: $generation, deviceInstanceId: $device_instance_id, sessionKind: $session_kind})-[:BELONGS_TO]->(u)",
                )
                .param("user_id", issued.record.user_id.clone())
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("identity_provider", issued.record.identity_provider.clone())
                .param("identity_subject_digest", encode_capability_bytes(&issued.record.identity_subject_digest))
                .param("issued_at", issued.record.issued_at)
                .param("expires_at", issued.record.expires_at)
                .param("generation", i64::try_from(issued.record.authorization_generation).map_err(backend)?)
                .param("device_instance_id", issued.record.device_instance_id.clone())
                .param("session_kind", issued.record.session_kind.as_str()),
            )
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let mut result = self.graph.execute(query("MATCH (a:AuthSession {selector: $selector})-[:BELONGS_TO]->(u:User) RETURN a AS a, u.id AS userId").param("selector", capability.selector())).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let record = auth_session_from_node(&row)?;
                Ok(active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms()).then_some(record))
            }
            None => Ok(None),
        }
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        let mut result = match space_id {
            Some(space_id) => self
                .graph
                .execute(
                    query("MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User) OPTIONAL MATCH (u)-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN a AS a, u.id AS userId, m.role AS role")
                        .param("session_id", session_id)
                        .param("space_id", space_id),
                )
                .await
                .map_err(backend)?,
            None => self.graph.execute(query("MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User) RETURN a AS a, u.id AS userId").param("session_id", session_id)).await.map_err(backend)?,
        };
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(SocketSessionBindingStatus::Unavailable) };
        let record = auth_session_from_node(&row)?;
        if record.user_id != user_id {
            return Ok(SocketSessionBindingStatus::Unavailable);
        }
        if record.revoked_at.is_some() || record.authorization_generation != authorization_generation {
            return Ok(SocketSessionBindingStatus::Revoked);
        }
        if record.expires_at <= now_ms {
            return Ok(SocketSessionBindingStatus::Expired);
        }
        let role = space_id.and_then(|_| row.get::<String>("role").ok().and_then(|role| SpaceRole::parse(&role)));
        semio_hub_directory::capped_socket_session_binding(self, &record, space_id, role).await
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        let mut revoked = self.revoke_auth_sessions_by("id", id, None, reason, actor_user_id, correlation_id, None).await?;
        Ok(revoked.pop())
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("user", user_id, None, reason, actor_user_id, correlation_id, None).await
    }

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("identity", provider, Some(subject_digest), reason, actor_user_id, correlation_id, None).await
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(query("MATCH (a:AuthAudit) RETURN a AS a ORDER BY a.occurredAt, a.id SKIP $offset LIMIT $limit").param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?))
            .await
            .map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(auth_audit_from_node(&row)?);
        }
        Ok(records)
    }
}

impl CredentialDirectory for Neo4jDirectory {
    async fn append_credential_audit(&self, fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord> {
        let record = auth_audit(now_ms(), &fact.event_kind, None, fact.target_user_id.as_deref(), fact.actor_user_id.as_deref(), None, &fact.outcome_code, fact.reason_code.as_deref(), &fact.correlation_id, &fact.peer_class)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        insert_auth_audit(&mut txn, &record).await?;
        txn.commit().await.map_err(backend)?;
        Ok(record)
    }

    async fn set_password_credential(&self, user_id: &str, encoded_credential: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(encoded_credential, "password credential", AUTH_TEXT_MAX_BYTES)?;
        let record = auth_audit(now_ms(), "credential-changed", None, Some(user_id), actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut updated = txn
            .execute(query("MATCH (u:User {id: $user_id}) SET u.passwordHash = $password_hash RETURN u.id AS id").param("user_id", user_id.to_string()).param("password_hash", encoded_credential.to_string()))
            .await
            .map_err(backend)?;
        let matched = updated.next(txn.handle()).await.map_err(backend)?.is_some();
        drop(updated);
        if !matched {
            txn.rollback().await.map_err(backend)?;
            return Err(DirectoryError::Conflict("password credential target user does not exist".into()));
        }
        insert_auth_audit(&mut txn, &record).await?;
        txn.commit().await.map_err(backend)?;
        Ok(())
    }
}

/// 🚫️ This backend stores no agent delegations: the trait defaults answer `unavailable`.
impl AgentDelegationDirectory for Neo4jDirectory {}
//#endregion
