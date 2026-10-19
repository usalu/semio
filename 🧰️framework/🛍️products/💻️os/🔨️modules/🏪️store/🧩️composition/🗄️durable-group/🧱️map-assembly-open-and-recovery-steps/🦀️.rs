//! 🧱️ Map assembly open and recovery steps: the map assembly operations that open, admit and recover the three member stores. See [`super`].

use super::*;

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
                        parent_prepared: None,
            drawing_prepared: None,
            value_prepared: None,
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

    pub fn advance(&mut self, body:usize, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedGroupDecisionError> {
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
                match store.advance_apply_batch(publication, body, grant,receiving) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked)=> return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,receipt))=>{if publication.phase()==super::ArtifactStoreOneItemPublicationPhase::Publishing{self.phase=DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing;}return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,receipt));}
                    Ok(_) if publication.phase() == super::ArtifactStoreOneItemPublicationPhase::Publishing => self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing,
                    Ok(_) => {}
                    Err(error) => self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Preparation { role: PARENT_ROLE, reason: error.to_string() }),
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing => {
                let store = self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let publication = self.drawing_publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.advance_apply_batch(publication, body, grant,receiving) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked)=> return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,receipt))=>{if publication.phase()==super::ArtifactStoreOneItemPublicationPhase::Publishing{self.phase=DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingValue;}return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,receipt));}
                    Ok(_) if publication.phase() == super::ArtifactStoreOneItemPublicationPhase::Publishing => self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingValue,
                    Ok(_) => {}
                    Err(error) => self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Preparation { role: DRAWING_ROLE, reason: error.to_string() }),
                }
            }
            DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingValue => {
                let store = self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                let publication = self.value_publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match store.advance_apply_batch(publication, body, grant,receiving) {
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::Blocked)=> return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked),
                    Ok(semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,receipt))=>{if publication.phase()==super::ArtifactStoreOneItemPublicationPhase::Publishing{self.phase=DurableOwnedThreeStoreMapAssemblyPhaseV1::TakingPrepared;}return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,receipt));}
                    Ok(_) if publication.phase() == super::ArtifactStoreOneItemPublicationPhase::Publishing => {
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
                                                if let Some(owner) = self.parent_prepared.as_mut() { if !retire_binding_member(owner, self.parent.as_mut().unwrap(), grant)? { return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,owner.retirement_progress)); } self.parent_prepared = None; return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(self.phase)); }
                        if let Some(owner) = self.drawing_prepared.as_mut() { if !retire_binding_member(owner, self.drawing.as_mut().unwrap(), grant)? { return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,owner.retirement_progress)); } self.drawing_prepared = None; return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(self.phase)); }
                        if let Some(owner) = self.value_prepared.as_mut() { if !retire_binding_member(owner, self.value.as_mut().unwrap(), grant)? { return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,owner.retirement_progress)); } self.value_prepared = None; return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(self.phase)); }
                        if let Some(prepared) = self.prepared.as_mut() { if !prepared.retire_originals_step(self.parent.as_mut().unwrap(), self.drawing.as_mut().unwrap(), self.value.as_mut().unwrap(), grant)? { return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,prepared.retirement_progress)); } }
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
                                    grant,
);
                match bound.and_then(DurableOwnedThreeStoreBoundV1::begin_retained_commit) {
                    Ok(coordinator) => {
                                                drop(self.prepared.take());
let host = coordinator.mount_map(
                            self.parent.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.drawing.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.value.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                            self.sink.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
                        );
                        *self.host = Some(host);
                        self.phase = DurableOwnedThreeStoreMapAssemblyPhaseV1::Mounted;
                                            return Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::OwnershipProgress(self.phase,semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()}));
}
                    Err((error, bound)) => {
                        prepared.parent.bound = Some(bound.parent); prepared.drawing.bound = Some(bound.drawing); prepared.value.bound = Some(bound.value); prepared.decision = Some(bound.decision); prepared.decision_pack = bound.decision_pack;
                        self.fail(DurableOwnedThreeStoreMapAssemblyFailureV1::Binding(error));
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
            && self.parent_prepared.is_none()
            && self.drawing_prepared.is_none()
            && self.value_prepared.is_none()
            && self.prepared.is_none()
            && self.host.is_none()
            && self.sink.is_none()
            && self.failure.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedThreeStoreMapAssemblyV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ValueToValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
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
                && self.parent_prepared.is_none()
                && self.drawing_prepared.is_none()
                && self.value_prepared.is_none()
                && self.prepared.is_none()
                && self.host.is_none()
                && self.sink.is_none(),
            "durable fixed-three Map assembly reached Drop before mounted or terminal owner handoff"
        );
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub fn begin_retained_commit(mut self) -> Result<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, (DurableOwnedGroupDecisionError, Self)> {
        if self.decision_pack.is_none() {
            return Err((DurableOwnedGroupDecisionError::InvalidOutcome,self));
        }
        let decision_pack = self.decision_pack.take().expect("decision admission retains its exact pack");
        let decision_sha256 = self.decision.decision_sha256.clone();
        let visibility_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = visibility_owner.view();
        Ok(DurableOwnedThreeStoreCommitV1 {
            parent: Some(self.parent),
            drawing: Some(self.drawing),
            value: Some(self.value),
            decision: Some(self.decision),
                        decision_retirement:None,output_retirement:None,
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

    pub(super) fn begin_retained_recovery(self, receipt: DurableOwnedGroupJournalReceiptV1) -> Result<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
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
                        decision_retirement:None,output_retirement:None,
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
