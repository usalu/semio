//! 🐚️ 🐚️ Note play app commands command — `save-download`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "save-download")]
pub struct SaveDownload {}

pub fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let data = crate::document_dsl::print_dsl(doc.snapshot);
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: "🗒️semio.note.dsl".into(), mime_type: "text/plain".into(), data, encoding: None }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
