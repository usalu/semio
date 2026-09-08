
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `Block5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Block5dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Block5dPlayApp>` builds it.
pub type Block5dApp = VcsArtifactApp<EditorApp<Block5dPlayApp>>;

pub async fn new_app() -> Block5dApp {
    app_with_registry().await
}

/// ✏️ Adapts `create_block5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
/// still expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn block5d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_block5d_app(), examples: Vec::new() }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
pub async fn app_with_registry() -> Block5dApp {
    new_app_with_registry::<EditorApp<Block5dPlayApp>>(block5d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Block5dApp, command: Block5dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Block5dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}
