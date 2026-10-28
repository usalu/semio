//! 📒️ Group decision, journal record and snapshot capture: the three member decision, the prepared, bound and recovery store records, the journal record with its operations and the owned three snapshot capture. See [`super`].

use super::*;
use super::map_assembly_recovery_completion::recover_decision_store_owned;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact,semio_framework_value_derive::RetireOwned)]
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
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ParentMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        DrawingMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ValueMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
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
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + Send + 'static,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + Send + 'static,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
        ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + Send + 'static,
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
    P: ValueToValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
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
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
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
    ParentP: ArtifactPack + ValueToValue + Send + Sync + Clone + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueFromValue + 'static,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<ParentP> + 'static,
    DrawingP: ArtifactPack + ValueToValue + Send + Sync + Clone + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueFromValue + 'static,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + 'static,
    ValueP: ArtifactPack + ValueToValue + Send + Sync + Clone + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueFromValue + 'static,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + Clone + semio_framework_os_spr::ColdMutation<ValueP> + 'static,
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

type Funding = semio_framework_value::retained_clone::RetainedCloneGrant;
type Receipt = semio_framework_value::RetainedCloneProgress;

fn funds(grant: Funding, copy: usize, capacity: usize) -> bool {
    grant.maximum_items > 0 && grant.maximum_depth > 0 && grant.maximum_copy_bytes >= copy && grant.maximum_capacity_bytes >= capacity
}

fn reference_bytes(reference: &semio_framework_artifact_reference::ArtifactRef) -> usize {
    reference.artifact_id.len() + reference.dialect.artifact_kind.len() + reference.dialect.standard.len() + reference.dialect.subset.len()
}

fn owner_bytes(owner: &Option<OwnerRef>) -> usize {
    owner.as_ref().map_or(0, |owner| reference_bytes(&owner.parent) + owner.slot.len() + owner.child_id.len())
}

fn unbound_text_bytes<P, M>(owner: &DurableStorePreparedOwnerV1<P, M>) -> Option<usize> {
    owner.unbound.as_ref().map(|unbound| unbound.recovery_schema.len() + unbound.sha256.len())
}

fn plan_bytes<P, M>(owner: &DurableStorePreparedOwnerV1<P, M>) -> usize {
    owner.plan.as_ref().map_or(0, |plan| plan.recovery_pack.len() + plan.recovery_pack_sha256.len())
}

fn visible(receipt: Receipt) -> Option<Receipt> {
    (receipt != Receipt::default()).then_some(receipt)
}

fn unbound_turn<P, M>(owner: &mut DurableStorePreparedOwnerV1<P, M>, schema: &str, grant: Funding, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(bool, Receipt), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource,
    M: ArtifactCanonicalJsonTree,
{
    let prepared = owner.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let (outcome, receipt) = owner.member_encoding.advance(prepared, schema, grant, native)?;
    let done = outcome.is_some();
    if outcome.is_some() { owner.unbound = outcome; }
    Ok((done, receipt))
}

fn bound_turn<P, M>(owner: &mut DurableStorePreparedOwnerV1<P, M>, store: &ArtifactStore<P, M>, group: &str, grant: Funding, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(bool, Receipt), DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    M: StoreMutation<P> + Clone + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + semio_framework_os_spr::ColdMutation<P>,
{
    let prepared = owner.prepared.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let wire = owner.member_encoding.wire.as_deref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let (plan, receipt) = owner.bound_encoding.advance(prepared, wire, &store.parts_ref(), group, grant, native)?;
    let done = plan.is_some();
    if plan.is_some() { owner.plan = plan; }
    Ok((done, receipt))
}

fn pack_hash_matches(plan: &DurableStoreBindingPlanV1) -> bool {
    let mut hasher = semio_framework_hash::Sha256::new();
    hasher.update(&plan.recovery_pack);
    let digest = hasher.finalize();
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let expected = plan.recovery_pack_sha256.as_bytes();
    expected.len() == 64 && digest.iter().enumerate().all(|(index, byte)| expected[2 * index] == HEX[usize::from(byte >> 4)] && expected[2 * index + 1] == HEX[usize::from(byte & 15)])
}

fn seal_member<P, M>(member: &mut DurableOwnedGroupMemberV1, owner: &DurableStorePreparedOwnerV1<P, M>) -> Result<(), DurableOwnedGroupDecisionError> {
    let plan = owner.plan.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let mut pack = Vec::new();
    pack.try_reserve_exact(plan.recovery_pack.len()).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    pack.extend_from_slice(&plan.recovery_pack);
    member.recovery_pack = pack;
    member.recovery_pack_sha256 = plan.recovery_pack_sha256.clone();
    member.post_generation = plan.post_generation;
    member.post_revision = plan.post_revision;
    Ok(())
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    /// 🪜️ Advances the original binding ladder by one funded turn: three unbound plans, the decision identity, three bound plans, the decision Pack, three one-owner transfers and the planner closures.
    /// `Ok(None)` means the grant cannot fund the next turn; every `Err` leaves each original owner exactly where its last successful turn left it.
    pub fn advance_binding(&mut self, parent: &ArtifactStore<ParentP, ParentMutation>, drawing: &ArtifactStore<DrawingP, DrawingMutation>, value: &ArtifactStore<ValueP, ValueMutation>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<Option<Receipt>, DurableOwnedGroupDecisionError> {
        let retained = grant.retained_grant();
        let error = DurableOwnedGroupDecisionError::Retirement;
        let mut accepted = |_| true;
        let mut native = semio_framework_value::NativeEncodeControl::new_retained(&mut accepted);
        if !self.issuers_captured {
            self.parent.capture_issuers(&parent.parts_ref())?;
            self.drawing.capture_issuers(&drawing.parts_ref())?;
            self.value.capture_issuers(&value.parts_ref())?;
            self.issuers_captured = true;
            return Ok(Some(Receipt::default()));
        }
        match self.binding_step {
            0 | 1 | 2 => {
                let (done, receipt) = match self.binding_step {
                    0 => unbound_turn(&mut self.parent, PARENT_RECOVERY_SCHEMA, retained, &mut native)?,
                    1 => unbound_turn(&mut self.drawing, DRAWING_RECOVERY_SCHEMA, retained, &mut native)?,
                    _ => unbound_turn(&mut self.value, VALUE_RECOVERY_SCHEMA, retained, &mut native)?,
                };
                self.binding_step += u8::from(done);
                Ok(visible(receipt))
            }
            3 => self.decision_turn(parent, drawing, value, retained, &mut native),
            4..=6 => {
                let group = self.decision.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.decision_sha256.as_str();
                let (done, receipt) = match self.binding_step {
                    4 => bound_turn(&mut self.parent, parent, group, retained, &mut native)?,
                    5 => bound_turn(&mut self.drawing, drawing, group, retained, &mut native)?,
                    _ => bound_turn(&mut self.value, value, group, retained, &mut native)?,
                };
                self.binding_step += u8::from(done);
                Ok(visible(receipt))
            }
            7 => self.decision_pack_turn(retained, &mut native),
            8..=10 => {
                let group = self.decision.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.decision_sha256.as_str();
                let receipt = match self.binding_step {
                    8 => self.parent.transfer(group, retained)?,
                    9 => self.drawing.transfer(group, retained)?,
                    _ => self.value.transfer(group, retained)?,
                };
                self.binding_step += u8::from(receipt.is_some());
                Ok(receipt.and_then(visible))
            }
            11 => {
                let receipt = if !self.parent.member_encoding.terminal_is_empty() { Some(self.parent.member_encoding.close(retained).map_err(error)?) }
                    else if !self.drawing.member_encoding.terminal_is_empty() { Some(self.drawing.member_encoding.close(retained).map_err(error)?) }
                    else if !self.value.member_encoding.terminal_is_empty() { Some(self.value.member_encoding.close(retained).map_err(error)?) }
                    else { self.binding_step = 12; return Ok(Some(Receipt::default())); };
                Ok(receipt.and_then(visible))
            }
            12 => {
                let receipt = if !self.parent.bound_encoding.terminal_is_empty() { Some(self.parent.bound_encoding.close(retained).map_err(error)?) }
                    else if !self.drawing.bound_encoding.terminal_is_empty() { Some(self.drawing.bound_encoding.close(retained).map_err(error)?) }
                    else if !self.value.bound_encoding.terminal_is_empty() { Some(self.value.bound_encoding.close(retained).map_err(error)?) }
                    else { self.binding_step = 13; return Ok(Some(Receipt::default())); };
                Ok(receipt.and_then(visible))
            }
            13 => {
                let receipt = if !self.decision_encoding.terminal_is_empty() { Some(self.decision_encoding.close(retained).map_err(error)?) }
                    else if !self.decision_identity.ready() { Some(self.decision_identity.close(retained).map_err(error)?) }
                    else { self.binding_step = 14; return Ok(Some(Receipt::default())); };
                Ok(receipt.and_then(visible))
            }
            _ => Err(DurableOwnedGroupDecisionError::InvalidOutcome),
        }
    }

    fn decision_turn(&mut self, parent: &ArtifactStore<ParentP, ParentMutation>, drawing: &ArtifactStore<DrawingP, DrawingMutation>, value: &ArtifactStore<ValueP, ValueMutation>, retained: Funding, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<Receipt>, DurableOwnedGroupDecisionError> {
        let error = DurableOwnedGroupDecisionError::Retirement;
        if self.decision.is_none() {
            let (parent_reference, parent_owner) = store_member_identity(&parent.parts_ref())?;
            let (drawing_reference, drawing_owner) = store_member_identity(&drawing.parts_ref())?;
            let (value_reference, value_owner) = store_member_identity(&value.parts_ref())?;
            if parent_reference == drawing_reference || parent_reference == value_reference || drawing_reference == value_reference {
                return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
            }
            let (Some(parent_text), Some(drawing_text), Some(value_text)) = (unbound_text_bytes(&self.parent), unbound_text_bytes(&self.drawing), unbound_text_bytes(&self.value)) else { return Err(DurableOwnedGroupDecisionError::InvalidOutcome) };
            let capacity = size_of::<DurableOwnedThreeMemberDecisionV1>()
                + DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.len() + DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1.len() + DURABLE_OWNED_GROUP_SHAPE_V1.len() + reference_bytes(&parent_reference)
                + PARENT_ROLE.len() + DRAWING_ROLE.len() + VALUE_ROLE.len() + parent_text + drawing_text + value_text
                + reference_bytes(&parent_reference) + reference_bytes(&drawing_reference) + reference_bytes(&value_reference) + owner_bytes(&parent_owner) + owner_bytes(&drawing_owner) + owner_bytes(&value_owner);
            if !funds(retained, capacity, capacity) { return Ok(None); }
            native.checkpoint().map_err(error)?;
            native.admit_turn_capacity(retained.maximum_capacity_bytes).map_err(error)?;
            let anchor = DurableOwnedGroupAnchorV1 { schema: native.copy_text(DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1).map_err(error)?, parent: parent_reference.clone(), shape: native.copy_text(DURABLE_OWNED_GROUP_SHAPE_V1).map_err(error)? };
            let members = [
                prepared_member(PARENT_ROLE, parent_reference, parent_owner, &self.parent, native)?,
                prepared_member(DRAWING_ROLE, drawing_reference, drawing_owner, &self.drawing, native)?,
                prepared_member(VALUE_ROLE, value_reference, value_owner, &self.value, native)?,
            ];
            let [parent_member, drawing_member, value_member] = members;
            if !prepared_base_matches_store(&parent.parts_ref(), &parent_member, self.parent.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?)
                || !prepared_base_matches_store(&drawing.parts_ref(), &drawing_member, self.drawing.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?)
                || !prepared_base_matches_store(&value.parts_ref(), &value_member, self.value.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?)
            {
                return Err(DurableOwnedGroupDecisionError::InvalidOwner);
            }
            self.decision = Some(Box::new(DurableOwnedThreeMemberDecisionV1 { schema: native.copy_text(DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1).map_err(error)?, anchor, anchor_sha256: String::new(), decision_sha256: String::new(), parent: parent_member, drawing: drawing_member, value: value_member }));
            return Ok(Some(Receipt { copied_items: 1, copied_bytes: capacity, retained_capacity_bytes: capacity, ..Default::default() }));
        }
        if !self.decision_identity.initialized() { self.decision_identity.initialize(); }
        let decision = self.decision.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let receipt = self.decision_identity.advance(decision, retained, native).map_err(error)?;
        if self.decision_identity.ready() { self.binding_step = 4; }
        Ok(visible(receipt))
    }

    fn decision_pack_turn(&mut self, retained: Funding, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<Receipt>, DurableOwnedGroupDecisionError> {
        let error = DurableOwnedGroupDecisionError::Retirement;
        match self.decision_plan_phase {
            0..=2 => {
                let plan = match self.decision_plan_phase {
                    0 => self.parent.plan.as_ref(),
                    1 => self.drawing.plan.as_ref(),
                    _ => self.value.plan.as_ref(),
                }.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if plan.recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES { return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge); }
                if !funds(retained, plan.recovery_pack.len(), 0) { return Ok(None); }
                native.checkpoint().map_err(error)?;
                if !valid_hash(&plan.recovery_pack_sha256) || !pack_hash_matches(plan) { return Err(DurableOwnedGroupDecisionError::InvalidHash); }
                let copied = plan.recovery_pack.len();
                self.decision_plan_phase += 1;
                Ok(visible(Receipt { copied_items: 1, copied_bytes: copied, ..Default::default() }))
            }
            3 => {
                let capacity = plan_bytes(&self.parent) + plan_bytes(&self.drawing) + plan_bytes(&self.value);
                if !funds(retained, capacity, capacity) { return Ok(None); }
                native.checkpoint().map_err(error)?;
                let decision = self.decision.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                seal_member(&mut decision.parent, &self.parent)?;
                seal_member(&mut decision.drawing, &self.drawing)?;
                seal_member(&mut decision.value, &self.value)?;
                self.decision_plan_phase = 4;
                Ok(Some(Receipt { copied_items: 1, copied_bytes: capacity, retained_capacity_bytes: capacity, ..Default::default() }))
            }
            4 => {
                let decision = self.decision.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let receipt = self.decision_encoding.advance(decision.as_ref(), <DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(), DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, retained, native)
                    .map_err(|reason| if reason.kind == semio_framework_value::ValueRefusalKind::OwnershipLimit && reason.message.contains("exact carrier limit") { DurableOwnedGroupDecisionError::EventTooLarge } else { error(reason) })?;
                if self.decision_encoding.ready() { self.decision_plan_phase = 5; }
                Ok(visible(receipt))
            }
            _ => {
                let Some((bytes, receipt)) = self.decision_encoding.take(retained) else { return Ok(None) };
                self.decision_pack = Some(bytes);
                self.decision_plan_phase = 0;
                self.binding_step = 8;
                Ok(Some(receipt))
            }
        }
    }

    /// 📦️ Hands the completely planned and transferred candidates to the commit coordinator once every planner is terminal.
    pub fn take_bound(&mut self) -> Option<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.binding_step != 14 || self.parent.bound.is_none() || self.drawing.bound.is_none() || self.value.bound.is_none() || self.decision.is_none() || self.decision_pack.is_none() { return None; }
        Some(DurableOwnedThreeStoreBoundV1 { decision: self.decision.take()?, parent: self.parent.bound.take()?, drawing: self.drawing.bound.take()?, value: self.value.bound.take()?, decision_pack: self.decision_pack.take() })
    }

    /// 🧪️ Drives the whole ladder under an unbounded grant for fixtures that bind without caring about turn boundaries.
    #[cfg(any(test, feature = "durable-group-testing"))]
    pub fn bind_store_owned(mut self, parent: &ArtifactStore<ParentP, ParentMutation>, drawing: &ArtifactStore<DrawingP, DrawingMutation>, value: &ArtifactStore<ValueP, ValueMutation>) -> Result<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
        let grant = semio_framework_os_store_one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX / 2, maximum_capacity_bytes: usize::MAX / 2, maximum_release_bytes: usize::MAX / 2, maximum_depth: 64 };
        while self.binding_step < 14 {
            if self.advance_binding(parent, drawing, value, grant)?.is_none() { return Err(DurableOwnedGroupDecisionError::InvalidOutcome); }
        }
        self.take_bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)
    }
}
