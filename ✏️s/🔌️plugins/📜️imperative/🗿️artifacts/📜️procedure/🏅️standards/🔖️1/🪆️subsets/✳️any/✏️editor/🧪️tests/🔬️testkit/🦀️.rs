
use super::*;
use semio_framework_plugin::app::App;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type ImperativeApp = VcsArtifactApp<EditorApp<ImperativePlayApp>>;

/// ✏️ `ImperativePlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<ImperativePlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<ImperativePlayApp>` builds it.
/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn imperative_app() -> ImperativeApp {
    new_app::<EditorApp<ImperativePlayApp>>().await
}

/// 🧪️ Adapts `create_imperative_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
/// still expect — framework testkit gap (w2-cad-report "SDK gaps found" #3), not modifiable here
/// (`🧰️framework/**` is outside this packet's lease).
pub fn imperative_app_manifest_for_testkit() -> App {
    App { definition: create_imperative_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline and materializes
/// declared action-arg defaults (e.g. `addStep`'s `kind`).
pub async fn imperative_app_with_registry() -> ImperativeApp {
    new_app_with_registry::<EditorApp<ImperativePlayApp>>(imperative_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut ImperativeApp, command: ImperativeCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut ImperativeApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}
