//! 🧱️ `add-unknown-chunk` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddUnknownChunk {
    pub index: usize,
    pub item: RiffChunk,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for AddUnknownChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "unknown-chunk", kind: "add-unknown-chunk", record: "AddUnknownChunk" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add unknown chunk", "Unbekannten Chunk hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<AviSnapshot, AviMutation> for AddUnknownChunk {
    fn diff(&self, _base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::ColdMutation<AviSnapshot>>::ReferenceDiff> {
        let Self { index, item } = self;
        protocol::MutationOutcome::new(AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: item.clone() }] }), ..AviDiff::default() })
    }
    fn inverse(&self, _base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index: *index })])
    }
}
//#endregion 🔖️Payload
