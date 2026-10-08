//! 🤏 Note mutation — `DragBlocks`: offsets several blocks by the same relative amount (multi-select drag/nudge).

use crate::schema::mutations::{note_label_number, NoteMutation};
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🤏 `drag-blocks` payload — offsets several blocks by the same relative amount (multi-select drag/nudge).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "drag-blocks")]
pub struct DragBlocks {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_blocks(ids: Vec<String>, dx: f64, dy: f64) -> NoteMutation {
    NoteMutation::DragBlocks(DragBlocks { ids, dx, dy })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DragBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "blocks", kind: "drag-blocks", record: "DraggedBlocks" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (note_label_number(self.dx), note_label_number(self.dy));
        let (en, de) = match self.ids.len() {
            1 => ("1 block".to_string(), "1 Block".to_string()),
            count => (format!("{count} blocks"), format!("{count} Blöcke")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Mutation

/// 🧭️ The current `(x, y)` of every block a drag moves — each addressed block and its whole subtree, each once — and the addressed ids that do not exist.
fn dragged_positions(payload: &DragBlocks, base: &NoteSnapshot) -> (std::collections::BTreeMap<String, (f64, f64)>, Vec<String>) {
    let mut moved = std::collections::BTreeMap::new();
    let mut missing = Vec::new();
    for id in &payload.ids {
        match crate::schema::find_block(&base.blocks, id) {
            Some(block) => {
                for member in crate::schema::flatten_blocks(std::slice::from_ref(block)) {
                    let (x, y, ..) = crate::schema::block_bounds(member);
                    moved.insert(crate::schema::block_id(member).to_string(), (x, y));
                }
            }
            None => missing.push(id.clone()),
        }
    }
    (moved, missing)
}
