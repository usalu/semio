//! ✏️ ✏️ Note play app commands command — `set-pencil-width`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-pencil-width")]
pub struct SetPencilWidth {
    pub value: f64,
}

pub fn handle(payload: &SetPencilWidth, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::change_pencil_width(Some(payload.value.clamp(1.0, 24.0)))]))
}
