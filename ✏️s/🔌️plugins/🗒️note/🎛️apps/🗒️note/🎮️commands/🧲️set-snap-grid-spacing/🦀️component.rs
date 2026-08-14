//! 🧲️ 🧲️ Note play app commands command — `set-snap-grid-spacing`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snap-grid-spacing")]
pub struct SetSnapGridSpacing {
    pub value: f64,
}

pub fn handle(payload: &SetSnapGridSpacing, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::apps::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_snap_grid_spacing(Some(payload.value.max(1.0)))]))
}
