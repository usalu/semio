//! 🎥️ Updates the addressed graph window's local viewport.

use crate::editor::rewriting::window_config::{addressed, RewritingWindowConfigMutation, SetCamera};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation, ViewModel};
use semio_s_artifact_trinity_jack::Camera;

pub(crate) fn set_viewport(_surface_id: &Option<String>, viewport_json: &str, view: Option<&ViewModel>) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.window-required"), "Rewriting viewport requires a host window context"))?;
    let camera = pack::from_json_str::<Camera>(viewport_json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.viewport"), error.to_string()))?;
    if !camera.x.is_finite() || !camera.y.is_finite() || !camera.zoom.is_finite() || camera.zoom <= 0.0 {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("rewriting.viewport"), "Viewport requires finite coordinates and a positive zoom"));
    }
    let mutation = addressed(view, RewritingWindowConfigMutation::SetCamera(SetCamera { camera: Some(camera) }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
