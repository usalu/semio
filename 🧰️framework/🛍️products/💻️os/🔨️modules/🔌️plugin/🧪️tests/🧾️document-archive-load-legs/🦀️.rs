// 🧾️ `Effect::LoadDocument` for a CHILDLESS single-document app — architect's, animate's and
// writer's exact shape. The composition suite next door only ever drives archives that carry owned
// members; the whole-document swap a `setActiveExample` emits carries `members: []` and a bare
// `store::empty_document_spr`, and that lane had no native coverage at all. Every batch-A plugin
// died on it with one opaque sentence (`📓️b1a-dormant-plugin-boots.md`).

/// 🏛️ The app shape every `setActiveExample` plugin has: `bounded_document_store_owners`, a
/// `bounded_document_store_initialization_job` over its own schema, a disposer, a one-item artifact
/// preparation factory, and a snapshot with no `#[child]` field. Delegates every other hook to
/// `TestApp::<true>` so this fixture states only what it is testing.
#[derive(Default)]
struct SingleDocumentApp;

impl ArtifactApp for SingleDocumentApp {
    const APP_ID: &'static str = "s.test.single-document@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.test.single-document/v1";
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.single-document", standard: StandardId("1"), subset: SubsetId::ANY };

    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = TestCommand;

    async fn initial_snapshot() -> Self::Snapshot {
        TestSnapshot { count: 0, label: "initial".into(), slot: Vec::new() }
    }

    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        Ok(Emit::default())
    }

    /// 📚️ The refresh poll's whole-document replacement, minted the way every app mints one:
    /// `encode_pack` plus a bare `store::empty_document_spr` naming the APP, not the live store.
    /// This lane never passed through the runtime's identity stamp.
    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        let pack = <TestSnapshot as store::ArtifactPack>::encode_pack(&TestSnapshot { count: 42, label: "example".into(), slot: Vec::new() });
        let spr = resolve_ready(store::empty_document_spr(Self::APP_ID, Self::DOCUMENT_SCHEMA));
        vec![Effect::LoadDocument { pack, spr }]
    }

    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("Single-document fixture"))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(TestCountOneItemPreparationFactory))
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> crate::app::ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        TestApp::<false>::build_config_store_owners()
    }

    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        TestApp::<false>::build_config_store_disposer()
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        TestApp::<false>::build_draft_store_owners()
    }

    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        TestApp::<false>::build_draft_store_disposer()
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        TestApp::<false>::build_presence_local_root_retirement_factory()
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        TestApp::<false>::build_presence_peer_retirement_factory()
    }

    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        TestApp::<false>::build_presence_store_disposer()
    }

    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        TestApp::<false>::build_transient_store_disposer()
    }
}

type SingleDocumentAppUnderTest = VcsArtifactApp<SingleDocumentApp, TestMembers>;

/// 📦️ The RAW effect an app mints in `reset_document_effect`: its own `encode_pack` bytes, a bare
/// `store::empty_document_spr` carrying no composition section at all, and an EMPTY member roster.
/// This is what leaves the command handler, NOT what may legally reach the archive loader.
async fn raw_single_document_archive(doc_id: &str, snapshot: &TestSnapshot) -> protocol::DocumentArchivePack {
    protocol::DocumentArchivePack {
        parent_pack: <TestSnapshot as store::ArtifactPack>::encode_pack(snapshot),
        parent_spr: Box::pin(store::empty_document_spr(doc_id, SingleDocumentApp::DOCUMENT_SCHEMA)).await,
        members: Vec::new(),
    }
}

/// 🪪️ The same archive after the runtime's own `stamp_load_document_identity` — the live store's
/// id, schema, dialect and owner written into the log's composition section. `ShellHost` round-trips
/// the effect's bytes untouched, so this is exactly what the guest's archive loader receives.
async fn stamped_single_document_archive(app: &SingleDocumentAppUnderTest, snapshot: &TestSnapshot) -> protocol::DocumentArchivePack {
    let envelope = app.store.envelope();
    let mut archive = Box::pin(raw_single_document_archive(&envelope.id, snapshot)).await;
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    archive.parent_spr = Box::pin(store::stamp_document_spr_identity(&archive.parent_spr, &envelope.id, SingleDocumentApp::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()))
        .await
        .expect("load-document identity stamp");
    archive
}

async fn drive_single_document_archive(app: &mut SingleDocumentAppUnderTest, operation: u64) -> protocol::DocumentArchiveLoadStatus {
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(app, operation)).await.expect("single-document archive operation status");
        if matches!(status.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            return status;
        }
        let _ = PluginApp::maintenance_step(app, 1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("single-document archive maintenance step");
        semio_framework_async::yield_once().await;
    }
    panic!("single-document archive exceeded its public maintenance progress authority")
}

fn archive_fault_text(status: &protocol::DocumentArchiveLoadStatus) -> String {
    if status.fault.is_empty() {
        return "<no fault bytes>".to_string();
    }
    dsl::diagnostic::decode_fault_bytes(&status.fault).describe()
}

/// 🎯️ THE slice's reproduction: a `setActiveExample`-shaped whole-document replace must land.
#[semio_framework_async_macros::async_test]
async fn a_childless_whole_document_archive_replaces_the_live_document() {
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp)).await;
    assert_eq!(app.snapshot().expect("initial projection").label, "initial");
    let archive = Box::pin(stamped_single_document_archive(&app, &TestSnapshot { count: 42, label: "example".into(), slot: Vec::new() })).await;
    PluginApp::begin_document_archive_load(&mut app, 91, archive).expect("whole-document archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 91)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "whole-document archive replacement failed: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut app, 91).expect("whole-document archive acknowledgement");
    let loaded = app.snapshot().expect("replaced projection");
    assert_eq!(loaded.count, 42, "the live document was not replaced by the archive's snapshot");
    assert_eq!(loaded.label, "example");
    close_member_admission_app(&mut app);
}

/// 🚪️ The lane that carries the effect must stamp it. The refresh poll is a real emission lane — a
/// tool run that finalizes into a whole-document replace leaves through it — and it handed the shell
/// the app's own unstamped log, which parent hydration then refused. Whatever the app mints, what
/// leaves here must already name the live store, so this round trip stamps nothing of its own.
#[semio_framework_async_macros::async_test]
async fn the_refresh_poll_lane_stamps_the_load_document_it_emits() {
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp)).await;
    let effects = Box::pin(app.pending_effects(None)).await;
    let Some(Effect::LoadDocument { pack, spr }) = effects.first().cloned() else { panic!("the refresh poll lane dropped the LoadDocument effect: {effects:?}") };
    let archive = protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
    PluginApp::begin_document_archive_load(&mut app, 94, archive).expect("polled archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 94)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the refresh poll lane emitted an unstamped LoadDocument: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut app, 94).expect("polled archive acknowledgement");
    assert_eq!(app.snapshot().expect("replaced projection").count, 42);
    close_member_admission_app(&mut app);
}

/// 🪪️ The identity stamp is the WHOLE contract between an app's `Effect::LoadDocument` and this
/// loader: an app mints its log with `store::empty_document_spr`, which carries no composition
/// section, and hydration refuses such a log outright (`MemberOpenDiagnostic::Identity`) long before
/// any of the three replacement legs runs. Whichever dispatch lane carries the effect MUST stamp it.
#[semio_framework_async_macros::async_test]
async fn an_unstamped_whole_document_archive_is_refused_by_parent_hydration() {
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp)).await;
    let live_id = app.store.envelope().id.clone();
    let archive = Box::pin(raw_single_document_archive(&live_id, &TestSnapshot { count: 9, label: "unstamped".into(), slot: Vec::new() })).await;
    PluginApp::begin_document_archive_load(&mut app, 93, archive).expect("unstamped archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 93)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Fault);
    assert!(archive_fault_text(&status).contains("Identity"), "an unstamped log must be refused as an identity mismatch: {}", archive_fault_text(&status));
    assert_eq!(app.snapshot().expect("retained projection").label, "initial");
    PluginApp::acknowledge_document_archive_load(&mut app, 93).expect("unstamped archive acknowledgement");
    close_member_admission_app(&mut app);
}

/// 🧭️ The refusal must name its leg on the wire, not collapse into one sentence for eleven causes.
/// An archive whose SPR carries a foreign schema is refused by the app's own initialization job;
/// before this slice the host saw only "failed closure, authority, or retained publication
/// validation" and no `#[cfg(test)]`-free way to tell which.
#[semio_framework_async_macros::async_test]
async fn a_refused_whole_document_archive_names_the_leg_that_refused_it() {
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp)).await;
    let envelope_id = app.store.envelope().id.clone();
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    let mut archive = Box::pin(stamped_single_document_archive(&app, &TestSnapshot { count: 7, label: "foreign".into(), slot: Vec::new() })).await;
    let foreign = Box::pin(store::empty_document_spr(&envelope_id, "semio.test.some-other-app/v1")).await;
    archive.parent_spr = Box::pin(store::stamp_document_spr_identity(&foreign, &envelope_id, "semio.test.some-other-app/v1", &dialect, None)).await.expect("foreign stamp");
    PluginApp::begin_document_archive_load(&mut app, 92, archive).expect("foreign-schema archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 92)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Fault);
    let text = archive_fault_text(&status);
    assert!(
        !text.contains("failed closure, authority, or retained publication validation"),
        "the terminal report is still the opaque three-leg collapse: {text}"
    );
    assert_eq!(app.snapshot().expect("retained projection").label, "initial", "a refused archive must leave the live document untouched");
    PluginApp::acknowledge_document_archive_load(&mut app, 92).expect("foreign-schema archive acknowledgement");
    close_member_admission_app(&mut app);
}
