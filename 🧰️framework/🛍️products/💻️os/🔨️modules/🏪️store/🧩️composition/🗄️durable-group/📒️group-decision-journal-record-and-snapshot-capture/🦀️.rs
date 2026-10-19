//! 📒️ Group decision, journal record and snapshot capture: the three member decision, the prepared, bound and recovery store records, the journal record with its operations and the owned three snapshot capture. See [`super`].

use super::*;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Eq, ToValue, FromValue, crate::os_dsl::DslArtifact,semio_framework_value_derive::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.durable-owned-three-member-decision")]
pub struct DurableOwnedThreeMemberDecisionV1 {
    pub schema: String,
    pub anchor: DurableOwnedGroupAnchorV1,
    pub anchor_sha256: String,
    pub decision_sha256: String,
    pub parent: DurableOwnedGroupMemberV1,
    pub drawing: DurableOwnedGroupMemberV1,
    pub value: DurableOwnedGroupMemberV1,
}

pub struct DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub parent: DurableStorePreparedOwnerV1<ParentP, ParentMutation>,
    pub drawing: DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>,
    pub value: DurableStorePreparedOwnerV1<ValueP, ValueMutation>,
    decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,
    decision_pack: Option<Vec<u8>>,
    binding_step: u8,
    decision_plan_phase: u8,
    decision_packet_hash: Option<semio_framework_hash::Sha256>,
    decision_identity:planning::DurableDecisionIdentityV1,
    issuers_captured:bool,
    decision_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>,
    decision_pack_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>,
    retirement_progress:semio_framework_value::RetainedCloneProgress,
    #[cfg(feature = "deflate")]
    decision_encoding: planning::DurablePackOutputV1,
}

pub struct DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub decision: Box<DurableOwnedThreeMemberDecisionV1>,
    pub parent: DurableStoreBoundOutcomeV1<ParentP, ParentMutation>,
    pub drawing: DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>,
    pub value: DurableStoreBoundOutcomeV1<ValueP, ValueMutation>,
    decision_pack: Option<Vec<u8>>,
}

#[expect(clippy::large_enum_variant, reason = "Recovery hands back all three exact preadmitted store outcomes without allocating at the ownership transition.")]
pub enum DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    Apply(DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    AlreadyApplied,
}

/// 🔐️ Store-admitted decision event whose private bytes and identities can only come from
/// one fully verified canonical fixed-three decision Pack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupJournalRecordV1 {
    canonical_pack: Vec<u8>,
    pub decision_sha256: String,
    pub anchor_sha256: String,
    pub document: semio_framework_artifact_reference::ArtifactRef,
    pub parent_edit_id: String,
    pub parent_post_revision: [u8; 32],
}

impl DurableOwnedGroupJournalRecordV1 {
    /// 🔏️ Admits one canonical decision Pack and derives its decision identity exclusively from
    /// the Store-verified fixed-three envelope.
    pub fn admit_canonical(canonical_pack: Vec<u8>) -> Result<Self, DurableOwnedGroupDecisionError> {
        let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&canonical_pack)?;
        let parent = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&decision.parent.recovery_pack)?;
        let edit: DslValue = parse_canonical_json_value(&parent.edit_canonical_json)?;
        let parent_edit_id = edit.get("id").and_then(DslValue::as_str).filter(|identity| !identity.is_empty() && identity.len() <= DURABLE_OWNED_GROUP_ID_MAX_BYTES).ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?.to_string();
        if decision.parent.post_revision == [0; 32] {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(Self { canonical_pack, decision_sha256: decision.decision_sha256, anchor_sha256: decision.anchor_sha256, document: decision.anchor.parent, parent_edit_id, parent_post_revision: decision.parent.post_revision })
    }

    pub fn admit(canonical_pack: Vec<u8>, claimed_decision_sha256: &str) -> Result<Self, DurableOwnedGroupDecisionError> {
        let record = Self::admit_canonical(canonical_pack)?;
        if record.decision_sha256 != claimed_decision_sha256 {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        Ok(record)
    }

    pub fn canonical_pack(&self) -> &[u8] {
        &self.canonical_pack
    }

    pub fn into_canonical_pack(self) -> Vec<u8> {
        self.canonical_pack
    }

    pub fn decision_sha256(&self) -> &str {
        &self.decision_sha256
    }

    pub fn anchor_sha256(&self) -> &str {
        &self.anchor_sha256
    }

    pub fn document(&self) -> &semio_framework_artifact_reference::ArtifactRef {
        &self.document
    }

    /// 📍️ Projects the parent edit identity only from the fully admitted bound outcome.
    pub fn parent_edit_id(&self) -> &str {
        &self.parent_edit_id
    }

    /// 🧬️ Projects the exact parent Store revision after the fixed-three visibility flip.
    pub fn parent_post_revision(&self) -> [u8; 32] {
        self.parent_post_revision
    }

    pub fn into_parts(self) -> (Vec<u8>, String, String, semio_framework_artifact_reference::ArtifactRef) {
        (self.canonical_pack, self.decision_sha256, self.anchor_sha256, self.document)
    }

    /// 🧬️ Re-admits the complete decision and verifies all three typed bound outcomes before
    /// exposing immutable edit projections; no publication seal or binding authority escapes.
    pub fn verify_fixed_three_edits<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(&self) -> Result<DurableOwnedGroupVerifiedThreeEditsV1<ParentMutation, DrawingMutation, ValueMutation>, DurableOwnedGroupDecisionError>
    where
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        ParentMutation: ValueToValue + ValueFromValue,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        DrawingMutation: ValueToValue + ValueFromValue,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        ValueMutation: ValueToValue + ValueFromValue,
    {
        let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&self.canonical_pack)?;
        if decision.decision_sha256 != self.decision_sha256 || decision.anchor_sha256 != self.anchor_sha256 || decision.anchor.parent != self.document {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        Ok(DurableOwnedGroupVerifiedThreeEditsV1 {
            parent: verify_bound_member_edit::<ParentP, ParentMutation>(&decision.parent, &decision.decision_sha256)?,
            drawing: verify_bound_member_edit::<DrawingP, DrawingMutation>(&decision.drawing, &decision.decision_sha256)?,
            value: verify_bound_member_edit::<ValueP, ValueMutation>(&decision.value, &decision.decision_sha256)?,
            decision_sha256: decision.decision_sha256,
            anchor_sha256: decision.anchor_sha256,
            document: decision.anchor.parent,
        })
    }

    /// ♻️ Reconstructs only Store-private sealed owners from one committed decision and the exact three retained live Stores.
    pub(super) fn begin_store_owned_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        receipt: DurableOwnedGroupJournalReceiptV1,
        parent: ArtifactStore<ParentP, ParentMutation>,
        drawing: ArtifactStore<DrawingP, DrawingMutation>,
        value: ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedMapRecoveryStartV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedMapRecoveryRejectedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>
    where
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
        ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
        DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
        ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    {
        let rejected = |error, parent, drawing, value| DurableOwnedMapRecoveryRejectedV1 { error, parent, drawing, value };
        let decision = match DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&self.canonical_pack) {
            Ok(decision) if decision.decision_sha256 == self.decision_sha256 && decision.anchor_sha256 == self.anchor_sha256 && decision.anchor.parent == self.document => decision,
            Ok(_) => return Err(rejected(DurableOwnedGroupDecisionError::InvalidHash, parent, drawing, value)),
            Err(error) => return Err(rejected(error, parent, drawing, value)),
        };
        if receipt.decision_sha256 != self.decision_sha256 || receipt.anchor_sha256 != self.anchor_sha256 || receipt.transaction_id == 0 {
            return Err(rejected(DurableOwnedGroupDecisionError::InvalidHash, parent, drawing, value));
        }
        match decision.recover_store_owned(&parent, &drawing, &value) {
            Ok(DurableOwnedThreeStoreRecoveryV1::AlreadyApplied) => Ok(DurableOwnedMapRecoveryStartV1::AlreadyApplied(DurableOwnedMapRecoveryOwnersV1 { parent, drawing, value })),
            Ok(DurableOwnedThreeStoreRecoveryV1::Apply(bound)) => match bound.begin_retained_recovery(receipt) {
                Ok(coordinator) => Ok(DurableOwnedMapRecoveryStartV1::Apply(DurableOwnedMapRecoveryHostV1 {
                    operation: std::mem::ManuallyDrop::new(Some(DurableOwnedMapRecoveryOperationV1 {
                        parent: std::mem::ManuallyDrop::new(Some(parent)),
                        drawing: std::mem::ManuallyDrop::new(Some(drawing)),
                        value: std::mem::ManuallyDrop::new(Some(value)),
                        coordinator: std::mem::ManuallyDrop::new(Some(coordinator)),
                    })),
                })),
                Err(error) => Err(rejected(error, parent, drawing, value)),
            },
            Err(error) => Err(rejected(error, parent, drawing, value)),
        }
    }
}

fn captured_store_snapshot<P, Mutation>(store: &ArtifactStore<P, Mutation>, visibility: Option<&Arc<semio_framework_os_vcs::ArtifactGroupVisibility>>, committed: bool) -> Result<(Arc<P>, u64, [u8; 32]), DurableOwnedGroupDecisionError>
where
    P: ValueToValue,
    Mutation: StoreMutation<P> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    match store.wire.durable_group_root.as_ref() {
        Some(root) => {
            let expected = visibility.ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
            if !Arc::ptr_eq(&root.visibility, expected) {
                return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
            }
            if committed {
                if root.adopted {
                    validate_adopted_store_member(&store.parts_ref(), expected)?;
                } else {
                    return Ok((Arc::clone(root.current.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?), root.generation, root.content_revision));
                }
            }
        }
        None if committed && visibility.is_some() => return Err(DurableOwnedGroupDecisionError::InvalidFrontier),
        None => {}
    }
    Ok((Arc::clone(&*store.live.current), store.cursor.generation, store.cursor.content_revision))
}

pub fn capture_store_owned_three_snapshot<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
    parent_store: &ArtifactStore<ParentP, ParentMutation>,
    drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
    value_store: &ArtifactStore<ValueP, ValueMutation>,
) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError>
where
    ParentP: ValueToValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    let roots = [parent_store.wire.durable_group_root.as_ref().map(|root| &root.visibility), drawing_store.wire.durable_group_root.as_ref().map(|root| &root.visibility), value_store.wire.durable_group_root.as_ref().map(|root| &root.visibility)];
    let visibility = roots.iter().flatten().next().copied();
    if visibility.is_some_and(|visibility| roots.iter().flatten().any(|candidate| !Arc::ptr_eq(visibility, candidate))) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let committed = visibility.is_some_and(|visibility| visibility.capture().committed_for(visibility) == Ok(true));
    if committed && roots.iter().any(Option::is_none) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let parent = captured_store_snapshot(parent_store, visibility, committed)?;
    let drawing = captured_store_snapshot(drawing_store, visibility, committed)?;
    let value = captured_store_snapshot(value_store, visibility, committed)?;
    Ok(DurableOwnedThreeStoreSnapshotV1 { parent: parent.0, drawing: drawing.0, value: value.0, generations: [parent.1, drawing.1, value.1], revisions: [parent.2, drawing.2, value.2] })
}
