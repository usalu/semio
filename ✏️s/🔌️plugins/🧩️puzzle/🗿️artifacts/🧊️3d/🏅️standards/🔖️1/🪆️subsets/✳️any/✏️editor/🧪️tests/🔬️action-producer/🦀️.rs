//! 🏭️ A normal puzzle3d action reaches its typed command through the framework-declared producer: the plugin declares none, yet an action and its undo run end to end on the mounted app.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, dispatch_fixture_action, fixture_identity, meta, new_app_with_registry, settle_history_verb, settle_registered_typed_operation, OriginalFixtureScope};
use semio_framework_plugin::{App, ArtifactApp, ArtifactEditor, EditorApp, MountedOwnerPolicyV1, PluginApp, VcsArtifactApp};
use semio_s_artifact_puzzle_3d_editor::{create_puzzle3d_app, Puzzle3dPlayApp};

type Puzzle3dMounted = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

/// 🏗️ The registered, instance-bound app, boxed because the raw app is tens of KiB.
async fn mounted(scope: &mut OriginalFixtureScope) -> Box<Puzzle3dMounted> {
    let mut app = Box::new(new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(|| App { definition: create_puzzle3d_app(), examples: Vec::new() }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity()).await);
    app.bind_instance_id(1).await;
    let _ = scope;
    app
}

fn target_volumes(app: &Puzzle3dMounted) -> usize {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(app.snapshot().expect("snapshot").typed())).get("targetVolumes").and_then(|volumes| volumes.as_array()).map_or(0, Vec::len)
}

#[semio_framework_async_macros::async_test]
async fn target_volume_action_and_undo_reach_the_command_through_the_framework_declared_producer() {
    assert!(<Puzzle3dPlayApp as ArtifactEditor>::original_action_command_producer().is_none(), "the plugin hand-writes no producer");
    assert!(<EditorApp<Puzzle3dPlayApp> as ArtifactApp>::original_action_command_producer().is_some(), "the framework supplies the generic producer");
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted(&mut scope).await;
    let before = target_volumes(&app);
    let action_meta = meta("local");
    let args: semio_framework_value::DslValue = serde_json::json!({ "origin": [1.0, 2.0, 3.0] }).into();
    {
        let mut original_args = Some(args);
        let mut original_meta = Some(action_meta.clone());
        for turn in 0..300 {
            let result = semio_framework_plugin::PluginApp::handle_action(&mut *app, "addTargetVolume", &mut original_args, &mut original_meta, &mut fixture_identity(), scope.cx()).await;
            eprintln!("[DEBUG] turn {turn} answered={:?} pending={} command_ready={} production={} input_ready={} emission={} typing={} delivering={}", result.as_ref().map(|answer| answer.is_some()).map_err(|_| "err"), app.original_typed_command.pending(), app.original_typed_command.command_ready(), app.original_typed_command.production_pending(), app.original_typed_command.input_ready(), app.original_emission.pending(), app.original_typing.pending(), app.host_events.delivering);
            if !matches!(result, Ok(None)) { result.expect("addTargetVolume is admitted through the producer"); break }
            app.maintenance_step(scope.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).ok();
        }
    }
    settle_registered_typed_operation(&mut *app, action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), scope.cx()).await.expect("the edit publishes");
    assert_eq!(target_volumes(&app), before + 1, "the typed command the producer born applied the edit");
    settle_history_verb(&mut *app, "undo", action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), scope.cx()).await;
    assert_eq!(target_volumes(&app), before, "undo restores the document");
    close_registered_fixture_app(&mut *app, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
}
