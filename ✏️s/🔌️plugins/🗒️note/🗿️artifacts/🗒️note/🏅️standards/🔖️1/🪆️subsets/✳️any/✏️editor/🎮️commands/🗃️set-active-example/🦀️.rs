//! 🗃️ 🗃️ Note play app commands command — `set-active-example`.

use crate::op::NoteMutation;
use crate::schema::semio_example_snapshot;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🗃️ Loads the example the shell picked by its registered id (`📚️examples/🎬️demo` → `demo`); an unknown id
/// faults instead of silently replacing the document with an empty one (ticket 26/09/17/NOTE-PLUGIN-END-TO-END:
/// the navbar sent `demo` while this matched only `semio`, so the Demo example always opened blank).
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let next_document = match payload.example_id.as_str() {
        crate::standards::v1::subsets::any::examples::demo::ID => semio_example_snapshot(),
        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("note.example.unknown"), format!("note has no example '{other}'"))),
    };
    Ok(Emit { effects: vec![crate::editor::note::reset_document_effect(&next_document)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
