//! 🫙️ The manifest-based immutable blob adapter used by publication and rebootstrap.

use super::*;

/// 🫙️ Manifest-based immutable blob adapter used by both publication and rebootstrap.
pub struct ArtifactChunkBlobStore<S> {
    pub storage: S,
}

impl<S> ArtifactChunkBlobStore<S> {
    /// 🧩️ Binds the raw-blob adapter to one dedicated CAS.
    pub const fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: ArtifactChunkCasStorage> ArtifactChunkBlobStore<S> {
    async fn stage_scoped(&self, space_id: &str, expected: ArtifactBlobIntegrity, bytes: &[u8], context: &OperationContext<'_>) -> Result<StagedArtifactBlob, AuthorityError> {
        context.checkpoint()?;
        let plan = prepare_artifact_cas_manifest_v1(space_id, bytes)?;
        if expected.sha256 != plan.manifest.raw_sha256 || expected.byte_length != plan.manifest.raw_byte_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS stage input"));
        }
        let total = u64::try_from(plan.manifest.chunks.len() + 1).map_err(|_| AuthorityError::ResourceLimit("artifact CAS progress"))?;
        for (index, (record, chunk)) in plan.manifest.chunks.iter().zip(bytes.chunks(ARTIFACT_CAS_CHUNK_BYTES)).enumerate() {
            context.checkpoint()?;
            let key = ArtifactCasObjectKey { space_id: space_id.to_string(), kind: ArtifactCasObjectKind::Chunk, digest: record.chunk_id };
            self.storage.put_if_absent(&key, chunk, context).await?;
            context.report(AuthorityProgress { stage: AuthorityProgressStage::CasChunkStored, completed_units: index as u64, total_units: total })?;
            if self.storage.get(&key, context).await? != chunk {
                return Err(AuthorityError::BlobIntegrity("artifact CAS chunk readback"));
            }
            context.report(AuthorityProgress { stage: AuthorityProgressStage::CasChunkVerified, completed_units: index as u64 + 1, total_units: total })?;
            semio_framework_async::yield_once().await;
        }
        context.checkpoint()?;
        let manifest_key = ArtifactCasObjectKey { space_id: space_id.to_string(), kind: ArtifactCasObjectKind::Manifest, digest: plan.manifest_id };
        self.storage.put_if_absent(&manifest_key, &plan.manifest_bytes, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CasManifestStored, completed_units: total - 1, total_units: total })?;
        if self.storage.get(&manifest_key, context).await? != plan.manifest_bytes {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest readback"));
        }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CasManifestVerified, completed_units: total, total_units: total })?;
        Ok(StagedArtifactBlob { storage_key: artifact_cas_manifest_locator_v1(plan.manifest_id), integrity: expected })
    }

    async fn read_scoped(&self, space_id: &str, staged: &StagedArtifactBlob, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let manifest_id = decode_artifact_cas_manifest_locator_v1(&staged.storage_key)?;
        let manifest_key = ArtifactCasObjectKey { space_id: space_id.to_string(), kind: ArtifactCasObjectKind::Manifest, digest: manifest_id };
        let manifest_bytes = self.storage.get(&manifest_key, context).await?;
        let manifest = decode_artifact_cas_manifest_v1(&manifest_bytes, space_id, manifest_id)?;
        if manifest.raw_sha256 != staged.integrity.sha256 || manifest.raw_byte_length != staged.integrity.byte_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest raw identity"));
        }
        let capacity = usize::try_from(manifest.raw_byte_length).map_err(|_| AuthorityError::ResourceLimit("artifact CAS raw byte"))?;
        let mut raw = Vec::with_capacity(capacity);
        let mut hash = Sha256::new();
        for record in &manifest.chunks {
            context.checkpoint()?;
            let key = ArtifactCasObjectKey { space_id: space_id.to_string(), kind: ArtifactCasObjectKind::Chunk, digest: record.chunk_id };
            let chunk = self.storage.get(&key, context).await?;
            if chunk.len() != record.byte_length as usize || chunk_id(space_id, &chunk)? != record.chunk_id {
                return Err(AuthorityError::BlobIntegrity("artifact CAS chunk reconstruction"));
            }
            hash.update(&chunk);
            raw.extend_from_slice(&chunk);
            semio_framework_async::yield_once().await;
        }
        if raw.len() as u64 != manifest.raw_byte_length || ArtifactHash(hash.finalize()) != manifest.raw_sha256 {
            return Err(AuthorityError::BlobIntegrity("artifact CAS raw reconstruction"));
        }
        context.checkpoint()?;
        Ok(raw)
    }
}

impl<S: ArtifactChunkCasStorage> ImmutableArtifactBlobStore for ArtifactChunkBlobStore<S> {
    async fn stage(&self, space_id: &str, expected: ArtifactBlobIntegrity, bytes: &[u8], context: &OperationContext<'_>) -> Result<StagedArtifactBlob, AuthorityError> {
        self.stage_scoped(space_id, expected, bytes, context).await
    }

    async fn read(&self, space_id: &str, staged: &StagedArtifactBlob, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.read_scoped(space_id, staged, context).await
    }
}
