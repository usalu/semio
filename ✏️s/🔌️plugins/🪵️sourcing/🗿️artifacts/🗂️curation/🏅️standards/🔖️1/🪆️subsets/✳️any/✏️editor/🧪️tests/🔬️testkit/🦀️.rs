
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry as new_app_with_registry_impl};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurationApp>>;

/// 🧪️ Framework testkit gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
/// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
/// not the `AppDefinition`-returning `create_sourcing_curation_app`. Local wrapper until that lands.
pub(crate) fn sourcing_manifest_for_testkit() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}

/// 🧪️ The app wired to the real manifest registry — the ONLY constructor this app has.
/// `semio_framework_plugin::testkit::new_app`'s bare, registry-less instance is unreachable here:
/// `bounded_first_step_tool_proofs!` declares fifteen tool rows, and `tool_job_registration`
/// admits a row only when its id is `InteractiveJobClassification::Migrated` in the LIVE registry,
/// so an empty `AppActionRegistry` rejects every row with `interactive-job.catalog-authority`.
/// Same reason trinity's `🔌️jack`/`♻️rewriting` editors are registry-backed.
pub async fn new_app() -> SourcingTestApp {
    let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>>(sourcing_manifest_for_testkit).await;
    // 🪪️ `dispatch_typed_command_inner` refuses any command whose `ActionMeta.instance_id` is not the
    // app's bound live runtime instance, and a freshly constructed app has none — so bind the id
    // `testkit::meta` stamps. A test that wants another instance rebinds (see
    // `retained_example_load_publishes_authored_stock_and_closes_exact_owners`, which uses 7).
    app.bind_instance_id(1).await;
    SourcingTestApp(app)
}

/// 🧹️ Owning guard around a live `SourcingApp` that runs the bounded plugin close protocol on the
/// way out. `ArtifactStore`'s `Drop` asserts a terminal-empty shallow shell, and an app holds four
/// of them (document, config, draft, interaction), so simply letting a test's app fall out of scope
/// aborts the whole test process — a non-unwinding `panic in a destructor during cleanup`, which
/// takes every other test in the binary with it. Draining here rather than at ~14 call sites keeps
/// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
/// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
pub struct SourcingTestApp(SourcingApp);

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
        for _ in 0..1_000_000 {
            if self.0.close_terminal_is_empty() {
                return;
            }
            self.0.close_step(1, 4_096).expect("test app closes within its exact grant");
        }
        panic!("test app must reach its terminal-empty shell");
    }
}

pub async fn dispatch(app: &mut SourcingApp, command: SourcingCurationCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut SourcingApp, body_key: &str) -> String {
    serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
}
