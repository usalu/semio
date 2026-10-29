//! 🛣️ Map commit, recovery hosts and assembly owners: the three store commit, the map commit operation, host and owners, the map recovery admission, owners, operation, host and start and the map assembly owners and assembly. See [`super`].

use super::*;

/// 🧷️ Store-owned fixed-three coordinator retaining all candidates through journal resolution.
pub(crate) struct DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub(super) parent: Option<DurableStoreBoundOutcomeV1<ParentP, ParentMutation>>,
    pub(super) drawing: Option<DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>>,
    pub(super) value: Option<DurableStoreBoundOutcomeV1<ValueP, ValueMutation>>,
    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,
    pub(super) decision_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>,
    pub(super) output_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>,
    pub(super) decision_pack: Option<Vec<u8>>,
    pub(super) decision_sha256: String,
    pub(super) visibility_owner: Option<semio_framework_os_vcs::ArtifactGroupVisibilityOwner>,
    pub(super) visibility: Option<Arc<semio_framework_os_vcs::ArtifactGroupVisibility>>,
    pub(super) journal: Option<Box<dyn DurableOwnedGroupJournalCommitV1>>,
    pub receipt: Option<DurableOwnedGroupJournalReceiptV1>,
    pub phase: DurableOwnedThreeStoreCommitPhaseV1,
    pub(super) recovery_committed: bool,
    pub cancel_requested: bool,
    pub(super) cancel_forwarded: bool,
    pub(super) journal_close_started: bool,
}

/// 🗺️ Retained Map publication owner. The three Stores, coordinator, and exclusive journal sink
/// cross the request boundary together and can leave it only through one terminal handoff.
pub(super) struct DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    pub(super) coordinator: std::mem::ManuallyDrop<Option<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub sink: std::mem::ManuallyDrop<Option<Box<dyn DurableOwnedGroupJournalSinkV1>>>,
}

/// 🏠️ Fixed host slot for one retained Map publication. The mounted operation is never exposed
/// to a request owner and cannot be replaced before its exact terminal handoff.
pub struct DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub operation: std::mem::ManuallyDrop<Option<DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🧳 Exact live owners returned after one retained Map publication reaches terminal emptiness.
pub struct DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
}

/// ♻️ Exact three Store owners returned by committed recovery after terminal publication or an idempotent post-frontier match.
pub struct DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
}

/// 🚧 Committed recovery rejection preserves every Store owner for corrected retry or close.
pub struct DurableOwnedMapRecoveryRejectedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub error: DurableOwnedGroupDecisionError,
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
}

pub(super) struct DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    pub drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    pub value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    pub(super) coordinator: std::mem::ManuallyDrop<Option<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🧭 Retained committed-recovery host; no journal sink exists because its opaque DB witness already proves the transaction committed.
pub struct DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub operation: std::mem::ManuallyDrop<Option<DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

/// 🔀 Either the exact committed decision still needs Store publication or all three Stores already match its post-frontier.
#[expect(clippy::large_enum_variant, reason = "Recovery transfers the exact admitted operation or original store owners without allocating during handoff.")]
pub enum DurableOwnedMapRecoveryStartV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    Apply(DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    AlreadyApplied(DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + Send + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + 'static,
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
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none()), "committed recovery admission reached Drop before exact DB witness transfer");
    }
}

/// 🧳 Exact owners returned after a pre-journal rejection or cancellation reaches terminal emptiness.
pub struct DurableOwnedThreeStoreMapAssemblyOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ParentP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<DrawingP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<ValueP> + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
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
    pub(super) parent_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<ParentP, ParentMutation>>>,
    pub(super) drawing_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<DrawingP, DrawingMutation>>>,
    pub(super) value_publication: std::mem::ManuallyDrop<Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<ValueP, ValueMutation>>>,
    pub prepared: std::mem::ManuallyDrop<Option<DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub host: std::mem::ManuallyDrop<Option<DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    pub sink: std::mem::ManuallyDrop<Option<Box<dyn DurableOwnedGroupJournalSinkV1>>>,
    pub phase: DurableOwnedThreeStoreMapAssemblyPhaseV1,
    pub failure: Option<DurableOwnedThreeStoreMapAssemblyFailureV1>,
    pub parent_prepared: Option<DurableStorePreparedOwnerV1<ParentP, ParentMutation>>,
    pub drawing_prepared: Option<DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>>,
    pub value_prepared: Option<DurableStorePreparedOwnerV1<ValueP, ValueMutation>>,
}
