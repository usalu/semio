//! 🔲️ 🔲️ Note play app commands command — `set-grid-subdivisions`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-grid-subdivisions")]
pub struct SetGridSubdivisions {
    pub value: f64,
}

pub fn handle(payload: &SetGridSubdivisions, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::change_grid_subdivisions(Some(payload.value.round().clamp(1.0, 16.0)))]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
