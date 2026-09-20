use super::*;

#[semio_framework_async_macros::async_test]
async fn create_energy_model_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_energy_model_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, MODEL_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EnergyModelViewer as ArtifactViewer>::DIALECT, MODEL_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_every_window() {
    let def = create_energy_model_viewer();
    for id in [structure::WINDOW_KIND_ID, zones::WINDOW_KIND_ID, simulation::WINDOW_KIND_ID, model_window::WINDOW_KIND_ID] {
        assert!(def.window_kinds.iter().any(|window| window.id == id), "missing window kind {id}");
    }
    assert!(def.interactions.is_empty(), "a read-only viewer declares no interaction domain");
}

/// 👁️ A viewer declares no document-mutating verb — its two kit windows use the READ-ONLY
/// `window_kind()` variants (no `set-node`/`set-cell`) and its simulation window declares none.
/// The builder injects the framework's own history/clipboard/tool actions into EVERY window
/// (`try_build_definition`), so the law is "no `ActionKind::Mutation`", not "no actions".
#[semio_framework_async_macros::async_test]
async fn viewer_declares_no_dispatchable_action() {
    let def = create_energy_model_viewer();
    for window in &def.window_kinds {
        for action in semio_framework::window_kind_actions(&def, window) {
            assert!(!matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "viewer window {} declares the mutating action {}", window.id, action.id);
        }
    }
    assert!(!def.window_kinds.iter().any(|window| semio_framework::window_kind_actions(&def, window).iter().any(|action| action.id == "set-node" || action.id == "set-cell")), "a viewer window carries a kit edit action");
}

//#region 🎥️ViewerCamera
/// 🎥️ The read-only surface's ONE verb. Before this landed, orbiting the viewer's 3d window
/// dispatched `setCamera` into a window kind that did not declare it and the shell dropped every
/// gesture — the same drop the editor's twin used to take (lane W1-B §8).
#[semio_framework_async_macros::async_test]
async fn the_viewer_declares_and_owns_the_camera_verb() {
    let def = create_energy_model_viewer();
    let window = def.window_kinds.iter().find(|window| window.id == model_window::WINDOW_KIND_ID).expect("the 3d window is declared");
    assert!(window.actions.iter().any(|action| action.id == model_window::SET_CAMERA_ACTION_ID), "the viewer's 3d window must declare setCamera or the host's dispatch is dropped: {:?}", window.actions.iter().map(|a| a.id.as_str()).collect::<Vec<_>>());
    assert_eq!(ENERGY_MODEL_VIEW_TOOL_IDS, &[model_window::SET_CAMERA_ACTION_ID], "the retained roster is exactly this viewer's own verbs");
    assert_eq!(<EnergyModelViewCommand as protocol::OpBinary>::TOOL_JOB_IDS, ENERGY_MODEL_VIEW_TOOL_IDS, "TOOL_JOB_IDS drives validate_tool_job_rows' set equality against the proof rows");
}

/// 🔒️ The runtime half of the read-only guarantee: every declared lane is config-shaped, and the
/// emitted `Emit` carries a window-config write and nothing else.
#[semio_framework_async_macros::async_test]
async fn every_viewer_publication_lane_is_a_window_config_lane() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let contracts = <EnergyModelViewCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].tool_id, model_window::SET_CAMERA_ACTION_ID);
    assert_eq!(contracts[0].lanes, &[ArtifactToolPublicationLane::WindowConfig], "a viewer may never name the artifact lane");
}

#[semio_framework_async_macros::async_test]
async fn a_camera_gesture_becomes_an_addressed_window_config_write_and_nothing_else() {
    // 🎥️ The command carries the ORBIT json the args bridge canonicalizes (`{position,target,zoom}`),
    // not the scene camera json the render publishes (`{position,target,up,fov}`) — two different
    // shapes, and only the first one carries a zoom.
    let command = EnergyModelViewCommand::SetCamera { camera: "{\"position\":[12.0,-9.0,7.5],\"target\":[4.0,3.0,1.35],\"zoom\":1.0}".into() };
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("window-1".into()),
        window_instances: vec![semio_framework::ViewWindowInstance { id: "window-1".into(), window_kind_id: model_window::WINDOW_KIND_ID.into() }],
        ..Default::default()
    };
    let emit = camera_emit(&command, Some(&view)).expect("the pose is admissible");
    assert_eq!(emit.window_config_mutations.len(), 1, "one addressed window-config write");
    assert!(emit.artifact_mutations.is_empty(), "a viewer NEVER emits a document mutation");
    assert!(emit.config_mutations.is_empty(), "the app config lane stays untouched");
    assert!(emit.effects.is_empty());
    assert_eq!(emit.coalesce_key.as_deref(), Some("energy.model.3d.viewer.camera:window-1"), "a burst of debounced orbit ticks must collapse per window instance");
}

#[semio_framework_async_macros::async_test]
async fn a_camera_gesture_without_a_concrete_window_is_refused_rather_than_written_anywhere() {
    let command = EnergyModelViewCommand::SetCamera { camera: "{\"position\":[1.0,1.0,1.0],\"target\":[0.0,0.0,0.0],\"zoom\":1.0}".into() };
    assert!(camera_emit(&command, None).is_err(), "a camera is addressed at ONE window instance");

    let view = semio_framework_plugin::ViewModel {
        window_id: Some("window-1".into()),
        window_instances: vec![semio_framework::ViewWindowInstance { id: "window-1".into(), window_kind_id: "energy.structure".into() }],
        ..Default::default()
    };
    assert!(camera_emit(&command, Some(&view)).is_err(), "a pose addressed at another window kind must not write the 3d pane");
    assert!(camera_emit(&EnergyModelViewCommand::SetCamera { camera: String::new() }, Some(&view)).is_err(), "an empty pose is a refusal, never a silent no-op");
    assert!(camera_emit(&EnergyModelViewCommand::SetCamera { camera: "{\"position\":[1.0,1.0,1.0],\"target\":[0.0,0.0,0.0],\"zoom\":0.0}".into() }, Some(&view)).is_err(), "a zero zoom is not a camera");
}

#[semio_framework_async_macros::async_test]
async fn the_args_bridge_canonicalizes_the_hosts_pose_and_refuses_a_foreign_action() {
    let args = dsl::json::from_json_str::<dsl::DslValue>("{\"camera\":{\"position\":[12.0,-9.0,7.5],\"target\":[4.0,3.0,1.35],\"zoom\":1.0}}").expect("args");
    let command = command_from_action(model_window::SET_CAMERA_ACTION_ID, Some(&args)).expect("the bridge accepts the host's pose");
    let EnergyModelViewCommand::SetCamera { camera } = &command;
    assert!(camera.contains("\"position\":[12.0,-9.0,7.5]"), "the pose is canonicalized to the scene camera json: {camera}");
    assert!(command_from_action("set-node", Some(&args)).is_err(), "the viewer has no document verb to bridge to");
    assert_eq!(EnergyModelViewer::command_id(&command), model_window::SET_CAMERA_ACTION_ID);
}

#[semio_framework_async_macros::async_test]
async fn the_view_command_round_trips_through_its_binary_codec() {
    use protocol::OpBinary;
    let command = EnergyModelViewCommand::SetCamera { camera: "{\"position\":[1.0,2.0,3.0],\"target\":[0.0,0.0,0.0],\"zoom\":1.0}".into() };
    assert_eq!(EnergyModelViewCommand::decode_op(&command.encode_op().expect("encode")).expect("decode"), command);
}
//#endregion 🎥️ViewerCamera
