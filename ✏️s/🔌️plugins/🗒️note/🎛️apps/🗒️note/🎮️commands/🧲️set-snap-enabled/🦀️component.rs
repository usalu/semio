//! 🧲️ 🧲️ Note play app commands command — `set-snap-enabled`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snap-enabled")]
pub struct SetSnapEnabled {
    pub value: Option<bool>,
}

pub fn handle(payload: &SetSnapEnabled, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::apps::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next = payload.value.unwrap_or(!doc.snapshot.snap_enabled.unwrap_or(false));
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_snap_enabled(Some(next))]))
}
