
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, VcsArtifactApp};

pub type DrawingApp = VcsArtifactApp<EditorApp<DrawingPlayApp>>;

/// ✏️ `DrawingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<DrawingPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<DrawingPlayApp>` builds it.

/// 🧪️ Draw fixtures carry the production manifest and its exact registered factories.
pub async fn drawing_app() -> DrawingApp {
    new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }).await
}

/// 🧰️ Sets the config's host-owned active utility to `utility`.
pub async fn set_utility(app: &mut DrawingApp, utility: &str) {
    app.dispatch_typed(DrawingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: utility.into() }), &meta("local")).await.expect("set active utility");
}
