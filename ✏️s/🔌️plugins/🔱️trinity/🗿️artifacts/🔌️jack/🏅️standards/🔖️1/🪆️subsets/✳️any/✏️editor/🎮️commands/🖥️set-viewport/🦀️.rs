//! 👁️ Trinity Jack app command — set the addressed graph window viewport.

use crate::editor::jack::window_config::{addressed, JackGraphWindowConfigMutation, SetCamera};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::Camera;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, ViewModel};

pub(crate) fn set_viewport(viewport_json: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, crate::editor::jack::config::JackConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("jack.window-required"), "Jack viewport requires a host window context"))?;
    match pack::from_json_str::<Camera>(viewport_json) {
        Ok(camera) if camera.x.is_finite() && camera.y.is_finite() && camera.zoom.is_finite() && camera.zoom > 0.0 => {
            let mutation = addressed(view, JackGraphWindowConfigMutation::SetCamera(SetCamera { camera: Some(camera) }))?;
            Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
        }
        Ok(_) => Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.viewport"), "Viewport requires finite coordinates and a positive zoom")),
        Err(error) => Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.viewport"), error.to_string())),
    }
}
