//! 🧱️ 🧱️ Note play app commands command — `delete-block`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::op::NoteMutation;
use crate::schema::mutations::delete_block as delete_block_mutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-block")]
pub struct DeleteBlock {
    pub block_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the deleted block's own removal from
// the "blocks" domain's selection is now the framework's job (`revalidate_interaction_state_after_document_change`
// prunes stale ids against `interaction_topology` after every document dispatch) — this handler no
// longer touches selection at all.
pub fn handle(payload: &DeleteBlock, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_block_mutation(payload.block_id.clone())]))
}
