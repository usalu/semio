//! 🚪️ Physical commitments admit borrowed creation facts before pure transitions.

use super::operation::*;
use crate::directory::error::DirectoryResult;
use directory::os_directory::{ArtifactHash};
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreateV1;
use semio_framework_hash::Sha256;

/// 🔏️ The exact request JSON and length-prefixed scope define one physical command commitment.
pub fn artifact_creation_command_digest_v1(space_id: &str, request: &SpaceArtifactCreateV1) -> DirectoryResult<String> {
    if !super::operation::text(space_id) || !request.validate() {
        return Err(super::operation::rejected("artifact creation intent is invalid"));
    }
    let canonical = semio_framework_pack_json::to_json_string(request);
    let mut digest = Sha256::new();
    digest.update(b"semio.hub.artifact-creation-intent.v1\0");
    for bytes in [space_id.as_bytes(), canonical.as_bytes()] {
        digest.update(&(bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(directory::os_directory::io::binary::artifact_hash::hex_lower(&digest.finalize()))
}

/// 🛡️ Physical intent admission precedes all use of its durable semantic fields.
pub fn validate_artifact_creation_intent_v1(intent: &ArtifactCreationIntentV1) -> DirectoryResult<()> {
    intent.validate_fields()?;
    if intent.command_sha256 != artifact_creation_command_digest_v1(&intent.scope.space_id, &intent.request)? {
        return Err(super::operation::rejected("artifact creation accepted identity is invalid"));
    }
    Ok(())
}

/// 🧬️ Exact pair bytes, storage coordinates and checkpoint commitments admit one preparation.
pub fn validate_artifact_creation_prepared_v1(prepared: &ArtifactCreationPreparedV1, intent: &ArtifactCreationIntentV1) -> DirectoryResult<()> {
    validate_artifact_creation_intent_v1(intent)?;
    prepared.validate_fields(intent)?;
    let d = &prepared.descriptor;
    let c = &prepared.checkpoint;
    let pack = ArtifactHash(Sha256::digest(&prepared.pack));
    let spr = ArtifactHash(Sha256::digest(&prepared.spr));
    if d.bootstrap_snapshot_hash != directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&pack)
        || descriptor_digest_v1(d).ok() != Some(c.descriptor_digest_v1)
        || c.pack.sha256 != pack
        || c.spr.sha256 != spr
        || c.pack.storage_key != format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&pack))
        || c.spr.storage_key != format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&spr))
    {
        return Err(super::operation::rejected("artifact creation prepared pair differs from its accepted intent"));
    }
    let mut aggregate = Sha256::new();
    aggregate.update(&prepared.pack);
    aggregate.update(&prepared.spr);
    if c.aggregate_sha256 != ArtifactHash(aggregate.finalize()) || super::super::checkpoint_id_encoding_v1(c).ok().map(|bytes| ArtifactHash(Sha256::digest(&bytes))) != Some(c.checkpoint_id) {
        return Err(super::operation::rejected("artifact creation prepared integrity differs"));
    }
    Ok(())
}

fn admit_facts(facts: &[ArtifactCreationFactV1]) -> DirectoryResult<AdmittedArtifactCreationFactsV1<'_>> {
    if facts.is_empty() || facts.len() > ARTIFACT_CREATION_FACTS_MAX {
        return Err(super::operation::rejected("artifact creation fact count is invalid"));
    }
    let ArtifactCreationFactBodyV1::Accepted { intent } = &facts[0].body else {
        return Err(super::operation::rejected("artifact creation history has no accepted intent"));
    };
    validate_artifact_creation_intent_v1(intent)?;
    for fact in facts {
        if let ArtifactCreationFactBodyV1::Prepared { candidate } = &fact.body {
            validate_artifact_creation_prepared_v1(candidate, intent)?;
        }
    }
    Ok(AdmittedArtifactCreationFactsV1::new(facts))
}

/// 📖️ Raw durable facts cross physical integrity admission before the semantic history fold.
pub fn fold_artifact_creation_facts_v1(facts: &[ArtifactCreationFactV1]) -> DirectoryResult<ArtifactCreationOperationV1> {
    ArtifactCreationOperationV1::fold(&admit_facts(facts)?)
}

/// ✍️ The candidate body is physically admitted against the immutable accepted fact first.
pub(crate) fn decide_artifact_creation_fact_append_v1(facts: &[ArtifactCreationFactV1], append: &ArtifactCreationFactAppendV1, observed_now_ms: u64) -> DirectoryResult<Option<ArtifactCreationFactV1>> {
    let admitted = admit_facts(facts)?;
    let ArtifactCreationFactBodyV1::Accepted { intent } = &facts[0].body else {
        return Err(super::operation::rejected("artifact creation history has no accepted intent"));
    };
    if let ArtifactCreationFactBodyV1::Prepared { candidate } = &append.body {
        validate_artifact_creation_prepared_v1(candidate, intent)?;
    }
    super::operation::decide_admitted_artifact_creation_fact_append_v1(&admitted, &AdmittedArtifactCreationAppendV1::new(append, intent), observed_now_ms)
}
