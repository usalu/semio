//! ➖️ `remove-stream` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveStream {
    pub index: usize,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for RemoveStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "stream", kind: "remove-stream", record: "RemoveStream" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(AviDiff { streams: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.streams.get(*index) {
                Some(stream) => vec![AviMutation::InsertStream(insert_stream::InsertStream { index: *index, stream: stream.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove stream", "Datenstrom entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
