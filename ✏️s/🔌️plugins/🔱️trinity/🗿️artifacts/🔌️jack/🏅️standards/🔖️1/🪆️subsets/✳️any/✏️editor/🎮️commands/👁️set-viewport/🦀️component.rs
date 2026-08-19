//! 👁️ 👁️ Trinity Jack app command — `set-viewport`.

use crate::editor::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn set_viewport(viewport_json: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match serde_json::from_str::<Camera>(viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![JackConfigMutation::SetCamera { camera }])),
        Err(_) => Ok(Emit::default()),
    }
}
