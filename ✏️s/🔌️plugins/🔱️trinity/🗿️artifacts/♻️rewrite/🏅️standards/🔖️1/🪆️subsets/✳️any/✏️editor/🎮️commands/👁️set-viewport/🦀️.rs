//! 👁️ 👁️ Trinity Rewrite app command — `set-viewport`.

use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_viewport(surface_id: &Option<String>, viewport_json: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    if surface_id.as_deref() == Some(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE) {
        match pack::from_json_str::<Camera>(viewport_json) {
            Ok(camera) => Ok(Emit::config(vec![RewriteConfigMutation::SetBeforePaneCamera { camera }])),
            Err(_) => Ok(Emit::default()),
        }
    } else {
        Ok(Emit::default())
    }
}
