//! 🧠️ The collision-checked, dependency-free memory CAS.

use super::*;

/// 🧠 Collision-checked dependency-free memory CAS.
#[derive(Default)]
pub struct MemoryArtifactChunkCasStorage {
    pub state: Mutex<MemoryArtifactCasState>,
}

#[derive(Default)]
struct MemoryArtifactCasState {
    pub(super) coordinator_id: Option<[u8; 32]>,
    physical_epochs: HashMap<ArtifactHash, u64>,
    pub(super) objects: HashMap<(ArtifactHash, ArtifactCasObjectKind, ArtifactHash), Vec<u8>>,
}

impl MemoryArtifactChunkCasStorage {
    fn map_key(key: &ArtifactCasObjectKey) -> Result<(ArtifactHash, ArtifactCasObjectKind, ArtifactHash), AuthorityError> {
        Ok((space_digest(&key.space_id)?, key.kind, key.digest))
    }
}

impl ArtifactChunkCasStorage for MemoryArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match state.coordinator_id {
            Some(current) if current != coordinator_id => Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into())),
            Some(_) => Ok(()),
            None => {
                state.coordinator_id = Some(coordinator_id);
                Ok(())
            }
        }
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let space = space_digest(space_id)?;
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.coordinator_id != Some(coordinator_id) {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        state.physical_epochs.entry(space).and_modify(|current| *current = (*current).max(epoch)).or_insert(epoch);
        Ok(())
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match state.objects.get(&Self::map_key(key)?) {
            Some(existing) if existing == bytes => Ok(ArtifactCasPutOutcome::AlreadyPresent),
            Some(_) => Err(AuthorityError::Store("artifact CAS immutable key collision".to_string())),
            None => {
                state.objects.insert(Self::map_key(key)?, bytes.to_vec());
                Ok(ArtifactCasPutOutcome::Inserted)
            }
        }
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let bytes = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).objects.get(&Self::map_key(key)?).cloned().ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let space = space_digest(&key.space_id)?;
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.coordinator_id.as_ref() != Some(fence.coordinator_id()) || state.physical_epochs.get(&space).copied() != Some(fence.physical_epoch()) {
            return Err(AuthorityError::Store("artifact CAS deletion fence is stale".to_string()));
        }
        let removed = state.objects.remove(&Self::map_key(key)?).is_some();
        Ok(if removed { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}
