//! 🧲️ 🧲️ Note play app commands command — `set-snap-enabled`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snap-enabled")]
pub struct SetSnapEnabled {
    pub value: Option<bool>,
}

pub async fn handle(payload: &SetSnapEnabled, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next = payload.value.unwrap_or(!doc.snapshot.snap_enabled.unwrap_or(false));
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_snap_enabled(Some(next))]))
}
