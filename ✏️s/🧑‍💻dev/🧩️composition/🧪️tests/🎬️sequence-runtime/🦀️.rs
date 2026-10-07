//! 🎬️ Actual Imperative extensions execute the public Sequence host composition.
use semio_s_imperative as imperative_engine;
use semio_s_artifact_sequence_sequence::editor::sequence::{SequenceHost, retire_run_result_cold};

fn installed() {
    use imperative_engine::{contributions_json_from_entries, register_native_imperative_module, sync_imperative_module_contributions};
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        register_native_imperative_module("imperative-extension-math", semio_s_plugin_imperative_math::register);
        register_native_imperative_module("imperative-extension-text", semio_s_plugin_imperative_text::register);
        register_native_imperative_module("imperative-extension-effect", semio_s_plugin_imperative_effect::register);
        let json = contributions_json_from_entries(&[
            semio_s_plugin_imperative_math::imperative_module_contribution(),
            semio_s_plugin_imperative_text::imperative_module_contribution(),
            semio_s_plugin_imperative_effect::imperative_module_contribution(),
            semio_s_plugin_imperative_control::imperative_module_contribution(),
        ]);
        sync_imperative_module_contributions(&json);
    });
}


#[semio_framework_async_macros::async_test]
async fn run_executes_default_snapshot_and_records_scope() {
    installed();
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let result = host.run();
    assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
    assert!(!result.effects.is_empty());
    // 🧊️ `run` mints a fresh scope dictionary (and one per effect) that nothing else owns — dropping
    // the result unretired trips `final Dictionary ownership must be explicitly retired or owned by a
    // cold boundary`, which is why the crate publishes its own exact retirement for this shape.
    retire_run_result_cold(result);
}


mod context {
    use semio_framework_plugin::{AppActionRegistry, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    use semio_s_artifact_sequence_sequence::editor::sequence::{SequencePlayApp, SequenceCommand, create_sequence_app};
    use semio_s_artifact_sequence_sequence::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_WINDOW_SCRIPT;
    use semio_s_artifact_stdio_semio::SemioMembers;
    use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation, close_registered_fixture_app, project_and_retire_fixture_tree};

    pub struct SequenceApp(pub(crate) VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>);
    impl std::ops::Deref for SequenceApp {
        type Target = VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl std::ops::DerefMut for SequenceApp {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl Drop for SequenceApp {
        fn drop(&mut self) {
            if !std::thread::panicking() { close_registered_fixture_app(&mut self.0); }
        }
    }
    pub async fn new_app() -> SequenceApp {
        super::installed();
        let mut app = VcsArtifactApp::<EditorApp<SequencePlayApp>, SemioMembers>::with_registry(EditorApp::default(), AppActionRegistry::from_definition(&create_sequence_app()), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(1).await;
        SequenceApp(app)
    }
    pub fn script_window_meta() -> semio_framework_plugin::ActionMeta {
        let window = ViewWindowInstance { id: format!("{SEQUENCE_PLAY_WINDOW_SCRIPT}#1"), window_kind_id: SEQUENCE_PLAY_WINDOW_SCRIPT.into() };
        semio_framework_plugin::ActionMeta { view_state: Some(ViewModel { window_id: Some(window.id.clone()), active_window_kind_id: Some(window.window_kind_id.clone()), window_instances: vec![window], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }), ..meta("local") }
    }
    pub async fn dispatch_in_script(app: &mut SequenceApp, command: SequenceCommand) {
        app.0.dispatch_typed(command, &script_window_meta()).await.expect("dispatch");
        settle_registered_typed_operation(&mut app.0, 1).await.expect("settle typed operation");
    }
    pub async fn render_in(app: &mut SequenceApp, body_key: &str, view: &ViewModel) -> String {
        project_and_retire_fixture_tree(app.0.render(body_key, None, view).await.expect("render")).expect("retire rendered tree")
    }
}
#[path = "🏃️playback/🦀️.rs"]
mod playback;

#[semio_framework_async_macros::async_test]
async fn function_steps_use_data_ports_without_visible_execution_pins() {
    installed();
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.add_step("math.add", 0.0, 0.0);
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).expect("projected step node");
    assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == semio_framework_artifact_infinite_dag::PortShape::Triangle && port.visible));
}


#[semio_framework_async_macros::async_test]
async fn text_steps_use_data_ports_without_visible_execution_pins() {
    installed();
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.add_step("text.concat", 0.0, 0.0);
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).expect("projected step node");
    assert!(node.inputs().iter().any(|port| port.id == "left" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "into" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == semio_framework_artifact_infinite_dag::PortShape::Triangle && port.visible));
}


#[path = "🪟️windows/🦀️.rs"]
mod windows;
