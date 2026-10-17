//! 🕸️ The dedicated Neo4j artifact CAS and its storage port.

use super::*;

/// 🕸️ Dedicated Neo4j artifact CAS label.
pub struct Neo4jArtifactChunkCasStorage {
    pub(super) graph: neo4rs::Graph,
}

impl Neo4jArtifactChunkCasStorage {
    /// 🔌️ Connects and installs the dedicated object-key uniqueness constraint.
    pub async fn connect(uri: &str, user: &str, password: &str) -> Result<Self, AuthorityError> {
        let graph = neo4rs::Graph::new(uri, user, password).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j connect failed".to_string()))?;
        graph.run(neo4rs::query("CREATE CONSTRAINT IF NOT EXISTS FOR (o:ArtifactCasObject) REQUIRE o.key IS UNIQUE")).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j schema failed".to_string()))?;
        graph.run(neo4rs::query("CREATE CONSTRAINT IF NOT EXISTS FOR (f:ArtifactCasSpaceFence) REQUIRE f.key IS UNIQUE")).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j fence schema failed".into()))?;
        Ok(Self { graph })
    }

    fn key(key: &ArtifactCasObjectKey) -> Result<String, AuthorityError> {
        Ok(format!("{}:{}:{}", hex_lower(&space_digest(&key.space_id)?.0), key.kind.name(), hex_lower(&key.digest.0)))
    }
}

impl ArtifactChunkCasStorage for Neo4jArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let mut result = self
            .graph
            .execute(neo4rs::query("MERGE (c:ArtifactCasCoordinator {id: 'singleton'}) ON CREATE SET c.coordinatorId = $coordinator RETURN c.coordinatorId AS coordinator").param("coordinator", coordinator_id.to_vec()))
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator write failed".into()))?;
        let row = result.next().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator read failed".into()))?.ok_or_else(|| AuthorityError::Store("artifact CAS Neo4j coordinator returned no row".into()))?;
        let current: neo4rs::BoltBytes = row.get("coordinator").map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator decode failed".into()))?;
        if current.value.as_ref() != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        context.checkpoint()
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let space = hex_lower(&space_digest(space_id)?.0);
        let key = format!("{}:{space}", hex_lower(&coordinator_id));
        let mut transaction = self.graph.start_txn().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j transaction failed".into()))?;
        let mut identity = transaction
            .execute(neo4rs::query("MATCH (c:ArtifactCasCoordinator {id: 'singleton'}) SET c.lockNonce = coalesce(c.lockNonce, 0) + 1 RETURN c.coordinatorId AS coordinator"))
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator read failed".into()))?;
        let row = identity.next(transaction.handle()).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator read failed".into()))?.ok_or_else(|| AuthorityError::Store("artifact CAS coordinator is not configured".into()))?;
        let current: neo4rs::BoltBytes = row.get("coordinator").map_err(|_| AuthorityError::Store("artifact CAS Neo4j coordinator decode failed".into()))?;
        if current.value.as_ref() != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        drop(identity);
        transaction.run(neo4rs::query("MERGE (f:ArtifactCasSpaceFence {key: $key}) ON CREATE SET f.coordinatorId = $coordinator, f.spaceDigest = $space, f.physicalEpoch = $epoch, f.updatedAtMs = $now ON MATCH SET f.physicalEpoch = CASE WHEN f.physicalEpoch < $epoch THEN $epoch ELSE f.physicalEpoch END, f.updatedAtMs = $now").param("key", key).param("coordinator", coordinator_id.to_vec()).param("space", space).param("epoch", i64::try_from(epoch).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?).param("now", i64::try_from(context.now_ms()).unwrap_or(i64::MAX))).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j physical epoch advance failed".into()))?;
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j commit failed".into()))?;
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let object_key = Self::key(key)?;
        let mut result = self.graph.execute(neo4rs::query("MERGE (o:ArtifactCasObject {key: $key}) ON CREATE SET o.spaceDigest = $space, o.kind = $kind, o.objectDigest = $digest, o.bytes = $bytes, o.byteLength = $length, o.inserted = true ON MATCH SET o.inserted = false RETURN o.byteLength AS length, o.bytes AS bytes, o.inserted AS inserted")
            .param("key", object_key).param("space", hex_lower(&space_digest(&key.space_id)?.0)).param("kind", key.kind.name()).param("digest", hex_lower(&key.digest.0)).param("bytes", bytes.to_vec()).param("length", bytes.len() as i64)).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j insert failed".to_string()))?;
        let row = result.next().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j readback failed".to_string()))?.ok_or_else(|| AuthorityError::Store("artifact CAS Neo4j returned no object".to_string()))?;
        let length: i64 = row.get("length").map_err(|_| AuthorityError::Store("artifact CAS Neo4j length decode failed".to_string()))?;
        let stored: neo4rs::BoltBytes = row.get("bytes").map_err(|_| AuthorityError::Store("artifact CAS Neo4j bytes decode failed".to_string()))?;
        let inserted: bool = row.get("inserted").map_err(|_| AuthorityError::Store("artifact CAS Neo4j insertion decode failed".to_string()))?;
        if length != bytes.len() as i64 || stored.value.as_ref() != bytes {
            return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
        }
        context.checkpoint()?;
        Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let mut result = self
            .graph
            .execute(neo4rs::query("MATCH (o:ArtifactCasObject {key: $key}) RETURN o.byteLength AS length, o.bytes AS bytes").param("key", Self::key(key)?))
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS Neo4j read failed".to_string()))?;
        let row = result.next().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j read failed".to_string()))?.ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        let length: i64 = row.get("length").map_err(|_| AuthorityError::Store("artifact CAS Neo4j length decode failed".to_string()))?;
        if length < 0 || usize::try_from(length).ok().is_none_or(|length| length > key.kind.maximum_bytes()) {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let stored: neo4rs::BoltBytes = row.get("bytes").map_err(|_| AuthorityError::Store("artifact CAS Neo4j bytes decode failed".to_string()))?;
        let bytes = stored.value.to_vec();
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let space = hex_lower(&space_digest(&key.space_id)?.0);
        let fence_key = format!("{}:{space}", hex_lower(fence.coordinator_id()));
        let mut transaction = self.graph.start_txn().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j transaction failed".into()))?;
        let mut result = transaction.execute(neo4rs::query("MATCH (f:ArtifactCasSpaceFence {key: $fence_key}) WHERE f.physicalEpoch = $epoch OPTIONAL MATCH (o:ArtifactCasObject {key: $object_key}) WITH f, collect(o) AS objects, count(o) AS deleted FOREACH (object IN objects | DELETE object) RETURN deleted").param("fence_key", fence_key).param("epoch", i64::try_from(fence.physical_epoch()).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?).param("object_key", Self::key(key)?)).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j delete failed".to_string()))?;
        let row = result.next(transaction.handle()).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j delete failed".to_string()))?.ok_or_else(|| AuthorityError::Store("artifact CAS deletion fence is stale".to_string()))?;
        let deleted: i64 = row.get("deleted").map_err(|_| AuthorityError::Store("artifact CAS Neo4j deletion decode failed".to_string()))?;
        drop(result);
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j commit failed".into()))?;
        Ok(if deleted == 0 { ArtifactCasDeleteOutcome::Missing } else { ArtifactCasDeleteOutcome::Deleted })
    }
}
