//! 👁️ Trinity Jack app command — set the addressed graph window viewport.

use crate::editor::jack::window_config::{addressed, JackGraphWindowConfigMutation, SetCamera};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::Camera;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, ViewModel, Viewport2d};

pub(crate) fn set_viewport(viewport: &Viewport2d, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("jack.window-required"), "Jack viewport requires a host window context"))?;
    viewport.validate().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("jack.viewport"), error.to_string()))?;
    let camera = Camera { x: viewport.x, y: viewport.y, zoom: viewport.zoom };
    let mutation = addressed(view, JackGraphWindowConfigMutation::SetCamera(SetCamera { camera: Some(camera) }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
