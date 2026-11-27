//! 🚪️ The mount of the sourcing editor tests: the concrete app, its fixture scope and its teardown, instantiated here once so no test executable instantiates the app generics again.

use semio_plugin::EditorApp;
use semio_os_kernel as protocol;
use semio_s_artifact_sourcing_curation::editor::sourcing::{SourcingCurationApp, SourcingCurationCommand, create_sourcing_curation_app};
use semio_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members as new_app_with_registry_impl, observe_registered_fixture_app, render_registered_fixture_app, OriginalFixtureScope};
use semio_plugin::{App, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// 🏟️ One test's UI value arena; leaving it first drains the built-node pages the test's dropped trees queued, since those pages still own arena values the arena cannot close over.
pub struct ArenaScope(Option<semio_ui_contract_values::testkit::UiArenaFixture>);

impl Drop for ArenaScope {
    fn drop(&mut self) {
        for _ in 0..1_000_000 {
            if semio_plugin::plugin_app_close_prelude::close_built_node_page_one() {
                break;
            }
        }
        drop(self.0.take());
    }
}

pub fn arena() -> ArenaScope {
    ArenaScope(Some(semio_ui_contract_values::testkit::UiArenaFixture::new()))
}

pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>;

/// 🧪️ Framework test context gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
/// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
/// not the `AppDefinition`-returning `create_sourcing_curation_app`. Local wrapper until that lands.
pub fn sourcing_manifest_for_tests() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}


/// 📱️ The app wired to the real manifest registry — the ONLY constructor this app has.
/// `semio_plugin::artifact_app_laws::new_app`'s bare, registry-less instance is unreachable here:
/// `bounded_first_step_tool_proofs!` declares fifteen tool rows, and `tool_job_registration`
/// admits a row only when its id is `InteractiveJobClassification::Migrated` in the LIVE registry,
/// so an empty `AppActionRegistry` rejects every row with `interactive-job.catalog-authority`.
/// Same reason trinity's `🔌️jack`/`♻️rewriting` editors are registry-backed.
pub async fn new_app() -> SourcingTestApp {
    let scope = OriginalFixtureScope::admit();
    let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>(sourcing_manifest_for_tests, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    SourcingTestApp(app, scope)
}

/// 🧹️ Owning guard around a live `SourcingApp` that runs the bounded plugin close protocol on the
/// way out. `ArtifactStore`'s `Drop` asserts a terminal-empty shallow shell, and an app holds four
/// of them (document, config, draft, interaction), so simply letting a test's app fall out of scope
/// aborts the whole test process — a non-unwinding `panic in a destructor during cleanup`, which
/// takes every other test in the binary with it. Draining here rather than at ~14 call sites keeps
/// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
/// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads
/// `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
pub struct SourcingTestApp(pub SourcingApp, pub OriginalFixtureScope);

impl SourcingTestApp {
    /// 🔄️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
    pub fn close_fixture(&mut self) {
        close_registered_fixture_app(&mut self.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
    }
}

impl std::ops::Deref for SourcingTestApp {
    type Target = SourcingApp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SourcingTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for SourcingTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close_fixture();
        }
    }
}

/// 🗄️ The live document archive of the mounted app.
pub async fn document_archive(app: &SourcingTestApp) -> protocol::DocumentArchivePack {
    PluginApp::document_archive(&app.0).await.expect("document archive")
}

/// 🧹️ Steps the live cleanup of the mounted app, one admitted maintenance turn at a time, until it is idle.
pub fn drain_live_cleanup_to_idle(app: &mut SourcingTestApp) {
    use semio_plugin::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerTurnV1, PluginLifecycleStep};
    const ZERO_PROGRESS_LIMIT: usize = 256;
    let mut stalled = 0usize;
    loop {
        let cx = app.1.cx();
        let identity = MountedOwnerIdentityV1 { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
        let phase = app.0.mounted_owner_maintenance_phase().expect("maintenance phase");
        let mut policy = semio_plugin::artifact_app_laws::fixture_mounted_policy();
        let loan = match phase { MountedOwnerPhaseV1::Preparation => &mut policy.preparation, MountedOwnerPhaseV1::Maintenance => &mut policy.maintenance, MountedOwnerPhaseV1::Close => &mut policy.close };
        loan.maximum_items = loan.maximum_items.min(1);
        loan.maximum_copy_bytes = loan.maximum_copy_bytes.min(4096);
        let Some(mut turn) = MountedOwnerTurnV1::admit(1, identity, phase, policy, cx).expect("maintenance admission") else { stalled += 1; assert!(stalled < ZERO_PROGRESS_LIMIT, "live cleanup admitted no turn for {ZERO_PROGRESS_LIMIT} consecutive steps"); continue };
        match app.0.mounted_owner_maintenance_step(&mut turn).expect("a live-cleanup maintenance unit never faults") {
            PluginLifecycleStep::Complete(_) => return,
            PluginLifecycleStep::Progress(receipt) if receipt != semio_framework_value::retained_clone::RetainedCloneProgress::default() => stalled = 0,
            step @ (PluginLifecycleStep::Progress(_) | PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. }) => { stalled += 1; assert!(stalled < ZERO_PROGRESS_LIMIT, "live cleanup made no progress for {ZERO_PROGRESS_LIMIT} consecutive steps, last {step:?} (browser: plugin.internal.zero-progress)"); }
        }
    }
}
