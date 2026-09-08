//! 🌱️ Host-owned artifact genesis and creation receipt boundaries.

use super::{ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, CheckpointCandidate, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactGenesisCodec, TrustedArtifactIdentity, ValidatingCanonicalArtifactAuthority};
use directory::os_directory::{ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentDescriptor, DocumentFrontier, DocumentOwner, DocumentScope};
use directory::os_io::ArtifactDialect;
use semio_framework_hash::Sha256;

#[path = "🧬️schema/🦀️.rs"]
mod operation;
pub use operation::*;

#[path = "🧑‍🏭️service-v1/🦀️.rs"]
mod service;
pub use service::ArtifactCreationServiceV1;

/// 🪪️ Only a freshly authenticated server session constructs execution authority.
#[derive(Clone, Debug, PartialEq, Eq, directory::ToValue, directory::FromValue)]
#[value(crate = "::directory", rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactCreationActorV1 {
    pub user_id: String,
    pub session_id: String,
    pub authorization_generation: u64,
}

/// 🔐️ An opaque bin-owned final writer fence is retained through commit and publication.
pub trait ArtifactCreationCommitLeaseV1: Send {}

/// ⏳️ Final authority acquisition is bounded by the request owner without exposing mutex types.
pub type ArtifactCreationCommitFutureV1<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = crate::directory::error::DirectoryResult<Box<dyn ArtifactCreationCommitLeaseV1>>> + Send + 'a>>;

/// 🛂️ Reacquires live user/session/space/membership gates only after private CAS staging.
pub trait ArtifactCreationCommitAuthorityV1: Send + Sync {
    fn acquire<'a>(&'a self, actor: &'a ArtifactCreationActorV1, space_id: &'a str) -> ArtifactCreationCommitFutureV1<'a>;
}

/// 🪪️ Server-owned creation coordinates; no client supplies pair bytes or a descriptor.
pub struct ArtifactGenesisRequest {
    pub scope: DocumentScope,
    pub kind_id: String,
}

/// 🪺️ The descriptor and initial checkpoint are derived together from one factory invocation.
pub struct ArtifactGenesisCandidate {
    pub descriptor: DocumentDescriptor,
    pub candidate: CheckpointCandidate,
}

impl ValidatingCanonicalArtifactAuthority<std::sync::Arc<super::trusted_catalog::VerifiedTrustedCatalog>> {
    /// 🌱️ Builds one private zero-history candidate without publishing or applying a domain edit.
    pub async fn materialize_genesis(&self, request: ArtifactGenesisRequest, context: &OperationContext<'_>) -> Result<ArtifactGenesisCandidate, AuthorityError> {
        context.checkpoint()?;
        let selected = self.catalog.artifact_creation_selection(&request.kind_id).ok_or_else(|| AuthorityError::Catalog("creation kind has no unambiguous verified editor".into()))?;
        let identity = TrustedArtifactIdentity {
            plugin_id: selected.package.plugin_id.clone(), package_id: selected.package.package_id.clone(), version: selected.package.version.clone(), package_hash: selected.package.component_sha256.clone(),
            artifact_kind: selected.artifact.kind.clone(), artifact_schema: selected.artifact.schema.clone(), pack_schema_hash: selected.artifact.pack_schema_hash.clone(),
        };
        materialize_selected_genesis(self.catalog.as_ref(), request, identity, selected.parent_dialect.clone(), context).await
    }
}

async fn materialize_selected_genesis<C: TrustedArtifactCatalog>(catalog: &C, request: ArtifactGenesisRequest, identity: TrustedArtifactIdentity, dialect: ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactGenesisCandidate, AuthorityError> where C::Codec: TrustedArtifactGenesisCodec {
        context.checkpoint()?;
        let target = directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationReadyV1 {
            document_id: request.scope.document_id.clone(), kind_id: identity.artifact_kind.clone(), artifact_schema: identity.artifact_schema.clone(),
            parent_dialect: directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationDialectV1 { artifact_kind: dialect.artifact_kind.clone(), standard: dialect.standard.clone(), subset: dialect.subset.clone() },
        };
        if !target.validate() || request.scope.space_id.is_empty() || request.scope.space_id.len() > 256 || request.scope.space_id.chars().any(char::is_control) {
            return Err(AuthorityError::InvalidScope);
        }
        if context.now_ms() > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER { return Err(AuthorityError::ResourceLimit("genesis publication clock")); }
        let limits = context.limits();
        if limits.max_pair_bytes == 0 || limits.max_pair_bytes > super::AUTHORITY_MAX_PAIR_BYTES || limits.max_operations == 0 || limits.max_operations > super::AUTHORITY_MAX_OPERATIONS || limits.max_operation_bytes == 0 || limits.max_operation_bytes > super::AUTHORITY_MAX_OPERATION_BYTES { return Err(AuthorityError::InvalidLimits); }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 1, total_units: 5 })?;
        let codec = catalog.resolve(&identity).await?;
        if codec.identity() != &identity || request.kind_id != identity.artifact_kind { return Err(AuthorityError::CodecIdentityMismatch); }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: 2, total_units: 5 })?;
        let pair: ArtifactPair = codec.initial_pair(&request.scope.document_id, &dialect, context).await?;
        if pair.pack.is_empty() || pair.spr.is_empty() { return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "genesis requires a complete nonempty pair".into() }); }
        super::validate_pair_budget(&pair, limits, ArtifactValidationStage::Input)?;
        codec.validate_pair(&pair, ArtifactValidationStage::Input, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::InputValidated, completed_units: 3, total_units: 5 })?;
        codec.validate_pair(&pair, ArtifactValidationStage::Output, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::OutputValidated, completed_units: 4, total_units: 5 })?;
        let descriptor = DocumentDescriptor {
            space_id: request.scope.space_id.clone(), document_id: request.scope.document_id.clone(), artifact_kind: identity.artifact_kind, artifact_schema: identity.artifact_schema,
            owner: DocumentOwner { plugin_id: identity.plugin_id, package_id: identity.package_id, version: identity.version, package_hash: identity.package_hash },
            pack_schema_hash: identity.pack_schema_hash, bootstrap_version: 1, bootstrap_frontier: DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 }, bootstrap_snapshot_hash: directory::os_directory::hex_lower(&Sha256::digest(&pair.pack)),
        };
        let descriptor_digest_v1 = directory::os_directory::descriptor_digest_v1(&descriptor).map_err(|error| AuthorityError::InvalidDescriptor(error.to_string()))?;
        let mut aggregate = Sha256::new();
        aggregate.update(&pair.pack);
        aggregate.update(&pair.spr);
        context.checkpoint()?;
        let published_at_ms = context.now_ms();
        if published_at_ms > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER { return Err(AuthorityError::ResourceLimit("genesis publication clock")); }
        let mut checkpoint = ArtifactCheckpoint {
            baseline_frontier: ArtifactFrontier { document_id: request.scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash([0; 32]) },
            scope: request.scope, checkpoint_id: ArtifactHash([0; 32]), parent_checkpoint_id: None, descriptor_digest_v1, pack: super::blob_reference(&pair.pack)?, spr: super::blob_reference(&pair.spr)?, aggregate_sha256: ArtifactHash(aggregate.finalize()), published_at_ms,
        };
        checkpoint.checkpoint_id = ArtifactHash(Sha256::digest(&super::checkpoint_id_encoding_v1(&checkpoint)?));
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Derived, completed_units: 5, total_units: 5 })?;
        Ok(ArtifactGenesisCandidate { descriptor, candidate: CheckpointCandidate { checkpoint, pair } })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
