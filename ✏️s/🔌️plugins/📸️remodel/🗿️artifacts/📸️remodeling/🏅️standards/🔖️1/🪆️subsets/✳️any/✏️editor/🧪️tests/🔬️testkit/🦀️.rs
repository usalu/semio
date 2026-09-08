
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `RemodelingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<RemodelingPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<RemodelingPlayApp>` builds it.
pub type RemodelingApp = VcsArtifactApp<EditorApp<RemodelingPlayApp>>;

/// ✏️ Adapts `create_remodeling_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/
/// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
/// (`🧰️framework/**` is outside this packet's lease).
pub fn remodeling_app_manifest_for_testkit() -> App {
    App { definition: create_remodeling_app(), examples: Vec::new() }
}

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
/// The RUNTIME side stays async (`ArtifactApp`/`VcsArtifactApp` are async traits, unlike the
/// AUTHORING `ArtifactEditor` this crate implements), so every harness entry point awaits.
pub async fn app() -> RemodelingApp {
    new_app::<EditorApp<RemodelingPlayApp>>().await
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn app_with_registry() -> RemodelingApp {
    new_app_with_registry::<EditorApp<RemodelingPlayApp>>(remodeling_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut RemodelingApp, command: RemodelingCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

/// 🖼️ The rendered tree as text — `ComponentTree` is neither `Serialize` nor `ToValue`, so its own
/// `Debug` projection is what body assertions match against.
pub async fn render(app: &mut RemodelingApp, body_key: &str) -> String {
    format!("{:?}", app.render(body_key, None, &ViewModel::default()).await.expect("render"))
}
