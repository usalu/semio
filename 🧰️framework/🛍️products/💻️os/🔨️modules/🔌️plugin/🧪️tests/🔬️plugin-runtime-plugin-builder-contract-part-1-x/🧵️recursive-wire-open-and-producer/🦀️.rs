// 🧵️ Recursive wire, open and producer: the recursive fixture wire helpers, the branch snapshot open operation and retirement, the member store producer and the parent restore corpus law.

//#region 🧬️ComposedParentFixture
fn recursive_fixture_wire_source(mutation: &RecursiveFixtureMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    Some(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"recursive", body: mutation })
}

fn recursive_fixture_wire_schema(_mutation: &RecursiveFixtureMutation) -> Option<(&'static str, &'static str)> {
    Some((RECURSIVE_FIXTURE_SEMANTICS.entity, RECURSIVE_FIXTURE_SEMANTICS.kind))
}

/// 🌱️ Quotes the fixture catalog: the bounded retirement source plus the wire-carrying preparation authority that lets its stores author.
#[doc(hidden)]
pub fn recursive_fixture_owners_birth_demand<P>() -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Sync,
    RecursiveFixtureMutation: Mutation<P>,
{
    let base = store::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<P, RecursiveFixtureMutation>()?;
    let preparation = store::operation_wire_preparation_factory_source_birth_demand::<P, RecursiveFixtureMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<P, RecursiveFixtureMutation, store::MutationApplyEdit<P, RecursiveFixtureMutation>>(), depth: 1 })?;
    let capacity_bytes = base.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "fixture catalog original source tree capacity overflow"))?;
    Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: base.depth.max(preparation.depth) })
}

/// 🗃️ Admits the fixture catalog under `grant`: bounded retirement owners and one wire-carrying preparation factory.
#[doc(hidden)]
pub fn recursive_fixture_owners<P>(prefix: &'static str, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<P, RecursiveFixtureMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<P, RecursiveFixtureMutation>>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Sync,
    RecursiveFixtureMutation: Mutation<P>,
{
    let preparation = store::operation_wire_preparation_factory_source_birth_demand::<P, RecursiveFixtureMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<P, RecursiveFixtureMutation, store::MutationApplyEdit<P, RecursiveFixtureMutation>>(), depth: 1 })
        .map_err(|error| store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
    store::os_store::document_store_owners::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || {
        store::operation_wire_preparation_factory(store::mutation_apply_preparation_factory::<P, RecursiveFixtureMutation>(), recursive_fixture_wire_source, recursive_fixture_wire_schema)
    })
}

/// 🏗️ Funds the fixture catalog from its own quotes for stores no scheduler grants.
fn funded_recursive_fixture_owners<P>(prefix: &'static str) -> Result<store::os_store::document_store_owners::DocumentStoreOwners<P, RecursiveFixtureMutation>, semio_framework_value::ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + semio_framework_value::retained_clone::RetainedClone + Sync,
    RecursiveFixtureMutation: Mutation<P>,
{
    store::os_store::document_store_owners::fund_document_store_owners(recursive_fixture_owners_birth_demand::<P>()?, |grant| recursive_fixture_owners::<P>(prefix, grant))
}

#[doc(hidden)]
pub struct RecursiveBranchSnapshotOpen {
    pub request: std::mem::ManuallyDrop<Option<store::MemberOpenRequest>>,
    pub snapshot: std::mem::ManuallyDrop<Option<RecursiveBranchSnapshot>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub input: Vec<u8>,
    pub expected_bytes: Option<usize>,
    pub diagnostic: Option<store::MemberOpenDiagnostic>,
    pub terminal: bool,
}

impl RecursiveBranchSnapshotOpen {
    pub fn reject(&mut self, diagnostic: store::MemberOpenDiagnostic) -> store::MemberSnapshotOpenStep {
        self.diagnostic.get_or_insert(diagnostic);
        store::MemberSnapshotOpenStep::Rejected(self.diagnostic.expect("recursive snapshot diagnostic"))
    }
}

impl store::MemberSnapshotOpenOperation for RecursiveBranchSnapshotOpen {
    type Snapshot = RecursiveBranchSnapshot;
    fn begin_birth_bytes(request: &store::MemberOpenRequest) -> Result<usize, store::MemberOpenDiagnostic> { request.admitted_expected().map(|_| 0) }

    fn begin(request: store::MemberOpenRequest) -> Result<Self, store::MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(store::MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self {
            request: std::mem::ManuallyDrop::new(Some(request)),
            snapshot: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            input: Vec::new(),
            expected_bytes: None,
            diagnostic: None,
            terminal: false,
        })
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> store::MemberSnapshotOpenStep {
        if let Some(diagnostic) = self.diagnostic {
            return store::MemberSnapshotOpenStep::Rejected(diagnostic);
        }
        if self.terminal {
            return store::MemberSnapshotOpenStep::Rejected(store::MemberOpenDiagnostic::Stale);
        }
        if let Err(diagnostic) = self.request.as_ref().expect("recursive snapshot retains request").check_step_authority(cx) { return self.reject(diagnostic); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || cx.should_yield() { return store::MemberSnapshotOpenStep::Pending(store::MemberSnapshotOpenProgress { opening: store::MemberOpenProgress { phase: store::MemberOpenPhase::Snapshot, completed: self.input.len() as u64, total: self.expected_bytes.unwrap_or(0) as u64 }, retained_progress: Default::default() }); }
        let frame = match self.request.as_mut().expect("recursive snapshot retains request").step_input(cx) {
            store::MemberOpenInputStep::Framed(frame) => frame,
            store::MemberOpenInputStep::Pending(progress) => return store::MemberSnapshotOpenStep::Pending(store::MemberSnapshotOpenProgress { opening: progress, retained_progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } }),
            store::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let expected_bytes = frame.snapshot_range().1;
        if self.expected_bytes.is_none() {
            let mut retained = None;
            let result = store::admit_member_input_buffer(&mut retained, expected_bytes, grant);
            if let Some(input) = retained { self.input = input; self.expected_bytes = Some(expected_bytes); }
            return match result {
                Ok(progress) => { let progress = progress.unwrap_or_default(); cx.consume_fuel(progress.copied_items as u64); store::MemberSnapshotOpenStep::Pending(store::MemberSnapshotOpenProgress { opening: store::MemberOpenProgress { phase: store::MemberOpenPhase::Snapshot, completed: 0, total: expected_bytes as u64 }, retained_progress: progress }) }
                Err(_) => self.reject(store::MemberOpenDiagnostic::Capacity),
            };
        }
        if self.input.len() < expected_bytes {
            let mut chunk = [0u8; 256];
            let maximum = chunk.len().min(expected_bytes - self.input.len()).min(grant.maximum_copy_bytes / 2).min(usize::try_from(cx.fuel_remaining() / 2).unwrap_or(usize::MAX));
            let copied = match self.request.as_ref().expect("recursive snapshot retains request").copy_snapshot_chunk(self.input.len(), &mut chunk[..maximum], cx) {
                Ok(copied) => copied,
                Err(diagnostic) => return self.reject(diagnostic),
            };
            self.input.extend_from_slice(&chunk[..copied]);
            cx.consume_fuel(copied as u64);
            return store::MemberSnapshotOpenStep::Pending(store::MemberSnapshotOpenProgress { opening: store::MemberOpenProgress { phase: store::MemberOpenPhase::Snapshot, completed: self.input.len() as u64, total: expected_bytes as u64 }, retained_progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: usize::from(copied != 0), copied_bytes: copied * 2, ..Default::default() } });
        }
        if self.snapshot.is_none() {
            let snapshot = match <RecursiveBranchSnapshot as ArtifactPack>::decode_pack(&self.input) {
                Ok(snapshot) => snapshot,
                Err(_) => return self.reject(store::MemberOpenDiagnostic::Decode),
            };
            *self.snapshot = Some(snapshot);
        }
        store::MemberSnapshotOpenStep::Ready
    }

    fn take_ready(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Option<(Self::Snapshot, store::MemberOpenRequest)> {
        if self.terminal || self.diagnostic.is_some() || self.request.as_ref()?.check_step_authority(cx).is_err() || self.input.len() != self.expected_bytes? {
            return None;
        }
        let snapshot = self.snapshot.take()?;
        let request = self.request.take()?;
        drop(std::mem::take(&mut self.input));
        self.expected_bytes = None;
        self.terminal = true;
        Some((snapshot, request))
    }
}

impl RecursiveBranchSnapshotOpen {
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(active, maximum_copy_bytes);
        }
        if self.snapshot.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.snapshot);
        }
        if self.request.is_some() {
            return semio_framework_value_retirement_owner::retirement_owner_demands(&self.request, maximum_copy_bytes);
        }
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.input.capacity(), depth: usize::from(!self.terminal), ..Default::default() })
    }
}

impl store::ErasedSnapshotRetirement for RecursiveBranchSnapshotOpen {
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.close_demands(maximum_body_bytes)}

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, semio_framework_value::ValueError> {
        let empty = store::RetainedCloneProgress::default();
        if self.terminal {
            return Ok(store::RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "recursive snapshot retirement exceeds admitted depth"));
        }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        self.diagnostic.get_or_insert(store::MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant);
        }
        if self.snapshot.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.snapshot, &mut self.active, grant);
        }
        if self.request.is_some() {
            let step = semio_framework_value_retirement_owner::retirement_owner_close(&mut self.request, grant)?;
            return Ok(store::RetainedCloneStep::Progress(step.progress()));
        }
        if self.input.capacity() != 0 {
            let released_bytes = self.input.capacity();
            drop(std::mem::take(&mut self.input));
            self.expected_bytes = None;
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        self.expected_bytes = None;
        self.terminal = true;
        Ok(store::RetainedCloneStep::Complete(store::RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.capacity() == 0 && self.expected_bytes.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }
}

impl Drop for RecursiveBranchSnapshotOpen {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || store::ErasedSnapshotRetirement::terminal_is_empty(self), "recursive snapshot decoder dropped before exact handoff or bounded retirement");
    }
}

pub struct RecursiveMemberStoreProducer<P>(::std::marker::PhantomData<fn()->(P)>);
impl<P> Copy for RecursiveMemberStoreProducer<P> {}
impl<P> Clone for RecursiveMemberStoreProducer<P> {fn clone(&self)->Self{*self}}
impl<P> RecursiveMemberStoreProducer<P> {pub const fn new()->Self{Self(::std::marker::PhantomData)}}

recursive_fixture_snapshot!(RecursiveBranchSnapshot, "recursive-branch-test");

store::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum RecursiveTestMembers, RecursiveTestMembersOpen {
        Branch("s.test.child", "native", "*", "semio.recursive-branch-test/v1") => (RecursiveBranchSnapshot, RecursiveFixtureMutation, RecursiveMemberStoreProducer<RecursiveBranchSnapshot>, RecursiveMemberStoreProducer::<RecursiveBranchSnapshot>::new()),
    }
}

async fn assert_parent_restore_case<const HAS_CHILD: bool>(row: &Value, cx:&mut semio_framework_job::StepContext<'_>) {
    let mut app = VcsArtifactApp::<ComposedParentApp<HAS_CHILD>, TestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    let dialect = test_child_dialect().await;
    let child_id = row["childId"].as_str().unwrap();
    let slot = row["slot"].as_str().unwrap();
    let mut member = TestMembers::create(child_id, &dialect, &TestSnapshot::default().encode_pack(), app.stores.store.local_actor_id().clone()).await.unwrap();
    member.set_owner(Some(store::OwnerRef {
        parent: ArtifactRef { artifact_id: app.stores.store.envelope().id.clone(), dialect: ComposedParentApp::<HAS_CHILD>::DIALECT.into() },
        slot: slot.into(), child_id: child_id.into(),
    })).await;
    let packed = member.envelope_pack_bytes().await.unwrap();
    close_member_admission_fixture(&mut member);
    PARENT_SNAPSHOT_CLONES.with(|count| count.set(0));
    let admitted = app.open_child(slot, child_id, dialect, &packed, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.is_ok();
    let parent_clones = PARENT_SNAPSHOT_CLONES.with(std::cell::Cell::get);
    let published = app.child_composition.test_child_admission_state(1).generation;
    close_member_admission_app(&mut app, cx);
    let expected = row["accepted"].as_bool().unwrap();
    assert_eq!(admitted, expected, "{}", row["id"]);
    assert_eq!(published, u64::from(expected), "{}", row["id"]);
    assert_eq!(parent_clones, 0, "{}", row["id"]);
}

#[semio_framework_async_macros::async_test]
async fn member_factory_parent_snapshot_restore_matches_neutral_corpus() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🧫️fixtures/🔣️.json"))).unwrap();
    for row in fixture["publicRestoreCases"].as_array().unwrap() {
        if row["parentHasChild"].as_bool().unwrap() { assert_parent_restore_case::<true>(row).await; }
        else { assert_parent_restore_case::<false>(row).await; }
    }
}
//#endregion 🧬️ComposedParentFixture
