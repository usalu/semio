//! 👥 Note mutation — `DuplicateBlocks`: copies several blocks at once (multi-select duplicate).

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 👥 `duplicate-blocks` payload — copies several blocks at once (multi-select duplicate).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-blocks")]
pub struct DuplicateBlocks {
    pub source_ids: Vec<String>,
    #[dsl(statements, block)]
    pub blocks: Vec<crate::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_blocks(source_ids: Vec<String>, blocks: Vec<crate::NoteBlockNode>) -> NoteMutation {
    NoteMutation::DuplicateBlocks(DuplicateBlocks { source_ids, blocks })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "blocks", kind: "duplicate-blocks", record: "DuplicatedBlocks" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Duplicate {} blocks", self.source_ids.len()), &format!("{} Blöcke duplizieren", self.source_ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.source_ids.clone()
    }
}
//#endregion 🔖️Mutation

/// 🧭️ Where each duplicate lands, in payload order: right after its source's BASE position (`index + 1`), the position every later duplicate of the same container is pinned to as well — plus the source ids that do not exist.
pub fn placements(payload: &DuplicateBlocks, base: &NoteSnapshot) -> (Vec<(Option<String>, usize, crate::NoteBlockNode)>, Vec<String>) {
    let mut placed = Vec::new();
    let mut missing = Vec::new();
    for (source_id, block) in payload.source_ids.iter().zip(payload.blocks.iter()) {
        match crate::schema::find_block_location(&base.blocks, source_id) {
            Some((parent_id, index)) => placed.push((parent_id, index + 1, block.clone())),
            None => missing.push(source_id.clone()),
        }
    }
    (placed, missing)
}
