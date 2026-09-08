//! 🔲️ 🔲️ Note play app commands command — `set-grid-spacing`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-grid-spacing")]
pub struct SetGridSpacing {
    pub value: f64,
}

pub fn handle(payload: &SetGridSpacing, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::change_grid_spacing(Some(payload.value.max(4.0)))]))
}
