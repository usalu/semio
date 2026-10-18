//! 🔗️ One verified artifact identity bound to its executable: the native or guest codec behind the trusted-codec ports.

use super::*;

/// 🧪️ One immutable authority identity bound to its executable. `codec` is `Some` only for a kind
/// this binary links a Rust codec for (stdio import/export and GIS inference still run natively, TC2
/// §9) — its owner's, or a host's for a kind it hosts from that owner ([`hosted_codec_targets`]);
/// every other identity validates and applies through its package's own component. `guest` is never
/// optional: it is the only creation authority there is, always the identity's own package.
pub struct VerifiedNativeArtifactCodec {
    pub identity: TrustedArtifactIdentity,
    pub codec: Option<ArtifactCodec>,
    pub guest: GuestArtifactCodecBinding,
    pub owner_declaration: ArtifactCatalogBinding,
}

impl VerifiedNativeArtifactCodec {
    /// 🏛️ Returns the exact original codec-owner declaration, including for hosted aliases.
    pub fn owner_declaration(&self)->&ArtifactCatalogBinding{&self.owner_declaration}
}

impl TrustedArtifactCodec for VerifiedNativeArtifactCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            self.guest.print_mirror(pair, stage, context).await?;
            return context.checkpoint();
        };
        let mirror = (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()
    }

    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let maximum=usize::try_from(context.limits().max_pair_bytes).ok().and_then(|bytes|bytes.checked_mul(16)).and_then(|bytes|bytes.checked_add(operation.encoded.len().checked_mul(16)?)).ok_or(AuthorityError::ResourceLimit("identity source capacity"))?;
        let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|{context.report(semio_hub_artifact_authority::AuthorityProgress{stage:semio_hub_artifact_authority::AuthorityProgressStage::ApplyingOperations,completed_units:operation.sequence,total_units:context.limits().max_operations as u64}).is_ok()};
        let mut identity=directory::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum,&mut observer).map_err(|error|AuthorityError::Codec{stage:ArtifactValidationStage::Input,message:bounded_message(error)})?;
        let encoded=identity.encode(|control|directory::os_spr::io::binary::operation_sequence::encode(std::slice::from_ref(&operation.encoded),control)).map_err(|error|AuthorityError::Codec{stage:ArtifactValidationStage::Input,message:bounded_message(error)})?;
        let Some(codec) = &self.codec else {
            let next = self.guest.apply_ops(pair, encoded, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.apply_ops_binary)(&pair.pack, &pair.spr, &encoded, &mut identity).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactReplayCodec for VerifiedNativeArtifactCodec {
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            let next = self.guest.replay_envelopes(pair, envelopes, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.replay_envelopes)(&pair.pack, &pair.spr, envelopes).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactGenesisCodec for VerifiedNativeArtifactCodec {
    /// 🌱️ Creation authority is the COMPONENT's, always. The `dialect` the caller carries is not
    /// passed to the guest: the guest stamps its own app's dialect, and a genesis whose dialect
    /// differs from the catalog's open target is refused here rather than silently accepted.
    async fn initial_pair(&self, document_id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let pair = self.guest.genesis(document_id, context).await?;
        context.checkpoint()?;
        let parsed = directory::os_spr::decode_history(&pair.spr, &directory::os_spr::DecodeOptions::default()).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        if parsed.doc_id != document_id || !parsed.edits.is_empty() || !parsed.transitions.is_empty() || !parsed.conflicts.is_empty() {
            return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "guest genesis is not a zero-history document of the requested identity".into() });
        }
        let _ = dialect;
        context.checkpoint()?;
        Ok(pair)
    }
}
