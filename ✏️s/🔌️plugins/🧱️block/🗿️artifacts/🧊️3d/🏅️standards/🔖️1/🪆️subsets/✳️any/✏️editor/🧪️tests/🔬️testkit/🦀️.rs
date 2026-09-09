
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

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
    let mut app = new_app_with_registry::<EditorApp<Block3dPlayApp>>(block3d_app_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    app
}

pub async fn dispatch(app: &mut Block3dApp, command: Block3dCommand) -> InvocationResult {
    let window_id = match &command {
        Block3dCommand::HoverSurface(payload) => payload.window_id.clone(),
        Block3dCommand::PlaceVortex(payload) => payload.window_id.clone(),
        Block3dCommand::LeaveSurface(_) => BLOCK3D_DEFAULT_WINDOW_ID.into(),
        _ => return app.dispatch_typed(command, &meta("local")).await.expect("dispatch"),
    };
    dispatch_in_window(app, command, &window_id).await
}

pub async fn dispatch_in_window(app: &mut Block3dApp, command: Block3dCommand, window_id: &str) -> InvocationResult {
    let view_state = world_view_state(window_id);
    app.dispatch_typed(command, &ActionMeta { view_state: Some(view_state), ..meta("local") }).await.expect("dispatch")
}

pub fn world_view_state(window_id: &str) -> ViewModel {
    ViewModel {
        active_window_kind_id: Some(world::BLOCK3D_WINDOW_WORLD.into()),
        active_utility_id: Some(BLOCK3D_UTILITY_SURFACE_BRUSH.into()),
        window_id: Some(window_id.into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: window_id.into(), window_kind_id: world::BLOCK3D_WINDOW_WORLD.into() }],
        ..ViewModel::default()
    }
}

pub async fn render(app: &mut Block3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

pub async fn main_window_measures(app: &mut Block3dApp) -> Vec<semio_framework_plugin::WindowMeasure> {
    app.window_measures(&ViewModel::default()).await.get(BLOCK3D_DEFAULT_WINDOW_ID).cloned().unwrap_or_default()
}
