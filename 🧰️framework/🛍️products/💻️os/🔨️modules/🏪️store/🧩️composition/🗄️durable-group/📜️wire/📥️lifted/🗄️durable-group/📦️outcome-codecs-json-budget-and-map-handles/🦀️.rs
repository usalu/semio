//! 📦️ Outcome codecs, JSON budget and map handles: the outcome parsing and verification, the canonical json parse and budget, the dsl, pack and snapshot codecs of the outcomes and the map handles and frontiers. See [`super`].

use super::*;

impl DurableUnboundOneItemOutcomeV1 {
    pub fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: semio_framework_os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = ::semio_framework_async::poll::resolve_ready(pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("unbound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 11 || manifest.schema_hash != pack::record::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let outcome = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if outcome.schema != DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1
            || !valid_identity(&outcome.recovery_schema)
            || !valid_identity(&outcome.actor)
            || outcome.post_snapshot_pack.is_empty()
            || outcome.post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES
            || outcome.encode_pack() != bytes
        {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(outcome)
    }
}

impl DurableStorePreparedOutcomeV1 {
    pub fn from_prepared<P, Mutation>(recovery_schema: &str, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> Result<Self, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        Mutation: ValueToValue + ValueFromValue,
    {
        let authority = &prepared.authority();
        if !valid_identity(recovery_schema) || authority.group_id().is_some() || !valid_edit_identities(prepared.edit()) {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        authority.validate_prepared(prepared).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let post_snapshot_pack = prepared.post_snapshot().encode_pack();
        if post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let outcome = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: recovery_schema.into(),
            operation: authority.operation().0,
            base_generation: authority.generation().0,
            base_revision: authority.base_revision(),
            base_applied_edit_count: authority.base_applied_edit_count().try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            next_sequence_number: authority.next_sequence_number(),
            next_clock_canonical_json: semio_framework_pack_json::to_json_string(&authority.next_clock()).into_bytes(),
            actor: authority.actor_utf8().clone(),
            edit_without_group_canonical_json: semio_framework_pack_json::to_json_string(prepared.edit()).into_bytes(),
            post_snapshot_pack,
        };
        let pack = outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let owned = Self { recovery_schema: recovery_schema.into(), sha256: semio_framework_hash::sha256_hex(&pack), pack };
        let verified = owned.verify_inverse::<P, Mutation>()?;
        if verified.authority.operation() != authority.operation()
            || verified.authority.generation() != authority.generation()
            || verified.authority.base_revision() != authority.base_revision()
            || verified.authority.base_applied_edit_count() != authority.base_applied_edit_count()
            || verified.authority.next_sequence_number() != authority.next_sequence_number()
            || verified.authority.next_clock() != authority.next_clock()
            || verified.authority.actor() != authority.actor()
            || semio_framework_value::ToValue::to_value(verified.edit.as_ref()) != semio_framework_value::ToValue::to_value(prepared.edit())
            || verified.post_snapshot.encode_pack() != prepared.post_snapshot().encode_pack()
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        Ok(owned)
    }

    pub fn verify_inverse<P, Mutation>(&self) -> Result<DurableStoreVerifiedOutcomeV1<P, Mutation>, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        Mutation: ValueToValue + ValueFromValue,
    {
        if self.sha256 != semio_framework_hash::sha256_hex(&self.pack) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let outcome = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&self.pack)?;
        if outcome.recovery_schema != self.recovery_schema {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let next_clock: HybridLogicalTimestamp = parse_canonical_json_value(&outcome.next_clock_canonical_json)?;
        let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_without_group_canonical_json)?;
        if !valid_edit_identities(&edit) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let post_options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: semio_framework_os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let post_snapshot = P::decode_pack_with(&outcome.post_snapshot_pack, &post_options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if post_snapshot.encode_pack() != outcome.post_snapshot_pack {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let base_applied_edit_count = outcome.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
            semio_framework_job::OperationId(outcome.operation),
            semio_framework_job::Generation(outcome.base_generation),
            outcome.base_revision,
            base_applied_edit_count,
            outcome.next_sequence_number,
            next_clock,
            outcome.actor,
            edit.line.clone(),
            None,
            None,
        ));
        authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        Ok(DurableStoreVerifiedOutcomeV1 { authority, edit: Box::new(edit), post_snapshot: Arc::new(post_snapshot) })
    }
}

pub fn parse_canonical_json_value<T>(bytes: &[u8]) -> Result<T, DurableOwnedGroupDecisionError>
where
    T: ValueToValue + ValueFromValue,
{
    validate_json_budget(bytes)?;
    let json = std::str::from_utf8(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    let value = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    if semio_framework_pack_json::to_json_string(&value).as_bytes() != bytes {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(value)
}

pub fn validate_json_budget(bytes: &[u8]) -> Result<(), DurableOwnedGroupDecisionError> {
    if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
        return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
    }
    let mut depth = 0usize;
    let mut items = usize::from(!bytes.is_empty());
    let mut in_string = false;
    let mut escaped = false;
    for byte in bytes.iter().copied() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else if byte < 0x20 {
                return Err(DurableOwnedGroupDecisionError::NonCanonical);
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth = depth.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if depth > DURABLE_OWNED_GROUP_JSON_MAX_DEPTH {
                    return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
                }
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1).ok_or(DurableOwnedGroupDecisionError::NonCanonical)?;
            }
            b',' => {
                items = items.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if items > DURABLE_OWNED_GROUP_JSON_MAX_ITEMS {
                    return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
                }
            }
            _ => {}
        }
    }
    if depth != 0 || in_string || escaped {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(())
}

pub fn valid_edit_identities<Mutation>(edit: &Edit<Mutation>) -> bool {
    valid_identity(&edit.id)
        && edit.actor.as_deref().is_none_or(valid_identity)
        && edit.mutation_meta.iter().all(|meta| {
            meta.mutation_id.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.dependencies.iter().all(|id| valid_identity(&id.0))
                && meta.author_id.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.semantic_kind.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.group_id.as_deref().is_none_or(valid_identity)
        })
}

impl DurableBoundOneItemOutcomeV1 {
    pub fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: semio_framework_os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = ::semio_framework_async::poll::resolve_ready(pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("bound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 15 || manifest.schema_hash != pack::record::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let outcome = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if outcome.schema != DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1
            || !valid_identity(&outcome.recovery_schema)
            || !valid_identity(&outcome.actor)
            || !valid_hash(&outcome.group_id)
            || outcome.post_snapshot_pack.is_empty()
            || outcome.post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES
            || outcome.encode_pack() != bytes
        {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(outcome)
    }

    pub fn unbound_pack<Mutation>(&self) -> Result<Vec<u8>, DurableOwnedGroupDecisionError>
    where
        Mutation: ValueToValue + ValueFromValue,
    {
        let mut edit: Edit<Mutation> = parse_canonical_json_value(&self.edit_canonical_json)?;
        if edit.mutation_meta.len() != 1 {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if meta.group_id.as_deref() != Some(self.group_id.as_str()) {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        meta.group_id = None;
        if !valid_edit_identities(&edit) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let outcome = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: self.recovery_schema.clone(),
            operation: self.operation,
            base_generation: self.base_generation,
            base_revision: self.base_revision,
            base_applied_edit_count: self.base_applied_edit_count,
            next_sequence_number: self.next_sequence_number,
            next_clock_canonical_json: self.next_clock_canonical_json.clone(),
            actor: self.actor.clone(),
            edit_without_group_canonical_json: semio_framework_pack_json::to_json_string(&edit).into_bytes(),
            post_snapshot_pack: self.post_snapshot_pack.clone(),
        };
        outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DurableMemberRecoveryFrontier {
    Base,
    Post,
}

impl ArtifactDsl for DurableUnboundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = semio_framework_os_semio::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid unbound outcome envelope");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableUnboundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unbound outcome pack envelope mismatch")));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DurableUnboundOneItemOutcomeV1 {

}

impl ArtifactDsl for DurableBoundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = semio_framework_os_semio::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid bound outcome envelope");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableBoundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "bound outcome pack envelope mismatch")));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DurableBoundOneItemOutcomeV1 {

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupMapHandlesV1 {
    pub drawing: semio_framework_artifact_reference::ArtifactRef,
    pub value: semio_framework_artifact_reference::ArtifactRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupMapFrontiersV1 {
    pub parent_generation: u64,
    pub parent_revision: [u8; 32],
    pub drawing_generation: u64,
    pub drawing_revision: [u8; 32],
    pub value_generation: u64,
    pub value_revision: [u8; 32],
}

pub fn valid_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

pub fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= DURABLE_OWNED_GROUP_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

pub fn valid_reference(reference: &semio_framework_artifact_reference::ArtifactRef) -> bool {
    valid_identity(&reference.artifact_id) && valid_identity(&reference.dialect.artifact_kind) && valid_identity(&reference.dialect.standard) && valid_identity(&reference.dialect.subset)
}
