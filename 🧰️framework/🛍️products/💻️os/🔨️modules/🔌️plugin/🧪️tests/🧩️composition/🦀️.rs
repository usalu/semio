//#region 🧬️ComposedParentFixture
type ParentFixtureChild = store::ArtifactChild<TestSnapshot>;

thread_local! { static PARENT_SNAPSHOT_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

#[derive(Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.composed")]
struct ComposedParentSnapshot {
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    slot: Option<ParentFixtureChild>,
}

impl Clone for ComposedParentSnapshot {
    fn clone(&self) -> Self {
        PARENT_SNAPSHOT_CLONES.with(|count| count.set(count.get() + 1));
        Self { slot: self.slot.clone() }
    }
}

impl store::ArtifactDsl for ComposedParentSnapshot {
    const EXTENSION: &'static str = "composed-parent-test";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let value = serde_json::from_str::<Value>(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        store::os_pack::json::to_json_string(self)
    }
}

impl ArtifactPack for ComposedParentSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(store::os_pack::json::to_json_string(self).into_bytes())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = serde_json::from_slice::<Value>(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

impl protocol::MutationDiff<ComposedParentSnapshot> for NoConfig {
    fn apply(&self, base: &ComposedParentSnapshot) -> protocol::MutationApplyResult<ComposedParentSnapshot> { Ok(base.clone()) }
    fn absorb(&mut self, _other: Self) {}
}

impl Mutation<ComposedParentSnapshot> for NoConfigMutation {
    type Diff = NoConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { match *self {} }
    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<NoConfig> { match *self {} }
    fn inverse(&self, _base: &ComposedParentSnapshot) -> Vec<Self> { match *self {} }
}

#[derive(Default)]
struct ComposedParentApp<const HAS_CHILD: bool = true>;

impl<const HAS_CHILD: bool> ArtifactApp for ComposedParentApp<HAS_CHILD> {
    const APP_ID: &'static str = "s.test.composed@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.composed-test/v1";
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.composed", standard: StandardId("1"), subset: SubsetId::ANY };
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("test.parent-projection"), error.to_string()))
    }
    type Snapshot = ComposedParentSnapshot;
    type Mutation = NoConfigMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = NoConfigMutation;

    async fn initial_snapshot() -> Self::Snapshot {
        Self::Snapshot { slot: HAS_CHILD.then(|| ParentFixtureChild::new("child-1".into(), ArtifactRef {
            artifact_id: "child-1".into(),
            dialect: ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() },
        })) }
    }
    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match *command {}
    }
    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("Composed parent fixture"))
    }
    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> { Some(bounded_document_store_owners()) }
    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(bounded_document_store_disposer()) }
    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> { TestApp::<false>::build_config_store_owners() }
    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { TestApp::<false>::build_config_store_disposer() }
    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> { TestApp::<false>::build_draft_store_owners() }
    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { TestApp::<false>::build_draft_store_disposer() }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_local_root_retirement_factory() }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_peer_retirement_factory() }
    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { TestApp::<false>::build_presence_store_disposer() }
    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { TestApp::<false>::build_transient_store_disposer() }
}

async fn assert_parent_restore_case<const HAS_CHILD: bool>(row: &Value) {
    let mut app = VcsArtifactApp::<ComposedParentApp<HAS_CHILD>, TestMembers>::new(ComposedParentApp::default()).await;
    let dialect = test_child_dialect().await;
    let child_id = row["childId"].as_str().unwrap();
    let slot = row["slot"].as_str().unwrap();
    let mut member = TestMembers::create(child_id, &dialect, &TestSnapshot::default().encode_pack()).await.unwrap();
    member.set_owner(Some(store::OwnerRef {
        parent: ArtifactRef { artifact_id: app.store.envelope().id.clone(), dialect: ComposedParentApp::<HAS_CHILD>::DIALECT.into() },
        slot: slot.into(), child_id: child_id.into(),
    })).await;
    let packed = member.envelope_pack_bytes().await.unwrap();
    close_member_admission_fixture(&mut member);
    PARENT_SNAPSHOT_CLONES.with(|count| count.set(0));
    let admitted = app.open_child(slot, child_id, dialect, &packed).await.is_ok();
    let parent_clones = PARENT_SNAPSHOT_CLONES.with(std::cell::Cell::get);
    let published = app.test_child_admission_state(1).generation;
    close_member_admission_app(&mut app);
    let expected = row["accepted"].as_bool().unwrap();
    assert_eq!(admitted, expected, "{}", row["id"]);
    assert_eq!(published, u64::from(expected), "{}", row["id"]);
    assert_eq!(parent_clones, 0, "{}", row["id"]);
}

#[semio_framework_async_macros::async_test]
async fn member_factory_parent_snapshot_restore_matches_neutral_corpus() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🏪️store/🧩️composition/🪪️member-dialect/🧫️fixtures/🔣️.json"))).unwrap();
    for row in fixture["publicRestoreCases"].as_array().unwrap() {
        if row["parentHasChild"].as_bool().unwrap() { assert_parent_restore_case::<true>(row).await; }
        else { assert_parent_restore_case::<false>(row).await; }
    }
    eprintln!("[DEBUG] actual parent snapshot restore matched four independent neutral authority cases");
}

struct ReadyComposedParentInitialization {
    candidate: Option<store::ArtifactStore<ComposedParentSnapshot, NoConfigMutation>>,
    closing: bool,
}

impl crate::app::ArtifactStoreInitializationAuthority<ComposedParentSnapshot, NoConfigMutation> for ReadyComposedParentInitialization {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if self.closing || cx.is_cancelled() {
            semio_framework_job::StepOutcome::Cancelled
        } else {
            semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            })
        }
    }

    fn request_cancel(&mut self) {
        self.closing = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<ComposedParentSnapshot, NoConfigMutation>> {
        self.candidate.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        let Some(candidate) = self.candidate.as_mut() else { return Ok(PluginCloseStep::Complete) };
        match candidate.close_owned_step(maximum_items, maximum_bytes).map_err(Fault::from)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
            store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "composed parent initializer candidate is externally retained" }),
            store::SnapshotRetirementStep::Complete if candidate.close_owned_terminal_is_empty() => {
                drop(self.candidate.take());
                Ok(PluginCloseStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("test.composed-parent-close"), "candidate close returned false terminal")),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.candidate.is_none()
    }
}

async fn retained_composed_replacement_fixture(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    parent_id: &str,
    child_count: i32,
) -> (crate::app::ArtifactEnvelopeDecodeOperationHandle, crate::app::OwnedDocumentMemberIngress) {
    let generation = semio_framework_job::Generation(app.store.generation_now());
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new("child-1".into(), child_reference.clone())) };
    let mut envelope = store::create_document_envelope::<ComposedParentSnapshot, NoConfigMutation>(ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_id, snapshot, None);
    envelope.dialect = Some(parent_dialect.clone());
    let mut candidate = store::ArtifactStore::new(envelope).await.expect("candidate parent store");
    candidate.install_member_store_owners_exact(ComposedParentApp::<true>::build_document_store_owners().expect("candidate parent owners"));
    let job = crate::app::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization { candidate: Some(candidate), closing: false }));
    let operation = semio_framework_job::OperationId(operation);
    app.store_replacement_jobs.insert_admitted(
        operation.0,
        crate::app::ActiveArtifactStoreReplacement::new(operation, generation, app.child_content_generation, job),
    );

    let owner = store::OwnerRef {
        parent: ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect },
        slot: "slot".into(),
        child_id: "child-1".into(),
    };
    let mut member = TestMembers::create("child-1", &child_dialect, &TestSnapshot { count: child_count, label: "replacement".into() }.encode_pack()).await.expect("candidate child");
    member.set_owner(Some(owner.clone())).await;
    let bytes = member.envelope_pack_bytes().await.expect("candidate child full envelope");
    close_member_admission_fixture(&mut member);
    let chunks = bytes.chunks(store::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes.len() }).expect("candidate member page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded member page")).unwrap_or_else(|_| panic!("pre-admitted candidate member page"));
    }
    pages.seal().expect("candidate member page set");
    let request = store::MemberOpenRequest::new(operation, generation, u64::MAX, child_reference.clone(), Some(owner.clone()), pages).admit(1).unwrap_or_else(|_| panic!("candidate member request"));
    let ingress = crate::app::OwnedDocumentMemberIngress::try_new(0, child_reference, owner, request).unwrap_or_else(|_| panic!("candidate member ingress"));
    (crate::app::ArtifactEnvelopeDecodeOperationHandle { operation, generation }, ingress)
}

async fn drive_composed_replacement_to(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    handle: crate::app::ArtifactEnvelopeDecodeOperationHandle,
    target: crate::app::ActiveArtifactStoreReplacementState,
) {
    for _ in 0..100_000 {
        if app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == target) {
            return;
        }
        app.maintenance_stage = 14;
        let _ = PluginApp::maintenance_step(app, 1, 4096).expect("retained composed replacement step");
        semio_framework_async::yield_once().await;
    }
    panic!("retained composed replacement did not reach {target:?}");
}

async fn live_composed_replacement_app() -> VcsArtifactApp<ComposedParentApp, TestMembers> {
    let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default()).await;
    let dialect = test_child_dialect().await;
    let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack()).await.expect("live child");
    app.register_child("slot", "child-1", dialect, member).await.expect("live child publication");
    app
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically() {
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.store.envelope().id.clone();
    let child_dialect = test_child_dialect().await;
    app.pending_child_pins.push(vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect }, checkpoint_id: "old-checkpoint".into() });
    let old_window_generation = app.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 701, "replacement-parent", 23).await;
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("member ingress registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("exact candidate member admission"));
    app.seal_owned_document_members(handle).expect("complete candidate member set");

    for _ in 0..100_000 {
        let committed = app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.committed);
        if committed {
            assert_eq!(app.store.envelope().id, "replacement-parent");
            assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("new content view").count, 23);
            assert_eq!(app.composition.graph_mut().await.owner_of("child-1").await, Some("replacement-parent"));
            assert!(app.pending_child_pins.is_empty());
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation + 1);
        } else {
            assert_eq!(app.store.envelope().id, old_parent_id);
            assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("old content view").count, 0);
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
        }
        if app.poll_artifact_store_replacement(handle) == crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        app.maintenance_stage = 14;
        let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("atomic composed replacement drive");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("terminal replacement acknowledgement"));
    close_member_admission_app(&mut app);
    eprintln!("[DEBUG] recursive replacement published parent/member/content/coordinator/window together and incrementally retired old roots, members, pins, and store");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle() {
    for (case, target) in [
        ("member-open", crate::app::ActiveArtifactStoreReplacementState::OpeningMembers),
        ("closure", crate::app::ActiveArtifactStoreReplacementState::ValidatingClosure),
        ("views", crate::app::ActiveArtifactStoreReplacementState::PreparingCandidateViews),
    ] {
        let mut app = live_composed_replacement_app().await;
        let old_parent_id = app.store.envelope().id.clone();
        let old_window_generation = app.window_transient_store.document_generation();
        let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 710 + case.len() as u64, &format!("cancel-{case}"), 31).await;
        drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
        assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("cancel member registry"));
        app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("cancel candidate ingress"));
        app.seal_owned_document_members(handle).expect("cancel candidate seal");
        drive_composed_replacement_to(&mut app, handle, target).await;
        if case == "member-open" {
            app.maintenance_stage = 14;
            let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("begin exact member open before cancellation");
            assert!(app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()));
        }
        app.cancel_artifact_store_replacement(handle).expect("cancel retained composed replacement");
        drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete).await;
        assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Cancelled, "{case}");
        assert_eq!(app.store.envelope().id, old_parent_id, "{case}");
        assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0, "{case}");
        assert_eq!(app.window_transient_store.document_generation(), old_window_generation, "{case}");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("cancel acknowledgement"));
        close_member_admission_app(&mut app);
    }
    eprintln!("[DEBUG] recursive replacement cancellation retained the live bundle during member open, closure validation, and immutable-view preparation");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication() {
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 729, "stale-parent", 47).await;
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("stale member registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("stale candidate ingress"));
    app.seal_owned_document_members(handle).expect("stale candidate seal");
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::CandidateReady).await;
    let generation = app.admit_child_content_publication().expect("real live child publication authority");
    app.publish_child_content_member(generation, "slot", "child-1").await.expect("real live child generation change");
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.store.envelope().id, old_parent_id);
    assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0);
    assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("stale replacement acknowledgement"));
    close_member_admission_app(&mut app);
    eprintln!("[DEBUG] recursive replacement fenced a real child-content generation change and retired its unpublished candidate without changing parent, child view, graph, or windows");
}
//#endregion 🧬️ComposedParentFixture

//#region 📨️EnvelopeDecodeLadder
/// 🐢️ Field owner that yields a fixed number of decode steps before it completes, so the decode
/// ladder is measurable without a domain field catalog. `budget` steps of `Pending` on the same
/// token exercise exactly the redelivery loop the real fresh decoder runs.
struct SlowEnvelopeFieldDecoder {
    budget: usize,
    steps: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl SlowEnvelopeFieldDecoder {
    fn new(budget: usize, steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>) -> Self {
        Self { budget, steps: std::sync::Arc::clone(steps) }
    }
}

impl store::ArtifactEnvelopeFieldDecoder<ComposedParentSnapshot, NoConfigMutation> for SlowEnvelopeFieldDecoder {
    fn accept_field_token(
        &mut self,
        _field_id: u16,
        _token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        self.steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        cx.consume_fuel(1);
        if self.budget == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        self.budget -= 1;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Ok(store::ArtifactEnvelopeFieldDecodeStep::RecordComplete)
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        true
    }
}

fn envelope_law_pages() -> store::OwnedSchemaDecodePages {
    let wire = br#"{"schema":"semio.composed-test/v1","id":"envelope-decode-law","vcs":{}}"#;
    let chunks = wire.chunks(store::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages =
        store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: wire.len() }).expect("envelope law page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded envelope law page")).unwrap_or_else(|_| panic!("pre-admitted envelope law page"));
    }
    pages.seal().expect("sealed envelope law page set");
    pages
}

fn install_slow_envelope_decode(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    budget: usize,
    steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>,
) -> crate::app::ArtifactEnvelopeDecodeOperationHandle {
    let operation = semio_framework_job::OperationId(operation);
    let generation = semio_framework_job::Generation(app.store.generation_now());
    app.admit_artifact_envelope_decode_owner(operation, generation, envelope_law_pages(), Box::new(SlowEnvelopeFieldDecoder::new(budget, steps)), store::ArtifactEnvelopeDecodeCompletion::new())
        .unwrap_or_else(|_| panic!("exact envelope decode owner admission"))
}

/// 🚿️ Drives one live envelope decode to its terminal poll through the reactor-turn pump, so the
/// app can close with every field lease returned.
async fn drain_envelope_decode(app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>, handle: crate::app::ArtifactEnvelopeDecodeOperationHandle) -> crate::app::ArtifactEnvelopeDecodeOperationPoll {
    let mut poll = crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending;
    for _ in 0..100_000 {
        PluginApp::advance_typed_operation_publication(app).await.expect("one reactor turn drives the envelope decode worker");
        poll = app.advance_artifact_envelope_load(handle).expect("envelope load advancement");
        if poll != crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending {
            return poll;
        }
        semio_framework_async::yield_once().await;
    }
    poll
}

/// ⚖️ LAW: a decode that has not finished is reported as `Pending`, never as a terminal `Fault`.
/// The store replacement it will hand to does not exist until the decode is `Ready`, and reading
/// that absent job here made every in-progress document load fail closed at the first turn
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault() {
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 811, 64, &steps);
    assert_eq!(app.poll_artifact_envelope_decode(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending);
    assert_eq!(
        app.advance_artifact_envelope_load(handle).expect("live decode advancement"),
        crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending,
        "an unfinished decode is pending, not a terminal fault"
    );
    assert_eq!(
        app.poll_artifact_store_replacement(handle),
        crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault,
        "no replacement job exists yet, which is exactly the reading that must not leak out of the load API"
    );
    drain_envelope_decode(&mut app, handle).await;
    close_member_admission_app(&mut app);
}

/// ⚖️ LAW: the reactor turn pumps the envelope decode worker to its terminal poll.
/// `ActiveArtifactEnvelopeDecode::drive` only SUBMITS a step to the process pool, and the pool runs
/// a submitted step on wasm solely when it is pumped; before this the ONLY driver was the
/// cooperative-maintenance rotation, so a document load never advanced inside an interactive turn.
#[semio_framework_async_macros::async_test]
async fn one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll() {
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 812, 256, &steps);
    assert!(PluginApp::has_runnable_typed_operations(&app), "a live envelope decode is runnable reactor-turn work");
    let poll = drain_envelope_decode(&mut app, handle).await;
    let driven = steps.load(std::sync::atomic::Ordering::SeqCst);
    assert_ne!(poll, crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending, "the reactor-turn pump left the decode pending after {driven} steps");
    assert!(driven >= 256, "the reactor-turn pump drove only {driven} decode steps");
    assert!(!PluginApp::has_runnable_typed_operations(&app), "a retired decode is no longer runnable reactor-turn work");
    close_member_admission_app(&mut app);
}
//#endregion 📨️EnvelopeDecodeLadder
