//! 🔲️ 🔲️ Note play app commands command — `set-grid-spacing`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-grid-spacing")]
pub struct SetGridSpacing {
    pub value: f64,
}

pub async fn handle(payload: &SetGridSpacing, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_grid_spacing(Some(payload.value.max(4.0)))]))
}
