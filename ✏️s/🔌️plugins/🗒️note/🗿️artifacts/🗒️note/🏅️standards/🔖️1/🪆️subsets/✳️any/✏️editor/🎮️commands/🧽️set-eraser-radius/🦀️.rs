//! ✏️ ✏️ Note play app commands command — `set-eraser-radius`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-eraser-radius")]
pub struct SetEraserRadius {
    pub value: f64,
}

pub fn handle(payload: &SetEraserRadius, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::change_eraser_radius(Some(payload.value.clamp(4.0, 48.0)))]))
}
