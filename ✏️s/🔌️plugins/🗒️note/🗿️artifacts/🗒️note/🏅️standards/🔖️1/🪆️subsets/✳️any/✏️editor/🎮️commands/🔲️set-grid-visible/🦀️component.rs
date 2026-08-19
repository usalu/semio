//! 🔲️ 🔲️ Note play app commands command — `set-grid-visible`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-grid-visible")]
pub struct SetGridVisible {
    pub value: Option<bool>,
}

pub async fn handle(payload: &SetGridVisible, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next = payload.value.unwrap_or(!doc.snapshot.grid_visible.unwrap_or(true));
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_grid_visible(Some(next))]))
}
