use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{ActionMeta, App, EditorApp, VcsArtifactApp, ViewModel};

pub type DrawingApp = VcsArtifactApp<EditorApp<DrawingPlayApp>>;

/// ✏️ `DrawingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<DrawingPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<DrawingPlayApp>` builds it.

/// 🧪️ Draw fixtures carry the production manifest and its exact registered factories.
pub async fn drawing_app() -> DrawingApp {
    new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }).await
}

/// 🧰️ Captures the host-owned active utility in one operation's invocation context.
pub fn meta_with_utility(utility: &str) -> ActionMeta {
    let mut action_meta = meta("local");
    action_meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..Default::default() });
    action_meta
}
