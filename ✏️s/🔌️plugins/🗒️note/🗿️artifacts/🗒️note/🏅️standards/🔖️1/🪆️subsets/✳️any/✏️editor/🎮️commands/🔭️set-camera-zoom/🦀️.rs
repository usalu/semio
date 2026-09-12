//! 🎥️ 🎥️ Note play app commands command — `set-camera-zoom`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub value: f64,
}

pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let view = ctx.view_state.as_ref().ok_or_else(|| Fault::from("note-composite-window-context-required"))?;
    let mut config = crate::editor::note::window::config_from_view(cfg);
    config.camera.zoom = payload.value;
    Ok(Emit { window_config_mutations: vec![crate::editor::note::window::addressed_config(view, config)?], ..Default::default() })
}
