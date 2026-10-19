//! 🧪️ Member identity and group testing helpers: the unsigned member, the member identity matches and the durable group testing outcome and member helpers. See [`super`].

use super::*;

pub(super) fn unsigned_member(member: &DurableOwnedGroupMemberV1) -> DurableOwnedThreeMemberUnsignedMemberV1 {
    DurableOwnedThreeMemberUnsignedMemberV1 {
        role: member.role.clone(),
        reference: member.reference.clone(),
        owner: member.owner.clone(),
        expected_generation: member.expected_generation,
        expected_revision: member.expected_revision,
        recovery_schema: member.recovery_schema.clone(),
        unbound_outcome_sha256: member.unbound_outcome_sha256.clone(),
    }
}

pub fn member_identity_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    if member.role != role || member.recovery_schema != recovery_schema || member.reference.to_uri() != reference_uri || !valid_reference(&member.reference) || !valid_identity(&member.recovery_schema) {
        return false;
    }
    match (role, &member.owner) {
        (PARENT_ROLE, None) => member.reference == anchor.parent,
        (DRAWING_ROLE, Some(owner)) => owner.parent == anchor.parent && owner.slot == DRAWING_ROLE && owner.child_id == "gismap-drawing",
        (VALUE_ROLE, Some(owner)) => owner.parent == anchor.parent && owner.slot == VALUE_ROLE && owner.child_id == "gismap-value",
        _ => false,
    }
}

pub fn member_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
    member_identity_matches(member, role, recovery_schema, reference_uri, anchor) && member.expected_generation.checked_add(1) == Some(member.post_generation) && member.post_revision != member.expected_revision
}

#[cfg(feature = "durable-group-testing")]
fn durable_group_test_revision(bytes: &[u8]) -> [u8; 32] {
    let digest = semio_framework_hash::sha256_hex(bytes);
    std::array::from_fn(|index| u8::from_str_radix(&digest[index * 2..index * 2 + 2], 16).expect("SHA-256 hexadecimal byte"))
}

/// 🧪️ One caller-issued fixture candidate and the exact issuers that close its original owners.
#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_outcome<P, Mutation>(recovery_schema: &str, ordinal: u64, edit: Edit<Mutation>, post_snapshot: P) -> Result<DurableStorePreparedOutcomeV1, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    Mutation: ValueToValue + ValueFromValue,
{
    let actor = edit.actor.clone().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
    let meta = edit.mutation_meta.first().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
        semio_framework_job::OperationId(10_000 + ordinal),
        semio_framework_job::Generation(ordinal),
        durable_group_test_revision(format!("durable-group-test-base-{ordinal}").as_bytes()),
        0,
        edit.sequence_number,
        meta.timestamp,
        actor,
        edit.line.clone(),
        meta.group_id.clone(),
        None,
    ));
    let prepared = authority.prepare_one_item(edit, Arc::new(post_snapshot)).map_err(DurableOwnedGroupDecisionError::Codec)?;
    DurableStorePreparedOutcomeV1::from_prepared(recovery_schema, &prepared)
}

#[cfg(feature = "durable-group-testing")]
fn durable_group_test_outcome<P, Mutation>(recovery_schema: &str, ordinal: u64, member: DurableOwnedGroupTestMember<P, Mutation>) -> Result<DurableStorePreparedOutcomeV1, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: ValueToValue + ValueFromValue + semio_framework_value::retirement::RetireOwned,
{
    let mut owners = semio_framework_os_store_one_item_batch::OneItemOwners::from_returned(member.edit, member.post_snapshot, member.mutation_retirement, member.snapshot_retirement);
    let outcome = (|| {
        let (edit, _) = owners.refused.as_ref().expect("original fixture candidates");
        let actor = edit.actor.clone().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
        let meta = edit.mutation_meta.first().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        *owners.authority = Some(Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(10_000 + ordinal), semio_framework_job::Generation(ordinal), durable_group_test_revision(format!("durable-group-test-base-{ordinal}").as_bytes()), 0, edit.sequence_number, meta.timestamp, actor, edit.line.clone(), meta.group_id.clone(), None)));
        let (edit, post) = owners.refused.take().expect("original fixture handoff");
        match owners.authority.as_ref().expect("original fixture authority").prepare_one_item(edit, post) {
            Ok(prepared) => *owners.prepared = Some(prepared),
            Err((error, edit, post)) => {
                *owners.refused = Some((edit, post));
                return Err(DurableOwnedGroupDecisionError::Preparation(error));
            }
        }
        DurableStorePreparedOutcomeV1::from_prepared(recovery_schema, owners.prepared.as_ref().expect("sealed original fixture"))
    })();
    owners.begin_returned_close().expect("original returned fixture cursor admission");
    while !owners.terminal_is_empty() {
        let copy = owners.close_demands(0).expect("fixture original copy quote").copy_bytes;
        let demand = owners.close_demands(copy).expect("fixture original close quote");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let step = owners.close_step(grant).expect("caller-issued fixture original closure");
        assert!(step.progress().fits(grant));
    }
    outcome
}
#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_member(role: &str, reference: semio_framework_artifact_reference::ArtifactRef, owner: Option<OwnerRef>, outcome: &DurableStorePreparedOutcomeV1) -> Result<DurableOwnedGroupMemberV1, DurableOwnedGroupDecisionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?;
    let post_revision = durable_group_test_revision(format!("{}:{}:{}", role, reference.to_uri(), outcome.sha256).as_bytes());
    Ok(DurableOwnedGroupMemberV1 {
        role: role.into(),
        reference,
        owner,
        expected_generation: unbound.base_generation,
        expected_revision: unbound.base_revision,
        recovery_schema: outcome.recovery_schema.clone(),
        recovery_pack: Vec::new(),
        recovery_pack_sha256: String::new(),
        unbound_outcome_sha256: outcome.sha256.clone(),
        post_generation: unbound.base_generation.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?,
        post_revision,
    })
}

#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_bind_member<Mutation>(member: &mut DurableOwnedGroupMemberV1, outcome: &DurableStorePreparedOutcomeV1, decision_sha256: &str) -> Result<(), DurableOwnedGroupDecisionError>
where
    Mutation: ValueToValue + ValueFromValue,
{
    let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?;
    let mut edit: Edit<Mutation> = parse_canonical_json_value(&unbound.edit_without_group_canonical_json)?;
    if edit.mutation_meta.len() != 1 {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if meta.group_id.replace(decision_sha256.into()).is_some() {
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
        group_id: decision_sha256.into(),
        edit_canonical_json: semio_framework_pack_json::to_json_string(&edit).into_bytes(),
        post_snapshot_pack: unbound.post_snapshot_pack,
        edit_digest: CursorRevisionAccumulator::edit_digest(&edit),
        post_generation: member.post_generation,
        post_revision: member.post_revision,
    };
    member.recovery_pack = bound.encode_pack();
    member.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&member.recovery_pack);
    Ok(())
}
