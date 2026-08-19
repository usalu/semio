//! ✏️ ✏️ Note play app commands command — `set-pencil-width`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-pencil-width")]
pub struct SetPencilWidth {
    pub value: f64,
}

pub async fn handle(payload: &SetPencilWidth, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_pencil_width(Some(payload.value.clamp(1.0, 24.0)))]))
}
