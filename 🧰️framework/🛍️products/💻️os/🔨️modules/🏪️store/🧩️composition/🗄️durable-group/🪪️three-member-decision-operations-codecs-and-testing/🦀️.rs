//! 🪪️ Three member decision operations, codecs and testing: the three member decision operations, its dsl, pack and snapshot codecs and the journal record testing helpers. See [`super`].

use super::*;

impl DurableOwnedThreeMemberDecisionV1 {
    #[cfg(any(test, feature = "durable-group-testing"))]
    pub fn seal_fixture(
        anchor: DurableOwnedGroupAnchorV1,
        mut parent: DurableOwnedGroupMemberV1,
        mut drawing: DurableOwnedGroupMemberV1,
        mut value: DurableOwnedGroupMemberV1,
        outcomes: &DurableStorePreparedOutcomesV1,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        if [&parent, &drawing, &value].iter().any(|member| member.recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES) {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        for (member, outcome) in [(&mut parent, &outcomes.parent), (&mut drawing, &outcomes.drawing), (&mut value, &outcomes.value)] {
            if member.recovery_schema != outcome.recovery_schema || outcome.sha256 != semio_framework_hash::sha256_hex(&outcome.pack) || DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?.recovery_schema != member.recovery_schema {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            member.unbound_outcome_sha256.clone_from(&outcome.sha256);
        }
        let anchor_sha256 = semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(&anchor).as_bytes());
        let mut decision = Self { schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(), anchor, anchor_sha256, decision_sha256: String::new(), parent, drawing, value };
        decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
        for (member, prepared) in [(&mut decision.parent, &outcomes.parent), (&mut decision.drawing, &outcomes.drawing), (&mut decision.value, &outcomes.value)] {
            let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&prepared.pack)?;
            let mut edit: Edit<String> = parse_canonical_json_value(&unbound.edit_without_group_canonical_json)?;
            if edit.mutation_meta.len() != 1 {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
            if meta.group_id.replace(decision.decision_sha256.clone()).is_some() {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            let bound = DurableBoundOneItemOutcomeV1 {
                schema: DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1.into(),
                recovery_schema: unbound.recovery_schema,
                operation: unbound.operation,
                base_generation: unbound.base_generation,
                base_revision: unbound.base_revision,
                base_applied_edit_count: unbound.base_applied_edit_count,
                next_sequence_number: unbound.next_sequence_number,
                next_clock_canonical_json: unbound.next_clock_canonical_json,
                actor: unbound.actor,
                group_id: decision.decision_sha256.clone(),
                edit_canonical_json: semio_framework_pack_json::to_json_string(&edit).into_bytes(),
                post_snapshot_pack: unbound.post_snapshot_pack,
                edit_digest: CursorRevisionAccumulator::edit_digest(&edit),
                post_generation: member.post_generation,
                post_revision: member.post_revision,
            };
            member.recovery_pack = bound.encode_pack();
            member.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&member.recovery_pack);
        }
        decision.validate()?;
        Ok(decision)
    }

    pub fn canonical_json(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    pub fn canonical_unsigned_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&DurableOwnedThreeMemberUnsignedV1::new(self.schema.clone(), self.anchor.clone(), &self.parent, &self.drawing, &self.value))
    }

    pub fn parse_canonical_json(json: &str) -> Result<Self, DurableOwnedGroupDecisionError> {
        if json.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let parsed = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| match error {
            semio_framework_pack_json::JsonError::DuplicateMember { .. } => DurableOwnedGroupDecisionError::NonCanonical,
            other => DurableOwnedGroupDecisionError::Codec(other.to_string()),
        })?;
        let decision = <Self as ValueFromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if decision.canonical_json() != json {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        decision.validate()?;
        Ok(decision)
    }

    pub fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: semio_framework_os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 16,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
            },
        };
        let file = ::semio_framework_async::poll::resolve_ready(pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("durable group manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 7 {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        if manifest.schema_hash != pack::record::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let decision = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        decision.validate()?;
        if decision.encode_pack() != bytes {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(decision)
    }

    pub fn validate(&self) -> Result<(), DurableOwnedGroupDecisionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _,DialectCoordinateText as _};

        if self.schema != DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1 || self.anchor.schema != DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1 || self.anchor.shape != DURABLE_OWNED_GROUP_SHAPE_V1 {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        if !valid_reference(&self.anchor.parent) || self.anchor.parent.dialect.to_coordinate() != "s.gis.gismap@1/*" {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let parent_uri = self.anchor.parent.to_uri();
        if !member_matches(&self.parent, PARENT_ROLE, PARENT_RECOVERY_SCHEMA, &parent_uri, &self.anchor)
            || !member_matches(&self.drawing, DRAWING_ROLE, DRAWING_RECOVERY_SCHEMA, "gismap-drawing!s.stdio.semio@v1/drawing", &self.anchor)
            || !member_matches(&self.value, VALUE_ROLE, VALUE_RECOVERY_SCHEMA, "gismap-value!s.stdio.semio@v1/value", &self.anchor)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
        for member in [&self.parent, &self.drawing, &self.value] {
            if member.recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
                return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
            }
            if !valid_hash(&member.recovery_pack_sha256) || !valid_hash(&member.unbound_outcome_sha256) || member.recovery_pack_sha256 != semio_framework_hash::sha256_hex(&member.recovery_pack) {
                return Err(DurableOwnedGroupDecisionError::InvalidHash);
            }
            let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
            if outcome.recovery_schema != member.recovery_schema
                || outcome.group_id != self.decision_sha256
                || outcome.base_generation != member.expected_generation
                || outcome.base_revision != member.expected_revision
                || outcome.post_generation != member.post_generation
                || outcome.post_revision != member.post_revision
            {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
        }
        let unsigned = self.canonical_unsigned_json();
        if unsigned.len() > DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        if self.anchor_sha256 != semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(&self.anchor).as_bytes()) || self.decision_sha256 != semio_framework_hash::sha256_hex(unsigned.as_bytes()) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        if self.encode_pack().len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        Ok(())
    }

    pub fn admit_map(&self, handles: &DurableOwnedGroupMapHandlesV1, frontiers: &DurableOwnedGroupMapFrontiersV1) -> Result<(), DurableOwnedGroupDecisionError> {
        self.validate()?;
        if handles.drawing != self.drawing.reference || handles.value != self.value.reference {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        if (frontiers.parent_generation, frontiers.parent_revision) != (self.parent.expected_generation, self.parent.expected_revision)
            || (frontiers.drawing_generation, frontiers.drawing_revision) != (self.drawing.expected_generation, self.drawing.expected_revision)
            || (frontiers.value_generation, frontiers.value_revision) != (self.value.expected_generation, self.value.expected_revision)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(())
    }
}

impl ArtifactDsl for DurableOwnedThreeMemberDecisionV1 {
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
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid durable group envelope");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableOwnedThreeMemberDecisionV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "durable group pack envelope mismatch")));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DurableOwnedThreeMemberDecisionV1 {

}

/// 🧪️ Builds the independently specified canonical journal record only for downstream
/// package law suites; production callers must obtain records through Store-owned assembly.
#[cfg(feature = "durable-group-testing")]
pub fn durable_owned_group_journal_test_record(snapshot_json: &str) -> DurableOwnedGroupJournalRecordV1 {
    let fixture: serde_json::Value = serde_json::from_str(snapshot_json).expect("durable group fixture");
    let hex = |value: &str| value.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("fixture hex"), 16).expect("fixture byte")).collect::<Vec<_>>();
    let revision = |value: &str| -> [u8; 32] { hex(value).try_into().expect("fixed fixture revision") };
    let reference = |value: &serde_json::Value| semio_framework_pack_json::from_json_str(&serde_json::to_string(value).expect("fixture reference json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture reference");
    let member = |value: &serde_json::Value| DurableOwnedGroupMemberV1 {
        role: value["role"].as_str().expect("fixture role").to_string(),
        reference: reference(&value["reference"]),
        owner: if value["owner"].is_null() {
            None
        } else {
            Some(semio_framework_pack_json::from_json_str(&serde_json::to_string(&value["owner"]).expect("fixture owner json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture owner"))
        },
        expected_generation: value["expectedGeneration"].as_u64().expect("fixture generation"),
        expected_revision: revision(value["expectedRevisionHex"].as_str().expect("fixture revision")),
        recovery_schema: value["recoverySchema"].as_str().expect("fixture recovery schema").to_string(),
        recovery_pack: Vec::new(),
        recovery_pack_sha256: String::new(),
        unbound_outcome_sha256: String::new(),
        post_generation: value["postGeneration"].as_u64().expect("fixture post generation"),
        post_revision: revision(value["postRevisionHex"].as_str().expect("fixture post revision")),
    };
    let outcome = |value: &serde_json::Value| DurableStorePreparedOutcomeV1 {
        recovery_schema: value["recoverySchema"].as_str().expect("fixture recovery schema").to_string(),
        pack: hex(value["unboundOutcomePackHex"].as_str().expect("fixture unbound outcome")),
        sha256: value["unboundOutcomeSha256"].as_str().expect("fixture unbound outcome hash").to_string(),
    };
    let members = &fixture["members"];
    let outcomes = DurableStorePreparedOutcomesV1 { parent: outcome(&members["parent"]), drawing: outcome(&members["drawing"]), value: outcome(&members["value"]) };
    let decision = DurableOwnedThreeMemberDecisionV1::seal_fixture(
        semio_framework_pack_json::from_json_str(&serde_json::to_string(&fixture["anchor"]).expect("fixture anchor json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture anchor"),
        member(&members["parent"]),
        member(&members["drawing"]),
        member(&members["value"]),
        &outcomes,
    )
    .expect("fixture decision");
    DurableOwnedGroupJournalRecordV1::admit(decision.encode_pack(), &decision.decision_sha256).expect("fixture journal record")
}

/// 🧪️ Builds a typed fixed-three decision Event for downstream WAL laws; production code cannot
/// fabricate this record and must obtain it from the retained Store-owned assembly.
#[cfg(feature = "durable-group-testing")]
pub fn durable_owned_group_journal_test_record_from_edits<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
    document: semio_framework_artifact_reference::ArtifactRef,
    parent: DurableOwnedGroupTestMember<ParentP, ParentMutation>,
    drawing: DurableOwnedGroupTestMember<DrawingP, DrawingMutation>,
    value: DurableOwnedGroupTestMember<ValueP, ValueMutation>,
) -> Result<DurableOwnedGroupJournalRecordV1, DurableOwnedGroupDecisionError>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ParentMutation: ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    DrawingMutation: ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ValueMutation: ValueToValue + ValueFromValue,
{
    let parent_outcome = durable_group_test_outcome(PARENT_RECOVERY_SCHEMA, 1, parent);
    let drawing_outcome = durable_group_test_outcome(DRAWING_RECOVERY_SCHEMA, 2, drawing);
    let value_outcome = durable_group_test_outcome(VALUE_RECOVERY_SCHEMA, 3, value);
    let (parent_outcome, drawing_outcome, value_outcome) = (parent_outcome?, drawing_outcome?, value_outcome?);
    let drawing_reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: "gismap-drawing".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: DRAWING_ROLE.into() } };
    let value_reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: "gismap-value".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: VALUE_ROLE.into() } };
    let anchor = DurableOwnedGroupAnchorV1 { schema: DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1.into(), parent: document.clone(), shape: DURABLE_OWNED_GROUP_SHAPE_V1.into() };
    let mut decision = DurableOwnedThreeMemberDecisionV1 {
        schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(),
        anchor_sha256: semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(&anchor).as_bytes()),
        decision_sha256: String::new(),
        parent: durable_group_test_member(PARENT_ROLE, document.clone(), None, &parent_outcome)?,
        drawing: durable_group_test_member(DRAWING_ROLE, drawing_reference.clone(), Some(OwnerRef { parent: document.clone(), slot: DRAWING_ROLE.into(), child_id: drawing_reference.artifact_id }), &drawing_outcome)?,
        value: durable_group_test_member(VALUE_ROLE, value_reference.clone(), Some(OwnerRef { parent: document, slot: VALUE_ROLE.into(), child_id: value_reference.artifact_id }), &value_outcome)?,
        anchor,
    };
    decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
    durable_group_test_bind_member::<ParentMutation>(&mut decision.parent, &parent_outcome, &decision.decision_sha256)?;
    durable_group_test_bind_member::<DrawingMutation>(&mut decision.drawing, &drawing_outcome, &decision.decision_sha256)?;
    durable_group_test_bind_member::<ValueMutation>(&mut decision.value, &value_outcome, &decision.decision_sha256)?;
    decision.validate()?;
    DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack())
}
