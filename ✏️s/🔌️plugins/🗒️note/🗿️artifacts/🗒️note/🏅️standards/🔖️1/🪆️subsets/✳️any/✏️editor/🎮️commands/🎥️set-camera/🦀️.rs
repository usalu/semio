//! 🎥️ 🎥️ Note play app commands command — `set-camera`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::NoteCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: NoteCamera,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let view = ctx.view_state.as_ref().ok_or_else(|| Fault::from("note-composite-window-context-required"))?;
    let config = crate::editor::note::window::NoteCompositeWindowConfig { camera: payload.camera.clone() };
    Ok(Emit { window_config_mutations: vec![crate::editor::note::window::addressed_config(view, config)?], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
