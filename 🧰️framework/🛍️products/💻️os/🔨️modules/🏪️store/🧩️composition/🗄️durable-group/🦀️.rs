//! 🗺️ Closed codec and admission fence for one durable parent+drawing+value Store decision.
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparedParts;

use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_dsl_record::DslField;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_dsl_record::Shape;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
use std::collections::BTreeMap;
pub use semio_framework_os_store_durable_group_wire::lifted_durable_group::{ArtifactStoreDurableGroupRootV1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_ID_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DurableOwnedGroupAnchorV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedGroupVerifiedThreeEditsV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableOwnedThreeStoreSnapshotV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableStoreVerifiedOutcomeV1, StagedRootRetirement};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DRAWING_RECOVERY_SCHEMA, DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableMemberRecoveryFrontier, DurableOwnedThreeMemberUnsignedV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_RECOVERY_SCHEMA, VALUE_ROLE, close_assembly_publication, close_journal_owner, durable_group_test_bind_member, durable_group_test_member, durable_group_test_outcome, member_identity_matches, member_matches, parse_canonical_json_value, take_assembly_prepared, unsigned_member, valid_hash, valid_reference};
use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStorePreparedOwnerV1, bound_member, clear_adopted_store_member, prepared_base_matches_store, prepared_member, recovery_frontier, retire_unstaged_store_member, stage_store_member, store_member_identity, validate_adopted_store_member, verify_bound_member_edit};
pub use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStoreBoundOutcomeV1, abort_staged_store_member, adopt_staged_store_member, stage_prebuilt_batch_root};


























macro_rules! value_field {
    ($kind:ty) => {
        impl DslField for $kind {
            fn shape() -> Shape {
                Shape::Value
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::Value(semio_framework_value::ToValue::to_value(self))
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::Value(value) => semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| error.to_string()),
                    other => Err(format!("expected Value, found {other:?}")),
                }
            }
        }
    };
}

value_field!(DurableOwnedGroupAnchorV1);
value_field!(DurableOwnedGroupMemberV1);








#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.durable-owned-three-member-decision")]
pub struct DurableOwnedThreeMemberDecisionV1 {
    pub(crate) schema: String,
    pub(crate) anchor: DurableOwnedGroupAnchorV1,
    pub(crate) anchor_sha256: String,
    pub(crate) decision_sha256: String,
    pub(crate) parent: DurableOwnedGroupMemberV1,
    pub(crate) drawing: DurableOwnedGroupMemberV1,
    pub(crate) value: DurableOwnedGroupMemberV1,
}



















pub struct DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub parent: DurableStorePreparedOwnerV1<ParentP, ParentMutation>,
    pub drawing: DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>,
    pub value: DurableStorePreparedOwnerV1<ValueP, ValueMutation>,
}

pub struct DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub decision: DurableOwnedThreeMemberDecisionV1,
    pub parent: DurableStoreBoundOutcomeV1<ParentP, ParentMutation>,
    pub drawing: DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>,
    pub value: DurableStoreBoundOutcomeV1<ValueP, ValueMutation>,
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
    decision_sha256: String,
    anchor_sha256: String,
    document: semio_framework_artifact_reference::ArtifactRef,
    parent_edit_id: String,
    parent_post_revision: [u8; 32],
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
    fn begin_store_owned_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
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
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue,
{
    match store.wire.durable_group_root.as_ref() {
        Some(root) => {
            let expected = visibility.ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
            if !Arc::ptr_eq(&root.visibility, expected) {
                return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
            }
            if committed {
                if root.adopted {
                    validate_adopted_store_member(store, expected)?;
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
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
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

/// 🧷️ Store-owned fixed-three coordinator retaining all candidates through journal resolution.
pub(super) struct DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub parent: Option<DurableStoreBoundOutcomeV1<ParentP, ParentMutation>>,
    pub drawing: Option<DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>>,
    pub value: Option<DurableStoreBoundOutcomeV1<ValueP, ValueMutation>>,
    pub decision: Option<DurableOwnedThreeMemberDecisionV1>,
    pub decision_pack: Option<Vec<u8>>,
    pub decision_sha256: String,
    visibility_owner: Option<semio_framework_os_vcs::ArtifactGroupVisibilityOwner>,
    pub visibility: Option<Arc<semio_framework_os_vcs::ArtifactGroupVisibility>>,
    journal: Option<Box<dyn DurableOwnedGroupJournalCommitV1>>,
    pub receipt: Option<DurableOwnedGroupJournalReceiptV1>,
    pub phase: DurableOwnedThreeStoreCommitPhaseV1,
    recovery_committed: bool,
    pub cancel_requested: bool,
    cancel_forwarded: bool,
    journal_close_started: bool,
}

/// 🗺️ Retained Map publication owner. The three Stores, coordinator, and exclusive journal sink
/// cross the request boundary together and can leave it only through one terminal handoff.
struct DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    coordinator: std::mem::ManuallyDrop<Option<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub sink: std::mem::ManuallyDrop<Option<Box<dyn DurableOwnedGroupJournalSinkV1>>>,
}

/// 🏠️ Fixed host slot for one retained Map publication. The mounted operation is never exposed
/// to a request owner and cannot be replaced before its exact terminal handoff.
pub struct DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub operation: std::mem::ManuallyDrop<Option<DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🧳 Exact live owners returned after one retained Map publication reaches terminal emptiness.
pub struct DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
    pub sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
}

/// 🛡️ One-shot exact-three-Store authority consumed together with a DB-owned committed witness.
#[must_use = "the exact three Store owners must transfer into recovery or remain retained"]
pub struct DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
}

/// ♻️ Exact three Store owners returned by committed recovery after terminal publication or an idempotent post-frontier match.
pub struct DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
}

/// 🚧 Committed recovery rejection preserves every Store owner for corrected retry or close.
pub struct DurableOwnedMapRecoveryRejectedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub error: DurableOwnedGroupDecisionError,
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
}

struct DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    coordinator: std::mem::ManuallyDrop<Option<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🧭 Retained committed-recovery host; no journal sink exists because its opaque DB witness already proves the transaction committed.
pub struct DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub operation: std::mem::ManuallyDrop<Option<DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🔀 Either the exact committed decision still needs Store publication or all three Stores already match its post-frontier.
#[expect(clippy::large_enum_variant, reason = "Recovery transfers the exact admitted operation or original store owners without allocating during handoff.")]
pub enum DurableOwnedMapRecoveryStartV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    Apply(DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    AlreadyApplied(DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
}



impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn new(parent: ArtifactStore<ParentP, ParentMutation>, drawing: ArtifactStore<DrawingP, DrawingMutation>, value: ArtifactStore<ValueP, ValueMutation>) -> Self {
        Self { parent: std::mem::ManuallyDrop::new(Some(parent)), drawing: std::mem::ManuallyDrop::new(Some(drawing)), value: std::mem::ManuallyDrop::new(Some(value)) }
    }

    /// ↩️ Returns an admission that has not entered committed recovery to its exact three Store owners.
    pub fn into_owners(mut self) -> DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
        DurableOwnedMapRecoveryOwnersV1 {
            parent: self.parent.take().expect("unconsumed recovery admission retains its parent Store"),
            drawing: self.drawing.take().expect("unconsumed recovery admission retains its drawing Store"),
            value: self.value.take().expect("unconsumed recovery admission retains its value Store"),
        }
    }

    /// ♻️ Performs only lower Store restoration. The supplied record and receipt do not become a DB witness or approval acknowledgement here.
    pub fn restore_untrusted_committed_record(
        mut self,
        record: &DurableOwnedGroupJournalRecordV1,
        receipt: DurableOwnedGroupJournalReceiptV1,
    ) -> Result<DurableOwnedMapRecoveryStartV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedMapRecoveryRejectedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let parent = self.parent.take().expect("one-shot recovery admission retains its parent Store");
        let drawing = self.drawing.take().expect("one-shot recovery admission retains its drawing Store");
        let value = self.value.take().expect("one-shot recovery admission retains its value Store");
        record.begin_store_owned_recovery(receipt, parent, drawing, value)
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none()), "committed recovery admission reached Drop before exact DB witness transfer");
    }
}











/// 🧳 Exact owners returned after a pre-journal rejection or cancellation reaches terminal emptiness.
pub struct DurableOwnedThreeStoreMapAssemblyOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
    pub sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    pub parent_mutation: Option<ParentMutation>,
    pub drawing_mutation: Option<DrawingMutation>,
    pub value_mutation: Option<ValueMutation>,
    pub failure: DurableOwnedThreeStoreMapAssemblyFailureV1,
}

/// 🏗️ Store-owned fixed-three admission and preparation owner. It never publishes an individual member.
pub struct DurableOwnedThreeStoreMapAssemblyV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    pub parent_admission: Option<DurableOwnedMapMemberAdmissionV1<ParentMutation>>,
    pub drawing_admission: Option<DurableOwnedMapMemberAdmissionV1<DrawingMutation>>,
    pub value_admission: Option<DurableOwnedMapMemberAdmissionV1<ValueMutation>>,
    pub parent_factory: Option<Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<ParentP, ParentMutation>>>,
    pub drawing_factory: Option<Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<DrawingP, DrawingMutation>>>,
    pub value_factory: Option<Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<ValueP, ValueMutation>>>,
    parent_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<ParentP, ParentMutation>>>,
    drawing_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<DrawingP, DrawingMutation>>>,
    value_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<ValueP, ValueMutation>>>,
    pub prepared: std::mem::ManuallyDrop<Option<DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub host: std::mem::ManuallyDrop<Option<DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub sink: std::mem::ManuallyDrop<Option<Box<dyn DurableOwnedGroupJournalSinkV1>>>,
    pub phase: DurableOwnedThreeStoreMapAssemblyPhaseV1,
    pub failure: Option<DurableOwnedThreeStoreMapAssemblyFailureV1>,
}





impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreMapAssemblyV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent: ArtifactStore<ParentP, ParentMutation>,
        drawing: ArtifactStore<DrawingP, DrawingMutation>,
        value: ArtifactStore<ValueP, ValueMutation>,
        parent_admission: DurableOwnedMapMemberAdmissionV1<ParentMutation>,
        drawing_admission: DurableOwnedMapMemberAdmissionV1<DrawingMutation>,
        value_admission: DurableOwnedMapMemberAdmissionV1<ValueMutation>,
        parent_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<ParentP, ParentMutation>>,
        drawing_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<DrawingP, DrawingMutation>>,
        value_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<ValueP, ValueMutation>>,
        sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    ) -> Self {
        Self {
            parent: std::mem::ManuallyDrop::new(Some(parent)),
            drawing: std::mem::ManuallyDrop::new(Some(drawing)),
            value: std::mem::ManuallyDrop::new(Some(value)),
            parent_admission: Some(parent_admission),
            drawing_admission: Some(drawing_admission),
            value_admission: Some(value_admission),
            parent_factory: Some(parent_factory),
            drawing_factory: Some(drawing_factory),
            value_factory: Some(value_factory),
            parent_publication: std::mem::ManuallyDrop::new(None),
            drawing_publication: std::mem::ManuallyDrop::new(None),
            value_publication: std::mem::ManuallyDrop::new(None),
            prepared: std::mem::ManuallyDrop::new(None),
            host: std::mem::ManuallyDrop::new(None),
            sink: std::mem::ManuallyDrop::new(Some(sink)),
            phase: DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingParent,
            failure: None,
        }
    }

    pub fn phase(&self) -> DurableOwnedThreeStoreMapAssemblyPhaseV1 {
        self.phase
    }

    /// 🛑️ Records the first failure and enters the three-publication close sequence. A repeated
    /// failure — every `cancel()` a maintenance driver issues before its bounded turn — must never
    /// rewind that sequence, or the close cursor restarts at the value publication on every turn and
    /// the parent publication is never reached.
    pub fn fail(&mut self, failure: DurableOwnedThreeStoreMapAssemblyFailureV1) {
        if self.failure.is_none() {
            self.failure = Some(failure);
        }
        self.parent_factory = None;
        self.drawing_factory = None;
        self.value_factory = None;
        if let Some(publication) = self.parent_publication.as_mut() {
            publication.begin_close();
        }
        if let Some(publication) = self.drawing_publication.as_mut() {
            publication.begin_close();
        }
        if let Some(publication) = self.value_publication.as_mut() {
            publication.begin_close();
        }
        if !matches!(self.phase, DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingValue | DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingDrawing | DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingParent) {
            self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingValue;
        }
    }

    pub fn cancel(&mut self) -> bool {
        match self.phase {
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted => self.host.as_mut().is_some_and(DurableOwnedMapCommitHostV1::cancel),
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal | DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty => false,
            _ => {
                self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Cancelled);
                true
            }
        }
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedGroupDecisionError> {
        if !grant.permits_one() {
            return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked);
        }
        match self.phase {
            DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingParent => {
                let admission = self.parent_admission.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let mutation = admission.mutation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let store = self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let factory = self.parent_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.begin_apply_batch(admission.operation, admission.expected_generation, admission.expected_revision, admission.actor.clone(), vec![mutation], semio_framework_os_store_space_schema::HistoryLane::Document, Some(factory), None) {
                    Ok(publication) => {
                        *self.parent_publication = Some(publication);
                        self.parent_admission = None;
                        self.parent_factory = None;
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingDrawing;
                    }
                    Err(rejected) => {
                        let (reason, mut mutations) = rejected.into_owners();
                        admission.mutation = mutations.pop();
                        self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Admission { role: PARENT_ROLE, reason });
                    }
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingDrawing => {
                let admission = self.drawing_admission.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let mutation = admission.mutation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let store = self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let factory = self.drawing_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.begin_apply_batch(admission.operation, admission.expected_generation, admission.expected_revision, admission.actor.clone(), vec![mutation], semio_framework_os_store_space_schema::HistoryLane::Document, Some(factory), None) {
                    Ok(publication) => {
                        *self.drawing_publication = Some(publication);
                        self.drawing_admission = None;
                        self.drawing_factory = None;
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingValue;
                    }
                    Err(rejected) => {
                        let (reason, mut mutations) = rejected.into_owners();
                        admission.mutation = mutations.pop();
                        self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Admission { role: DRAWING_ROLE, reason });
                    }
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::AdmittingValue => {
                let admission = self.value_admission.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let mutation = admission.mutation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let store = self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let factory = self.value_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.begin_apply_batch(admission.operation, admission.expected_generation, admission.expected_revision, admission.actor.clone(), vec![mutation], semio_framework_os_store_space_schema::HistoryLane::Document, Some(factory), None) {
                    Ok(publication) => {
                        *self.value_publication = Some(publication);
                        self.value_admission = None;
                        self.value_factory = None;
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingParent;
                    }
                    Err(rejected) => {
                        let (reason, mut mutations) = rejected.into_owners();
                        admission.mutation = mutations.pop();
                        self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Admission { role: VALUE_ROLE, reason });
                    }
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingParent => {
                let store = self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let publication = self.parent_publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.advance_apply_batch(publication, grant) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked) => return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(_) if publication.phase() == semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing => self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing,
                    Ok(_) => {}
                    Err(error) => self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Preparation { role: PARENT_ROLE, reason: error.to_string() }),
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing => {
                let store = self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let publication = self.drawing_publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.advance_apply_batch(publication, grant) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked) => return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(_) if publication.phase() == semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing => self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingValue,
                    Ok(_) => {}
                    Err(error) => self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Preparation { role: DRAWING_ROLE, reason: error.to_string() }),
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingValue => {
                let store = self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let publication = self.value_publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.advance_apply_batch(publication, grant) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked) => return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(_) if publication.phase() == semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing => {
                        let parent = take_assembly_prepared(&mut self.parent_publication)?;
                        let drawing = take_assembly_prepared(&mut self.drawing_publication)?;
                        let value = take_assembly_prepared(&mut self.value_publication)?;
                        match DurableOwnedThreeStorePreparedV1::from_store_prepared(parent, drawing, value) {
                            Ok(prepared) => *self.prepared = Some(prepared),
                            Err(error) => self.failure = Some(DurableOwnedThreeStoreMapAssemblyFailureV1::Binding(error)),
                        }
                        if let Some(publication) = self.parent_publication.as_mut() {
                            publication.begin_close();
                        }
                        if let Some(publication) = self.drawing_publication.as_mut() {
                            publication.begin_close();
                        }
                        if let Some(publication) = self.value_publication.as_mut() {
                            publication.begin_close();
                        }
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingValue;
                    }
                    Ok(_) => {}
                    Err(error) => self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Preparation { role: VALUE_ROLE, reason: error.to_string() }),
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingValue => {
                if close_assembly_publication(&mut self.value_publication, grant)? {
                    self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingDrawing;
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingDrawing => {
                if close_assembly_publication(&mut self.drawing_publication, grant)? {
                    self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingParent;
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::ClosingParent => {
                if close_assembly_publication(&mut self.parent_publication, grant)? {
                    if self.failure.is_some() {
                        drop(self.prepared.take());
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal;
                    } else {
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Binding;
                    }
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Binding => {
                let prepared = self.prepared.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let bound = prepared.bind_store_owned(
                    self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                    self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                    self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                );
                match bound.and_then(DurableOwnedThreeStoreBoundV1::begin_retained_commit) {
                    Ok(coordinator) => {
                        let host = coordinator.mount_map(
                            self.parent.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.drawing.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.value.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.sink.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                        );
                        *self.host = Some(host);
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted;
                    }
                    Err(error) => {
                        self.failure = Some(DurableOwnedThreeStoreMapAssemblyFailureV1::Binding(error));
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal;
                    }
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted => return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted),
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal => return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal),
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty => return Err(DurableOwnedGroupDecisionError::InvalidOutcome),
        }
        Ok(match self.phase {
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted => DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted,
            DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal => DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal,
            phase => DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(phase),
        })
    }

    /// 🔬️ What each of the three publications still owns while the assembly sits in a `Closing*`
    /// phase. `close_assembly_publication` answers only `true`/`false`, so a cancelled assembly that
    /// never reaches `Terminal` reports the same `Progress(<phase>)` every turn with no way to say
    /// WHICH owner refuses to retire. This names it.
    pub fn closing_witness(&self) -> String {
        fn owner<P, Mutation>(publication: &Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mutation>>) -> String {
            publication.as_ref().map_or_else(
                || "retired".to_string(),
                |publication| {
                    format!("{:?}/terminal_is_empty={}/admitted={}/staged={}/closing_owner={}", publication.phase(), publication.terminal_is_empty(), publication.admitted_items(), publication.staged_items(), publication.closing_owner_witness())
                },
            )
        }
        format!(
            "phase={:?} failure={} prepared={} host={} parent={} drawing={} value={}",
            self.phase,
            self.failure.is_some(),
            self.prepared.is_some(),
            self.host.is_some(),
            owner(&self.parent_publication),
            owner(&self.drawing_publication),
            owner(&self.value_publication),
        )
    }

    pub fn take_mounted_host(&mut self) -> Option<DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.phase != DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted {
            return None;
        }
        let host = self.host.take()?;
        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty;
        Some(host)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedThreeStoreMapAssemblyOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.phase != DurableOwnedThreeStoreMapAssemblyPhaseV1::Terminal {
            return None;
        }
        let owners = DurableOwnedThreeStoreMapAssemblyOwnersV1 {
            parent: self.parent.take()?,
            drawing: self.drawing.take()?,
            value: self.value.take()?,
            sink: self.sink.take()?,
            parent_mutation: self.parent_admission.take().and_then(|mut admission| admission.mutation.take()),
            drawing_mutation: self.drawing_admission.take().and_then(|mut admission| admission.mutation.take()),
            value_mutation: self.value_admission.take().and_then(|mut admission| admission.mutation.take()),
            failure: self.failure.take()?,
        };
        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty;
        Some(owners)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty
            && self.parent.is_none()
            && self.drawing.is_none()
            && self.value.is_none()
            && self.parent_admission.is_none()
            && self.drawing_admission.is_none()
            && self.value_admission.is_none()
            && self.parent_factory.is_none()
            && self.drawing_factory.is_none()
            && self.value_factory.is_none()
            && self.parent_publication.is_none()
            && self.drawing_publication.is_none()
            && self.value_publication.is_none()
            && self.prepared.is_none()
            && self.host.is_none()
            && self.sink.is_none()
            && self.failure.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedThreeStoreMapAssemblyV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || self.phase == DurableOwnedThreeStoreMapAssemblyPhaseV1::Empty
                && self.parent.is_none()
                && self.drawing.is_none()
                && self.value.is_none()
                && self.parent_admission.is_none()
                && self.drawing_admission.is_none()
                && self.value_admission.is_none()
                && self.parent_publication.is_none()
                && self.drawing_publication.is_none()
                && self.value_publication.is_none()
                && self.prepared.is_none()
                && self.host.is_none()
                && self.sink.is_none(),
            "durable fixed-three Map assembly reached Drop before mounted or terminal owner handoff"
        );
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub fn begin_retained_commit(self) -> Result<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
        self.decision.validate()?;
        let decision_pack = self.decision.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if decision_pack.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let decision_sha256 = self.decision.decision_sha256.clone();
        let visibility_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = visibility_owner.view();
        Ok(DurableOwnedThreeStoreCommitV1 {
            parent: Some(self.parent),
            drawing: Some(self.drawing),
            value: Some(self.value),
            decision: Some(self.decision),
            decision_pack: Some(decision_pack),
            decision_sha256,
            visibility_owner: Some(visibility_owner),
            visibility: Some(visibility),
            journal: None,
            receipt: None,
            phase: DurableOwnedThreeStoreCommitPhaseV1::StagingParent,
            recovery_committed: false,
            cancel_requested: false,
            cancel_forwarded: false,
            journal_close_started: false,
        })
    }

    fn begin_retained_recovery(self, receipt: DurableOwnedGroupJournalReceiptV1) -> Result<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
        self.decision.validate()?;
        if receipt.decision_sha256 != self.decision.decision_sha256 || receipt.anchor_sha256 != self.decision.anchor_sha256 || receipt.transaction_id == 0 {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let decision_sha256 = self.decision.decision_sha256.clone();
        let visibility_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = visibility_owner.view();
        Ok(DurableOwnedThreeStoreCommitV1 {
            parent: Some(self.parent),
            drawing: Some(self.drawing),
            value: Some(self.value),
            decision: Some(self.decision),
            decision_pack: None,
            decision_sha256,
            visibility_owner: Some(visibility_owner),
            visibility: Some(visibility),
            journal: None,
            receipt: Some(receipt),
            phase: DurableOwnedThreeStoreCommitPhaseV1::StagingParent,
            recovery_committed: true,
            cancel_requested: false,
            cancel_forwarded: false,
            journal_close_started: false,
        })
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> DurableOwnedThreeStoreCommitPhaseV1 {
        self.phase
    }

    pub fn cancel(&mut self) -> bool {
        if self.recovery_committed {
            return false;
        }
        if matches!(
            self.phase,
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
                | DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal
                | DurableOwnedThreeStoreCommitPhaseV1::Complete
        ) {
            return false;
        }
        self.cancel_requested = true;
        true
    }

    fn begin_abort(&mut self) -> Result<(), DurableOwnedGroupDecisionError> {
        if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::abort) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingValue;
        Ok(())
    }

    pub fn visibility(&self) -> Result<Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, DurableOwnedGroupDecisionError> {
        self.visibility.as_ref().map(Arc::clone).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)
    }

    fn start_journal_close(&mut self) {
        if self.journal_close_started {
            return;
        }
        if let Some(journal) = self.journal.as_mut() {
            journal.begin_close();
            self.journal_close_started = true;
        }
    }

    pub fn acknowledge(&mut self, receipt: &DurableOwnedGroupJournalReceiptV1) -> bool {
        if self.phase != DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck || self.receipt.as_ref() != Some(receipt) {
            return false;
        }
        self.start_journal_close();
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
            && self.parent.is_none()
            && self.drawing.is_none()
            && self.value.is_none()
            && self.decision.is_none()
            && self.decision_pack.is_none()
            && self.decision_sha256.is_empty()
            && self.visibility_owner.is_none()
            && self.visibility.is_none()
            && self.journal.is_none()
            && self.receipt.is_none()
    }

    pub fn mount_map(
        self,
        parent: ArtifactStore<ParentP, ParentMutation>,
        drawing: ArtifactStore<DrawingP, DrawingMutation>,
        value: ArtifactStore<ValueP, ValueMutation>,
        sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    ) -> DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
        DurableOwnedMapCommitHostV1 {
            operation: std::mem::ManuallyDrop::new(Some(DurableOwnedMapCommitOperationV1 {
                parent: std::mem::ManuallyDrop::new(Some(parent)),
                drawing: std::mem::ManuallyDrop::new(Some(drawing)),
                value: std::mem::ManuallyDrop::new(Some(value)),
                coordinator: std::mem::ManuallyDrop::new(Some(self)),
                sink: std::mem::ManuallyDrop::new(Some(sink)),
            })),
        }
    }

    pub fn advance(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        sink: &mut dyn DurableOwnedGroupJournalSinkV1,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.advance_with_sink(parent_store, drawing_store, value_store, Some(sink), grant)
    }

    fn advance_recovery(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.advance_with_sink(parent_store, drawing_store, value_store, None, grant)
    }

    fn advance_with_sink(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        sink: Option<&mut dyn DurableOwnedGroupJournalSinkV1>,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        if self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete);
        }
        if !grant.permits_one() {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
        }
        if self.cancel_requested
            && matches!(self.phase, DurableOwnedThreeStoreCommitPhaseV1::StagingParent | DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing | DurableOwnedThreeStoreCommitPhaseV1::StagingValue | DurableOwnedThreeStoreCommitPhaseV1::StartingJournal)
        {
            self.begin_abort()?;
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase));
        }
        match self.phase {
            DurableOwnedThreeStoreCommitPhaseV1::StagingParent => {
                let outcome = self.parent.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut parent_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing,
                    Err((error, outcome)) => {
                        self.parent = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing => {
                let outcome = self.drawing.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut drawing_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingValue,
                    Err((error, outcome)) => {
                        self.drawing = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingValue => {
                let outcome = self.value.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut value_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StartingJournal,
                    Err((error, outcome)) => {
                        self.value = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StartingJournal => {
                if self.recovery_committed {
                    let receipt = self.receipt.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                    if receipt.decision_sha256 != self.decision_sha256 || self.decision.as_ref().is_none_or(|decision| receipt.anchor_sha256 != decision.anchor_sha256) {
                        return Err(DurableOwnedGroupDecisionError::InvalidHash);
                    }
                    if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::commit) {
                        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
                    }
                    self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease;
                } else {
                    let decision_bytes = self.decision_pack.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.len();
                    if grant.maximum_copy_bytes < decision_bytes {
                        return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                    }
                    let decision_pack = self.decision_pack.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                    self.journal = Some(sink.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.begin_commit(decision_pack, self.decision_sha256.clone()));
                    self.phase = DurableOwnedThreeStoreCommitPhaseV1::Journal;
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::Journal => {
                let journal = self.journal.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if self.cancel_requested && !self.cancel_forwarded {
                    journal.cancel();
                    self.cancel_forwarded = true;
                }
                match journal.advance(semio_framework_os_store_one_item::ArtifactStoreOneItemGrant { maximum_items: 1, ..grant }).map_err(DurableOwnedGroupDecisionError::Codec)? {
                    DurableOwnedGroupJournalAdvanceV1::Pending => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    DurableOwnedGroupJournalAdvanceV1::Absent => {
                        self.begin_abort()?;
                    }
                    DurableOwnedGroupJournalAdvanceV1::Rejected(reason) => {
                        self.begin_abort()?;
                        return Err(DurableOwnedGroupDecisionError::Codec(reason));
                    }
                    DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => {
                        if receipt.decision_sha256 != self.decision_sha256 || self.decision.as_ref().is_none_or(|decision| receipt.anchor_sha256 != decision.anchor_sha256) {
                            return Err(DurableOwnedGroupDecisionError::InvalidHash);
                        }
                        if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::commit) {
                            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
                        }
                        self.receipt = Some(receipt);
                        self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease;
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease => {
                let root = parent_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !parent_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease => {
                let root = drawing_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !drawing_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease => {
                let root = value_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !value_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent => {
                adopt_staged_store_member(&mut parent_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing => {
                adopt_staged_store_member(&mut drawing_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue => {
                adopt_staged_store_member(&mut value_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingParent => {
                validate_adopted_store_member(&parent_store.parts_ref(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing => {
                validate_adopted_store_member(&drawing_store.parts_ref(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingValue => {
                let visibility = self.visibility()?;
                validate_adopted_store_member(&value_store.parts_ref(), &visibility)?;
                clear_adopted_store_member(&mut parent_store.parts(), &visibility)?;
                clear_adopted_store_member(&mut drawing_store.parts(), &visibility)?;
                clear_adopted_store_member(&mut value_store.parts(), &visibility)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck => {
                return Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(self.receipt.clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?));
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingValue => {
                if self.value.is_some() {
                    retire_unstaged_store_member(&mut value_store.parts(), &mut self.value, grant)?;
                } else {
                    abort_staged_store_member(&mut value_store.parts(), &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing => {
                if self.drawing.is_some() {
                    retire_unstaged_store_member(&mut drawing_store.parts(), &mut self.drawing, grant)?;
                } else {
                    abort_staged_store_member(&mut drawing_store.parts(), &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingParent => {
                if self.parent.is_some() {
                    retire_unstaged_store_member(&mut parent_store.parts(), &mut self.parent, grant)?;
                } else {
                    abort_staged_store_member(&mut parent_store.parts(), &self.visibility()?)?;
                }
                self.start_journal_close();
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal => {
                if self.journal.is_some() {
                    if grant.maximum_depth == 0 { return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked); }
                    let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant.retained_grant() };
                    match close_journal_owner(&mut self.journal, child).map_err(DurableOwnedGroupDecisionError::Retirement)? {
                        semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {},
                        semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == Default::default() => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked),
                        semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    }
                }
                self.decision = None;
                self.decision_pack = None;
                self.decision_sha256.clear();
                self.visibility_owner = None;
                self.visibility = None;
                self.receipt = None;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::Complete;
            }
            DurableOwnedThreeStoreCommitPhaseV1::Complete => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete),
        }
        Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase))
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
                && self.parent.is_none()
                && self.drawing.is_none()
                && self.value.is_none()
                && self.decision.is_none()
                && self.decision_pack.is_none()
                && self.decision_sha256.is_empty()
                && self.visibility_owner.is_none()
                && self.visibility.is_none()
                && self.journal.is_none()
                && self.receipt.is_none(),
            "durable owned group coordinator reached Drop before exact terminal emptiness"
        );
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.coordinator.as_ref().map(DurableOwnedThreeStoreCommitV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        capture_store_owned_three_snapshot(
            self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
        )
    }

    pub fn cancel(&mut self) -> bool {
        self.coordinator.as_mut().is_some_and(DurableOwnedThreeStoreCommitV1::cancel)
    }

    fn acknowledge_retained(&mut self) -> bool {
        let Some(receipt) = self.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.clone()) else { return false };
        self.coordinator.as_mut().is_some_and(|coordinator| coordinator.acknowledge(&receipt))
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        let coordinator = self.coordinator.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let parent = self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let drawing = self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let value = self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let sink = self.sink.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        coordinator.advance(parent, drawing, value, sink.as_mut(), grant)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if !self.coordinator.as_ref().is_some_and(DurableOwnedThreeStoreCommitV1::terminal_is_empty) || self.parent.is_none() || self.drawing.is_none() || self.value.is_none() || self.sink.is_none() {
            return None;
        }
        drop(self.coordinator.take().expect("validated terminal Map coordinator remains owned"));
        Some(DurableOwnedMapCommitOwnersV1 {
            parent: self.parent.take().expect("validated terminal parent Store remains owned"),
            drawing: self.drawing.take().expect("validated terminal drawing Store remains owned"),
            value: self.value.take().expect("validated terminal value Store remains owned"),
            sink: self.sink.take().expect("validated terminal journal sink remains owned"),
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none()), "retained Map commit operation reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.operation.as_ref().and_then(DurableOwnedMapCommitOperationV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        self.operation.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.capture_snapshot()
    }

    pub fn cancel(&mut self) -> bool {
        self.operation.as_mut().is_some_and(DurableOwnedMapCommitOperationV1::cancel)
    }

    pub fn acknowledge(&mut self) -> bool {
        self.operation.as_mut().is_some_and(DurableOwnedMapCommitOperationV1::acknowledge_retained)
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.operation.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.advance(grant)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let owners = self.operation.as_mut()?.take_terminal_owners()?;
        drop(self.operation.take().expect("terminal Map host retains its empty operation slot"));
        Some(owners)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.coordinator.as_ref().map(DurableOwnedThreeStoreCommitV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        capture_store_owned_three_snapshot(
            self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
        )
    }

    pub fn acknowledge(&mut self) -> bool {
        let Some(receipt) = self.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.clone()) else { return false };
        self.coordinator.as_mut().is_some_and(|coordinator| coordinator.acknowledge(&receipt))
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.coordinator.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.advance_recovery(
            self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            grant,
        )
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if !self.coordinator.as_ref().is_some_and(DurableOwnedThreeStoreCommitV1::terminal_is_empty) || self.parent.is_none() || self.drawing.is_none() || self.value.is_none() {
            return None;
        }
        drop(self.coordinator.take().expect("validated terminal Map recovery coordinator remains owned"));
        Some(DurableOwnedMapRecoveryOwnersV1 {
            parent: self.parent.take().expect("validated terminal recovered parent Store remains owned"),
            drawing: self.drawing.take().expect("validated terminal recovered drawing Store remains owned"),
            value: self.value.take().expect("validated terminal recovered value Store remains owned"),
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none()), "retained committed Map recovery reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.operation.as_ref().and_then(DurableOwnedMapRecoveryOperationV1::phase)
    }

    pub fn capture_snapshot(&self) -> Option<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>> {
        self.operation.as_ref()?.capture_snapshot().ok()
    }

    #[doc(hidden)]
    pub fn acknowledge_restoration(&mut self, expected: &DurableOwnedGroupJournalReceiptV1) -> bool {
        let Some(operation) = self.operation.as_mut() else { return false };
        let Some(actual) = operation.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.as_ref()) else { return false };
        if actual != expected {
            return false;
        }
        operation.acknowledge()
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1 {
        let Some(operation) = self.operation.as_mut() else { return DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidOutcome) };
        match operation.advance(grant) {
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(phase)) => DurableOwnedMapRecoveryAdvanceV1::Progress(phase),
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => DurableOwnedMapRecoveryAdvanceV1::Blocked,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => DurableOwnedMapRecoveryAdvanceV1::Complete,
            Err(error) => DurableOwnedMapRecoveryAdvanceV1::Fault(error),
        }
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let owners = self.operation.as_mut()?.take_terminal_owners()?;
        drop(self.operation.take().expect("terminal Map recovery host retains its empty operation slot"));
        Some(owners)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "committed Map recovery host reached Drop before terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "retained Map host reached Drop before exact terminal owner handoff");
    }
}























impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ParentMutation: ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    DrawingMutation: ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ValueMutation: ValueToValue + ValueFromValue,
{
    pub fn from_store_prepared(
        parent: ArtifactStoreOneItemPrepared<ParentP, ParentMutation>,
        drawing: ArtifactStoreOneItemPrepared<DrawingP, DrawingMutation>,
        value: ArtifactStoreOneItemPrepared<ValueP, ValueMutation>,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        Ok(Self { parent: DurableStorePreparedOwnerV1::unbound(parent, PARENT_RECOVERY_SCHEMA)?, drawing: DurableStorePreparedOwnerV1::unbound(drawing, DRAWING_RECOVERY_SCHEMA)?, value: DurableStorePreparedOwnerV1::unbound(value, VALUE_RECOVERY_SCHEMA)? })
    }
}





impl DurableOwnedThreeMemberDecisionV1 {
    pub fn recover_store_owned<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError>
    where
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
    {
        self.validate()?;
        let identities = [store_member_identity(&parent_store.parts_ref())?, store_member_identity(&drawing_store.parts_ref())?, store_member_identity(&value_store.parts_ref())?];
        for ((reference, owner), member) in identities.iter().zip([&self.parent, &self.drawing, &self.value]) {
            if reference != &member.reference || owner != &member.owner {
                return Err(DurableOwnedGroupDecisionError::InvalidOwner);
            }
        }
        let frontiers = [recovery_frontier(&parent_store.parts_ref(), &self.parent, &self.decision_sha256)?, recovery_frontier(&drawing_store.parts_ref(), &self.drawing, &self.decision_sha256)?, recovery_frontier(&value_store.parts_ref(), &self.value, &self.decision_sha256)?];
        if frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Post) {
            return Ok(DurableOwnedThreeStoreRecoveryV1::AlreadyApplied);
        }
        if !frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Base) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(DurableOwnedThreeStoreRecoveryV1::Apply(DurableOwnedThreeStoreBoundV1 {
            parent: DurableStoreBoundOutcomeV1::recover(&parent_store.parts_ref(), &self.parent, &self.decision_sha256)?,
            drawing: DurableStoreBoundOutcomeV1::recover(&drawing_store.parts_ref(), &self.drawing, &self.decision_sha256)?,
            value: DurableStoreBoundOutcomeV1::recover(&value_store.parts_ref(), &self.value, &self.decision_sha256)?,
            decision: self.clone(),
        }))
    }
}









impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
{
    pub fn bind_store_owned(
        self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let (parent_reference, parent_owner) = store_member_identity(&parent_store.parts_ref())?;
        let (drawing_reference, drawing_owner) = store_member_identity(&drawing_store.parts_ref())?;
        let (value_reference, value_owner) = store_member_identity(&value_store.parts_ref())?;
        if parent_reference == drawing_reference || parent_reference == value_reference || drawing_reference == value_reference {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let anchor = DurableOwnedGroupAnchorV1 { schema: DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1.into(), parent: parent_reference.clone(), shape: DURABLE_OWNED_GROUP_SHAPE_V1.into() };
        let parent = prepared_member(PARENT_ROLE, parent_reference.clone(), parent_owner.clone(), &self.parent)?;
        let drawing = prepared_member(DRAWING_ROLE, drawing_reference.clone(), drawing_owner.clone(), &self.drawing)?;
        let value = prepared_member(VALUE_ROLE, value_reference.clone(), value_owner.clone(), &self.value)?;
        if !member_identity_matches(&parent, PARENT_ROLE, PARENT_RECOVERY_SCHEMA, &parent_reference.to_uri(), &anchor)
            || !member_identity_matches(&drawing, DRAWING_ROLE, DRAWING_RECOVERY_SCHEMA, "gismap-drawing!s.stdio.semio@v1/drawing", &anchor)
            || !member_identity_matches(&value, VALUE_ROLE, VALUE_RECOVERY_SCHEMA, "gismap-value!s.stdio.semio@v1/value", &anchor)
            || !prepared_base_matches_store(&parent_store.parts_ref(), &parent, &self.parent.prepared)
            || !prepared_base_matches_store(&drawing_store.parts_ref(), &drawing, &self.drawing.prepared)
            || !prepared_base_matches_store(&value_store.parts_ref(), &value, &self.value.prepared)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
        let anchor_sha256 = semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(&anchor).as_bytes());
        let mut decision = DurableOwnedThreeMemberDecisionV1 { schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(), anchor, anchor_sha256, decision_sha256: String::new(), parent, drawing, value };
        decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
        let parent = self.parent.bind(&parent_store.parts_ref(), &decision.decision_sha256)?;
        let drawing = self.drawing.bind(&drawing_store.parts_ref(), &decision.decision_sha256)?;
        let value = self.value.bind(&value_store.parts_ref(), &decision.decision_sha256)?;
        decision.parent = bound_member(PARENT_ROLE, parent_reference, parent_owner, &parent);
        decision.drawing = bound_member(DRAWING_ROLE, drawing_reference, drawing_owner, &drawing);
        decision.value = bound_member(VALUE_ROLE, value_reference, value_owner, &value);
        decision.validate()?;
        Ok(DurableOwnedThreeStoreBoundV1 { decision, parent, drawing, value })
    }
}



























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
        semio_framework_pack_json::to_json_string(&DurableOwnedThreeMemberUnsignedV1 {
            schema: self.schema.clone(),
            anchor: self.anchor.clone(),
            parent: unsigned_member(&self.parent),
            drawing: unsigned_member(&self.drawing),
            value: unsigned_member(&self.value),
        })
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
        if manifest.schema_hash != semio_framework_os_pack::schema_hash(&Self::__dsl_spec()) {
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
    parent_edit: Edit<ParentMutation>,
    parent_post_snapshot: ParentP,
    drawing_edit: Edit<DrawingMutation>,
    drawing_post_snapshot: DrawingP,
    value_edit: Edit<ValueMutation>,
    value_post_snapshot: ValueP,
) -> Result<DurableOwnedGroupJournalRecordV1, DurableOwnedGroupDecisionError>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ParentMutation: ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    DrawingMutation: ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ValueMutation: ValueToValue + ValueFromValue,
{
    let parent_outcome = durable_group_test_outcome(PARENT_RECOVERY_SCHEMA, 1, parent_edit, parent_post_snapshot)?;
    let drawing_outcome = durable_group_test_outcome(DRAWING_RECOVERY_SCHEMA, 2, drawing_edit, drawing_post_snapshot)?;
    let value_outcome = durable_group_test_outcome(VALUE_RECOVERY_SCHEMA, 3, value_edit, value_post_snapshot)?;
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


