//! 🎥️ The camera gesture bridge: the dispatched action's args become the typed command, and the command becomes the addressed window-config write.

use crate::modes::view::windows::model as model_window;
use crate::mutations::EnergyModelMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraftMutation;
use super::EnergyModelViewCommand;

//#region 🎥️Camera
/// 🎥️ Turns one `setCamera` gesture into the addressed window-config write, or into an explicit
/// refusal. This is the WHOLE behavior of the viewer's command channel.
///
/// 🔒️ The emitted `Emit` carries `window_config_mutations` and NOTHING else — no artifact mutation,
/// no config mutation, no effect — which is the runtime half of the read-only guarantee whose
/// compile-time half is `ViewEmit`. `reduce`'s own signature could carry a document mutation; this
/// function is the one place that decides, and it never builds one.
pub(super) fn camera_emit(command: &EnergyModelViewCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<EnergyModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let EnergyModelViewCommand::SetCamera { camera } = command;
    let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.viewer.window-required"), "a camera change requires a concrete 3d model window"))?;
    if camera.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {position,target,zoom} pose"));
    }
    let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(camera, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is not a value: {error}")))?;
    let pose = <model_window::config::EnergyModelViewerCameraPose as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is malformed: {error}")))?;
    if !pose.is_valid() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera pose is not finite, or its zoom is not positive"));
    }
    let mutation = model_window::config::EnergyModelViewerWindowConfigMutation::SetCamera(model_window::config::SetCamera { camera: pose });
    Ok(Emit {
        window_config_mutations: vec![model_window::config::addressed(view, mutation)?],
        // 🐢️ Nothing to repaint: the pane that sent the pose already holds it.
        ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        ..Default::default()
    })
}

/// 🪪️ The bridge from a dispatched action's args to the typed command. The pose is validated and
/// canonicalized to the exact JSON string `World3dScene::camera_json` consumes, so a malformed
/// gesture is refused here rather than silently ignored.
pub(super) fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<EnergyModelViewCommand, Fault> {
    if action != model_window::SET_CAMERA_ACTION_ID {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the energy model viewer has no command for action '{action}'")));
    }
    let camera = args
        .and_then(|args| args.get("camera"))
        .and_then(|value| {
            let pose = <store::Viewport3dOrbit as semio_framework_value::FromValue>::from_value(value.clone()).ok()?;
            pose.validate().ok()?;
            Some(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&pose)))
        })
        .unwrap_or_default();
    Ok(EnergyModelViewCommand::SetCamera { camera })
}
//#endregion 🎥️Camera
