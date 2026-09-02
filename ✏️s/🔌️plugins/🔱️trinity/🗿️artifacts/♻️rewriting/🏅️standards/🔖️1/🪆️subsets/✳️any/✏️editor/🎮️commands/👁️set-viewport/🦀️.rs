//! 👁️ 👁️ Trinity Rewriting app command — `set-viewport`.

use crate::artifacts::jack::Camera;
use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_viewport(surface_id: &Option<String>, viewport_json: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    if surface_id.as_deref() == Some(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE) {
        match pack::from_json_str::<Camera>(viewport_json) {
            Ok(camera) => Ok(Emit::config(vec![RewritingConfigMutation::SetBeforePaneCamera { camera }])),
            Err(_) => Ok(Emit::default()),
        }
    } else {
        Ok(Emit::default())
    }
}
