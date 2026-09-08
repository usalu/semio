//! 👁️ 👁️ Trinity Rewriting app command — `set-viewport`.

use semio_s_artifact_trinity_jack::Camera;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_viewport(surface_id: &Option<String>, viewport_json: &str) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    if surface_id.as_deref() == Some(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE) {
        match pack::from_json_str::<Camera>(viewport_json) {
            Ok(camera) => Emit::config(vec![RewritingConfigMutation::SetBeforePaneCamera(crate::editor::rewriting::config::SetBeforePaneCamera { camera })]),
            Err(_) => Emit::default(),
        }
    } else {
        Emit::default()
    }
}
