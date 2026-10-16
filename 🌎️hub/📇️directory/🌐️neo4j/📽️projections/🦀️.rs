//! 📽️ Event persistence and the projection of directory events into the read tables.

use super::*;

impl Neo4jDirectory {
    pub(super) async fn project_verified_checkpoint(&self, txn: &mut Txn, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        let key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
        txn.run(
            query(
                "MATCH (c:ArtifactCheckpoint {scopeCheckpointKey: $key})
                 CREATE (p:ArtifactCheckpointPrivate {scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, eventSeq: $event_seq, payload: $payload})
                 MERGE (c)-[:HAS_PRIVATE_AUTHORITY]->(p)",
            )
            .param("key", key)
            .param("space_id", checkpoint.scope.space_id.clone())
            .param("document_id", checkpoint.scope.document_id.clone())
            .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
            .param("payload", semio_framework_pack_json::to_json_string(checkpoint)),
        )
        .await
        .map_err(backend)?;
        Ok(())
    }
}

//#region 🔖️Projections
impl Neo4jDirectory {
    /// 🏛️ Reads projection authority after the Neo4j directory-counter writer lock.
    pub(super) async fn projection_rejection(&self, txn: &mut Txn, body: &DirectoryEventBody) -> DirectoryResult<Option<DirectoryProjectionRejectionV1>> {
        let Some(space_id) = directory_projection_space_v1(body) else {
            return Ok(None);
        };
        let mut result = txn.execute(query("MATCH (s:Space {id: $id}) RETURN s.kind AS kind").param("id", space_id)).await.map_err(backend)?;
        let kind = result.next(txn.handle()).await.map_err(backend)?.map(|row| row.get::<String>("kind").map_err(backend)).transpose()?;
        Ok(directory_projection_rejection_v1(body, kind.as_deref()))
    }

    /// 🧮️ Applies one event and enforces intrinsic archive authority in the same transaction.
    pub(super) async fn project(&self, txn: &mut Txn, event: &DirectoryEvent) -> DirectoryResult<()> {
        if let Some(reason) = self.projection_rejection(txn, &event.body).await? {
            return Err(reason.into_error());
        }
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                txn.run(
                    query("MERGE (u:User {id: $id}) ON CREATE SET u.email = $email, u.displayName = $display_name, u.createdAt = $created_at")
                        .param("id", user_id.clone())
                        .param("email", email.clone())
                        .param("display_name", display_name.clone())
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                txn.run(
                    query("MERGE (s:Space {id: $id}) ON CREATE SET s.name = $name, s.ownerUserId = $owner_user_id, s.kind = $kind, s.visibility = $visibility, s.createdAt = $created_at")
                        .param("id", space_id.clone())
                        .param("name", name.clone())
                        .param("owner_user_id", owner_user_id.clone())
                        .param("kind", kind_to_str(*space_kind))
                        .param("visibility", visibility_to_str(*visibility))
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.name = $name").param("id", space_id.clone()).param("name", name.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.visibility = $visibility").param("id", space_id.clone()).param("visibility", visibility_to_str(*visibility))).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.kind = 'archive'").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (:User)-[m:MEMBER_OF]->(:Space {id: $id}) WHERE m.role = 'author' SET m.role = 'spectator'").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                txn.run(query("MATCH (g:ShareGrant {spaceId: $id}) DETACH DELETE g").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (r:ArtifactRetention {spaceId: $id}) DETACH DELETE r").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (p:ArtifactCheckpointPrivate {spaceId: $id}) DETACH DELETE p").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (c:ArtifactCheckpoint {spaceId: $id}) DETACH DELETE c").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (i:DocumentIndex {spaceId: $id}) DETACH DELETE i").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (d:DocumentDescriptor {spaceId: $id}) DETACH DELETE d").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (i:SpaceInvite {spaceId: $id}) DETACH DELETE i").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (s:Space {id: $id}) DETACH DELETE s").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                txn.run(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) DELETE m").param("user_id", user_id.clone()).param("space_id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                let scope_key = document_scope_key_v1(&DocumentScope::new(&descriptor.space_id, &descriptor.document_id));
                txn.run(
                    query("MATCH (s:Space {id: $space_id}) MERGE (d:DocumentDescriptor {scopeKey: $scope_key}) ON CREATE SET d.spaceId = $space_id, d.documentId = $document_id, d.descriptor = $descriptor, d.announcedAt = $announced_at MERGE (s)-[:CONTAINS_DOCUMENT]->(d)")
                        .param("space_id", descriptor.space_id.clone())
                        .param("scope_key", scope_key)
                        .param("document_id", descriptor.document_id.clone())
                        .param("descriptor", semio_framework_pack_json::to_json_string(descriptor))
                        .param("announced_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let scope_key = document_scope_key_v1(scope);
                let mut result = txn.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?;
                let descriptor_json: String = row.get("descriptor").map_err(backend)?;
                drop(result);
                let descriptor: DocumentDescriptor = semio_framework_pack_json::from_json_str(&descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?;
                let indexed = semio_hub_directory::document_index_projection_v1(event, &descriptor)?;
                let payload = semio_framework_pack_json::to_json_string(&indexed);
                let mut previous = txn.execute(query("MATCH (i:DocumentIndex {scopeKey: $scope_key}) RETURN i.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                if let Some(row) = previous.next(txn.handle()).await.map_err(backend)? {
                    let stored: String = row.get("payload").map_err(backend)?;
                    if stored != payload {
                        return Err(DirectoryError::Conflict("document index is already bound".into()));
                    }
                }
                drop(previous);
                txn.run(
                    query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) MERGE (i:DocumentIndex {scopeKey: $scope_key}) ON CREATE SET i.spaceId = $space_id, i.documentId = $document_id, i.payload = $payload MERGE (d)-[:INDEXED_AS]->(i)")
                        .param("scope_key", scope_key)
                        .param("space_id", scope.space_id.clone())
                        .param("document_id", scope.document_id.clone())
                        .param("payload", payload),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let scope_key = document_scope_key_v1(&checkpoint.scope);
                let mut descriptors = txn.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let descriptor_row = descriptors.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("checkpoint document descriptor".into()))?;
                let descriptor: DocumentDescriptor = semio_framework_pack_json::from_json_str(&descriptor_row.get::<String>("descriptor").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?;
                drop(descriptors);
                let mut indexes = txn.execute(query("MATCH (i:DocumentIndex {scopeKey: $scope_key}) RETURN i.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let index: Option<directory::os_directory::DirectoryIndexedDocumentViewV1> = match indexes.next(txn.handle()).await.map_err(backend)? {
                    Some(row) => Some(semio_framework_pack_json::from_json_str(&row.get::<String>("payload").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?),
                    None => None,
                };
                drop(indexes);
                semio_hub_directory::validate_checkpoint_index_v1(index.as_ref(), &descriptor, checkpoint)?;
                let mut heads = txn.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:ACTIVE_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let active: Option<PublishedArtifactCheckpoint> = match heads.next(txn.handle()).await.map_err(backend)? {
                    Some(row) => Some(semio_framework_pack_json::from_json_str(&row.get::<String>("payload").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?),
                    None => None,
                };
                drop(heads);
                let mut counts = txn.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN count(c) AS count").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let count: i64 = counts.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("checkpoint count is unavailable".into()))?.get("count").map_err(backend)?;
                drop(counts);
                semio_hub_directory::validate_published_checkpoint_lineage(&descriptor, active.as_ref(), u64::try_from(count).map_err(backend)?, checkpoint)?;
                let checkpoint_key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
                let payload = semio_framework_pack_json::to_json_string(checkpoint);
                txn.run(
                    query(
                        "MATCH (d:DocumentDescriptor {scopeKey: $scope_key})
                         OPTIONAL MATCH (d)-[active:ACTIVE_CHECKPOINT]->(:ArtifactCheckpoint)
                         DELETE active
                         MERGE (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                         ON CREATE SET c.spaceId = $space_id, c.documentId = $document_id, c.checkpointId = $checkpoint_id, c.parentCheckpointId = $parent_checkpoint_id, c.descriptorDigest = $descriptor_digest, c.frontierDocumentId = $frontier_document_id, c.headEditOrdinal = $head_edit_ordinal, c.headEditId = $head_edit_id, c.lastCommitSeq = $last_commit_seq, c.chainHash = $chain_hash, c.packSha256 = $pack_sha256, c.packByteLength = $pack_byte_length, c.sprSha256 = $spr_sha256, c.sprByteLength = $spr_byte_length, c.aggregateSha256 = $aggregate_sha256, c.publishedAt = $published_at, c.eventSeq = $event_seq, c.payload = $payload
                         SET c.payload = $payload
                         MERGE (d)-[:HAS_CHECKPOINT]->(c)
                         MERGE (d)-[:ACTIVE_CHECKPOINT]->(c)",
                    )
                    .param("scope_key", scope_key)
                    .param("checkpoint_key", checkpoint_key)
                    .param("space_id", checkpoint.scope.space_id.clone())
                    .param("document_id", checkpoint.scope.document_id.clone())
                    .param("checkpoint_id", hex_lower(&checkpoint.checkpoint_id.0))
                    .param("parent_checkpoint_id", checkpoint.parent_checkpoint_id.map(|id| hex_lower(&id.0)).unwrap_or_default())
                    .param("descriptor_digest", hex_lower(&checkpoint.descriptor_digest_v1.0))
                    .param("frontier_document_id", checkpoint.baseline_frontier.document_id.clone())
                    .param("head_edit_ordinal", i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?)
                    .param("head_edit_id", checkpoint.baseline_frontier.head_edit_id.clone())
                    .param("last_commit_seq", i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?)
                    .param("chain_hash", hex_lower(&checkpoint.baseline_frontier.chain_hash.0))
                    .param("pack_sha256", hex_lower(&checkpoint.pack.sha256.0))
                    .param("pack_byte_length", i64::try_from(checkpoint.pack.byte_length).map_err(backend)?)
                    .param("spr_sha256", hex_lower(&checkpoint.spr.sha256.0))
                    .param("spr_byte_length", i64::try_from(checkpoint.spr.byte_length).map_err(backend)?)
                    .param("aggregate_sha256", hex_lower(&checkpoint.aggregate_sha256.0))
                    .param("published_at", i64::try_from(checkpoint.published_at_ms).map_err(backend)?)
                    .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
                    .param("payload", payload),
                )
                .await
                .map_err(backend)?;
                if let Some(parent) = checkpoint.parent_checkpoint_id {
                    txn.run(
                        query(
                            "MATCH (p:ArtifactCheckpoint {scopeCheckpointKey: $parent_key}), (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                             MERGE (p)-[:NEXT_CHECKPOINT]->(c)",
                        )
                        .param("parent_key", checkpoint_key_v1(&checkpoint.scope, parent))
                        .param("checkpoint_key", checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id)),
                    )
                    .await
                    .map_err(backend)?;
                }
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let scope_key = document_scope_key_v1(&retention.scope);
                let checkpoint_key = checkpoint_key_v1(&retention.scope, retention.retained_checkpoint_id);
                let payload = semio_framework_pack_json::to_json_string(retention);
                txn.run(
                    query(
                        "MATCH (d:DocumentDescriptor {scopeKey: $scope_key}), (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                         OPTIONAL MATCH (d)-[old:RETENTION_FLOOR]->(:ArtifactCheckpoint)
                         DELETE old
                         MERGE (r:ArtifactRetention {scopeKey: $scope_key})
                         SET r.spaceId = $space_id, r.documentId = $document_id, r.retainedCheckpointId = $retained_checkpoint_id, r.floorDocumentId = $floor_document_id, r.floorHeadEditOrdinal = $floor_head_edit_ordinal, r.floorHeadEditId = $floor_head_edit_id, r.floorLastCommitSeq = $floor_last_commit_seq, r.floorChainHash = $floor_chain_hash, r.checkpointLineageHead = $lineage_head, r.eventSeq = $event_seq, r.payload = $payload
                         MERGE (d)-[:HAS_RETENTION]->(r)
                         MERGE (d)-[:RETENTION_FLOOR]->(c)",
                    )
                    .param("scope_key", scope_key)
                    .param("checkpoint_key", checkpoint_key)
                    .param("space_id", retention.scope.space_id.clone())
                    .param("document_id", retention.scope.document_id.clone())
                    .param("retained_checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                    .param("floor_document_id", retention.retained_floor.document_id.clone())
                    .param("floor_head_edit_ordinal", i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?)
                    .param("floor_head_edit_id", retention.retained_floor.head_edit_id.clone())
                    .param("floor_last_commit_seq", i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?)
                    .param("floor_chain_hash", hex_lower(&retention.retained_floor.chain_hash.0))
                    .param("lineage_head", hex_lower(&retention.checkpoint_lineage_head.0))
                    .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
                    .param("payload", payload),
                )
                .await
                .map_err(backend)?;
            }
        }
        Ok(())
    }
}

impl ProjectionRebuild for Neo4jDirectory {
    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut writer = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        writer.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory rebuild writer lock returned no row".into()))?;
        drop(writer);
        let mut count_result = txn.execute(query("MATCH (e:DirectoryEvent) RETURN count(e) AS count")).await.map_err(backend)?;
        let count: i64 = count_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory event count returned no row".into()))?.get("count").map_err(backend)?;
        drop(count_result);
        let total = u64::try_from(count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        txn.run(query("MATCH (r:ArtifactCasReservation) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (r:ArtifactCasReference) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (r:ArtifactRetention) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (p:ArtifactCheckpointPrivate) DETACH DELETE p")).await.map_err(backend)?;
        txn.run(query("MATCH (c:ArtifactCheckpoint) DETACH DELETE c")).await.map_err(backend)?;
        txn.run(query("MATCH (i:DocumentIndex) DETACH DELETE i")).await.map_err(backend)?;
        txn.run(query("MATCH (d:DocumentDescriptor) DETACH DELETE d")).await.map_err(backend)?;
        txn.run(query("MATCH (s:Space) DETACH DELETE s")).await.map_err(backend)?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        while replayed < total {
            let mut result = txn.execute(query("MATCH (e:DirectoryEvent) WHERE e.seq > $cursor RETURN e AS e ORDER BY e.seq LIMIT 512").param("cursor", cursor)).await.map_err(backend)?;
            let mut events = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                events.push(event_from_node(&row)?);
            }
            drop(result);
            if events.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for event in &events {
                cursor = i64::try_from(event.seq).map_err(backend)?;
                if let Err(error) = self.project(&mut txn, event).await {
                    txn.rollback().await.map_err(backend)?;
                    return Err(DirectoryError::Backend(format!("Neo4j projection rebuild event {}: {error}", event.seq)));
                }
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let mut private = txn.execute(query("MATCH (a:ArtifactAuthorityEvent {eventSeq: $event_seq}) RETURN a.payload AS payload").param("event_seq", cursor)).await.map_err(backend)?;
                    let row = private.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq)))?;
                    let payload: String = row.get("payload").map_err(backend)?;
                    drop(private);
                    let checkpoint: ArtifactCheckpoint = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?;
                    self.project_verified_checkpoint(&mut txn, event, &checkpoint).await?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        let mut ledger_cursor = 0i64;
        loop {
            let mut result = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent) WHERE e.generation > $cursor RETURN e AS e ORDER BY e.generation LIMIT 512").param("cursor", ledger_cursor)).await.map_err(backend)?;
            let mut entries = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                entries.push(row.get::<neo4rs::Node>("e").map_err(backend)?);
            }
            drop(result);
            if entries.is_empty() {
                break;
            }
            for entry in entries {
                ledger_cursor = entry.get("generation").map_err(backend)?;
                let operation: String = entry.get("operation").map_err(backend)?;
                match operation.as_str() {
                    "reserve" | "publish" => {
                        let encoded: neo4rs::BoltBytes = entry.get("plan").map_err(backend)?;
                        let reservation = ArtifactCasReservation::unfenced(
                            decode_artifact_cas_ownership_v1(&encoded.value).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                            u64::try_from(ledger_cursor).map_err(backend)?,
                            u64::try_from(entry.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                            u64::try_from(entry.get::<i64>("expiresAtMs").map_err(backend)?).map_err(backend)?,
                        );
                        if operation == "reserve" {
                            cas_project_reserve(&mut txn, &reservation).await?;
                        } else {
                            cas_project_publish(&mut txn, &reservation, ledger_cursor).await?;
                        }
                    }
                    "retention" => {
                        let scope = DocumentScope::new(entry.get::<String>("spaceId").map_err(backend)?, entry.get::<String>("documentId").map_err(backend)?);
                        let checkpoint_id = ArtifactHash(decode_auth_digest_hex(&entry.get::<String>("checkpointId").map_err(backend)?)?);
                        cas_project_release(&mut txn, "retention", &scope.space_id, Some(&scope), Some(checkpoint_id)).await?;
                    }
                    "space-delete" => {
                        let space_id: String = entry.get("spaceId").map_err(backend)?;
                        cas_project_release(&mut txn, "space-delete", &space_id, None, None).await?;
                    }
                    _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
                }
            }
        }
        txn.commit().await.map_err(backend)?;
        Ok(replayed)
    }
}
//#endregion 🔖️Projections
