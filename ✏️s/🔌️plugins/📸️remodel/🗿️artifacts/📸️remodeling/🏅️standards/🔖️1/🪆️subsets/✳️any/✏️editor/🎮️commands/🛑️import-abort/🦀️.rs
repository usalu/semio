//! 🛑️ Remodeling play app commands — `import-abort`: cancelling an import reverts its open streamed transaction with zero
//! trace — no edit, no history row, nothing announced (design §15, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). A user
//! cancel (the palette action, a host's Cancel) ends the dispatching window's import, else the import still open in the
//! document from any window — the way out of an import its window left open. A host abort (`reason`: the importing
//! window closing) ends that window's own import only.

use crate::editor::remodeling::commands::import_video_frame_payload::import_transaction;
use crate::editor::remodeling::transient::RemodelingWindowTransient;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🛑️ `reason` names a host abort (`retired`); `None` is a user cancel.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "import-abort")]
pub struct ImportAbort {
    pub reason: Option<String>,
}

/// 🛑️ The abort through the dispatching window's tool state `window`: its import's transaction (`streams.last()` for a
/// user cancel from a window without one); with nothing open the runtime answers an empty emission.
pub fn handle_in_window(payload: &ImportAbort, doc: &ArtifactView<'_, RemodelingSnapshot>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    Ok(match (&window.import, &payload.reason) {
        (Some(import), _) => (import.stream_id.as_deref().map_or_else(Emit::default, |stream_id| Emit::abort_transaction(import_transaction(stream_id))), RemodelingWindowTransient::default()),
        (None, Some(_)) => (Emit::default(), window.clone()),
        (None, None) => (doc.snapshot.streams.last().map_or_else(Emit::default, |stream| Emit::abort_transaction(import_transaction(&stream.id))), window.clone()),
    })
}

/// 🛑️ The abort dispatched without a window: a user cancel of the import still open in the document.
pub fn handle(payload: &ImportAbort, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    handle_in_window(payload, doc, &RemodelingWindowTransient::default()).map(|(emit, _)| emit)
}
