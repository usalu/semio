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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasReservation {
    pub plan: ArtifactCasOwnershipPlanV1,
    pub generation: u64,
    pub write_epoch: u64,
    pub expires_at_ms: u64,
}

/// 🛡️ Non-forgeable in-crate proof that the directory rechecked one object at a ledger generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCasDeleteFence {
    object: ArtifactCasObjectKey,
    ledger_generation: u64,
}

impl ArtifactCasDeleteFence {
    pub(crate) fn new(object: ArtifactCasObjectKey, ledger_generation: u64) -> Self {
        Self { object, ledger_generation }
    }

    pub const fn ledger_generation(&self) -> u64 {
        self.ledger_generation
    }

    fn permits(&self, object: &ArtifactCasObjectKey) -> bool {
        self.ledger_generation > 0 && &self.object == object
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
    /// ➕️ Inserts an exact verified object or confirms byte-for-byte identity on collision.
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError>;

    /// 📖️ Reads one exact object with a kind-specific admission bound.
    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError>;

    /// 🧹️ Deletes only with an exact fence minted by the directory's immediate reachability recheck.
    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError>;
}

impl<T: ArtifactChunkCasStorage> ArtifactChunkCasStorage for Arc<T> {
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
            if remainder == 0 { ARTIFACT_CAS_CHUNK_BYTES as u64 } else { remainder }
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
    if space_id != expected_space_id || raw_byte_length > AUTHORITY_MAX_PAIR_BYTES || chunk_bytes != ARTIFACT_CAS_CHUNK_BYTES as u32 || chunk_count > ARTIFACT_CAS_MAX_CHUNKS || chunk_count as u64 != raw_byte_length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES as u64) {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest shape"));
    }
    let mut chunks = Vec::with_capacity(chunk_count);
    for index in 0..chunk_count {
        let ordinal = u32_field(&mut cursor)?;
        let byte_length = u32_field(&mut cursor)?;
        let chunk_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS chunk hash"))?);
        let expected_length = if index + 1 == chunk_count {
            let remainder = raw_byte_length % ARTIFACT_CAS_CHUNK_BYTES as u64;
            if remainder == 0 { ARTIFACT_CAS_CHUNK_BYTES as u64 } else { remainder }
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
    let manifest = ArtifactCasManifestV1 {
        space_id: space_id.to_string(),
        raw_sha256: ArtifactHash(Sha256::digest(raw)),
        raw_byte_length: raw.len() as u64,
        chunk_bytes: ARTIFACT_CAS_CHUNK_BYTES as u32,
        chunks,
    };
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
    if checkpoint.pack.sha256 != pack.manifest.raw_sha256
        || checkpoint.pack.byte_length != pack.manifest.raw_byte_length
        || checkpoint.spr.sha256 != spr.manifest.raw_sha256
        || checkpoint.spr.byte_length != spr.manifest.raw_byte_length
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership raw identity"));
    }
    let mut objects = Vec::with_capacity(pack.manifest.chunks.len() + spr.manifest.chunks.len() + 2);
    objects.extend(pack.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.extend(spr.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: pack.manifest_id });
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: spr.manifest_id });
    objects.sort_by_key(|object| (object.kind, object.digest.0));
    objects.dedup();
    let plan = ArtifactCasOwnershipPlanV1 {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        pack_manifest_id: pack.manifest_id,
        spr_manifest_id: spr.manifest_id,
        objects,
    };
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
    objects: Mutex<HashMap<(ArtifactHash, ArtifactCasObjectKind, ArtifactHash), Vec<u8>>>,
}

impl MemoryArtifactChunkCasStorage {
    fn map_key(key: &ArtifactCasObjectKey) -> Result<(ArtifactHash, ArtifactCasObjectKind, ArtifactHash), AuthorityError> {
        Ok((space_digest(&key.space_id)?, key.kind, key.digest))
    }
}

impl ArtifactChunkCasStorage for MemoryArtifactChunkCasStorage {
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let mut objects = self.objects.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match objects.get(&Self::map_key(key)?) {
            Some(existing) if existing == bytes => Ok(ArtifactCasPutOutcome::AlreadyPresent),
            Some(_) => Err(AuthorityError::Store("artifact CAS immutable key collision".to_string())),
            None => {
                objects.insert(Self::map_key(key)?, bytes.to_vec());
                Ok(ArtifactCasPutOutcome::Inserted)
            }
        }
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let bytes = self.objects.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&Self::map_key(key)?).cloned().ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let removed = self.objects.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&Self::map_key(key)?).is_some();
        Ok(if removed { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}

/// 📁️ Dedicated filesystem CAS rooted below `artifact-cas/v1`.
pub struct FsArtifactChunkCasStorage {
    root: PathBuf,
    nonce: std::sync::atomic::AtomicU64,
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

    async fn read_path(&self, key: &ArtifactCasObjectKey, path: &Path, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        use tokio::io::AsyncReadExt as _;
        context.checkpoint()?;
        let file = tokio::fs::File::open(path).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object not found".to_string()))?;
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
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        use tokio::io::AsyncWriteExt as _;
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let target = self.object_path(key)?;
        let parent = target.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem object parent missing".to_string()))?;
        tokio::fs::create_dir_all(parent).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object directory create failed".to_string()))?;
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
        match tokio::fs::remove_file(self.object_path(key)?).await {
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
            let inserted = transaction.execute(
                "INSERT OR IGNORE INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![tenant, kind, digest, expected, length],
            ).map_err(|_| AuthorityError::Store("artifact CAS SQLite insert failed".to_string()))? == 1;
            let stored_length: Option<i64> = transaction.query_row(
                "SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3",
                rusqlite::params![tenant, kind, digest],
                |row| row.get(0),
            ).optional().map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?;
            if stored_length != Some(length) {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            let stored: Vec<u8> = transaction.query_row(
                "SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3",
                rusqlite::params![tenant, kind, digest],
                |row| row.get(0),
            ).map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))?;
            if stored != expected {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            transaction.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".to_string()))?;
            Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
        }).await.map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
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
            let length: i64 = connection.query_row(
                "SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3",
                rusqlite::params![tenant, kind, digest],
                |row| row.get(0),
            ).optional().map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?.ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
            if length < 0 || usize::try_from(length).ok().is_none_or(|length| length > maximum) {
                return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
            }
            connection.query_row(
                "SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3",
                rusqlite::params![tenant, kind, digest],
                |row| row.get(0),
            ).map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))
        }).await.map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
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
        let deleted = tokio::task::spawn_blocking(move || -> Result<bool, AuthorityError> {
            let connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            Ok(connection.execute(
                "DELETE FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3",
                rusqlite::params![tenant, kind, digest],
            ).map_err(|_| AuthorityError::Store("artifact CAS SQLite delete failed".to_string()))? == 1)
        }).await.map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
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
        sqlx_core::query::query("CREATE TABLE IF NOT EXISTS hub_artifact_cas_object (
            space_digest BYTEA NOT NULL CHECK(octet_length(space_digest) = 32),
            kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
            object_digest BYTEA NOT NULL CHECK(octet_length(object_digest) = 32),
            bytes BYTEA NOT NULL,
            byte_length BIGINT NOT NULL,
            PRIMARY KEY(space_digest, kind, object_digest),
            CHECK(byte_length = octet_length(bytes)),
            CHECK((kind = 'chunk' AND byte_length BETWEEN 1 AND 262144) OR (kind = 'manifest' AND byte_length BETWEEN 1 AND 65536))
        )").execute(&pool).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL schema failed".to_string()))?;
        Ok(Self { pool })
    }
}

#[cfg(feature = "postgres")]
impl ArtifactChunkCasStorage for PostgresArtifactChunkCasStorage {
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let length = i64::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS object byte"))?;
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".to_string()))?;
        let inserted = sqlx_core::query::query("INSERT INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
            .bind(&tenant).bind(key.kind.name()).bind(&digest).bind(bytes).bind(length).execute(&mut *transaction).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL insert failed".to_string()))?.rows_affected() == 1;
        let row: (i64, Vec<u8>) = sqlx_core::query_as::query_as("SELECT byte_length, bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant).bind(key.kind.name()).bind(&digest).fetch_one(&mut *transaction).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL object read failed".to_string()))?;
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
            .bind(&tenant).bind(key.kind.name()).bind(&digest).fetch_one(&self.pool).await.map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        if length.0 < 0 || usize::try_from(length.0).ok().is_none_or(|length| length > key.kind.maximum_bytes()) {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let row: (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant).bind(key.kind.name()).bind(&digest).fetch_one(&self.pool).await.map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
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
        let deleted = sqlx_core::query::query("DELETE FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant).bind(key.kind.name()).bind(&digest).execute(&self.pool).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL delete failed".to_string()))?.rows_affected() == 1;
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
        Ok(Self { graph })
    }

    fn key(key: &ArtifactCasObjectKey) -> Result<String, AuthorityError> {
        Ok(format!("{}:{}:{}", hex_lower(&space_digest(&key.space_id)?.0), key.kind.name(), hex_lower(&key.digest.0)))
    }
}

#[cfg(feature = "neo4j")]
impl ArtifactChunkCasStorage for Neo4jArtifactChunkCasStorage {
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
        let mut result = self.graph.execute(neo4rs::query("MATCH (o:ArtifactCasObject {key: $key}) RETURN o.byteLength AS length, o.bytes AS bytes").param("key", Self::key(key)?)).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j read failed".to_string()))?;
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
        let mut result = self.graph.execute(neo4rs::query("MATCH (o:ArtifactCasObject {key: $key}) WITH collect(o) AS objects, count(o) AS deleted FOREACH (object IN objects | DELETE object) RETURN deleted").param("key", Self::key(key)?)).await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j delete failed".to_string()))?;
        let row = result.next().await.map_err(|_| AuthorityError::Store("artifact CAS Neo4j delete failed".to_string()))?.ok_or_else(|| AuthorityError::Store("artifact CAS Neo4j returned no deletion result".to_string()))?;
        let deleted: i64 = row.get("deleted").map_err(|_| AuthorityError::Store("artifact CAS Neo4j deletion decode failed".to_string()))?;
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
        fn now_ms(&self) -> u64 { self.now.load(Ordering::SeqCst) }
        fn is_cancelled(&self) -> bool { self.cancelled.load(Ordering::SeqCst) }
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json")).expect("fixture JSON");
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
            scope: DocumentScope::new("space-a", "document-a"), checkpoint_id: ArtifactHash([3; 32]), parent_checkpoint_id: None,
            descriptor_digest_v1: ArtifactHash([4; 32]),
            baseline_frontier: directory::os_directory::ArtifactFrontier { document_id: "document-a".into(), head_edit_ordinal: 1, head_edit_id: "edit-1".into(), last_commit_seq: 1, chain_hash: ArtifactHash([5; 32]) },
            pack: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.pack)), byte_length: pair.pack.len() as u64, storage_key: String::new() },
            spr: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.spr)), byte_length: pair.spr.len() as u64, storage_key: String::new() },
            aggregate_sha256: ArtifactHash([6; 32]), published_at_ms: 1,
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
        let staged = adapter.stage(space_id, expected, &raw, &context).await.expect("stage deletion fixture");
        let plan = prepare_artifact_cas_manifest_v1(space_id, &raw).expect("deletion plan");
        let key = ArtifactCasObjectKey { space_id: space_id.into(), kind: ArtifactCasObjectKind::Chunk, digest: plan.manifest.chunks[0].chunk_id };
        let wrong = ArtifactCasDeleteFence::new(ArtifactCasObjectKey { space_id: "wrong-space".into(), kind: key.kind, digest: key.digest }, 1);
        assert!(storage.delete_if_unreferenced(&key, &wrong, &context).await.is_err());
        let fence = ArtifactCasDeleteFence::new(key.clone(), 1);
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
        if root.exists() { std::fs::remove_dir_all(&root).expect("clean stale fixture") }
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
