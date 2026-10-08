//! 🧺 Note mutation — `DeleteBlocks`: removes several blocks at once (multi-select delete).

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧺 `delete-blocks` payload — removes several blocks at once (multi-select delete).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-blocks")]
pub struct DeleteBlocks {
    pub ids: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_blocks(ids: Vec<String>) -> NoteMutation {
    NoteMutation::DeleteBlocks(DeleteBlocks { ids })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DeleteBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "blocks", kind: "delete-blocks", record: "DeletedBlocks" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {} blocks", self.ids.len()), &format!("{} Blöcke löschen", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Mutation

/// 🧭️ The blocks a delete really removes, in descending `(parent, index)` order: listed ids that exist, each once, without the ids that vanish with a listed ancestor.
pub fn removal_roots(payload: &DeleteBlocks, base: &NoteSnapshot) -> Vec<(Option<String>, usize, crate::NoteBlockNode)> {
    let mut roots: Vec<(Option<String>, usize, crate::NoteBlockNode)> = Vec::new();
    for (position, id) in payload.ids.iter().enumerate() {
        let (Some(block), Some((parent_id, index))) = (crate::schema::find_block(&base.blocks, id), crate::schema::find_block_location(&base.blocks, id)) else {
            continue;
        };
        let repeated = payload.ids[..position].contains(id);
        let nested = payload.ids.iter().filter(|other| *other != id).filter_map(|other| crate::schema::find_block(&base.blocks, other)).any(|ancestor| crate::schema::find_block(&ancestor_children(ancestor), id).is_some());
        if !repeated && !nested {
            roots.push((parent_id, index, block.clone()));
        }
    }
    roots.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    roots
}

fn ancestor_children(block: &crate::NoteBlockNode) -> Vec<crate::NoteBlockNode> {
    match block {
        crate::NoteBlockNode::Group { children, .. } => children.clone(),
        _ => Vec::new(),
    }
}
