//! 🛟️ Verified public-checkpoint reads and bounded lag-rebootstrap transport planning.

use crate::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores};
use crate::artifact_authority::{checkpoint_id_encoding_v1, ArtifactBlobIntegrity, ArtifactPair, AuthorityError, AuthorityLimits, AuthorityOperationControl, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob, AUTHORITY_MAX_PAIR_BYTES};
use crate::directory::{published_artifact_checkpoint, HubDirectories, HubDirectory};
use directory::os_directory::{descriptor_digest_v1, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentScope, PublishedArtifactBlob, RebootstrapRequired};
use protocol::{
    ArtifactBootstrap, ArtifactBootstrapChunkBytes, ArtifactBootstrapPair, RuntimeFrontierSummary, ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES,
    REBOOTSTRAP_SCOPE_MAX_BYTES,
};
use semio_framework_hash::Sha256;
use std::sync::Arc;

/// 🕰️ Fixed maximum time for one metadata-and-pair rebootstrap read.
pub const REBOOTSTRAP_DEADLINE_MS: u64 = 15_000;
pub const CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE: &str = "application/vnd.semio.canonical-checkpoint-pair.v1";
pub const CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES: usize = 16 * 1024;
pub const CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES: usize = 4 * 1024;
pub const CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS: u32 = 16_384;
const CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION: u32 = 1;
const CANONICAL_CHECKPOINT_PAIR_ETAG_DOMAIN: &[u8] = b"semio.hub.canonical-checkpoint-pair-etag.v1\0";
const PAIR_HEADER: u8 = 1;
const PAIR_DATA: u8 = 2;
const PAIR_TERMINAL: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RebootstrapProgressStage {
    Authorize,
    Metadata,
    VerifyPack,
    VerifySpr,
    StreamPack,
    StreamSpr,
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
    pair_reader: VerifiedActiveCheckpointPairReader,
}

impl VerifiedRebootstrapSource {
    pub const fn new(directory: Arc<HubDirectories>, storage: Arc<ArtifactChunkCasStores>) -> Self {
        Self { pair_reader: VerifiedActiveCheckpointPairReader::new(directory, storage) }
    }

    async fn verified(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<(directory::os_directory::DocumentDescriptor, ArtifactCheckpoint, RebootstrapRequired), RebootstrapError> {
        self.pair_reader.verified(scope, context).await
    }

    pub async fn control(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<RebootstrapRequired, RebootstrapError> {
        self.verified(scope, context).await.map(|(_, _, control)| control)
    }

    pub async fn active_pair(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<VerifiedActiveCheckpointPair, RebootstrapError> {
        self.pair_reader.read(scope, context).await
    }

    pub async fn load(&self, scope: &DocumentScope, required_tail_frontier: RuntimeFrontierSummary, context: &RebootstrapContext<'_>) -> Result<VerifiedArtifactBootstrap, RebootstrapError> {
        let (descriptor, checkpoint, control) = self.verified(scope, context).await?;
        if required_tail_frontier.document_id.0 != scope.document_id || required_tail_frontier.head_edit_ordinal < checkpoint.baseline_frontier.head_edit_ordinal || required_tail_frontier.last_commit_seq < checkpoint.baseline_frontier.last_commit_seq
        {
            return Err(RebootstrapError::Integrity);
        }
        let verified = self.pair_reader.read_selected(scope, &checkpoint, context).await?;
        let pair = verified.pair;
        let total = checkpoint.pack.byte_length.checked_add(checkpoint.spr.byte_length).ok_or(RebootstrapError::ResourceLimit)?;
        let inline = total <= ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64;
        let chunk_count = if inline { 0 } else { u32::try_from(total.div_ceil(ARTIFACT_BOOTSTRAP_CHUNK_BYTES as u64)).map_err(|_| RebootstrapError::ResourceLimit)? };
        if chunk_count > ARTIFACT_BOOTSTRAP_MAX_CHUNKS {
            return Err(RebootstrapError::ResourceLimit);
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalCheckpointPairSelection {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: ArtifactHash,
    pub active_checkpoint_id: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: PublishedArtifactBlob,
    pub spr: PublishedArtifactBlob,
    pub aggregate_sha256: ArtifactHash,
}

pub struct VerifiedActiveCheckpointPair {
    pub selection: CanonicalCheckpointPairSelection,
    pair: ArtifactPair,
}

impl VerifiedActiveCheckpointPair {
    pub fn pair(&self) -> &ArtifactPair {
        &self.pair
    }

    pub fn data_record_count(&self) -> u32 {
        pair_record_count(self.pair.pack.len() as u64, self.pair.spr.len() as u64).expect("verified pair record count")
    }

    pub fn data_record(&self, ordinal: u32, context: &RebootstrapContext<'_>) -> Result<Option<CanonicalPairDataRecord<'_>>, RebootstrapError> {
        context.checkpoint()?;
        let pack_records = u32::try_from(self.pair.pack.len().div_ceil(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES)).map_err(|_| RebootstrapError::ResourceLimit)?;
        let (part, part_ordinal, bytes) = if ordinal < pack_records { (CanonicalPairPart::Pack, ordinal, self.pair.pack.as_slice()) } else { (CanonicalPairPart::Spr, ordinal.saturating_sub(pack_records), self.pair.spr.as_slice()) };
        let start = usize::try_from(part_ordinal).map_err(|_| RebootstrapError::ResourceLimit)?.checked_mul(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES).ok_or(RebootstrapError::ResourceLimit)?;
        if start >= bytes.len() {
            return Ok(None);
        }
        let end = start.checked_add(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES).ok_or(RebootstrapError::ResourceLimit)?.min(bytes.len());
        context.report(RebootstrapProgress {
            stage: match part {
                CanonicalPairPart::Pack => RebootstrapProgressStage::StreamPack,
                CanonicalPairPart::Spr => RebootstrapProgressStage::StreamSpr,
            },
            completed_units: u64::from(ordinal) + 1,
            total_units: u64::from(self.data_record_count()),
        })?;
        Ok(Some(CanonicalPairDataRecord { part, ordinal, byte_offset: start as u64, bytes: &bytes[start..end] }))
    }
}

pub struct VerifiedActiveCheckpointPairReader {
    directory: Arc<HubDirectories>,
    blobs: ArtifactChunkBlobStore<Arc<ArtifactChunkCasStores>>,
}

impl VerifiedActiveCheckpointPairReader {
    pub const fn new(directory: Arc<HubDirectories>, storage: Arc<ArtifactChunkCasStores>) -> Self {
        Self { directory, blobs: ArtifactChunkBlobStore::new(storage) }
    }

    async fn verified(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<(directory::os_directory::DocumentDescriptor, ArtifactCheckpoint, RebootstrapRequired), RebootstrapError> {
        context.checkpoint()?;
        if scope.space_id.is_empty() || scope.document_id.is_empty() || scope.space_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES || scope.document_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES {
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
        preflight_pair(checkpoint.pack.byte_length, checkpoint.spr.byte_length)?;
        let control = RebootstrapRequired { scope: scope.clone(), checkpoint_id: checkpoint.checkpoint_id, descriptor_digest_v1: digest, baseline_frontier: checkpoint.baseline_frontier.clone() };
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::Metadata, completed_units: 1, total_units: 4 })?;
        Ok((descriptor, checkpoint, control))
    }

    async fn read_selected(&self, scope: &DocumentScope, checkpoint: &ArtifactCheckpoint, context: &RebootstrapContext<'_>) -> Result<VerifiedActiveCheckpointPair, RebootstrapError> {
        preflight_pair(checkpoint.pack.byte_length, checkpoint.spr.byte_length)?;
        let authority_control = AuthorityControl { context };
        let authority_context = OperationContext::new(context.deadline_ms, AuthorityLimits::maximum(), &authority_control);
        let pack = self.blobs.read(&scope.space_id, &staged(&checkpoint.pack), &authority_context).await.map_err(map_authority)?;
        if ArtifactHash(Sha256::digest(&pack)) != checkpoint.pack.sha256 || pack.len() as u64 != checkpoint.pack.byte_length {
            return Err(RebootstrapError::Integrity);
        }
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::VerifyPack, completed_units: 2, total_units: 4 })?;
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
        context.report(RebootstrapProgress { stage: RebootstrapProgressStage::VerifySpr, completed_units: 3, total_units: 4 })?;
        Ok(VerifiedActiveCheckpointPair {
            selection: CanonicalCheckpointPairSelection {
                scope: scope.clone(),
                descriptor_digest_v1: checkpoint.descriptor_digest_v1,
                active_checkpoint_id: checkpoint.checkpoint_id,
                baseline_frontier: checkpoint.baseline_frontier.clone(),
                pack: PublishedArtifactBlob { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length },
                spr: PublishedArtifactBlob { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length },
                aggregate_sha256: checkpoint.aggregate_sha256,
            },
            pair: ArtifactPair { pack, spr },
        })
    }

    pub async fn read(&self, scope: &DocumentScope, context: &RebootstrapContext<'_>) -> Result<VerifiedActiveCheckpointPair, RebootstrapError> {
        let (_, checkpoint, _) = self.verified(scope, context).await?;
        self.read_selected(scope, &checkpoint, context).await
    }
}

fn pair_record_count(pack_length: u64, spr_length: u64) -> Result<u32, RebootstrapError> {
    let chunk = CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES as u64;
    let count = pack_length.div_ceil(chunk).checked_add(spr_length.div_ceil(chunk)).ok_or(RebootstrapError::ResourceLimit)?;
    u32::try_from(count).map_err(|_| RebootstrapError::ResourceLimit)
}

fn preflight_pair(pack_length: u64, spr_length: u64) -> Result<u64, RebootstrapError> {
    if pack_length == 0 || spr_length == 0 {
        return Err(RebootstrapError::Integrity);
    }
    let total = pack_length.checked_add(spr_length).ok_or(RebootstrapError::ResourceLimit)?;
    if total > AUTHORITY_MAX_PAIR_BYTES || total > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES || pair_record_count(pack_length, spr_length)? > CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS {
        return Err(RebootstrapError::ResourceLimit);
    }
    Ok(total)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CanonicalPairPart {
    Pack = 1,
    Spr = 2,
}

pub struct CanonicalPairDataRecord<'a> {
    pub part: CanonicalPairPart,
    pub ordinal: u32,
    pub byte_offset: u64,
    pub bytes: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CanonicalPairTerminal {
    Complete = 0,
    Cancelled = 1,
    Unavailable = 2,
    Integrity = 3,
    Deadline = 4,
}

fn append_length_prefixed(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), RebootstrapError> {
    let length = u32::try_from(bytes.len()).map_err(|_| RebootstrapError::ResourceLimit)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn append_frame(output: &mut Vec<u8>, payload: &[u8]) -> Result<(), RebootstrapError> {
    append_length_prefixed(output, payload)
}

fn canonical_pair_header_payload(selection: &CanonicalCheckpointPairSelection) -> Result<Vec<u8>, RebootstrapError> {
    if selection.scope.space_id.is_empty()
        || selection.scope.document_id.is_empty()
        || selection.scope.space_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || selection.scope.document_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || selection.baseline_frontier.document_id != selection.scope.document_id
    {
        return Err(RebootstrapError::Integrity);
    }
    preflight_pair(selection.pack.byte_length, selection.spr.byte_length)?;
    let mut output = Vec::with_capacity(512);
    output.push(PAIR_HEADER);
    output.extend_from_slice(&CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION.to_be_bytes());
    append_length_prefixed(&mut output, selection.scope.space_id.as_bytes())?;
    append_length_prefixed(&mut output, selection.scope.document_id.as_bytes())?;
    output.extend_from_slice(&selection.descriptor_digest_v1.0);
    output.extend_from_slice(&selection.active_checkpoint_id.0);
    append_length_prefixed(&mut output, selection.baseline_frontier.document_id.as_bytes())?;
    output.extend_from_slice(&selection.baseline_frontier.head_edit_ordinal.to_be_bytes());
    append_length_prefixed(&mut output, selection.baseline_frontier.head_edit_id.as_bytes())?;
    output.extend_from_slice(&selection.baseline_frontier.last_commit_seq.to_be_bytes());
    output.extend_from_slice(&selection.baseline_frontier.chain_hash.0);
    output.extend_from_slice(&selection.pack.sha256.0);
    output.extend_from_slice(&selection.pack.byte_length.to_be_bytes());
    output.extend_from_slice(&selection.spr.sha256.0);
    output.extend_from_slice(&selection.spr.byte_length.to_be_bytes());
    output.extend_from_slice(&selection.aggregate_sha256.0);
    if output.len() > CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES {
        return Err(RebootstrapError::ResourceLimit);
    }
    Ok(output)
}

pub fn append_canonical_pair_header(output: &mut Vec<u8>, selection: &CanonicalCheckpointPairSelection) -> Result<(), RebootstrapError> {
    append_frame(output, &canonical_pair_header_payload(selection)?)
}

pub fn append_canonical_pair_data(output: &mut Vec<u8>, record: &CanonicalPairDataRecord<'_>) -> Result<(), RebootstrapError> {
    if record.bytes.is_empty() || record.bytes.len() > CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES {
        return Err(RebootstrapError::Integrity);
    }
    let mut payload = Vec::with_capacity(18 + record.bytes.len());
    payload.push(PAIR_DATA);
    payload.push(record.part as u8);
    payload.extend_from_slice(&record.ordinal.to_be_bytes());
    payload.extend_from_slice(&record.byte_offset.to_be_bytes());
    append_length_prefixed(&mut payload, record.bytes)?;
    append_frame(output, &payload)
}

pub fn append_canonical_pair_terminal(output: &mut Vec<u8>, terminal: CanonicalPairTerminal) -> Result<(), RebootstrapError> {
    append_frame(output, &[PAIR_TERMINAL, terminal as u8])
}

fn hex_lower(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 15) as usize] as char);
    }
    output
}

pub fn canonical_pair_etag(selection: &CanonicalCheckpointPairSelection) -> Result<String, RebootstrapError> {
    let mut hash = Sha256::new();
    hash.update(CANONICAL_CHECKPOINT_PAIR_ETAG_DOMAIN);
    hash.update(&canonical_pair_header_payload(selection)?);
    Ok(format!("\"{}\"", hex_lower(&hash.finalize())))
}

pub fn encode_verified_canonical_pair(pair: &VerifiedActiveCheckpointPair, context: &RebootstrapContext<'_>) -> Result<Vec<u8>, RebootstrapError> {
    let record_count = pair.data_record_count();
    let total = preflight_pair(pair.selection.pack.byte_length, pair.selection.spr.byte_length)?;
    let overhead =
        usize::try_from(record_count).map_err(|_| RebootstrapError::ResourceLimit)?.checked_mul(22).ok_or(RebootstrapError::ResourceLimit)?.checked_add(CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES + 6).ok_or(RebootstrapError::ResourceLimit)?;
    let capacity = usize::try_from(total).map_err(|_| RebootstrapError::ResourceLimit)?.checked_add(overhead).ok_or(RebootstrapError::ResourceLimit)?;
    let mut output = Vec::with_capacity(capacity);
    append_canonical_pair_header(&mut output, &pair.selection)?;
    for ordinal in 0..record_count {
        let record = pair.data_record(ordinal, context)?.ok_or(RebootstrapError::Integrity)?;
        append_canonical_pair_data(&mut output, &record)?;
    }
    append_canonical_pair_terminal(&mut output, CanonicalPairTerminal::Complete)?;
    Ok(output)
}

struct PairCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> PairCursor<'a> {
    fn take(&mut self, length: usize) -> Result<&'a [u8], RebootstrapError> {
        let end = self.offset.checked_add(length).ok_or(RebootstrapError::ResourceLimit)?;
        let value = self.bytes.get(self.offset..end).ok_or(RebootstrapError::Integrity)?;
        self.offset = end;
        Ok(value)
    }

    fn byte(&mut self) -> Result<u8, RebootstrapError> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, RebootstrapError> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().map_err(|_| RebootstrapError::Integrity)?))
    }

    fn u64(&mut self) -> Result<u64, RebootstrapError> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().map_err(|_| RebootstrapError::Integrity)?))
    }

    fn text(&mut self, maximum: usize) -> Result<String, RebootstrapError> {
        let length = usize::try_from(self.u32()?).map_err(|_| RebootstrapError::ResourceLimit)?;
        if length == 0 || length > maximum {
            return Err(RebootstrapError::ResourceLimit);
        }
        std::str::from_utf8(self.take(length)?).map(str::to_owned).map_err(|_| RebootstrapError::Integrity)
    }

    fn hash(&mut self) -> Result<ArtifactHash, RebootstrapError> {
        let hash = ArtifactHash(self.take(32)?.try_into().map_err(|_| RebootstrapError::Integrity)?);
        if hash.0 == [0; 32] {
            Err(RebootstrapError::Integrity)
        } else {
            Ok(hash)
        }
    }
}

fn next_frame<'a>(cursor: &mut PairCursor<'a>, maximum: usize) -> Result<&'a [u8], RebootstrapError> {
    let length = usize::try_from(cursor.u32()?).map_err(|_| RebootstrapError::ResourceLimit)?;
    if length == 0 || length > maximum {
        return Err(RebootstrapError::ResourceLimit);
    }
    cursor.take(length)
}

pub fn decode_canonical_checkpoint_pair(input: &[u8]) -> Result<VerifiedActiveCheckpointPair, RebootstrapError> {
    let maximum_wire = usize::try_from(AUTHORITY_MAX_PAIR_BYTES)
        .map_err(|_| RebootstrapError::ResourceLimit)?
        .checked_add(CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES)
        .and_then(|value| value.checked_add(CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS as usize * 22 + 6))
        .ok_or(RebootstrapError::ResourceLimit)?;
    if input.len() > maximum_wire {
        return Err(RebootstrapError::ResourceLimit);
    }
    let mut input_cursor = PairCursor { bytes: input, offset: 0 };
    let header = next_frame(&mut input_cursor, CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES)?;
    let mut cursor = PairCursor { bytes: header, offset: 0 };
    if cursor.byte()? != PAIR_HEADER || cursor.u32()? != CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION {
        return Err(RebootstrapError::Integrity);
    }
    let space_id = cursor.text(REBOOTSTRAP_SCOPE_MAX_BYTES)?;
    let document_id = cursor.text(REBOOTSTRAP_SCOPE_MAX_BYTES)?;
    let descriptor_digest_v1 = cursor.hash()?;
    let active_checkpoint_id = cursor.hash()?;
    let frontier_document_id = cursor.text(REBOOTSTRAP_SCOPE_MAX_BYTES)?;
    let head_edit_ordinal = cursor.u64()?;
    let head_edit_id = cursor.text(REBOOTSTRAP_SCOPE_MAX_BYTES)?;
    let last_commit_seq = cursor.u64()?;
    let chain_hash = cursor.hash()?;
    let pack_hash = cursor.hash()?;
    let pack_length = cursor.u64()?;
    let spr_hash = cursor.hash()?;
    let spr_length = cursor.u64()?;
    let aggregate_sha256 = cursor.hash()?;
    if cursor.offset != header.len() || frontier_document_id != document_id {
        return Err(RebootstrapError::Integrity);
    }
    let total = preflight_pair(pack_length, spr_length)?;
    let mut pack = Vec::with_capacity(usize::try_from(pack_length).map_err(|_| RebootstrapError::ResourceLimit)?);
    let mut spr = Vec::with_capacity(usize::try_from(spr_length).map_err(|_| RebootstrapError::ResourceLimit)?);
    let expected_records = pair_record_count(pack_length, spr_length)?;
    for ordinal in 0..expected_records {
        let frame = next_frame(&mut input_cursor, CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES + 18)?;
        let mut record = PairCursor { bytes: frame, offset: 0 };
        if record.byte()? != PAIR_DATA {
            return Err(RebootstrapError::Integrity);
        }
        let part = record.byte()?;
        if record.u32()? != ordinal {
            return Err(RebootstrapError::Integrity);
        }
        let offset = record.u64()?;
        let length = usize::try_from(record.u32()?).map_err(|_| RebootstrapError::ResourceLimit)?;
        if length == 0 || length > CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES {
            return Err(RebootstrapError::Integrity);
        }
        let bytes = record.take(length)?;
        if record.offset != frame.len() {
            return Err(RebootstrapError::Integrity);
        }
        let target = match part {
            1 if pack.len() < pack_length as usize => &mut pack,
            2 if pack.len() == pack_length as usize => &mut spr,
            _ => return Err(RebootstrapError::Integrity),
        };
        if offset != target.len() as u64 {
            return Err(RebootstrapError::Integrity);
        }
        target.extend_from_slice(bytes);
    }
    let terminal = next_frame(&mut input_cursor, 2)?;
    if terminal != [PAIR_TERMINAL, CanonicalPairTerminal::Complete as u8] || input_cursor.offset != input.len() || pack.len() as u64 != pack_length || spr.len() as u64 != spr_length {
        return Err(RebootstrapError::Integrity);
    }
    if ArtifactHash(Sha256::digest(&pack)) != pack_hash || ArtifactHash(Sha256::digest(&spr)) != spr_hash {
        return Err(RebootstrapError::Integrity);
    }
    let mut aggregate = Sha256::new();
    aggregate.update(&pack);
    aggregate.update(&spr);
    if ArtifactHash(aggregate.finalize()) != aggregate_sha256 || total != pack_length + spr_length {
        return Err(RebootstrapError::Integrity);
    }
    let selection = CanonicalCheckpointPairSelection {
        scope: DocumentScope::new(space_id, document_id),
        descriptor_digest_v1,
        active_checkpoint_id,
        baseline_frontier: ArtifactFrontier { document_id: frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash },
        pack: PublishedArtifactBlob { sha256: pack_hash, byte_length: pack_length },
        spr: PublishedArtifactBlob { sha256: spr_hash, byte_length: spr_length },
        aggregate_sha256,
    };
    canonical_pair_header_payload(&selection)?;
    Ok(VerifiedActiveCheckpointPair { selection, pair: ArtifactPair { pack, spr } })
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

    fn canonical_pair_fixture() -> VerifiedActiveCheckpointPair {
        let pack: Vec<u8> = (0..5_001).map(|index| ((index * 17 + 3) % 256) as u8).collect();
        let spr: Vec<u8> = (0..4_097).map(|index| ((index * 29 + 7) % 256) as u8).collect();
        let mut aggregate = Sha256::new();
        aggregate.update(&pack);
        aggregate.update(&spr);
        VerifiedActiveCheckpointPair {
            selection: CanonicalCheckpointPairSelection {
                scope: DocumentScope::new("space:alpha", "doc:tokyo"),
                descriptor_digest_v1: ArtifactHash([0x11; 32]),
                active_checkpoint_id: ArtifactHash([0x22; 32]),
                baseline_frontier: ArtifactFrontier { document_id: "doc:tokyo".into(), head_edit_ordinal: 7, head_edit_id: "edit:7".into(), last_commit_seq: 6, chain_hash: ArtifactHash([0x33; 32]) },
                pack: PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pack)), byte_length: pack.len() as u64 },
                spr: PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&spr)), byte_length: spr.len() as u64 },
                aggregate_sha256: ArtifactHash(aggregate.finalize()),
            },
            pair: ArtifactPair { pack, spr },
        }
    }

    #[test]
    fn canonical_pair_preflight_is_before_allocation_and_record_bounded() {
        assert_eq!(preflight_pair(4_096, AUTHORITY_MAX_PAIR_BYTES - 4_096), Ok(AUTHORITY_MAX_PAIR_BYTES));
        assert_eq!(preflight_pair(1, AUTHORITY_MAX_PAIR_BYTES - 1), Err(RebootstrapError::ResourceLimit));
        assert_eq!(preflight_pair(0, 1), Err(RebootstrapError::Integrity));
        assert_eq!(preflight_pair(u64::MAX, 1), Err(RebootstrapError::ResourceLimit));
    }

    #[test]
    fn canonical_pair_neutral_framing_is_pack_then_spr_terminal_and_fail_closed() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🪢️canonical-pair/🔣️.json")).expect("canonical pair fixture");
        let pair = canonical_pair_fixture();
        let control = test_control();
        let context = RebootstrapContext::new(100, &control);
        let encoded = encode_verified_canonical_pair(&pair, &context).expect("encode canonical pair");
        let decoded = decode_canonical_checkpoint_pair(&encoded).expect("decode canonical pair");
        assert_eq!(decoded.selection, pair.selection);
        assert_eq!(decoded.pair(), pair.pair());
        assert_eq!(decoded.data_record_count(), fixture["expected"]["dataRecords"].as_u64().expect("data records") as u32);
        assert_eq!(encoded.len(), fixture["expected"]["wireBytes"].as_u64().expect("wire bytes") as usize);
        assert_eq!(hex_lower(&decoded.selection.pack.sha256.0), fixture["pack"]["sha256"]);
        assert_eq!(hex_lower(&decoded.selection.spr.sha256.0), fixture["spr"]["sha256"]);
        assert_eq!(hex_lower(&decoded.selection.aggregate_sha256.0), fixture["expected"]["aggregateSha256"]);
        assert_eq!(canonical_pair_etag(&decoded.selection).expect("etag"), fixture["expected"]["etag"]);

        let mut no_terminal = encoded.clone();
        no_terminal.truncate(no_terminal.len() - 6);
        assert!(decode_canonical_checkpoint_pair(&no_terminal).is_err());
        let mut wrong_ordinal = encoded.clone();
        let header_length = u32::from_be_bytes(wrong_ordinal[..4].try_into().expect("header length")) as usize;
        wrong_ordinal[4 + header_length + 4 + 2 + 3] ^= 1;
        assert!(matches!(decode_canonical_checkpoint_pair(&wrong_ordinal), Err(RebootstrapError::Integrity)));
        let mut trailing = encoded;
        trailing.push(0);
        assert!(matches!(decode_canonical_checkpoint_pair(&trailing), Err(RebootstrapError::Integrity)));
    }

    #[test]
    fn canonical_pair_encoding_cancels_without_terminal_acceptance() {
        let pair = canonical_pair_fixture();
        let control = test_control();
        control.cancelled.store(true, Ordering::SeqCst);
        let context = RebootstrapContext::new(100, &control);
        assert!(matches!(encode_verified_canonical_pair(&pair, &context), Err(RebootstrapError::Cancelled)));
        control.cancelled.store(false, Ordering::SeqCst);
        control.now.store(100, Ordering::SeqCst);
        assert!(matches!(encode_verified_canonical_pair(&pair, &context), Err(RebootstrapError::DeadlineExceeded)));
    }

    #[tokio::test]
    async fn lag_rebootstrap_neutral_wire_contract() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🛟️lag-rebootstrap/🔣️.json")).expect("fixture JSON");
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
