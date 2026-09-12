//! 🎥️ Updates the addressed graph window's local viewport.

use crate::editor::rewriting::window_config::{addressed, RewritingWindowConfigMutation, SetCamera};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation, ViewModel, Viewport2d};
use semio_s_artifact_trinity_jack::Camera;

pub(crate) fn set_viewport(_surface_id: &Option<String>, viewport: &Viewport2d, view: Option<&ViewModel>) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.window-required"), "Rewriting viewport requires a host window context"))?;
    viewport.validate().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.viewport"), error.to_string()))?;
    let camera = Camera { x: viewport.x, y: viewport.y, zoom: viewport.zoom };
    let mutation = addressed(view, RewritingWindowConfigMutation::SetCamera(SetCamera { camera: Some(camera) }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
