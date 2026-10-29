//! 📒️ Group decision, journal record and snapshot capture: the three member decision, the prepared, bound and recovery store records, the journal record with its operations and the owned three snapshot capture. See [`super`].

use super::*;
use super::map_assembly_recovery_completion::recover_decision_store_owned;

pub struct DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub parent: DurableStorePreparedOwnerV1<ParentP, ParentMutation>,
    pub drawing: DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>,
    pub value: DurableStorePreparedOwnerV1<ValueP, ValueMutation>,
    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,
    pub(super) decision_pack: Option<Vec<u8>>,
    pub(super) binding_step: u8,
    pub(super) decision_plan_phase: u8,
    pub(super) decision_packet_hash: Option<semio_framework_hash::Sha256>,
    pub(super) decision_identity:planning::DurableDecisionIdentityV1,
    pub(super) issuers_captured:bool,
    pub(super) decision_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>,
    pub(super) decision_pack_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>,
    pub(super) retirement_progress:semio_framework_value::RetainedCloneProgress,
    #[cfg(feature = "deflate")]
    pub(super) decision_encoding: planning::DurablePackOutputV1,
}

pub struct DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub decision: Box<DurableOwnedThreeMemberDecisionV1>,
    pub parent: DurableStoreBoundOutcomeV1<ParentP, ParentMutation>,
    pub drawing: DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>,
    pub value: DurableStoreBoundOutcomeV1<ValueP, ValueMutation>,
    pub(super) decision_pack: Option<Vec<u8>>,
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
    pub(super) canonical_pack: Vec<u8>,
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
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        ParentMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP> + StoreMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        DrawingMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + StoreMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        ValueMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP> + StoreMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
        ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
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
        match recover_decision_store_owned(&decision, &parent, &drawing, &value) {
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
    P: semio_framework_os_spr::StoreState,
    Mutation: Clone + StoreMutation<P> + ValueToValue + ValueFromValue + semio_framework_os_spr::ColdMutation<P> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub fn originals_empty(&self) -> bool { self.parent.originals_empty() && self.drawing.originals_empty() && self.value.originals_empty() }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
{
    pub fn retire_originals_step(&mut self, parent: &mut ArtifactStore<ParentP, ParentMutation>, drawing: &mut ArtifactStore<DrawingP, DrawingMutation>, value: &mut ArtifactStore<ValueP, ValueMutation>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
        self.retirement_progress=Default::default();
        if !self.decision_identity.ready(){self.retirement_progress=self.decision_identity.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false);}
        if !self.decision_encoding.terminal_is_empty() { self.retirement_progress=self.decision_encoding.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false); }
        if !self.parent.originals_empty() { retire_binding_member(&mut self.parent, &mut parent.parts(), grant)?;self.retirement_progress=self.parent.retirement_progress(); return Ok(false); }
        if !self.drawing.originals_empty() { retire_binding_member(&mut self.drawing, &mut drawing.parts(), grant)?;self.retirement_progress=self.drawing.retirement_progress(); return Ok(false); }
        if !self.value.originals_empty() { retire_binding_member(&mut self.value, &mut value.parts(), grant)?;self.retirement_progress=self.value.retirement_progress(); return Ok(false); }
        if let Some(owner)=self.decision_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_retirement=None;}return Ok(false);}
        if let Some(owner)=self.decision_pack_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_pack_retirement=None;}return Ok(false);}
        let copy=size_of::<Box<DurableOwnedThreeMemberDecisionV1>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>()+size_of::<DurablePlanningOutputV1>();
        if self.decision.is_some()||self.decision_pack.is_some(){if !grant.permits_one()||grant.maximum_copy_bytes<copy{return Ok(false);}if let Some(decision)=self.decision.take(){self.decision_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(decision).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}else if let Some(pack)=self.decision_pack.take(){self.decision_pack_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(pack)).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}self.retirement_progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false);}
        Ok(true)
    }
}
