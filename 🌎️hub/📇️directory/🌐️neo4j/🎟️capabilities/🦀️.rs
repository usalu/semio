//! 🎟️ Share tokens and invites: the capability issuers and their atomic redemption.

use super::*;

impl Neo4jDirectory {
    /// 🔁️ One attempt of [`InviteDirectory::redeem_invite_atomic`]; the trait method retries it while Neo4j reports a transient fault.
    async fn redeem_invite_atomic_attempt(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let accepted_at_ms = now_ms();
        let mut counter_lock = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        counter_lock.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter lock returned no row".into()))?;
        drop(counter_lock);
        let mut invite_lock = txn.execute(query("MATCH (i:SpaceInvite {selector: $selector}) SET i.claimNonce = coalesce(i.claimNonce, 0) + 1 RETURN i AS i").param("selector", capability.selector())).await.map_err(backend)?;
        let record = invite_lock.next(txn.handle()).await.map_err(backend)?.map(|row| invite_from_node(&row)).transpose()?;
        drop(invite_lock);
        let mut user_query = txn.execute(query("MATCH (u:User {id: $user_id}) RETURN count(u) AS count").param("user_id", user_id)).await.map_err(backend)?;
        let user_exists = user_query.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("count").ok()).unwrap_or(0) == 1;
        drop(user_query);
        let space_kind: Option<String> = match record.as_ref() {
            Some(invite) => {
                let mut result = txn.execute(query("MATCH (s:Space {id: $space_id}) RETURN s.kind AS kind").param("space_id", invite.space_id.clone())).await.map_err(backend)?;
                let kind = result.next(txn.handle()).await.map_err(backend)?.map(|row| row.get::<String>("kind").map_err(backend)).transpose()?;
                drop(result);
                kind
            }
            None => None,
        };
        let space_state = InviteRedemptionSpaceStateV1::from_kind(space_kind.as_deref())?;
        match invite_redemption_preflight(record.as_ref(), capability, actor, user_id, user_exists, space_state, accepted_at_ms) {
            InviteRedemptionPreflight::AlreadyCommitted => {
                let invite = record.as_ref().expect("committed preflight requires a record");
                let mut result = txn.execute(query("MATCH (e:DirectoryEvent {id: $event_id}) RETURN e AS e").param("event_id", invite.accepted_event_id.clone().unwrap_or_default())).await.map_err(backend)?;
                let event = result.next(txn.handle()).await.map_err(backend)?.map(|row| event_from_node(&row)).transpose()?;
                drop(result);
                let event = verify_invite_redemption_event(invite, event, user_id)?;
                txn.commit().await.map_err(backend)?;
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
        let mut claimed = txn
            .execute(
                query("MATCH (i:SpaceInvite {id: $invite_id}) WHERE i.acceptedAt IS NULL AND i.acceptedEventId IS NULL AND i.revokedAt IS NULL AND i.expiresAt > $accepted_at SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id RETURN count(i) AS count")
                    .param("invite_id", invite.id.clone())
                    .param("accepted_at", accepted_at_ms)
                    .param("event_id", id.clone()),
            )
            .await
            .map_err(backend)?;
        let changed = claimed.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("count").ok()).unwrap_or(0);
        drop(claimed);
        if changed != 1 {
            return Err(DirectoryError::Backend("Neo4j invitation claim lost its transaction fence".into()));
        }
        let event = NewDirectoryEvent {
            hlc,
            actor: actor.clone(),
            space_id: Some(invite.space_id.clone()),
            user_id: Some(user_id.to_string()),
            body: DirectoryEventBody::InviteRedeemed { space_id: invite.space_id, user_id: user_id.to_string(), invite_id: invite.id, role: role_to_wire(invite.role) },
        };
        let mut counter = txn.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        let sequence: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
        drop(counter);
        let sequence = u64::try_from(sequence).map_err(backend)?;
        if sequence > DIRECTORY_WIRE_INTEGER_MAX {
            return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
        }
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let persisted = DirectoryEvent { seq: sequence, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms: accepted_at_ms };
        validate_directory_event_page_event(&persisted).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        txn.run(
            query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                .param("seq", i64::try_from(sequence).map_err(backend)?)
                .param("id", id.clone())
                .param("hlc_physical", event.hlc.physical_ms)
                .param("hlc_logical", i64::from(event.hlc.logical))
                .param("actor_kind", actor_kind_to_str(event.actor.kind))
                .param("actor_id", event.actor.id.clone())
                .param("space_id", event.space_id.clone())
                .param("user_id", event.user_id.clone())
                .param("kind", kind)
                .param("payload", payload_value.to_string())
                .param("recorded_at", accepted_at_ms),
        )
        .await
        .map_err(backend)?;
        if let Err(error) = self.project(&mut txn, &persisted).await {
            txn.rollback().await.map_err(backend)?;
            return Err(error);
        }
        txn.commit().await.map_err(backend)?;
        Ok(InviteRedemptionCommit::NewlyCommitted { event: persisted })
    }
}

//#region ShareTokens
impl ShareTokenDirectory for Neo4jDirectory {
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let scope_key = document_scope_key_v1(scope);
        let mut result = txn
            .execute(
                query(
                    "MATCH (s:Space {id: $space_id}), (d:DocumentDescriptor {scopeKey: $scope_key, spaceId: $space_id, documentId: $document_id})
                 CREATE (s)-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, documentId: $document_id, createdAt: $created_at, expiresAt: $expires_at})-[:GRANTS_DOCUMENT]->(d)
                 RETURN count(g) AS c",
                )
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", scope_key)
                .param("created_at", issued.record.created_at)
                .param("expires_at", issued.record.expires_at),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed != 1 {
            return Err(DirectoryError::NotFound(format!("document descriptor {}/{}", scope.space_id, scope.document_id)));
        }
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
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
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn.execute(
                query("MATCH (s:Space {id: $space_id}), (d:DocumentDescriptor {scopeKey: $scope_key, spaceId: $space_id, documentId: $document_id}) CREATE (s)-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, documentId: $document_id, createdAt: $created_at, expiresAt: $expires_at})-[:GRANTS_DOCUMENT]->(d) RETURN count(g) AS c")
                    .param("id", issued.record.id.clone())
                    .param("selector", issued.record.selector.clone())
                    .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone())
                    .param("scope_key", document_scope_key_v1(scope))
                    .param("created_at", issued.record.created_at)
                    .param("expires_at", issued.record.expires_at),
            )
            .await
            .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (g:ShareGrant {id: $id, spaceId: $space_id, documentId: $document_id}) WHERE g.revokedAt IS NULL SET g.revokedAt = $revoked_at, g.revokedReason = $reason RETURN count(g) AS c")
                    .param("id", share_id)
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone())
                    .param("revoked_at", revoked_at)
                    .param("reason", reason),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&mut txn, &audit).await?;
            txn.commit().await.map_err(backend)?;
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
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (g:ShareGrant {id: $id, spaceId: $space_id, documentId: $document_id}) WHERE g.revokedAt IS NULL SET g.revokedAt = $revoked_at, g.revokedReason = $reason RETURN count(g) AS c")
                        .param("id", share_id)
                        .param("space_id", scope.space_id.clone())
                        .param("document_id", scope.document_id.clone())
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (:Space {id: $space_id})-[:HAS_SHARE_GRANT]->(g:ShareGrant {selector: $selector, spaceId: $space_id, documentId: $document_id})-[:GRANTS_DOCUMENT]->(:DocumentDescriptor {scopeKey: $scope_key})
                     RETURN g AS g",
                )
                .param("selector", capability.selector())
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", document_scope_key_v1(scope)),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let record = share_from_node(&row)?;
        Ok(active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms()).then_some(record))
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (:Space {id: $space_id})-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, spaceId: $space_id, documentId: $document_id})-[:GRANTS_DOCUMENT]->(:DocumentDescriptor {scopeKey: $scope_key})
                     RETURN g AS g",
                )
                .param("id", share_id)
                .param("selector", selector)
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", document_scope_key_v1(scope)),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(SocketShareBindingStatus::Unavailable) };
        let record = share_from_node(&row)?;
        Ok(if record.revoked_at.is_some() {
            SocketShareBindingStatus::Revoked
        } else if record.expires_at <= now_ms {
            SocketShareBindingStatus::Expired
        } else {
            SocketShareBindingStatus::Active { expires_at_ms: record.expires_at }
        })
    }
}
//#endregion

//#region Invites
impl InviteDirectory for Neo4jDirectory {
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(
            query("CREATE (i:SpaceInvite {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, role: $role, createdAt: $created_at, expiresAt: $expires_at})")
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("space_id", space_id)
                .param("role", role.as_str())
                .param("created_at", issued.record.created_at)
                .param("expires_at", issued.record.expires_at),
        )
        .await
        .map_err(backend)?;
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
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
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (s:Space {id: $space_id}) CREATE (i:SpaceInvite {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, role: $role, createdAt: $created_at, expiresAt: $expires_at}) RETURN count(i) AS c")
                        .param("id", issued.record.id.clone())
                        .param("selector", issued.record.selector.clone())
                        .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                        .param("space_id", space_id)
                        .param("role", role.as_str())
                        .param("created_at", issued.record.created_at)
                        .param("expires_at", issued.record.expires_at),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        let mut result = self
            .graph
            .execute(query("MATCH (i:SpaceInvite {selector: $selector}) MATCH (s:Space {id: i.spaceId}) MATCH (u:User {id: $user_id}) RETURN i AS i").param("selector", capability.selector()).param("user_id", user_id))
            .await
            .map_err(backend)?;
        let record = result.next().await.map_err(backend)?.map(|row| invite_from_node(&row)).transpose()?;
        verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.redeem_invite_atomic_attempt(capability, actor, user_id, hlc).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) WHERE i.revokedAt IS NULL AND i.acceptedAt IS NULL SET i.revokedAt = $revoked_at, i.revokedReason = $reason RETURN count(i) AS c")
                    .param("id", invite_id)
                    .param("space_id", space_id)
                    .param("revoked_at", revoked_at)
                    .param("reason", reason),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed == 0 {
            drop(result);
            let mut accepted = txn.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) RETURN i.acceptedAt AS acceptedAt").param("id", invite_id).param("space_id", space_id)).await.map_err(backend)?;
            let accepted = accepted.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("acceptedAt").ok()).is_some();
            return if accepted { Err(DirectoryError::Conflict("invite already accepted".into())) } else { Err(DirectoryError::NotFound(format!("invite {invite_id}"))) };
        }
        drop(result);
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
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
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) WHERE i.revokedAt IS NULL AND i.acceptedAt IS NULL SET i.revokedAt = $revoked_at, i.revokedReason = $reason RETURN count(i) AS c")
                        .param("id", invite_id)
                        .param("space_id", space_id)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id}) RETURN i AS i ORDER BY i.createdAt DESC").param("space_id", space_id)).await.map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            invites.push(invite_from_node(&row)?);
        }
        Ok(invites)
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (i:SpaceInvite {spaceId: $space_id})
                     WHERE $after_created_at IS NULL OR i.createdAt < $after_created_at OR (i.createdAt = $after_created_at AND i.id < $after_id)
                     RETURN i.id AS inviteId, i.role AS role, i.createdAt AS createdAt, i.expiresAt AS expiresAt, i.revokedAt AS revokedAt, i.acceptedAt AS acceptedAt
                     ORDER BY i.createdAt DESC, i.id DESC LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("after_created_at", after.map(|(created_at, _)| created_at))
                .param("after_id", after.map(|(_, invite_id)| invite_id.to_owned()))
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            invites.push(SpaceAdministrationInviteRow {
                invite_id: row.get("inviteId").map_err(backend)?,
                role: SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored invite role is invalid".into()))?,
                created_at_ms: row.get("createdAt").map_err(backend)?,
                expires_at_ms: row.get("expiresAt").map_err(backend)?,
                revoked: row.get::<i64>("revokedAt").is_ok(),
                accepted: row.get::<i64>("acceptedAt").is_ok(),
            });
        }
        Ok(invites)
    }
}
//#endregion
