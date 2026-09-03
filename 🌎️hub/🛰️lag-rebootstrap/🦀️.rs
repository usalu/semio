//! 🛟️ Verified public-checkpoint reads and bounded lag-rebootstrap transport planning.

use crate::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores};
use crate::artifact_authority::{checkpoint_id_encoding_v1, ArtifactBlobIntegrity, ArtifactPair, AuthorityError, AuthorityLimits, AuthorityOperationControl, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob};
use crate::directory::{published_artifact_checkpoint, HubDirectories, HubDirectory};
use directory::os_directory::{descriptor_digest_v1, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentScope, RebootstrapRequired};
use protocol::{ArtifactBootstrap, ArtifactBootstrapChunkBytes, ArtifactBootstrapPair, RuntimeFrontierSummary, ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, REBOOTSTRAP_SCOPE_MAX_BYTES};
use semio_framework_hash::Sha256;
use std::sync::Arc;

/// 🕰️ Fixed maximum time for one metadata-and-pair rebootstrap read.
pub const REBOOTSTRAP_DEADLINE_MS: u64 = 15_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RebootstrapProgressStage {
    Metadata,
    PackRead,
    SprRead,
    Ready,
    Chunk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RebootstrapProgress {
    pub stage: RebootstrapProgressStage,
    pub completed_units: u64,
    pub total_units: u64,
}

pub trait RebootstrapTransferControl: Send + Sync {
    fn now_ms(&self) -> u64;
    fn is_cancelled(&self) -> bool;
    fn report(&self, progress: RebootstrapProgress);
}

pub struct RebootstrapContext<'a> {
    deadline_ms: u64,
    control: &'a dyn RebootstrapTransferControl,
}

impl<'a> RebootstrapContext<'a> {
    pub const fn new(deadline_ms: u64, control: &'a dyn RebootstrapTransferControl) -> Self {
        Self { deadline_ms, control }
    }

    pub fn checkpoint(&self) -> Result<(), RebootstrapError> {
        if self.control.is_cancelled() {
            return Err(RebootstrapError::Cancelled);
        }
        if self.control.now_ms() >= self.deadline_ms {
            return Err(RebootstrapError::DeadlineExceeded);
        }
        Ok(())
    }

    fn report(&self, progress: RebootstrapProgress) -> Result<(), RebootstrapError> {
        self.control.report(progress);
        self.checkpoint()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RebootstrapError {
    Cancelled,
    DeadlineExceeded,
    Unavailable,
    AuthorityIdentityChanged,
    Integrity,
    ResourceLimit,
}

impl std::fmt::Display for RebootstrapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Cancelled => "rebootstrap cancelled",
            Self::DeadlineExceeded => "rebootstrap deadline exceeded",
            Self::Unavailable => "verified public checkpoint unavailable",
            Self::AuthorityIdentityChanged => "artifact authority identity changed",
            Self::Integrity => "verified public checkpoint integrity failed",
            Self::ResourceLimit => "rebootstrap resource limit exceeded",
        })
    }
}

impl std::error::Error for RebootstrapError {}

struct AuthorityControl<'a> {
    context: &'a RebootstrapContext<'a>,
}

impl AuthorityOperationControl for AuthorityControl<'_> {
    fn now_ms(&self) -> u64 {
        self.context.control.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.context.control.is_cancelled()
    }

    fn report(&self, _progress: crate::artifact_authority::AuthorityProgress) {}
}

fn map_authority(error: AuthorityError) -> RebootstrapError {
    match error {
        AuthorityError::Cancelled => RebootstrapError::Cancelled,
        AuthorityError::DeadlineExceeded => RebootstrapError::DeadlineExceeded,
        AuthorityError::ResourceLimit(_) | AuthorityError::PairResourceLimit(_) => RebootstrapError::ResourceLimit,
        _ => RebootstrapError::Integrity,
    }
}

fn staged(reference: &directory::os_directory::ArtifactBlobRef) -> StagedArtifactBlob {
    StagedArtifactBlob { storage_key: reference.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: reference.sha256, byte_length: reference.byte_length } }
}

fn exact_checkpoint(checkpoint: &ArtifactCheckpoint) -> Result<(), RebootstrapError> {
    let encoded = checkpoint_id_encoding_v1(checkpoint).map_err(map_authority)?;
    if checkpoint.checkpoint_id != ArtifactHash(Sha256::digest(&encoded)) {
        return Err(RebootstrapError::Integrity);
    }
    Ok(())
}

fn wire_frontier(frontier: &ArtifactFrontier) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary {
        document_id: protocol::ArtifactId(frontier.document_id.clone()),
        head_edit_ordinal: frontier.head_edit_ordinal,
        head_edit_id: frontier.head_edit_id.clone(),
        last_commit_seq: frontier.last_commit_seq,
        chain_hash: frontier.chain_hash.0,
    }
}

fn decode_hash(value: &str) -> Result<[u8; 32], RebootstrapError> {
    if value.len() != 64 || value.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(RebootstrapError::AuthorityIdentityChanged);
    }
    let mut output = [0; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        output[index] = nibble(pair[0]) << 4 | nibble(pair[1]);
    }
    if output == [0; 32] {
        return Err(RebootstrapError::AuthorityIdentityChanged);
    }
    Ok(output)
}

pub struct VerifiedArtifactBootstrap {
    pub control: RebootstrapRequired,
    pub bootstrap: ArtifactBootstrap,
    pair: Option<ArtifactPair>,
}

impl VerifiedArtifactBootstrap {
    pub fn chunk(&self, index: u32, context: &RebootstrapContext<'_>) -> Result<Option<ArtifactBootstrapChunkBytes>, RebootstrapError> {
        context.checkpoint()?;
        let Some(pair) = self.pair.as_ref() else { return Ok(None) };
        if index >= self.bootstrap.chunk_count {
            return Ok(None);
        }
        let start = usize::try_from(index).map_err(|_| RebootstrapError::ResourceLimit)?.checked_mul(ARTIFACT_BOOTSTRAP_CHUNK_BYTES).ok_or(RebootstrapError::ResourceLimit)?;
        let total = pair.pack.len().checked_add(pair.spr.len()).ok_or(RebootstrapError::ResourceLimit)?;
        let end = start.checked_add(ARTIFACT_BOOTSTRAP_CHUNK_BYTES).ok_or(RebootstrapError::ResourceLimit)?.min(total);
        let mut chunk = ArtifactBootstrapChunkBytes::allocate_fixed();
        if start < pair.pack.len() {
            let pack_end = end.min(pair.pack.len());
            if !chunk.try_extend_from_slice(&pair.pack[start..pack_end]) {
                return Err(RebootstrapError::ResourceLimit);
            }
        }
        if end > pair.pack.len() {
            let spr_start = start.saturating_sub(pair.pack.len());
            let spr_end = end - pair.pack.len();
            if !chunk.try_extend_from_slice(&pair.spr[spr_start..spr_end]) {
                return Err(RebootstrapError::ResourceLimit);
            }
        }
        if chunk.is_empty() {
            return Err(RebootstrapError::Integrity);
        }
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::Chunk, completed_units: u64::from(index) + 1, total_units: u64::from(self.bootstrap.chunk_count) })?;
        Ok(Some(chunk))
    }
}

pub struct VerifiedRebootstrapSource {
    directory: Arc<HubDirectories>,
    blobs: ArtifactChunkBlobStore<Arc<ArtifactChunkCasStores>>,
}

impl VerifiedRebootstrapSource {
    pub const fn new(directory: Arc<HubDirectories>, storage: Arc<ArtifactChunkCasStores>) -> Self {
        Self { directory, blobs: ArtifactChunkBlobStore::new(storage) }
    }

    async fn verified(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<(directory::os_directory::DocumentDescriptor, ArtifactCheckpoint, RebootstrapRequired), RebootstrapError> {
        context.checkpoint()?;
        if scope.space_id.is_empty()
            || scope.document_id.is_empty()
            || scope.space_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
            || scope.document_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        {
            return Err(RebootstrapError::ResourceLimit);
        }
        let descriptor = self.directory.get_document_descriptor(scope).await.map_err(|_| RebootstrapError::Unavailable)?.ok_or(RebootstrapError::Unavailable)?;
        let active = self.directory.get_active_artifact_checkpoint(scope).await.map_err(|_| RebootstrapError::Unavailable)?.ok_or(RebootstrapError::Unavailable)?;
        let checkpoint = self.directory.get_verified_artifact_checkpoint(scope, active.checkpoint_id).await.map_err(|_| RebootstrapError::Unavailable)?.ok_or(RebootstrapError::Unavailable)?;
        if checkpoint.scope != *scope || published_artifact_checkpoint(&checkpoint) != active {
            return Err(RebootstrapError::Integrity);
        }
        let digest = descriptor_digest_v1(&descriptor).map_err(|_| RebootstrapError::AuthorityIdentityChanged)?;
        if checkpoint.descriptor_digest_v1 != digest {
            return Err(RebootstrapError::AuthorityIdentityChanged);
        }
        exact_checkpoint(&checkpoint)?;
        let control = RebootstrapRequired { scope: scope.clone(), checkpoint_id: checkpoint.checkpoint_id, descriptor_digest_v1: digest, baseline_frontier: checkpoint.baseline_frontier.clone() };
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::Metadata, completed_units: 1, total_units: 4 })?;
        Ok((descriptor, checkpoint, control))
    }

    pub async fn control(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<RebootstrapRequired, RebootstrapError> {
        self.verified(scope, context).await.map(|(_, _, control)| control)
    }

    pub async fn load(&self, scope: &DocumentScope, required_tail_frontier: RuntimeFrontierSummary, context: &RebootstrapContext<'_>) -> Result<VerifiedArtifactBootstrap, RebootstrapError> {
        let (descriptor, checkpoint, control) = self.verified(scope, context).await?;
        if required_tail_frontier.document_id.0 != scope.document_id
            || required_tail_frontier.head_edit_ordinal < checkpoint.baseline_frontier.head_edit_ordinal
            || required_tail_frontier.last_commit_seq < checkpoint.baseline_frontier.last_commit_seq
        {
            return Err(RebootstrapError::Integrity);
        }
        let authority_control = AuthorityControl { context };
        let authority_context = OperationContext::new(context.deadline_ms, AuthorityLimits::maximum(), &authority_control);
        let pack = self.blobs.read(&scope.space_id, &staged(&checkpoint.pack), &authority_context).await.map_err(map_authority)?;
        if ArtifactHash(Sha256::digest(&pack)) != checkpoint.pack.sha256 || pack.len() as u64 != checkpoint.pack.byte_length {
            return Err(RebootstrapError::Integrity);
        }
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::PackRead, completed_units: 2, total_units: 4 })?;
        let spr = self.blobs.read(&scope.space_id, &staged(&checkpoint.spr), &authority_context).await.map_err(map_authority)?;
        if ArtifactHash(Sha256::digest(&spr)) != checkpoint.spr.sha256 || spr.len() as u64 != checkpoint.spr.byte_length {
            return Err(RebootstrapError::Integrity);
        }
        let mut aggregate = Sha256::new();
        aggregate.update(&pack);
        aggregate.update(&spr);
        if ArtifactHash(aggregate.finalize()) != checkpoint.aggregate_sha256 {
            return Err(RebootstrapError::Integrity);
        }
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::SprRead, completed_units: 3, total_units: 4 })?;
        let total = checkpoint.pack.byte_length.checked_add(checkpoint.spr.byte_length).ok_or(RebootstrapError::ResourceLimit)?;
        if total > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES {
            return Err(RebootstrapError::ResourceLimit);
        }
        let inline = total <= ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64;
        let chunk_count = if inline { 0 } else { u32::try_from(total.div_ceil(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64)).map_err(|_| RebootstrapError::ResourceLimit)? };
        if chunk_count > ARTIFACT_BOOTSTRAP_MAX_CHUNKS {
            return Err(RebootstrapError::ResourceLimit);
        }
        let pair = ArtifactPair { pack, spr };
        let bootstrap = ArtifactBootstrap {
            format_version: ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
            descriptor_hash: checkpoint.descriptor_digest_v1.0,
            artifact_schema: descriptor.artifact_schema,
            artifact_kind: descriptor.artifact_kind,
            pack_schema_hash: decode_hash(&descriptor.pack_schema_hash)?,
            baseline_frontier: wire_frontier(&checkpoint.baseline_frontier),
            pack_hash: checkpoint.pack.sha256.0,
            spr_hash: checkpoint.spr.sha256.0,
            pack_length: checkpoint.pack.byte_length,
            spr_length: checkpoint.spr.byte_length,
            chunk_count,
            aggregate_hash: checkpoint.aggregate_sha256.0,
            required_tail_frontier,
            inline: inline.then(|| ArtifactBootstrapPair { pack: pair.pack.clone(), spr: pair.spr.clone() }),
        };
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::Ready, completed_units: 4, total_units: 4 })?;
        Ok(VerifiedArtifactBootstrap { control, bootstrap, pair: (!inline).then_some(pair) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use directory::{DslValue, FromValue};
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::Mutex;

    struct TestControl {
        now: AtomicU64,
        cancelled: AtomicBool,
        progress: Mutex<Vec<RebootstrapProgress>>,
    }

    impl RebootstrapTransferControl for TestControl {
        fn now_ms(&self) -> u64 {
            self.now.load(Ordering::SeqCst)
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }

        fn report(&self, progress: RebootstrapProgress) {
            self.progress.lock().expect("progress lock").push(progress);
        }
    }

    fn test_control() -> TestControl {
        TestControl { now: AtomicU64::new(1), cancelled: AtomicBool::new(false), progress: Mutex::new(Vec::new()) }
    }

    #[tokio::test]
    async fn lag_rebootstrap_neutral_wire_contract() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️lag-rebootstrap/🔣️.json")).expect("fixture JSON");
        let message = directory::os_directory::DirectoryStreamMessage::from_value(DslValue::from(fixture["control"].clone())).expect("directory control");
        let directory::os_directory::DirectoryStreamMessage::RebootstrapRequired { control } = message else { panic!("typed rebootstrap") };
        let public = directory::os_pack::json::to_json_string(&directory::os_directory::DirectoryStreamMessage::RebootstrapRequired { control: control.clone() });
        assert!(!public.contains("storageKey"));
        let frame = protocol::ServerFrame::RebootstrapRequired {
            control: protocol::RebootstrapRequired {
                space_id: control.scope.space_id,
                document_id: control.scope.document_id,
                checkpoint_id: control.checkpoint_id.0,
                descriptor_hash: control.descriptor_digest_v1.0,
                baseline_frontier: wire_frontier(&control.baseline_frontier),
            },
        };
        let encoded = protocol::encode_server_frame(&frame, protocol::Lane::Command).await;
        assert_eq!(encoded.get(1), Some(&12));
        assert_eq!(protocol::decode_server_frame(&encoded).await.expect("decode"), (protocol::Lane::Command, frame));
        assert_eq!(fixture["closeCode"].as_u64(), Some(1013));
        assert_eq!(fixture["closeReason"].as_str(), Some("rebootstrap-required"));
        assert_eq!(fixture["scopeMaximumBytes"].as_u64(), Some(REBOOTSTRAP_SCOPE_MAX_BYTES as u64));
        assert_eq!(fixture["inlineMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64));
        assert_eq!(fixture["chunkMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64));
        assert_eq!(fixture["totalMaximumBytes"].as_u64(), Some(ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES));
        assert_eq!(fixture["chunkMaximumCount"].as_u64(), Some(u64::from(ARTIFACT_BOOTSTRAP_MAX_CHUNKS)));
    }

    #[test]
    fn chunk_boundary_progress_cancel_and_deadline_are_bounded() {
        let control = test_control();
        let context = RebootstrapContext::new(100, &control);
        let scope = DocumentScope::new("space", "document");
        let frontier = ArtifactFrontier { document_id: "document".into(), head_edit_ordinal: 1, head_edit_id: "edit-1".into(), last_commit_seq: 1, chain_hash: ArtifactHash([3; 32]) };
        let public_control = RebootstrapRequired { scope, checkpoint_id: ArtifactHash([1; 32]), descriptor_digest_v1: ArtifactHash([2; 32]), baseline_frontier: frontier.clone() };
        let pair = ArtifactPair { pack: vec![4; ARTIFACT_BOOTSTRAP_CHUNK_BYTES], spr: vec![5] };
        let bootstrap = ArtifactBootstrap {
            format_version: ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
            descriptor_hash: [2; 32],
            artifact_schema: "fixture@1".into(),
            artifact_kind: "fixture".into(),
            pack_schema_hash: [6; 32],
            baseline_frontier: wire_frontier(&frontier),
            pack_hash: [7; 32],
            spr_hash: [8; 32],
            pack_length: ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64,
            spr_length: 1,
            chunk_count: 2,
            aggregate_hash: [9; 32],
            required_tail_frontier: wire_frontier(&frontier),
            inline: None,
        };
        let transfer = VerifiedArtifactBootstrap { control: public_control, bootstrap, pair: Some(pair) };
        assert_eq!(transfer.chunk(0, &context).expect("first").expect("first chunk").len(), ARTIFACT_BOOTSTRAP_CHUNK_BYTES);
        assert_eq!(transfer.chunk(1, &context).expect("second").expect("second chunk").len(), 1);
        assert!(transfer.chunk(2, &context).expect("end").is_none());
        control.cancelled.store(true, Ordering::SeqCst);
        assert_eq!(transfer.chunk(0, &context), Err(RebootstrapError::Cancelled));
        control.cancelled.store(false, Ordering::SeqCst);
        control.now.store(100, Ordering::SeqCst);
        assert_eq!(transfer.chunk(0, &context), Err(RebootstrapError::DeadlineExceeded));
        assert_eq!(control.progress.lock().expect("progress lock").len(), 2);
    }
}
