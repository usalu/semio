//! 🧩️ Space-scoped fixed-chunk artifact CAS and canonical manifest codec.

use super::{ArtifactBlobIntegrity, ArtifactPair, AuthorityError, AuthorityProgress, AuthorityProgressStage, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob, AUTHORITY_MAX_PAIR_BYTES};
use directory::os_directory::{hex_lower, ArtifactCheckpoint, ArtifactHash, DocumentScope};
use semio_framework_hash::Sha256;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// 🧱️ Exact storage chunk width for artifact CAS v1.
pub const ARTIFACT_CAS_CHUNK_BYTES: usize = 256 * 1024;
/// 🧯️ Maximum chunk records admitted by one manifest.
pub const ARTIFACT_CAS_MAX_CHUNKS: usize = 256;
/// 🧯️ Maximum canonical manifest bytes admitted before decode allocation.
pub const ARTIFACT_CAS_MAX_MANIFEST_BYTES: usize = 64 * 1024;
/// 🧯️ Maximum UTF-8 bytes in the tenant identity included in every object digest.
pub const ARTIFACT_CAS_MAX_SPACE_BYTES: usize = 1024;
/// 🔐️ Domain prefix for one space-scoped artifact chunk identity.
pub const ARTIFACT_CAS_CHUNK_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.chunk.v1\0";
/// 🔐️ Domain prefix and exact version marker for the canonical manifest bytes.
pub const ARTIFACT_CAS_MANIFEST_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.manifest.v1\0";
/// 🔐️ Domain prefix for the private on-disk tenant partition.
pub const ARTIFACT_CAS_SPACE_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.space.v1\0";
const ARTIFACT_CAS_COORDINATOR_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.coordinator.v1\0";
const ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.physical-fence.v1\0";
/// 🪢️ Private locator prefix stored only in the checkpoint authority journal.
pub const ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX: &str = "semio.artifact-cas.manifest/v1/";
/// 🔐️ Domain prefix for the private ownership plan journal.
pub const ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.ownership.v1\0";
/// 🧯️ Maximum unique objects owned by one pack/SPR pair.
pub const ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS: usize = ARTIFACT_CAS_MAX_CHUNKS + 2;
/// 🧯️ Maximum canonical private ownership-plan bytes.
pub const ARTIFACT_CAS_OWNERSHIP_MAX_BYTES: usize = 64 * 1024;
/// ⏳️ Fixed post-deadline protection for an interrupted publication.
pub const ARTIFACT_CAS_RESERVATION_GRACE_MS: u64 = 60_000;

/// 🧱️ One contiguous canonical chunk record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasChunkV1 {
    pub ordinal: u32,
    pub byte_length: u32,
    pub chunk_id: ArtifactHash,
}

/// 📜️ Strictly decoded space-scoped artifact manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasManifestV1 {
    pub space_id: String,
    pub raw_sha256: ArtifactHash,
    pub raw_byte_length: u64,
    pub chunk_bytes: u32,
    pub chunks: Vec<ArtifactCasChunkV1>,
}

/// 🧭 Pure manifest plan retaining metadata and canonical bytes without duplicating raw bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasManifestPlan {
    pub manifest: ArtifactCasManifestV1,
    pub manifest_bytes: Vec<u8>,
    pub manifest_id: ArtifactHash,
}

/// 🧺 Dedicated CAS namespace discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArtifactCasObjectKind {
    Chunk,
    Manifest,
}

impl ArtifactCasObjectKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Chunk => "chunk",
            Self::Manifest => "manifest",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AuthorityError> {
        match value {
            "chunk" => Ok(Self::Chunk),
            "manifest" => Ok(Self::Manifest),
            _ => Err(AuthorityError::BlobIntegrity("artifact CAS object kind")),
        }
    }

    fn maximum_bytes(self) -> usize {
        match self {
            Self::Chunk => ARTIFACT_CAS_CHUNK_BYTES,
            Self::Manifest => ARTIFACT_CAS_MAX_MANIFEST_BYTES,
        }
    }
}

/// 🔑️ Structural private object key; storage derives its tenant partition from `space_id`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactCasObjectKey {
    pub space_id: String,
    pub kind: ArtifactCasObjectKind,
    pub digest: ArtifactHash,
}

/// 📒️ Canonical private reachability plan committed before the first CAS write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasOwnershipPlanV1 {
    pub scope: DocumentScope,
    pub checkpoint_id: ArtifactHash,
    pub pack_manifest_id: ArtifactHash,
    pub spr_manifest_id: ArtifactHash,
    pub objects: Vec<ArtifactCasObjectKey>,
}

/// 🎟️ Exact expiring directory-ledger reservation token.
#[derive(Clone, PartialEq, Eq)]
pub struct ArtifactCasReservation {
    pub plan: ArtifactCasOwnershipPlanV1,
    pub generation: u64,
    pub write_epoch: u64,
    pub expires_at_ms: u64,
    coordinator_id: [u8; 32],
    physical_epoch: u64,
}

impl ArtifactCasReservation {
    pub(crate) const fn unfenced(plan: ArtifactCasOwnershipPlanV1, generation: u64, write_epoch: u64, expires_at_ms: u64) -> Self {
        Self { plan, generation, write_epoch, expires_at_ms, coordinator_id: [0; 32], physical_epoch: 0 }
    }

    pub(crate) const fn fenced(plan: ArtifactCasOwnershipPlanV1, generation: u64, write_epoch: u64, expires_at_ms: u64, coordinator_id: [u8; 32], physical_epoch: u64) -> Self {
        Self { plan, generation, write_epoch, expires_at_ms, coordinator_id, physical_epoch }
    }

    pub(crate) const fn coordinator_id(&self) -> &[u8; 32] {
        &self.coordinator_id
    }

    pub(crate) const fn physical_epoch(&self) -> u64 {
        self.physical_epoch
    }
}

impl std::fmt::Debug for ArtifactCasReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ArtifactCasReservation(<opaque>)")
    }
}

/// 🛡️ Opaque proof binding one directory reachability recheck to a CAS physical epoch.
pub struct ArtifactCasDeleteFence {
    object: ArtifactCasObjectKey,
    ledger_generation: u64,
    coordinator_id: [u8; 32],
    physical_epoch: u64,
    lease_token: [u8; 32],
}

impl ArtifactCasDeleteFence {
    pub(crate) fn new(object: ArtifactCasObjectKey, ledger_generation: u64, coordinator_id: [u8; 32], physical_epoch: u64, lease_token: [u8; 32]) -> Self {
        Self { object, ledger_generation, coordinator_id, physical_epoch, lease_token }
    }

    pub const fn ledger_generation(&self) -> u64 {
        self.ledger_generation
    }

    pub(crate) const fn object(&self) -> &ArtifactCasObjectKey {
        &self.object
    }

    pub(crate) const fn coordinator_id(&self) -> &[u8; 32] {
        &self.coordinator_id
    }

    pub(crate) const fn physical_epoch(&self) -> u64 {
        self.physical_epoch
    }

    pub(crate) const fn lease_token(&self) -> &[u8; 32] {
        &self.lease_token
    }

    fn permits(&self, object: &ArtifactCasObjectKey) -> bool {
        self.ledger_generation > 0 && self.physical_epoch > 0 && self.coordinator_id != [0; 32] && &self.object == object
    }
}

impl std::fmt::Debug for ArtifactCasDeleteFence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ArtifactCasDeleteFence(<opaque>)")
    }
}

/// 🧹️ Physical deletion result; missing orphan bytes are an idempotent success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCasDeleteOutcome {
    Deleted,
    Missing,
}

/// ✅️ Collision-checked immutable insertion result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCasPutOutcome {
    Inserted,
    AlreadyPresent,
}

/// 🗄️ Dedicated artifact CAS port; generic database payload storage never crosses this seam.
pub trait ArtifactChunkCasStorage: Send + Sync {
    /// 🪪 Binds a CAS namespace to one durable directory coordinator identity.
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError>;

    /// ⏩ Monotonically activates a directory fence epoch before staging or deletion.
    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError>;

    /// ➕️ Inserts an exact verified object or confirms byte-for-byte identity on collision.
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError>;

    /// 📖️ Reads one exact object with a kind-specific admission bound.
    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError>;

    /// 🧹️ Deletes only with an exact fence minted by the directory's immediate reachability recheck.
    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError>;
}

impl<T: ArtifactChunkCasStorage> ArtifactChunkCasStorage for Arc<T> {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.as_ref().configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.as_ref().advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        self.as_ref().put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.as_ref().get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        self.as_ref().delete_if_unreferenced(key, fence, context).await
    }
}

fn append_field(output: &mut Vec<u8>, value: &[u8]) -> Result<(), AuthorityError> {
    let length = u64::try_from(value.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS manifest byte"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn valid_space(space_id: &str) -> bool {
    !space_id.is_empty() && space_id.len() <= ARTIFACT_CAS_MAX_SPACE_BYTES
}

fn chunk_id(space_id: &str, bytes: &[u8]) -> Result<ArtifactHash, AuthorityError> {
    if !valid_space(space_id) || bytes.is_empty() || bytes.len() > ARTIFACT_CAS_CHUNK_BYTES {
        return Err(AuthorityError::BlobIntegrity("artifact CAS chunk identity"));
    }
    let mut hash = Sha256::new();
    hash.update(ARTIFACT_CAS_CHUNK_DOMAIN_V1);
    hash.update(&(space_id.len() as u64).to_be_bytes());
    hash.update(space_id.as_bytes());
    hash.update(&8u64.to_be_bytes());
    hash.update(&(bytes.len() as u64).to_be_bytes());
    hash.update(&(bytes.len() as u64).to_be_bytes());
    hash.update(bytes);
    Ok(ArtifactHash(hash.finalize()))
}

fn space_digest(space_id: &str) -> Result<ArtifactHash, AuthorityError> {
    if !valid_space(space_id) {
        return Err(AuthorityError::BlobIntegrity("artifact CAS space identity"));
    }
    let mut hash = Sha256::new();
    hash.update(ARTIFACT_CAS_SPACE_DOMAIN_V1);
    hash.update(&(space_id.len() as u64).to_be_bytes());
    hash.update(space_id.as_bytes());
    Ok(ArtifactHash(hash.finalize()))
}

/// 🧬️ Computes the exact domain-separated v1 chunk identity.
pub fn artifact_cas_chunk_id_v1(space_id: &str, bytes: &[u8]) -> Result<ArtifactHash, AuthorityError> {
    chunk_id(space_id, bytes)
}

/// 🧬️ Encodes one manifest in its only canonical byte representation.
pub fn encode_artifact_cas_manifest_v1(manifest: &ArtifactCasManifestV1) -> Result<Vec<u8>, AuthorityError> {
    if !valid_space(&manifest.space_id)
        || manifest.raw_byte_length > AUTHORITY_MAX_PAIR_BYTES
        || manifest.chunk_bytes != ARTIFACT_CAS_CHUNK_BYTES as u32
        || manifest.chunks.len() > ARTIFACT_CAS_MAX_CHUNKS
        || manifest.chunks.len() as u64 != manifest.raw_byte_length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES as u64)
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest shape"));
    }
    let mut output = Vec::with_capacity(ARTIFACT_CAS_MANIFEST_DOMAIN_V1.len() + 128 + manifest.chunks.len() * 64);
    output.extend_from_slice(ARTIFACT_CAS_MANIFEST_DOMAIN_V1);
    append_field(&mut output, manifest.space_id.as_bytes())?;
    append_field(&mut output, &manifest.raw_sha256.0)?;
    append_field(&mut output, &manifest.raw_byte_length.to_be_bytes())?;
    append_field(&mut output, &manifest.chunk_bytes.to_be_bytes())?;
    append_field(&mut output, &(manifest.chunks.len() as u32).to_be_bytes())?;
    for (index, chunk) in manifest.chunks.iter().enumerate() {
        let expected_length = if index + 1 == manifest.chunks.len() {
            let remainder = manifest.raw_byte_length % ARTIFACT_CAS_CHUNK_BYTES as u64;
            if remainder == 0 {
                ARTIFACT_CAS_CHUNK_BYTES as u64
            } else {
                remainder
            }
        } else {
            ARTIFACT_CAS_CHUNK_BYTES as u64
        };
        if chunk.ordinal as usize != index || chunk.byte_length == 0 || u64::from(chunk.byte_length) != expected_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest chunk shape"));
        }
        append_field(&mut output, &chunk.ordinal.to_be_bytes())?;
        append_field(&mut output, &chunk.byte_length.to_be_bytes())?;
        append_field(&mut output, &chunk.chunk_id.0)?;
    }
    if output.len() > ARTIFACT_CAS_MAX_MANIFEST_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS manifest byte"));
    }
    Ok(output)
}

struct FieldCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> FieldCursor<'a> {
    fn field(&mut self, exact: Option<usize>) -> Result<&'a [u8], AuthorityError> {
        let length_end = self.offset.checked_add(8).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let length_bytes: [u8; 8] = self.bytes.get(self.offset..length_end).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let length = usize::try_from(u64::from_be_bytes(length_bytes)).map_err(|_| AuthorityError::ResourceLimit("artifact CAS manifest byte"))?;
        if exact.is_some_and(|expected| expected != length) {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest field width"));
        }
        let end = length_end.checked_add(length).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let value = self.bytes.get(length_end..end).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        self.offset = end;
        Ok(value)
    }
}

fn u32_field(cursor: &mut FieldCursor<'_>) -> Result<u32, AuthorityError> {
    Ok(u32::from_be_bytes(cursor.field(Some(4))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest integer"))?))
}

fn u64_field(cursor: &mut FieldCursor<'_>) -> Result<u64, AuthorityError> {
    Ok(u64::from_be_bytes(cursor.field(Some(8))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest integer"))?))
}

/// 🔍️ Strictly decodes, scope-checks, and canonicality-checks one manifest before use.
pub fn decode_artifact_cas_manifest_v1(bytes: &[u8], expected_space_id: &str, expected_manifest_id: ArtifactHash) -> Result<ArtifactCasManifestV1, AuthorityError> {
    if bytes.len() > ARTIFACT_CAS_MAX_MANIFEST_BYTES || !valid_space(expected_space_id) || !bytes.starts_with(ARTIFACT_CAS_MANIFEST_DOMAIN_V1) || ArtifactHash(Sha256::digest(bytes)) != expected_manifest_id {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest identity"));
    }
    let mut cursor = FieldCursor { bytes, offset: ARTIFACT_CAS_MANIFEST_DOMAIN_V1.len() };
    let space_bytes = cursor.field(None)?;
    if space_bytes.len() > ARTIFACT_CAS_MAX_SPACE_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS space byte"));
    }
    let space_id = std::str::from_utf8(space_bytes).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest space"))?.to_string();
    let raw_sha256 = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS raw hash"))?);
    let raw_byte_length = u64_field(&mut cursor)?;
    let chunk_bytes = u32_field(&mut cursor)?;
    let chunk_count = usize::try_from(u32_field(&mut cursor)?).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk count"))?;
    if space_id != expected_space_id
        || raw_byte_length > AUTHORITY_MAX_PAIR_BYTES
        || chunk_bytes != ARTIFACT_CAS_CHUNK_BYTES as u32
        || chunk_count > ARTIFACT_CAS_MAX_CHUNKS
        || chunk_count as u64 != raw_byte_length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES as u64)
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest shape"));
    }
    let mut chunks = Vec::with_capacity(chunk_count);
    for index in 0..chunk_count {
        let ordinal = u32_field(&mut cursor)?;
        let byte_length = u32_field(&mut cursor)?;
        let chunk_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS chunk hash"))?);
        let expected_length = if index + 1 == chunk_count {
            let remainder = raw_byte_length % ARTIFACT_CAS_CHUNK_BYTES as u64;
            if remainder == 0 {
                ARTIFACT_CAS_CHUNK_BYTES as u64
            } else {
                remainder
            }
        } else {
            ARTIFACT_CAS_CHUNK_BYTES as u64
        };
        if ordinal as usize != index || byte_length == 0 || u64::from(byte_length) != expected_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest chunk shape"));
        }
        chunks.push(ArtifactCasChunkV1 { ordinal, byte_length, chunk_id });
    }
    if cursor.offset != bytes.len() {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest trailing byte"));
    }
    let manifest = ArtifactCasManifestV1 { space_id, raw_sha256, raw_byte_length, chunk_bytes, chunks };
    if encode_artifact_cas_manifest_v1(&manifest)? != bytes {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest canonical encoding"));
    }
    Ok(manifest)
}

/// 🧭 Plans exact raw/chunk/manifest identities without retaining a second copy of raw bytes.
pub fn prepare_artifact_cas_manifest_v1(space_id: &str, raw: &[u8]) -> Result<ArtifactCasManifestPlan, AuthorityError> {
    if !valid_space(space_id) || raw.len() as u64 > AUTHORITY_MAX_PAIR_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS raw byte"));
    }
    let mut chunks = Vec::with_capacity(raw.len().div_ceil(ARTIFACT_CAS_CHUNK_BYTES));
    for (ordinal, bytes) in raw.chunks(ARTIFACT_CAS_CHUNK_BYTES).enumerate() {
        chunks.push(ArtifactCasChunkV1 {
            ordinal: u32::try_from(ordinal).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk count"))?,
            byte_length: u32::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk byte"))?,
            chunk_id: chunk_id(space_id, bytes)?,
        });
    }
    let manifest = ArtifactCasManifestV1 { space_id: space_id.to_string(), raw_sha256: ArtifactHash(Sha256::digest(raw)), raw_byte_length: raw.len() as u64, chunk_bytes: ARTIFACT_CAS_CHUNK_BYTES as u32, chunks };
    let manifest_bytes = encode_artifact_cas_manifest_v1(&manifest)?;
    let manifest_id = ArtifactHash(Sha256::digest(&manifest_bytes));
    Ok(ArtifactCasManifestPlan { manifest, manifest_bytes, manifest_id })
}

fn validate_ownership_plan(plan: &ArtifactCasOwnershipPlanV1) -> Result<(), AuthorityError> {
    if !valid_space(&plan.scope.space_id)
        || plan.scope.document_id.is_empty()
        || plan.checkpoint_id.0 == [0; 32]
        || plan.pack_manifest_id.0 == [0; 32]
        || plan.spr_manifest_id.0 == [0; 32]
        || plan.objects.len() < 2
        || plan.objects.len() > ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership shape"));
    }
    let mut previous: Option<(ArtifactCasObjectKind, [u8; 32])> = None;
    let mut pack = false;
    let mut spr = false;
    for object in &plan.objects {
        if object.space_id != plan.scope.space_id || object.digest.0 == [0; 32] {
            return Err(AuthorityError::BlobIntegrity("artifact CAS ownership object"));
        }
        let identity = (object.kind, object.digest.0);
        if previous.is_some_and(|value| value >= identity) {
            return Err(AuthorityError::BlobIntegrity("artifact CAS ownership order"));
        }
        previous = Some(identity);
        pack |= object.kind == ArtifactCasObjectKind::Manifest && object.digest == plan.pack_manifest_id;
        spr |= object.kind == ArtifactCasObjectKind::Manifest && object.digest == plan.spr_manifest_id;
    }
    if !pack || !spr {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership manifest"));
    }
    Ok(())
}

/// 🧭️ Derives the exact pair ownership set before any physical CAS write.
pub fn prepare_artifact_cas_ownership_v1(checkpoint: &ArtifactCheckpoint, pair: &ArtifactPair) -> Result<ArtifactCasOwnershipPlanV1, AuthorityError> {
    let pack = prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.pack)?;
    let spr = prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.spr)?;
    if checkpoint.pack.sha256 != pack.manifest.raw_sha256 || checkpoint.pack.byte_length != pack.manifest.raw_byte_length || checkpoint.spr.sha256 != spr.manifest.raw_sha256 || checkpoint.spr.byte_length != spr.manifest.raw_byte_length {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership raw identity"));
    }
    let mut objects = Vec::with_capacity(pack.manifest.chunks.len() + spr.manifest.chunks.len() + 2);
    objects.extend(pack.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.extend(spr.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: pack.manifest_id });
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: spr.manifest_id });
    objects.sort_by_key(|object| (object.kind, object.digest.0));
    objects.dedup();
    let plan = ArtifactCasOwnershipPlanV1 { scope: checkpoint.scope.clone(), checkpoint_id: checkpoint.checkpoint_id, pack_manifest_id: pack.manifest_id, spr_manifest_id: spr.manifest_id, objects };
    validate_ownership_plan(&plan)?;
    Ok(plan)
}

/// 🧬️ Encodes the private ownership journal input canonically.
pub fn encode_artifact_cas_ownership_v1(plan: &ArtifactCasOwnershipPlanV1) -> Result<Vec<u8>, AuthorityError> {
    validate_ownership_plan(plan)?;
    let mut output = Vec::with_capacity(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1.len() + 256 + plan.objects.len() * 64);
    output.extend_from_slice(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1);
    append_field(&mut output, plan.scope.space_id.as_bytes())?;
    append_field(&mut output, plan.scope.document_id.as_bytes())?;
    append_field(&mut output, &plan.checkpoint_id.0)?;
    append_field(&mut output, &plan.pack_manifest_id.0)?;
    append_field(&mut output, &plan.spr_manifest_id.0)?;
    append_field(&mut output, &(plan.objects.len() as u32).to_be_bytes())?;
    for object in &plan.objects {
        append_field(&mut output, object.kind.name().as_bytes())?;
        append_field(&mut output, &object.digest.0)?;
    }
    if output.len() > ARTIFACT_CAS_OWNERSHIP_MAX_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS ownership byte"));
    }
    Ok(output)
}

/// 🔍️ Strictly decodes one canonical private ownership journal input.
pub fn decode_artifact_cas_ownership_v1(bytes: &[u8]) -> Result<ArtifactCasOwnershipPlanV1, AuthorityError> {
    if bytes.len() > ARTIFACT_CAS_OWNERSHIP_MAX_BYTES || !bytes.starts_with(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1) {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership identity"));
    }
    let mut cursor = FieldCursor { bytes, offset: ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1.len() };
    let space_id = std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership space"))?.to_string();
    let document_id = std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership document"))?.to_string();
    let checkpoint_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership checkpoint"))?);
    let pack_manifest_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership manifest"))?);
    let spr_manifest_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership manifest"))?);
    let count = usize::try_from(u32_field(&mut cursor)?).map_err(|_| AuthorityError::ResourceLimit("artifact CAS ownership count"))?;
    if count > ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS {
        return Err(AuthorityError::ResourceLimit("artifact CAS ownership count"));
    }
    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = ArtifactCasObjectKind::parse(std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership kind"))?)?;
        let digest = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership object"))?);
        objects.push(ArtifactCasObjectKey { space_id: space_id.clone(), kind, digest });
    }
    if cursor.offset != bytes.len() {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership trailing byte"));
    }
    let plan = ArtifactCasOwnershipPlanV1 { scope: DocumentScope::new(space_id, document_id), checkpoint_id, pack_manifest_id, spr_manifest_id, objects };
    validate_ownership_plan(&plan)?;
    if encode_artifact_cas_ownership_v1(&plan)? != bytes {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership canonical encoding"));
    }
    Ok(plan)
}

/// 🔗️ Verifies that final private locators exactly name the reserved manifests.
pub fn validate_artifact_cas_publication_v1(plan: &ArtifactCasOwnershipPlanV1, checkpoint: &ArtifactCheckpoint) -> Result<(), AuthorityError> {
    validate_ownership_plan(plan)?;
    if plan.scope != checkpoint.scope
        || plan.checkpoint_id != checkpoint.checkpoint_id
        || decode_artifact_cas_manifest_locator_v1(&checkpoint.pack.storage_key)? != plan.pack_manifest_id
        || decode_artifact_cas_manifest_locator_v1(&checkpoint.spr.storage_key)? != plan.spr_manifest_id
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS reserved publication"));
    }
    Ok(())
}

/// 🪢️ Encodes one private manifest locator without exposing tenant or chunk identities.
pub fn artifact_cas_manifest_locator_v1(manifest_id: ArtifactHash) -> String {
    format!("{ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX}{}", hex_lower(&manifest_id.0))
}

/// 🔓️ Strictly decodes one private manifest locator.
pub fn decode_artifact_cas_manifest_locator_v1(locator: &str) -> Result<ArtifactHash, AuthorityError> {
    let hexadecimal = locator.strip_prefix(ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX).ok_or_else(|| AuthorityError::Store("invalid artifact CAS manifest locator".to_string()))?;
    if hexadecimal.len() != 64 || hexadecimal.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(AuthorityError::Store("invalid artifact CAS manifest locator".to_string()));
    }
    let mut output = [0; 32];
    for (index, pair) in hexadecimal.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        output[index] = nibble(pair[0]) << 4 | nibble(pair[1]);
    }
    Ok(ArtifactHash(output))
}

fn validate_object(key: &ArtifactCasObjectKey, bytes: &[u8]) -> Result<(), AuthorityError> {
    if !valid_space(&key.space_id) || bytes.len() > key.kind.maximum_bytes() {
        return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
    }
    let actual = match key.kind {
        ArtifactCasObjectKind::Chunk => chunk_id(&key.space_id, bytes)?,
        ArtifactCasObjectKind::Manifest => {
            let digest = ArtifactHash(Sha256::digest(bytes));
            let _ = decode_artifact_cas_manifest_v1(bytes, &key.space_id, digest)?;
            digest
        }
    };
    if actual != key.digest {
        return Err(AuthorityError::BlobIntegrity("artifact CAS object key"));
    }
    Ok(())
}

/// 🧠 Collision-checked dependency-free memory CAS.
#[derive(Default)]
pub struct MemoryArtifactChunkCasStorage {
    state: Mutex<MemoryArtifactCasState>,
}

#[derive(Default)]
struct MemoryArtifactCasState {
    coordinator_id: Option<[u8; 32]>,
    physical_epochs: HashMap<ArtifactHash, u64>,
    objects: HashMap<(ArtifactHash, ArtifactCasObjectKind, ArtifactHash), Vec<u8>>,
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

/// 📁️ Dedicated filesystem CAS rooted below `artifact-cas/v1`.
pub struct FsArtifactChunkCasStorage {
    root: PathBuf,
    nonce: std::sync::atomic::AtomicU64,
}

struct ArtifactCasFileFence(std::fs::File);

#[cfg(unix)]
fn open_artifact_cas_leaf(path: &Path, write: bool, create: bool) -> std::io::Result<std::fs::File> {
    use std::os::unix::ffi::OsStrExt as _;
    use std::os::unix::io::FromRawFd as _;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CREATE: i32 = 0x40;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_CREATE: i32 = 0x0200;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_NOFOLLOW: i32 = 0x20_000;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_NOFOLLOW: i32 = 0x0100;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CLOEXEC: i32 = 0x8_0000;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_CLOEXEC: i32 = 0x0100_0000;
    unsafe extern "C" {
        fn open(path: *const core::ffi::c_char, flags: i32, ...) -> i32;
    }
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS path contains NUL"))?;
    let flags = (if write { 2 } else { 0 }) | O_NOFOLLOW | O_CLOEXEC | (if create { O_CREATE } else { 0 });
    let descriptor = unsafe { open(path.as_ptr(), flags, 0o600 as core::ffi::c_int) };
    if descriptor < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { std::fs::File::from_raw_fd(descriptor) })
    }
}

#[cfg(windows)]
fn open_artifact_cas_leaf(path: &Path, write: bool, create: bool) -> std::io::Result<std::fs::File> {
    use std::os::windows::fs::MetadataExt as _;
    use std::os::windows::fs::OpenOptionsExt as _;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(write).create(create).truncate(false).custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    let file = options.open(path)?;
    if file.metadata()?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS leaf is a reparse point"))
    } else {
        Ok(file)
    }
}

#[cfg(unix)]
fn try_lock_file(file: std::fs::File) -> std::io::Result<Option<ArtifactCasFileFence>> {
    use std::os::fd::AsRawFd as _;
    unsafe extern "C" {
        fn flock(fd: i32, operation: i32) -> i32;
    }
    if unsafe { flock(file.as_raw_fd(), 2 | 4) } == 0 {
        return Ok(Some(ArtifactCasFileFence(file)));
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.kind(), std::io::ErrorKind::WouldBlock) {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(unix)]
impl Drop for ArtifactCasFileFence {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd as _;
        unsafe extern "C" {
            fn flock(fd: i32, operation: i32) -> i32;
        }
        let _ = unsafe { flock(self.0.as_raw_fd(), 8) };
    }
}

#[cfg(windows)]
fn try_lock_file(file: std::fs::File) -> std::io::Result<Option<ArtifactCasFileFence>> {
    use std::os::windows::io::AsRawHandle as _;
    #[repr(C)]
    struct Overlapped {
        internal: usize,
        internal_high: usize,
        offset: u32,
        offset_high: u32,
        event: *mut core::ffi::c_void,
    }
    unsafe extern "system" {
        fn LockFileEx(file: *mut core::ffi::c_void, flags: u32, reserved: u32, low: u32, high: u32, overlapped: *mut Overlapped) -> i32;
    }
    let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: std::ptr::null_mut() };
    if unsafe { LockFileEx(file.as_raw_handle(), 2 | 1, 0, 1, 0, &mut overlapped) } != 0 {
        return Ok(Some(ArtifactCasFileFence(file)));
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.raw_os_error(), Some(33 | 158)) {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(windows)]
impl Drop for ArtifactCasFileFence {
    fn drop(&mut self) {
        use std::os::windows::io::AsRawHandle as _;
        #[repr(C)]
        struct Overlapped {
            internal: usize,
            internal_high: usize,
            offset: u32,
            offset_high: u32,
            event: *mut core::ffi::c_void,
        }
        unsafe extern "system" {
            fn UnlockFileEx(file: *mut core::ffi::c_void, reserved: u32, low: u32, high: u32, overlapped: *mut Overlapped) -> i32;
        }
        let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: std::ptr::null_mut() };
        let _ = unsafe { UnlockFileEx(self.0.as_raw_handle(), 0, 1, 0, &mut overlapped) };
    }
}

impl FsArtifactChunkCasStorage {
    /// 🏗️ Creates and validates one non-symlink dedicated storage root.
    pub async fn open(root: &Path) -> Result<Self, AuthorityError> {
        tokio::fs::create_dir_all(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root create failed".to_string()))?;
        let metadata = tokio::fs::symlink_metadata(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root metadata failed".to_string()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem root must be a non-symlink directory".to_string()));
        }
        let root = tokio::fs::canonicalize(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root canonicalization failed".to_string()))?;
        Ok(Self { root, nonce: std::sync::atomic::AtomicU64::new(1) })
    }

    fn object_path(&self, key: &ArtifactCasObjectKey) -> Result<PathBuf, AuthorityError> {
        Ok(self.root.join(hex_lower(&space_digest(&key.space_id)?.0)).join(key.kind.name()).join(hex_lower(&key.digest.0)))
    }

    fn validate_directory(path: &Path) -> Result<(), AuthorityError> {
        let metadata = std::fs::symlink_metadata(path).map_err(|_| AuthorityError::Store("artifact CAS filesystem directory metadata failed".into()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem path must be a non-symlink directory".into()));
        }
        Ok(())
    }

    fn reject_symlink(path: &Path) -> Result<(), AuthorityError> {
        match std::fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_symlink() => Err(AuthorityError::Store("artifact CAS filesystem leaf must not be a symlink".into())),
            Ok(_) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(AuthorityError::Store("artifact CAS filesystem leaf metadata failed".into())),
        }
    }

    fn validate_opened_leaf(path: &Path, file: &std::fs::File) -> Result<(), AuthorityError> {
        let path_metadata = std::fs::symlink_metadata(path).map_err(|_| AuthorityError::Store("artifact CAS filesystem leaf metadata failed".into()))?;
        if path_metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem leaf must not be a symlink".into()));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            let opened = file.metadata().map_err(|_| AuthorityError::Store("artifact CAS filesystem opened leaf metadata failed".into()))?;
            if opened.dev() != path_metadata.dev() || opened.ino() != path_metadata.ino() {
                return Err(AuthorityError::Store("artifact CAS filesystem leaf changed during open".into()));
            }
        }
        Ok(())
    }

    fn read_fence(path: &Path) -> Result<Vec<u8>, AuthorityError> {
        use std::io::Read as _;
        Self::reject_symlink(path)?;
        let mut file = open_artifact_cas_leaf(path, false, false).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence read failed".into()))?;
        Self::validate_opened_leaf(path, &file)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence read failed".into()))?;
        Ok(bytes)
    }

    fn validate_object_parents(&self, key: &ArtifactCasObjectKey) -> Result<(), AuthorityError> {
        let target = self.object_path(key)?;
        let kind = target.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem object parent missing".into()))?;
        let space = kind.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem space parent missing".into()))?;
        Self::validate_directory(space)?;
        Self::validate_directory(kind)
    }

    async fn acquire_file_fence(&self, path: &Path, context: &OperationContext<'_>) -> Result<ArtifactCasFileFence, AuthorityError> {
        loop {
            context.checkpoint()?;
            let path = path.to_path_buf();
            let attempt = tokio::task::spawn_blocking(move || -> std::io::Result<Option<ArtifactCasFileFence>> {
                if std::fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS lock is a symlink"));
                }
                let file = open_artifact_cas_leaf(&path, true, true)?;
                Self::validate_opened_leaf(&path, &file).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS lock changed during open"))?;
                try_lock_file(file)
            })
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS filesystem fence worker failed".into()))?
            .map_err(|error| AuthorityError::Store(format!("artifact CAS filesystem fence lock failed: {error}")))?;
            if let Some(fence) = attempt {
                return Ok(fence);
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }

    fn encode_fence(domain: &[u8], coordinator_id: [u8; 32], epoch: Option<u64>) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(domain.len() + 72);
        bytes.extend_from_slice(domain);
        bytes.extend_from_slice(&coordinator_id);
        if let Some(epoch) = epoch {
            bytes.extend_from_slice(&epoch.to_be_bytes());
        }
        let checksum = Sha256::digest(&bytes);
        bytes.extend_from_slice(&checksum);
        bytes
    }

    fn decode_fence(bytes: &[u8], domain: &[u8], has_epoch: bool) -> Result<([u8; 32], u64), AuthorityError> {
        let payload = domain.len() + 32 + if has_epoch { 8 } else { 0 };
        if bytes.len() != payload + 32 || &bytes[..domain.len()] != domain || Sha256::digest(&bytes[..payload]) != bytes[payload..] {
            return Err(AuthorityError::Store("artifact CAS filesystem fence metadata is invalid".into()));
        }
        let mut coordinator_id = [0; 32];
        coordinator_id.copy_from_slice(&bytes[domain.len()..domain.len() + 32]);
        let epoch = if has_epoch { u64::from_be_bytes(bytes[domain.len() + 32..payload].try_into().map_err(|_| AuthorityError::Store("artifact CAS filesystem fence metadata is invalid".into()))?) } else { 0 };
        Ok((coordinator_id, epoch))
    }

    #[cfg(unix)]
    fn replace_fence(from: &Path, to: &Path) -> std::io::Result<()> {
        std::fs::rename(from, to)
    }

    #[cfg(windows)]
    fn replace_fence(from: &Path, to: &Path) -> std::io::Result<()> {
        use std::os::windows::ffi::OsStrExt as _;
        unsafe extern "system" {
            fn MoveFileExW(existing: *const u16, replacement: *const u16, flags: u32) -> i32;
        }
        let from: Vec<u16> = from.as_os_str().encode_wide().chain([0]).collect();
        let to: Vec<u16> = to.as_os_str().encode_wide().chain([0]).collect();
        if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 1 | 8) } != 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    #[cfg(unix)]
    fn sync_fence_parent(path: &Path) -> std::io::Result<()> {
        std::fs::File::open(path)?.sync_all()
    }

    #[cfg(windows)]
    fn sync_fence_parent(_: &Path) -> std::io::Result<()> {
        Ok(())
    }

    fn write_fence(path: &Path, bytes: &[u8]) -> Result<(), AuthorityError> {
        use std::io::Write as _;
        Self::reject_symlink(path)?;
        Self::validate_directory(path.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem fence parent missing".into()))?)?;
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
        let temporary = path.with_extension(format!("tmp.{}.{nonce}", std::process::id()));
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence temporary create failed".into()))?;
        file.write_all(bytes).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence write failed".into()))?;
        file.sync_all().map_err(|_| AuthorityError::Store("artifact CAS filesystem fence sync failed".into()))?;
        drop(file);
        Self::replace_fence(&temporary, path).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence replace failed".into()))?;
        let parent = path.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem fence parent missing".into()))?;
        Self::sync_fence_parent(parent).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence directory sync failed".into()))
    }

    async fn read_path(&self, key: &ArtifactCasObjectKey, path: &Path, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        use tokio::io::AsyncReadExt as _;
        context.checkpoint()?;
        self.validate_object_parents(key)?;
        Self::reject_symlink(path)?;
        let path = path.to_path_buf();
        let std_file = tokio::task::spawn_blocking(move || {
            let file = open_artifact_cas_leaf(&path, false, false).map_err(|_| AuthorityError::Store("artifact CAS filesystem object not found".to_string()))?;
            Self::validate_opened_leaf(&path, &file)?;
            Ok::<_, AuthorityError>(file)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS filesystem object open worker failed".into()))??;
        let file = tokio::fs::File::from_std(std_file);
        let metadata = file.metadata().await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object metadata failed".to_string()))?;
        if !metadata.is_file() || metadata.len() > key.kind.maximum_bytes() as u64 {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(key.kind.maximum_bytes() as u64 + 1).read_to_end(&mut bytes).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object read failed".to_string()))?;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }
}

impl ArtifactChunkCasStorage for FsArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let _fence = self.acquire_file_fence(&self.root.join("coordinator.lock"), context).await?;
        let path = self.root.join("coordinator-v1");
        match std::fs::symlink_metadata(&path) {
            Ok(_) => {
                if Self::decode_fence(&Self::read_fence(&path)?, ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, false)?.0 != coordinator_id {
                    return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Self::write_fence(&path, &Self::encode_fence(ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, coordinator_id, None))?,
            Err(_) => return Err(AuthorityError::Store("artifact CAS filesystem coordinator read failed".into())),
        }
        context.checkpoint()
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let coordinator = Self::read_fence(&self.root.join("coordinator-v1")).map_err(|_| AuthorityError::Store("artifact CAS coordinator is not configured".into()))?;
        if Self::decode_fence(&coordinator, ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, false)?.0 != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        let space_root = self.root.join(hex_lower(&space_digest(space_id)?.0));
        tokio::fs::create_dir_all(&space_root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem space create failed".into()))?;
        Self::validate_directory(&space_root)?;
        let _fence = self.acquire_file_fence(&space_root.join("fence.lock"), context).await?;
        let path = space_root.join("fence-v1");
        let current = match std::fs::symlink_metadata(&path) {
            Ok(_) => {
                let (current_id, current_epoch) = Self::decode_fence(&Self::read_fence(&path)?, ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, true)?;
                if current_id != coordinator_id {
                    return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
                }
                current_epoch
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
            Err(_) => return Err(AuthorityError::Store("artifact CAS filesystem physical epoch read failed".into())),
        };
        if epoch > current {
            Self::write_fence(&path, &Self::encode_fence(ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, coordinator_id, Some(epoch)))?;
        }
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        use tokio::io::AsyncWriteExt as _;
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let target = self.object_path(key)?;
        let parent = target.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem object parent missing".to_string()))?;
        tokio::fs::create_dir_all(parent).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object directory create failed".to_string()))?;
        self.validate_object_parents(key)?;
        Self::reject_symlink(&target)?;
        let nonce = self.nonce.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temporary = parent.join(format!(".{}.{}.tmp", std::process::id(), nonce));
        let mut options = tokio::fs::OpenOptions::new();
        options.write(true).create_new(true);
        let mut file = options.open(&temporary).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem temporary create failed".to_string()))?;
        file.write_all(bytes).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object write failed".to_string()))?;
        file.sync_all().await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object sync failed".to_string()))?;
        drop(file);
        context.checkpoint()?;
        let inserted = match tokio::fs::hard_link(&temporary, &target).await {
            Ok(()) => true,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
            Err(_) => {
                let _ = tokio::fs::remove_file(&temporary).await;
                return Err(AuthorityError::Store("artifact CAS filesystem atomic install failed".to_string()));
            }
        };
        let _ = tokio::fs::remove_file(&temporary).await;
        let exact = self.read_path(key, &target, context).await?;
        if exact != bytes {
            return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
        }
        Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.read_path(key, &self.object_path(key)?, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let space_root = self.root.join(hex_lower(&space_digest(&key.space_id)?.0));
        Self::validate_directory(&space_root)?;
        let _physical_fence = self.acquire_file_fence(&space_root.join("fence.lock"), context).await?;
        let metadata = Self::read_fence(&space_root.join("fence-v1"))?;
        let (coordinator_id, physical_epoch) = Self::decode_fence(&metadata, ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, true)?;
        if &coordinator_id != fence.coordinator_id() || physical_epoch != fence.physical_epoch() {
            return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
        }
        self.validate_object_parents(key)?;
        let object_path = self.object_path(key)?;
        Self::reject_symlink(&object_path)?;
        match tokio::fs::remove_file(object_path).await {
            Ok(()) => Ok(ArtifactCasDeleteOutcome::Deleted),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ArtifactCasDeleteOutcome::Missing),
            Err(_) => Err(AuthorityError::Store("artifact CAS filesystem object delete failed".to_string())),
        }
    }
}

#[cfg(feature = "sqlite")]
/// 🪶️ Dedicated SQLite artifact CAS table, separate from generic database payloads.
pub struct SqliteArtifactChunkCasStorage {
    connection: Arc<Mutex<rusqlite::Connection>>,
}

#[cfg(feature = "sqlite")]
impl SqliteArtifactChunkCasStorage {
    const SCHEMA: &'static str = "CREATE TABLE IF NOT EXISTS hub_artifact_cas_object (
        space_digest BLOB NOT NULL CHECK(length(space_digest) = 32),
        kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
        object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
        bytes BLOB NOT NULL,
        byte_length INTEGER NOT NULL,
        PRIMARY KEY(space_digest, kind, object_digest),
        CHECK(byte_length = length(bytes)),
        CHECK((kind = 'chunk' AND byte_length BETWEEN 1 AND 262144) OR (kind = 'manifest' AND byte_length BETWEEN 1 AND 65536))
    );
    CREATE TABLE IF NOT EXISTS hub_artifact_cas_coordinator (
        singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
        coordinator_id BLOB NOT NULL CHECK(length(coordinator_id) = 32)
    );
    CREATE TABLE IF NOT EXISTS hub_artifact_cas_space_fence (
        coordinator_id BLOB NOT NULL CHECK(length(coordinator_id) = 32),
        space_digest BLOB NOT NULL CHECK(length(space_digest) = 32),
        physical_epoch INTEGER NOT NULL CHECK(physical_epoch >= 1),
        updated_at_ms INTEGER NOT NULL,
        PRIMARY KEY(coordinator_id, space_digest)
    )";

    /// 🔌️ Opens and bootstraps one durable dedicated CAS table.
    pub async fn open(path: &Path) -> Result<Self, AuthorityError> {
        let path = path.to_path_buf();
        let connection = tokio::task::spawn_blocking(move || -> Result<rusqlite::Connection, AuthorityError> {
            let connection = rusqlite::Connection::open(path).map_err(|_| AuthorityError::Store("artifact CAS SQLite open failed".to_string()))?;
            connection.pragma_update(None, "journal_mode", "WAL").map_err(|_| AuthorityError::Store("artifact CAS SQLite WAL setup failed".to_string()))?;
            connection.pragma_update(None, "synchronous", "FULL").map_err(|_| AuthorityError::Store("artifact CAS SQLite durability setup failed".to_string()))?;
            connection.execute_batch(Self::SCHEMA).map_err(|_| AuthorityError::Store("artifact CAS SQLite schema failed".to_string()))?;
            Ok(connection)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        Ok(Self { connection: Arc::new(Mutex::new(connection)) })
    }

    #[cfg(test)]
    async fn memory() -> Result<Self, AuthorityError> {
        let connection = rusqlite::Connection::open_in_memory().map_err(|_| AuthorityError::Store("artifact CAS SQLite memory open failed".to_string()))?;
        connection.execute_batch(Self::SCHEMA).map_err(|_| AuthorityError::Store("artifact CAS SQLite memory schema failed".to_string()))?;
        Ok(Self { connection: Arc::new(Mutex::new(connection)) })
    }
}

#[cfg(feature = "sqlite")]
impl ArtifactChunkCasStorage for SqliteArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || -> Result<(), AuthorityError> {
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            tx.execute("INSERT OR IGNORE INTO hub_artifact_cas_coordinator(singleton, coordinator_id) VALUES (1, ?1)", [coordinator_id.as_slice()]).map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator write failed".into()))?;
            let current: Vec<u8> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton = 1", [], |row| row.get(0)).map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator read failed".into()))?;
            if current != coordinator_id {
                return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
            }
            tx.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(())
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".into()))??;
        context.checkpoint()
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let connection = self.connection.clone();
        let space = space_digest(space_id)?.0.to_vec();
        let epoch = i64::try_from(epoch).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let now = i64::try_from(context.now_ms()).unwrap_or(i64::MAX);
        tokio::task::spawn_blocking(move || -> Result<(), AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            let current: Option<Vec<u8>> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton = 1", [], |row| row.get(0)).optional().map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator read failed".into()))?;
            if current.as_deref() != Some(coordinator_id.as_slice()) { return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into())); }
            tx.execute("INSERT INTO hub_artifact_cas_space_fence(coordinator_id, space_digest, physical_epoch, updated_at_ms) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(coordinator_id, space_digest) DO UPDATE SET physical_epoch = MAX(physical_epoch, excluded.physical_epoch), updated_at_ms = excluded.updated_at_ms", rusqlite::params![coordinator_id.as_slice(), space, epoch, now]).map_err(|_| AuthorityError::Store("artifact CAS SQLite physical epoch advance failed".into()))?;
            tx.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(())
        }).await.map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".into()))??;
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let expected = bytes.to_vec();
        let length = i64::try_from(expected.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS object byte"))?;
        let outcome = tokio::task::spawn_blocking(move || -> Result<ArtifactCasPutOutcome, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let transaction = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".to_string()))?;
            let inserted = transaction
                .execute("INSERT OR IGNORE INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![tenant, kind, digest, expected, length])
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite insert failed".to_string()))?
                == 1;
            let stored_length: Option<i64> = transaction
                .query_row("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?;
            if stored_length != Some(length) {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            let stored: Vec<u8> = transaction
                .query_row("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))?;
            if stored != expected {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            transaction.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".to_string()))?;
            Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        context.checkpoint()?;
        Ok(outcome)
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let maximum = key.kind.maximum_bytes();
        let bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let length: i64 = connection
                .query_row("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?
                .ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
            if length < 0 || usize::try_from(length).ok().is_none_or(|length| length > maximum) {
                return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
            }
            connection
                .query_row("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let coordinator_id = *fence.coordinator_id();
        let physical_epoch = i64::try_from(fence.physical_epoch()).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let deleted = tokio::task::spawn_blocking(move || -> Result<bool, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let transaction = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            let current: Option<i64> = transaction
                .query_row("SELECT physical_epoch FROM hub_artifact_cas_space_fence WHERE coordinator_id = ?1 AND space_digest = ?2", rusqlite::params![coordinator_id.as_slice(), tenant], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite physical epoch read failed".into()))?;
            if current != Some(physical_epoch) {
                return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
            }
            let deleted = transaction
                .execute("DELETE FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest])
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite delete failed".to_string()))?
                == 1;
            transaction.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(deleted)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        Ok(if deleted { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}

#[cfg(feature = "postgres")]
/// 🐘️ Dedicated PostgreSQL artifact CAS table.
pub struct PostgresArtifactChunkCasStorage {
    pool: sqlx_postgres::PgPool,
}

#[cfg(feature = "postgres")]
impl PostgresArtifactChunkCasStorage {
    /// 🔌️ Connects and bootstraps the dedicated CAS table.
    pub async fn connect(database_url: &str) -> Result<Self, AuthorityError> {
        let pool = sqlx_postgres::PgPoolOptions::new().max_connections(16).connect(database_url).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL connect failed".to_string()))?;
        sqlx_core::query::query(
            "CREATE TABLE IF NOT EXISTS hub_artifact_cas_object (
            space_digest BYTEA NOT NULL CHECK(octet_length(space_digest) = 32),
            kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
            object_digest BYTEA NOT NULL CHECK(octet_length(object_digest) = 32),
            bytes BYTEA NOT NULL,
            byte_length BIGINT NOT NULL,
            PRIMARY KEY(space_digest, kind, object_digest),
            CHECK(byte_length = octet_length(bytes)),
            CHECK((kind = 'chunk' AND byte_length BETWEEN 1 AND 262144) OR (kind = 'manifest' AND byte_length BETWEEN 1 AND 65536))
        )",
        )
        .execute(&pool)
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL schema failed".to_string()))?;
        sqlx_core::query::query("CREATE TABLE IF NOT EXISTS hub_artifact_cas_coordinator (singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK(singleton), coordinator_id BYTEA NOT NULL CHECK(octet_length(coordinator_id) = 32))")
            .execute(&pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator schema failed".into()))?;
        sqlx_core::query::query("CREATE TABLE IF NOT EXISTS hub_artifact_cas_space_fence (coordinator_id BYTEA NOT NULL CHECK(octet_length(coordinator_id) = 32), space_digest BYTEA NOT NULL CHECK(octet_length(space_digest) = 32), physical_epoch BIGINT NOT NULL CHECK(physical_epoch >= 1), updated_at_ms BIGINT NOT NULL, PRIMARY KEY(coordinator_id, space_digest))").execute(&pool).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL fence schema failed".into()))?;
        Ok(Self { pool })
    }
}

#[cfg(feature = "postgres")]
impl ArtifactChunkCasStorage for PostgresArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_coordinator(singleton, coordinator_id) VALUES (TRUE, $1) ON CONFLICT(singleton) DO NOTHING")
            .bind(coordinator_id.as_slice())
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator write failed".into()))?;
        let (current,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton FOR UPDATE")
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator read failed".into()))?;
        if current != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        Ok(())
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let tenant = space_digest(space_id)?.0.to_vec();
        let epoch = i64::try_from(epoch).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        let (current,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton FOR UPDATE")
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator read failed".into()))?;
        if current != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_space_fence(coordinator_id, space_digest, physical_epoch, updated_at_ms) VALUES ($1,$2,$3,$4) ON CONFLICT(coordinator_id, space_digest) DO UPDATE SET physical_epoch = GREATEST(hub_artifact_cas_space_fence.physical_epoch, excluded.physical_epoch), updated_at_ms = excluded.updated_at_ms").bind(coordinator_id.as_slice()).bind(tenant).bind(epoch).bind(i64::try_from(context.now_ms()).unwrap_or(i64::MAX)).execute(&mut *transaction).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL physical epoch advance failed".into()))?;
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let length = i64::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS object byte"))?;
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".to_string()))?;
        let inserted = sqlx_core::query::query("INSERT INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .bind(bytes)
            .bind(length)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL insert failed".to_string()))?
            .rows_affected()
            == 1;
        let row: (i64, Vec<u8>) = sqlx_core::query_as::query_as("SELECT byte_length, bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL object read failed".to_string()))?;
        if row.0 != length || row.1 != bytes {
            return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
        }
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".to_string()))?;
        context.checkpoint()?;
        Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let length: (i64,) = sqlx_core::query_as::query_as("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        if length.0 < 0 || usize::try_from(length.0).ok().is_none_or(|length| length > key.kind.maximum_bytes()) {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let row: (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        validate_object(key, &row.0)?;
        context.checkpoint()?;
        Ok(row.0)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        let current: Option<(i64,)> = sqlx_core::query_as::query_as("SELECT physical_epoch FROM hub_artifact_cas_space_fence WHERE coordinator_id = $1 AND space_digest = $2 FOR UPDATE")
            .bind(fence.coordinator_id().as_slice())
            .bind(&tenant)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL physical epoch read failed".into()))?;
        if current != Some((i64::try_from(fence.physical_epoch()).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?,)) {
            return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
        }
        let deleted = sqlx_core::query::query("DELETE FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL delete failed".to_string()))?
            .rows_affected()
            == 1;
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        Ok(if deleted { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}

#[cfg(feature = "neo4j")]
/// 🕸️ Dedicated Neo4j artifact CAS label.
pub struct Neo4jArtifactChunkCasStorage {
    graph: neo4rs::Graph,
}

#[cfg(feature = "neo4j")]
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

#[cfg(feature = "neo4j")]
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

/// 🔀️ Concrete runtime-selected artifact CAS without trait-object futures.
pub enum ArtifactChunkCasStores {
    Memory(MemoryArtifactChunkCasStorage),
    Filesystem(FsArtifactChunkCasStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(SqliteArtifactChunkCasStorage),
    #[cfg(feature = "postgres")]
    Postgres(PostgresArtifactChunkCasStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(Neo4jArtifactChunkCasStorage),
}

impl ArtifactChunkCasStorage for ArtifactChunkCasStores {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        match self {
            Self::Memory(storage) => storage.configure_coordinator(coordinator_id, context).await,
            Self::Filesystem(storage) => storage.configure_coordinator(coordinator_id, context).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(storage) => storage.configure_coordinator(coordinator_id, context).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.configure_coordinator(coordinator_id, context).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.configure_coordinator(coordinator_id, context).await,
        }
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        match self {
            Self::Memory(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            Self::Filesystem(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
        }
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.put_if_absent(key, bytes, context).await,
            Self::Filesystem(storage) => storage.put_if_absent(key, bytes, context).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(storage) => storage.put_if_absent(key, bytes, context).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.put_if_absent(key, bytes, context).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.put_if_absent(key, bytes, context).await,
        }
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.get(key, context).await,
            Self::Filesystem(storage) => storage.get(key, context).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(storage) => storage.get(key, context).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.get(key, context).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.get(key, context).await,
        }
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            Self::Filesystem(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.delete_if_unreferenced(key, fence, context).await,
        }
    }
}

/// 🫙️ Manifest-based immutable blob adapter used by both publication and rebootstrap.
pub struct ArtifactChunkBlobStore<S> {
    storage: S,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact_authority::{AuthorityLimits, AuthorityOperationControl, AuthorityProgress};
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    struct TestControl {
        now: AtomicU64,
        cancelled: AtomicBool,
    }

    impl AuthorityOperationControl for TestControl {
        fn now_ms(&self) -> u64 {
            self.now.load(Ordering::SeqCst)
        }
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }
        fn report(&self, _: AuthorityProgress) {}
    }

    fn control() -> TestControl {
        TestControl { now: AtomicU64::new(1), cancelled: AtomicBool::new(false) }
    }

    fn bytes(length: usize) -> Vec<u8> {
        (0..length).map(|index| ((index * 31 + index / 251) % 256) as u8).collect()
    }

    #[test]
    fn artifact_chunk_cas_manifest_boundaries_are_canonical_and_space_scoped() {
        for length in [0, 1, 262_143, 262_144, 262_145] {
            let raw = bytes(length);
            let plan = prepare_artifact_cas_manifest_v1("space-ü", &raw).expect("plan");
            assert_eq!(plan.manifest.chunks.len(), length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES));
            assert_eq!(decode_artifact_cas_manifest_v1(&plan.manifest_bytes, "space-ü", plan.manifest_id).expect("decode"), plan.manifest);
            assert_ne!(prepare_artifact_cas_manifest_v1("space-b", &raw).expect("other space").manifest_id, plan.manifest_id);
            let mut trailing = plan.manifest_bytes.clone();
            trailing.push(0);
            assert!(decode_artifact_cas_manifest_v1(&trailing, "space-ü", ArtifactHash(Sha256::digest(&trailing))).is_err());
            let mut mutated = plan.manifest_bytes.clone();
            if let Some(last) = mutated.last_mut() {
                *last ^= 1;
                assert!(decode_artifact_cas_manifest_v1(&mutated, "space-ü", plan.manifest_id).is_err());
            }
        }
        assert!(decode_artifact_cas_manifest_locator_v1("semio.artifact-cas.manifest/v1/AA").is_err());
    }

    #[test]
    fn artifact_chunk_cas_neutral_fixture_matches_repository_sha256() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("fixture JSON");
        let space_id = fixture["spaceId"].as_str().expect("space id");
        let assert_vector = |vector: &serde_json::Value| {
            let length = usize::try_from(vector["length"].as_u64().expect("length")).expect("bounded length");
            let plan = prepare_artifact_cas_manifest_v1(space_id, &bytes(length)).expect("plan vector");
            assert_eq!(hex_lower(&plan.manifest.raw_sha256.0), vector["rawSha256"].as_str().expect("raw hash"));
            assert_eq!(plan.manifest.chunks.len() as u64, vector["chunkCount"].as_u64().expect("chunk count"));
            assert_eq!(plan.manifest_bytes.len() as u64, vector["manifestBytes"].as_u64().expect("manifest bytes"));
            assert_eq!(hex_lower(&plan.manifest_id.0), vector["manifestId"].as_str().expect("manifest id"));
            assert_eq!(plan.manifest.chunks.first().map(|chunk| hex_lower(&chunk.chunk_id.0)), vector["firstChunkId"].as_str().map(str::to_string));
            assert_eq!(plan.manifest.chunks.last().map(|chunk| hex_lower(&chunk.chunk_id.0)), vector["lastChunkId"].as_str().map(str::to_string));
        };
        for vector in fixture["vectors"].as_array().expect("vectors") {
            assert_vector(vector);
        }
        assert_vector(&fixture["largePair"]["pack"]);
        let ledger = &fixture["retentionLedger"];
        assert_eq!(ledger["reservationMaximumTtlMs"].as_u64(), Some(crate::directory::ARTIFACT_CAS_RESERVATION_MAX_TTL_MS));
        assert_eq!(ledger["reservationGraceMs"].as_u64(), Some(ARTIFACT_CAS_RESERVATION_GRACE_MS));
        assert_eq!(ledger["sweepPageMaximum"].as_u64(), Some(crate::directory::ARTIFACT_CAS_SWEEP_PAGE_MAX as u64));
        assert_eq!(ledger["sweepObjectMaximum"].as_u64(), Some(crate::directory::ARTIFACT_CAS_SWEEP_OBJECT_MAX as u64));
    }

    #[test]
    fn artifact_chunk_cas_ownership_codec_is_canonical_scoped_and_locator_exact() {
        let pair = ArtifactPair { pack: bytes(ARTIFACT_CAS_CHUNK_BYTES + 1), spr: bytes(1) };
        let mut checkpoint = ArtifactCheckpoint {
            scope: DocumentScope::new("space-a", "document-a"),
            checkpoint_id: ArtifactHash([3; 32]),
            parent_checkpoint_id: None,
            descriptor_digest_v1: ArtifactHash([4; 32]),
            baseline_frontier: directory::os_directory::ArtifactFrontier { document_id: "document-a".into(), head_edit_ordinal: 1, head_edit_id: "edit-1".into(), last_commit_seq: 1, chain_hash: ArtifactHash([5; 32]) },
            pack: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.pack)), byte_length: pair.pack.len() as u64, storage_key: String::new() },
            spr: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.spr)), byte_length: pair.spr.len() as u64, storage_key: String::new() },
            aggregate_sha256: ArtifactHash([6; 32]),
            published_at_ms: 1,
        };
        let plan = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("ownership plan");
        let encoded = encode_artifact_cas_ownership_v1(&plan).expect("ownership encoding");
        assert_eq!(decode_artifact_cas_ownership_v1(&encoded).expect("ownership decode"), plan);
        checkpoint.pack.storage_key = artifact_cas_manifest_locator_v1(plan.pack_manifest_id);
        checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(plan.spr_manifest_id);
        validate_artifact_cas_publication_v1(&plan, &checkpoint).expect("exact reserved locators");
        checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(plan.pack_manifest_id);
        assert!(validate_artifact_cas_publication_v1(&plan, &checkpoint).is_err());
        let mut mutated = encoded;
        *mutated.last_mut().expect("ownership byte") ^= 1;
        assert!(decode_artifact_cas_ownership_v1(&mutated).is_err());
    }

    async fn storage_roundtrip_law<S: ArtifactChunkCasStorage>(storage: Arc<S>) {
        let raw = bytes(496 * 1024 + 1);
        let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        let adapter = ArtifactChunkBlobStore::new(storage.clone());
        let staged = adapter.stage("space-a", expected, &raw, &context).await.expect("stage > legacy max");
        assert!(staged.storage_key.starts_with(ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX));
        assert_eq!(adapter.read("space-a", &staged, &context).await.expect("read"), raw);
        assert!(adapter.read("space-b", &staged, &context).await.is_err());
        assert_eq!(adapter.stage("space-a", expected, &raw, &context).await.expect("idempotent"), staged);
    }

    async fn fenced_delete_law<S: ArtifactChunkCasStorage>(storage: Arc<S>, space_id: &str) {
        let raw = bytes(ARTIFACT_CAS_CHUNK_BYTES + 1);
        let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        let adapter = ArtifactChunkBlobStore::new(storage.clone());
        storage.configure_coordinator([1; 32], &context).await.expect("configure deletion coordinator");
        storage.advance_physical_epoch([1; 32], space_id, 1, &context).await.expect("activate deletion epoch");
        let staged = adapter.stage(space_id, expected, &raw, &context).await.expect("stage deletion fixture");
        let plan = prepare_artifact_cas_manifest_v1(space_id, &raw).expect("deletion plan");
        let key = ArtifactCasObjectKey { space_id: space_id.into(), kind: ArtifactCasObjectKind::Chunk, digest: plan.manifest.chunks[0].chunk_id };
        let wrong = ArtifactCasDeleteFence::new(ArtifactCasObjectKey { space_id: "wrong-space".into(), kind: key.kind, digest: key.digest }, 1, [1; 32], 1, [2; 32]);
        assert!(storage.delete_if_unreferenced(&key, &wrong, &context).await.is_err());
        let stale_fence = ArtifactCasDeleteFence::new(key.clone(), 1, [1; 32], 1, [2; 32]);
        storage.advance_physical_epoch([1; 32], space_id, 2, &context).await.expect("activate later reservation epoch");
        assert!(storage.delete_if_unreferenced(&key, &stale_fence, &context).await.is_err());
        assert!(storage.get(&key, &context).await.is_ok());
        let fence = ArtifactCasDeleteFence::new(key.clone(), 1, [1; 32], 2, [2; 32]);
        assert_eq!(storage.delete_if_unreferenced(&key, &fence, &context).await.expect("fenced delete"), ArtifactCasDeleteOutcome::Deleted);
        assert_eq!(storage.delete_if_unreferenced(&key, &fence, &context).await.expect("idempotent fenced delete"), ArtifactCasDeleteOutcome::Missing);
        assert!(adapter.read(space_id, &staged, &context).await.is_err());
    }

    #[tokio::test]
    async fn artifact_chunk_cas_memory_roundtrip_crosses_legacy_payload_ceiling() {
        let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
        storage_roundtrip_law(storage.clone()).await;
        fenced_delete_law(storage, "memory-delete-space").await;
    }

    #[tokio::test]
    async fn artifact_chunk_cas_filesystem_roundtrip_restart_and_collision_checks() {
        let root = std::env::temp_dir().join(format!("semio-artifact-cas-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("clean stale fixture")
        }
        let storage = Arc::new(FsArtifactChunkCasStorage::open(&root).await.expect("open filesystem CAS"));
        storage_roundtrip_law(storage).await;
        let reopened = Arc::new(FsArtifactChunkCasStorage::open(&root).await.expect("reopen filesystem CAS"));
        let raw = bytes(496 * 1024 + 1);
        let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        let plan = prepare_artifact_cas_manifest_v1("space-a", &raw).expect("restart plan");
        let staged = StagedArtifactBlob { storage_key: artifact_cas_manifest_locator_v1(plan.manifest_id), integrity: expected };
        let adapter = ArtifactChunkBlobStore::new(reopened.clone());
        assert_eq!(adapter.read("space-a", &staged, &context).await.expect("read after restart"), raw);
        let first = &plan.manifest.chunks[0];
        let first_key = ArtifactCasObjectKey { space_id: "space-a".into(), kind: ArtifactCasObjectKind::Chunk, digest: first.chunk_id };
        let mut corrupted = bytes(first.byte_length as usize);
        corrupted[0] ^= 1;
        tokio::fs::write(reopened.object_path(&first_key).expect("chunk path"), corrupted).await.expect("inject one-bit corruption");
        assert!(adapter.read("space-a", &staged, &context).await.is_err());
        fenced_delete_law(reopened, "filesystem-delete-space").await;
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[tokio::test]
    async fn artifact_chunk_cas_filesystem_process_epoch_fences_stale_delete() {
        const ROOT_ENV: &str = "SEMIO_ARTIFACT_CAS_PROCESS_FENCE_ROOT";
        const EPOCH_ENV: &str = "SEMIO_ARTIFACT_CAS_PROCESS_FENCE_EPOCH";
        if let (Ok(root), Ok(epoch)) = (std::env::var(ROOT_ENV), std::env::var(EPOCH_ENV)) {
            let storage = FsArtifactChunkCasStorage::open(Path::new(&root)).await.expect("child opens shared filesystem CAS");
            let control = control();
            let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
            storage.configure_coordinator([7; 32], &context).await.expect("child verifies coordinator");
            storage.advance_physical_epoch([7; 32], "process-race-space", epoch.parse().expect("child epoch"), &context).await.expect("child advances shared epoch");
            return;
        }
        let root = std::env::temp_dir().join(format!("semio-artifact-cas-process-fence-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("clean process fence fixture")
        }
        let storage = FsArtifactChunkCasStorage::open(&root).await.expect("open parent filesystem CAS");
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        storage.configure_coordinator([7; 32], &context).await.expect("configure process coordinator");
        storage.advance_physical_epoch([7; 32], "process-race-space", 1, &context).await.expect("activate parent epoch");
        let raw = b"cross-process-fence";
        let key = ArtifactCasObjectKey { space_id: "process-race-space".into(), kind: ArtifactCasObjectKind::Chunk, digest: artifact_cas_chunk_id_v1("process-race-space", raw).expect("process object digest") };
        storage.put_if_absent(&key, raw, &context).await.expect("store process object");
        let executable = std::env::current_exe().expect("test executable");
        let spawn = |epoch: u64| {
            std::process::Command::new(&executable).arg("artifact_chunk_cas_filesystem_process_epoch_fences_stale_delete").arg("--test-threads=1").env(ROOT_ENV, &root).env(EPOCH_ENV, epoch.to_string()).spawn().expect("spawn filesystem fence child")
        };
        let mut second = spawn(2);
        let mut third = spawn(3);
        assert!(second.wait().expect("wait epoch two child").success());
        assert!(third.wait().expect("wait epoch three child").success());
        let stale = ArtifactCasDeleteFence::new(key.clone(), 1, [7; 32], 2, [8; 32]);
        assert!(storage.delete_if_unreferenced(&key, &stale, &context).await.is_err());
        assert_eq!(storage.get(&key, &context).await.expect("stale delete preserves bytes"), raw);
        let current = ArtifactCasDeleteFence::new(key.clone(), 1, [7; 32], 3, [8; 32]);
        assert_eq!(storage.delete_if_unreferenced(&key, &current, &context).await.expect("current process fence deletes"), ArtifactCasDeleteOutcome::Deleted);
        std::fs::remove_dir_all(root).expect("remove process fence fixture");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn artifact_chunk_cas_filesystem_rejects_symlinked_space_lock_and_metadata() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("semio-artifact-cas-symlink-{}", std::process::id()));
        let outside = std::env::temp_dir().join(format!("semio-artifact-cas-symlink-outside-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("clean symlink root")
        }
        if outside.exists() {
            std::fs::remove_dir_all(&outside).expect("clean symlink outside")
        }
        std::fs::create_dir_all(&outside).expect("create symlink outside");
        let storage = FsArtifactChunkCasStorage::open(&root).await.expect("open symlink fixture CAS");
        let descriptor_test = open_artifact_cas_leaf(&root.join("descriptor-flags"), true, true).expect("open descriptor flag fixture");
        use std::os::fd::AsRawFd as _;
        unsafe extern "C" {
            fn fcntl(fd: i32, command: i32, ...) -> i32;
        }
        assert_ne!(unsafe { fcntl(descriptor_test.as_raw_fd(), 1) } & 1, 0, "artifact CAS leaf descriptor is close-on-exec");
        drop(descriptor_test);
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        storage.configure_coordinator([9; 32], &context).await.expect("configure symlink fixture");
        let space_root = root.join(hex_lower(&space_digest("symlink-space").expect("space digest").0));
        symlink(&outside, &space_root).expect("install space symlink");
        assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 1, &context).await.is_err());
        std::fs::remove_file(&space_root).expect("remove space symlink");
        storage.advance_physical_epoch([9; 32], "symlink-space", 1, &context).await.expect("create safe space fence");
        let fence_path = space_root.join("fence-v1");
        std::fs::remove_file(&fence_path).expect("remove safe fence metadata");
        let outside_fence = outside.join("fence-v1");
        std::fs::write(&outside_fence, b"redirected").expect("write outside fence");
        symlink(&outside_fence, &fence_path).expect("install fence symlink");
        assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 2, &context).await.is_err());
        std::fs::remove_file(&fence_path).expect("remove fence symlink");
        let lock_path = space_root.join("fence.lock");
        std::fs::remove_file(&lock_path).expect("remove safe fence lock");
        symlink(&outside_fence, &lock_path).expect("install lock symlink");
        assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 2, &context).await.is_err());
        std::fs::remove_dir_all(root).expect("remove symlink fixture");
        std::fs::remove_dir_all(outside).expect("remove symlink outside");
    }

    #[cfg(feature = "sqlite")]
    #[tokio::test]
    async fn artifact_chunk_cas_sqlite_roundtrip_crosses_legacy_payload_ceiling() {
        let storage = Arc::new(SqliteArtifactChunkCasStorage::memory().await.expect("SQLite CAS"));
        storage_roundtrip_law(storage.clone()).await;
        fenced_delete_law(storage, "sqlite-delete-space").await;
    }

    #[tokio::test]
    async fn artifact_chunk_cas_cancellation_and_max_plus_one_fail_before_storage() {
        let storage = MemoryArtifactChunkCasStorage::default();
        let control = control();
        control.cancelled.store(true, Ordering::SeqCst);
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        let key = ArtifactCasObjectKey { space_id: "space".into(), kind: ArtifactCasObjectKind::Chunk, digest: ArtifactHash([1; 32]) };
        assert!(matches!(storage.put_if_absent(&key, &[1], &context).await, Err(AuthorityError::Cancelled)));
        assert!(prepare_artifact_cas_manifest_v1("space", &vec![0; AUTHORITY_MAX_PAIR_BYTES as usize + 1]).is_err());
        assert!(artifact_cas_chunk_id_v1("space", &vec![0; ARTIFACT_CAS_CHUNK_BYTES + 1]).is_err());
    }
}
