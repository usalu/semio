//! 🤏 Note mutation — `DragBlocks`: offsets several blocks by the same relative amount (multi-select drag/nudge).

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🤏 `drag-blocks` payload — offsets several blocks by the same relative amount (multi-select drag/nudge).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (note_label_number(self.dx), note_label_number(self.dy));
        let (en, de) = match self.ids.len() {
            1 => ("1 block".to_string(), "1 Block".to_string()),
            count => (format!("{count} blocks"), format!("{count} Blöcke")),
        };
        protocol::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}

/// 🔢️ A drag label's offset, `(en, de)`: two decimals at most, trailing zeros trimmed, a German decimal comma.
fn note_label_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let en = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded }).trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}
//#endregion 🔖️Mutation
