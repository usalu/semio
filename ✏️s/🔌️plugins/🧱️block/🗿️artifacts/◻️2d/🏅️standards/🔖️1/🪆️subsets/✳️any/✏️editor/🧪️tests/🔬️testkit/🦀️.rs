
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Block2dApp = VcsArtifactApp<EditorApp<Block2dPlayApp>>;

pub async fn new_app() -> Block2dApp {
    app_with_registry().await
}

/// ✏️ Adapts `create_block2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still
/// expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn block2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_block2d_app(), examples: Vec::new() }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
pub async fn app_with_registry() -> Block2dApp {
    new_app_with_registry::<EditorApp<Block2dPlayApp>>(block2d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Block2dApp, command: Block2dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Block2dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}
