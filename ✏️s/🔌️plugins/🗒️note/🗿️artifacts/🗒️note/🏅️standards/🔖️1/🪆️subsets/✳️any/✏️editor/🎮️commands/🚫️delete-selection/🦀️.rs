//! 🧱️ 🧱️ Note play app commands command — `delete-selection`.

use crate::op::NoteMutation;
use crate::schema::mutations::delete_blocks as delete_blocks_mutation;
use crate::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ctx.selected_block_ids` is the
// "blocks" domain's current selection, resolved once by `ArtifactEditor::handle` — clearing it back to
// empty after the delete is the framework's job (pruned automatically once the ids no longer exist).
pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    if ctx.selected_block_ids.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![delete_blocks_mutation(ctx.selected_block_ids.clone())]))
}
