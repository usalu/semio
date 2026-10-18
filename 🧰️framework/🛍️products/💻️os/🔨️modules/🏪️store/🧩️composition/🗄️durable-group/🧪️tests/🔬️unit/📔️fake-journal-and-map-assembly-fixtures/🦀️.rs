/// 📔️ Fake journal and map assembly fixtures: the fake journal resolution, state, sink and commit, the durable journal close, the demo map assembly and its member admission, drive and drain helpers.

#[derive(Clone, Copy)]
enum FakeJournalResolution {
    Commit,
    AbsentAfterCancel,
    ErrorThenCommit,
    ErrorThenAbsentAfterCancel,
    WrongAnchorThenAbsentAfterCancel,
}

#[derive(Default)]
struct FakeJournalState {
    begins: usize,
    advances: usize,
    cancelled: bool,
    close_count: usize,
    decision_pack: Vec<u8>,
}

struct FakeJournalSink {
    resolution: FakeJournalResolution,
    state: Arc<std::sync::Mutex<FakeJournalState>>,
}

struct FakeJournalCommit {
    resolution: FakeJournalResolution,
    state: Option<Arc<std::sync::Mutex<FakeJournalState>>>,
    decision_sha256: String,
    anchor_sha256: String,
    close_started: bool,
}

impl DurableOwnedGroupJournalSinkV1 for FakeJournalSink {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn DurableOwnedGroupJournalCommitV1> {
        let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&decision_pack).expect("fake journal independently admits the canonical decision");
        assert_eq!(decision.decision_sha256, decision_sha256);
        let mut state = self.state.lock().unwrap();
        state.begins += 1;
        state.decision_pack = decision_pack;
        drop(state);
        Box::new(FakeJournalCommit { resolution: self.resolution, state: Some(Arc::clone(&self.state)), decision_sha256, anchor_sha256: decision.anchor_sha256, close_started: false })
    }
}

impl DurableOwnedGroupJournalCommitV1 for FakeJournalCommit {
    fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedGroupJournalAdvanceV1, String> {
        if !grant.permits_one() {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        let mut state = self.state.as_ref().unwrap().lock().unwrap();
        state.advances += 1;
        if state.advances == 1 {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        match self.resolution {
            FakeJournalResolution::Commit => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 41, segment_index: 7 }))
            }
            FakeJournalResolution::AbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::AbsentAfterCancel => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            FakeJournalResolution::ErrorThenCommit if state.advances == 2 => Err("uncertain sync result".into()),
            FakeJournalResolution::ErrorThenCommit => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 43, segment_index: 9 }))
            }
            FakeJournalResolution::ErrorThenAbsentAfterCancel if state.advances == 2 => Err("uncertain sync result".into()),
            FakeJournalResolution::ErrorThenAbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::ErrorThenAbsentAfterCancel => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            FakeJournalResolution::WrongAnchorThenAbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::WrongAnchorThenAbsentAfterCancel => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: "0".repeat(64), decision_sha256: self.decision_sha256.clone(), transaction_id: 47, segment_index: 11 }))
            }
        }
    }

    fn cancel(&mut self) {
        self.state.as_ref().unwrap().lock().unwrap().cancelled = true;
    }

    fn begin_close(&mut self) {
        self.state.as_ref().unwrap().lock().unwrap().close_count += 1;
        self.close_started = true;
    }

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let release_bytes = if self.decision_sha256.capacity() != 0 { self.decision_sha256.capacity() } else if self.anchor_sha256.capacity() != 0 { self.anchor_sha256.capacity() } else if self.state.as_ref().is_some_and(|state| Arc::strong_count(state) < 2) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "journal fixture retains its original observer authority")); } else { 0 };
        Ok(semio_framework_value::RetirementDemand { release_bytes, depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

 }

impl semio_framework_value::ErasedSnapshotRetirement for FakeJournalCommit {
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.depth) }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.close_started || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.decision_sha256.capacity() != 0 { drop(std::mem::take(&mut self.decision_sha256)); }
        else if self.anchor_sha256.capacity() != 0 { drop(std::mem::take(&mut self.anchor_sha256)); }
        else { drop(self.state.take()); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn terminal_is_empty(&self) -> bool { self.close_started && self.decision_sha256.capacity() == 0 && self.anchor_sha256.capacity() == 0 && self.state.is_none() }

}

#[test]
fn durable_journal_close_preserves_original_capacities_and_exact_receipts() {
    use semio_framework_value::{RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️journal/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut decision = String::with_capacity(row["decisionCapacity"].as_u64().unwrap() as usize);
        decision.push_str(row["decision"].as_str().unwrap());
        let mut anchor = String::with_capacity(row["anchorCapacity"].as_u64().unwrap() as usize);
        anchor.push_str(row["anchor"].as_str().unwrap());
        let original = (decision.as_ptr(), anchor.as_ptr());
        let physical = decision.capacity() + anchor.capacity() + std::mem::size_of::<FakeJournalCommit>();
        let mut owner: Option<Box<dyn DurableOwnedGroupJournalCommitV1>> = Some(Box::new(FakeJournalCommit { resolution: FakeJournalResolution::Commit, state: Some(Arc::clone(&state)), decision_sha256: decision, anchor_sha256: anchor, close_started: false }));
        owner.as_mut().unwrap().begin_close();
        let frame_pointer = owner.as_ref().unwrap().as_ref() as *const dyn DurableOwnedGroupJournalCommitV1 as *const ();
        let mut released = 0;
        for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
            let Some(journal) = owner.as_ref() else { break; };
            let demand = if journal.terminal_is_empty() { RetirementDemand { release_bytes: std::mem::size_of_val(journal.as_ref()), depth: 1, ..Default::default() } } else { journal.retirement_demands(4096).unwrap() };
            let grant = RetainedCloneGrant { maximum_copy_bytes: fixture["maximumBodyBytes"].as_u64().unwrap() as usize, ..semio_framework_value::testing::physical_test_close_grant() };
            let (quote, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| journal.retirement_demands(grant.maximum_copy_bytes).unwrap());
            assert_eq!((born, freed), (0, 0));
            for denial in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }] {
                let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| close_journal_owner(&mut owner, denial).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((born, freed), (0, 0));
                assert_eq!(owner.as_ref().unwrap().as_ref() as *const dyn DurableOwnedGroupJournalCommitV1 as *const (), frame_pointer);
            }
            if demand.release_bytes != 0 {
                let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| close_journal_owner(&mut owner, RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((born, freed), (0, 0));
                assert_eq!(owner.as_ref().unwrap().retirement_demands(grant.maximum_copy_bytes).unwrap(), quote);
            }
            let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| close_journal_owner(&mut owner, grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(born, 0);
            assert_eq!(freed, step.progress().released_bytes);
            released += freed;
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.is_none()); }
        }
        assert!(owner.is_none());
        assert_eq!(released, physical);
        assert_eq!(state.lock().unwrap().close_count, 1);
        let (_, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| drop(owner));
        assert_eq!((born, freed), (0, 0));
    }
}

type DemoMapAssembly = DurableOwnedThreeStoreMapAssemblyV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation>;

fn map_member_admission(store: &ArtifactStore<DemoSnapshot, DemoMutation>, ordinal: u64, next: i32, stale: bool) -> DurableOwnedMapMemberAdmissionV1<DemoMutation> {
    DurableOwnedMapMemberAdmissionV1::new(
        semio_framework_job::OperationId(300 + ordinal),
        store.generation_now() + u64::from(stale),
        store.content_revision_now(),
        semio_framework_value::SharedUtf8::admit(format!("map-owner-{ordinal}"), semio_framework_value::testing::physical_test_close_grant()).unwrap_or_else(|(error,_)| panic!("funded original actor: {error}")).0,
        DemoMutation::SetN(SetN { n: next }),
    )
}

fn map_assembly(
    parent: ArtifactStore<DemoSnapshot, DemoMutation>,
    drawing: ArtifactStore<DemoSnapshot, DemoMutation>,
    value: ArtifactStore<DemoSnapshot, DemoMutation>,
    stale_role: Option<&str>,
    resolution: FakeJournalResolution,
    state: Arc<std::sync::Mutex<FakeJournalState>>,
) -> (DemoMapAssembly, [Arc<DemoOneItemPreparationFactory>; 3]) {
    let parent_admission = map_member_admission(&parent, 1, 7, stale_role == Some(PARENT_ROLE));
    let drawing_admission = map_member_admission(&drawing, 2, 11, stale_role == Some(DRAWING_ROLE));
    let value_admission = map_member_admission(&value, 3, 13, stale_role == Some(VALUE_ROLE));
    let factories = std::array::from_fn(|_| Arc::new(DemoOneItemPreparationFactory::admissible()));
    let parent_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[0].clone();
    let drawing_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[1].clone();
    let value_factory: Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[2].clone();
    (DurableOwnedThreeStoreMapAssemblyV1::new(parent, drawing, value, parent_admission, drawing_admission, value_admission, parent_factory, drawing_factory, value_factory, Box::new(FakeJournalSink { resolution, state })), factories)
}

fn drive_assembly_to_mounted(assembly: &mut DemoMapAssembly) {
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    for _ in 0..65_536 {
        match assembly.advance(grant).expect("fixed-three assembly turn") {
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted => return,
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(_) | DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked => {}
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal => panic!("valid assembly reached rejection terminal"),
        }
    }
    panic!("fixed-three assembly did not mount within its bounded turns");
}

fn drain_assembly_rejection(assembly: &mut DemoMapAssembly) -> DurableOwnedThreeStoreMapAssemblyOwnersV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation> {
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    for _ in 0..65_536 {
        match assembly.advance(grant).expect("rejected fixed-three assembly cleanup turn") {
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal => return assembly.take_terminal_owners().expect("terminal assembly returns every owner"),
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(_) | DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked => {}
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted => panic!("rejected assembly mounted a journal host"),
        }
    }
    panic!("rejected fixed-three assembly did not terminate within its bounded turns");
}

fn close_assembly_owners(owners: DurableOwnedThreeStoreMapAssemblyOwnersV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation>) {
    let DurableOwnedThreeStoreMapAssemblyOwnersV1 { mut parent, mut drawing, mut value, sink, .. } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}
