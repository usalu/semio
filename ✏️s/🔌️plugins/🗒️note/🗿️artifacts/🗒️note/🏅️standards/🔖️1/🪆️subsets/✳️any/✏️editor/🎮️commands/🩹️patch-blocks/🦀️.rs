//! 🧱️ 🧱️ Note play app commands command — `patch-blocks`.

use crate::op::NoteMutation;
use crate::schema::mutations::{
    change_block_font_size, change_block_ink_width, change_block_locked, change_block_visible, edit_block_math, edit_block_text, insert_table_column, insert_table_row, move_block as move_block_mutation, remove_table_column, remove_table_row,
    rename_block, resize_block,
};
use crate::schema::{block_bounds, find_block};
use crate::{NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-blocks")]
pub struct PatchBlocks {
    pub block_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🩹️ Routes the inspector's typed field/value pair to the one narrow semantic mutation that
/// owns it — one mutation per (id, field) pair, batched into a single `Emit` so a multi-select
/// patch is still one undo step. Replaces the old `note_engine::patch_block_field`
/// whole-document-clone + whole-collection re-dump. A request that cannot move the document is refused by name —
/// no ids, an unknown field, a value the field cannot read (`app.command.invalid-args`), an id the note does not
/// hold (`mutation.target-missing`) — instead of an empty emit a caller would read as an accepted edit.
pub fn handle(payload: &PatchBlocks, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    if payload.block_ids.is_empty() {
        return Err(invalid("patchBlocks needs at least one block id".into()));
    }
    let number = || payload.value.trim().parse::<f64>().map_err(|_| invalid(format!("patchBlocks field '{}' needs a number, got '{}'", payload.field, payload.value)));
    let flag = || payload.value.trim().parse::<bool>().map_err(|_| invalid(format!("patchBlocks field '{}' needs true or false, got '{}'", payload.field, payload.value)));
    let document = doc.snapshot;
    let missing: Vec<&str> = payload.block_ids.iter().filter(|id| find_block(&document.blocks, id).is_none()).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the note has no block {}", missing.join(", "))));
    }
    let mut mutations = Vec::with_capacity(payload.block_ids.len());
    for id in &payload.block_ids {
        let block = find_block(&document.blocks, id).expect("checked above");
        let (x, y, width, height) = block_bounds(block);
        mutations.push(match payload.field.as_str() {
            "name" => rename_block(id.clone(), payload.value.clone()),
            "visible" => change_block_visible(id.clone(), flag()?),
            "locked" => change_block_locked(id.clone(), flag()?),
            "x" => move_block_mutation(id.clone(), number()?, y),
            "y" => move_block_mutation(id.clone(), x, number()?),
            "width" => resize_block(id.clone(), number()?, height),
            "height" => resize_block(id.clone(), width, number()?),
            "textContent" => edit_block_text(id.clone(), vec![NoteTextParagraph { runs: vec![NoteTextRun { text: payload.value.clone(), bold: None, italic: None, underline: None, link: None }] }]),
            "textSize" => change_block_font_size(id.clone(), number()?),
            "mathTex" => edit_block_math(id.clone(), payload.value.clone()),
            "inkWidth" => change_block_ink_width(id.clone(), number()?),
            "tableAddRow" => insert_table_row(id.clone()),
            "tableRemoveRow" => remove_table_row(id.clone()),
            "tableAddColumn" => insert_table_column(id.clone()),
            "tableRemoveColumn" => remove_table_column(id.clone()),
            other => return Err(invalid(format!("notes have no patchable block field '{other}'"))),
        });
    }
    Ok(Emit::mutations(mutations))
}
