
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `Block3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Block3dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Block3dPlayApp>` builds it.
pub type Block3dApp = VcsArtifactApp<EditorApp<Block3dPlayApp>>;

pub async fn new_app() -> Block3dApp {
    app_with_registry().await
}

/// ✏️ Adapts `create_block3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
pub fn block3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_block3d_app(), examples: Vec::new() }
}

pub async fn app_with_registry() -> Block3dApp {
    new_app_with_registry::<EditorApp<Block3dPlayApp>>(block3d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Block3dApp, command: Block3dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Block3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

pub async fn main_window_measures(app: &mut Block3dApp) -> Vec<semio_framework_plugin::WindowMeasure> {
    app.window_measures().await.get(BLOCK3D_DEFAULT_WINDOW_ID).cloned().unwrap_or_default()
}
