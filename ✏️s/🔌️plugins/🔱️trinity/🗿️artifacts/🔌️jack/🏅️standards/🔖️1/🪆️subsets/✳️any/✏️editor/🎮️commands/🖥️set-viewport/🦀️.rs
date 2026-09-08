//! 👁️ 👁️ Trinity Jack app command — `set-viewport`.

use crate::op::TrinityGraphMutation;
use crate::Camera;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_viewport(viewport_json: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    match pack::from_json_str::<Camera>(viewport_json) {
        Ok(camera) => Emit::config(vec![JackConfigMutation::SetCamera(crate::editor::jack::config::SetCamera { camera })]),
        Err(_) => Emit::default(),
    }
}
