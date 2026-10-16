//! 🎟️ Share tokens and invites: the capability issuers and their atomic redemption.

use super::*;

//#region ShareTokens
impl ShareTokenDirectory for PostgresDirectory {
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let changed = sqlx_core::query::query(
            "INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason)
             SELECT $1, $2, $3, $4, $5, $6, $7, NULL, NULL
             FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id
             WHERE s.id = $4 AND d.space_id = $4 AND d.document_id = $5
             FOR KEY SHARE OF s, d",
        )
        .bind(&issued.record.id)
        .bind(&issued.record.selector)
        .bind(issued.record.secret_digest.as_slice())
        .bind(&scope.space_id)
        .bind(&scope.document_id)
        .bind(issued.record.created_at)
        .bind(issued.record.expires_at)
        .execute(&mut *tx)
        .await
        .map_err(backend)?;
        if changed.rows_affected() != 1 {
            return Err(DirectoryError::NotFound(format!("document descriptor {}/{}", scope.space_id, scope.document_id)));
        }
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
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
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed = admin_effect_try!(
            tx,
            sqlx_core::query::query(
                "INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason)
             SELECT $1,$2,$3,$4,$5,$6,$7,NULL,NULL FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id
             WHERE s.id = $4 AND d.space_id = $4 AND d.document_id = $5 FOR KEY SHARE OF s, d",
            )
            .bind(&issued.record.id)
            .bind(&issued.record.selector)
            .bind(issued.record.secret_digest.as_slice())
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(issued.record.created_at)
            .bind(issued.record.expires_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)
        );
        if changed.rows_affected() != 1 {
            return admin_effect_rollback(tx).await;
        }
        admin_effect_try!(tx, insert_auth_audit(&mut tx, &audit).await);
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&mut tx, &receipt).await);
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let result = sqlx_core::query::query("UPDATE hub_share_grant SET revoked_at = $4, revoked_reason = $5 WHERE id = $1 AND space_id = $2 AND document_id = $3 AND revoked_at IS NULL")
            .bind(share_id)
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(revoked_at)
            .bind(reason)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        if result.rows_affected() == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&mut tx, &audit).await?;
            tx.commit().await.map_err(backend)?;
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
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed = admin_effect_try!(
            tx,
            sqlx_core::query::query("UPDATE hub_share_grant SET revoked_at = $4, revoked_reason = $5 WHERE id = $1 AND space_id = $2 AND document_id = $3 AND revoked_at IS NULL")
                .bind(share_id)
                .bind(&scope.space_id)
                .bind(&scope.document_id)
                .bind(revoked_at)
                .bind(reason)
                .execute(&mut *tx)
                .await
                .map_err(backend)
        );
        if changed.rows_affected() != 1 {
            return admin_effect_rollback(tx).await;
        }
        admin_effect_try!(tx, insert_auth_audit(&mut tx, &audit).await);
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&mut tx, &receipt).await);
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let row: Option<ShareRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE space_id = $1 AND document_id = $2 AND selector = $3")
                .bind(&scope.space_id)
                .bind(&scope.document_id)
                .bind(capability.selector())
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        let record = row.map(share_from_row).transpose()?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        let row: Option<ShareRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE id = $1 AND selector = $2 AND space_id = $3 AND document_id = $4")
                .bind(share_id)
                .bind(selector)
                .bind(&scope.space_id)
                .bind(&scope.document_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(match row.map(share_from_row).transpose()? {
            None => SocketShareBindingStatus::Unavailable,
            Some(record) if record.revoked_at.is_some() => SocketShareBindingStatus::Revoked,
            Some(record) if record.expires_at <= now_ms => SocketShareBindingStatus::Expired,
            Some(record) => SocketShareBindingStatus::Active { expires_at_ms: record.expires_at },
        })
    }
}
//#endregion

//#region Invites
impl InviteDirectory for PostgresDirectory {
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query(
            "INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id) VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, NULL, NULL, NULL)",
        )
        .bind(&issued.record.id)
        .bind(&issued.record.selector)
        .bind(issued.record.secret_digest.as_slice())
        .bind(space_id)
        .bind(role.as_str())
        .bind(issued.record.created_at)
        .bind(issued.record.expires_at)
        .execute(&mut *tx)
        .await
        .map_err(backend)?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
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
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        admin_effect_try!(
            tx,
            sqlx_core::query::query("INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id) VALUES ($1,$2,$3,$4,$5,$6,$7,NULL,NULL,NULL,NULL)")
                .bind(&issued.record.id)
                .bind(&issued.record.selector)
                .bind(issued.record.secret_digest.as_slice())
                .bind(space_id)
                .bind(role.as_str())
                .bind(issued.record.created_at)
                .bind(issued.record.expires_at)
                .execute(&mut *tx)
                .await
                .map_err(backend)
        );
        admin_effect_try!(tx, insert_auth_audit(&mut tx, &audit).await);
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&mut tx, &receipt).await);
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        let row: Option<InviteRow> = sqlx_core::query_as::query_as(
            "SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE selector = $1 AND EXISTS(SELECT 1 FROM hub_space WHERE id = hub_space_invite.space_id) AND EXISTS(SELECT 1 FROM hub_user WHERE id = $2)",
        ).bind(capability.selector()).bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        let record = row.map(invite_from_row).transpose()?;
        verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let _: (i64,) = sqlx_core::query_as::query_as("SELECT seq FROM hub_directory_event_head WHERE singleton FOR UPDATE").fetch_one(&mut *tx).await.map_err(backend)?;
        let accepted_at_ms = now_ms();
        let row: Option<InviteRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE selector = $1 FOR UPDATE")
                .bind(capability.selector())
                .fetch_optional(&mut *tx)
                .await
                .map_err(backend)?;
        let record = row.map(invite_from_row).transpose()?;
        let (user_exists,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_user WHERE id = $1)").bind(user_id).fetch_one(&mut *tx).await.map_err(backend)?;
        let space_kind: Option<(String,)> = match record.as_ref() {
            Some(invite) => sqlx_core::query_as::query_as("SELECT kind FROM hub_space WHERE id = $1").bind(&invite.space_id).fetch_optional(&mut *tx).await.map_err(backend)?,
            None => None,
        };
        let space_state = InviteRedemptionSpaceStateV1::from_kind(space_kind.as_ref().map(|row| row.0.as_str()))?;
        let preflight = invite_redemption_preflight(record.as_ref(), capability, actor, user_id, user_exists, space_state, accepted_at_ms);
        match preflight {
            InviteRedemptionPreflight::AlreadyCommitted => {
                let invite = record.as_ref().expect("committed preflight requires a record");
                let event_row: Option<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> =
                    sqlx_core::query_as::query_as("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE id = $1")
                        .bind(invite.accepted_event_id.as_deref().unwrap_or_default())
                        .fetch_optional(&mut *tx)
                        .await
                        .map_err(backend)?;
                let event = verify_invite_redemption_event(invite, event_row.map(event_from_row).transpose()?, user_id)?;
                tx.commit().await.map_err(backend)?;
                return Ok(InviteRedemptionCommit::AlreadyCommitted { event });
            }
            InviteRedemptionPreflight::Revoked => return Err(DirectoryError::Conflict("invite already revoked".into())),
            InviteRedemptionPreflight::Expired => return Err(DirectoryError::Conflict("invite expired".into())),
            InviteRedemptionPreflight::Denied => return Err(DirectoryError::Unauthorized),
            InviteRedemptionPreflight::Corrupt => return Err(DirectoryError::Backend("invite acceptance marker is incomplete".into())),
            InviteRedemptionPreflight::Claim => {}
        }
        let invite = record.expect("claim preflight requires a record");
        let id = time_ordered_id();
        let updated = sqlx_core::query::query("UPDATE hub_space_invite SET accepted_at = $2, accepted_event_id = $3 WHERE id = $1 AND accepted_at IS NULL AND accepted_event_id IS NULL AND revoked_at IS NULL AND expires_at > $2")
            .bind(&invite.id)
            .bind(accepted_at_ms)
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        if updated.rows_affected() != 1 {
            return Err(DirectoryError::Backend("PostgreSQL invitation claim lost its row fence".into()));
        }
        let event = NewDirectoryEvent {
            hlc,
            actor: actor.clone(),
            space_id: Some(invite.space_id.clone()),
            user_id: Some(user_id.to_string()),
            body: DirectoryEventBody::InviteRedeemed { space_id: invite.space_id, user_id: user_id.to_string(), invite_id: invite.id, role: role_to_wire(invite.role) },
        };
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default();
        let (sequence,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
        let persisted = DirectoryEvent {
            seq: u64::try_from(sequence).map_err(backend)?,
            id: id.clone(),
            hlc: event.hlc,
            actor: event.actor.clone(),
            space_id: event.space_id.clone(),
            user_id: event.user_id.clone(),
            body: event.body.clone(),
            recorded_at_ms: accepted_at_ms,
        };
        validate_directory_event_page_event(&persisted).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        sqlx_core::query::query("INSERT INTO hub_directory_event (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
            .bind(sequence)
            .bind(&id)
            .bind(event.hlc.physical_ms)
            .bind(i64::from(event.hlc.logical))
            .bind(actor_kind_to_str(event.actor.kind))
            .bind(&event.actor.id)
            .bind(&event.space_id)
            .bind(&event.user_id)
            .bind(kind)
            .bind(&payload_value)
            .bind(accepted_at_ms)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        if let Err(error) = self.project(&mut tx, &persisted).await {
            tx.rollback().await.map_err(backend)?;
            return Err(error);
        }
        tx.commit().await.map_err(backend)?;
        Ok(InviteRedemptionCommit::NewlyCommitted { event: persisted })
    }

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let result = sqlx_core::query::query("UPDATE hub_space_invite SET revoked_at = $2, revoked_reason = $3 WHERE space_id = $4 AND id = $1 AND revoked_at IS NULL AND accepted_at IS NULL")
            .bind(invite_id)
            .bind(revoked_at)
            .bind(reason)
            .bind(space_id)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        if result.rows_affected() == 0 {
            let accepted: Option<(Option<i64>,)> = sqlx_core::query_as::query_as("SELECT accepted_at FROM hub_space_invite WHERE space_id = $2 AND id = $1").bind(invite_id).bind(space_id).fetch_optional(&mut *tx).await.map_err(backend)?;
            return if accepted.and_then(|row| row.0).is_some() { Err(DirectoryError::Conflict("invite already accepted".into())) } else { Err(DirectoryError::NotFound(format!("invite {invite_id}"))) };
        }
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
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
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed = admin_effect_try!(
            tx,
            sqlx_core::query::query("UPDATE hub_space_invite SET revoked_at = $2, revoked_reason = $3 WHERE space_id = $4 AND id = $1 AND revoked_at IS NULL AND accepted_at IS NULL")
                .bind(invite_id)
                .bind(revoked_at)
                .bind(reason)
                .bind(space_id)
                .execute(&mut *tx)
                .await
                .map_err(backend)
        );
        if changed.rows_affected() != 1 {
            return admin_effect_rollback(tx).await;
        }
        admin_effect_try!(tx, insert_auth_audit(&mut tx, &audit).await);
        let receipt = admin_effect_try!(tx, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(tx, insert_admin_operation_effect_receipt(&mut tx, &receipt).await);
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let rows: Vec<InviteRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE space_id = $1 ORDER BY created_at DESC")
                .bind(space_id)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        rows.into_iter().map(invite_from_row).collect()
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, i64, i64, bool, bool)> = sqlx_core::query_as::query_as(
            "SELECT id, role, created_at, expires_at, revoked_at IS NOT NULL, accepted_at IS NOT NULL
             FROM hub_space_invite
             WHERE space_id = $1 AND ($2::bigint IS NULL OR created_at < $2 OR (created_at = $2 AND id < $3))
             ORDER BY created_at DESC, id DESC LIMIT $4",
        )
        .bind(space_id)
        .bind(after.map(|(created_at, _)| created_at))
        .bind(after.map(|(_, invite_id)| invite_id))
        .bind(i64::try_from(limit).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(invite_id, role, created_at_ms, expires_at_ms, revoked, accepted)| {
                SpaceRole::parse(&role).map(|role| SpaceAdministrationInviteRow { invite_id, role, created_at_ms, expires_at_ms, revoked, accepted }).ok_or_else(|| DirectoryError::Backend("stored invite role is invalid".into()))
            })
            .collect()
    }
}
//#endregion
