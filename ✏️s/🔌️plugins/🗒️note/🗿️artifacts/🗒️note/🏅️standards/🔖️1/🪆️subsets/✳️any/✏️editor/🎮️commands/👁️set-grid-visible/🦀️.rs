//! 🔲️ 🔲️ Note play app commands command — `set-grid-visible`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-grid-visible")]
pub struct SetGridVisible {
    pub value: Option<bool>,
}

pub fn handle(payload: &SetGridVisible, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next = payload.value.unwrap_or(!doc.snapshot.grid_visible.unwrap_or(true));
    Ok(Emit::mutations(vec![crate::schema::mutations::change_grid_visible(Some(next))]))
}
