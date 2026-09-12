//! 🧱️ 🧱️ Note play app commands command — `add-block`.

use crate::op::NoteMutation;
use crate::schema::create_block_by_kind;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block used to also become the
// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
// lowpoly's `add-primitive`).
pub fn handle(payload: &AddBlock, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let block = create_block_by_kind(&mut ctx.id_owner, &payload.kind, payload.x, payload.y);
    Ok(Emit::mutations(vec![crate::schema::mutations::create_block(block, None, None)]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
