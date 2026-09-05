//! 🧲️ 🧲️ Note play app commands command — `set-snap-grid-spacing`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snap-grid-spacing")]
pub struct SetSnapGridSpacing {
    pub value: f64,
}

pub async fn handle(payload: &SetSnapGridSpacing, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_snap_grid_spacing(Some(payload.value.max(1.0)))]))
}
