//! 🧬️ Artifact CAS object model: limits and domains, chunk and manifest records, object keys, ownership plans, reservations and the delete fence.

use super::*;

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
pub(super) const ARTIFACT_CAS_COORDINATOR_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.coordinator.v1\0";
pub(super) const ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1: &[u8] = b"semio.hub.artifact-cas.physical-fence.v1\0";
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

    pub(super) fn maximum_bytes(self) -> usize {
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
    pub coordinator_id: [u8; 32],
    pub physical_epoch: u64,
}

impl ArtifactCasReservation {
    pub const fn unfenced(plan: ArtifactCasOwnershipPlanV1, generation: u64, write_epoch: u64, expires_at_ms: u64) -> Self {
        Self { plan, generation, write_epoch, expires_at_ms, coordinator_id: [0; 32], physical_epoch: 0 }
    }

    pub const fn fenced(plan: ArtifactCasOwnershipPlanV1, generation: u64, write_epoch: u64, expires_at_ms: u64, coordinator_id: [u8; 32], physical_epoch: u64) -> Self {
        Self { plan, generation, write_epoch, expires_at_ms, coordinator_id, physical_epoch }
    }

    pub const fn coordinator_id(&self) -> &[u8; 32] {
        &self.coordinator_id
    }

    pub const fn physical_epoch(&self) -> u64 {
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
    pub object: ArtifactCasObjectKey,
    pub ledger_generation: u64,
    pub coordinator_id: [u8; 32],
    pub physical_epoch: u64,
    pub lease_token: [u8; 32],
}

impl ArtifactCasDeleteFence {
    pub fn new(object: ArtifactCasObjectKey, ledger_generation: u64, coordinator_id: [u8; 32], physical_epoch: u64, lease_token: [u8; 32]) -> Self {
        Self { object, ledger_generation, coordinator_id, physical_epoch, lease_token }
    }

    pub const fn ledger_generation(&self) -> u64 {
        self.ledger_generation
    }

    pub const fn object(&self) -> &ArtifactCasObjectKey {
        &self.object
    }

    pub const fn coordinator_id(&self) -> &[u8; 32] {
        &self.coordinator_id
    }

    pub const fn physical_epoch(&self) -> u64 {
        self.physical_epoch
    }

    pub const fn lease_token(&self) -> &[u8; 32] {
        &self.lease_token
    }

    pub(super) fn permits(&self, object: &ArtifactCasObjectKey) -> bool {
        self.ledger_generation > 0 && self.physical_epoch > 0 && self.coordinator_id != [0; 32] && &self.object == object
    }
}

impl std::fmt::Debug for ArtifactCasDeleteFence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ArtifactCasDeleteFence(<opaque>)")
    }
}
