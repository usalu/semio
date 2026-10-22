//! 🪪️ Three member decision operations, codecs and testing: the three member decision operations, its dsl, pack and snapshot codecs and the journal record testing helpers. See [`super`].

use super::*;


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
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    ParentMutation: ValueToValue + ValueFromValue + semio_framework_value::retirement::RetireOwned + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    DrawingMutation: ValueToValue + ValueFromValue + semio_framework_value::retirement::RetireOwned + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    ValueMutation: ValueToValue + ValueFromValue + semio_framework_value::retirement::RetireOwned + Send + 'static,
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
